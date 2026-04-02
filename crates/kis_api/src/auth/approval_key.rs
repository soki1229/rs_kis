use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::current_kst;
use crate::rest::http::fetch_approval_key;
use crate::{KisConfig, KisError};

/// KIS approval_key does not include an explicit expiry in the API response.
/// Based on KIS documentation, the key is valid for ~24 hours.
/// We use a conservative TTL of 12 hours to ensure timely refresh.
const APPROVAL_KEY_TTL_HOURS: i64 = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApprovalKeyCache {
    approval_key: String,
    expires_at: DateTime<FixedOffset>,
}

/// Manages approval_key lifecycle with in-memory and file caching.
/// Mirrors the same 3-stage strategy as `TokenManager`.
#[derive(Clone)]
pub struct ApprovalKeyManager {
    inner: Arc<Inner>,
}

struct Inner {
    config: KisConfig,
    cache: RwLock<Option<ApprovalKeyCache>>,
    http: reqwest::Client,
}

impl ApprovalKeyManager {
    pub fn new(config: KisConfig) -> Self {
        Self::with_http(config, super::build_http_client())
    }

    /// Create with an externally-provided `reqwest::Client` (shared instance).
    pub fn with_http(config: KisConfig, http: reqwest::Client) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                cache: RwLock::new(None),
                http,
            }),
        }
    }

    /// Returns a valid approval_key, refreshing only when expired or missing.
    pub async fn approval_key(&self) -> Result<String, KisError> {
        // 1. Check in-memory cache
        {
            let guard = self.inner.cache.read().await;
            if let Some(ref cached) = *guard {
                if cached.expires_at > current_kst() + chrono::Duration::minutes(5) {
                    return Ok(cached.approval_key.clone());
                }
            }
        }

        // 2. Try file cache (reuse the token_cache_path directory with _approval suffix)
        if let Some(path) = self.cache_file_path() {
            if let Ok(cached) = self.load_from_file(&path).await {
                if cached.expires_at > current_kst() + chrono::Duration::minutes(5) {
                    let key = cached.approval_key.clone();
                    *self.inner.cache.write().await = Some(cached);
                    return Ok(key);
                }
            }
        }

        // 3. Fetch from KIS API
        let key = fetch_approval_key(&self.inner.http, &self.inner.config).await?;
        let cached = ApprovalKeyCache {
            approval_key: key.clone(),
            expires_at: current_kst() + chrono::Duration::hours(APPROVAL_KEY_TTL_HOURS),
        };

        // Save to file cache
        if let Some(path) = self.cache_file_path() {
            let _ = self.save_to_file(&path, &cached).await;
        }

        *self.inner.cache.write().await = Some(cached);
        Ok(key)
    }

    /// Derives the approval_key cache file path from token_cache_path.
    /// e.g., `~/.kis_vts_token.json` → `~/.kis_vts_approval_key.json`
    fn cache_file_path(&self) -> Option<PathBuf> {
        self.inner.config.token_cache_path.as_ref().map(|p| {
            let stem = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("kis_token");
            let parent = p.parent().unwrap_or(p.as_path());
            parent.join(format!("{}_approval_key.json", stem))
        })
    }

    async fn load_from_file(&self, path: &PathBuf) -> Result<ApprovalKeyCache, KisError> {
        let data = tokio::fs::read_to_string(path)
            .await
            .map_err(KisError::Io)?;
        serde_json::from_str(&data).map_err(KisError::Parse)
    }

    async fn save_to_file(
        &self,
        path: &PathBuf,
        cached: &ApprovalKeyCache,
    ) -> Result<(), KisError> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(KisError::Io)?;
        }
        let data = serde_json::to_string_pretty(cached).map_err(KisError::Parse)?;
        tokio::fs::write(path, data).await.map_err(KisError::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_key_manager_is_clone() {
        let config = KisConfig::builder()
            .app_key("k")
            .app_secret("s")
            .account_num("a")
            .build()
            .unwrap();
        let mgr = ApprovalKeyManager::new(config);
        let _mgr2 = mgr.clone();
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn approval_key_manager_is_send_sync() {
        assert_send_sync::<ApprovalKeyManager>();
    }

    #[test]
    fn cache_file_path_derives_correctly() {
        let config = KisConfig::builder()
            .app_key("k")
            .app_secret("s")
            .account_num("a")
            .token_cache("/tmp/.kis_vts_token.json")
            .build()
            .unwrap();
        let mgr = ApprovalKeyManager::new(config);
        let path = mgr.cache_file_path().unwrap();
        assert_eq!(
            path.to_str().unwrap(),
            "/tmp/.kis_vts_token_approval_key.json"
        );
    }

    #[test]
    fn cache_file_path_present_with_default_config() {
        let config = KisConfig::builder()
            .app_key("k")
            .app_secret("s")
            .account_num("a")
            .build()
            .unwrap();
        let mgr = ApprovalKeyManager::new(config);
        let path = mgr.cache_file_path().unwrap();
        assert!(path.to_str().unwrap().contains("approval_key"));
    }
}
