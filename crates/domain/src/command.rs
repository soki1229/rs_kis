use crate::types::OrderRequest;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub enum BotCommand {
    Start,
    Stop,
    Pause,
    ForceOrder(OrderRequest),
    SetRiskLimit(Decimal),
    QueryStatus,
    // SetStrategy — StrategyParams 정의 후 추가
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OrderRequest, Side};
    use rust_decimal_macros::dec;

    #[test]
    fn bot_command_is_clone() {
        let cmd = BotCommand::SetRiskLimit(dec!(0.05));
        let cloned = cmd.clone();
        // Clone 성공 — 컴파일되면 통과
        let _ = cloned;
    }

    #[test]
    fn bot_command_force_order_contains_request() {
        let req = OrderRequest {
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: Some(dec!(130.00)),
        };
        let cmd = BotCommand::ForceOrder(req.clone());
        match cmd {
            BotCommand::ForceOrder(r) => assert_eq!(r.symbol, "NVDA"),
            _ => panic!("wrong variant"),
        }
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn bot_command_is_send_sync() {
        assert_send_sync::<BotCommand>();
    }
}
