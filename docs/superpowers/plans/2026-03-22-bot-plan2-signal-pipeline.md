# trading-bot Plan 2: Signal Pipeline (Regime · Discovery · Qualification · Signal Engine)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 종목 풀 발굴부터 "지금 진입할까?" 판정까지의 신호 파이프라인 전체를 구현한다. Plan 1 Foundation이 완료된 상태에서 시작.

**Architecture:** 각 모듈은 독립적인 struct로 구현되며 `Arc<BotConfig>`와 `SqlitePool`을 공유한다. 모든 로직은 KIS/Finnhub 연동 없이 fixture 데이터로 단위 테스트한다. 실제 API 호출은 통합 테스트 경로(`KIS_INTEGRATION_TEST=1`)에서만.

**Tech Stack:** Plan 1 + kis_api (price, volume_surge, price_ranking, minute_chart), reqwest (Finnhub REST), chrono (KST), tokio::sync::watch (regime broadcast), tokio::sync::mpsc (discovery events)

**Spec reference:** `docs/superpowers/specs/2026-03-22-trading-bot-design.md` — Sections 1 (Regime), 2 (Universe Discovery), 3 (Entry Qualification + Setup Score), 4 (Signal Engine)

**선행 조건:** Plan 1 완료 (`cargo test -p kis_bot` 전체 통과)

---

## File Map

| 파일 | 역할 |
|------|------|
| `crates/bot/src/regime/mod.rs` | `MarketRegimeFilter` — KIS 지수 기반 레짐 판정 |
| `crates/bot/src/discovery/mod.rs` | `Watchlist` + `WatchlistEntry` + TTL 관리 |
| `crates/bot/src/qualification/hard_block.rs` | HARD BLOCK 필터 (이벤트, 블랙아웃, 등락률) |
| `crates/bot/src/qualification/setup_score.rs` | `SetupScoreCalculator` (가산/감점 합산) |
| `crates/bot/src/qualification/mod.rs` | `EntryQualifier` (HARD BLOCK + Score 실행) |
| `crates/bot/src/signal/rule_engine.rs` | `RuleEngine` — strength 계산 |
| `crates/bot/src/signal/llm_engine.rs` | `LlmEngine` — Claude Haiku REST 호출 |
| `crates/bot/src/signal/mod.rs` | `SignalEngine` — qualifier 통과 후 진입 판정 |

---

## Task 8: MarketRegimeFilter

**Files:**
- Create: `crates/bot/src/regime/mod.rs`
- Create: `crates/bot/tests/test_regime.rs`

- [ ] **Step 1: lib.rs에 모듈 등록**

`crates/bot/src/lib.rs` 에 추가:
```rust
pub mod regime;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_regime.rs`:
```rust
use kis_bot::regime::{RegimeInput, classify_regime};
use kis_bot::types::MarketRegime;

#[test]
fn trending_when_ma5_above_ma20_normal_volume() {
    let input = RegimeInput {
        ma5: 100.0,
        ma20: 98.0,
        daily_change_pct: 0.8,  // ±1.5% 미만
        volume_ratio: 1.0,      // 정상
    };
    assert_eq!(classify_regime(&input), MarketRegime::Trending);
}

#[test]
fn volatile_when_large_daily_move() {
    let input = RegimeInput {
        ma5: 100.0,
        ma20: 98.0,
        daily_change_pct: 2.0,  // +2.0% > +1.5%
        volume_ratio: 1.0,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Volatile);
}

#[test]
fn volatile_when_large_drop() {
    let input = RegimeInput {
        ma5: 98.0,
        ma20: 100.0,
        daily_change_pct: -1.8,  // -1.8% < -1.5%
        volume_ratio: 1.0,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Volatile);
}

#[test]
fn quiet_when_tiny_move_low_volume() {
    let input = RegimeInput {
        ma5: 98.0,
        ma20: 100.0,
        daily_change_pct: 0.2,  // ±0.3% 미만
        volume_ratio: 0.5,      // 저조
    };
    assert_eq!(classify_regime(&input), MarketRegime::Quiet);
}

#[test]
fn volatile_takes_priority_over_quiet() {
    // 등락률이 크면 거래량 무관하게 Volatile
    let input = RegimeInput {
        ma5: 98.0,
        ma20: 100.0,
        daily_change_pct: -2.0,
        volume_ratio: 0.3,
    };
    assert_eq!(classify_regime(&input), MarketRegime::Volatile);
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_regime 2>&1 | head -10
```

- [ ] **Step 4: regime/mod.rs 구현**

