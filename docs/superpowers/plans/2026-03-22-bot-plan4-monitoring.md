# trading-bot Plan 4: Monitoring (Strategy Stats · Alerts · Drift Detection)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 전략 드리프트 감지, 자동 프로파일 전환, AlertRouter(심각도별 채널 분리) 구현으로 봇의 지속적 운영 가능성을 확보한다.

**Architecture:** `StrategyMonitor`는 거래 종료마다 `strategy_stats` DB에 집계 지표를 기록하고 강제 전환 조건과 알림 조건을 평가한다. `AlertRouter`는 `tokio::sync::broadcast`로 `AlertEvent`를 발행하고, CRITICAL/WARN/INFO 심각도별로 채널 정책을 표시한다. 알림 발송(Telegram) 자체는 `trading-server` 담당이므로 이 크레이트는 발행만 한다.

**Tech Stack:** Plan 1-3 + sqlx (strategy_stats 읽기/쓰기), tokio::sync::broadcast (AlertEvent channel)

**Spec reference:** `docs/superpowers/specs/2026-03-22-trading-bot-design.md` — Sections 12 (Profile 자동 전환), 15 (전략 검증 지표, 경보 vs 강제 전환, 알림 채널 정책)

**선행 조건:** Plan 1, 2, 3 완료

---

## File Map

| 파일 | 역할 |
|------|------|
| `crates/bot/src/monitoring/alert.rs` | `AlertRouter` — broadcast 채널 + 심각도별 발행 |
| `crates/bot/src/monitoring/mod.rs` | `StrategyMonitor` — 지표 집계 + 드리프트 감지 + 강제 전환 |
| `crates/bot/tests/test_alert.rs` | AlertRouter 단위 테스트 |
| `crates/bot/tests/test_monitoring.rs` | StrategyMonitor 단위 테스트 |

---

## Task 22: AlertRouter — 심각도별 채널 발행

**Files:**
- Create: `crates/bot/src/monitoring/alert.rs`
- Create: `crates/bot/src/monitoring/mod.rs`
- Create: `crates/bot/tests/test_alert.rs`

- [ ] **Step 1: lib.rs 등록**

`crates/bot/src/lib.rs` 에 추가:
```rust
pub mod monitoring;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_alert.rs`:
```rust
use kis_bot::monitoring::alert::{AlertRouter, AlertSeverity};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn critical_event_received_by_subscriber() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.critical("Kill switch triggered".to_string());

    let event = timeout(Duration::from_millis(100), rx.recv())
        .await
        .expect("should receive")
        .expect("channel should not close");

    assert_eq!(event.severity, AlertSeverity::Critical);
    assert!(event.message.contains("Kill switch"));
}

#[tokio::test]
async fn warn_event_received() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.warn("MDD reached -10%".to_string());

    let event = rx.recv().await.unwrap();
    assert_eq!(event.severity, AlertSeverity::Warn);
}

#[tokio::test]
async fn info_event_received() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.info("LLM underperforming".to_string());

    let event = rx.recv().await.unwrap();
    assert_eq!(event.severity, AlertSeverity::Info);
}

#[tokio::test]
async fn multiple_subscribers_all_receive() {
    let router = AlertRouter::new(64);
    let mut rx1 = router.subscribe();
    let mut rx2 = router.subscribe();

    router.critical("test".to_string());

    let e1 = rx1.recv().await.unwrap();
    let e2 = rx2.recv().await.unwrap();
    assert_eq!(e1.severity, AlertSeverity::Critical);
    assert_eq!(e2.severity, AlertSeverity::Critical);
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_alert 2>&1 | head -10
```

- [ ] **Step 4: alert.rs 구현**

