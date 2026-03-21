use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub qty: i64,
    pub avg_price: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: Side,
    pub qty: u64,
    pub price: Option<Decimal>, // None = 시장가
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderResult {
    pub order_id: String,
    pub symbol: String,
    pub side: Side,
    pub qty: u64,
    pub price: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FillInfo {
    pub order_id: String,
    pub symbol: String,
    pub filled_qty: u64,
    pub filled_price: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PnLReport {
    pub realized: Decimal,
    pub unrealized: Decimal,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn decimal_avoids_float_rounding_error() {
        // f64 누적 오류 재현
        let f: f64 = 0.1 + 0.2;
        assert_ne!(f, 0.3); // f64는 0.3이 아님!

        // Decimal은 정확
        let d = dec!(0.1) + dec!(0.2);
        assert_eq!(d, dec!(0.3));
    }

    #[test]
    fn position_pnl_calculation_is_exact() {
        // 매수 100주 @ $134.20, 현재가 $135.50
        let avg = dec!(134.20);
        let current = dec!(135.50);
        let qty = Decimal::from(100u64);
        let unrealized = (current - avg) * qty;
        assert_eq!(unrealized, dec!(130.00));
    }

    #[test]
    fn order_request_market_order_has_no_price() {
        let req = OrderRequest {
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: None,
        };
        assert!(req.price.is_none());
    }

    #[test]
    fn order_request_limit_order_has_price() {
        let req = OrderRequest {
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: Some(dec!(130.00)),
        };
        assert_eq!(req.price, Some(dec!(130.00)));
    }

    #[test]
    fn types_are_clone_and_debug() {
        let pos = Position {
            symbol: "AAPL".into(),
            qty: 50,
            avg_price: dec!(175.30),
        };
        let cloned = pos.clone();
        assert_eq!(pos, cloned);
        let _ = format!("{:?}", cloned);
    }
}