`crates/bot/src/regime/mod.rs`:
```rust
use crate::types::MarketRegime;

/// KIS API로부터 계산된 시장 지표 스냅샷.
/// NASD/S&P500 지수 기반으로 1시간마다 갱신.
pub struct RegimeInput {
    /// 나스닥 지수 5일 이동평균
    pub ma5: f64,
    /// 나스닥 지수 20일 이동평균
    pub ma20: f64,
    /// 당일 지수 등락률 (%)
    pub daily_change_pct: f64,
    /// 시장 거래량 / 20일 평균 거래량 비율
    pub volume_ratio: f64,
}

/// 스펙 Section 1 판정 로직.
/// Volatile 판정이 Quiet보다 우선 (등락률이 크면 거래량 무관).
pub fn classify_regime(input: &RegimeInput) -> MarketRegime {
    let abs_change = input.daily_change_pct.abs();

    // 1순위: 등락률 ±1.5% 이상 → Volatile
    if abs_change >= 1.5 {
        return MarketRegime::Volatile;
    }

    // 2순위: 등락률 ±0.3% 미만 + 거래량 저조 → Quiet
    if abs_change < 0.3 && input.volume_ratio < 0.8 {
        return MarketRegime::Quiet;
    }

    // 기본: Trending
    MarketRegime::Trending
}

/// 레짐 상태를 태스크 간 공유하기 위한 watch channel 타입 별칭.
pub type RegimeSender = tokio::sync::watch::Sender<MarketRegime>;
pub type RegimeReceiver = tokio::sync::watch::Receiver<MarketRegime>;

pub fn regime_channel(initial: MarketRegime) -> (RegimeSender, RegimeReceiver) {
    tokio::sync::watch::channel(initial)
}
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_regime
```

Expected: 5 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/regime/ crates/bot/tests/test_regime.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add MarketRegimeFilter with classify_regime()"
```

---

## Task 9: Watchlist + Universe Discovery

**Files:**
- Create: `crates/bot/src/discovery/mod.rs`
- Create: `crates/bot/tests/test_discovery.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod discovery;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_discovery.rs`:
```rust
use kis_bot::discovery::{Watchlist, WatchlistEntry, AddReason};
use chrono::Utc;

#[test]
fn add_and_contains() {
    let mut wl = Watchlist::new(30);
    wl.add("NVDA".to_string(), AddReason::VolumeSurge);
    assert!(wl.contains("NVDA"));
}

#[test]
fn max_capacity_enforced() {
    let mut wl = Watchlist::new(2);
    wl.add("AAPL".to_string(), AddReason::PriceRanking);
    wl.add("NVDA".to_string(), AddReason::VolumeSurge);
    wl.add("TSLA".to_string(), AddReason::News);   // 3번째 → 거절
    assert_eq!(wl.len(), 2);
}

#[test]
fn expired_entries_removed() {
    let mut wl = Watchlist::new(30);
    // 2 거래일 전 (48h)에 추가된 항목처럼 시뮬레이션
    let old_time = Utc::now() - chrono::Duration::hours(49);
    wl.add_with_time("AAPL".to_string(), AddReason::VolumeSurge, old_time);
    wl.remove_expired();
    assert!(!wl.contains("AAPL"));
}

#[test]
fn active_entries_not_removed() {
    let mut wl = Watchlist::new(30);
    wl.add("NVDA".to_string(), AddReason::VolumeSurge);
    wl.remove_expired();
    assert!(wl.contains("NVDA"));
}

#[test]
fn duplicate_add_refreshes_ttl() {
    let mut wl = Watchlist::new(30);
    let old_time = Utc::now() - chrono::Duration::hours(49);
    wl.add_with_time("AAPL".to_string(), AddReason::VolumeSurge, old_time);
    // 새로 추가하면 TTL 리셋
    wl.add("AAPL".to_string(), AddReason::News);
    wl.remove_expired();
    assert!(wl.contains("AAPL"), "re-added entry should survive expiry check");
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_discovery 2>&1 | head -10
```

- [ ] **Step 4: discovery/mod.rs 구현**

`crates/bot/src/discovery/mod.rs`:
```rust
use chrono::{DateTime, Utc};
use std::collections::HashMap;

const TTL_HOURS: i64 = 48; // 2 거래일

#[derive(Debug, Clone)]
pub enum AddReason {
    VolumeSurge,
    PriceRanking,
    News,
}

#[derive(Debug, Clone)]
pub struct WatchlistEntry {
    pub symbol: String,
    pub reason: AddReason,
    pub added_at: DateTime<Utc>,
    /// 실적 이벤트 추적 종목은 Some(이벤트 종료 시각) → TTL 연장
    pub event_end: Option<DateTime<Utc>>,
}

impl WatchlistEntry {
    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        if let Some(end) = self.event_end {
            return now > end;
        }
        now - self.added_at > chrono::Duration::hours(TTL_HOURS)
    }
}

