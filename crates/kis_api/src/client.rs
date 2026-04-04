use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;

use crate::auth::{build_http_client, ApprovalKeyManager, TokenManager};
use crate::rest::overseas::analysis::{market, ranking};
use crate::rest::overseas::inquiry::{balance, orders, profit};
use crate::rest::overseas::quote;
use crate::rest::rate_limit::RateLimiter;
use crate::traits::KisApi;
use crate::{
    CancelOrderRequest, CancelOrderResponse, CandleBar, DailyChartRequest, Exchange, Holiday,
    KisConfig, KisError, KisStream, NewsItem, OrderHistoryItem, OrderHistoryRequest,
    PlaceOrderRequest, PlaceOrderResponse, RankingItem, UnfilledOrder,
};

/// Execute a REST call with automatic 401 → token refresh → retry (once).
///
/// Usage: `with_401_retry!(self, |token| { async_call(token).await })`
/// `token` is bound as `&str` inside the body.
macro_rules! with_401_retry {
    ($self:expr, |$token:ident| $body:expr) => {{
        let __tok = $self.throttled_token().await?;
        let $token: &str = &__tok;
        match $body {
            Err(KisError::Auth(_)) => {
                log::warn!("401 received — refreshing token and retrying");
                let __tok2 = $self.inner.token_manager.refresh().await?;
                let $token: &str = &__tok2;
                $body
            }
            other => other,
        }
    }};
}

/// Default REST rate limit: 15 requests/second (KIS allows ~20).
const DEFAULT_RATE_LIMIT: u32 = 15;

struct Inner {
    config: KisConfig,
    token_manager: TokenManager,
    approval_key_manager: ApprovalKeyManager,
    http: Client,
    rate_limiter: RateLimiter,
}

/// KIS REST API 클라이언트. `Clone`은 저렴 (`Arc` 복사).
#[derive(Clone)]
pub struct KisClient {
    inner: Arc<Inner>,
}

impl KisClient {
    /// 동기 생성자. `KisConfig::builder()` 로 설정 후 생성.
    /// A single `reqwest::Client` (with 30 s request / 10 s connect timeouts)
    /// is shared across the main client, `TokenManager`, and `ApprovalKeyManager`.
    pub fn new(config: KisConfig) -> Self {
        let http = build_http_client();
        let token_manager = TokenManager::with_http(config.clone(), http.clone());
        let approval_key_manager = ApprovalKeyManager::with_http(config.clone(), http.clone());
        Self {
            inner: Arc::new(Inner {
                config,
                token_manager,
                approval_key_manager,
                http,
                rate_limiter: RateLimiter::new(DEFAULT_RATE_LIMIT),
            }),
        }
    }

    /// Shared-component accessors — allow `KisDomesticClient` (or other
    /// consumers using the same app-key) to reuse the same `TokenManager`,
    /// `ApprovalKeyManager`, `reqwest::Client`, and `RateLimiter`.
    pub fn token_manager(&self) -> &TokenManager {
        &self.inner.token_manager
    }
    pub fn approval_key_manager(&self) -> &ApprovalKeyManager {
        &self.inner.approval_key_manager
    }
    pub fn shared_http(&self) -> &Client {
        &self.inner.http
    }
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.inner.rate_limiter
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

    /// Wait for rate-limit slot, then get a valid token. Used by every REST method.
    async fn throttled_token(&self) -> Result<String, KisError> {
        self.inner.rate_limiter.acquire().await;
        self.token().await
    }

    /// 해외주식 주문
    pub async fn place_order(
        &self,
        req: crate::PlaceOrderRequest,
    ) -> Result<crate::PlaceOrderResponse, KisError> {
        with_401_retry!(self, |token| {
            crate::rest::overseas::order::place::place_order(
                self.http(),
                self.config(),
                token,
                req.clone(),
            )
            .await
        })
    }

    /// 해외주식 정정/취소 주문
    pub async fn cancel_order(
        &self,
        req: crate::CancelOrderRequest,
    ) -> Result<crate::CancelOrderResponse, KisError> {
        with_401_retry!(self, |token| {
            crate::rest::overseas::order::cancel::cancel_order(
                self.http(),
                self.config(),
                token,
                req.clone(),
            )
            .await
        })
    }

    /// 해외주식 잔고 조회
    pub async fn balance(&self) -> Result<crate::BalanceResponse, KisError> {
        with_401_retry!(self, |token| {
            balance::balance(self.http(), self.config(), token).await
        })
    }

