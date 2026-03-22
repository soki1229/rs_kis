# trading-bot Plan 1: Foundation (Types · Config · DB)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Set up the `crates/bot` crate with all dependencies, core types, TOML config loading, and SQLite schema — the foundation every other plan builds on.

**Architecture:** `BotConfig` is loaded from a TOML file at startup. Core enums (`MarketRegime`, `BotState`, `OrderState`) are defined in `types.rs`. SQLite state is managed via `sqlx` with a migration file at `src/db/migrations/`. No runtime logic yet — this plan only produces a compiling crate with tested primitives.

**Tech Stack:** Rust 2021, sqlx 0.8 (sqlite + runtime-tokio + migrate), toml 0.8, thiserror 2, tokio 1 (full), chrono 0.4, rust_decimal 1, reqwest 0.12, rand 0.8, tracing 0.1

**Spec reference:** `docs/superpowers/specs/2026-03-22-trading-bot-design.md` — Sections 12 (Config), 9 (DB schema), error types throughout

---

## File Map

| 파일 | 역할 |
|------|------|
| `crates/bot/Cargo.toml` | 의존성 전체 |
| `crates/bot/src/lib.rs` | public re-exports |
| `crates/bot/src/error.rs` | `BotError` enum |
| `crates/bot/src/types.rs` | `MarketRegime`, `BotState`, `OrderState`, `Direction`, `RuleSignal`, `LlmVerdict`, `AlertEvent`, `KillSwitchMode` |
| `crates/bot/src/config.rs` | `BotConfig` + 모든 섹션 struct + `from_file()` + profile override |
| `crates/bot/src/db.rs` | `connect()`, migration 실행 |
| `crates/bot/src/db/migrations/0001_initial.sql` | 5개 테이블 schema |

---

## Task 1: Cargo.toml 의존성

**Files:**
- Modify: `crates/bot/Cargo.toml`

- [ ] **Step 1: 기존 파일 읽기**

```bash
cat crates/bot/Cargo.toml
```

- [ ] **Step 2: 전체 Cargo.toml 교체**

```toml
[package]
name = "kis_bot"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }
tokio-util = { version = "0.7", features = ["rt"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# Error handling
thiserror = "2"

# Date/time (KST)
chrono = { version = "0.4", features = ["serde"] }

# Decimal arithmetic (no f64 for prices/quantities)
rust_decimal = { version = "1", features = ["serde-with-str"] }
rust_decimal_macros = "1"

# Database
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "sqlite",
    "macros",
    "chrono",
    "migrate",
] }

# HTTP (for Finnhub REST API)
reqwest = { version = "0.12", features = ["json"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# File path expansion (~/ → absolute)
shellexpand = "3"

# Jitter for retry backoff
rand = "0.8"

# Internal crates
kis_api = { path = "../kis_api" }
domain = { path = "../domain" }

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
tempfile = "3"
```

- [ ] **Step 3: 컴파일 확인**

```bash
cargo check -p kis_bot
```

Expected: 컴파일 통과 (경고는 무방)

- [ ] **Step 4: Commit**

```bash
git add crates/bot/Cargo.toml
git commit -m "feat(bot): add all foundation dependencies"
```

---

## Task 2: types.rs — 핵심 도메인 타입 (error.rs보다 먼저 구현)

> **순서 주의:** `error.rs`가 `types::KillSwitchMode`를 참조하므로 `types.rs`를 먼저 구현한다.

**Files:**
- Create: `crates/bot/src/types.rs`
- Modify: `crates/bot/src/lib.rs`
- Create: `crates/bot/tests/test_types.rs`

- [ ] **Step 1: lib.rs에 `pub mod types;` 추가**

