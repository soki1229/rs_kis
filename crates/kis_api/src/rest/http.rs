use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{KisConfig, KisError};

/// KIS REST API 응답 공통 래퍼
#[derive(Debug, serde::Deserialize)]
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

/// KIS REST API 실행기
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

    let mut req = client
        .request(params.method, &url)
        .header("authorization", format!("Bearer {token}"))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", params.tr_id)
        .header("content-type", "application/json; charset=utf-8")
        .header("accept", "text/plain");

    if let Some(query) = params.query {
        // Value::Object → query string
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

    let resp = req.send().await.map_err(KisError::Network)?;

    let status = resp.status();
    let text = resp.text().await.map_err(KisError::Network)?;

    if status == StatusCode::UNAUTHORIZED {
        return Err(KisError::Auth("401 Unauthorized — token expired?".into()));
    }

    if !status.is_success() {
        // KIS가 에러 응답도 JSON으로 반환하는 경우
        if let Ok(v) = serde_json::from_str::<Value>(&text) {
            let code = v["msg_cd"].as_str().unwrap_or("unknown").to_string();
            let msg = v["msg1"].as_str().unwrap_or(&text).to_string();
            return Err(KisError::Api { code, message: msg });
        }
        return Err(KisError::Api {
            code: status.as_str().to_string(),
            message: text,
        });
    }

    // 응답 JSON 파싱
    let v: Value = serde_json::from_str(&text).map_err(KisError::Parse)?;

    // rt_cd가 "0"이 아닌 경우 API 에러
    if let Some(rt_cd) = v["rt_cd"].as_str() {
        if rt_cd != "0" {
            let code = v["msg_cd"].as_str().unwrap_or("unknown").to_string();
            let msg = v["msg1"].as_str().unwrap_or("unknown").to_string();
            return Err(KisError::Api { code, message: msg });
        }
    }

    // T로 역직렬화
    serde_json::from_value::<T>(v).map_err(KisError::Parse)
}

/// 승인키(approval key) 발급 — WebSocket 인증에 사용
pub async fn fetch_approval_key(
    client: &Client,
    config: &KisConfig,
) -> Result<String, KisError> {
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
        return Err(KisError::Auth(format!("approval key fetch failed: {}", resp.status())));
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
        fn assert_execute_signature<T: serde::de::DeserializeOwned>()
        where
            T: serde::de::DeserializeOwned,
        {
            let _f = execute::<T>;
            let _ = _f; // suppress unused warning
        }
        assert_execute_signature::<Value>();
    }
}
