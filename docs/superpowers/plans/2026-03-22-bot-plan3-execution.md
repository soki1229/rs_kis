# trading-bot Plan 3: Execution + Control (Risk · Order SM · Position · Kill Switch · Recovery · Bot Loop)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 리스크 계산부터 실제 주문 제출, 포지션 관리, Kill Switch, RecoveryCheck, graceful shutdown을 포함한 봇 핵심 실행 엔진을 구현한다.

**Architecture:** `RiskSizer`는 순수 함수형으로 수량을 계산하고, `OrderStateMachine`은 상태 전이를 검증한다. `KillSwitch`는 파일 기반으로 재시작 후에도 상태를 유지한다. `RecoveryChecker`는 재시작 시 브로커 상태를 정본으로 정규화한다. `BotRunner`가 모든 tokio task를 조율한다.

**Tech Stack:** Plan 1+2 + rust_decimal (사이즈 계산), sqlx (상태 영속화), tokio::sync::CancellationToken (shutdown), serde_json (kill switch 파일)

**Spec reference:** `docs/superpowers/specs/2026-03-22-trading-bot-design.md` — Sections 5 (Risk Sizing), 6 (Execution), 7 (Position), 8 (Session Risk), 9 (Recovery), 10 (Graceful Shutdown)

**선행 조건:** Plan 1, 2 완료

---

## File Map

| 파일 | 역할 |
|------|------|
| `crates/bot/src/risk/mod.rs` | `RiskSizer` — ATR 기반 수량 계산 |
| `crates/bot/src/risk/guard.rs` | `RiskGuard` — 세션 단위 한도 검사 |
| `crates/bot/src/execution/order_state.rs` | `OrderStateMachine` — 전이 검증 |
| `crates/bot/src/execution/mod.rs` | `ExecutionEngine` — 주문 제출/모니터링 |
| `crates/bot/src/position/mod.rs` | `PositionManager` — 익절/손절/trailing stop |
| `crates/bot/src/control/kill_switch.rs` | `KillSwitch` — 파일 영속화 |
| `crates/bot/src/control/recovery.rs` | `RecoveryChecker` — 5단계 재시작 복구 |
| `crates/bot/src/control/mod.rs` | `SessionRiskControl` — 횡단 감시 |
| `crates/bot/src/bot.rs` | `BotRunner` — 메인 루프 + graceful shutdown |

---

## Task 15: RiskSizer — ATR 기반 수량 계산

**Files:**
- Create: `crates/bot/src/risk/mod.rs`
- Create: `crates/bot/tests/test_risk_sizer.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod risk;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_risk_sizer.rs`:
```rust
use kis_bot::risk::{RiskSizerInput, calculate_size};
use rust_decimal_macros::dec;
use rust_decimal::Decimal;

#[test]
fn basic_size_calculation() {
    // 스펙 Section 5 예시 재현
    // 계좌 $10,000, risk 0.5%, ATR $18, multiplier 1.5, strength 0.80 → 1.15x
    let input = RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.005),
        atr: dec!(18),
        atr_stop_multiplier: dec!(1.5),
        entry_price: dec!(892),
        strength: 0.80,
        regime_factor: dec!(1.0),  // Trending
        profile_factor: dec!(1.0), // Default
        max_position_pct: dec!(0.10),
    };
    let size = calculate_size(&input);
    // base = 50 / 27 ≈ 1.85, × 1.15 ≈ 2.13
    assert!(size > dec!(2.0) && size < dec!(2.5), "size = {}", size);
}

#[test]
fn volatile_regime_halves_size() {
    let input = RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.005),
        atr: dec!(18),
        atr_stop_multiplier: dec!(1.5),
        entry_price: dec!(892),
        strength: 0.70,
        regime_factor: dec!(0.5),  // Volatile
        profile_factor: dec!(1.0),
        max_position_pct: dec!(0.10),
    };
    let volatile_size = calculate_size(&input);

    let trending_input = RiskSizerInput { regime_factor: dec!(1.0), ..input };
    let trending_size = calculate_size(&trending_input);

    // Volatile = Trending × 0.5 (±5% 허용)
    let ratio = volatile_size / trending_size;
    assert!(
        ratio > dec!(0.45) && ratio < dec!(0.55),
        "volatile should be ~50% of trending, got ratio {}", ratio
    );
}

#[test]
fn max_position_pct_caps_size() {
    let input = RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.10),  // 극단적으로 높음
        atr: dec!(1),
        atr_stop_multiplier: dec!(0.1),
        entry_price: dec!(100),
        strength: 1.0,
        regime_factor: dec!(1.0),
        profile_factor: dec!(1.6),
        max_position_pct: dec!(0.10),  // 10% 한도
    };
    let size = calculate_size(&input);
    // 최대 = 10000 × 0.10 / 100 = 10주
    assert!(size <= dec!(10), "capped size should be ≤ 10, got {}", size);
}

#[test]
fn strength_factor_table() {
    use kis_bot::risk::strength_size_factor;
    assert!((strength_size_factor(0.60) - 0.75).abs() < 0.001);
    assert!((strength_size_factor(0.70) - 1.00).abs() < 0.001);
    assert!((strength_size_factor(0.80) - 1.15).abs() < 0.001);
    assert!((strength_size_factor(0.90) - 1.25).abs() < 0.001);
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_risk_sizer 2>&1 | head -10
```

