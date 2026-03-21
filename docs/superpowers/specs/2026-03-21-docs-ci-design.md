# rs_kis 문서 재작성 + GitHub Actions CI 설계 스펙

## 목표

1. `README.md`와 `docs/usage.md`를 현재 구현 상태에 맞게 재작성한다.
2. GitHub Actions CI 파이프라인(fmt / lint / test)을 추가한다.

---

## 범위

| 항목 | 파일 |
|------|------|
| 프로젝트 소개 + 빠른 시작 | `README.md` (전면 재작성) |
| 카테고리별 예시 + 전체 API 목록 | `docs/usage.md` (신규) |
| 포맷 검사 | `.github/workflows/fmt.yml` |
| Clippy lint | `.github/workflows/lint.yml` |
| 단위 테스트 | `.github/workflows/test.yml` |

---

## README.md 설계

### 구조

```
# rs_kis
[badges: fmt / lint / test]

## What is this?
## Setup
## Usage
## Disclaimer
```

### 섹션별 내용

**What is this?**
- 한 문단: KIS OpenAPI Rust 비공식 라이브러리, Cargo workspace 구조
- 주요 기능 bullet:
  - 해외주식 주문/취소
  - 계좌 잔고, 미체결, 체결내역, 기간손익
  - 현재가, 호가, 차트(일봉/분봉)
  - 종목 검색, 뉴스, 배당, 휴장일
  - 시세분석 (등락률/거래량/시총 순위 등)
  - WebSocket 실시간 체결가/호가

**Setup**

`.env.example`을 복사해 `.env`로 저장하고 최소 아래 3개를 채운다:

```
APP_KEY=...
APP_SECRET=...
ACCOUNT_NUM=XXXXXXXX-XX
```

나머지 변수(REST_URL, WS_URL, TOKEN_CACHE_PATH 등)의 기본값 및 VTS 설정은 `.env.example` 참고.

`KisConfig::from_env()?` / `KisConfig::from_env_vts()?` 로 로드 (`Result` 반환).

**Usage**
카테고리별 코드 스니펫 4개 (각 10~15줄). 모든 스니펫은 컴파일 가능한 형태로 작성:
- `from_env()?`, `from_env_vts()?` — `?` 연산자 포함
- WebSocket 스니펫은 `use kis_api::KisApi;` import 포함 (`stream()`은 trait method)

1. 주문/계좌 — `place_order`, `balance`
2. 시세 — `price`, `daily_chart`
3. 시세분석 — `price_ranking` (RankingRequest 필드 주석 포함)
4. WebSocket — `stream().await?`, `subscribe`, `recv`

마지막에 `→ 전체 API 목록: [docs/usage.md](docs/usage.md)` 링크.

**Dependency 주의**: 이 크레이트는 crates.io에 미게재. `path` 또는 `git` 의존성으로만 사용 가능. Cargo.toml에 `kis_api = { path = "crates/kis_api" }` 형태로 추가.

**Disclaimer**
KIS 공식 비연관, 개인 프로젝트, 사용 책임 본인.

---

## docs/usage.md 설계

### 구조

```
# API Reference

## 환경 설정
## 클라이언트 생성
## 전체 공개 메서드

### 주문/계좌
### 시세
### 시세분석
### WebSocket

## VTS 모의투자
## 통합 테스트 실행
```

### 전체 공개 메서드 (테이블)

각 섹션에 메서드명 / 설명 / 반환 타입 3열 테이블.

**주문/계좌 (7개)**

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `place_order(req)` | 해외주식 주문 | `PlaceOrderResponse` |
| `cancel_order(req)` | 주문 정정/취소 | `CancelOrderResponse` |
| `balance()` | 계좌 잔고 | `BalanceResponse` |
| `unfilled_orders()` | 미체결 목록 | `Vec<UnfilledOrder>` |
| `order_history(req)` | 체결 내역 | `Vec<OrderHistoryItem>` |
| `period_profit(req)` | 기간 손익 | `PeriodProfitResponse` |
| `buyable_amount(req)` | 매수 가능 금액 | `BuyableAmountResponse` |

> `check_deposit`은 내부 모듈에 구현되어 있으나 현재 미공개(pub use 미등록). 향후 추가 예정.

**시세 (9개)**

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

