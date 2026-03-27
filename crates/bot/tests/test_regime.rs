use kis_bot::regime::{RegimeInput, classify_regime};
use kis_bot::types::MarketRegime;

#[test]
fn trending_when_ma5_above_ma20_normal_volume() {
    let input = RegimeInput {
        ma5: 100.0,
        ma20: 98.0,
        daily_change_pct: 0.8,
        volume_ratio: 1.0,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Trending);
}

#[test]
fn volatile_when_large_daily_move() {
    let input = RegimeInput {
        ma5: 100.0,
        ma20: 98.0,
        daily_change_pct: 2.0,
        volume_ratio: 1.0,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Volatile);
}

#[test]
fn volatile_when_large_drop() {
    let input = RegimeInput {
        ma5: 98.0,
        ma20: 100.0,
        daily_change_pct: -1.8,
        volume_ratio: 1.0,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Volatile);
}

#[test]
fn quiet_when_tiny_move_low_volume() {
    let input = RegimeInput {
        ma5: 98.0,
        ma20: 100.0,
        daily_change_pct: 0.2,
        volume_ratio: 0.5,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Quiet);
}

#[test]
fn volatile_takes_priority_over_quiet() {
    let input = RegimeInput {
        ma5: 98.0,
        ma20: 100.0,
        daily_change_pct: -2.0,
        volume_ratio: 0.3,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Volatile);
}