pub struct Watchlist {
    entries: HashMap<String, WatchlistEntry>,
    max_size: usize,
}

impl Watchlist {
    pub fn new(max_size: usize) -> Self {
        Self { entries: HashMap::new(), max_size }
    }

    pub fn add(&mut self, symbol: String, reason: AddReason) {
        self.add_with_time(symbol, reason, Utc::now());
    }

    pub fn add_with_time(&mut self, symbol: String, reason: AddReason, time: DateTime<Utc>) {
        // 이미 있으면 TTL 갱신
        if self.entries.contains_key(&symbol) {
            if let Some(e) = self.entries.get_mut(&symbol) {
                e.added_at = time;
            }
            return;
        }
        // 최대 크기 초과 시 거절
        if self.entries.len() >= self.max_size {
            return;
        }
        self.entries.insert(symbol.clone(), WatchlistEntry {
            symbol,
            reason,
            added_at: time,
            event_end: None,
        });
    }

    pub fn extend_event_window(&mut self, symbol: &str, end: DateTime<Utc>) {
        if let Some(e) = self.entries.get_mut(symbol) {
            e.event_end = Some(end);
        }
    }

    pub fn remove_expired(&mut self) {
        let now = Utc::now();
        self.entries.retain(|_, e| !e.is_expired(now));
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.entries.contains_key(symbol)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn symbols(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }
}
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_discovery
```

Expected: 5 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/discovery/ crates/bot/tests/test_discovery.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add Watchlist with 2-trading-day TTL and capacity limit"
```

---

## Task 10: HARD BLOCK 필터

**Files:**
- Create: `crates/bot/src/qualification/hard_block.rs`
- Create: `crates/bot/src/qualification/mod.rs`
- Create: `crates/bot/tests/test_hard_block.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod qualification;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_hard_block.rs`:
```rust
use kis_bot::qualification::hard_block::{HardBlockChecker, HardBlockInput};
use chrono::{TimeZone, Utc};

fn make_input(
    daily_change_pct: f64,
    mins_since_open: i64,
    mins_until_close: i64,
    has_earnings_today: bool,
) -> HardBlockInput {
    HardBlockInput {
        daily_change_pct,
        mins_since_open,
        mins_until_close,
        has_earnings_event: has_earnings_today,
        has_fomc_today: false,
        entry_blackout_open_mins: 15,
        entry_blackout_close_mins: 15,
    }
}

#[test]
fn blocks_earnings_event() {
    let checker = HardBlockChecker;
    let input = make_input(0.5, 60, 300, true);
    assert!(checker.check(&input).is_some(), "earnings should block");
}

#[test]
fn blocks_open_blackout() {
    let checker = HardBlockChecker;
    let input = make_input(0.5, 10, 300, false); // 10분 < 15분
    assert!(checker.check(&input).is_some());
}

#[test]
fn blocks_close_blackout() {
    let checker = HardBlockChecker;
    let input = make_input(0.5, 300, 10, false); // 10분 < 15분
    assert!(checker.check(&input).is_some());
}

#[test]
fn blocks_large_daily_gain() {
    let checker = HardBlockChecker;
    let input = make_input(11.0, 60, 300, false); // > 10%
    assert!(checker.check(&input).is_some());
}

#[test]
fn blocks_large_daily_loss() {
    let checker = HardBlockChecker;
    let input = make_input(-3.5, 60, 300, false); // < -3%
    assert!(checker.check(&input).is_some());
}

#[test]
fn passes_normal_conditions() {
    let checker = HardBlockChecker;
    let input = make_input(2.0, 60, 300, false);
    assert!(checker.check(&input).is_none(), "normal conditions should pass");
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_hard_block 2>&1 | head -10
```

- [ ] **Step 4: hard_block.rs 구현**

`crates/bot/src/qualification/hard_block.rs`:
```rust
/// HARD BLOCK 발동 사유. 해당되면 Setup Score 계산 없이 즉시 종료.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardBlockReason {
    EarningsEvent,
    FomcOrCpi,
    OpenBlackout,
    CloseBlackout,
    DailyGainExceeded,   // > +10%
    DailyLossExceeded,   // < -3%
}

pub struct HardBlockInput {
    pub daily_change_pct: f64,
    pub mins_since_open: i64,
    pub mins_until_close: i64,
    pub has_earnings_event: bool,
    pub has_fomc_today: bool,
    pub entry_blackout_open_mins: i64,
    pub entry_blackout_close_mins: i64,
}

pub struct HardBlockChecker;

impl HardBlockChecker {
    /// None = 통과. Some(reason) = 차단.
    pub fn check(&self, input: &HardBlockInput) -> Option<HardBlockReason> {
        if input.has_earnings_event {
            return Some(HardBlockReason::EarningsEvent);
        }
        if input.has_fomc_today {
            return Some(HardBlockReason::FomcOrCpi);
        }
        if input.mins_since_open < input.entry_blackout_open_mins {
            return Some(HardBlockReason::OpenBlackout);
        }
        if input.mins_until_close < input.entry_blackout_close_mins {
            return Some(HardBlockReason::CloseBlackout);
        }
        if input.daily_change_pct > 10.0 {
            return Some(HardBlockReason::DailyGainExceeded);
        }
        if input.daily_change_pct < -3.0 {
            return Some(HardBlockReason::DailyLossExceeded);
        }
        None
    }
}
```

`crates/bot/src/qualification/mod.rs`:
```rust
pub mod hard_block;
pub mod setup_score;  // 다음 Task에서 추가
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_hard_block
```

Expected: 6 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/qualification/ crates/bot/tests/test_hard_block.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add HardBlockChecker with 5 block conditions"
```

---

## Task 11: SetupScoreCalculator

**Files:**
- Create: `crates/bot/src/qualification/setup_score.rs`
- Create: `crates/bot/tests/test_setup_score.rs`

- [ ] **Step 1: 실패할 테스트 작성**

`crates/bot/tests/test_setup_score.rs`:
```rust
use kis_bot::qualification::setup_score::{SetupScoreInput, calculate_setup_score};
use kis_bot::types::MarketRegime;

fn base_input() -> SetupScoreInput {
    SetupScoreInput {
        ma5_above_ma20: false,
        volume_ratio: 1.0,
        recent_5min_volume_ratio: 1.0,
        bid_ask_imbalance: 1.0,
        new_high_last_10min: false,
        has_news_catalyst: false,
        daily_change_pct: 2.0,
        mins_until_close: 300,
        entry_blackout_close_mins: 15,
        regime: MarketRegime::Trending,
    }
}

#[test]
fn zero_score_when_no_conditions_met() {
    let score = calculate_setup_score(&base_input());
    assert_eq!(score, 0);
}

#[test]
fn ma5_above_ma20_adds_20() {
    let mut input = base_input();
    input.ma5_above_ma20 = true;
    assert_eq!(calculate_setup_score(&input), 20);
}

#[test]
fn volume_ratio_2x_adds_20() {
    let mut input = base_input();
    input.volume_ratio = 2.0;
    assert_eq!(calculate_setup_score(&input), 20);
}

#[test]
fn all_positive_conditions_max_100() {
    let input = SetupScoreInput {
        ma5_above_ma20: true,       // +20
        volume_ratio: 2.5,          // +20
        recent_5min_volume_ratio: 1.6, // +15
        bid_ask_imbalance: 1.4,     // +20
        new_high_last_10min: true,  // +15
        has_news_catalyst: true,    // +10
        daily_change_pct: 2.0,      // 감점 없음
        mins_until_close: 300,
        entry_blackout_close_mins: 15,
        regime: MarketRegime::Trending,
    };
    assert_eq!(calculate_setup_score(&input), 100);
}

#[test]
fn overheat_daily_gain_subtracts_15() {
    let input = SetupScoreInput {
        ma5_above_ma20: true,  // +20
        daily_change_pct: 7.5, // > 7% → -15
        ..base_input()
    };
    assert_eq!(calculate_setup_score(&input), 5); // 20 - 15 = 5
}

#[test]
fn volatile_regime_subtracts_10() {
    let input = SetupScoreInput {
        ma5_above_ma20: true, // +20
        regime: MarketRegime::Volatile, // -10
        ..base_input()
    };
    assert_eq!(calculate_setup_score(&input), 10);
}

#[test]
fn score_can_go_negative() {
    let input = SetupScoreInput {
        daily_change_pct: 8.0,       // -15
        regime: MarketRegime::Volatile, // -10
        mins_until_close: 20,        // < 15*2=30 → -10
        entry_blackout_close_mins: 15,
        ..base_input()
    };
    assert_eq!(calculate_setup_score(&input), -35);
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_setup_score 2>&1 | head -10
```

- [ ] **Step 3: setup_score.rs 구현**

`crates/bot/src/qualification/setup_score.rs`:
```rust
use crate::types::MarketRegime;

pub struct SetupScoreInput {
    // ── 가산 조건 ──────────────────────────────────────────
    /// 5일 MA > 20일 MA
    pub ma5_above_ma20: bool,
    /// 거래량 / 20일 평균 거래량 비율 (≥ 2.0x → +20)
    pub volume_ratio: f64,
    /// 최근 5분 거래량 / 이전 5분 거래량 (≥ 1.5x → +15)
    pub recent_5min_volume_ratio: f64,
    /// 매수잔량 / 매도잔량 (≥ 1.3 → +20)
    pub bid_ask_imbalance: f64,
    /// 최근 10분 내 신고가 돌파 여부
    pub new_high_last_10min: bool,
    /// 24h 이내 뉴스 촉매 존재
    pub has_news_catalyst: bool,

    // ── 감점 조건 ──────────────────────────────────────────
    /// 당일 등락률 (> +7% → -15)
    pub daily_change_pct: f64,
    /// 장 마감까지 남은 분 (< blackout*2 → -10)
    pub mins_until_close: i64,
    pub entry_blackout_close_mins: i64,
    /// Volatile 레짐 → -10
    pub regime: MarketRegime,
}

/// 스펙 Section 3-2 기반 Setup Score 계산.
/// 반환값: i32 (음수 가능)
pub fn calculate_setup_score(input: &SetupScoreInput) -> i32 {
    let mut score: i32 = 0;

    // 가산
    if input.ma5_above_ma20 { score += 20; }
    if input.volume_ratio >= 2.0 { score += 20; }
    if input.recent_5min_volume_ratio >= 1.5 { score += 15; }
    if input.bid_ask_imbalance >= 1.3 { score += 20; }
    if input.new_high_last_10min { score += 15; }
    if input.has_news_catalyst { score += 10; }

    // 감점
    if input.daily_change_pct > 7.0 { score -= 15; }
    if input.mins_until_close < input.entry_blackout_close_mins * 2 { score -= 10; }
    if input.regime == MarketRegime::Volatile { score -= 10; }

    score
}
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_setup_score
```

Expected: 7 tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/qualification/setup_score.rs crates/bot/tests/test_setup_score.rs
git commit -m "feat(bot): add SetupScoreCalculator with all spec conditions"
```

---

## Task 12: RuleEngine

**Files:**
- Create: `crates/bot/src/signal/rule_engine.rs`
- Create: `crates/bot/src/signal/mod.rs`
- Create: `crates/bot/tests/test_rule_engine.rs`

- [ ] **Step 1: lib.rs 등록**

```rust
pub mod signal;
```

- [ ] **Step 2: 실패할 테스트 작성**

`crates/bot/tests/test_rule_engine.rs`:
```rust
use kis_bot::signal::rule_engine::{RuleEngineInput, RuleEngine};
use kis_bot::types::Direction;

fn consistent_long_input() -> RuleEngineInput {
    RuleEngineInput {
        // 최근 5분봉 방향: 연속 상승
        recent_candle_directions: vec![true, true, true, true, true],
        // 현재가가 전일 종가 대비 ATR의 30% 위에 있음
        price_vs_prev_close_atr_ratio: 0.30,
        // 거래량 가속도: 최근 3구간 순증
        volume_acceleration: vec![1.0, 1.2, 1.5],
    }
}

#[test]
fn strong_long_signal_high_strength() {
    let engine = RuleEngine;
    let signal = engine.evaluate(&consistent_long_input());
    assert!(signal.is_some());
    let s = signal.unwrap();
    assert_eq!(s.direction, Direction::Long);
    assert!(s.strength >= 0.75, "consistent bullish should be strong: {}", s.strength);
}

#[test]
fn weak_signal_below_threshold() {
    let engine = RuleEngine;
    let input = RuleEngineInput {
        recent_candle_directions: vec![true, false, true, false, true], // 50/50
        price_vs_prev_close_atr_ratio: 0.05,
        volume_acceleration: vec![1.0, 0.9, 1.0],
    };
    let signal = engine.evaluate(&input);
    // 혼조 신호는 None 또는 strength < 0.55
    if let Some(s) = signal {
        assert!(s.strength < 0.55, "mixed signal should be weak: {}", s.strength);
    }
}

#[test]
fn consistent_down_gives_short_signal() {
    let engine = RuleEngine;
    let input = RuleEngineInput {
        recent_candle_directions: vec![false, false, false, false, false],
        price_vs_prev_close_atr_ratio: -0.40,
        volume_acceleration: vec![1.0, 1.3, 1.6],
    };
    let signal = engine.evaluate(&input);
    assert!(signal.is_some());
    assert_eq!(signal.unwrap().direction, Direction::Short);
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_rule_engine 2>&1 | head -10
```

- [ ] **Step 4: rule_engine.rs 구현**

`crates/bot/src/signal/rule_engine.rs`:
```rust
use crate::types::{Direction, RuleSignal};

pub struct RuleEngineInput {
    /// 최근 N분봉 방향 (true=상승, false=하락)
    pub recent_candle_directions: Vec<bool>,
    /// (현재가 - 전일종가) / ATR_14 비율. 양수=상승 압력.
    pub price_vs_prev_close_atr_ratio: f64,
    /// 최근 3구간 거래량 (가속도 측정용)
    pub volume_acceleration: Vec<f64>,
}

pub struct RuleEngine;

impl RuleEngine {
    /// 스펙 Section 4-1 기반 RuleSignal 계산.
    /// strength = 세 요소 가중 평균 (각 0.0~1.0 정규화).
    /// strength < 0.40 이면 None 반환 (신호 없음).
    pub fn evaluate(&self, input: &RuleEngineInput) -> Option<RuleSignal> {
        let direction_score = self.direction_consistency(&input.recent_candle_directions);
        let price_score = self.price_position_score(input.price_vs_prev_close_atr_ratio);
        let volume_score = self.volume_acceleration_score(&input.volume_acceleration);

        // 가중 평균: 방향 40%, 가격 위치 30%, 거래량 가속 30%
        let raw_strength = direction_score * 0.40 + price_score * 0.30 + volume_score * 0.30;

        if raw_strength < 0.40 {
            return None;
        }

        // 방향 결정: 분봉 방향 다수결
        let up_count = input.recent_candle_directions.iter().filter(|&&d| d).count();
        let direction = if up_count * 2 >= input.recent_candle_directions.len() {
            Direction::Long
        } else {
            Direction::Short
        };

        Some(RuleSignal {
            direction,
            strength: raw_strength.clamp(0.0, 1.0),
        })
    }

    /// 연속성 점수 (0.0~1.0). 5분봉 중 같은 방향이 몇 개인지.
    fn direction_consistency(&self, directions: &[bool]) -> f64 {
        if directions.is_empty() {
            return 0.5;
        }
        let up = directions.iter().filter(|&&d| d).count();
        let majority = up.max(directions.len() - up);
        majority as f64 / directions.len() as f64
    }

    /// 가격 위치 점수 (0.0~1.0). ATR 비율 ±0.5 범위를 0~1로 매핑.
    fn price_position_score(&self, atr_ratio: f64) -> f64 {
        ((atr_ratio + 0.5) / 1.0).clamp(0.0, 1.0)
    }

    /// 거래량 가속도 점수 (0.0~1.0). 증가 추세이면 높음.
    fn volume_acceleration_score(&self, volumes: &[f64]) -> f64 {
        if volumes.len() < 2 {
            return 0.5;
        }
        let increasing = volumes.windows(2).filter(|w| w[1] > w[0]).count();
        increasing as f64 / (volumes.len() - 1) as f64
    }
}
```

`crates/bot/src/signal/mod.rs`:
```rust
pub mod llm_engine;  // 다음 Task
pub mod rule_engine;
```

- [ ] **Step 5: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_rule_engine
```

Expected: 3 tests pass

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/signal/ crates/bot/tests/test_rule_engine.rs crates/bot/src/lib.rs
git commit -m "feat(bot): add RuleEngine with 3-factor strength calculation"
```

---

## Task 13: LlmEngine (Claude Haiku REST 호출)

**Files:**
- Create: `crates/bot/src/signal/llm_engine.rs`
- Create: `crates/bot/tests/test_llm_engine.rs`

- [ ] **Step 1: 실패할 테스트 작성 (응답 파싱 위주)**

`crates/bot/tests/test_llm_engine.rs`:
```rust
use kis_bot::signal::llm_engine::{LlmResponse, parse_llm_response};
use kis_bot::types::LlmVerdict;

#[test]
fn parse_enter_verdict() {
    let json = r#"{"verdict":"ENTER","bull":"strong momentum","bear":"overbought","neutral":"ok","block_reason":null}"#;
    let resp: LlmResponse = parse_llm_response(json).unwrap();
    assert_eq!(resp.verdict, LlmVerdict::Enter);
    assert!(resp.block_reason.is_none());
}

#[test]
fn parse_block_verdict_with_reason() {
    let json = r#"{"verdict":"BLOCK","bull":"","bear":"FDA rejection news","neutral":"","block_reason":"negative catalyst"}"#;
    let resp: LlmResponse = parse_llm_response(json).unwrap();
    assert_eq!(resp.verdict, LlmVerdict::Block);
    assert_eq!(resp.block_reason.as_deref(), Some("negative catalyst"));
}

#[test]
fn parse_watch_verdict() {
    let json = r#"{"verdict":"WATCH","bull":"","bear":"","neutral":"mixed","block_reason":null}"#;
    let resp: LlmResponse = parse_llm_response(json).unwrap();
    assert_eq!(resp.verdict, LlmVerdict::Watch);
}

#[test]
fn invalid_json_returns_error() {
    let result = parse_llm_response("not json");
    assert!(result.is_err());
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_llm_engine 2>&1 | head -10
```

- [ ] **Step 3: llm_engine.rs 구현**

`crates/bot/src/signal/llm_engine.rs`:
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::error::BotError;
use crate::types::LlmVerdict;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmResponse {
    pub verdict: LlmVerdict,
    pub bull: String,
    pub bear: String,
    pub neutral: String,
    pub block_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LlmInput {
    pub symbol: String,
    pub price: f64,
    pub change_pct: f64,
    pub volume_ratio: f64,
    pub ma_state: String,
    pub setup_score: i32,
    pub news_headlines: Vec<String>,
}

pub fn parse_llm_response(json: &str) -> Result<LlmResponse, BotError> {
    serde_json::from_str(json).map_err(BotError::Json)
}

pub struct LlmEngine {
    client: Client,
    api_key: String,
    model: String,
    timeout: Duration,
}

impl LlmEngine {
    pub fn new(api_key: String, model: String, timeout_secs: u64) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub async fn evaluate(&self, input: &LlmInput) -> Result<LlmResponse, BotError> {
        let news_text = input.news_headlines
            .iter()
            .enumerate()
            .map(|(i, h)| format!("  {}. {}", i + 1, h))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "종목: {} | 현재가: {:.2} | 등락률: {:.2}%\n\
             거래량 비율: {:.1}x | MA 상태: {} | Setup Score: {}/100\n\
             최근 뉴스 (24h):\n{}\n\n\
             세 관점에서 각각 판단하라:\n\
             - Bull: 매수 근거 (1~2문장)\n\
             - Bear: 위험 요인 (1~2문장)\n\
             - Neutral: 중립 평가 (1문장)\n\n\
             JSON으로만 응답하라. 형식:\n\
             {{\"verdict\":\"ENTER\"|\"WATCH\"|\"BLOCK\",\"bull\":\"...\",\"bear\":\"...\",\"neutral\":\"...\",\"block_reason\":null|\"...\"}}\n\
             BLOCK인 경우에만 block_reason을 채워라.",
            input.symbol, input.price, input.change_pct,
            input.volume_ratio, input.ma_state, input.setup_score,
            news_text
        );

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 300,
            "messages": [{"role": "user", "content": prompt}]
        });

        let resp = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .timeout(self.timeout)
            .json(&body)
            .send()
            .await
            .map_err(BotError::Http)?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(BotError::Llm(format!("Anthropic API error {}: {}", status, text)));
        }