    /// 해외주식 미체결 조회
    pub async fn unfilled_orders(&self) -> Result<Vec<crate::UnfilledOrder>, KisError> {
        with_401_retry!(self, |token| {
            orders::unfilled_orders(self.http(), self.config(), token).await
        })
    }

    /// 해외주식 체결내역 조회
    pub async fn order_history(
        &self,
        req: crate::OrderHistoryRequest,
    ) -> Result<Vec<crate::OrderHistoryItem>, KisError> {
        with_401_retry!(self, |token| {
            orders::order_history(self.http(), self.config(), token, req.clone()).await
        })
    }

    /// 해외주식 기간손익 조회
    pub async fn period_profit(
        &self,
        req: crate::PeriodProfitRequest,
    ) -> Result<crate::PeriodProfitResponse, KisError> {
        with_401_retry!(self, |token| {
            profit::period_profit(self.http(), self.config(), token, req.clone()).await
        })
    }

    /// 해외주식 매수가능금액 조회
    pub async fn buyable_amount(
        &self,
        req: crate::BuyableAmountRequest,
    ) -> Result<crate::BuyableAmountResponse, KisError> {
        with_401_retry!(self, |token| {
            profit::buyable_amount(self.http(), self.config(), token, req.clone()).await
        })
    }

    /// 해외주식 현재가 조회
    pub async fn price(
        &self,
        symbol: &str,
        exchange: &crate::Exchange,
    ) -> Result<crate::PriceResponse, KisError> {
        with_401_retry!(self, |token| {
            quote::price::price(self.http(), self.config(), token, symbol, exchange).await
        })
    }

    /// 해외주식 호가 조회
    pub async fn orderbook(
        &self,
        symbol: &str,
        exchange: &crate::Exchange,
    ) -> Result<crate::OrderbookResponse, KisError> {
        with_401_retry!(self, |token| {
            quote::orderbook::orderbook(self.http(), self.config(), token, symbol, exchange).await
        })
    }

    /// 해외주식 일/주/월봉 조회
    pub async fn daily_chart(
        &self,
        req: crate::DailyChartRequest,
    ) -> Result<Vec<crate::CandleBar>, KisError> {
        with_401_retry!(self, |token| {
            quote::chart::daily_chart(self.http(), self.config(), token, req.clone()).await
        })
    }

    /// 해외주식 분봉 조회
    pub async fn minute_chart(
        &self,
        req: crate::MinuteChartRequest,
    ) -> Result<Vec<crate::MinuteBar>, KisError> {
        with_401_retry!(self, |token| {
            quote::chart::minute_chart(self.http(), self.config(), token, req.clone()).await
        })
    }

    /// 해외주식 종목 검색
    pub async fn search(&self, keyword: &str) -> Result<Vec<crate::SearchResult>, KisError> {
        with_401_retry!(self, |token| {
            quote::search::search(self.http(), self.config(), token, keyword).await
        })
    }

    /// 해외주식 종목 정보 조회
    pub async fn symbol_info(
        &self,
        symbol: &str,
        exchange: &crate::Exchange,
    ) -> Result<crate::SymbolInfo, KisError> {
        with_401_retry!(self, |token| {
            quote::search::symbol_info(self.http(), self.config(), token, symbol, exchange).await
        })
    }

    /// 해외주식 뉴스 조회
    pub async fn news(&self, symbol: &str) -> Result<Vec<crate::NewsItem>, KisError> {
        with_401_retry!(self, |token| {
            quote::corporate::news(self.http(), self.config(), token, symbol).await
        })
    }

