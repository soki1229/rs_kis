use kis_bot::risk::{RiskSizerInput, calculate_size};
use rust_decimal_macros::dec;
use rust_decimal::Decimal;

#[test]
fn basic_size_calculation() {
    // entry_price=100 → max_size=10, cap 미적용
    // base=50/27≈1.851, sf=1.15(strength=0.80) → final≈2.129
    let input = RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.005),
        atr: dec!(18),
        atr_stop_multiplier: dec!(1.5),
        entry_price: dec!(100),
        strength: 0.80,
        regime_factor: dec!(1.0),
        profile_factor: dec!(1.0),
        max_position_pct: dec!(0.10),
    };
    let size = calculate_size(&input);
    assert!(size > dec!(2.0) && size < dec!(2.5), "size = {}", size);
}

#[test]
fn volatile_regime_halves_size() {
    // entry_price=100 → max_size=10, cap 미적용 → regime 비율 순수 비교 가능
    let input = RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.005),
        atr: dec!(18),
        atr_stop_multiplier: dec!(1.5),
        entry_price: dec!(100),
        strength: 0.70,
        regime_factor: dec!(0.5),
        profile_factor: dec!(1.0),
        max_position_pct: dec!(0.10),
    };
    let volatile_size = calculate_size(&input);

    let trending_input = RiskSizerInput { regime_factor: dec!(1.0), ..input };
    let trending_size = calculate_size(&trending_input);

    let ratio = volatile_size / trending_size;
    assert!(
        ratio > dec!(0.45) && ratio < dec!(0.55),
        "volatile should be ~50% of trending, got ratio {}", ratio
    );
}

#[test]
fn max_position_pct_caps_size() {
    let input = RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.10),
        atr: dec!(1),
        atr_stop_multiplier: dec!(0.1),
        entry_price: dec!(100),
        strength: 1.0,
        regime_factor: dec!(1.0),
        profile_factor: dec!(1.6),
        max_position_pct: dec!(0.10),
    };
    let size = calculate_size(&input);
    assert!(size <= dec!(10), "capped size should be ≤ 10, got {}", size);
}

#[test]
fn strength_factor_table() {
    use kis_bot::risk::strength_size_factor;
    assert!((strength_size_factor(0.60) - 0.75).abs() < 0.001);
    assert!((strength_size_factor(0.70) - 1.00).abs() < 0.001);
    assert!((strength_size_factor(0.80) - 1.15).abs() < 0.001);
    assert!((strength_size_factor(0.90) - 1.25).abs() < 0.001);
}