        let raw: serde_json::Value = resp.json().await.map_err(BotError::Http)?;
        let content = raw["content"][0]["text"]
            .as_str()
            .ok_or_else(|| BotError::Llm("no content in response".to_string()))?;

        parse_llm_response(content)
    }
}
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_llm_engine
```

Expected: 4 tests pass (모두 파싱 테스트, 실제 API 호출 없음)

- [ ] **Step 5: Commit**

```bash
git add crates/bot/src/signal/llm_engine.rs crates/bot/tests/test_llm_engine.rs
git commit -m "feat(bot): add LlmEngine with Claude Haiku REST call and response parsing"
```

---

## Task 14: SignalEngine — 전체 파이프라인 통합

**Files:**
- Modify: `crates/bot/src/signal/mod.rs`
- Create: `crates/bot/tests/test_signal_engine.rs`

- [ ] **Step 1: 실패할 테스트 작성**

`crates/bot/tests/test_signal_engine.rs`:
```rust
use kis_bot::signal::SignalDecision;
use kis_bot::types::{Direction, LlmVerdict};

#[test]
fn score_below_60_is_rejected() {
    let decision = SignalDecision::evaluate_without_llm(
        55,   // setup_score < 60
        None, // no rule signal needed
        0.65,
    );
    assert!(matches!(decision, SignalDecision::Rejected { .. }));
}

