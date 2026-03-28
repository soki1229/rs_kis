use kis_bot::qualification::hard_block::{HardBlockChecker, HardBlockInput};

fn make_input(
    daily_change_pct: f64,
    mins_since_open: i64,
    mins_until_close: i64,
    has_earnings_today: bool,
) -> HardBlockInput {
    HardBlockInput {
        daily_change_pct,
        mins_since_open,
        mins_until_close,
        has_earnings_event: has_earnings_today,
        has_fomc_today: false,
        entry_blackout_open_mins: 15,
        entry_blackout_close_mins: 15,
    }
}

#[test]
fn blocks_earnings_event() {
    let checker = HardBlockChecker;
    let input = make_input(0.5, 60, 300, true);
    assert!(checker.check(&input).is_some(), "earnings should block");
}

#[test]
fn blocks_open_blackout() {
    let checker = HardBlockChecker;
    let input = make_input(0.5, 10, 300, false);
    assert!(checker.check(&input).is_some());
}

#[test]
fn blocks_close_blackout() {
    let checker = HardBlockChecker;
    let input = make_input(0.5, 300, 10, false);
    assert!(checker.check(&input).is_some());
}

#[test]
fn blocks_large_daily_gain() {
    let checker = HardBlockChecker;
    let input = make_input(11.0, 60, 300, false);
    assert!(checker.check(&input).is_some());
}

#[test]
fn blocks_large_daily_loss() {
    let checker = HardBlockChecker;
    let input = make_input(-3.5, 60, 300, false);
    assert!(checker.check(&input).is_some());
}

#[test]
fn passes_normal_conditions() {
    let checker = HardBlockChecker;
    let input = make_input(2.0, 60, 300, false);
    assert!(
        checker.check(&input).is_none(),
        "normal conditions should pass"
    );
}