**시세분석 (7개)**

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `price_ranking(req)` | 등락률 순위 | `Vec<RankingItem>` |
| `volume_ranking(exchange, count)` | 거래량 순위 | `Vec<RankingItem>` |
| `volume_surge(exchange, count)` | 거래량 급증 | `Vec<VolumeSurgeItem>` |
| `volume_power(symbol, exchange)` | 체결강도 | `Vec<VolumePowerItem>` |
| `new_highlow(exchange, kind, count)` | 신고/신저가 (`HighLowKind::High` / `::Low`) | `Vec<NewHighLowItem>` |
| `market_cap(exchange, count)` | 시가총액 순위 | `Vec<MarketCapItem>` |
| `trade_turnover(exchange, count)` | 거래회전율 | `Vec<TradeTurnoverItem>` |

`RankingRequest` 필드 (docs/usage.md에 별도 설명):
- `exchange: Exchange`
- `sort: RankingSort` — `RankingSort::ChangeRate` / `::Volume` / `::MarketCap`
- `count: u32`

`HighLowKind` 변형: `High` (신고가), `Low` (신저가).

**WebSocket**

`stream()`은 `KisApi` trait method — `use kis_api::KisApi;` import 필요.

| 메서드 | 설명 | 반환 |
|--------|------|------|
| `stream()` | WebSocket 스트림 생성 (`KisApi` trait) | `KisStream` |
| `stream.subscribe(symbol, kind)` | 종목 구독 (`SubscriptionKind::Price` / `::Orderbook`) | `()` |
| `stream.unsubscribe(symbol, kind)` | 종목 구독 해제 | `()` |
| `stream.receiver()` | 이벤트 수신 채널 획득 | `EventReceiver` |
| `receiver.recv()` | 이벤트 수신 (`KisEvent::Transaction` / `::Quote`) | `Result<KisEvent, KisError>` |

### 통합 테스트 실행

환경변수 `KIS_INTEGRATION_TEST=1`로 설정 후 실행. `.env.example`의 `--features integration` 주석은 오기이며, 실제 구현은 런타임 env var 방식임.

구현 시 `.env.example`의 해당 주석도 수정할 것:
```
# 기존(오기): cargo test --features integration 실행 시 참조
# 수정: KIS_INTEGRATION_TEST=1 cargo test -p kis_api
```

```bash
KIS_INTEGRATION_TEST=1 cargo test -p kis_api
```

VTS 모의투자 환경 사용 권장 (`from_env_vts()`).

### KisConfigBuilder

`KisConfigBuilder`는 `lib.rs`에서 공개 export되어 있음. env 파일 없이 코드에서 직접 설정을 구성할 때 사용:

```rust
let config = KisConfig::builder()
    .app_key("...")
    .app_secret("...")
    .account_num("XXXXXXXX-XX")
    .mock(false)
    .build()?;
```

`docs/usage.md` "클라이언트 생성" 섹션에 `from_env()` 방식과 함께 builder 방식도 병기.

---

## GitHub Actions CI 설계

### 공통 설정

- 트리거: `push` (모든 브랜치) + `pull_request` (master 대상만)
- 러너: `ubuntu-latest`
- Rust 툴체인: `dtolnay/rust-toolchain@stable`
- 캐시: `Swatinem/rust-cache@v2` (fmt 포함 3개 모두 적용)

### fmt.yml

```yaml
name: fmt
on:
  push:
  pull_request:
    branches: [master]
jobs:
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all --check
```

### lint.yml

```yaml
name: lint
on:
  push:
  pull_request:
    branches: [master]
jobs:
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace --all-targets -- -D warnings
```

### test.yml

```yaml
name: test
on:
  push:
  pull_request:
    branches: [master]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace
        # KIS_INTEGRATION_TEST 미설정 → 단위 테스트만 실행
```

### README badge

배지 URL (완성형):

```markdown
[![fmt](https://github.com/soki1229/rs_kis/actions/workflows/fmt.yml/badge.svg)](https://github.com/soki1229/rs_kis/actions/workflows/fmt.yml)
[![lint](https://github.com/soki1229/rs_kis/actions/workflows/lint.yml/badge.svg)](https://github.com/soki1229/rs_kis/actions/workflows/lint.yml)
[![test](https://github.com/soki1229/rs_kis/actions/workflows/test.yml/badge.svg)](https://github.com/soki1229/rs_kis/actions/workflows/test.yml)
```

---

## 파일 변경 요약

| 파일 | 작업 |
|------|------|
| `README.md` | 전면 재작성 |
| `docs/usage.md` | 신규 생성 |
| `.github/workflows/fmt.yml` | 신규 생성 |
| `.github/workflows/lint.yml` | 신규 생성 |
| `.github/workflows/test.yml` | 신규 생성 |