- [ ] **Step 4: risk/mod.rs 구현**

`crates/bot/src/risk/mod.rs`:
```rust
pub mod guard;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct RiskSizerInput {
    pub account_balance: Decimal,
    pub risk_per_trade: Decimal,
    pub atr: Decimal,
    pub atr_stop_multiplier: Decimal,
    pub entry_price: Decimal,
    pub strength: f64,
    /// Trending=1.0, Volatile=0.5, Quiet=N/A
    pub regime_factor: Decimal,
    /// Default=1.0, Conservative=0.6, Aggressive=1.6
    pub profile_factor: Decimal,
    pub max_position_pct: Decimal,
}

/// 스펙 Section 5 최종 사이즈 계산식.
/// final_size = base_size × strength_factor × regime_factor × profile_factor
/// capped at account_balance × max_position_pct / entry_price
pub fn calculate_size(input: &RiskSizerInput) -> Decimal {
    if input.atr == dec!(0) || input.atr_stop_multiplier == dec!(0) {
        return dec!(0);
    }

    let stop_distance = input.atr * input.atr_stop_multiplier;
    let allowed_loss = input.account_balance * input.risk_per_trade;
    let base_size = allowed_loss / stop_distance;

    let sf = Decimal::from_f64_retain(strength_size_factor(input.strength))
        .unwrap_or(dec!(1.0));

    let final_size = base_size * sf * input.regime_factor * input.profile_factor;

    // 포지션 비중 상한
    let max_size = input.account_balance * input.max_position_pct / input.entry_price;

    final_size.min(max_size).max(dec!(0))
}

/// 스펙 Section 5 strength 구간별 사이즈 배율 테이블.
pub fn strength_size_factor(strength: f64) -> f64 {
    match strength {
        s if s >= 0.85 => 1.25,
        s if s >= 0.75 => 1.15,
        s if s >= 0.65 => 1.00,
        _ => 0.75,
    }
}
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_risk_sizer
```

Expected: 4 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/risk/ crates/bot/tests/test_risk_sizer.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add RiskSizer with ATR-based sizing and profile/regime factors"
```

---

## Task 16: OrderStateMachine — 상태 전이 검증

**Files:**
- Create: `crates/bot/src/execution/order_state.rs`
- Create: `crates/bot/src/execution/mod.rs`
- Create: `crates/bot/tests/test_order_state.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod execution;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_order_state.rs`:
```rust
use kis_bot::execution::order_state::{OrderStateMachine, TransitionError};
use kis_bot::types::OrderState;
use rust_decimal_macros::dec;

#[test]
fn pending_to_submitted() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    assert_eq!(sm.state(), &OrderState::Submitted);
}

#[test]
fn submitted_to_fully_filled() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::FullyFilled).unwrap();
    assert_eq!(sm.state(), &OrderState::FullyFilled);
}

#[test]
fn submitted_to_partially_filled_to_fully_filled() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::PartiallyFilled { filled_qty: dec!(1) }).unwrap();
    sm.transition(OrderState::FullyFilled).unwrap();
    assert_eq!(sm.state(), &OrderState::FullyFilled);
}

#[test]
fn partially_filled_to_cancelled_partial() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::PartiallyFilled { filled_qty: dec!(1) }).unwrap();
    sm.transition(OrderState::CancelledPartial { filled_qty: dec!(1) }).unwrap();
    assert!(matches!(sm.state(), OrderState::CancelledPartial { .. }));
}

