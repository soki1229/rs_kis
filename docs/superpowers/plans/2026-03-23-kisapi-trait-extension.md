# KisApi Trait Extension Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add 5 new methods (`place_order`, `cancel_order`, `daily_chart`, `unfilled_orders`, `news`) to the `KisApi` trait so server components can use them via `Arc<dyn KisApi>`.

**Architecture:** Both files change: `traits.rs` gets the new method signatures (with updated imports), `client.rs` gets 5 delegating impls in the existing `impl KisApi for KisClient` block. Pattern is identical to the existing 3 methods — each trait method just calls `self.<method>(req).await`. Rust's method resolution ensures the inherent method is called, not the trait method (no infinite recursion).

**Tech Stack:** Rust, `async_trait 0.1`, existing `KisClient` inherent methods (already implemented).

**Spec:** `docs/superpowers/specs/2026-03-23-kisapi-trait-extension-design.md`

> **⚠️ Downstream impact:** After this plan, `rs_kis_pilot` will fail to compile because its mock `impl KisApi` structs (in `tick.rs` and `scheduler.rs`) don't implement the new methods. The first task of the US Signal+Regime plan fixes this.

---

## File Structure

```
crates/kis_api/src/
  traits.rs     — ADD 5 method signatures + import 8 new types
  client.rs     — ADD 5 delegating impls in `impl KisApi for KisClient`
```

---

### Task 1: Add 5 methods to `KisApi` trait

**Files:**
- Modify: `crates/kis_api/src/traits.rs`

- [ ] **Step 1: Write the failing test**

In `crates/kis_api/src/traits.rs`, add inside `mod tests`:

```rust
#[test]
fn kis_api_all_nine_methods_are_object_safe() {
    // Box<dyn KisApi> still compiles with 9 methods
    struct MockNine;
    #[async_trait::async_trait]
    impl KisApi for MockNine {
        async fn stream(&self) -> Result<crate::KisStream, crate::KisError> { unimplemented!() }
        async fn volume_ranking(&self, _: &crate::Exchange, _: u32) -> Result<Vec<crate::RankingItem>, crate::KisError> { Ok(vec![]) }
        async fn holidays(&self, _: &str) -> Result<Vec<crate::Holiday>, crate::KisError> { Ok(vec![]) }
        async fn place_order(&self, _: crate::PlaceOrderRequest) -> Result<crate::PlaceOrderResponse, crate::KisError> { unimplemented!() }
        async fn cancel_order(&self, _: crate::CancelOrderRequest) -> Result<crate::CancelOrderResponse, crate::KisError> { unimplemented!() }
        async fn daily_chart(&self, _: crate::DailyChartRequest) -> Result<Vec<crate::CandleBar>, crate::KisError> { Ok(vec![]) }
        async fn unfilled_orders(&self) -> Result<Vec<crate::UnfilledOrder>, crate::KisError> { Ok(vec![]) }
        async fn news(&self, _: &str) -> Result<Vec<crate::NewsItem>, crate::KisError> { Ok(vec![]) }
    }
    let _: Option<Box<dyn KisApi>> = None;
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test -p kis_api -- traits::tests::kis_api_all_nine_methods_are_object_safe
```

Expected: FAIL — `place_order`, `cancel_order`, etc. not found on trait.

- [ ] **Step 3: Add 5 methods to the trait**

Replace `crates/kis_api/src/traits.rs` content entirely:

