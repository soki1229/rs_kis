use kis_bot::types::*;

#[test]
fn market_regime_serde_roundtrip() {
    let r = MarketRegime::Trending;
    let json = serde_json::to_string(&r).unwrap();
    let back: MarketRegime = serde_json::from_str(&json).unwrap();
    assert_eq!(r, back);
}

#[test]
fn order_state_partially_filled() {
    use rust_decimal_macros::dec;
    let s = OrderState::PartiallyFilled {
        filled_qty: dec!(1.5),
    };
    let json = serde_json::to_string(&s).unwrap();
    let back: OrderState = serde_json::from_str(&json).unwrap();
    assert!(matches!(back, OrderState::PartiallyFilled { .. }));
}

#[test]
fn kill_switch_mode_debug() {
    let m = KillSwitchMode::Hard;
    assert_eq!(format!("{m:?}"), "Hard");
}

#[test]
fn rule_signal_strength_range() {
    let s = RuleSignal {
        direction: Direction::Long,
        strength: 0.75,
    };
    assert!(s.strength >= 0.0 && s.strength <= 1.0);
}