#[test]
fn invalid_transition_returns_error() {
    let mut sm = OrderStateMachine::new();
    // PendingSubmit → FullyFilled 는 허용 안 됨
    let result = sm.transition(OrderState::FullyFilled);
    assert!(result.is_err());
}

#[test]
fn terminal_state_cannot_transition() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::FullyFilled).unwrap();
    // FullyFilled → 무엇이든 불가
    let result = sm.transition(OrderState::Cancelled);
    assert!(result.is_err());
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_order_state 2>&1 | head -10
```

- [ ] **Step 4: order_state.rs 구현**

`crates/bot/src/execution/order_state.rs`:
```rust
use crate::types::OrderState;

#[derive(Debug, thiserror::Error)]
#[error("Invalid transition from {from:?} to {to:?}")]
pub struct TransitionError {
    pub from: String,
    pub to: String,
}

pub struct OrderStateMachine {
    state: OrderState,
}

impl OrderStateMachine {
    pub fn new() -> Self {
        Self { state: OrderState::PendingSubmit }
    }

    pub fn state(&self) -> &OrderState {
        &self.state
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.state,
            OrderState::FullyFilled
            | OrderState::CancelledPartial { .. }
            | OrderState::Cancelled
            | OrderState::Failed { .. }
        )
    }

    /// 유효하지 않은 전이는 Err 반환. 성공 시 내부 상태 갱신.
    pub fn transition(&mut self, next: OrderState) -> Result<(), TransitionError> {
        if self.is_terminal() {
            return Err(TransitionError {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            });
        }

        let valid = match (&self.state, &next) {
            (OrderState::PendingSubmit, OrderState::Submitted) => true,
            (OrderState::PendingSubmit, OrderState::Failed { .. }) => true,
            (OrderState::Submitted, OrderState::PartiallyFilled { .. }) => true,
            (OrderState::Submitted, OrderState::FullyFilled) => true,
            (OrderState::Submitted, OrderState::Cancelled) => true,
            (OrderState::Submitted, OrderState::Failed { .. }) => true,
            (OrderState::PartiallyFilled { .. }, OrderState::FullyFilled) => true,
            (OrderState::PartiallyFilled { .. }, OrderState::CancelledPartial { .. }) => true,
            _ => false,
        };

        if !valid {
            return Err(TransitionError {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            });
        }

        self.state = next;
        Ok(())
    }
}

impl Default for OrderStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
```

`crates/bot/src/execution/mod.rs`:
```rust
pub mod order_state;
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_order_state
```

Expected: 6 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/execution/ crates/bot/tests/test_order_state.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add OrderStateMachine with valid transition enforcement"
```

---

## Task 17: RiskGuard — 세션 한도 검사

**Files:**
- Create: `crates/bot/src/risk/guard.rs`
- Create: `crates/bot/tests/test_risk_guard.rs`

- [ ] **Step 1: 실패할 테스트 작성**

`crates/bot/tests/test_risk_guard.rs`:
```rust
use kis_bot::risk::guard::{RiskGuard, SessionStats, RiskGuardInput};
use rust_decimal_macros::dec;

fn make_guard(max_positions: u32, daily_loss_limit: f64, consec_limit: u32) -> RiskGuard {
    RiskGuard::new(max_positions, dec!(daily_loss_limit), consec_limit, dec!(0.30))
}

#[test]
fn blocks_when_max_positions_reached() {
    let guard = make_guard(3, 0.015, 3);
    let stats = SessionStats {
        open_positions: 3,
        daily_pnl: dec!(0),
        consecutive_losses: 0,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    let result = guard.check_entry(&stats);
    assert!(result.is_err(), "should block at max_positions");
}

#[test]
fn blocks_when_daily_loss_exceeded() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 0,
        daily_pnl: dec!(-200),  // -200 on $10,000 = -2% > limit 1.5%
        consecutive_losses: 0,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_err());
}

#[test]
fn blocks_when_consecutive_loss_limit_reached() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 0,
        daily_pnl: dec!(0),
        consecutive_losses: 3,  // 3회 도달 = 한도 도달
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_err());
}

#[test]
fn passes_when_all_clear() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 2,
        daily_pnl: dec!(50),
        consecutive_losses: 1,
        account_balance: dec!(10000),
        pending_order_value: dec!(0),
    };
    assert!(guard.check_entry(&stats).is_ok());
}

#[test]
fn hard_kill_switch_when_pending_orders_exceed_30pct() {
    let guard = make_guard(5, 0.015, 3);
    let stats = SessionStats {
        open_positions: 0,
        daily_pnl: dec!(0),
        consecutive_losses: 0,
        account_balance: dec!(10000),
        pending_order_value: dec!(3100),  // 31% > 30%
    };
    let result = guard.check_kill_switch_hard(&stats);
    assert!(result, "should trigger HARD kill switch");
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_risk_guard 2>&1 | head -10
```

