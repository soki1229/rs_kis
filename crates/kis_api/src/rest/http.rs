use std::time::Duration;

use rand::Rng;
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{KisConfig, KisError};

/// KIS REST API 응답 공통 래퍼
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct KisResponse<T> {
    #[serde(rename = "rt_cd")]
    result_code: String,
    #[serde(rename = "msg_cd")]
    message_code: String,
    #[serde(rename = "msg1")]
    message: String,
    #[serde(flatten)]
    data: Option<T>,
}

/// REST API 요청 파라미터
pub struct RequestParams<'a> {
    pub method: Method,
    pub path: &'a str,
    pub tr_id: &'a str,
    /// Query string 파라미터 (GET)
    pub query: Option<&'a Value>,
    /// JSON body (POST/PUT)
    pub body: Option<&'a Value>,
}

const MAX_RETRIES: u32 = 3;
/// Backoff schedule: 500ms, 1s, 2s.
const BACKOFF_MS: [u64; 3] = [500, 1000, 2000];

/// Compute a jittered backoff delay for the given attempt (0-indexed).
fn backoff_delay(attempt: u32) -> Duration {
    let base = BACKOFF_MS[attempt.min(MAX_RETRIES - 1) as usize];
    let jitter = rand::thread_rng().gen_range(0..=base / 4);
    Duration::from_millis(base + jitter)
}

/// Returns `true` if the reqwest error occurred before any HTTP response was
/// received (i.e. a connection or timeout error — safe to retry even for POST).
fn is_network_error(e: &reqwest::Error) -> bool {
    e.is_connect() || e.is_timeout()
}

/// Whether the given HTTP status code is retryable (for idempotent requests).
fn is_retryable_status(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

/// Whether the request method is safe to retry after receiving an HTTP response.
/// POST is NOT safe — we could duplicate orders.
fn is_idempotent(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS
    )
}

/// Outcome of a single attempt — lets the retry loop distinguish
/// "no response received" from "got an HTTP response we can inspect".
enum Attempt<T> {
    /// Successful parse result.
    Ok(T),
    /// Retryable failure (network error before response, or retryable status).
    Retry(KisError),
    /// Non-retryable failure — return immediately.
    Fail(KisError),
}

/// KIS REST API 실행기 with retry + exponential backoff.
pub async fn execute<T>(
    client: &Client,
    config: &KisConfig,
    token: &str,
    params: RequestParams<'_>,
) -> Result<T, KisError>
where
    T: DeserializeOwned,
{
    let url = format!("{}{}", config.rest_url, params.path);
    let is_idem = is_idempotent(&params.method);

    let mut last_err: Option<KisError> = None;

    for attempt in 0..=MAX_RETRIES {
        if attempt > 0 {
            let delay = backoff_delay(attempt - 1);
            log::warn!(
                "REST retry {}/{} after {:?}: {}",
                attempt,
                MAX_RETRIES,
                delay,
                last_err.as_ref().unwrap()
            );
            tokio::time::sleep(delay).await;
        }

        match try_execute::<T>(client, config, token, &params, &url, is_idem).await {
            Attempt::Ok(v) => return Ok(v),
            Attempt::Fail(e) => return Err(e),
            Attempt::Retry(e) => {
                last_err = Some(e);
            }
        }
    }

    Err(last_err.unwrap())
}

