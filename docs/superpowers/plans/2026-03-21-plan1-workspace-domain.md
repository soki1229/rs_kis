# Plan 1: Workspace Setup + Domain Crate

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `rs_kis` 단일 크레이트를 Cargo workspace로 전환하고, 모든 크레이트가 공유하는 `domain` 크레이트를 구현한다.

**Architecture:** 현재 루트의 `Cargo.toml`을 workspace manifest로 전환하고 기존 소스를 `crates/kis_api/`로 이동한다. `domain` 크레이트는 `BotCommand`, `BotEvent`, `Position` 등 공유 타입만 포함하며 비즈니스 로직이 없다. 모든 금액/가격은 `rust_decimal::Decimal` 사용.

**Tech Stack:** Rust 2021 edition, Cargo workspace, `rust_decimal`, `serde`, `chrono`

---

## File Map

```
/ (rs_kis repo root)
├── Cargo.toml                             MODIFY → workspace manifest
├── crates/
│   ├── kis_api/
│   │   ├── Cargo.toml                     CREATE → 기존 root Cargo.toml 내용 이전
│   │   └── src/                           CREATE → 기존 src/ 이동
│   ├── domain/
│   │   ├── Cargo.toml                     CREATE
│   │   └── src/
│   │       ├── lib.rs                     CREATE
│   │       ├── command.rs                 CREATE → BotCommand
│   │       ├── event.rs                   CREATE → BotEvent
│   │       └── types.rs                   CREATE → Position, OrderRequest, PnLReport, Side, AlertLevel
│   ├── bot/
│   │   ├── Cargo.toml                     CREATE (skeleton)
│   │   └── src/lib.rs                     CREATE (skeleton)
│   └── server/
│       ├── Cargo.toml                     CREATE (skeleton)
│       └── src/main.rs                    CREATE (skeleton)
└── docs/superpowers/
    ├── specs/2026-03-21-trading-system-design.md
    └── plans/2026-03-21-plan1-workspace-domain.md
```

---

## Task 1: Workspace로 전환

현재 루트 `Cargo.toml`을 workspace manifest로 교체하고, 기존 크레이트를 `crates/kis_api/`로 이동한다.

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/kis_api/Cargo.toml`
- Create: `crates/kis_api/src/` (기존 `src/` 이동)

- [ ] **Step 1: `crates/` 디렉터리 생성 후 기존 src를 kis_api로 이동**

```bash
mkdir -p crates/kis_api
cp -r src/ crates/kis_api/src    # 후행 슬래시 필수 — 없으면 재실행 시 src/src/ 중첩 발생
cp Cargo.toml crates/kis_api/Cargo.toml
```

- [ ] **Step 2: `crates/kis_api/Cargo.toml` 수정 — package name 확인 + `[features]` 제거**

`[package]` name을 `kis_api`로 변경하고, 기존 `[features] vts_mock_disabled = []` 블록을 삭제한다.
이 플래그는 Plan 2에서 `KisConfig::mock(bool)`으로 대체되므로 더 이상 필요 없다.

```toml
[package]
name = "kis_api"
version = "0.1.0"
edition = "2021"

# [features] 블록 삭제 — vts_mock_disabled는 KisConfig::mock(bool)으로 대체됨
```

- [ ] **Step 3: 루트 `Cargo.toml`을 workspace manifest로 교체**

```toml
[workspace]
members = [
    "crates/kis_api",
    "crates/domain",
    "crates/bot",
    "crates/server",
]
resolver = "2"
```

- [ ] **Step 4: `cargo check -p kis_api` 실행하여 기존 코드가 여전히 컴파일되는지 확인**

```bash
cargo check -p kis_api
```

Expected: 컴파일 성공 또는 기존과 동일한 에러만 발생 (새 에러 없음)

- [ ] **Step 5: 기존 루트 `src/` 제거**

```bash
rm -rf src
```

- [ ] **Step 6: `cargo check -p kis_api` 재실행**

```bash
cargo check -p kis_api
```

Expected: 성공

- [ ] **Step 7: 커밋**

```bash
git add -A
git commit -m "chore: convert rs_kis to Cargo workspace, move source to crates/kis_api"
```

---

## Task 2: `bot`, `server` 스켈레톤 크레이트 생성

이후 플랜에서 채워질 빈 크레이트를 미리 만들어 workspace가 완성된 상태로 유지.

**Files:**
- Create: `crates/bot/Cargo.toml`
- Create: `crates/bot/src/lib.rs`
- Create: `crates/server/Cargo.toml`
- Create: `crates/server/src/main.rs`

- [ ] **Step 1: `crates/bot/Cargo.toml` 작성**

```toml
[package]
name = "bot"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- [ ] **Step 2: `crates/bot/src/lib.rs` 작성**

```rust
// Trading algorithm crate — implementation in Plan 3
```

- [ ] **Step 3: `crates/server/Cargo.toml` 작성**

```toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "server"
path = "src/main.rs"

[dependencies]
```

- [ ] **Step 4: `crates/server/src/main.rs` 작성**