`crates/bot/src/monitoring/alert.rs`:
```rust
use tokio::sync::broadcast;

/// 알림 심각도. 스펙 Section 15 채널 정책:
/// - Critical: Telegram 즉시 + 로그 (trading-server 담당)
/// - Warn: 로그 + Telegram (trading-server 담당)
/// - Info: 로그만 (Telegram 미발송)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertSeverity {
    Critical,
    Warn,
    Info,
}

#[derive(Debug, Clone)]
pub struct AlertMessage {
    pub severity: AlertSeverity,
    pub message: String,
}

/// trading-bot이 발행하는 알림 채널.
/// trading-server가 구독하여 심각도에 따라 Telegram 발송.
/// trading-bot은 발송 성공 여부와 무관하게 채널에 넣고 계속 실행.
#[derive(Clone)]
pub struct AlertRouter {
    sender: broadcast::Sender<AlertMessage>,
}

impl AlertRouter {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AlertMessage> {
        self.sender.subscribe()
    }

    pub fn critical(&self, message: String) {
        self.send(AlertSeverity::Critical, message);
    }

    pub fn warn(&self, message: String) {
        self.send(AlertSeverity::Warn, message);
    }

    pub fn info(&self, message: String) {
        self.send(AlertSeverity::Info, message);
    }

    fn send(&self, severity: AlertSeverity, message: String) {
        // 구독자 없어도 에러 무시 (로그만)
        let _ = self.sender.send(AlertMessage { severity, message });
    }
}
```

`crates/bot/src/monitoring/mod.rs`:
```rust
pub mod alert;
pub mod strategy_monitor;  // 다음 Task
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_alert
```

Expected: 4 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/monitoring/ crates/bot/tests/test_alert.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add AlertRouter with broadcast channel and CRITICAL/WARN/INFO severity"
```

---

## Task 23: StrategyMonitor — 드리프트 감지 + 강제 전환

**Files:**
- Create: `crates/bot/src/monitoring/strategy_monitor.rs`
- Create: `crates/bot/tests/test_monitoring.rs`

- [ ] **Step 1: 실패할 테스트 작성**

`crates/bot/tests/test_monitoring.rs`:
```rust
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
        llm_win_rate_vs_rule: 0.0,  // LLM이 rule-only보다 얼마나 낮은지 (음수 = 부진)
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
    input.mdd_pct = 0.055; // 5.5% > 5%
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
    input.conservative_days_elapsed = 4;  // ≥ 3일
    input.conservative_7d_r = 1.5;        // > +1R
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
    input.conservative_days_elapsed = 2;  // < 3일 쿨다운
    input.conservative_7d_r = 1.5;
    input.consecutive_losses = 0;
    let decisions = evaluate_monitoring(&input);
    assert!(
        !decisions.iter().any(|d| matches!(d, MonitoringDecision::ReturnToDefault)),
        "should not return before cooldown"
    );
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_monitoring 2>&1 | head -10
```

- [ ] **Step 3: strategy_monitor.rs 구현**

`crates/bot/src/monitoring/strategy_monitor.rs`:
```rust
use crate::config::ProfileName;

pub struct MonitoringInput {
    pub current_profile: ProfileName,
    /// 최근 7일 누적 R (손절 기준 R 배수)
    pub rolling_7d_r: f64,
    /// 최대 드로다운 비율 (예: 0.05 = 5%)
    pub mdd_pct: f64,
    /// 동일 레짐 연속 손실 횟수
    pub regime_consecutive_losses: u32,
    /// 최근 30일 누적 R
    pub rolling_30d_r: f64,
    /// LLM ENTER 승률 - rule-only 승률 (음수 = LLM 부진)
    pub llm_win_rate_vs_rule: f64,
    /// Setup Score 80+ 구간 승률
    pub score_80plus_win_rate: f64,
    /// Conservative 모드로 전환된 후 경과 거래일 수
    pub conservative_days_elapsed: u32,
    /// 현재 연속 손실 횟수
    pub consecutive_losses: u32,
    /// Conservative 전환 이후 7일 R
    pub conservative_7d_r: f64,
}

