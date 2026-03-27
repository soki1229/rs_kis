/// Plan 1 전체가 올바르게 연결되는지 확인하는 통합 smoke test.
use kis_bot::{
    config::{BotConfig, ProfileName},
    error::BotError,
    types::{BotState, KillSwitchMode, MarketRegime},
};
use std::path::Path;

#[test]
fn all_types_importable() {
    let _ = BotState::Active;
    let _ = MarketRegime::Trending;
    let _ = KillSwitchMode::Soft;
}

#[test]
fn config_load_and_profile_override() {
    let path = Path::new("tests/fixtures/bot_config.toml");
    let mut cfg = BotConfig::from_file(path).unwrap();

    // Default profile
    assert_eq!(cfg.effective_consecutive_loss_limit(), 3);

    // Conservative override
    cfg.profile.active = ProfileName::Conservative;
    assert_eq!(cfg.effective_consecutive_loss_limit(), 2);
}

#[test]
fn bot_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let bot_err: BotError = io_err.into();
    assert!(bot_err.to_string().contains("IO error"));
}