```rust
// Integration hub — implementation in Plan 4
fn main() {}
```

- [ ] **Step 5: `cargo check` (workspace 전체)**

```bash
cargo check
```

Expected: 모든 크레이트 성공

- [ ] **Step 6: 커밋**

```bash
git add crates/bot crates/server
git commit -m "chore: add bot and server skeleton crates to workspace"
```

---

## Task 3: `domain` 크레이트 — 기반 설정

**Files:**
- Create: `crates/domain/Cargo.toml`
- Create: `crates/domain/src/lib.rs`

- [ ] **Step 1: `crates/domain/Cargo.toml` 작성**

```toml
[package]
name = "domain"
version = "0.1.0"
edition = "2021"

[dependencies]
rust_decimal = { version = "1", features = ["serde-with-str"] }
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
# async-trait은 domain 크레이트에 불필요 — 트레이트/비즈니스 로직 없음
```

- [ ] **Step 2: `crates/domain/src/lib.rs` 작성**

```rust
mod command;
mod event;
mod types;

pub use command::BotCommand;
pub use event::BotEvent;
pub use types::{AlertLevel, FillInfo, OrderRequest, OrderResult, PnLReport, Position, Side};
```

- [ ] **Step 3: `cargo check -p domain`**

```bash
cargo check -p domain
```

Expected: 컴파일 에러 발생 ("file not found for module" 류) — `lib.rs`가 참조하는 모듈 파일이 아직 없으므로 의도된 실패

---

## Task 4: `domain` — 가격/금액 타입 (TDD)

`f64` 대신 `Decimal`을 써야 하는 이유를 테스트로 증명하고, 올바른 타입을 정의한다.

**Files:**
- Create: `crates/domain/src/types.rs`

- [ ] **Step 1: 실패하는 테스트 작성 — f64 vs Decimal 정밀도**

`crates/domain/src/types.rs` 파일 생성:

```rust
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub qty: i64,
    pub avg_price: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: Side,
    pub qty: u64,
    pub price: Option<Decimal>, // None = 시장가
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderResult {
    pub order_id: String,
    pub symbol: String,
    pub side: Side,
    pub qty: u64,
    pub price: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FillInfo {
    pub order_id: String,
    pub symbol: String,
    pub filled_qty: u64,
    pub filled_price: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PnLReport {
    pub realized: Decimal,
    pub unrealized: Decimal,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

// 미정의 타입 (구현 시 추가 예정)
// pub struct BotStatus { ... }

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn decimal_avoids_float_rounding_error() {
        // f64 누적 오류 재현
        let f: f64 = 0.1 + 0.2;
        assert_ne!(f, 0.3); // f64는 0.3이 아님!

        // Decimal은 정확
        let d = dec!(0.1) + dec!(0.2);
        assert_eq!(d, dec!(0.3));
    }

    #[test]
    fn position_pnl_calculation_is_exact() {
        // 매수 100주 @ $134.20, 현재가 $135.50
        let avg = dec!(134.20);
        let current = dec!(135.50);
        let qty = Decimal::from(100u64);
        let unrealized = (current - avg) * qty;
        assert_eq!(unrealized, dec!(130.00));
    }

    #[test]
    fn order_request_market_order_has_no_price() {
        let req = OrderRequest {
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: None,
        };
        assert!(req.price.is_none());
    }

    #[test]
    fn order_request_limit_order_has_price() {
        let req = OrderRequest {
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: Some(dec!(130.00)),
        };
        assert_eq!(req.price, Some(dec!(130.00)));
    }

    #[test]
    fn types_are_clone_and_debug() {
        let pos = Position {
            symbol: "AAPL".into(),
            qty: 50,
            avg_price: dec!(175.30),
        };
        let cloned = pos.clone();
        assert_eq!(pos, cloned);
        let _ = format!("{:?}", cloned);
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p domain types::tests
```

Expected: FAIL (타입 미정의로 컴파일 에러)

- [ ] **Step 3: `Cargo.toml`에 `rust_decimal_macros` 추가**

`crates/domain/Cargo.toml`의 `[dependencies]`에 추가:
```toml
rust_decimal_macros = "1"
```

- [ ] **Step 4: 테스트 재실행 — 통과 확인**

```bash
cargo test -p domain types::tests
```

Expected: 5개 PASS

- [ ] **Step 5: 커밋**

```bash
git add crates/domain
git commit -m "feat(domain): add money/order types with Decimal precision"
```

---

## Task 5: `domain` — BotCommand 정의 (TDD)

**Files:**
- Create: `crates/domain/src/command.rs`

- [ ] **Step 1: 실패하는 테스트 작성**

`crates/domain/src/command.rs`:

