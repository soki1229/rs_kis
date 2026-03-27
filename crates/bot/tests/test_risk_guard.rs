use kis_bot::risk::guard::{RiskGuard, SessionStats};
use rust_decimal_macros::dec;

fn make_guard(max_positions: u32, daily_loss_limit: f64, consec_limit: u32) -> RiskGuard {
    let limit = rust_decimal::Decimal::try_from(daily_loss_limit).unwrap();
    RiskGuard::new(max_positions, limit, consec_limit, dec!(0.30))
}

#[test]
fn blocks_when_max_positions_reached() {
    let guard = make_guard(3, 0.015, 3);
    let stats = SessionStats {
        open_positions: 3,
        daily_pnl: dec!(0),
        consecutive_losses: 0,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_err());
}

#[test]
fn blocks_when_daily_loss_exceeded() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 0,
        daily_pnl: dec!(-200),
        consecutive_losses: 0,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_err());
}

#[test]
fn blocks_when_consecutive_loss_limit_reached() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 0,
        daily_pnl: dec!(0),
        consecutive_losses: 3,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_err());
}

#[test]
fn passes_when_all_clear() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 2,
        daily_pnl: dec!(50),
        consecutive_losses: 1,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_ok());
}

#[test]
fn hard_kill_switch_when_pending_orders_exceed_30pct() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 0,
        daily_pnl: dec!(0),
        consecutive_losses: 0,
        account_balance: dec!(10000),
        pending_order_value: dec!(3100),
    };
    assert!(guard.check_kill_switch_hard(&stats));
}