`crates/bot/src/lib.rs`:
```rust
pub mod types;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_types.rs`:
```rust
use kis_bot::types::*;
use serde_json;

#[test]
fn market_regime_serde_roundtrip() {
    let r = MarketRegime::Trending;
    let json = serde_json::to_string(&r).unwrap();
    let back: MarketRegime = serde_json::from_str(&json).unwrap();
    assert_eq!(r, back);
}

#[test]
fn order_state_partially_filled() {
    use rust_decimal_macros::dec;
    let s = OrderState::PartiallyFilled { filled_qty: dec!(1.5) };
    let json = serde_json::to_string(&s).unwrap();
    let back: OrderState = serde_json::from_str(&json).unwrap();
    assert!(matches!(back, OrderState::PartiallyFilled { .. }));
}

#[test]
fn kill_switch_mode_debug() {
    let m = KillSwitchMode::Hard;
    assert_eq!(format!("{m:?}"), "Hard");
}

#[test]
fn rule_signal_strength_range() {
    let s = RuleSignal { direction: Direction::Long, strength: 0.75 };
    assert!(s.strength >= 0.0 && s.strength <= 1.0);
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_types 2>&1 | head -10
```

Expected: compile error

- [ ] **Step 4: types.rs 구현**

`crates/bot/src/types.rs`:
```rust
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
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_types
```

Expected: 4 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/types.rs crates/bot/tests/test_types.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add core domain types (MarketRegime, BotState, OrderState, etc.)"
```

---

## Task 3: error.rs — BotError

**Files:**
- Create: `crates/bot/src/error.rs`
- Modify: `crates/bot/src/lib.rs`

- [ ] **Step 1: lib.rs에 `pub mod error;` 추가**

```rust
pub mod error;
pub mod types;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_error.rs`:
```rust
use kis_bot::error::BotError;

#[test]
fn bot_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BotError>();
}

