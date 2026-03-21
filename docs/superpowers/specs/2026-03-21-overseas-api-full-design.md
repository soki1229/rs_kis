# 해외주식 전체 API 설계 스펙

## 목표

`kis_api` 라이브러리가 KIS OpenAPI의 **해외주식 전체 엔드포인트**를 커버하도록 구현한다.
장기적으로 CLI 기반 HTS(Home Trading System)의 기반 라이브러리로 사용된다.

## 범위

| 플랜 | 내용 | 엔드포인트 수 |
|------|------|-------------|
| Plan 2 (기존) | 아키텍처 재설계 (KisConfig, KisError, TokenManager, KisStream, 트레이트) + 샘플 2개 | 2 |
| Plan 3a | 해외주식 주문/계좌 REST API 전체 | ~18 |
| Plan 3b | 해외주식 시세 REST API 전체 | ~17 |
| Plan 3c | 해외주식 시세분석 REST API 전체 | ~11 |

Plan 2는 별도로 작성된 플랜(`2026-03-21-plan2-kis-api.md`)대로 실행한다.
이 스펙은 Plan 3a/3b/3c의 설계 기준이다.

---

## 아키텍처 개요

Plan 2가 확립하는 기반 위에 API 엔드포인트를 추가한다.

```
KisClient (Arc<Inner>)
  ├── TokenManager          — 인증 토큰 관리 (Plan 2)
  ├── rest::Http            — HTTP executor (Plan 2)
  └── 공개 메서드 (Plan 3a/3b/3c)
       ├── 주문/계좌 (order, cancel, balance, ...)
       ├── 시세 (price, orderbook, chart, search, ...)
       └── 시세분석 (ranking, volume, market_cap, ...)

KisStream (Plan 2 + 3b WebSocket)
  └── subscribe_price / subscribe_orderbook / receiver()
```

---

## 모듈 구조

```
crates/kis_api/src/rest/overseas/
├── mod.rs
├── order/
│   ├── mod.rs
│   ├── place.rs        — 매수/매도 주문 (TTTT1002U/TTTT1006U)
│   ├── cancel.rs       — 정정/취소 (TTTT1004U)
│   ├── reserved.rs     — 예약주문/취소/조회
│   └── daytime.rs      — 미국 주간거래 주문/정정취소
├── inquiry/
│   ├── mod.rs
│   ├── balance.rs      — 잔고 조회 (TTTS3012R)
│   ├── orders.rs       — 미체결(TTTS3018R) / 체결내역(TTTS3035R)
│   ├── profit.rs       — 기간손익 (TTTS3039R)
│   └── amount.rs       — 매수가능금액 / 외화증거금
├── quote/
│   ├── mod.rs
│   ├── price.rs        — 현재가 상세 (HHDFS76200200)
│   ├── orderbook.rs    — 호가 (HHDFS76200100)
│   ├── chart.rs        — 일봉/분봉 차트
│   ├── search.rs       — 종목 검색/정보
│   └── news.rs         — 뉴스/속보/배당
└── analysis/
    ├── mod.rs
    ├── ranking.rs      — 등락률/거래량/체결강도 순위
    └── market.rs       — 시가총액/업종/신고신저가
```

---

## 타입 설계 원칙

### 강타입 Request/Response

모든 엔드포인트는 명시적인 Request 구조체와 Response 구조체를 갖는다.
`serde_json::Value` 반환 금지.

```rust
// Request: 필수 파라미터는 필드, 선택 파라미터는 Option<>
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub qty: u64,
    pub price: Option<Decimal>,  // None = 시장가
}

// Response: KIS API JSON 필드를 Rust 타입으로 변환
pub struct PlaceOrderResponse {
    pub order_id: String,
    pub order_time: DateTime<FixedOffset>,
}
```

### 공통 타입

```rust
pub enum Exchange {
    NASD,   // 나스닥
    NYSE,   // 뉴욕
    AMEX,   // 아멕스
    TKSE,   // 도쿄
    SEHK,   // 홍콩
    SHAA,   // 상해
    SZAA,   // 심천
    HASE,   // 하노이
    VNSE,   // 호치민
}

pub enum OrderSide { Buy, Sell }

pub enum OrderType {
    Market,         // 시장가
    Limit,          // 지정가
    LimitAon,       // 조건부지정가
}
```

### 금액 규칙

- 모든 가격/금액 필드: `rust_decimal::Decimal` (f64 금지)
- KIS API 문자열 숫자(`"134.20"`) → `Decimal` 역직렬화: `#[serde(with = "rust_decimal::serde::str")]`

### 날짜/시간 규칙

- 모든 시각: `DateTime<FixedOffset>` (KST = UTC+9)
- KIS API 문자열 날짜(`"20260321"`, `"143022"`) → `NaiveDate`/`NaiveTime` 파싱 후 KST `FixedOffset` 적용

---

## KisClient 공개 API

`KisClient`가 모든 엔드포인트를 메서드로 노출한다.

