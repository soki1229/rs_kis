use crate::{
    BalanceResponse, CancelOrderRequest, CancelOrderResponse, CandleBar, DailyChartRequest,
    DepositInfo, Exchange, Holiday, KisError, KisStream, NewsItem, OrderHistoryItem,
    OrderHistoryRequest, PlaceOrderRequest, PlaceOrderResponse, RankingItem, UnfilledOrder,
};
use async_trait::async_trait;

/// REST + WebSocket 클라이언트 트레이트 (의존성 역전 / 테스트 모킹용)
#[async_trait]
pub trait KisApi: Send + Sync {
    /// WebSocket 스트림 생성
    async fn stream(&self) -> Result<KisStream, KisError>;

    /// 해외주식 거래량 순위 조회
    async fn volume_ranking(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<RankingItem>, KisError>;

    /// 해외 공휴일 조회 (country: "USA", "JPN" 등)
    async fn holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError>;

    /// 해외주식 매수/매도 주문 제출
    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError>;

    /// 해외주식 주문 정정/취소
    async fn cancel_order(&self, req: CancelOrderRequest) -> Result<CancelOrderResponse, KisError>;

    /// 해외주식 일/주/월봉 조회 (최대 요청 건수는 API 제한 따름)
    async fn daily_chart(&self, req: DailyChartRequest) -> Result<Vec<CandleBar>, KisError>;

    /// 설정된 계좌의 미체결 주문 전체 조회.
    /// KR/US 파이프라인이 각각 별도 KisConfig(별도 계좌)로 KisClient를 생성하므로
    /// 파라미터 없이 해당 계좌 전체 미체결을 반환한다.
    async fn unfilled_orders(&self) -> Result<Vec<UnfilledOrder>, KisError>;

    /// 해외주식 뉴스 조회 (`has_news_catalyst` 판단용)
    async fn news(&self, symbol: &str) -> Result<Vec<NewsItem>, KisError>;

    /// 해외주식 체결내역 조회 (poll_until_filled에서 체결 vs 취소 구분용)
    async fn order_history(
        &self,
        req: OrderHistoryRequest,
    ) -> Result<Vec<OrderHistoryItem>, KisError>;

    /// 해외주식 잔고 조회 (risk sizing용 account_balance 확보)
    async fn balance(&self) -> Result<BalanceResponse, KisError>;

    /// USD 예수금 조회 (주문가능 USD 현금 — risk sizing의 available cash)
    async fn check_deposit(&self) -> Result<DepositInfo, KisError>;
}

use crate::rest::domestic::{
    DomesticCancelOrderRequest, DomesticCancelOrderResponse, DomesticDailyChartRequest,
    DomesticExchange, DomesticOrderHistoryItem, DomesticOrderHistoryRequest,
    DomesticPlaceOrderRequest, DomesticPlaceOrderResponse, DomesticRankingItem,
    DomesticUnfilledOrder,
};

/// 국내주식 클라이언트 트레이트 (KOSPI/KOSDAQ)
#[async_trait]
pub trait KisDomesticApi: Send + Sync {
    /// 국내주식 실시간 WebSocket 스트림 (H0STCNT0 / H0STASP0 구독용)
    async fn domestic_stream(&self) -> Result<KisStream, KisError>;

    /// 국내주식 거래량 순위 조회
    async fn domestic_volume_ranking(
        &self,
        exchange: &DomesticExchange,
        count: u32,
    ) -> Result<Vec<DomesticRankingItem>, KisError>;

    /// 국내 휴장일 조회. `date`: 기준일자 YYYYMMDD (ex. "20260404").
    /// 해당 날짜가 개장일이 아니면 Holiday vec 반환, 개장일이면 빈 vec 반환.
    async fn domestic_holidays(&self, date: &str) -> Result<Vec<Holiday>, KisError>;

    /// 국내주식 매수/매도 주문
    async fn domestic_place_order(
        &self,
        req: DomesticPlaceOrderRequest,
    ) -> Result<DomesticPlaceOrderResponse, KisError>;

    /// 국내주식 주문 취소
    async fn domestic_cancel_order(
        &self,
        req: DomesticCancelOrderRequest,
    ) -> Result<DomesticCancelOrderResponse, KisError>;

    /// 국내주식 일/주/월봉 조회
    async fn domestic_daily_chart(
        &self,
        req: DomesticDailyChartRequest,
    ) -> Result<Vec<CandleBar>, KisError>;

    /// 국내주식 미체결 주문 전체 조회
    async fn domestic_unfilled_orders(&self) -> Result<Vec<DomesticUnfilledOrder>, KisError>;

    /// 국내주식 체결내역 조회 (poll_kr_until_filled에서 체결 vs 취소 구분용)
    async fn domestic_order_history(
        &self,
        req: DomesticOrderHistoryRequest,
    ) -> Result<Vec<DomesticOrderHistoryItem>, KisError>;

