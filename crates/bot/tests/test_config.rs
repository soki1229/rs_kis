use kis_bot::config::{BotConfig, ProfileName};
use std::path::Path;

#[test]
fn load_default_config() {
    let path = Path::new("tests/fixtures/bot_config.toml");
    let cfg = BotConfig::from_file(path).expect("config should parse");
    assert_eq!(cfg.profile.active, ProfileName::Default);
    assert_eq!(cfg.risk.consecutive_loss_limit, 3);
    assert_eq!(cfg.signal.setup_score_threshold_entry, 60);
    assert_eq!(cfg.signal.setup_score_threshold_llm, 80);
    assert_eq!(cfg.llm.model, "claude-haiku-4-5-20251001");
    assert_eq!(cfg.trading_hours.entry_blackout_open_mins, 15);
}

#[test]
fn effective_risk_conservative_overrides() {
    let path = Path::new("tests/fixtures/bot_config.toml");
    let mut cfg = BotConfig::from_file(path).unwrap();
    cfg.profile.active = ProfileName::Conservative;
    let risk = cfg.effective_risk_per_trade();
    // Conservative overrides default 0.005 → 0.003
    assert!((risk - 0.003).abs() < f64::EPSILON);
}

#[test]
fn effective_risk_aggressive_overrides() {
    let path = Path::new("tests/fixtures/bot_config.toml");
    let mut cfg = BotConfig::from_file(path).unwrap();
    cfg.profile.active = ProfileName::Aggressive;
    assert!((cfg.effective_risk_per_trade() - 0.008).abs() < f64::EPSILON);
}

#[test]
fn missing_file_returns_error() {
    let result = BotConfig::from_file(Path::new("/nonexistent/path.toml"));
    assert!(result.is_err());
}