- [ ] **Step 3: guard.rs 구현**

`crates/bot/src/risk/guard.rs`:
```rust
use rust_decimal::Decimal;
use crate::error::BotError;

pub struct SessionStats {
    pub open_positions: u32,
    pub daily_pnl: Decimal,
    pub consecutive_losses: u32,
    pub account_balance: Decimal,
    pub pending_order_value: Decimal,
}

pub struct RiskGuard {
    max_open_positions: u32,
    daily_loss_limit: Decimal,      // 계좌 잔고의 비율 (예: 0.015 = 1.5%)
    consecutive_loss_limit: u32,
    pending_order_limit_pct: Decimal, // 예: 0.30 = 30%
}

impl RiskGuard {
    pub fn new(
        max_open_positions: u32,
        daily_loss_limit: Decimal,
        consecutive_loss_limit: u32,
        pending_order_limit_pct: Decimal,
    ) -> Self {
        Self {
            max_open_positions,
            daily_loss_limit,
            consecutive_loss_limit,
            pending_order_limit_pct,
        }
    }

    /// 신규 진입 가능 여부. Err = 차단.
    pub fn check_entry(&self, stats: &SessionStats) -> Result<(), BotError> {
        if stats.open_positions >= self.max_open_positions {
            return Err(BotError::RiskBlocked {
                reason: format!(
                    "max_open_positions {} reached",
                    self.max_open_positions
                ),
            });
        }

        let loss_limit_abs = stats.account_balance * self.daily_loss_limit;
        if stats.daily_pnl < -loss_limit_abs {
            return Err(BotError::RiskBlocked {
                reason: format!(
                    "daily_loss_limit exceeded: pnl={}, limit=-{}",
                    stats.daily_pnl, loss_limit_abs
                ),
            });
        }

        if stats.consecutive_losses >= self.consecutive_loss_limit {
            return Err(BotError::RiskBlocked {
                reason: format!(
                    "consecutive_loss_limit {} reached",
                    self.consecutive_loss_limit
                ),
            });
        }

        Ok(())
    }

    /// 미체결 비중이 30% 초과 → HARD Kill Switch 조건.
    /// true = HARD Kill Switch 즉시 발동 필요.
    pub fn check_kill_switch_hard(&self, stats: &SessionStats) -> bool {
        if stats.account_balance.is_zero() {
            return false;
        }
        let pct = stats.pending_order_value / stats.account_balance;
        pct > self.pending_order_limit_pct
    }
}
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_risk_guard
```

Expected: 5 tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/risk/guard.rs crates/bot/tests/test_risk_guard.rs
git commit -m "feat(bot): add RiskGuard with session-level entry blocking"
```

---

## Task 18: KillSwitch — 파일 기반 영속화

**Files:**
- Create: `crates/bot/src/control/kill_switch.rs`
- Create: `crates/bot/src/control/mod.rs`
- Create: `crates/bot/tests/test_kill_switch.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod control;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_kill_switch.rs`:
```rust
use kis_bot::control::kill_switch::KillSwitch;
use kis_bot::types::KillSwitchMode;
use tempfile::tempdir;

#[test]
fn no_file_means_no_kill_switch() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path);
    assert!(ks.current_mode().is_none());
}

#[test]
fn soft_kill_switch_persists_to_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path.clone());

    ks.activate(KillSwitchMode::Soft, "KIS API 3 errors", "HTTP 503 x3").unwrap();

    let ks2 = KillSwitch::new(path);
    assert_eq!(ks2.current_mode(), Some(KillSwitchMode::Soft));
}

#[test]
fn hard_kill_switch_persists() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path.clone());

    ks.activate(KillSwitchMode::Hard, "balance mismatch", "").unwrap();
    assert_eq!(ks.current_mode(), Some(KillSwitchMode::Hard));
}