```rust
use crate::{
    CancelOrderRequest, CancelOrderResponse, CandleBar, DailyChartRequest,
    Exchange, Holiday, KisError, KisStream, NewsItem, PlaceOrderRequest,
    PlaceOrderResponse, RankingItem, UnfilledOrder,
};
use async_trait::async_trait;

/// REST + WebSocket 클라이언트 트레이트 (의존성 역전 / 테스트 모킹용)
#[async_trait]
pub trait KisApi: Send + Sync {
    /// WebSocket 스트림 생성
    async fn stream(&self) -> Result<KisStream, KisError>;

    /// 해외주식 거래량 순위 조회
    async fn volume_ranking(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<RankingItem>, KisError>;

    /// 해외 공휴일 조회 (country: "USA", "JPN" 등)
    async fn holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError>;

    /// 해외주식 매수/매도 주문 제출
    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError>;

    /// 해외주식 주문 정정/취소
    async fn cancel_order(&self, req: CancelOrderRequest) -> Result<CancelOrderResponse, KisError>;

    /// 해외주식 일/주/월봉 조회 (최대 요청 건수는 API 제한 따름)
    async fn daily_chart(&self, req: DailyChartRequest) -> Result<Vec<CandleBar>, KisError>;

    /// 설정된 계좌의 미체결 주문 전체 조회.
    /// KR/US 파이프라인이 각각 별도 KisConfig(별도 계좌)로 KisClient를 생성하므로
    /// 파라미터 없이 해당 계좌 전체 미체결을 반환한다.
    async fn unfilled_orders(&self) -> Result<Vec<UnfilledOrder>, KisError>;

    /// 해외주식 뉴스 조회 (`has_news_catalyst` 판단용)
    async fn news(&self, symbol: &str) -> Result<Vec<NewsItem>, KisError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kis_api_extended_is_object_safe() {
        // Box<dyn KisApi>가 컴파일되는지 확인 (volume_ranking + holidays 추가 후)
        let _: Option<Box<dyn KisApi>> = None;
    }

    #[test]
    fn kis_api_all_nine_methods_are_object_safe() {
        // Box<dyn KisApi> still compiles with 9 methods
        struct MockNine;
        #[async_trait::async_trait]
        impl KisApi for MockNine {
            async fn stream(&self) -> Result<crate::KisStream, crate::KisError> { unimplemented!() }
            async fn volume_ranking(&self, _: &crate::Exchange, _: u32) -> Result<Vec<crate::RankingItem>, crate::KisError> { Ok(vec![]) }
            async fn holidays(&self, _: &str) -> Result<Vec<crate::Holiday>, crate::KisError> { Ok(vec![]) }
            async fn place_order(&self, _: crate::PlaceOrderRequest) -> Result<crate::PlaceOrderResponse, crate::KisError> { unimplemented!() }
            async fn cancel_order(&self, _: crate::CancelOrderRequest) -> Result<crate::CancelOrderResponse, crate::KisError> { unimplemented!() }
            async fn daily_chart(&self, _: crate::DailyChartRequest) -> Result<Vec<crate::CandleBar>, crate::KisError> { Ok(vec![]) }
            async fn unfilled_orders(&self) -> Result<Vec<crate::UnfilledOrder>, crate::KisError> { Ok(vec![]) }
            async fn news(&self, _: &str) -> Result<Vec<crate::NewsItem>, crate::KisError> { Ok(vec![]) }
        }
        let _: Option<Box<dyn KisApi>> = None;
    }
}
```

- [ ] **Step 4: Run test to verify it compiles and passes**

```bash
cargo test -p kis_api -- traits::tests
```

Expected: 2 tests pass.

---

### Task 2: Add 5 delegating impls in `client.rs`

**Files:**
- Modify: `crates/kis_api/src/client.rs`

- [ ] **Step 1: Verify the current impl KisApi block compiles with new trait**

```bash
cargo build -p kis_api
```

Expected: FAIL — `impl KisApi for KisClient` is missing 5 required methods.

- [ ] **Step 2: Add 5 delegating impls to `impl KisApi for KisClient`**

In `crates/kis_api/src/client.rs`, inside the existing `#[async_trait]\nimpl KisApi for KisClient` block (currently ends at line ~278), add after the `holidays` impl:

```rust
    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError> {
        self.place_order(req).await
    }

    async fn cancel_order(&self, req: CancelOrderRequest) -> Result<CancelOrderResponse, KisError> {
        self.cancel_order(req).await
    }

    async fn daily_chart(&self, req: DailyChartRequest) -> Result<Vec<CandleBar>, KisError> {
        self.daily_chart(req).await
    }

    async fn unfilled_orders(&self) -> Result<Vec<UnfilledOrder>, KisError> {
        self.unfilled_orders().await
    }

    async fn news(&self, symbol: &str) -> Result<Vec<NewsItem>, KisError> {
        self.news(symbol).await
    }
```

> Note: `use` imports at top of client.rs already pull in all needed types via `crate::` — check
> existing imports and add any missing ones. The pattern `self.place_order(req).await` calls
> the inherent method (Rust resolves inherent over trait — no infinite recursion).

- [ ] **Step 3: Build and test**

```bash
cargo build -p kis_api
cargo test -p kis_api
```

Expected: All existing tests pass (79 tests), build succeeds.

- [ ] **Step 4: Commit**

```bash
git add crates/kis_api/src/traits.rs crates/kis_api/src/client.rs
git commit -m "feat(kis_api): add place_order, cancel_order, daily_chart, unfilled_orders, news to KisApi trait"
```

---

### Task 3: Verify full test suite

- [ ] **Step 1: Run full test suite with clippy**

```bash
cargo clippy -- -D warnings
cargo test
```

Expected: 0 warnings, all tests pass.

- [ ] **Step 2: Push**

```bash
git push
```
