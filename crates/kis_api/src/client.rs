use crate::auth::{TokenRequest, TokenResponse};
use crate::endpoints;
use crate::error::{ApiResponseHeader, KisError};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[derive(Clone)]
pub struct KisClient {
    inner: Arc<Inner>,
}

struct Inner {
    access_token: RwLock<String>,
    token_expires_at: Mutex<Option<DateTime<Utc>>>,
    app_key: String,
    app_secret: String,
    client: Client,
    cache_path: Option<PathBuf>,
    env: KisEnv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KisEnv {
    Real,
    Vts,
}

#[derive(Serialize, Deserialize)]
struct TokenCache {
    access_token: String,
    expires_at: DateTime<Utc>,
}

impl KisClient {
    pub async fn new(app_key: &str, app_secret: &str, env: KisEnv) -> Result<Self, KisError> {
        Self::with_cache(app_key, app_secret, env, None).await
    }

    pub async fn with_cache(
        app_key: &str,
        app_secret: &str,
        env: KisEnv,
        cache_path: Option<PathBuf>,
    ) -> Result<Self, KisError> {
        let client = Client::builder()
            .user_agent("rs_kis/0.2 (Zero Boilerplate)")
            .build()?;

        let this = Self {
            inner: Arc::new(Inner {
                access_token: RwLock::new(String::new()),
                token_expires_at: Mutex::new(None),
                app_key: app_key.to_string(),
                app_secret: app_secret.to_string(),
                client,
                cache_path,
                env,
            }),
        };

        if let Some(path) = &this.inner.cache_path {
            if let Ok(cache_str) = std::fs::read_to_string(path) {
                if let Ok(cache) = serde_json::from_str::<TokenCache>(&cache_str) {
                    if cache.expires_at > Utc::now() + chrono::Duration::minutes(1) {
                        {
                            let mut token = this.inner.access_token.write().await;
                            *token = cache.access_token;
                        }
                        {
                            let mut expires = this.inner.token_expires_at.lock().await;
                            *expires = Some(cache.expires_at);
                        }
                        return Ok(this);
                    }
                }
            }
        }

        this.refresh_token().await?;
        Ok(this)
    }

    pub async fn refresh_token(&self) -> Result<(), KisError> {
        let base_url = match self.env() {
            KisEnv::Real => "https://openapi.koreainvestment.com:9443",
            KisEnv::Vts => "https://openapivts.koreainvestment.com:29443",
        };
        let url = format!("{}/oauth2/tokenP", base_url);
        let req = TokenRequest {
            grant_type: "client_credentials".to_string(),
            appkey: self.inner.app_key.clone(),
            appsecret: self.inner.app_secret.clone(),
        };

        let resp = self.inner.client.post(&url).json(&req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(KisError::Auth(format!("HTTP {}: {}", status, text)));
        }

        let resp_data: TokenResponse = resp.json().await?;
        let expires_at = Utc::now() + chrono::Duration::seconds(resp_data.expires_in as i64);
        {
            let mut token = self.inner.access_token.write().await;
            *token = resp_data.access_token.clone();
            let mut expires = self.inner.token_expires_at.lock().await;
            *expires = Some(expires_at);
        }
        if let Some(path) = &self.inner.cache_path {
            let cache = TokenCache {
                access_token: resp_data.access_token,
                expires_at,
            };
            if let Ok(cache_json) = serde_json::to_string(&cache) {
                let _ = std::fs::write(path, cache_json);
            }
        }
        Ok(())
    }

    pub fn stock(&self) -> endpoints::Stock {
        endpoints::Stock(self.clone())
    }

    pub fn overseas(&self) -> endpoints::Overseas {
        endpoints::Overseas(self.clone())
    }

    pub fn env(&self) -> KisEnv {
        self.inner.env
    }

    pub async fn approval_key(&self) -> Result<String, KisError> {
        let base_url = match self.env() {
            KisEnv::Real => "https://openapi.koreainvestment.com:9443",
            KisEnv::Vts => "https://openapivts.koreainvestment.com:29443",
        };
        let url = format!("{}/oauth2/Approval", base_url);
        let req = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": self.inner.app_key,
            "secretkey": self.inner.app_secret,
        });

