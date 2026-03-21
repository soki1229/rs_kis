use crate::types::{AlertLevel, FillInfo, OrderResult, PnLReport, Position};

#[derive(Debug, Clone)]
pub enum BotEvent {
    OrderPlaced(OrderResult),
    OrderFilled(FillInfo),
    PositionChanged(Position),
    DailyPnL(PnLReport),
    Alert { level: AlertLevel, msg: String },
    // StatusReport(BotStatus) — BotStatus 정의 후 추가
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use crate::types::{AlertLevel, OrderResult, PnLReport, Side};
    use chrono::NaiveDate;

    fn sample_order_result() -> OrderResult {
        OrderResult {
            order_id: "ORD-001".into(),
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: dec!(130.00),
        }
    }

    #[test]
    fn bot_event_is_clone() {
        let ev = BotEvent::OrderPlaced(sample_order_result());
        let _ = ev.clone();
    }

    #[test]
    fn alert_event_carries_level_and_message() {
        let ev = BotEvent::Alert {
            level: AlertLevel::Critical,
            msg: "connection lost".into(),
        };
        match ev {
            BotEvent::Alert { level, msg } => {
                assert_eq!(level, AlertLevel::Critical);
                assert_eq!(msg, "connection lost");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn daily_pnl_event_contains_report() {
        let report = PnLReport {
            realized: dec!(320.50),
            unrealized: dec!(45.00),
            date: NaiveDate::from_ymd_opt(2026, 3, 21).unwrap(),
        };
        let ev = BotEvent::DailyPnL(report);
        match ev {
            BotEvent::DailyPnL(r) => assert_eq!(r.realized, dec!(320.50)),
            _ => panic!("wrong variant"),
        }
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn bot_event_is_send_sync() {
        assert_send_sync::<BotEvent>();
    }
}