#[test]
fn bot_error_display() {
    let e = BotError::Config("missing key".to_string());
    assert!(e.to_string().contains("missing key"));
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_error 2>&1 | head -20
```

Expected: compile error (BotError not defined)

- [ ] **Step 4: BotError 구현**

`crates/bot/src/error.rs`:
```rust
use thiserror::Error;
use crate::types::KillSwitchMode;

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
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_error
```

Expected: 2 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/error.rs crates/bot/src/lib.rs crates/bot/tests/test_error.rs
git commit -m "feat(bot): add BotError with all From impls"
```

---

## Task 4: config.rs — BotConfig TOML 로딩 + 프로파일 override

**Files:**
- Create: `crates/bot/src/config.rs`
- Create: `crates/bot/tests/test_config.rs`
- Create: `crates/bot/tests/fixtures/bot_config.toml`

- [ ] **Step 1: fixture TOML 작성**

`crates/bot/tests/fixtures/bot_config.toml`:
```toml
[profile]
active = "default"

[risk]
risk_per_trade = 0.005
max_open_positions = 5
max_position_pct = 0.10
max_sector_exposure = 0.30
daily_loss_limit = 0.015
consecutive_loss_limit = 3
atr_stop_multiplier = 1.5

[signal]
setup_score_threshold_entry = 60
setup_score_threshold_llm = 80
rule_strength_threshold = 0.65
fallback_rule_strength = 0.70

[llm]
model = "claude-haiku-4-5-20251001"
timeout_secs = 10

[position]
profit_target_1_atr = 2.0
profit_target_2_atr = 4.0
trailing_atr_trending = 2.0
trailing_atr_volatile = 1.0

[trading_hours]
entry_blackout_open_mins = 15
entry_blackout_close_mins = 15
eod_liquidate_mins = 15

[monitoring]
regime_consecutive_loss_limit = 5
regime_suspend_days = 7
llm_underperformance_weeks = 3
cumulative_r_alert_threshold = -5.0
mdd_alert_pct = 0.10

[finnhub]
api_key = "test_api_key"

[state]
db_path = "/tmp/test_trading_bot.db"
kill_switch_path = "/tmp/test_kill_switch"
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_config.rs`:
```rust
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
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_config 2>&1 | head -10
```

Expected: compile error

- [ ] **Step 4: config.rs 구현**

`crates/bot/src/config.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::BotError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub profile: ProfileSection,
    pub risk: RiskSection,
    pub signal: SignalSection,
    pub llm: LlmSection,
    pub position: PositionSection,
    pub trading_hours: TradingHoursSection,
    pub monitoring: MonitoringSection,
    pub finnhub: FinnhubSection,
    pub state: StateSection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileSection {
    pub active: ProfileName,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileName {
    Default,
    Conservative,
    Aggressive,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskSection {
    pub risk_per_trade: f64,
    pub max_open_positions: u32,
    pub max_position_pct: f64,
    pub max_sector_exposure: f64,
    pub daily_loss_limit: f64,
    pub consecutive_loss_limit: u32,
    pub atr_stop_multiplier: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignalSection {
    pub setup_score_threshold_entry: i32,
    pub setup_score_threshold_llm: i32,
    pub rule_strength_threshold: f64,
    pub fallback_rule_strength: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmSection {
    pub model: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PositionSection {
    pub profit_target_1_atr: f64,
    pub profit_target_2_atr: f64,
    pub trailing_atr_trending: f64,
    pub trailing_atr_volatile: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradingHoursSection {
    pub entry_blackout_open_mins: i64,
    pub entry_blackout_close_mins: i64,
    pub eod_liquidate_mins: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoringSection {
    pub regime_consecutive_loss_limit: u32,
    pub regime_suspend_days: u32,
    pub llm_underperformance_weeks: u32,
    pub cumulative_r_alert_threshold: f64,
    pub mdd_alert_pct: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinnhubSection {
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateSection {
    pub db_path: String,
    pub kill_switch_path: String,
}

impl BotConfig {
    pub fn from_file(path: &Path) -> Result<Self, BotError> {
        let content = std::fs::read_to_string(path).map_err(BotError::Io)?;
        toml::from_str(&content).map_err(|e| BotError::Config(e.to_string()))
    }

    /// 현재 프로파일에 따른 `risk_per_trade` 값 반환.
    /// Conservative: 0.003, Aggressive: 0.008, Default: config 값 그대로.
    pub fn effective_risk_per_trade(&self) -> f64 {
        match self.profile.active {
            ProfileName::Conservative => 0.003_f64.min(self.risk.risk_per_trade),
            ProfileName::Aggressive => 0.008_f64.max(self.risk.risk_per_trade),
            ProfileName::Default => self.risk.risk_per_trade,
        }
    }

    /// 현재 프로파일에 따른 Setup Score 진입 임계값.
    pub fn effective_score_threshold_entry(&self) -> i32 {
        match self.profile.active {
            ProfileName::Conservative => 70,
            ProfileName::Aggressive => 55,
            ProfileName::Default => self.signal.setup_score_threshold_entry,
        }
    }

    /// 현재 프로파일에 따른 LLM 호출 임계값.
    pub fn effective_score_threshold_llm(&self) -> i32 {
        match self.profile.active {
            ProfileName::Conservative => 85,
            ProfileName::Aggressive => 75,
            ProfileName::Default => self.signal.setup_score_threshold_llm,
        }
    }

    /// 현재 프로파일에 따른 consecutive_loss_limit.
    pub fn effective_consecutive_loss_limit(&self) -> u32 {
        match self.profile.active {
            ProfileName::Conservative => 2,
            ProfileName::Aggressive => 4,
            ProfileName::Default => self.risk.consecutive_loss_limit,
        }
    }

    /// 현재 프로파일에 따른 atr_stop_multiplier.
    pub fn effective_atr_stop_multiplier(&self) -> f64 {
        match self.profile.active {
            ProfileName::Conservative => 2.0,
            ProfileName::Aggressive => 1.2,
            ProfileName::Default => self.risk.atr_stop_multiplier,
        }
    }
}
```

`crates/bot/src/lib.rs` 에 추가:
```rust
pub mod config;
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_config
```

Expected: 4 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/config.rs crates/bot/tests/test_config.rs \
        crates/bot/tests/fixtures/bot_config.toml crates/bot/src/lib.rs
git commit -m "feat(bot): add BotConfig with TOML loading and profile overrides"
```

---

## Task 5: DB 마이그레이션 SQL 작성

**Files:**
- Create: `crates/bot/src/db/migrations/0001_initial.sql`

- [ ] **Step 1: 디렉터리 생성 및 SQL 파일 작성**

`crates/bot/src/db/migrations/0001_initial.sql`:
```sql
-- orders: 모든 주문 기록. broker_order_id 없으면 재시작 복구 불가.
CREATE TABLE IF NOT EXISTS orders (
    id                 TEXT PRIMARY KEY,
    broker_order_id    TEXT,                       -- KIS 반환 주문번호, 제출 직후 저장 필수
    symbol             TEXT NOT NULL,
    side               TEXT NOT NULL CHECK(side IN ('buy','sell')),
    state              TEXT NOT NULL,              -- OrderState JSON
    order_type         TEXT NOT NULL DEFAULT 'marketable_limit',
    qty                TEXT NOT NULL,              -- Decimal as string
    price              TEXT,                       -- nullable: 재호가 전 미정
    filled_qty         TEXT NOT NULL DEFAULT '0',
    submitted_at       TEXT,
    updated_at         TEXT NOT NULL
);

-- positions: 현재 보유 포지션.
-- atr_at_entry: 진입 시점 ATR_14 (손절가 기준, 이후 갱신 없음).
-- trailing_stop_price: 1차 목표 달성 후에만 설정됨.
CREATE TABLE IF NOT EXISTS positions (
    id                   TEXT PRIMARY KEY,
    order_id             TEXT NOT NULL REFERENCES orders(id),
    symbol               TEXT NOT NULL,
    qty                  TEXT NOT NULL,
    entry_price          TEXT NOT NULL,
    stop_price           TEXT NOT NULL,
    trailing_stop_price  TEXT,
    atr_at_entry         TEXT NOT NULL,
    profit_target_1      TEXT NOT NULL,
    profit_target_2      TEXT NOT NULL,
    partial_exit_done    INTEGER NOT NULL DEFAULT 0, -- 1차 익절 완료 여부
    regime_at_entry      TEXT NOT NULL,              -- MarketRegime JSON
    entered_at           TEXT NOT NULL,
    updated_at           TEXT NOT NULL
);

-- session_stats: 당일 통계 (날짜별 1행).
CREATE TABLE IF NOT EXISTS session_stats (
    date                TEXT PRIMARY KEY,  -- 'YYYY-MM-DD' KST
    pnl                 TEXT NOT NULL DEFAULT '0',
    consecutive_losses  INTEGER NOT NULL DEFAULT 0,
    trade_count         INTEGER NOT NULL DEFAULT 0,
    max_drawdown        TEXT NOT NULL DEFAULT '0',
    profile_active      TEXT NOT NULL DEFAULT 'default',
    updated_at          TEXT NOT NULL
);

-- signal_log: 모든 신호 판정 이력 (분석용).
CREATE TABLE IF NOT EXISTS signal_log (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol           TEXT NOT NULL,
    setup_score      INTEGER NOT NULL,
    rule_direction   TEXT,              -- 'long' | 'short' | NULL (미달)
    rule_strength    REAL,
    llm_verdict      TEXT,              -- 'enter' | 'watch' | 'block' | NULL (미호출)
    llm_block_reason TEXT,
    action           TEXT NOT NULL,    -- 'enter' | 'skip' | 'block' | 'expired'
    expire_reason    TEXT,             -- '2_day_elapsed' | 'event_window_ended'
    regime           TEXT NOT NULL,
    logged_at        TEXT NOT NULL
);

-- strategy_stats: 드리프트 모니터링 지표 (섹션 15).
CREATE TABLE IF NOT EXISTS strategy_stats (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    date       TEXT NOT NULL,   -- 'YYYY-MM-DD'
    metric     TEXT NOT NULL,   -- 'score_60_69_winrate' | 'llm_enter_avg_r' | ...
    value      REAL NOT NULL,
    updated_at TEXT NOT NULL
);
```

- [ ] **Step 2: Commit (SQL 파일만 — DB 코드는 다음 Task)**

```bash
git add crates/bot/src/db/
git commit -m "feat(bot): add SQLite migration schema (5 tables)"
```

---

## Task 6: db.rs — 연결 + 마이그레이션 실행

**Files:**
- Create: `crates/bot/src/db.rs` (또는 `crates/bot/src/db/mod.rs`)
- Create: `crates/bot/tests/test_db.rs`

- [ ] **Step 1: 실패할 테스트 작성**

`crates/bot/tests/test_db.rs`:
```rust
use kis_bot::db::connect;
use tempfile::tempdir;

#[tokio::test]
async fn connect_creates_schema() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db").to_str().unwrap().to_string();

    let pool = connect(&db_path).await.expect("connect should succeed");

    // orders 테이블 존재 확인
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='orders'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, 1, "orders table should exist");

    // positions 테이블 존재 확인
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='positions'"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0, 1, "positions table should exist");
}

