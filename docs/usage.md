# API Reference

KIS OpenAPI Rust 라이브러리 `kis_api`의 전체 공개 API 목록과 사용법.

---

## 환경 설정

`.env.example`을 복사해 `.env`로 저장하고 값을 채운다:

```bash
cp .env.example .env
```

최소 필수 항목:

| 변수 | 설명 |
|------|------|
| `APP_KEY` | KIS 실전투자 앱키 |
| `APP_SECRET` | KIS 실전투자 시크릿 |
| `ACCOUNT_NUM` | 계좌번호 (예: `50123456-01`) |

나머지 변수(`REST_URL`, `WS_URL`, `TOKEN_CACHE_PATH`)는 기본값이 설정되어 있으므로 변경 불필요.
VTS 모의투자 변수는 [VTS 모의투자](#vts-모의투자) 섹션 참고.

---

## 클라이언트 생성

**환경변수 방식 (권장):**

```rust
use kis_api::{KisClient, KisConfig};

let config = KisConfig::from_env()?;  // .env 로드
let client = KisClient::new(config);
```

**Builder 방식 (코드에서 직접 설정):**

```rust
use kis_api::{KisClient, KisConfig};

let config = KisConfig::builder()
    .app_key("YOUR_APP_KEY")
    .app_secret("YOUR_APP_SECRET")
    .account_num("XXXXXXXX-XX")
    .mock(false)
    .build()?;
let client = KisClient::new(config);
```

`KisClient`는 `Clone` 가능(`Arc<Inner>` 내부 공유). 여러 태스크에서 공유 시 `.clone()` 사용.

---

## 전체 공개 메서드

### 주문/계좌

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `place_order(req)` | 해외주식 주문 | `PlaceOrderResponse` |
| `cancel_order(req)` | 주문 정정/취소 | `CancelOrderResponse` |
| `balance()` | 계좌 잔고 | `BalanceResponse` |
| `unfilled_orders()` | 미체결 목록 | `Vec<UnfilledOrder>` |
| `order_history(req)` | 체결 내역 | `Vec<OrderHistoryItem>` |
| `period_profit(req)` | 기간 손익 | `PeriodProfitResponse` |
| `buyable_amount(req)` | 매수 가능 금액 | `BuyableAmountResponse` |

> `check_deposit`은 내부 구현되어 있으나 현재 미공개. 향후 추가 예정.

---

### 시세

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `price(symbol, exchange)` | 현재가 상세 | `PriceResponse` |
| `orderbook(symbol, exchange)` | 호가 | `OrderbookResponse` |
| `daily_chart(req)` | 일봉 | `Vec<CandleBar>` |
| `minute_chart(req)` | 분봉 | `Vec<MinuteBar>` |
| `search(keyword)` | 종목 검색 | `Vec<SearchResult>` |
| `symbol_info(symbol, exchange)` | 종목 정보 | `SymbolInfo` |
| `news(symbol)` | 뉴스 | `Vec<NewsItem>` |
| `dividend(symbol, exchange)` | 배당/권리 | `Vec<DividendItem>` |
| `holidays(country)` | 국가별 휴장일 | `Vec<Holiday>` |

---

### 시세분석

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `price_ranking(req)` | 등락률 순위 (sort 필드로 기준 선택) | `Vec<RankingItem>` |
| `volume_ranking(exchange, count)` | 거래량 순위 | `Vec<RankingItem>` |
| `volume_surge(exchange, count)` | 거래량 급증 | `Vec<VolumeSurgeItem>` |
| `volume_power(symbol, exchange)` | 체결강도 | `Vec<VolumePowerItem>` |
| `new_highlow(exchange, kind, count)` | 신고/신저가 | `Vec<NewHighLowItem>` |
| `market_cap(exchange, count)` | 시가총액 순위 | `Vec<MarketCapItem>` |
| `trade_turnover(exchange, count)` | 거래회전율 | `Vec<TradeTurnoverItem>` |

**`RankingRequest` 필드:**

```rust
use kis_api::{Exchange, RankingRequest, RankingSort};

let req = RankingRequest {
    exchange: Exchange::NASD,
    sort: RankingSort::ChangeRate, // ChangeRate / Volume / MarketCap
    count: 10,
};
```

**`HighLowKind` 변형:** `HighLowKind::High` (신고가), `HighLowKind::Low` (신저가).

---

### WebSocket

`stream()`은 `KisApi` trait method. 반드시 `use kis_api::KisApi;` import 필요.

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `stream()` | WebSocket 스트림 생성 (`KisApi` trait) | `KisStream` |
| `stream.subscribe(symbol, kind)` | 종목 구독 | `()` |
| `stream.unsubscribe(symbol, kind)` | 종목 구독 해제 | `()` |
| `stream.receiver()` | 이벤트 수신 채널 획득 | `EventReceiver` |
| `receiver.recv()` | 이벤트 수신 | `Result<KisEvent, KisError>` |

`SubscriptionKind` 변형: `Price` (실시간 체결가), `Orderbook` (실시간 호가).

`KisEvent` 변형: `Transaction(TransactionData)`, `Quote(QuoteData)`, `OrderConfirm(OrderConfirmData)`.

`KisError::Lagged(n)` — 수신 버퍼 초과로 `n`개 이벤트 유실.
`KisError::StreamClosed` — 연결 종료.

---

## VTS 모의투자

KIS에서 제공하는 VTS(Virtual Trading System) 환경으로 실제 주문 없이 테스트 가능.

`.env`에 VTS 변수 추가 (`.env.example` 참고):

```
VTS_APP_KEY=...
VTS_APP_SECRET=...
VTS_ACCOUNT_NUM=XXXXXXXX-XX
```

```rust
let config = KisConfig::from_env_vts()?;  // VTS 환경 자동 선택
let client = KisClient::new(config);
```

`from_env_vts()`는 `VTS_APP_KEY`, `VTS_APP_SECRET`, `VTS_ACCOUNT_NUM`을 로드하고 `mock: true`로 설정한다. REST/WebSocket URL도 VTS 전용으로 자동 전환됨.

---

## 통합 테스트 실행

단위 테스트는 fixture JSON 기반으로 항상 실행 가능:

```bash
cargo test -p kis_api
```

실제 KIS API를 호출하는 통합 테스트는 환경변수로 활성화:

```bash
KIS_INTEGRATION_TEST=1 cargo test -p kis_api
```

VTS 환경 사용 권장. `.env`에 VTS 변수가 설정되어 있어야 함.
