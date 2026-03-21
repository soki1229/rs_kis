use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{KisConfig, KisError};

/// KST(UTC+9) FixedOffset
pub fn current_kst() -> DateTime<FixedOffset> {
    let kst = FixedOffset::east_opt(9 * 3600).expect("KST offset valid");
    chrono::Utc::now().with_timezone(&kst)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenCache {
    access_token: String,
    expires_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    /// 예: "86400" (초)
    expires_in: Option<String>,
    /// 예: "2026-03-22 09:30:00" (KIS API가 반환하는 만료 시각 문자열)
    access_token_token_expired: Option<String>,
}

/// 토큰 자동 갱신 + 파일 캐시 관리
#[derive(Clone)]
pub struct TokenManager {
    inner: Arc<Inner>,
}

struct Inner {
    config: KisConfig,
    cache: RwLock<Option<TokenCache>>,
    http: reqwest::Client,
}

impl TokenManager {
    pub fn new(config: KisConfig) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                cache: RwLock::new(None),
                http: reqwest::Client::new(),
            }),
        }
    }

    /// 유효한 토큰 반환. 만료 또는 미존재 시 자동 갱신.
    pub async fn token(&self) -> Result<String, KisError> {
        // 1. 캐시에서 유효한 토큰 확인
        {
            let guard = self.inner.cache.read().await;
            if let Some(ref cached) = *guard {
                if cached.expires_at > current_kst() + chrono::Duration::minutes(5) {
                    return Ok(cached.access_token.clone());
                }
            }
        }

        // 2. 파일 캐시에서 로드 시도
        if let Some(path) = &self.inner.config.token_cache_path {
            if let Ok(token) = self.load_from_file(path).await {
                if token.expires_at > current_kst() + chrono::Duration::minutes(5) {
                    let t = token.access_token.clone();
                    *self.inner.cache.write().await = Some(token);
                    return Ok(t);
                }
            }
        }

        // 3. KIS API에서 신규 발급
        let token = self.fetch_token().await?;
        let t = token.access_token.clone();

        // 4. 파일 캐시 저장
        if let Some(path) = &self.inner.config.token_cache_path {
            let _ = self.save_to_file(path, &token).await;
        }

        *self.inner.cache.write().await = Some(token);
        Ok(t)
    }

    async fn fetch_token(&self) -> Result<TokenCache, KisError> {
        let cfg = &self.inner.config;
        let url = format!("{}/oauth2/tokenP", cfg.rest_url);

        let body = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": cfg.app_key,
            "appsecret": cfg.app_secret,
        });

        let resp = self.inner.http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(KisError::Network)?;

        if !resp.status().is_success() {
            return Err(KisError::Auth(format!("token fetch failed: {}", resp.status())));
        }

        let tr: TokenResponse = resp.json().await.map_err(KisError::Network)?;

        // 만료 시각 파싱: expires_in(초) 또는 access_token_token_expired 문자열 사용
        let expires_at = if let Some(exp_str) = tr.access_token_token_expired {
            // 형식: "2026-03-22 09:30:00"
            let kst = FixedOffset::east_opt(9 * 3600).unwrap();
            chrono::NaiveDateTime::parse_from_str(&exp_str, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| ndt.and_local_timezone(kst).unwrap())
                .unwrap_or_else(|_| current_kst() + chrono::Duration::hours(23))
        } else if let Some(secs_str) = tr.expires_in {
            let secs: i64 = secs_str.parse().unwrap_or(86400);
            current_kst() + chrono::Duration::seconds(secs)
        } else {
            current_kst() + chrono::Duration::hours(23)
        };

        Ok(TokenCache {
            access_token: tr.access_token,
            expires_at,
        })
    }

    async fn load_from_file(&self, path: &PathBuf) -> Result<TokenCache, KisError> {
        let data = tokio::fs::read_to_string(path).await.map_err(KisError::Io)?;
        serde_json::from_str(&data).map_err(KisError::Parse)
    }

    async fn save_to_file(&self, path: &PathBuf, token: &TokenCache) -> Result<(), KisError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(KisError::Io)?;
        }
        let data = serde_json::to_string_pretty(token).map_err(KisError::Parse)?;
        tokio::fs::write(path, data).await.map_err(KisError::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_kst_is_utc_plus_9() {
        let now = current_kst();
        assert_eq!(now.offset().local_minus_utc(), 9 * 3600);
    }

    #[test]
    fn token_manager_is_clone() {
        let config = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .build()
            .unwrap();
        let tm = TokenManager::new(config);
        let _tm2 = tm.clone();
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn token_manager_is_send_sync() {
        assert_send_sync::<TokenManager>();
    }
}