#[tokio::test]
async fn connect_idempotent() {
    // 동일 경로에 두 번 연결해도 마이그레이션이 중복 실행되지 않아야 함
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db").to_str().unwrap().to_string();

    let _pool1 = connect(&db_path).await.unwrap();
    let pool2 = connect(&db_path).await.unwrap();

    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='signal_log'"
    )
    .fetch_one(&pool2)
    .await
    .unwrap();
    assert_eq!(row.0, 1);
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_db 2>&1 | head -10
```

Expected: compile error

- [ ] **Step 3: db.rs 구현**

`crates/bot/src/db.rs`:
```rust
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::error::BotError;

/// SQLite에 연결하고 마이그레이션을 실행한다.
/// `db_path`는 `~/...` 형태 허용 (shellexpand 적용).
pub async fn connect(db_path: &str) -> Result<SqlitePool, BotError> {
    let expanded = shellexpand::tilde(db_path).into_owned();

    // 부모 디렉터리 생성
    if let Some(parent) = std::path::Path::new(&expanded).parent() {
        tokio::fs::create_dir_all(parent).await.map_err(BotError::Io)?;
    }

    let url = format!("sqlite://{}?mode=rwc", expanded);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    sqlx::migrate!("src/db/migrations").run(&pool).await?;

    Ok(pool)
}
```

`crates/bot/src/lib.rs` 에 추가:
```rust
pub mod db;
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_db
```

Expected: 2 tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/db.rs crates/bot/src/lib.rs crates/bot/tests/test_db.rs
git commit -m "feat(bot): add SQLite connect() with auto-migration"
```