    /// 해외주식 기간별권리(배당 포함) 조회
    /// `start_date`/`end_date`: YYYYMMDD
    pub async fn dividend(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<crate::DividendItem>, KisError> {
        with_401_retry!(self, |token| {
            quote::corporate::dividend(
                self.http(),
                self.config(),
                token,
                symbol,
                start_date,
                end_date,
            )
            .await
        })
    }

    /// 해외주식 휴장일 조회
    pub async fn holidays(&self, country: &str) -> Result<Vec<crate::Holiday>, KisError> {
        with_401_retry!(self, |token| {
            quote::corporate::holidays(self.http(), self.config(), token, country).await
        })
    }

    /// 해외주식 등락률 순위 조회
    pub async fn price_ranking(
        &self,
        req: crate::RankingRequest,
    ) -> Result<Vec<crate::RankingItem>, KisError> {
        with_401_retry!(self, |token| {
            ranking::price_ranking(self.http(), self.config(), token, req.clone()).await
        })
    }

    /// 해외주식 거래량 순위 조회
    pub async fn volume_ranking(
        &self,
        exchange: &crate::Exchange,
        count: u32,
    ) -> Result<Vec<crate::RankingItem>, KisError> {
        with_401_retry!(self, |token| {
            ranking::volume_ranking(self.http(), self.config(), token, exchange, count).await
        })
    }

    /// 해외주식 거래량 급증 순위 조회
    pub async fn volume_surge(
        &self,
        exchange: &crate::Exchange,
        count: u32,
    ) -> Result<Vec<crate::VolumeSurgeItem>, KisError> {
        with_401_retry!(self, |token| {
            ranking::volume_surge(self.http(), self.config(), token, exchange, count).await
        })
    }

    /// 해외주식 체결강도 순위 조회 (거래소 전체 순위)
    pub async fn volume_power(
        &self,
        exchange: &crate::Exchange,
        count: u32,
    ) -> Result<Vec<crate::VolumePowerItem>, KisError> {
        with_401_retry!(self, |token| {
            market::volume_power(self.http(), self.config(), token, exchange, count).await
        })
    }

    /// 해외주식 신고가/신저가 순위 조회
    pub async fn new_highlow(
        &self,
        exchange: &crate::Exchange,
        kind: &crate::HighLowKind,
        count: u32,
    ) -> Result<Vec<crate::NewHighLowItem>, KisError> {
        with_401_retry!(self, |token| {
            market::new_highlow(self.http(), self.config(), token, exchange, kind, count).await
        })
    }

    /// 해외주식 시가총액 순위 조회
    pub async fn market_cap(
        &self,
        exchange: &crate::Exchange,
        count: u32,
    ) -> Result<Vec<crate::MarketCapItem>, KisError> {
        with_401_retry!(self, |token| {
            market::market_cap(self.http(), self.config(), token, exchange, count).await
        })
    }

    /// 해외주식 거래회전율 순위 조회
    pub async fn trade_turnover(
        &self,
        exchange: &crate::Exchange,
        count: u32,
    ) -> Result<Vec<crate::TradeTurnoverItem>, KisError> {
        with_401_retry!(self, |token| {
            market::trade_turnover(self.http(), self.config(), token, exchange, count).await
        })
    }
}

#[async_trait]
impl KisApi for KisClient {
    async fn stream(&self) -> Result<KisStream, KisError> {
        let approval_key = self.inner.approval_key_manager.approval_key().await?;
        KisStream::connect(self.config().clone(), approval_key).await
    }

    async fn volume_ranking(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<RankingItem>, KisError> {
        self.volume_ranking(exchange, count).await
    }

    async fn holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError> {
        self.holidays(country).await
    }

    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError> {
        self.place_order(req).await
    }

    async fn cancel_order(&self, req: CancelOrderRequest) -> Result<CancelOrderResponse, KisError> {
        self.cancel_order(req).await
    }

    async fn daily_chart(&self, req: DailyChartRequest) -> Result<Vec<CandleBar>, KisError> {
        self.daily_chart(req).await
    }

    async fn unfilled_orders(&self) -> Result<Vec<UnfilledOrder>, KisError> {
        self.unfilled_orders().await
    }

    async fn news(&self, symbol: &str) -> Result<Vec<NewsItem>, KisError> {
        self.news(symbol).await
    }

    async fn order_history(
        &self,
        req: OrderHistoryRequest,
    ) -> Result<Vec<OrderHistoryItem>, KisError> {
        self.order_history(req).await
    }

    async fn balance(&self) -> Result<crate::BalanceResponse, KisError> {
        self.balance().await
    }

    async fn check_deposit(&self) -> Result<crate::DepositInfo, KisError> {
        with_401_retry!(self, |token| {
            crate::rest::overseas::deposit::check_deposit(self.http(), self.config(), token).await
        })
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

    #[test]
    fn shared_accessors_return_same_types() {
        let c = make_client();
        let _tm = c.token_manager();
        let _ak = c.approval_key_manager();
        let _http = c.shared_http();
        let _rl = c.rate_limiter();
    }
}
