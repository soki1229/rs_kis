use rust_decimal::Decimal;
use serde::Deserialize;

/// 국내주식 거래소 구분
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomesticExchange {
    /// 한국거래소 유가증권 (KOSPI)
    KOSPI,
    /// 한국거래소 코스닥 (KOSDAQ)
    KOSDAQ,
}

impl DomesticExchange {
    /// KIS API 거래소 코드 반환
    pub fn as_str(&self) -> &'static str {
        match self {
            DomesticExchange::KOSPI => "KSC",
            DomesticExchange::KOSDAQ => "KSQ",
        }
    }

    /// KIS 주문 시장분류코드 (국내주식 주문 API 파라미터)
    pub fn market_code(&self) -> &'static str {
        match self {
            DomesticExchange::KOSPI => "J",  // 장내
            DomesticExchange::KOSDAQ => "Q", // 코스닥
        }
    }
}

/// 국내주식 주문 유형
#[derive(Debug, Clone, Copy)]
pub enum DomesticOrderType {
    /// 지정가
    Limit,
    /// 시장가
    Market,
}

impl DomesticOrderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DomesticOrderType::Limit => "00",   // KIS 지정가 코드
            DomesticOrderType::Market => "01",  // KIS 시장가 코드
        }
    }
}

/// 국내주식 주문 요청
#[derive(Debug, Clone)]
pub struct DomesticPlaceOrderRequest {
    pub symbol: String,
    pub exchange: DomesticExchange,
    pub side: crate::OrderSide,
    pub order_type: DomesticOrderType,
    /// 정수 주수 (국내는 소수점 없음)
    pub qty: u32,
    pub price: Option<Decimal>,
}

/// 국내주식 주문 응답
#[derive(Debug, Clone, Deserialize)]
pub struct DomesticPlaceOrderResponse {
    pub order_no: String,
    pub order_date: String,
    pub order_time: String,
}

/// 국내주식 취소 요청
#[derive(Debug, Clone)]
pub struct DomesticCancelOrderRequest {
    pub symbol: String,
    pub exchange: DomesticExchange,
    pub original_order_no: String,
    pub qty: u32,
    pub price: Option<Decimal>,
}

/// 국내주식 취소 응답
#[derive(Debug, Clone, Deserialize)]
pub struct DomesticCancelOrderResponse {
    pub order_no: String,
    pub order_date: String,
    pub order_time: String,
}

/// 국내주식 미체결 주문
#[derive(Debug, Clone)]
pub struct DomesticUnfilledOrder {
    pub order_no: String,
    pub symbol: String,
    pub exchange: String,
    /// "01" = 매수, "02" = 매도
    pub side_cd: String,
    pub qty: u32,
    pub price: Decimal,
    pub filled_qty: u32,
    pub remaining_qty: u32,
}

/// 국내주식 거래량 순위 항목
#[derive(Debug, Clone)]
pub struct DomesticRankingItem {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub price: Decimal,
    pub volume: Decimal,
}

/// 국내주식 일봉 조회 요청
#[derive(Debug, Clone)]
pub struct DomesticDailyChartRequest {
    pub symbol: String,
    pub period: crate::ChartPeriod,
    /// 수정주가 사용 여부
    pub adj_price: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domestic_exchange_market_codes() {
        assert_eq!(DomesticExchange::KOSPI.market_code(), "J");
        assert_eq!(DomesticExchange::KOSDAQ.market_code(), "Q");
    }

    #[test]
    fn domestic_order_type_codes() {
        assert_eq!(DomesticOrderType::Limit.as_str(), "00");
        assert_eq!(DomesticOrderType::Market.as_str(), "01");
    }
}