#[test]
fn score_60_79_rule_only_passes_with_strong_signal() {
    use kis_bot::types::RuleSignal;
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
    use kis_bot::types::RuleSignal;
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
    use kis_bot::types::RuleSignal;
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
    use kis_bot::types::RuleSignal;
    let signal = RuleSignal { direction: Direction::Long, strength: 0.80 };
    let decision = SignalDecision::evaluate_with_llm(
        90,
        Some(signal),
        0.55,
        LlmVerdict::Enter,
    );
    assert!(matches!(decision, SignalDecision::Enter { .. }));
}
```

- [ ] **Step 2: 테스트 실패 확인**

```bash
cargo test -p kis_bot --test test_signal_engine 2>&1 | head -10
```

- [ ] **Step 3: signal/mod.rs 구현**

`crates/bot/src/signal/mod.rs`:
```rust
pub mod llm_engine;
pub mod rule_engine;

use crate::types::{Direction, LlmVerdict, RuleSignal};

#[derive(Debug, Clone)]
pub enum SignalDecision {
    /// 진입 확정. direction과 strength를 Risk Sizing에 전달.
    Enter { direction: Direction, strength: f64, setup_score: i32 },
    /// 관망 (LLM WATCH 판정). 신호는 유효하지만 지금 진입 안 함.
    Watch { reason: String },
    /// 진입 거절.
    Rejected { reason: String },
}

