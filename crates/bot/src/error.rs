use crate::types::KillSwitchMode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Database migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("KIS API error: {0}")]
    KisApi(#[from] kis_api::KisError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Kill switch active: {mode:?}")]
    KillSwitchActive { mode: KillSwitchMode },

    #[error("Recovery check failed: {reason}")]
    RecoveryFailed { reason: String },

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Risk guard blocked: {reason}")]
    RiskBlocked { reason: String },
}
