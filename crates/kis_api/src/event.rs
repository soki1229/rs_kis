use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

/// 실시간 체결 데이터
#[derive(Debug, Clone)]
pub struct TransactionData {
    pub symbol: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub time: DateTime<FixedOffset>,
    pub is_buy: bool,
}

/// 실시간 호가 데이터 (최우선 호가)
#[derive(Debug, Clone)]
pub struct QuoteData {
    pub symbol: String,
    pub ask_price: Decimal,
    pub bid_price: Decimal,
    pub ask_qty: Decimal,
    pub bid_qty: Decimal,
    pub time: DateTime<FixedOffset>,
}

/// 주문체결통보 데이터
#[derive(Debug, Clone)]
pub struct OrderConfirmData {
    pub order_id: String,
    pub symbol: String,
    pub filled_qty: Decimal,
    pub filled_price: Decimal,
    pub time: DateTime<FixedOffset>,
}

/// WebSocket으로 수신되는 실시간 이벤트
#[derive(Debug, Clone)]
pub enum KisEvent {
    /// 실시간 체결가
    Transaction(TransactionData),
    /// 실시간 호가
    Quote(QuoteData),
    /// 주문체결통보
    OrderConfirm(OrderConfirmData),
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal_macros::dec;

    fn kst() -> FixedOffset {
        FixedOffset::east_opt(9 * 3600).unwrap()
    }

    fn sample_transaction() -> TransactionData {
        TransactionData {
            symbol: "AAPL".into(),
            price: dec!(190.50),
            qty: dec!(10),
            time: kst().with_ymd_and_hms(2026, 3, 21, 10, 0, 0).unwrap(),
            is_buy: true,
        }
    }

    #[test]
    fn transaction_event_is_clone() {
        let ev = KisEvent::Transaction(sample_transaction());
        let _ = ev.clone();
    }

    #[test]
    fn quote_event_fields() {
        let q = QuoteData {
            symbol: "AAPL".into(),
            ask_price: dec!(191.00),
            bid_price: dec!(190.90),
            ask_qty: dec!(100),
            bid_qty: dec!(200),
            time: kst().with_ymd_and_hms(2026, 3, 21, 10, 0, 0).unwrap(),
        };
        assert_eq!(q.ask_price, dec!(191.00));
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn kis_event_is_send_sync() {
        assert_send_sync::<KisEvent>();
    }
}