---

## Task 7: lib.rs 정리 + 전체 smoke test

**Files:**
- Modify: `crates/bot/src/lib.rs`
- Create: `crates/bot/tests/test_smoke.rs`

- [ ] **Step 1: lib.rs 최종 상태 확인**

`crates/bot/src/lib.rs` 최종:
```rust
pub mod config;
pub mod db;
pub mod error;
pub mod types;
```

- [ ] **Step 2: smoke test 작성**

`crates/bot/tests/test_smoke.rs`:
```rust
/// Plan 1 전체가 올바르게 연결되는지 확인하는 통합 smoke test.
use kis_bot::{
    config::{BotConfig, ProfileName},
    error::BotError,
    types::{BotState, MarketRegime, OrderState, KillSwitchMode},
};
use std::path::Path;

#[test]
fn all_types_importable() {
    // 컴파일되면 통과
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
```

- [ ] **Step 3: 모든 테스트 실행**

```bash
cargo test -p kis_bot
```

Expected: 전체 테스트 통과 (0 failures)

- [ ] **Step 4: `cargo clippy` 통과 확인**

```bash
cargo clippy -p kis_bot -- -D warnings
```

Expected: 경고 없음 또는 경고가 있어도 0 error

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/lib.rs crates/bot/tests/test_smoke.rs
git commit -m "feat(bot): Plan 1 complete — foundation types, config, DB ready"
```

---

## 검증 체크리스트

Plan 1 완료 후 아래가 모두 true여야 한다:

- [ ] `cargo test -p kis_bot` — 전체 통과
- [ ] `cargo clippy -p kis_bot -- -D warnings` — 에러 없음
- [ ] `cargo check --workspace` — workspace 전체 컴파일
- [ ] `crates/bot/tests/fixtures/bot_config.toml` 존재
- [ ] `crates/bot/src/db/migrations/0001_initial.sql` 존재 (5개 테이블)
