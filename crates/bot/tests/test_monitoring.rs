use kis_bot::monitoring::strategy_monitor::{
    MonitoringInput, MonitoringDecision, evaluate_monitoring,
};
use kis_bot::config::ProfileName;

fn default_input() -> MonitoringInput {
    MonitoringInput {
        current_profile: ProfileName::Default,
        rolling_7d_r: 0.0,
        mdd_pct: 0.0,
        regime_consecutive_losses: 0,
        rolling_30d_r: 0.0,
        llm_win_rate_vs_rule: 0.0,
        score_80plus_win_rate: 0.5,
        conservative_days_elapsed: 0,
        consecutive_losses: 0,
        conservative_7d_r: 0.0,
    }
}

#[test]
fn no_action_when_all_clear() {
    let input = default_input();
    let decisions = evaluate_monitoring(&input);
    assert!(decisions.is_empty());
}

#[test]
fn force_conservative_when_7d_r_below_minus_2() {
    let mut input = default_input();
    input.rolling_7d_r = -2.5;
    let decisions = evaluate_monitoring(&input);
    assert!(
        decisions.iter().any(|d| matches!(d, MonitoringDecision::ForceConservative { .. })),
        "should force conservative"
    );
}

#[test]
fn force_conservative_when_mdd_5pct() {
    let mut input = default_input();
    input.mdd_pct = 0.055;
    let decisions = evaluate_monitoring(&input);
    assert!(decisions.iter().any(|d| matches!(d, MonitoringDecision::ForceConservative { .. })));
}

#[test]
fn warn_alert_when_30d_r_below_minus_5() {
    let mut input = default_input();
    input.rolling_30d_r = -5.5;
    let decisions = evaluate_monitoring(&input);
    assert!(
        decisions.iter().any(|d| matches!(d, MonitoringDecision::WarnAlert { .. })),
        "should emit warn alert"
    );
}

#[test]
fn regime_suspend_when_5_consecutive_losses() {
    let mut input = default_input();
    input.regime_consecutive_losses = 5;
    let decisions = evaluate_monitoring(&input);
    assert!(decisions.iter().any(|d| matches!(d, MonitoringDecision::SuspendRegime { .. })));
}

#[test]
fn return_to_default_after_cooldown_and_recovery() {
    let mut input = default_input();
    input.current_profile = ProfileName::Conservative;
    input.conservative_days_elapsed = 4;
    input.conservative_7d_r = 1.5;
    input.consecutive_losses = 0;
    let decisions = evaluate_monitoring(&input);
    assert!(
        decisions.iter().any(|d| matches!(d, MonitoringDecision::ReturnToDefault)),
        "should return to default after cooldown"
    );
}

#[test]
fn no_return_to_default_before_cooldown() {
    let mut input = default_input();
    input.current_profile = ProfileName::Conservative;
    input.conservative_days_elapsed = 2;
    input.conservative_7d_r = 1.5;
    input.consecutive_losses = 0;
    let decisions = evaluate_monitoring(&input);
    assert!(
        !decisions.iter().any(|d| matches!(d, MonitoringDecision::ReturnToDefault)),
        "should not return before cooldown"
    );
}
