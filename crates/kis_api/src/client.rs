use std::sync::Arc;

use reqwest::Client;
use async_trait::async_trait;

use crate::{KisConfig, KisError, KisStream};
use crate::auth::TokenManager;
use crate::rest::http::fetch_approval_key;
use crate::traits::{KisApi, KisEventSource};

struct Inner {
    config: KisConfig,
    #[allow(dead_code)] // Plan 3a에서 REST 엔드포인트 추가 시 사용
    token_manager: TokenManager,
    http: Client,
}

/// KIS REST API 클라이언트. `Clone`은 저렴 (`Arc` 복사).
#[derive(Clone)]
pub struct KisClient {
    inner: Arc<Inner>,
}

impl KisClient {
    /// 동기 생성자. `KisConfig::builder()` 로 설정 후 생성.
    pub fn new(config: KisConfig) -> Self {
        let token_manager = TokenManager::new(config.clone());
        Self {
            inner: Arc::new(Inner {
                config,
                token_manager,
                http: Client::new(),
            }),
        }
    }

    /// 현재 유효한 액세스 토큰 반환 (내부 헬퍼)
    #[allow(dead_code)] // Plan 3a에서 REST 엔드포인트 추가 시 사용
    pub(crate) async fn token(&self) -> Result<String, KisError> {
        self.inner.token_manager.token().await
    }

    /// reqwest Client 참조 (내부 헬퍼)
    pub(crate) fn http(&self) -> &Client {
        &self.inner.http
    }

    /// KisConfig 참조 (내부 헬퍼)
    pub(crate) fn config(&self) -> &KisConfig {
        &self.inner.config
    }
}

#[async_trait]
impl KisApi for KisClient {
    async fn stream(&self) -> Result<KisStream, KisError> {
        let approval_key = fetch_approval_key(self.http(), self.config()).await?;
        KisStream::connect(self.config().clone(), approval_key).await
    }
}

#[async_trait]
impl KisEventSource for KisClient {
    async fn event_stream(&self) -> Result<KisStream, KisError> {
        self.stream().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_client() -> KisClient {
        let config = KisConfig::builder()
            .app_key("test_key")
            .app_secret("test_secret")
            .account_num("12345678-01")
            .mock(true)
            .build()
            .unwrap();
        KisClient::new(config)
    }

    #[test]
    fn client_is_clone() {
        let c = make_client();
        let _c2 = c.clone();
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn client_is_send_sync() {
        assert_send_sync::<KisClient>();
    }

    #[test]
    fn client_implements_kis_api() {
        fn accepts_kis_api(_: &impl KisApi) {}
        let c = make_client();
        accepts_kis_api(&c);
    }
}
