use crate::auth::{TokenRequest, TokenResponse};
use crate::endpoints;
use crate::error::{ApiResponse, KisError};
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
    base_url: String,
    cache_path: Option<PathBuf>,
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

        let base_url = match env {
            KisEnv::Vts => "https://openapivts.koreainvestment.com:29443".to_string(),
            KisEnv::Real => "https://openapi.koreainvestment.com:8080".to_string(),
        };

        let this = Self {
            inner: Arc::new(Inner {
                access_token: RwLock::new(String::new()),
                token_expires_at: Mutex::new(None),
                app_key: app_key.to_string(),
                app_secret: app_secret.to_string(),
                client,
                base_url,
                cache_path,
            }),
        };

        // 1. 캐시 파일에서 로드 시도
        if let Some(path) = &this.inner.cache_path {
            if let Ok(cache_str) = std::fs::read_to_string(path) {
                if let Ok(cache) = serde_json::from_str::<TokenCache>(&cache_str) {
                    // 만료 시각이 1분 이상 남았을 때만 재사용
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

        // 2. 캐시가 없거나 만료된 경우 새로 발급
        this.refresh_token().await?;
        Ok(this)
    }

    pub async fn refresh_token(&self) -> Result<(), KisError> {
        let url = format!("{}/oauth2/tokenP", self.inner.base_url);
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

        let body_text = resp.text().await?;
        let resp_data: TokenResponse = match serde_json::from_str(&body_text) {
            Ok(data) => data,
            Err(e) => {
                return Err(KisError::Auth(format!(
                    "Failed to parse token response: {}. Raw body: {}",
                    e, body_text
                )));
            }
        };

        let expires_at = Utc::now() + chrono::Duration::seconds(resp_data.expires_in as i64);

        // 메모리 업데이트
        {
            let mut token = self.inner.access_token.write().await;
            *token = resp_data.access_token.clone();
            let mut expires = self.inner.token_expires_at.lock().await;
            *expires = Some(expires_at);
        }

        // 파일 캐시 업데이트
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
        if self.inner.base_url.contains("openapivts") {
            KisEnv::Vts
        } else {
            KisEnv::Real
        }
    }

    /// WebSocket 연결에 필요한 approval_key 발급.
    pub async fn approval_key(&self) -> Result<String, KisError> {
        let url = format!("{}/oauth2/Approval", self.inner.base_url);
        let req = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": self.inner.app_key,
            "secretkey": self.inner.app_secret,
        });

        let resp = self.inner.client.post(&url).json(&req).send().await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(KisError::Auth(format!("Approval key failed ({}): {}", status, text)));
        }

        let json: serde_json::Value = resp.json().await?;
        json["approval_key"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| KisError::Auth("approval_key not found in response".to_string()))
    }

    /// 이 클라이언트 환경에 맞는 WebSocket URL 반환.
    pub fn ws_url(&self) -> &'static str {
        match self.env() {
            KisEnv::Real => "wss://ops.koreainvestment.com:21000",
            KisEnv::Vts => "wss://ops.koreainvestment.com:31000",
        }
    }

    /// WS 구독 메시지 헤더용 app_key 반환.
    pub fn app_key(&self) -> &str {
        &self.inner.app_key
    }


    pub async fn post<R, B>(&self, path: &str, tr_id: &str, body: B) -> Result<R, KisError>
    where
        R: for<'de> Deserialize<'de> + Default,
        B: Serialize,
    {
        let token = self.inner.access_token.read().await.clone();
        let url = format!("{}/{}", self.inner.base_url, path.trim_start_matches('/'));

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

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(KisError::Http {
                status,
                message: text,
            });
        }

        let json: ApiResponse<R> = resp.json().await?;
        json.into_result()
    }

    pub async fn get<R, Q>(&self, path: &str, tr_id: &str, query: Q) -> Result<R, KisError>
    where
        R: for<'de> Deserialize<'de> + Default,
        Q: Serialize,
    {
        let token = self.inner.access_token.read().await.clone();
        let url = format!("{}/{}", self.inner.base_url, path.trim_start_matches('/'));

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

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(KisError::Http {
                status,
                message: text,
            });
        }

        let json: ApiResponse<R> = resp.json().await?;
        json.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_url_real_env() {
        let inner = Arc::new(Inner {
            access_token: RwLock::new(String::new()),
            token_expires_at: Mutex::new(None),
            app_key: "key".to_string(),
            app_secret: "secret".to_string(),
            client: Client::new(),
            base_url: "https://openapi.koreainvestment.com:8080".to_string(),
            cache_path: None,
        });
        let client = KisClient { inner };
        assert_eq!(client.ws_url(), "wss://ops.koreainvestment.com:21000");
    }

    #[test]
    fn ws_url_vts_env() {
        let inner = Arc::new(Inner {
            access_token: RwLock::new(String::new()),
            token_expires_at: Mutex::new(None),
            app_key: "key".to_string(),
            app_secret: "secret".to_string(),
            client: Client::new(),
            base_url: "https://openapivts.koreainvestment.com:29443".to_string(),
            cache_path: None,
        });
        let client = KisClient { inner };
        assert_eq!(client.ws_url(), "wss://ops.koreainvestment.com:31000");
    }
}
