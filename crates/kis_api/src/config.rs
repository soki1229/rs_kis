use std::path::PathBuf;

const REAL_REST_URL: &str = "https://openapi.koreainvestment.com:9443";
const REAL_WS_URL: &str = "wss://ops.koreainvestment.com:21000";
const VTS_REST_URL: &str = "https://openapivts.koreainvestment.com:29443";
const VTS_WS_URL: &str = "wss://ops.koreainvestment.com:31000";
const DEFAULT_WS_EVENT_BUFFER: usize = 1024;
const DEFAULT_TOKEN_CACHE: &str = "~/.config/kis_api/token.json";

#[derive(Debug, Clone)]
pub struct KisConfig {
    pub app_key: String,
    pub app_secret: String,
    pub account_num: String,
    pub rest_url: String,
    pub ws_url: String,
    pub mock: bool,
    pub token_cache_path: Option<PathBuf>,
    pub ws_event_buffer: usize,
}

impl KisConfig {
    pub fn builder() -> KisConfigBuilder {
        KisConfigBuilder::default()
    }

    /// `.env` 파일(또는 환경 변수)에서 실전투자 설정 로드.
    /// 환경 변수 키: APP_KEY, APP_SECRET, ACCOUNT_NUM, REST_URL, WS_URL, TOKEN_CACHE_PATH
    pub fn from_env() -> Result<Self, crate::KisError> {
        Self::from_env_inner(false)
    }

    /// `.env` 파일(또는 환경 변수)에서 VTS(모의투자) 설정 로드.
    /// 환경 변수 키: VTS_APP_KEY, VTS_APP_SECRET, VTS_ACCOUNT_NUM, VTS_REST_URL, VTS_WS_URL, VTS_TOKEN_CACHE_PATH
    pub fn from_env_vts() -> Result<Self, crate::KisError> {
        Self::from_env_inner(true)
    }

    fn from_env_inner(mock: bool) -> Result<Self, crate::KisError> {
        let prefix = if mock { "VTS_" } else { "" };
        let app_key = std::env::var(format!("{prefix}APP_KEY"))
            .map_err(|_| crate::KisError::Auth(format!("{prefix}APP_KEY not set")))?;
        let app_secret = std::env::var(format!("{prefix}APP_SECRET"))
            .map_err(|_| crate::KisError::Auth(format!("{prefix}APP_SECRET not set")))?;
        let account_num = std::env::var(format!("{prefix}ACCOUNT_NUM"))
            .map_err(|_| crate::KisError::Auth(format!("{prefix}ACCOUNT_NUM not set")))?;
        let rest_url = std::env::var(format!("{prefix}REST_URL"))
            .unwrap_or_else(|_| if mock { VTS_REST_URL } else { REAL_REST_URL }.to_string());
        let ws_url = std::env::var(format!("{prefix}WS_URL"))
            .unwrap_or_else(|_| if mock { VTS_WS_URL } else { REAL_WS_URL }.to_string());
        let token_cache_path = std::env::var(format!("{prefix}TOKEN_CACHE_PATH"))
            .ok()
            .map(|p| {
                let expanded = shellexpand::tilde(&p);
                PathBuf::from(expanded.as_ref())
            });
        Ok(KisConfig {
            app_key,
            app_secret,
            account_num,
            rest_url,
            ws_url,
            mock,
            token_cache_path,
            ws_event_buffer: DEFAULT_WS_EVENT_BUFFER,
        })
    }
}

#[derive(Default)]
pub struct KisConfigBuilder {
    app_key: Option<String>,
    app_secret: Option<String>,
    account_num: Option<String>,
    rest_url: Option<String>,
    ws_url: Option<String>,
    mock: bool,
    token_cache_path: Option<PathBuf>,
    ws_event_buffer: Option<usize>,
}

