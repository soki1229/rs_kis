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
       ├── 주문/계좌 (place_order, cancel_order, balance, ...)
       ├── 시세 (price, orderbook, chart, search, ...)
       └── 시세분석 (price_ranking, volume_ranking, ...)

KisStream (Plan 2)
  — client.stream().await? 로 생성
  — subscribe(symbol, kind) 로 구독
  — receiver() 로 EventReceiver 획득
```

---

## 모듈 구조

### 디렉터리 트리 (파일 → 담당 메서드 명시)

```
crates/kis_api/src/rest/overseas/
├── mod.rs
│
├── order/
│   ├── mod.rs
│   ├── place.rs        — place_order(), place_daytime_order()
│   ├── cancel.rs       — cancel_order(), cancel_daytime_order()
│   └── reserved.rs     — place_reserved_order(), cancel_reserved_order(), reserved_orders()
│
├── inquiry/
│   ├── mod.rs
│   ├── balance.rs      — balance(), buyable_amount(), foreign_margin()
│   ├── orders.rs       — unfilled_orders(), order_history(from, to)
│   └── profit.rs       — period_profit(from, to)
│
├── quote/
│   ├── mod.rs
│   ├── price.rs        — price(), orderbook()
│   ├── chart.rs        — daily_chart(), minute_chart(), index_chart()
│   ├── search.rs       — search(), symbol_info()
│   └── corporate.rs    — news(), dividend(), holidays()
│
└── analysis/
    ├── mod.rs
    ├── ranking.rs      — price_ranking(), volume_ranking(), volume_surge(),
    │                     volume_power(), new_highlow()
    └── market.rs       — market_cap(), trade_turnover()
```

---

## 타입 설계 원칙

### 강타입 Request/Response

모든 엔드포인트는 명시적인 Request 구조체와 Response 구조체를 갖는다.
`serde_json::Value` 반환 금지. Request는 값으로 전달(by value).

```rust
// Request: 필수 파라미터는 필드, 선택 파라미터는 Option<>
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub qty: Decimal,            // Decimal — 분수 주식 대응
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
    Other(String),  // 미래 거래소 확장용 (XLON, XPAR 등)
}

pub enum OrderSide { Buy, Sell }

pub enum OrderType {
    Market,         // 시장가 (01)
    Limit,          // 지정가 (00)
    LimitAon,       // 조건부지정가 (32 — US LOC)
    MarketClose,    // 장마감시장가 (34 — US MOC)
    Other(String),  // 미래 KIS 추가 코드 대응
}

pub enum SubscriptionKind {
    Price,      // 실시간 체결가 (HDFSCNT0)
    Orderbook,  // 실시간 호가 (HDFSASP0/HDFSASP1)
}
```

### 금액/수량 규칙

- 모든 가격/금액/수량 필드: `rust_decimal::Decimal` (f64 금지)
- KIS API 문자열 숫자(`"134.20"`) → `Decimal` 역직렬화: `#[serde(with = "rust_decimal::serde::str")]`
- 주문 수량 `qty: Decimal` — 정수형 주식과 분수 주식 모두 대응

### 날짜/시간 규칙

- 모든 시각: `DateTime<FixedOffset>` (KST = UTC+9)
- KIS API 날짜 문자열(`"20260321"`, `"143022"`) → 파싱 후 KST FixedOffset 적용

---

## KisClient 공개 API

Plan 2의 설계 그대로: `KisConfig::builder()` → `KisClient::new(config)` (동기 생성자).

```rust
// 클라이언트 생성 (Plan 2 패턴 그대로)
let config = KisConfig::builder()
    .app_key("...")
    .app_secret("...")
    .account("XXXXXXXX-XX")
    .mock(true)               // true = VTS 모의투자
    .token_cache("~/.config/kis_api/token.json")
    .build()?;

let client = KisClient::new(config);

// ── 주문/계좌 ─────────────────────────────────────────────
client.place_order(req).await?              // 주문 (PlaceOrderRequest → by value)
client.cancel_order(req).await?             // 정정/취소
client.place_daytime_order(req).await?      // 미국 주간거래
client.cancel_daytime_order(req).await?     // 미국 주간거래 취소
client.place_reserved_order(req).await?     // 예약주문
client.cancel_reserved_order(id).await?     // 예약취소
client.reserved_orders().await?             // 예약주문 목록

client.balance().await?                     // 잔고
client.unfilled_orders().await?             // 미체결
client.order_history(from, to).await?       // 체결내역
client.period_profit(from, to).await?       // 기간손익
client.buyable_amount(req).await?           // 매수가능금액
client.foreign_margin(currency).await?      // 외화증거금

// ── 시세 ─────────────────────────────────────────────────
client.price(symbol, exchange).await?              // 현재가 상세
client.orderbook(symbol, exchange).await?          // 호가
client.daily_chart(req).await?                     // 일봉
client.minute_chart(req).await?                    // 분봉
client.index_chart(req).await?                     // 지수 분봉
client.search(keyword).await?                      // 종목 검색
client.symbol_info(symbol, exchange).await?        // 종목 정보
client.news(symbol).await?                         // 뉴스
client.dividend(symbol, exchange).await?           // 배당/권리
client.holidays(country).await?                    // 국가별 휴장일

// ── 시세분석 ─────────────────────────────────────────────
client.price_ranking(req).await?          // 등락률 순위
client.volume_ranking(req).await?         // 거래량 순위
client.volume_surge(exchange).await?      // 거래량 급증
client.volume_power(symbol).await?        // 체결강도
client.new_highlow(exchange).await?       // 신고/신저가
client.market_cap(exchange).await?        // 시가총액 순위
client.trade_turnover(exchange).await?    // 거래회전율

// ── WebSocket 실시간 ──────────────────────────────────────
// KisStream은 client.stream()으로 생성 (Plan 2 패턴)
let stream = client.stream().await?;

// 종목별 + 종류별 구독 (Plan 2의 단일 subscribe()를 확장)
stream.subscribe("AAPL", SubscriptionKind::Price).await?;
stream.subscribe("AAPL", SubscriptionKind::Orderbook).await?;
stream.unsubscribe("AAPL", SubscriptionKind::Price).await?;

let mut rx = stream.receiver();   // EventReceiver — recv()는 Result<KisEvent, KisError>
loop {
    match rx.recv().await {
        Ok(KisEvent::Transaction(data)) => { /* 실시간 체결 */ }
        Ok(KisEvent::Quote(data))       => { /* 실시간 호가 */ }
        Ok(KisEvent::OrderConfirm(data))=> { /* 주문체결통보 */ }
        Err(KisError::Lagged(n))        => { /* n개 이벤트 유실 — best-effort */ }
        Err(KisError::StreamClosed)     => break,
        Err(e)                          => { /* 기타 오류 처리 */ }
    }
}
```