        let resp = self.inner.client.post(&url).json(&req).send().await?;
        let data: serde_json::Value = resp.json().await?;
        data["approval_key"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| KisError::Auth("approval_key not found".to_string()))
    }

    pub fn ws_url(&self) -> &'static str {
        match self.env() {
            KisEnv::Real => crate::generated::config::REAL_WS_URL,
            KisEnv::Vts => crate::generated::config::VTS_WS_URL,
        }
    }

    pub fn app_key(&self) -> &str {
        &self.inner.app_key
    }

    pub async fn token_expires_at(&self) -> Option<DateTime<Utc>> {
        *self.inner.token_expires_at.lock().await
    }

    pub async fn post<R, B>(
        &self,
        path: &str,
        tr_id: &str,
        base_url: &str,
        body: B,
    ) -> Result<R, KisError>
    where
        R: for<'de> Deserialize<'de> + Default,
        B: Serialize,
    {
        if tr_id == "모의투자 미지원" {
            return Err(KisError::NotSupportedInVts);
        }
        let env_label = if self.env() == KisEnv::Real {
            "Real"
        } else {
            "VTS"
        };
        tracing::debug!(target: "kis_api", "[{}] POST {} (tr_id: {})", env_label, path, tr_id);

        let token = self.inner.access_token.read().await.clone();
        let url = format!("{}/{}", base_url, path.trim_start_matches('/'));

        let resp = self
            .inner
            .client
            .post(&url)
            .header("authorization", format!("Bearer {}", token))
            .header("appkey", &self.inner.app_key)
            .header("appsecret", &self.inner.app_secret)
            .header("tr_id", tr_id)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        let full_body: serde_json::Value = resp.json().await?;
        let header: ApiResponseHeader = serde_json::from_value(full_body.clone())?;

        if header.is_success() {
            // Success: Return the full JSON so the caller can pick output, output1, etc.
            Ok(serde_json::from_value(normalize_empty_obj_to_arr(
                full_body,
            ))?)
        } else {
            Err(header.to_error())
        }
    }

    pub async fn get<R, Q>(
        &self,
        path: &str,
        tr_id: &str,
        base_url: &str,
        query: Q,
    ) -> Result<R, KisError>
    where
        R: for<'de> Deserialize<'de> + Default,
        Q: Serialize,
    {
        if tr_id == "모의투자 미지원" {
            return Err(KisError::NotSupportedInVts);
        }
        let env_label = if self.env() == KisEnv::Real {
            "Real"
        } else {
            "VTS"
        };
        tracing::debug!(target: "kis_api", "[{}] GET {} (tr_id: {})", env_label, path, tr_id);

        let token = self.inner.access_token.read().await.clone();
        let url = format!("{}/{}", base_url, path.trim_start_matches('/'));

        let resp = self
            .inner
            .client
            .get(&url)
            .header("authorization", format!("Bearer {}", token))
            .header("appkey", &self.inner.app_key)
            .header("appsecret", &self.inner.app_secret)
            .header("tr_id", tr_id)
            .header("content-type", "application/json")
            .query(&query)
            .send()
            .await?;

        let full_body: serde_json::Value = resp.json().await?;
        let header: ApiResponseHeader = serde_json::from_value(full_body.clone())?;

        if header.is_success() {
            Ok(serde_json::from_value(normalize_empty_obj_to_arr(
                full_body,
            ))?)
        } else {
            Err(header.to_error())
        }
    }
}

/// KIS API 응답 정규화:
/// - `output*` 키 값이 `{}` (빈 객체) → `[]`
/// - `output*` 키 값이 `{...}` (단일 객체) → `[{...}]`
/// - 나머지는 재귀적으로 동일 처리
///
/// KIS API가 배열 대신 단일 객체 또는 빈 객체를 반환하는 두 가지 quirk를 모두 처리한다.
fn normalize_empty_obj_to_arr(v: serde_json::Value) -> serde_json::Value {
    match v {
        serde_json::Value::Object(map) => {
            let converted = map
                .into_iter()
                .map(|(k, v)| {
                    let v = if k.starts_with("output") {
                        match v {
                            serde_json::Value::Object(inner) => {
                                if inner.is_empty() {
                                    serde_json::Value::Array(vec![])
                                } else {
                                    serde_json::Value::Array(vec![serde_json::Value::Object(inner)])
                                }
                            }
                            other => normalize_empty_obj_to_arr(other),
                        }
                    } else {
                        normalize_empty_obj_to_arr(v)
                    };
                    (k, v)
                })
                .collect();
            serde_json::Value::Object(converted)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(normalize_empty_obj_to_arr).collect())
        }
        // KIS API가 숫자 필드를 " 0.00" 처럼 공백 포함 문자열로 반환하는 경우 trim
        serde_json::Value::String(s) => serde_json::Value::String(s.trim().to_string()),
        other => other,
    }
}
