# KisApi Trait Extension Design

**Date:** 2026-03-23
**Repo:** rs_kis (`crates/kis_api`)
**Status:** Design approved, pending implementation

---

## 1. Overview

`KisApi` trait에 현재 `KisClient` inherent method로만 존재하는 4개 메서드를 추가 노출한다.
이 변경은 `rs_kis_pilot` server crate의 신규 컴포넌트(RegimeTask, ExecutionTask, PositionTask)가
`Arc<dyn KisApi>`를 통해 해당 기능을 사용하기 위한 선결 조건이다.

---

## 2. 설계 원칙

`KisApi` trait은 의존성 역전(DI) + 테스트 모킹용 최소 인터페이스다.
`KisClient` inherent method가 생겨도 trait에 자동 노출하지 않는 이유:

- **Mocking 비용**: trait 메서드가 많을수록 `MockKisApi` 구현체가 커짐
- **Interface Segregation**: 소비자가 생기는 시점에만 추가
- **실제 소비자 원칙**: inherent method는 "KIS API가 지원한다"는 선언, trait 노출은 "server 컴포넌트가 실제로 쓴다"는 시점에 추가

이번 추가는 RegimeTask(`daily_chart`), ExecutionTask(`place_order`, `cancel_order`, `unfilled_orders`)의 첫 소비자가 생기는 시점이다.

---

## 3. 추가 메서드

```rust
#[async_trait]
pub trait KisApi: Send + Sync {
    // 기존
    async fn stream(&self) -> Result<KisStream, KisError>;
    async fn volume_ranking(&self, exchange: &Exchange, count: u32) -> Result<Vec<RankingItem>, KisError>;
    async fn holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError>;

    // 신규
    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError>;
    async fn cancel_order(&self, req: CancelOrderRequest) -> Result<CancelOrderResponse, KisError>;
    async fn daily_chart(&self, req: DailyChartRequest) -> Result<Vec<CandleBar>, KisError>;
    /// KIS 계좌는 KisConfig에 단일 계좌번호로 설정된다.
    /// KR/US 파이프라인이 각각 별도 KisConfig(별도 계좌)로 KisClient를 생성하므로
    /// 파라미터 없이 설정된 계좌의 미체결 주문 전체를 조회한다.
    async fn unfilled_orders(&self) -> Result<Vec<UnfilledOrder>, KisError>;
    async fn news(&self, symbol: &str) -> Result<Vec<NewsItem>, KisError>;
}
```

**소비자별 용도:**

| 메서드 | 소비자 | 용도 |
|--------|--------|------|
| `place_order` | ExecutionTask | 모의투자 주문 제출 |
| `cancel_order` | ExecutionTask | graceful shutdown 시 미체결 주문 취소 |
| `daily_chart` | RegimeTask, SignalTask | QQQ 레짐 계산, 종목 MA/ATR 계산 |
| `unfilled_orders` | ExecutionTask | 체결 확인 polling, shutdown 전 미체결 목록 조회 |
| `news` | SignalTask | `has_news_catalyst` 판단 (뉴스 유무 확인) |

---

## 4. 구현 패턴

`KisClient`의 `impl KisApi`에서 동명 inherent method로 위임 (무한 재귀 없음 — Rust는 inherent method가 trait method보다 우선):

```rust
async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError> {
    self.place_order(req).await
}
```

---

## 5. 파일 변경

- `crates/kis_api/src/traits.rs` — 메서드 5개 추가 (`place_order`, `cancel_order`, `daily_chart`, `unfilled_orders`, `news`)
- `crates/kis_api/src/client.rs` — `impl KisApi` 블록에 위임 5개 추가

---

## 6. 테스트 전략

| 대상 | 방식 |
|------|------|
| `kis_api_trait_is_object_safe` | 기존 테스트 — `Box<dyn KisApi>` 컴파일 확인 |
| `place_order` / `cancel_order` | `crates/kis_api/tests/integration/order.rs` 통합 테스트 (`KIS_INTEGRATION_TEST=1`) |
| `daily_chart` | 기존 fixture 단위 테스트 확장 |
| `unfilled_orders` | fixture 단위 테스트 |
| `news` | fixture 단위 테스트 (응답 파싱 확인) |

---

## 7. Non-Goals

- `KisClient` inherent method 전체를 trait에 노출하는 것 (소비자 없는 메서드는 추가하지 않음)
- 국내주식 관련 메서드 추가 (KR 파이프라인 구현 시 별도 spec)
