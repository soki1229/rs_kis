use kis_bot::signal::rule_engine::{RuleEngine, RuleEngineInput};
use kis_bot::types::Direction;

fn consistent_long_input() -> RuleEngineInput {
    RuleEngineInput {
        recent_candle_directions: vec![true, true, true, true, true],
        price_vs_prev_close_atr_ratio: 0.30,
        volume_acceleration: vec![1.0, 1.2, 1.5],
    }
}

#[test]
fn strong_long_signal_high_strength() {
    let engine = RuleEngine;
    let signal = engine.evaluate(&consistent_long_input());
    assert!(signal.is_some());
    let s = signal.unwrap();
    assert_eq!(s.direction, Direction::Long);
    assert!(
        s.strength >= 0.75,
        "consistent bullish should be strong: {}",
        s.strength
    );
}

#[test]
fn weak_signal_below_threshold() {
    let engine = RuleEngine;
    // 3/5 up, ATR 중립(0.0), 거래량 감소 → strength ~0.54 < 0.60
    let input = RuleEngineInput {
        recent_candle_directions: vec![true, false, true, false, true],
        price_vs_prev_close_atr_ratio: 0.0,
        volume_acceleration: vec![1.0, 0.9, 0.8],
    };
    let signal = engine.evaluate(&input);
    if let Some(s) = signal {
        assert!(
            s.strength < 0.60,
            "mixed signal should be weak: {}",
            s.strength
        );
    }
}

#[test]
fn consistent_down_gives_no_signal() {
    let engine = RuleEngine;
    let input = RuleEngineInput {
        recent_candle_directions: vec![false, false, false, false, false],
        price_vs_prev_close_atr_ratio: -0.40,
        volume_acceleration: vec![1.0, 1.3, 1.6],
    };
    let signal = engine.evaluate(&input);
    assert!(signal.is_none(), "Short 신호는 현재 미지원 — None 반환 기대");
}