#[test]
fn clear_removes_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path);

    ks.activate(KillSwitchMode::Soft, "test", "").unwrap();
    assert!(ks.current_mode().is_some());

    ks.clear().unwrap();
    assert!(ks.current_mode().is_none());
}

#[test]
fn file_content_includes_reason() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path.clone());

    ks.activate(KillSwitchMode::Soft, "KIS API error", "detail info").unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("KIS API error"));
    assert!(content.contains("detail info"));
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_kill_switch 2>&1 | head -10
```

- [ ] **Step 4: kill_switch.rs 구현**

`crates/bot/src/control/kill_switch.rs`:
```rust
use chrono::Utc;
use crate::error::BotError;
use crate::types::{KillSwitchFile, KillSwitchMode};

pub struct KillSwitch {
    path: String,
}

impl KillSwitch {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// 파일 존재 시 파싱. None = Kill Switch 비활성.
    pub fn current_mode(&self) -> Option<KillSwitchMode> {
        let content = std::fs::read_to_string(&self.path).ok()?;
        let file: KillSwitchFile = serde_json::from_str(&content).ok()?;
        Some(file.mode)
    }

    /// Kill Switch 발동. 파일에 JSON으로 기록.
    pub fn activate(&self, mode: KillSwitchMode, reason: &str, details: &str) -> Result<(), BotError> {
        let kst = chrono::FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());
        let file = KillSwitchFile {
            mode,
            reason: reason.to_string(),
            triggered_at: kst,
            details: details.to_string(),
        };
        let json = serde_json::to_string_pretty(&file).map_err(BotError::Json)?;
        std::fs::write(&self.path, json).map_err(BotError::Io)
    }

    /// Kill Switch 해제. 파일 삭제.
    pub fn clear(&self) -> Result<(), BotError> {
        if std::path::Path::new(&self.path).exists() {
            std::fs::remove_file(&self.path).map_err(BotError::Io)?;
        }
        Ok(())
    }
}
```

`crates/bot/src/control/mod.rs`:
```rust
pub mod kill_switch;
pub mod recovery;   // 다음 Task
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_kill_switch
```

Expected: 5 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/control/ crates/bot/tests/test_kill_switch.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add KillSwitch with file-based persistence and SOFT/HARD modes"
```

---

## Task 19: RecoveryChecker — 재시작 복구 절차

**Files:**
- Create: `crates/bot/src/control/recovery.rs`
- Create: `crates/bot/tests/test_recovery.rs`

- [ ] **Step 1: 실패할 테스트 작성**

`crates/bot/tests/test_recovery.rs`:
```rust
use kis_bot::control::recovery::{
    RecoveryInput, RecoveryOutcome, RecoveryFailureCode, run_recovery_check,
};
use rust_decimal_macros::dec;

fn make_input(
    db_total: f64,
    broker_total: f64,
    orphaned: bool,
    unreconciled_fills: usize,
) -> RecoveryInput {
    RecoveryInput {
        db_position_total: dec!(db_total),
        broker_balance_total: dec!(broker_total),
        has_orphaned_submitted_orders: orphaned,
        unreconciled_fill_count: unreconciled_fills,
        has_orders_without_broker_id: false,
        mismatch_threshold_pct: dec!(0.05),
    }
}

#[test]
fn clean_state_passes() {
    let input = make_input(1000.0, 1010.0, false, 0); // 1% diff < 5%
    let result = run_recovery_check(&input);
    assert!(matches!(result, RecoveryOutcome::Pass));
}

#[test]
fn large_balance_mismatch_fails() {
    let input = make_input(1000.0, 1100.0, false, 0); // 10% diff > 5%
    let result = run_recovery_check(&input);
    assert!(matches!(
        result,
        RecoveryOutcome::Fail { code: RecoveryFailureCode::BalanceMismatch, .. }
    ));
}

#[test]
fn orphaned_order_fails() {
    let input = make_input(1000.0, 1000.0, true, 0);
    let result = run_recovery_check(&input);
    assert!(matches!(
        result,
        RecoveryOutcome::Fail { code: RecoveryFailureCode::OrphanedOrder, .. }
    ));
}

#[test]
fn unreconciled_fill_is_auto_fixed_not_fail() {
    // UNRECONCILED_FILL은 Kill Switch 없이 자동 복구
    let input = make_input(1000.0, 1000.0, false, 2);
    let result = run_recovery_check(&input);
    assert!(matches!(result, RecoveryOutcome::AutoFixed { count: 2 }));
}

#[test]
fn missing_broker_id_fails() {
    let mut input = make_input(1000.0, 1000.0, false, 0);
    input.has_orders_without_broker_id = true;
    let result = run_recovery_check(&input);
    assert!(matches!(
        result,
        RecoveryOutcome::Fail { code: RecoveryFailureCode::BrokerOrderMissing, .. }
    ));
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_recovery 2>&1 | head -10
```