#[derive(Debug, Clone)]
pub enum MonitoringDecision {
    /// Default → Conservative 강제 전환
    ForceConservative { reason: String },
    /// Conservative → Default 복귀 허용 (쿨다운 3거래일 + 조건 충족)
    ReturnToDefault,
    /// 동일 레짐 5연속 손실 → 해당 레짐 7일 중단
    SuspendRegime { days: u32 },
    /// WARN 알림만 발송 (행동 변경 없음)
    WarnAlert { message: String },
    /// INFO 알림만 발송
    InfoAlert { message: String },
}

/// 스펙 Section 15 경보 vs 강제 전환 평가. 순수 함수.
/// 하나의 상황에 여러 DecisionDecision이 동시에 반환될 수 있음.
pub fn evaluate_monitoring(input: &MonitoringInput) -> Vec<MonitoringDecision> {
    let mut decisions = Vec::new();

    // ── 강제 전환 ──────────────────────────────────────────────────────

    if input.current_profile == ProfileName::Default {
        if input.rolling_7d_r < -2.0 {
            decisions.push(MonitoringDecision::ForceConservative {
                reason: format!("7d R = {:.1}R < -2R", input.rolling_7d_r),
            });
        }
        if input.mdd_pct >= 0.05 {
            decisions.push(MonitoringDecision::ForceConservative {
                reason: format!("MDD {:.1}% >= 5%", input.mdd_pct * 100.0),
            });
        }
    }

    if input.regime_consecutive_losses >= 5 {
        decisions.push(MonitoringDecision::SuspendRegime { days: 7 });
    }

    // ── Conservative → Default 복귀 (쿨다운 + 조건 충족) ──────────────

    if input.current_profile == ProfileName::Conservative
        && input.conservative_days_elapsed >= 3
        && input.conservative_7d_r > 1.0
        && input.consecutive_losses == 0
    {
        decisions.push(MonitoringDecision::ReturnToDefault);
    }

    // ── 알림 전용 (행동 변경 없음) ────────────────────────────────────

    if input.rolling_30d_r < -5.0 {
        decisions.push(MonitoringDecision::WarnAlert {
            message: format!(
                "[WARN] 30-day cumulative R below -5R ({:.1}R). Strategy parameter review recommended.",
                input.rolling_30d_r
            ),
        });
    }

    if input.mdd_pct >= 0.10 {
        decisions.push(MonitoringDecision::WarnAlert {
            message: "[WARN] MDD reached -10%. Review strategy before next session.".to_string(),
        });
    }

    if input.llm_win_rate_vs_rule < -0.10 {
        decisions.push(MonitoringDecision::InfoAlert {
            message: "[INFO] LLM ENTER win rate underperforming rule-only by 10%p for 3 weeks. Consider raising setup_score_threshold_llm.".to_string(),
        });
    }

    if input.score_80plus_win_rate < 0.40 {
        decisions.push(MonitoringDecision::InfoAlert {
            message: "[INFO] Score 80+ win rate below 40% for 2 weeks. Check signal thresholds or feature design.".to_string(),
        });
    }

    decisions
}
```

`crates/bot/src/monitoring/mod.rs` 수정:
```rust
pub mod alert;
pub mod strategy_monitor;
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_monitoring
```

Expected: 7 tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/monitoring/strategy_monitor.rs crates/bot/tests/test_monitoring.rs \
        crates/bot/src/monitoring/mod.rs
git commit -m "feat(bot): add StrategyMonitor with drift detection and auto profile switching"
```

---

## Task 24: 전체 통합 확인 + lib.rs 정리

**Files:**
- Modify: `crates/bot/src/lib.rs`
- Create: `crates/bot/tests/test_integration_smoke.rs`

- [ ] **Step 1: lib.rs 최종 상태 확인**

`crates/bot/src/lib.rs`:
```rust
pub mod bot;
pub mod config;
pub mod control;
pub mod db;
pub mod discovery;
pub mod error;
pub mod execution;
pub mod monitoring;
pub mod position;
pub mod qualification;
pub mod regime;
pub mod risk;
pub mod signal;
pub mod types;
```

