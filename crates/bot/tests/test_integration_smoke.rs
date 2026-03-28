/// Plan 1-4 전체가 올바르게 연결되는지 확인.
/// 모든 public API가 컴파일되고 기본 연계가 동작하는지 검증.
use kis_bot::{
    config::{BotConfig, ProfileName},
    monitoring::alert::{AlertRouter, AlertSeverity},
    monitoring::strategy_monitor::{evaluate_monitoring, MonitoringDecision, MonitoringInput},
    qualification::setup_score::{calculate_setup_score, SetupScoreInput},
    regime::{classify_regime, RegimeInput},
    risk::{calculate_size, strength_size_factor, PortfolioContext, RiskSizerInput},
    signal::SignalDecision,
    types::*,
};
use rust_decimal_macros::dec;
use std::path::Path;

#[test]
fn full_pipeline_happy_path() {
    // 1. Config 로드
    let cfg = BotConfig::from_file(Path::new("tests/fixtures/bot_config.toml")).unwrap();

    // 2. Regime 판정
    let regime = classify_regime(&RegimeInput {
        ma5: 100.0,
        ma20: 98.0,
        daily_change_pct: 0.8,
        volume_ratio: 1.0,
    });
    assert_eq!(regime, MarketRegime::Trending);

    // 3. Setup Score
    let score = calculate_setup_score(&SetupScoreInput {
        ma5_above_ma20: true,
        volume_ratio: 2.5,
        recent_5min_volume_ratio: 1.6,
        bid_ask_imbalance: 1.4,
        new_high_last_10min: true,
        has_news_catalyst: true,
        daily_change_pct: 2.0,
        mins_until_close: 300,
        entry_blackout_close_mins: 15,
        regime: MarketRegime::Trending,
    });
    assert_eq!(score, 100);

    // 4. Signal decision
    let signal = RuleSignal {
        direction: Direction::Long,
        strength: 0.80,
    };
    let trace = SignalDecision::evaluate_without_llm(
        score,
        Some(signal.clone()),
        cfg.signal.rule_strength_threshold,
    );
    assert!(matches!(trace.decision, SignalDecision::Enter { .. }));

    // 5. Risk sizing
    let size = calculate_size(
        &RiskSizerInput {
            account_balance: dec!(10000),
            risk_per_trade: dec!(0.005),
            atr: dec!(18),
            atr_stop_multiplier: dec!(1.5),
            entry_price: dec!(100),
            strength: signal.strength,
            regime_factor: dec!(1.0),
            profile_factor: dec!(1.0),
            max_position_pct: dec!(0.10),
        },
        &PortfolioContext {
            open_position_count: 0,
            max_open_positions: 5,
            current_drawdown_pct: dec!(0),
        },
    );
    assert!(size > dec!(0));
}

#[tokio::test]
async fn alert_router_emits_critical() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.critical("Kill switch HARD triggered".to_string());
    let evt = rx.recv().await.unwrap();
    assert_eq!(evt.severity, AlertSeverity::Critical);
}

#[test]
fn monitoring_force_conservative_on_drawdown() {
    let input = MonitoringInput {
        current_profile: ProfileName::Default,
        rolling_7d_r: -2.5,
        mdd_pct: 0.0,
        regime_consecutive_losses: 0,
        rolling_30d_r: 0.0,
        llm_win_rate_vs_rule: 0.0,
        score_80plus_win_rate: 0.5,
        conservative_days_elapsed: 0,
        consecutive_losses: 0,
        conservative_7d_r: 0.0,
    };
    let decisions = evaluate_monitoring(&input);
    assert!(decisions
        .iter()
        .any(|d| matches!(d, MonitoringDecision::ForceConservative { .. })));
}

#[test]
fn strength_size_factor_table_spot_check() {
    assert!((strength_size_factor(0.90) - 1.25).abs() < 0.001);
    assert!((strength_size_factor(0.60) - 0.75).abs() < 0.001);
}
