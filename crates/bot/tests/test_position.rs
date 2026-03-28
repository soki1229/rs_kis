use kis_bot::position::{evaluate_exit, ExitDecision, PositionState};
use kis_bot::types::MarketRegime;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn make_position(entry: f64, atr: f64) -> PositionState {
    let entry = Decimal::try_from(entry).unwrap();
    let atr = Decimal::try_from(atr).unwrap();
    PositionState {
        entry_price: entry,
        stop_price: entry - atr * dec!(1.5),
        atr_at_entry: atr,
        profit_target_1: entry + atr * dec!(2.0),
        profit_target_2: entry + atr * dec!(4.0),
        trailing_stop_price: None,
        partial_exit_done: false,
        regime: MarketRegime::Trending,
        profit_target_1_atr: dec!(2.0),
        profit_target_2_atr: dec!(4.0),
        trailing_atr_trending: dec!(2.0),
        trailing_atr_volatile: dec!(1.0),
    }
}

#[test]
fn stop_hit_triggers_full_exit() {
    let pos = make_position(100.0, 5.0);
    let decision = evaluate_exit(&pos, dec!(92.0));
    assert!(matches!(decision, ExitDecision::StopLoss));
}

#[test]
fn first_target_hit_triggers_partial() {
    let pos = make_position(100.0, 5.0);
    let decision = evaluate_exit(&pos, dec!(110.5));
    assert!(matches!(decision, ExitDecision::PartialExit { pct } if pct == dec!(0.5)));
}

#[test]
fn second_target_hit_triggers_full_exit() {
    let mut pos = make_position(100.0, 5.0);
    pos.partial_exit_done = true;
    let decision = evaluate_exit(&pos, dec!(121.0));
    assert!(matches!(decision, ExitDecision::FullExit));
}

#[test]
fn trailing_stop_hit_after_partial_exit() {
    let mut pos = make_position(100.0, 5.0);
    pos.partial_exit_done = true;
    pos.trailing_stop_price = Some(dec!(115.0));
    let decision = evaluate_exit(&pos, dec!(114.0));
    assert!(matches!(decision, ExitDecision::TrailingStop));
}

#[test]
fn no_exit_when_in_normal_range() {
    let pos = make_position(100.0, 5.0);
    let decision = evaluate_exit(&pos, dec!(105.0));
    assert!(matches!(decision, ExitDecision::Hold));
}

#[test]
fn trailing_stop_price_for_trending_regime() {
    use kis_bot::position::calculate_trailing_stop;
    let stop = calculate_trailing_stop(
        dec!(120),
        dec!(5),
        &MarketRegime::Trending,
        dec!(2.0),
        dec!(1.0),
    );
    assert_eq!(stop, dec!(110));
}