```rust
use rust_decimal::Decimal;
use crate::types::OrderRequest;

#[derive(Debug, Clone)]
pub enum BotCommand {
    Start,
    Stop,
    Pause,
    ForceOrder(OrderRequest),
    SetRiskLimit(Decimal),
    QueryStatus,
    // SetStrategy — StrategyParams 정의 후 추가
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use crate::types::{OrderRequest, Side};

    #[test]
    fn bot_command_is_clone() {
        let cmd = BotCommand::SetRiskLimit(dec!(0.05));
        let cloned = cmd.clone();
        // Clone 성공 — 컴파일되면 통과
        let _ = cloned;
    }

    #[test]
    fn bot_command_force_order_contains_request() {
        let req = OrderRequest {
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: Some(dec!(130.00)),
        };
        let cmd = BotCommand::ForceOrder(req.clone());
        match cmd {
            BotCommand::ForceOrder(r) => assert_eq!(r.symbol, "NVDA"),
            _ => panic!("wrong variant"),
        }
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn bot_command_is_send_sync() {
        assert_send_sync::<BotCommand>();
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p domain command::tests
```

Expected: FAIL (모듈 없음)

- [ ] **Step 3: `lib.rs`에 `command` 모듈 등록 확인 후 테스트 재실행**

```bash
cargo test -p domain command::tests
```

Expected: 3개 PASS

- [ ] **Step 4: 커밋**

```bash
git add crates/domain/src/command.rs crates/domain/src/lib.rs
git commit -m "feat(domain): add BotCommand enum"
```

---

## Task 6: `domain` — BotEvent 정의 (TDD)

**Files:**
- Create: `crates/domain/src/event.rs`

- [ ] **Step 1: 실패하는 테스트 작성**

`crates/domain/src/event.rs`:

```rust
use crate::types::{AlertLevel, FillInfo, OrderResult, PnLReport, Position};

#[derive(Debug, Clone)]
pub enum BotEvent {
    OrderPlaced(OrderResult),
    OrderFilled(FillInfo),
    PositionChanged(Position),
    DailyPnL(PnLReport),
    Alert { level: AlertLevel, msg: String },
    // StatusReport(BotStatus) — BotStatus 정의 후 추가
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use crate::types::{AlertLevel, FillInfo, OrderResult, PnLReport, Position, Side};
    use chrono::NaiveDate;

    fn sample_order_result() -> OrderResult {
        OrderResult {
            order_id: "ORD-001".into(),
            symbol: "NVDA".into(),
            side: Side::Buy,
            qty: 10,
            price: dec!(130.00),
        }
    }

    #[test]
    fn bot_event_is_clone() {
        let ev = BotEvent::OrderPlaced(sample_order_result());
        let _ = ev.clone();
    }

    #[test]
    fn alert_event_carries_level_and_message() {
        let ev = BotEvent::Alert {
            level: AlertLevel::Critical,
            msg: "connection lost".into(),
        };
        match ev {
            BotEvent::Alert { level, msg } => {
                assert_eq!(level, AlertLevel::Critical);
                assert_eq!(msg, "connection lost");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn daily_pnl_event_contains_report() {
        let report = PnLReport {
            realized: dec!(320.50),
            unrealized: dec!(45.00),
            date: NaiveDate::from_ymd_opt(2026, 3, 21).unwrap(),
        };
        let ev = BotEvent::DailyPnL(report);
        match ev {
            BotEvent::DailyPnL(r) => assert_eq!(r.realized, dec!(320.50)),
            _ => panic!("wrong variant"),
        }
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn bot_event_is_send_sync() {
        assert_send_sync::<BotEvent>();
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p domain event::tests
```

Expected: FAIL

- [ ] **Step 3: `lib.rs` pub use 업데이트 확인 후 테스트 재실행**

```bash
cargo test -p domain event::tests
```

Expected: 4개 PASS

- [ ] **Step 4: domain 전체 테스트 실행**

```bash
cargo test -p domain
```

Expected: 전체 PASS

- [ ] **Step 5: 커밋**

```bash
git add crates/domain/src/event.rs crates/domain/src/lib.rs
git commit -m "feat(domain): add BotEvent enum"
```

---

## Task 7: `kis_api`에서 `domain` 의존 추가 및 workspace 검증

**Files:**
- Modify: `crates/kis_api/Cargo.toml`

- [ ] **Step 1: `kis_api`의 `Cargo.toml`에 domain 의존 추가**

```toml
[dependencies]
domain = { path = "../domain" }
# ... 기존 의존성 유지
```

- [ ] **Step 2: workspace 전체 빌드 검증**

```bash
cargo build
```

Expected: 성공

- [ ] **Step 3: workspace 전체 테스트 실행**

```bash
cargo test
```

Expected: domain 테스트 전체 PASS, kis_api 기존 테스트 유지

- [ ] **Step 4: 최종 커밋**

```bash
git add crates/kis_api/Cargo.toml
git commit -m "chore(kis_api): add domain crate dependency"
```

---

## 완료 기준

- [ ] `cargo check` workspace 전체 성공
- [ ] `cargo test -p domain` 전체 PASS
- [ ] `domain` 크레이트에 `unsafe`, `f64` 금액 계산, 전역 상태 없음
- [ ] `kis_api` 기존 테스트 깨지지 않음