    /// 국내주식 잔고 조회 (예수금 총금액 → purchase_amount)
    async fn domestic_balance(&self) -> Result<BalanceResponse, KisError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kis_api_extended_is_object_safe() {
        // Box<dyn KisApi>가 컴파일되는지 확인 (volume_ranking + holidays 추가 후)
        let _: Option<Box<dyn KisApi>> = None;
    }

    #[test]
    fn kis_api_is_object_safe() {
        // Box<dyn KisApi> still compiles with all methods
        #[allow(dead_code)]
        struct MockNine;
        #[async_trait::async_trait]
        impl KisApi for MockNine {
            async fn stream(&self) -> Result<crate::KisStream, crate::KisError> {
                unimplemented!()
            }
            async fn volume_ranking(
                &self,
                _: &crate::Exchange,
                _: u32,
            ) -> Result<Vec<crate::RankingItem>, crate::KisError> {
                Ok(vec![])
            }
            async fn holidays(&self, _: &str) -> Result<Vec<crate::Holiday>, crate::KisError> {
                Ok(vec![])
            }
            async fn place_order(
                &self,
                _: crate::PlaceOrderRequest,
            ) -> Result<crate::PlaceOrderResponse, crate::KisError> {
                unimplemented!()
            }
            async fn cancel_order(
                &self,
                _: crate::CancelOrderRequest,
            ) -> Result<crate::CancelOrderResponse, crate::KisError> {
                unimplemented!()
            }
            async fn daily_chart(
                &self,
                _: crate::DailyChartRequest,
            ) -> Result<Vec<crate::CandleBar>, crate::KisError> {
                Ok(vec![])
            }
            async fn unfilled_orders(&self) -> Result<Vec<crate::UnfilledOrder>, crate::KisError> {
                Ok(vec![])
            }
            async fn news(&self, _: &str) -> Result<Vec<crate::NewsItem>, crate::KisError> {
                Ok(vec![])
            }
            async fn order_history(
                &self,
                _: crate::OrderHistoryRequest,
            ) -> Result<Vec<crate::OrderHistoryItem>, crate::KisError> {
                Ok(vec![])
            }
            async fn balance(&self) -> Result<crate::BalanceResponse, crate::KisError> {
                Ok(crate::BalanceResponse {
                    items: vec![],
                    summary: crate::BalanceSummary {
                        purchase_amount: rust_decimal::Decimal::ZERO,
                        realized_pnl: rust_decimal::Decimal::ZERO,
                        total_pnl: rust_decimal::Decimal::ZERO,
                    },
                })
            }
            async fn check_deposit(&self) -> Result<crate::DepositInfo, crate::KisError> {
                Ok(crate::DepositInfo {
                    currency: "USD".to_string(),
                    amount: rust_decimal::Decimal::ZERO,
                })
            }
        }
        let _: Option<Box<dyn KisApi>> = None;
    }

    #[test]
    fn kis_domestic_api_is_object_safe() {
        #[allow(dead_code)]
        struct MockDomestic;

        #[async_trait::async_trait]
        impl KisDomesticApi for MockDomestic {
            async fn domestic_stream(&self) -> Result<KisStream, KisError> {
                unimplemented!()
            }
            async fn domestic_volume_ranking(
                &self,
                _: &DomesticExchange,
                _: u32,
            ) -> Result<Vec<DomesticRankingItem>, KisError> {
                Ok(vec![])
            }
            async fn domestic_holidays(&self, _: &str) -> Result<Vec<Holiday>, KisError> {
                Ok(vec![])
            }
            async fn domestic_place_order(
                &self,
                _: DomesticPlaceOrderRequest,
            ) -> Result<DomesticPlaceOrderResponse, KisError> {
                unimplemented!()
            }
            async fn domestic_cancel_order(
                &self,
                _: DomesticCancelOrderRequest,
            ) -> Result<DomesticCancelOrderResponse, KisError> {
                unimplemented!()
            }
            async fn domestic_daily_chart(
                &self,
                _: DomesticDailyChartRequest,
            ) -> Result<Vec<CandleBar>, KisError> {
                Ok(vec![])
            }
            async fn domestic_unfilled_orders(
                &self,
            ) -> Result<Vec<DomesticUnfilledOrder>, KisError> {
                Ok(vec![])
            }
            async fn domestic_order_history(
                &self,
                _: DomesticOrderHistoryRequest,
            ) -> Result<Vec<DomesticOrderHistoryItem>, KisError> {
                Ok(vec![])
            }
            async fn domestic_balance(&self) -> Result<BalanceResponse, KisError> {
                Ok(BalanceResponse {
                    items: vec![],
                    summary: crate::BalanceSummary {
                        purchase_amount: rust_decimal::Decimal::ZERO,
                        realized_pnl: rust_decimal::Decimal::ZERO,
                        total_pnl: rust_decimal::Decimal::ZERO,
                    },
                })
            }
        }

        let _: Option<Box<dyn KisDomesticApi>> = None;
    }
}
