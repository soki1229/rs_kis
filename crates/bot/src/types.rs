use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ── Market / Bot state ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketRegime {
    Trending,
    Volatile,
    Quiet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotState {
    RecoveryCheck,
    Active,
    EntryPaused { reason: EntryPauseReason },
    SoftBlocked,
    HardBlocked,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryPauseReason {
    DailyLossLimit,
    QuietRegime,
    RegimeSuspended { regime: MarketRegime },
}

// ── Kill Switch ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KillSwitchMode {
    Soft,
    Hard,
}

/// JSON payload written to the kill switch file on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchFile {
    pub mode: KillSwitchMode,
    pub reason: String,
    pub triggered_at: DateTime<FixedOffset>,
    pub details: String,
}

// ── Order ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderState {
    PendingSubmit,
    Submitted,
    PartiallyFilled { filled_qty: Decimal },
    FullyFilled,
    CancelledPartial { filled_qty: Decimal },
    Cancelled,
    Failed { reason: String },
}

// ── Signal ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Long,
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSignal {
    pub direction: Direction,
    /// 0.0 ~ 1.0. Setup Score 60~79 경로: ≥ 0.65 필요. 80+ 경로: ≥ 0.55 허용.
    pub strength: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmVerdict {
    Enter,
    Watch,
    Block,
}

// ── Alerts ─────────────────────────────────────────────────────────────────

/// trading-bot → trading-server 알림 채널 이벤트.
/// CRITICAL: Telegram 즉시 발송. WARN: 로그+Telegram. INFO: 로그만.
#[derive(Debug, Clone)]
pub enum AlertEvent {
    Info { message: String },
    Warn { message: String },
    Critical { message: String },
}
