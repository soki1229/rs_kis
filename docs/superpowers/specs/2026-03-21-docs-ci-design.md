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
```
# .env 파일 생성 (.env.example 참고)
APP_KEY=...
APP_SECRET=...
ACCOUNT_NUM=XXXXXXXX-XX
```

`KisConfig::from_env()` / `KisConfig::from_env_vts()` 사용.

**Usage**
카테고리별 코드 스니펫 4개:
1. 주문/계좌 — `place_order`, `balance`
2. 시세 — `price`, `daily_chart`
3. 시세분석 — `price_ranking`
4. WebSocket — `stream().await?`, `subscribe`, `recv`

각 스니펫 10~15줄. 마지막에 `→ 전체 API 목록: [docs/usage.md](docs/usage.md)` 링크.

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
| `new_highlow(exchange, kind, count)` | 신고/신저가 | `Vec<NewHighLowItem>` |
| `market_cap(exchange, count)` | 시가총액 순위 | `Vec<MarketCapItem>` |
| `trade_turnover(exchange, count)` | 거래회전율 | `Vec<TradeTurnoverItem>` |

**WebSocket**

`client.stream().await?` → `KisStream`
`stream.subscribe(symbol, SubscriptionKind::Price).await?`
`stream.receiver()` → `EventReceiver` → `recv().await` → `Result<KisEvent, KisError>`

---

## GitHub Actions CI 설계

### 공통 설정

- 트리거: `push` (모든 브랜치) + `pull_request` (master 대상)
- 러너: `ubuntu-latest`
- Rust 툴체인: `dtolnay/rust-toolchain@stable`
- 캐시: `Swatinem/rust-cache@v2`

### fmt.yml

```yaml
name: fmt
on: [push, pull_request]
jobs:
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all --check
```

### lint.yml

```yaml
name: lint
on: [push, pull_request]
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
on: [push, pull_request]
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

```markdown
[![fmt](https://github.com/soki1229/rs_kis/actions/workflows/fmt.yml/badge.svg)](...)
[![lint](https://github.com/soki1229/rs_kis/actions/workflows/lint.yml/badge.svg)](...)
[![test](https://github.com/soki1229/rs_kis/actions/workflows/test.yml/badge.svg)](...)
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