**Plan 2와의 관계:**
- `KisStream` 생성: `client.stream().await?` (Plan 2 패턴 유지)
- `subscribe(symbol, kind)` 시그니처: Plan 2의 단일 `subscribe(symbol)` → `subscribe(symbol, kind: SubscriptionKind)` 로 확장. Plan 2 구현 시 이 시그니처를 선택.
- `unsubscribe(symbol, kind)` 시그니처: 동일하게 Plan 2의 `unsubscribe(symbol)` → `unsubscribe(symbol, kind: SubscriptionKind)` 로 확장.
- 이 두 변경은 Plan 2 플랜 문서에도 반영 필요.

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
- Exchange/OrderSide/OrderType enum 변환 검증
- Fixture 파일 위치: `crates/kis_api/tests/fixtures/overseas/{module}/{endpoint}.json`

### 통합 테스트 (런타임 env var 기반)

Cargo feature flag 없이 **런타임 환경변수**로만 제어. 컴파일은 항상 되고 실행만 skip.

```rust
macro_rules! skip_unless_integration {
    () => {
        if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" {
            return;
        }
    };
}

#[tokio::test]
async fn test_balance_vts() {
    skip_unless_integration!();
    let config = KisConfig::from_env_vts().unwrap();
    let client = KisClient::new(config);
    let result = client.balance().await;
    assert!(result.is_ok());
}
```

### VTS 지원 범위

VTS(모의투자)가 지원하지 않는 엔드포인트는 통합 테스트에서 `#[ignore]` 처리.
알려진 VTS 미지원 카테고리:
- `analysis/` 일부 (시세분석 랭킹 류) — 단위 테스트만 작성
- 예약주문 일부 — VTS 환경 동작 미보장

---

## 에러 처리

Plan 2의 `KisError`를 그대로 사용 (WebSocket 변형 포함):

```rust
KisError::Auth(String)             // 인증 실패
KisError::Network(reqwest::Error)  // 네트워크 오류 (재시도 가능)
KisError::Api { code, message }    // KIS API 오류 응답
KisError::WebSocket(String)        // WebSocket 오류 (재시도 가능)
KisError::Parse(serde_json::Error) // 응답 파싱 실패
KisError::Io(std::io::Error)       // 파일 I/O 오류
KisError::Lagged(u64)              // WebSocket 이벤트 유실
KisError::StreamClosed             // WebSocket 연결 종료
```

`is_retryable()` → `Network`, `WebSocket` 계열만 true.

---

## 구현 우선순위

```
Plan 3a — 주문/계좌 핵심
  place_order, cancel_order
  balance, unfilled_orders, order_history, period_profit, buyable_amount

Plan 3b — 시세 + WebSocket
  price, orderbook, daily_chart, minute_chart
  search, symbol_info, news, dividend, holidays, index_chart
  WebSocket: Price + Orderbook + OrderConfirm 구독

Plan 3c — 시세분석
  price_ranking, volume_ranking, volume_surge, volume_power,
  new_highlow, market_cap, trade_turnover

후순위 (별도 플랜)
  reserved_order, daytime_order (주간거래), foreign_margin
```

**범위 외 (bot 크레이트 담당):**
- TWAP/VWAP 같은 알고리즘 주문 로직은 `kis_api`가 아닌 `bot` 크레이트 책임

---

## 선행 조건

- Plan 1 완료: Cargo workspace + domain 크레이트
- Plan 2 완료: KisConfig, KisError, TokenManager, KisStream, KisApi trait, HTTP executor
- Plan 2의 `KisStream::subscribe()` 시그니처: `subscribe(symbol: &str, kind: SubscriptionKind)` 사용