/// Single-attempt execution. Returns an `Attempt` to tell the caller whether
/// to retry, fail, or return the value.
async fn try_execute<T>(
    client: &Client,
    config: &KisConfig,
    token: &str,
    params: &RequestParams<'_>,
    url: &str,
    is_idem: bool,
) -> Attempt<T>
where
    T: DeserializeOwned,
{
    // ---- Build request (not cloneable, so rebuilt each attempt) ----
    let mut req = client
        .request(params.method.clone(), url)
        .header("authorization", format!("Bearer {token}"))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", params.tr_id)
        .header("content-type", "application/json; charset=utf-8")
        .header("accept", "text/plain");

    if let Some(query) = params.query {
        if let Some(obj) = query.as_object() {
            let pairs: Vec<(String, String)> = obj
                .iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect();
            req = req.query(&pairs);
        }
    }

    if let Some(body) = params.body {
        req = req.json(body);
    }

    // ---- Send ----
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            // Network error *before* any response → always safe to retry.
            if is_network_error(&e) {
                return Attempt::Retry(KisError::Network(e));
            }
            return Attempt::Fail(KisError::Network(e));
        }
    };

    // ---- We received an HTTP response ----
    let status = resp.status();

    // Read the body (may also fail with network error).
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            // Body read failure — treat as network error.
            if is_network_error(&e) {
                return Attempt::Retry(KisError::Network(e));
            }
            return Attempt::Fail(KisError::Network(e));
        }
    };

    // 401 — let the caller (KisClient) handle token refresh.
    if status == StatusCode::UNAUTHORIZED {
        return Attempt::Fail(KisError::Auth("401 Unauthorized — token expired?".into()));
    }

    // Retryable HTTP status — but only for idempotent methods.
    if is_retryable_status(status) && is_idem {
        let err = make_api_error(status, &text);
        return Attempt::Retry(err);
    }

    if !status.is_success() {
        return Attempt::Fail(make_api_error(status, &text));
    }

    // ---- Parse response ----
    let v: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => return Attempt::Fail(KisError::Parse(e)),
    };

    if let Some(rt_cd) = v["rt_cd"].as_str() {
        if rt_cd != "0" {
            let code = v["msg_cd"].as_str().unwrap_or("unknown").to_string();
            let msg = v["msg1"].as_str().unwrap_or("unknown").to_string();
            return Attempt::Fail(KisError::Api { code, message: msg });
        }
    }

    match serde_json::from_value::<T>(v) {
        Ok(val) => Attempt::Ok(val),
        Err(e) => Attempt::Fail(KisError::Parse(e)),
    }
}

/// Construct a `KisError::Api` from a non-success response.
fn make_api_error(status: StatusCode, text: &str) -> KisError {
    if let Ok(v) = serde_json::from_str::<Value>(text) {
        let code = v["msg_cd"].as_str().unwrap_or("unknown").to_string();
        let msg = v["msg1"].as_str().unwrap_or(text).to_string();
        return KisError::Api { code, message: msg };
    }
    KisError::Api {
        code: status.as_str().to_string(),
        message: text.to_string(),
    }
}

/// 승인키(approval key) 발급 — WebSocket 인증에 사용
pub async fn fetch_approval_key(client: &Client, config: &KisConfig) -> Result<String, KisError> {
    let url = format!("{}/oauth2/Approval", config.rest_url);
    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": config.app_key,
        "secretkey": config.app_secret,
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(KisError::Network)?;

    if !resp.status().is_success() {
        return Err(KisError::Auth(format!(
            "approval key fetch failed: {}",
            resp.status()
        )));
    }

    let v: Value = resp.json().await.map_err(KisError::Network)?;
    v["approval_key"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| KisError::Auth("approval_key not in response".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_params_compiles() {
        let _ = RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/trading/order",
            tr_id: "TTTT1002U",
            query: None,
            body: None,
        };
    }

    #[test]
    fn execute_fn_exists() {
        // 타입 시그니처 컴파일 확인 (실제 호출은 통합 테스트에서)
        fn assert_execute_signature<T: serde::de::DeserializeOwned>() {
            let _f = execute::<T>;
            let _ = _f; // suppress unused warning
        }
        assert_execute_signature::<Value>();
    }

    #[test]
    fn get_is_idempotent() {
        assert!(is_idempotent(&Method::GET));
        assert!(is_idempotent(&Method::PUT));
        assert!(is_idempotent(&Method::DELETE));
        assert!(!is_idempotent(&Method::POST));
        assert!(!is_idempotent(&Method::PATCH));
    }

    #[test]
    fn retryable_statuses() {
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(is_retryable_status(StatusCode::BAD_GATEWAY));
        assert!(is_retryable_status(StatusCode::SERVICE_UNAVAILABLE));
        assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
        assert!(!is_retryable_status(StatusCode::UNAUTHORIZED));
        assert!(!is_retryable_status(StatusCode::OK));
    }

    #[test]
    fn backoff_delay_increases() {
        // Verify base schedule (jitter makes exact values non-deterministic).
        let d0 = backoff_delay(0);
        let d1 = backoff_delay(1);
        let d2 = backoff_delay(2);
        assert!(d0 >= Duration::from_millis(500));
        assert!(d1 >= Duration::from_millis(1000));
        assert!(d2 >= Duration::from_millis(2000));
    }
}