- [ ] **Step 3: recovery.rs 구현**

`crates/bot/src/control/recovery.rs`:
```rust
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryFailureCode {
    BalanceMismatch,
    OrphanedOrder,
    BrokerOrderMissing,
}

#[derive(Debug, Clone)]
pub enum RecoveryOutcome {
    /// 모든 검사 통과 → 정상 기동
    Pass,
    /// UNRECONCILED_FILL: DB 상태만 갱신하고 계속 진행
    AutoFixed { count: usize },
    /// Kill Switch 발동 필요
    Fail { code: RecoveryFailureCode, detail: String },
}

pub struct RecoveryInput {
    /// DB `positions` 테이블 총 평가액
    pub db_position_total: Decimal,
    /// 브로커 `balance()` 총 평가액
    pub broker_balance_total: Decimal,
    /// DB에 `SUBMITTED` 상태이나 브로커 `unfilled_orders`에 없는 주문 존재
    pub has_orphaned_submitted_orders: bool,
    /// DB는 `SUBMITTED`이나 브로커에서 체결 완료된 주문 수 (WS 누락)
    pub unreconciled_fill_count: usize,
    /// DB에 `broker_order_id` 없는 `SUBMITTED` 주문 존재
    pub has_orders_without_broker_id: bool,
    /// 불일치 허용 임계값 (예: 0.05 = 5%)
    pub mismatch_threshold_pct: Decimal,
}

/// 스펙 Section 9 재시작 복구 절차 (순수 함수 — 실제 API 호출 없음).
/// RecoveryCheck 재시도(5초×3회)는 호출 측에서 구현.
pub fn run_recovery_check(input: &RecoveryInput) -> RecoveryOutcome {
    // 1순위: broker_order_id 없는 SUBMITTED 주문
    if input.has_orders_without_broker_id {
        return RecoveryOutcome::Fail {
            code: RecoveryFailureCode::BrokerOrderMissing,
            detail: "SUBMITTED orders found without broker_order_id".to_string(),
        };
    }

    // 2순위: 잔고 불일치
    if !input.broker_balance_total.is_zero() {
        let diff = (input.db_position_total - input.broker_balance_total).abs();
        let pct = diff / input.broker_balance_total;
        if pct > input.mismatch_threshold_pct {
            return RecoveryOutcome::Fail {
                code: RecoveryFailureCode::BalanceMismatch,
                detail: format!(
                    "db={} broker={} diff={:.1}%",
                    input.db_position_total,
                    input.broker_balance_total,
                    pct * Decimal::ONE_HUNDRED
                ),
            };
        }
    }

    // 3순위: orphaned order
    if input.has_orphaned_submitted_orders {
        return RecoveryOutcome::Fail {
            code: RecoveryFailureCode::OrphanedOrder,
            detail: "SUBMITTED orders not found in broker unfilled_orders".to_string(),
        };
    }

    // UNRECONCILED_FILL: 자동 복구 (Kill Switch 없음)
    if input.unreconciled_fill_count > 0 {
        return RecoveryOutcome::AutoFixed { count: input.unreconciled_fill_count };
    }

    RecoveryOutcome::Pass
}
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_recovery
```

Expected: 5 tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/control/recovery.rs crates/bot/tests/test_recovery.rs
git commit -m "feat(bot): add RecoveryChecker with 4 failure codes and auto-fix for UNRECONCILED_FILL"
```

---

## Task 20: PositionManager — 익절/손절/trailing stop

**Files:**
- Create: `crates/bot/src/position/mod.rs`
- Create: `crates/bot/tests/test_position.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod position;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_position.rs`:
```rust
use kis_bot::position::{ExitDecision, PositionState, evaluate_exit};
use kis_bot::types::MarketRegime;
use rust_decimal_macros::dec;