- [ ] **Step 2: 통합 smoke test 작성**

`crates/bot/tests/test_integration_smoke.rs`:
```rust
/// Plan 1-4 전체가 올바르게 연결되는지 확인.
/// 모든 public API가 컴파일되고 기본 연계가 동작하는지 검증.
use kis_bot::{
    config::{BotConfig, ProfileName},
    monitoring::alert::{AlertRouter, AlertSeverity},
    monitoring::strategy_monitor::{MonitoringInput, evaluate_monitoring},
    qualification::setup_score::{SetupScoreInput, calculate_setup_score},
    regime::{RegimeInput, classify_regime},
    risk::{RiskSizerInput, calculate_size, strength_size_factor},
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
        ma5: 100.0, ma20: 98.0,
        daily_change_pct: 0.8, volume_ratio: 1.0,
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
    let signal = RuleSignal { direction: Direction::Long, strength: 0.80 };
    let decision = SignalDecision::evaluate_without_llm(
        score,
        Some(signal.clone()),
        cfg.signal.rule_strength_threshold,
    );
    assert!(matches!(decision, SignalDecision::Enter { .. }));

    // 5. Risk sizing
    let size = calculate_size(&RiskSizerInput {
        account_balance: dec!(10000),
        risk_per_trade: dec!(0.005),
        atr: dec!(18),
        atr_stop_multiplier: dec!(1.5),
        entry_price: dec!(892),
        strength: signal.strength,
        regime_factor: dec!(1.0),
        profile_factor: dec!(1.0),
        max_position_pct: dec!(0.10),
    });
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
    use kis_bot::monitoring::strategy_monitor::MonitoringDecision;
    let decisions = evaluate_monitoring(&input);
    assert!(decisions.iter().any(|d| matches!(d, MonitoringDecision::ForceConservative { .. })));
}
```

- [ ] **Step 3: 전체 테스트 실행**

```bash
cargo test -p kis_bot
```

Expected: 전체 통과 (0 failures)

- [ ] **Step 4: clippy 통과**

```bash
cargo clippy -p kis_bot -- -D warnings
```

Expected: 에러 없음

- [ ] **Step 5: workspace 전체 컴파일**

```bash
cargo check --workspace
```

Expected: 에러 없음

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/lib.rs crates/bot/tests/test_integration_smoke.rs
git commit -m "feat(bot): Plan 4 complete — monitoring, alert routing, full pipeline smoke test"
```

---

## 검증 체크리스트

Plan 4 (전체 완료) 후 아래가 모두 true여야 한다:

- [ ] `cargo test -p kis_bot` — 전체 통과
- [ ] `cargo clippy -p kis_bot -- -D warnings` — 에러 없음
- [ ] `cargo check --workspace` — workspace 전체 컴파일
- [ ] `AlertRouter` — CRITICAL/WARN/INFO 3가지 심각도, 다중 구독자 수신 검증
- [ ] `StrategyMonitor` — 강제 전환 2가지, 복귀 조건 (쿨다운 포함), 레짐 중단, 알림 전용 2가지 검증
- [ ] 통합 smoke test — 레짐 → 점수 → 신호 → 사이즈 → 알림 전체 파이프라인 연결 확인

---

## 4개 Plan 전체 완료 후 최종 상태

| Plan | 산출물 | 핵심 테스트 수 |
|------|--------|--------------|
| Plan 1: Foundation | Types, Config, DB | ~15개 |
| Plan 2: Signal Pipeline | Regime, Discovery, Score, Rule, LLM, Signal | ~25개 |
| Plan 3: Execution + Control | RiskSizer, OrderSM, Position, KillSwitch, Recovery, BotHandle | ~27개 |
| Plan 4: Monitoring | AlertRouter, StrategyMonitor | ~11개 |

**합계: ~78개 단위 테스트, 0 external API 호출 (통합 테스트는 `KIS_INTEGRATION_TEST=1` 별도)**
