use std::sync::Arc;

use reqwest::Client;
use async_trait::async_trait;

use crate::{KisConfig, KisError, KisStream};
use crate::auth::TokenManager;
use crate::rest::http::fetch_approval_key;
use crate::rest::overseas::inquiry::{balance, orders, profit};
use crate::rest::overseas::quote;
use crate::traits::{KisApi, KisEventSource};

struct Inner {
    config: KisConfig,
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

    /// 해외주식 주문
    pub async fn place_order(&self, req: crate::PlaceOrderRequest) -> Result<crate::PlaceOrderResponse, KisError> {
        let token = self.token().await?;
        crate::rest::overseas::order::place::place_order(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 정정/취소 주문
    pub async fn cancel_order(&self, req: crate::CancelOrderRequest) -> Result<crate::CancelOrderResponse, KisError> {
        let token = self.token().await?;
        crate::rest::overseas::order::cancel::cancel_order(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 잔고 조회
    pub async fn balance(&self) -> Result<crate::BalanceResponse, KisError> {
        let token = self.token().await?;
        balance::balance(self.http(), self.config(), &token).await
    }

    /// 해외주식 미체결 조회
    pub async fn unfilled_orders(&self) -> Result<Vec<crate::UnfilledOrder>, KisError> {
        let token = self.token().await?;
        orders::unfilled_orders(self.http(), self.config(), &token).await
    }

    /// 해외주식 체결내역 조회
    pub async fn order_history(&self, req: crate::OrderHistoryRequest) -> Result<Vec<crate::OrderHistoryItem>, KisError> {
        let token = self.token().await?;
        orders::order_history(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 기간손익 조회
    pub async fn period_profit(&self, req: crate::PeriodProfitRequest) -> Result<crate::PeriodProfitResponse, KisError> {
        let token = self.token().await?;
        profit::period_profit(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 매수가능금액 조회
    pub async fn buyable_amount(&self, req: crate::BuyableAmountRequest) -> Result<crate::BuyableAmountResponse, KisError> {
        let token = self.token().await?;
        profit::buyable_amount(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 현재가 조회
    pub async fn price(&self, symbol: &str, exchange: &crate::Exchange) -> Result<crate::PriceResponse, KisError> {
        let token = self.token().await?;
        quote::price::price(self.http(), self.config(), &token, symbol, exchange).await
    }

    /// 해외주식 호가 조회
    pub async fn orderbook(&self, symbol: &str, exchange: &crate::Exchange) -> Result<crate::OrderbookResponse, KisError> {
        let token = self.token().await?;
        quote::orderbook::orderbook(self.http(), self.config(), &token, symbol, exchange).await
    }

    /// 해외주식 일/주/월봉 조회
    pub async fn daily_chart(&self, req: crate::DailyChartRequest) -> Result<Vec<crate::CandleBar>, KisError> {
        let token = self.token().await?;
        quote::chart::daily_chart(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 분봉 조회
    pub async fn minute_chart(&self, req: crate::MinuteChartRequest) -> Result<Vec<crate::MinuteBar>, KisError> {
        let token = self.token().await?;
        quote::chart::minute_chart(self.http(), self.config(), &token, req).await
    }

    /// 해외주식 종목 검색
    pub async fn search(&self, keyword: &str) -> Result<Vec<crate::SearchResult>, KisError> {
        let token = self.token().await?;
        quote::search::search(self.http(), self.config(), &token, keyword).await
    }

    /// 해외주식 종목 정보 조회
    pub async fn symbol_info(&self, symbol: &str, exchange: &crate::Exchange) -> Result<crate::SymbolInfo, KisError> {
        let token = self.token().await?;
        quote::search::symbol_info(self.http(), self.config(), &token, symbol, exchange).await
    }

    /// 해외주식 뉴스 조회
    pub async fn news(&self, symbol: &str) -> Result<Vec<crate::NewsItem>, KisError> {
        let token = self.token().await?;
        quote::corporate::news(self.http(), self.config(), &token, symbol).await
    }

    /// 해외주식 배당 조회
    pub async fn dividend(&self, symbol: &str, exchange: &crate::Exchange) -> Result<Vec<crate::DividendItem>, KisError> {
        let token = self.token().await?;
        quote::corporate::dividend(self.http(), self.config(), &token, symbol, exchange).await
    }

    /// 해외주식 휴장일 조회
    pub async fn holidays(&self, country: &str) -> Result<Vec<crate::Holiday>, KisError> {
        let token = self.token().await?;
        quote::corporate::holidays(self.http(), self.config(), &token, country).await
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
