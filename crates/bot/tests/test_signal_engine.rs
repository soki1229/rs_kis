use kis_bot::signal::SignalDecision;
use kis_bot::types::{Direction, LlmVerdict, RuleSignal};

#[test]
fn score_below_60_is_rejected() {
    let decision = SignalDecision::evaluate_without_llm(
        55,
        None,
        0.65,
    );
    assert!(matches!(decision, SignalDecision::Rejected { .. }));
}

#[test]
fn score_60_79_rule_only_passes_with_strong_signal() {
    let signal = RuleSignal { direction: Direction::Long, strength: 0.70 };
    let decision = SignalDecision::evaluate_without_llm(
        70,
        Some(signal),
        0.65,
    );
    assert!(matches!(decision, SignalDecision::Enter { .. }));
}

#[test]
fn score_60_79_rule_only_fails_with_weak_signal() {
    let signal = RuleSignal { direction: Direction::Long, strength: 0.60 };
    let decision = SignalDecision::evaluate_without_llm(
        70,
        Some(signal),
        0.65,
    );
    assert!(matches!(decision, SignalDecision::Rejected { .. }));
}

#[test]
fn llm_block_overrides_strong_rule() {
    let signal = RuleSignal { direction: Direction::Long, strength: 0.90 };
    let decision = SignalDecision::evaluate_with_llm(
        85,
        Some(signal),
        0.55,
        LlmVerdict::Block,
    );
    assert!(matches!(decision, SignalDecision::Rejected { .. }));
}

#[test]
fn llm_enter_with_strong_rule_enters() {
    let signal = RuleSignal { direction: Direction::Long, strength: 0.80 };
    let decision = SignalDecision::evaluate_with_llm(
        90,
        Some(signal),
        0.55,
        LlmVerdict::Enter,
    );
    assert!(matches!(decision, SignalDecision::Enter { .. }));
}
