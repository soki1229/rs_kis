use kis_bot::control::recovery::{
    run_recovery_check, RecoveryFailureCode, RecoveryInput, RecoveryOutcome,
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn make_input(
    db_total: f64,
    broker_total: f64,
    orphaned: bool,
    unreconciled_fills: usize,
) -> RecoveryInput {
    RecoveryInput {
        db_position_total: Decimal::try_from(db_total).unwrap(),
        broker_balance_total: Decimal::try_from(broker_total).unwrap(),
        has_orphaned_submitted_orders: orphaned,
        unreconciled_fill_count: unreconciled_fills,
        has_orders_without_broker_id: false,
        mismatch_threshold_pct: dec!(0.05),
    }
}

#[test]
fn clean_state_passes() {
    let input = make_input(1000.0, 1010.0, false, 0);
    let result = run_recovery_check(&input);
    assert!(matches!(result, RecoveryOutcome::Pass));
}

#[test]
fn large_balance_mismatch_fails() {
    let input = make_input(1000.0, 1100.0, false, 0);
    let result = run_recovery_check(&input);
    assert!(matches!(
        result,
        RecoveryOutcome::Fail {
            code: RecoveryFailureCode::BalanceMismatch,
            ..
        }
    ));
}

#[test]
fn orphaned_order_fails() {
    let input = make_input(1000.0, 1000.0, true, 0);
    let result = run_recovery_check(&input);
    assert!(matches!(
        result,
        RecoveryOutcome::Fail {
            code: RecoveryFailureCode::OrphanedOrder,
            ..
        }
    ));
}

#[test]
fn unreconciled_fill_is_auto_fixed_not_fail() {
    let input = make_input(1000.0, 1000.0, false, 2);
    let result = run_recovery_check(&input);
    assert!(matches!(result, RecoveryOutcome::AutoFixed { count: 2 }));
}

#[test]
fn missing_broker_id_fails() {
    let mut input = make_input(1000.0, 1000.0, false, 0);
    input.has_orders_without_broker_id = true;
    let result = run_recovery_check(&input);
    assert!(matches!(
        result,
        RecoveryOutcome::Fail {
            code: RecoveryFailureCode::BrokerOrderMissing,
            ..
        }
    ));
}