fn make_position(entry: f64, atr: f64) -> PositionState {
    PositionState {
        entry_price: dec!(entry),
        stop_price: dec!(entry) - dec!(atr) * dec!(1.5),
        atr_at_entry: dec!(atr),
        profit_target_1: dec!(entry) + dec!(atr) * dec!(2.0),
        profit_target_2: dec!(entry) + dec!(atr) * dec!(4.0),
        trailing_stop_price: None,
        partial_exit_done: false,
        regime: MarketRegime::Trending,
        profit_target_1_atr: dec!(2.0),
        profit_target_2_atr: dec!(4.0),
        trailing_atr_trending: dec!(2.0),
        trailing_atr_volatile: dec!(1.0),
    }
}

#[test]
fn stop_hit_triggers_full_exit() {
    let pos = make_position(100.0, 5.0); // stop = 92.5
    let current_price = dec!(92.0);
    let decision = evaluate_exit(&pos, current_price);
    assert!(matches!(decision, ExitDecision::StopLoss));
}

#[test]
fn first_target_hit_triggers_partial() {
    let pos = make_position(100.0, 5.0); // target1 = 110
    let current_price = dec!(110.5);
    let decision = evaluate_exit(&pos, current_price);
    assert!(matches!(decision, ExitDecision::PartialExit { pct } if pct == dec!(0.5)));
}

#[test]
fn second_target_hit_triggers_full_exit() {
    let mut pos = make_position(100.0, 5.0); // target2 = 120
    pos.partial_exit_done = true;
    let current_price = dec!(121.0);
    let decision = evaluate_exit(&pos, current_price);
    assert!(matches!(decision, ExitDecision::FullExit));
}

#[test]
fn trailing_stop_hit_after_partial_exit() {
    let mut pos = make_position(100.0, 5.0);
    pos.partial_exit_done = true;
    pos.trailing_stop_price = Some(dec!(115.0));
    let current_price = dec!(114.0); // below trailing stop
    let decision = evaluate_exit(&pos, current_price);
    assert!(matches!(decision, ExitDecision::TrailingStop));
}

#[test]
fn no_exit_when_in_normal_range() {
    let pos = make_position(100.0, 5.0);
    let current_price = dec!(105.0); // above stop, below target1
    let decision = evaluate_exit(&pos, current_price);
    assert!(matches!(decision, ExitDecision::Hold));
}

