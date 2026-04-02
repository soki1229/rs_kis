use crate::{
    auth::{build_http_client, ApprovalKeyManager, TokenManager},
    rest::domestic::{
        inquiry::{
            domestic_balance, domestic_daily_chart, domestic_order_history,
            domestic_unfilled_orders, domestic_volume_ranking,
        },
        order::{domestic_cancel_order, domestic_place_order},
        types::*,
    },
    rest::rate_limit::RateLimiter,
    BalanceResponse, CandleBar, Holiday, KisConfig, KisDomesticApi, KisError, KisStream,
};
use async_trait::async_trait;

const DEFAULT_RATE_LIMIT: u32 = 15;

/// 국내주식 전용 KIS 클라이언트
pub struct KisDomesticClient {
    config: KisConfig,
    token_manager: TokenManager,
    approval_key_manager: ApprovalKeyManager,
    http: reqwest::Client,
    rate_limiter: RateLimiter,
}

impl KisDomesticClient {
    pub fn new(config: KisConfig) -> Self {
        let http = build_http_client();
        let token_manager = TokenManager::with_http(config.clone(), http.clone());
        let approval_key_manager = ApprovalKeyManager::with_http(config.clone(), http.clone());
        Self {
            config,
            token_manager,
            approval_key_manager,
            http,
            rate_limiter: RateLimiter::new(DEFAULT_RATE_LIMIT),
        }
    }

    async fn throttled_token(&self) -> Result<String, KisError> {
        self.rate_limiter.acquire().await;
        self.token_manager.token().await
    }
}

#[async_trait]
impl KisDomesticApi for KisDomesticClient {
    async fn domestic_stream(&self) -> Result<KisStream, KisError> {
        let approval_key = self.approval_key_manager.approval_key().await?;
        KisStream::connect(self.config.clone(), approval_key).await
    }

    async fn domestic_volume_ranking(
        &self,
        exchange: &DomesticExchange,
        count: u32,
    ) -> Result<Vec<DomesticRankingItem>, KisError> {
        let token = self.throttled_token().await?;
        domestic_volume_ranking(&self.http, &self.config, &token, exchange, count).await
    }

    async fn domestic_holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError> {
        let tok = self.throttled_token().await?;
        let result = crate::rest::overseas::quote::corporate::holidays(
            &self.http,
            &self.config,
            &tok,
            country,
        )
        .await;
        match result {
            Err(KisError::Auth(_)) => {
                log::warn!("401 received — refreshing token and retrying");
                let tok2 = self.token_manager.refresh().await?;
                crate::rest::overseas::quote::corporate::holidays(
                    &self.http,
                    &self.config,
                    &tok2,
                    country,
                )
                .await
            }
            other => other,
        }
    }

    async fn domestic_place_order(
        &self,
        req: DomesticPlaceOrderRequest,
    ) -> Result<DomesticPlaceOrderResponse, KisError> {
        let token = self.throttled_token().await?;
        domestic_place_order(&self.http, &self.config, &token, req).await
    }

    async fn domestic_cancel_order(
        &self,
        req: DomesticCancelOrderRequest,
    ) -> Result<DomesticCancelOrderResponse, KisError> {
        let token = self.throttled_token().await?;
        domestic_cancel_order(&self.http, &self.config, &token, req).await
    }

    async fn domestic_daily_chart(
        &self,
        req: DomesticDailyChartRequest,
    ) -> Result<Vec<CandleBar>, KisError> {
        let token = self.throttled_token().await?;
        domestic_daily_chart(&self.http, &self.config, &token, req).await
    }

    async fn domestic_unfilled_orders(&self) -> Result<Vec<DomesticUnfilledOrder>, KisError> {
        let token = self.throttled_token().await?;
        domestic_unfilled_orders(&self.http, &self.config, &token).await
    }

    async fn domestic_order_history(
        &self,
        req: DomesticOrderHistoryRequest,
    ) -> Result<Vec<DomesticOrderHistoryItem>, KisError> {
        let token = self.throttled_token().await?;
        domestic_order_history(&self.http, &self.config, &token, req).await
    }

    async fn domestic_balance(&self) -> Result<BalanceResponse, KisError> {
        let token = self.throttled_token().await?;
        domestic_balance(&self.http, &self.config, &token).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_client() -> KisDomesticClient {
        let config = KisConfig::builder()
            .app_key("test_key")
            .app_secret("test_secret")
            .account_num("50123456-01")
            .mock(true)
            .build()
            .unwrap();
        KisDomesticClient::new(config)
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn domestic_client_is_send_sync() {
        assert_send_sync::<KisDomesticClient>();
    }

    #[test]
    fn domestic_client_implements_trait() {
        fn accepts_domestic_api(_: &impl KisDomesticApi) {}
        let c = make_client();
        accepts_domestic_api(&c);
    }

    #[test]
    fn domestic_place_order_request_builds() {
        let req = DomesticPlaceOrderRequest {
            symbol: "005930".into(),
            exchange: DomesticExchange::KOSPI,
            side: crate::OrderSide::Buy,
            order_type: DomesticOrderType::Limit,
            qty: 10,
            price: Some(rust_decimal_macros::dec!(70000)),
        };
        assert_eq!(req.symbol, "005930");
        assert_eq!(req.exchange.market_code(), "J");
    }
}