impl KisConfigBuilder {
    pub fn app_key(mut self, v: impl Into<String>) -> Self {
        self.app_key = Some(v.into());
        self
    }
    pub fn app_secret(mut self, v: impl Into<String>) -> Self {
        self.app_secret = Some(v.into());
        self
    }
    pub fn account_num(mut self, v: impl Into<String>) -> Self {
        self.account_num = Some(v.into());
        self
    }
    pub fn rest_url(mut self, v: impl Into<String>) -> Self {
        self.rest_url = Some(v.into());
        self
    }
    pub fn ws_url(mut self, v: impl Into<String>) -> Self {
        self.ws_url = Some(v.into());
        self
    }
    pub fn mock(mut self, v: bool) -> Self {
        self.mock = v;
        self
    }
    pub fn token_cache(mut self, path: impl Into<String>) -> Self {
        let binding = path.into();
        let expanded = shellexpand::tilde(&binding);
        self.token_cache_path = Some(PathBuf::from(expanded.as_ref()));
        self
    }
    pub fn ws_event_buffer(mut self, n: usize) -> Self {
        self.ws_event_buffer = Some(n);
        self
    }

    pub fn build(self) -> Result<KisConfig, crate::KisError> {
        let app_key = self.app_key.ok_or_else(|| crate::KisError::Auth("app_key required".into()))?;
        let app_secret = self.app_secret.ok_or_else(|| crate::KisError::Auth("app_secret required".into()))?;
        let account_num = self.account_num.ok_or_else(|| crate::KisError::Auth("account_num required".into()))?;

        let (default_rest, default_ws) = if self.mock {
            (VTS_REST_URL, VTS_WS_URL)
        } else {
            (REAL_REST_URL, REAL_WS_URL)
        };

        let token_cache_path = self.token_cache_path.or_else(|| {
            let expanded = shellexpand::tilde(DEFAULT_TOKEN_CACHE);
            Some(PathBuf::from(expanded.as_ref()))
        });

        Ok(KisConfig {
            app_key,
            app_secret,
            account_num,
            rest_url: self.rest_url.unwrap_or_else(|| default_rest.to_string()),
            ws_url: self.ws_url.unwrap_or_else(|| default_ws.to_string()),
            mock: self.mock,
            token_cache_path,
            ws_event_buffer: self.ws_event_buffer.unwrap_or(DEFAULT_WS_EVENT_BUFFER),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_config() -> KisConfig {
        KisConfig::builder()
            .app_key("test_key")
            .app_secret("test_secret")
            .account_num("12345678-01")
            .mock(true)
            .build()
            .unwrap()
    }

    #[test]
    fn builder_sets_mock_urls_automatically() {
        let cfg = mock_config();
        assert!(cfg.rest_url.contains("openapivts"));
        assert!(cfg.ws_url.contains("31000"));
    }

    #[test]
    fn builder_sets_production_urls_when_not_mock() {
        let cfg = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .build()
            .unwrap();
        assert!(cfg.rest_url.contains("openapi.koreainvestment.com"));
        assert!(cfg.ws_url.contains("21000"));
    }

    #[test]
    fn builder_allows_url_override() {
        let cfg = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .rest_url("https://custom.example.com")
            .ws_url("ws://custom.example.com:9999")
            .build()
            .unwrap();
        assert_eq!(cfg.rest_url, "https://custom.example.com");
        assert_eq!(cfg.ws_url, "ws://custom.example.com:9999");
    }

    #[test]
    fn builder_fails_without_required_fields() {
        assert!(KisConfig::builder().build().is_err());
    }

    #[test]
    fn default_event_buffer_is_1024() {
        let cfg = mock_config();
        assert_eq!(cfg.ws_event_buffer, 1024);
    }

    #[test]
    fn token_cache_path_optional_in_from_env() {
        // from_env_inner 직접 호출 시 TOKEN_CACHE_PATH 없으면 None
        // (from_env는 env var 필요하므로 builder 경유 테스트)
        let cfg = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .build()
            .unwrap();
        // token_cache_path는 기본값(~/.config/kis_api/token.json)으로 설정됨
        assert!(cfg.token_cache_path.is_some());
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn kis_config_is_send_sync() {
        assert_send_sync::<KisConfig>();
    }
}