#[test]
fn trailing_stop_price_for_trending_regime() {
    use kis_bot::position::calculate_trailing_stop;
    // 최고가 120, ATR 5, Trending → 최고가 - ATR×2.0 = 110
    let stop = calculate_trailing_stop(dec!(120), dec!(5), &MarketRegime::Trending, dec!(2.0), dec!(1.0));
    assert_eq!(stop, dec!(110));
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_position 2>&1 | head -10
```

- [ ] **Step 4: position/mod.rs 구현**

`crates/bot/src/position/mod.rs`:
```rust
use rust_decimal::Decimal;
use crate::types::MarketRegime;

pub struct PositionState {
    pub entry_price: Decimal,
    pub stop_price: Decimal,
    pub atr_at_entry: Decimal,
    pub profit_target_1: Decimal,
    pub profit_target_2: Decimal,
    pub trailing_stop_price: Option<Decimal>,
    pub partial_exit_done: bool,
    pub regime: MarketRegime,
    pub profit_target_1_atr: Decimal,
    pub profit_target_2_atr: Decimal,
    pub trailing_atr_trending: Decimal,
    pub trailing_atr_volatile: Decimal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitDecision {
    /// 손절가 도달 → 시장가 전량 청산
    StopLoss,
    /// 1차 목표가 도달 → 50% 부분 익절. pct = 0.5
    PartialExit { pct: Decimal },
    /// 2차 목표가 도달 (1차 익절 후) → 잔여 전량 청산
    FullExit,
    /// Trailing stop 도달 → 잔여 전량 청산
    TrailingStop,
    /// 아무 조건도 해당 없음 → 보유 유지
    Hold,
}

/// 스펙 Section 7 기반 익절/손절 판정. 순수 함수.
/// 우선순위: StopLoss > FullExit > TrailingStop > PartialExit > Hold
pub fn evaluate_exit(pos: &PositionState, current_price: Decimal) -> ExitDecision {
    // 손절가 도달
    if current_price <= pos.stop_price {
        return ExitDecision::StopLoss;
    }

    if pos.partial_exit_done {
        // 2차 목표가 도달
        if current_price >= pos.profit_target_2 {
            return ExitDecision::FullExit;
        }
        // Trailing stop 도달 (1차 익절 후에만 적용)
        if let Some(ts) = pos.trailing_stop_price {
            if current_price <= ts {
                return ExitDecision::TrailingStop;
            }
        }
    } else {
        // 1차 목표가 도달
        if current_price >= pos.profit_target_1 {
            return ExitDecision::PartialExit { pct: Decimal::new(5, 1) }; // 0.5
        }
    }

    ExitDecision::Hold
}

/// 레짐별 trailing stop 가격 계산. 스펙 Section 7 기반.
/// trailing_stop = 최고가 - ATR × multiplier
pub fn calculate_trailing_stop(
    high_price: Decimal,
    atr: Decimal,
    regime: &MarketRegime,
    trending_multiplier: Decimal,
    volatile_multiplier: Decimal,
) -> Decimal {
    let multiplier = match regime {
        MarketRegime::Trending => trending_multiplier,
        MarketRegime::Volatile => volatile_multiplier,
        MarketRegime::Quiet => return high_price, // Quiet: trailing stop 없음 (이 함수 호출 안 함)
    };
    high_price - atr * multiplier
}
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_position
```

Expected: 6 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/position/ crates/bot/tests/test_position.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add PositionManager with ATR-based exit and trailing stop"
```

---

## Task 21: BotRunner 뼈대 + graceful shutdown

**Files:**
- Create: `crates/bot/src/bot.rs`
- Create: `crates/bot/tests/test_bot_shutdown.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod bot;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_bot_shutdown.rs`:
```rust
use kis_bot::bot::BotHandle;
use tokio_util::sync::CancellationToken;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn cancellation_token_stops_runner() {
    let token = CancellationToken::new();
    let handle = BotHandle::new(token.clone());

    // 1초 내에 shutdown 완료되어야 함
    token.cancel();
    let result = timeout(Duration::from_secs(1), handle.wait()).await;
    assert!(result.is_ok(), "shutdown should complete within 1 second");
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_bot_shutdown 2>&1 | head -10
```

- [ ] **Step 4: bot.rs 구현 (뼈대 + shutdown)**

`crates/bot/src/bot.rs`:
```rust
use tokio_util::sync::CancellationToken;

/// BotRunner의 외부 제어 핸들.
/// Plan 3 이후 메서드를 추가하며 확장.
pub struct BotHandle {
    token: CancellationToken,
    _task: tokio::task::JoinHandle<()>,
}

impl BotHandle {
    pub fn new(token: CancellationToken) -> Self {
        let t = token.clone();
        let task = tokio::spawn(async move {
            // 메인 루프 — shutdown 신호까지 대기
            t.cancelled().await;
            tracing::info!("Bot shutdown signal received");
        });
        Self { token, _task: task }
    }

    pub async fn wait(self) {
        let _ = self._task.await;
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }
}
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_bot_shutdown
```

Expected: 1 test pass

- [ ] **Step 6: 전체 테스트 확인**

```bash
cargo test -p kis_bot
```

Expected: 전체 통과 (0 failures)

- [ ] **Step 7: Commit**

```bash
git add crates/bot/src/bot.rs crates/bot/tests/test_bot_shutdown.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add BotHandle skeleton with CancellationToken shutdown"
```

---

## 검증 체크리스트

Plan 3 완료 후 아래가 모두 true여야 한다:

- [ ] `cargo test -p kis_bot` — 전체 통과
- [ ] `cargo clippy -p kis_bot -- -D warnings` — 에러 없음
- [ ] `RiskSizer::calculate_size()` — 4개 케이스 (기본 계산, Volatile 50%, 상한 캡, strength 테이블) 검증
- [ ] `OrderStateMachine` — 유효/무효 전이 6개 검증
- [ ] `RiskGuard` — 3가지 차단 + 1가지 통과 + HARD 트리거 검증
- [ ] `KillSwitch` — 파일 생성/읽기/삭제 5개 검증
- [ ] `RecoveryChecker` — 4가지 실패 코드 + Pass + AutoFixed 검증
- [ ] `PositionManager` — StopLoss/PartialExit/FullExit/TrailingStop/Hold 6개 검증
- [ ] `BotHandle` — CancellationToken 기반 shutdown 1초 내 완료 검증