```rust
// 클라이언트 생성
let client = KisClient::builder()
    .app_key("...")
    .app_secret("...")
    .account("XXXXXXXX-XX")
    .mock(true)               // true = VTS 모의투자
    .token_cache("~/.config/kis_api/token.json")
    .build()
    .await?;

// ── 주문/계좌 ──────────────────────────────────────────────
client.place_order(req).await?          // 주문
client.cancel_order(req).await?         // 정정/취소
client.place_reserved_order(req).await? // 예약주문
client.cancel_reserved_order(id).await? // 예약취소
client.reserved_orders().await?         // 예약주문 목록
client.place_daytime_order(req).await?  // 미국 주간거래
client.cancel_daytime_order(req).await? // 미국 주간거래 취소

client.balance().await?                 // 잔고
client.unfilled_orders().await?         // 미체결
client.order_history(from, to).await?   // 체결내역
client.period_profit(from, to).await?   // 기간손익
client.buyable_amount(req).await?       // 매수가능금액
client.foreign_margin(currency).await?  // 외화증거금

// ── 시세 ──────────────────────────────────────────────────
client.price(symbol, exchange).await?         // 현재가 상세
client.orderbook(symbol, exchange).await?     // 호가
client.daily_chart(req).await?                // 일봉
client.minute_chart(req).await?               // 분봉
client.search(keyword).await?                 // 종목 검색
client.symbol_info(symbol, exchange).await?   // 종목 정보
client.news(symbol).await?                    // 뉴스
client.breaking_news().await?                 // 속보
client.dividend(symbol, exchange).await?      // 배당/권리
client.holidays(country).await?               // 국가별 휴장일
client.index_chart(req).await?                // 지수 분봉

// ── 시세분석 ──────────────────────────────────────────────
client.price_ranking(req).await?       // 등락률 순위
client.volume_ranking(req).await?      // 거래량 순위
client.volume_surge(exchange).await?   // 거래량 급증
client.volume_power(symbol).await?     // 체결강도
client.new_highlow(exchange).await?    // 신고/신저가
client.market_cap(exchange).await?     // 시가총액 순위
client.trade_turnover(exchange).await? // 거래회전율

// ── WebSocket 실시간 ───────────────────────────────────────
let stream = KisStream::connect(&client).await?;
stream.subscribe_price("AAPL").await?;        // 실시간 체결가
stream.subscribe_orderbook("AAPL").await?;    // 실시간 호가
stream.unsubscribe_price("AAPL").await?;
let mut rx = stream.receiver();               // EventReceiver
while let Ok(event) = rx.recv().await {
    match event {
        KisEvent::Transaction(data) => { .. }
        KisEvent::Quote(data) => { .. }
        KisEvent::OrderConfirm(data) => { .. }
    }
}
```

---

## 환경 변수 / 설정

`.env.example` 기준:

| 변수 | 설명 | 기본값 |
|------|------|--------|
| `APP_KEY` | 실전투자 앱키 | — |
| `APP_SECRET` | 실전투자 시크릿 | — |
| `ACCOUNT_NUM` | 실전투자 계좌번호 | — |
| `REST_URL` | 실전 REST URL | `https://openapi.koreainvestment.com:9443` |
| `WS_URL` | 실전 WebSocket URL | `wss://ops.koreainvestment.com:21000` |
| `VTS_APP_KEY` | 모의투자 앱키 | — |
| `VTS_APP_SECRET` | 모의투자 시크릿 | — |
| `VTS_ACCOUNT_NUM` | 모의투자 계좌번호 | — |
| `VTS_REST_URL` | 모의투자 REST URL | `https://openapivts.koreainvestment.com:29443` |
| `VTS_WS_URL` | 모의투자 WebSocket URL | `wss://ops.koreainvestment.com:31000` |
| `TOKEN_CACHE_PATH` | 토큰 캐시 경로 | `~/.config/kis_api/token.json` |
| `KIS_INTEGRATION_TEST` | 통합 테스트 활성화 | `0` |

`KisConfig::builder().mock(true)` 시 VTS 변수 자동 선택.

---

## 검증 전략

### 단위 테스트 (항상 실행)

- Request 구조체 직렬화 검증 (JSON 필드명 올바른지)
- Response 구조체 역직렬화 검증 (fixture JSON → 타입 변환)
- Exchange/OrderSide/OrderType 변환 검증

### 통합 테스트 (`KIS_INTEGRATION_TEST=1`)

- `KisConfig::from_env_vts()` — `.env`의 VTS 자격증명 로드
- 실제 VTS 서버 호출
- 응답 구조 검증 (필드 존재, 타입 일치, 에러 없음)
- `#[cfg(feature = "integration")]` + `skip_unless_integration!()` 매크로로 조건부 실행

```rust
// 통합 테스트 예시
#[tokio::test]
#[cfg(feature = "integration")]
async fn test_balance_returns_valid_response() {
    skip_unless_integration!();
    let client = KisClient::from_env_vts().await.unwrap();
    let result = client.balance().await;
    assert!(result.is_ok());
}
```

---

## 에러 처리

Plan 2의 `KisError`를 그대로 사용:

```rust
KisError::Auth(String)        // 인증 실패
KisError::Network(reqwest::Error) // 네트워크 오류 (재시도 가능)
KisError::Api { code, message }   // KIS API 오류 응답
KisError::Parse(serde_json::Error) // 응답 파싱 실패
KisError::Lagged(u64)         // WebSocket 이벤트 유실
KisError::StreamClosed        // WebSocket 연결 종료
```

`KisError::is_retryable()` → Network, WebSocket 계열만 true.

---

## 구현 우선순위

```
1순위 (Plan 3a) — 거래 핵심
  place_order, cancel_order, balance, unfilled_orders,
  order_history, period_profit, buyable_amount

2순위 (Plan 3b) — 시세
  price, orderbook, daily_chart, minute_chart, search,
  symbol_info, news, WebSocket (price + orderbook + order_confirm)

3순위 (Plan 3c) — 시세분석
  price_ranking, volume_ranking, volume_surge,
  new_highlow, market_cap

후순위 — 고급 기능
  reserved_order, daytime_order, algo_order (TWAP/VWAP),
  foreign_margin, breaking_news, dividend, holidays
```

---

## 선행 조건

- Plan 1 완료: Cargo workspace + domain 크레이트
- Plan 2 완료: KisConfig, KisError, TokenManager, KisStream, KisApi trait, HTTP executor