impl SignalDecision {
    /// LLM 없는 경로 (Setup Score 60~79).
    pub fn evaluate_without_llm(
        setup_score: i32,
        rule_signal: Option<RuleSignal>,
        rule_threshold: f64,
    ) -> Self {
        if setup_score < 60 {
            return Self::Rejected { reason: format!("setup_score {} < 60", setup_score) };
        }
        match rule_signal {
            None => Self::Rejected { reason: "no rule signal".to_string() },
            Some(s) if s.strength < rule_threshold => Self::Rejected {
                reason: format!("strength {:.2} < threshold {:.2}", s.strength, rule_threshold),
            },
            Some(s) => Self::Enter {
                direction: s.direction,
                strength: s.strength,
                setup_score,
            },
        }
    }

    /// LLM 포함 경로 (Setup Score ≥ 80).
    pub fn evaluate_with_llm(
        setup_score: i32,
        rule_signal: Option<RuleSignal>,
        rule_threshold: f64,
        llm_verdict: LlmVerdict,
    ) -> Self {
        // LLM BLOCK은 rule signal 무관하게 항상 우선
        if llm_verdict == LlmVerdict::Block {
            return Self::Rejected { reason: "LLM verdict: BLOCK".to_string() };
        }

        match rule_signal {
            None => Self::Rejected { reason: "no rule signal".to_string() },
            Some(s) if s.strength < rule_threshold => Self::Rejected {
                reason: format!("strength {:.2} < threshold {:.2}", s.strength, rule_threshold),
            },
            Some(s) => {
                if llm_verdict == LlmVerdict::Watch {
                    Self::Watch { reason: "LLM verdict: WATCH".to_string() }
                } else {
                    Self::Enter {
                        direction: s.direction,
                        strength: s.strength,
                        setup_score,
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 4: 테스트 통과 확인**

```bash
cargo test -p kis_bot --test test_signal_engine
```

Expected: 5 tests pass

- [ ] **Step 5: 전체 테스트 확인**

```bash
cargo test -p kis_bot
```

Expected: 전체 통과

- [ ] **Step 6: Commit**

```bash
git add crates/bot/src/signal/mod.rs crates/bot/tests/test_signal_engine.rs
git commit -m "feat(bot): add SignalEngine decision logic (LLM/rule-only paths)"
```

---

## 검증 체크리스트

Plan 2 완료 후 아래가 모두 true여야 한다:

- [ ] `cargo test -p kis_bot` — 전체 통과 (Plan 1 + 2 합산)
- [ ] `cargo clippy -p kis_bot -- -D warnings` — 에러 없음
- [ ] `classify_regime()` — 5개 경계값 테스트 통과
- [ ] `calculate_setup_score()` — 최고점 100, 음수 가능, 각 조건 독립 검증 완료
- [ ] `RuleEngine::evaluate()` — strength 계산 3개 케이스 통과
- [ ] `SignalDecision` — LLM/rule-only 양 경로, BLOCK 우선순위 검증 완료
