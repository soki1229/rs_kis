use crate::{
    auth::TokenManager,
    rest::{
        domestic::{
            inquiry::{domestic_daily_chart, domestic_unfilled_orders, domestic_volume_ranking},
            order::{domestic_cancel_order, domestic_place_order},
            types::*,
        },
        http::fetch_approval_key,
    },
    CandleBar, Holiday, KisConfig, KisDomesticApi, KisError, KisStream,
};
use async_trait::async_trait;

/// 국내주식 전용 KIS 클라이언트
pub struct KisDomesticClient {
    config: KisConfig,
    token_manager: TokenManager,
    http: reqwest::Client,
}

impl KisDomesticClient {
    pub fn new(config: KisConfig) -> Self {
        let token_manager = TokenManager::new(config.clone());
        Self {
            config,
            token_manager,
            http: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl KisDomesticApi for KisDomesticClient {
    async fn domestic_stream(&self) -> Result<KisStream, KisError> {
        let approval_key = fetch_approval_key(&self.http, &self.config).await?;
        KisStream::connect(self.config.clone(), approval_key).await
    }

    async fn domestic_volume_ranking(
        &self,
        exchange: &DomesticExchange,
        count: u32,
    ) -> Result<Vec<DomesticRankingItem>, KisError> {
        let token = self.token_manager.token().await?;
        domestic_volume_ranking(&self.http, &self.config, &token, exchange, count).await
    }

    async fn domestic_holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError> {
        let token = self.token_manager.token().await?;
        crate::rest::overseas::quote::corporate::holidays(&self.http, &self.config, &token, country).await
    }

    async fn domestic_place_order(
        &self,
        req: DomesticPlaceOrderRequest,
    ) -> Result<DomesticPlaceOrderResponse, KisError> {
        let token = self.token_manager.token().await?;
        domestic_place_order(&self.http, &self.config, &token, req).await
    }

    async fn domestic_cancel_order(
        &self,
        req: DomesticCancelOrderRequest,
    ) -> Result<DomesticCancelOrderResponse, KisError> {
        let token = self.token_manager.token().await?;
        domestic_cancel_order(&self.http, &self.config, &token, req).await
    }

    async fn domestic_daily_chart(
        &self,
        req: DomesticDailyChartRequest,
    ) -> Result<Vec<CandleBar>, KisError> {
        let token = self.token_manager.token().await?;
        domestic_daily_chart(&self.http, &self.config, &token, req).await
    }

    async fn domestic_unfilled_orders(&self) -> Result<Vec<DomesticUnfilledOrder>, KisError> {
        let token = self.token_manager.token().await?;
        domestic_unfilled_orders(&self.http, &self.config, &token).await
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
