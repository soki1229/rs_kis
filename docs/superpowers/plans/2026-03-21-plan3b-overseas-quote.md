# Plan 3b: 해외주식 시세 REST API

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `crates/kis_api`에 해외주식 시세 REST API 전체를 구현한다 — 현재가, 호가, 일/분봉 차트, 종목검색, 기업정보(뉴스/배당/휴장일) 포함 총 11개 엔드포인트.

**Architecture:** Plan 2의 flat `rest/overseas/price.rs`를 `rest/overseas/quote/` 서브디렉터리로 재구조화한다. 각 기능군은 독립 파일(`price.rs`, `orderbook.rs`, `chart.rs`, `search.rs`, `corporate.rs`)로 분리된다. 모든 함수는 `async fn(config, token, ...)` 시그니처를 유지하고, `KisClient` 퍼블릭 메서드가 이를 래핑한다.

**Tech Stack:** Rust 2021, `tokio`, `reqwest`, `serde`, `serde_json`, `rust_decimal`, `chrono`, `thiserror`

**선행 조건:**
- Plan 2 완료: `KisConfig`, `KisError`, `TokenManager`, `KisClient`, `KisStream`, `rest::http::execute`
- Plan 3a 완료: `rest/overseas/types.rs` — `Exchange`, `OrderSide`, `OrderType`, `SubscriptionKind`

---

## File Map

```
crates/kis_api/src/rest/overseas/
├── mod.rs                  MODIFY  — quote 서브모듈 추가
│
├── types.rs                (Plan 3a — 변경 없음)
├── order/                  (Plan 3a — 변경 없음)
├── inquiry/                (Plan 3a — 변경 없음)
│
└── quote/
    ├── mod.rs              CREATE  — 서브모듈 선언 + pub use
    ├── price.rs            CREATE  — PriceResponse, price(), exchange_to_price_code()
    ├── orderbook.rs        CREATE  — OrderbookResponse, OrderbookLevel, orderbook()
    ├── chart.rs            CREATE  — DailyChartRequest, MinuteChartRequest, CandleBar, MinuteBar
    ├── search.rs           CREATE  — SearchResult, SymbolInfo, search(), symbol_info()
    └── corporate.rs        CREATE  — NewsItem, DividendItem, Holiday + 3개 함수

crates/kis_api/tests/
└── fixtures/
    └── overseas/
        └── quote/
            ├── price.json               CREATE — /quotations/price 응답 픽스처
            ├── orderbook.json           CREATE — /quotations/inquire-asking-price 픽스처
            ├── daily_chart.json         CREATE — /quotations/dailyprice 픽스처
            ├── minute_chart.json        CREATE — /quotations/inquire-time-itemchartprice 픽스처
            ├── search.json              CREATE — /quotations/inquire-search 픽스처
            ├── symbol_info.json         CREATE — /quotations/search-info 픽스처
            ├── news.json                CREATE — /quotations/news-title 픽스처
            ├── dividend.json            CREATE — /quotations/period-rights 픽스처
            └── holidays.json           CREATE — /quotations/countries-holiday 픽스처

crates/kis_api/src/client.rs            MODIFY  — 시세 퍼블릭 메서드 추가
crates/kis_api/src/lib.rs               MODIFY  — pub use 추가
```

---

## Task 0: 디렉터리 재구조화 — flat price.rs → quote/ 서브디렉터리

Plan 2가 생성한 `rest/overseas/price.rs`를 `rest/overseas/quote/price.rs`로 이동하고, `quote/` 서브디렉터리 구조를 확립한다.

**Files:**
- Create: `crates/kis_api/src/rest/overseas/quote/mod.rs`
- Move content: `price.rs` → `quote/price.rs` (Plan 2 `current_price` 함수 유지)
- Modify: `crates/kis_api/src/rest/overseas/mod.rs`

### Step 0-1: `quote/` 디렉터리 생성 및 기존 price.rs 이동

```bash
mkdir -p crates/kis_api/src/rest/overseas/quote
# Plan 2의 price.rs 내용을 quote/price.rs로 이동
# (Plan 2가 price.rs를 생성하지 않은 경우 빈 파일로 시작)
mv crates/kis_api/src/rest/overseas/price.rs \
   crates/kis_api/src/rest/overseas/quote/price.rs
```

Plan 2가 `price.rs`를 생성하지 않았다면:

```bash
mkdir -p crates/kis_api/src/rest/overseas/quote
touch crates/kis_api/src/rest/overseas/quote/price.rs
```

### Step 0-2: `quote/mod.rs` 생성

```rust
// crates/kis_api/src/rest/overseas/quote/mod.rs
pub mod price;
pub mod orderbook;
pub mod chart;
pub mod search;
pub mod corporate;

pub use price::{PriceResponse, price};
pub use orderbook::{OrderbookLevel, OrderbookResponse, orderbook};
pub use chart::{
    ChartPeriod, DailyChartRequest, MinuteChartRequest,
    CandleBar, MinuteBar, daily_chart, minute_chart,
};
pub use search::{SearchResult, SymbolInfo, search, symbol_info};
pub use corporate::{NewsItem, DividendItem, Holiday, news, dividend, holidays};
```

나머지 파일들은 태스크별로 채운다. 이 시점에서는 각 파일을 빈 상태로 생성:

```bash
touch crates/kis_api/src/rest/overseas/quote/orderbook.rs
touch crates/kis_api/src/rest/overseas/quote/chart.rs
touch crates/kis_api/src/rest/overseas/quote/search.rs
touch crates/kis_api/src/rest/overseas/quote/corporate.rs
```

### Step 0-3: `rest/overseas/mod.rs` 수정

```rust
// crates/kis_api/src/rest/overseas/mod.rs
pub mod types;
pub mod order;
pub mod inquiry;
pub mod quote;

pub use types::{Exchange, OrderSide, OrderType, SubscriptionKind};
```

### Step 0-4: 컴파일 확인

```bash
cargo check -p kis_api
```

Expected: 성공 (빈 파일들은 경고만 발생, 에러 없음)

- [ ] **Step 0-1: quote/ 디렉터리 생성 + price.rs 이동**
- [ ] **Step 0-2: quote/mod.rs 생성 (빈 re-export 포함)**
- [ ] **Step 0-3: rest/overseas/mod.rs에 `pub mod quote` 추가**
- [ ] **Step 0-4: `cargo check -p kis_api` 통과**
- [ ] **Step 0-5: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/
git commit -m "refactor(kis_api): move overseas/price.rs into quote/ subdirectory"
```

---

## Task 1: price() — 현재가

**Endpoint:** `GET /uapi/overseas-price/v1/quotations/price`
**TR ID:** `HHDFS00000300`
**Query params:** `AUTH("")`, `EXCD`(3-letter code), `SYMB`

**Files:**
- Rewrite: `crates/kis_api/src/rest/overseas/quote/price.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/price.json`

### Step 1-1: 픽스처 JSON 작성

```bash
mkdir -p crates/kis_api/tests/fixtures/overseas/quote
```

`crates/kis_api/tests/fixtures/overseas/quote/price.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": {
    "rsym": "DNASD  AAPL",
    "zdiv": "2",
    "curr": "USD",
    "vnit": "1",
    "open": "178.5000",
    "high": "182.3400",
    "low":  "177.9200",
    "last": "181.2300",
    "base": "179.6600",
    "pvol": "55123456",
    "pamt": "9912345678.00",
    "tvol": "62345100",
    "tamt": "11287654321.00",
    "ordy": "Y",
    "pbid": "181.2200",
    "pask": "181.2400",
    "diff": "1.5700",
    "rate": "0.87"
  }
}
```

### Step 1-2: 실패하는 테스트 먼저 작성

`quote/price.rs` 상단에 테스트 모듈을 먼저 작성:

```rust
// crates/kis_api/src/rest/overseas/quote/price.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture() -> serde_json::Value {
        let json = std::fs::read_to_string(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/fixtures/overseas/quote/price.json"
            )
        )
        .expect("fixture file not found");
        serde_json::from_str(&json).expect("invalid json")
    }

    #[test]
    fn parse_price_response() {
        let raw: KisPriceRaw = serde_json::from_value(
            load_fixture()["output"].clone()
        )
        .expect("deserialize failed");

        let resp = PriceResponse::from(raw);

        assert_eq!(resp.last, dec!(181.2300));
        assert_eq!(resp.open, dec!(178.5000));
        assert_eq!(resp.high, dec!(182.3400));
        assert_eq!(resp.low,  dec!(177.9200));
        assert_eq!(resp.diff, dec!(1.5700));
        assert_eq!(resp.rate, dec!(0.87));
        assert_eq!(resp.volume, dec!(62345100));
        assert_eq!(resp.bid,  dec!(181.2200));
        assert_eq!(resp.ask,  dec!(181.2400));
        assert!(resp.orderable);
    }

    #[test]
    fn exchange_price_codes() {
        use crate::rest::overseas::types::Exchange;
        assert_eq!(exchange_to_price_code(&Exchange::NASD), "NAS");
        assert_eq!(exchange_to_price_code(&Exchange::NYSE),  "NYS");
        assert_eq!(exchange_to_price_code(&Exchange::AMEX),  "AMS");
        assert_eq!(exchange_to_price_code(&Exchange::TKSE),  "TSE");
        assert_eq!(exchange_to_price_code(&Exchange::SEHK),  "HKS");
        assert_eq!(exchange_to_price_code(&Exchange::SHAA),  "SHS");
        assert_eq!(exchange_to_price_code(&Exchange::SZAA),  "SZS");
        assert_eq!(exchange_to_price_code(&Exchange::HASE),  "HSX");
        assert_eq!(exchange_to_price_code(&Exchange::VNSE),  "HNX");
    }
}
```

```bash
cargo test -p kis_api rest::overseas::quote::price::tests 2>&1 | head -30
```

Expected: 컴파일 에러 (`PriceResponse`, `KisPriceRaw`, `exchange_to_price_code` 미정의)

### Step 1-3: 구현 작성

```rust
// crates/kis_api/src/rest/overseas/quote/price.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::config::KisConfig;
use crate::error::KisError;
use crate::rest::http::execute;
use crate::rest::overseas::types::Exchange;

// ── Exchange → 3-letter price endpoint code ──────────────────────────────────
// 주의: 이 엔드포인트는 NASD 대신 NAS 등 3자 코드를 사용함 (다른 엔드포인트와 다름)
pub fn exchange_to_price_code(e: &Exchange) -> &'static str {
    match e {
        Exchange::NASD => "NAS",
        Exchange::NYSE => "NYS",
        Exchange::AMEX => "AMS",
        Exchange::TKSE => "TSE",
        Exchange::SEHK => "HKS",
        Exchange::SHAA => "SHS",
        Exchange::SZAA => "SZS",
        Exchange::HASE => "HSX",
        Exchange::VNSE => "HNX",
        Exchange::Other(s) => {
            // Other는 호출자가 올바른 코드를 직접 넘길 책임
            // static str 반환 제약으로 인해 "???" 반환 (실제 사용 시 Other는 피할 것)
            let _ = s;
            "???"
        }
    }
}

// ── KIS API raw JSON 역직렬화 타입 ────────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub(crate) struct KisPriceRaw {
    #[serde(with = "rust_decimal::serde::str")]
    pub last: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub open: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub high: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub low: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub diff: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub rate: Decimal,
    #[serde(rename = "tvol", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(rename = "pbid", with = "rust_decimal::serde::str")]
    pub bid: Decimal,
    #[serde(rename = "pask", with = "rust_decimal::serde::str")]
    pub ask: Decimal,
    #[serde(rename = "ordy")]
    pub ordy: String,
}

// ── 공개 응답 타입 ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PriceResponse {
    /// 현재가 (LAST)
    pub last: Decimal,
    /// 시가 (OPEN)
    pub open: Decimal,
    /// 고가 (HIGH)
    pub high: Decimal,
    /// 저가 (LOW)
    pub low: Decimal,
    /// 전일대비 (DIFF)
    pub diff: Decimal,
    /// 등락률 % (RATE)
    pub rate: Decimal,
    /// 거래량 (TVOL)
    pub volume: Decimal,
    /// 매수호가 (PBID)
    pub bid: Decimal,
    /// 매도호가 (PASK)
    pub ask: Decimal,
    /// 주문가능여부 (ORDY == "Y")
    pub orderable: bool,
}

impl From<KisPriceRaw> for PriceResponse {
    fn from(r: KisPriceRaw) -> Self {
        Self {
            last: r.last,
            open: r.open,
            high: r.high,
            low: r.low,
            diff: r.diff,
            rate: r.rate,
            volume: r.volume,
            bid: r.bid,
            ask: r.ask,
            orderable: r.ordy == "Y",
        }
    }
}

// ── KIS API wrapper 응답 ───────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
struct PriceApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: KisPriceRaw,
}

// ── 공개 함수 ──────────────────────────────────────────────────────────────────
/// 해외주식 현재가 조회
///
/// GET /uapi/overseas-price/v1/quotations/price  (TR: HHDFS00000300)
pub async fn price(
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<PriceResponse, KisError> {
    let excd = exchange_to_price_code(exchange);

    let params = [
        ("AUTH", ""),
        ("EXCD", excd),
        ("SYMB", symbol),
    ];

    let resp: PriceApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/price",
        "HHDFS00000300",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(PriceResponse::from(resp.output))
}

#[cfg(test)]
mod tests {
    // ... (위 Step 1-2 테스트 코드)
}
```

### Step 1-4: 테스트 통과 확인

```bash
cargo test -p kis_api rest::overseas::quote::price::tests
```

Expected:
```
test rest::overseas::quote::price::tests::parse_price_response ... ok
test rest::overseas::quote::price::tests::exchange_price_codes ... ok

test result: ok. 2 passed; 0 failed
```

- [ ] **Step 1-1: 픽스처 JSON 작성**
- [ ] **Step 1-2: 실패하는 테스트 먼저 작성 + 실패 확인**
- [ ] **Step 1-3: PriceResponse, KisPriceRaw, exchange_to_price_code 구현**
- [ ] **Step 1-4: 테스트 통과 확인**
- [ ] **Step 1-5: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/quote/price.rs \
        crates/kis_api/tests/fixtures/overseas/quote/price.json
git commit -m "feat(kis_api): implement overseas price() with PriceResponse"
```

---

## Task 2: orderbook() — 호가 (매수/매도 5단계)

**Endpoint:** `GET /uapi/overseas-price/v1/quotations/inquire-asking-price`
**TR ID:** `HHDFS76200100`
**Query params:** `AUTH("")`, `EXCD`(4-letter), `SYMB`

> 이 엔드포인트는 4-letter 거래소 코드(NASD, NYSE 등)를 사용한다. `types.rs`의 `Exchange::to_code()` 활용.

**Files:**
- Write: `crates/kis_api/src/rest/overseas/quote/orderbook.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/orderbook.json`

### Step 2-1: 픽스처 JSON 작성

`crates/kis_api/tests/fixtures/overseas/quote/orderbook.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": {
    "rsym": "DNASD  AAPL",
    "last": "181.2300",
    "bidp": "181.2200",
    "askp": "181.2400",
    "bidp2": "181.2100",
    "askp2": "181.2500",
    "bidp3": "181.2000",
    "askp3": "181.2600",
    "bidp4": "181.1900",
    "askp4": "181.2700",
    "bidp5": "181.1800",
    "askp5": "181.2800",
    "bidq": "1200",
    "askq": "800",
    "bidq2": "950",
    "askq2": "1100",
    "bidq3": "2300",
    "askq3": "600",
    "bidq4": "1800",
    "askq4": "450",
    "bidq5": "3200",
    "askq5": "2100"
  }
}
```

### Step 2-2: 실패하는 테스트 작성

```rust
// crates/kis_api/src/rest/overseas/quote/orderbook.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture() -> serde_json::Value {
        let json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/orderbook.json"
        ))
        .expect("fixture not found");
        serde_json::from_str(&json).expect("invalid json")
    }

    #[test]
    fn parse_orderbook_response() {
        let raw: KisOrderbookRaw =
            serde_json::from_value(load_fixture()["output"].clone())
                .expect("deserialize failed");

        let resp = OrderbookResponse::from(raw);

        assert_eq!(resp.last, dec!(181.2300));
        assert_eq!(resp.symbol, "DNASD  AAPL");
        assert_eq!(resp.levels.len(), 5);

        let l1 = &resp.levels[0];
        assert_eq!(l1.bid_price, dec!(181.2200));
        assert_eq!(l1.ask_price, dec!(181.2400));
        assert_eq!(l1.bid_qty,   dec!(1200));
        assert_eq!(l1.ask_qty,   dec!(800));

        let l5 = &resp.levels[4];
        assert_eq!(l5.bid_price, dec!(181.1800));
        assert_eq!(l5.ask_price, dec!(181.2800));
    }
}
```

```bash
cargo test -p kis_api rest::overseas::quote::orderbook::tests 2>&1 | head -20
```

Expected: 컴파일 에러 (미구현)

### Step 2-3: 구현 작성

```rust
// crates/kis_api/src/rest/overseas/quote/orderbook.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::config::KisConfig;
use crate::error::KisError;
use crate::rest::http::execute;
use crate::rest::overseas::types::Exchange;

#[derive(Debug, Deserialize)]
pub(crate) struct KisOrderbookRaw {
    #[serde(rename = "rsym")]
    pub symbol: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub last: Decimal,
    // 매수호가 1~5
    #[serde(rename = "bidp", with = "rust_decimal::serde::str")]
    pub bidp1: Decimal,
    #[serde(rename = "bidp2", with = "rust_decimal::serde::str")]
    pub bidp2: Decimal,
    #[serde(rename = "bidp3", with = "rust_decimal::serde::str")]
    pub bidp3: Decimal,
    #[serde(rename = "bidp4", with = "rust_decimal::serde::str")]
    pub bidp4: Decimal,
    #[serde(rename = "bidp5", with = "rust_decimal::serde::str")]
    pub bidp5: Decimal,
    // 매도호가 1~5
    #[serde(rename = "askp", with = "rust_decimal::serde::str")]
    pub askp1: Decimal,
    #[serde(rename = "askp2", with = "rust_decimal::serde::str")]
    pub askp2: Decimal,
    #[serde(rename = "askp3", with = "rust_decimal::serde::str")]
    pub askp3: Decimal,
    #[serde(rename = "askp4", with = "rust_decimal::serde::str")]
    pub askp4: Decimal,
    #[serde(rename = "askp5", with = "rust_decimal::serde::str")]
    pub askp5: Decimal,
    // 매수잔량 1~5
    #[serde(rename = "bidq", with = "rust_decimal::serde::str")]
    pub bidq1: Decimal,
    #[serde(rename = "bidq2", with = "rust_decimal::serde::str")]
    pub bidq2: Decimal,
    #[serde(rename = "bidq3", with = "rust_decimal::serde::str")]
    pub bidq3: Decimal,
    #[serde(rename = "bidq4", with = "rust_decimal::serde::str")]
    pub bidq4: Decimal,
    #[serde(rename = "bidq5", with = "rust_decimal::serde::str")]
    pub bidq5: Decimal,
    // 매도잔량 1~5
    #[serde(rename = "askq", with = "rust_decimal::serde::str")]
    pub askq1: Decimal,
    #[serde(rename = "askq2", with = "rust_decimal::serde::str")]
    pub askq2: Decimal,
    #[serde(rename = "askq3", with = "rust_decimal::serde::str")]
    pub askq3: Decimal,
    #[serde(rename = "askq4", with = "rust_decimal::serde::str")]
    pub askq4: Decimal,
    #[serde(rename = "askq5", with = "rust_decimal::serde::str")]
    pub askq5: Decimal,
}

/// 단일 호가 레벨 (매수/매도 한 쌍)
#[derive(Debug, Clone)]
pub struct OrderbookLevel {
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    pub bid_qty: Decimal,
    pub ask_qty: Decimal,
}

/// 호가 응답 (5단계)
#[derive(Debug, Clone)]
pub struct OrderbookResponse {
    /// 종목코드 (RSYM)
    pub symbol: String,
    /// 현재가 (LAST)
    pub last: Decimal,
    /// 호가 5단계 (levels[0]이 최우선)
    pub levels: Vec<OrderbookLevel>,
}

impl From<KisOrderbookRaw> for OrderbookResponse {
    fn from(r: KisOrderbookRaw) -> Self {
        let bid_prices = [r.bidp1, r.bidp2, r.bidp3, r.bidp4, r.bidp5];
        let ask_prices = [r.askp1, r.askp2, r.askp3, r.askp4, r.askp5];
        let bid_qtys   = [r.bidq1, r.bidq2, r.bidq3, r.bidq4, r.bidq5];
        let ask_qtys   = [r.askq1, r.askq2, r.askq3, r.askq4, r.askq5];

        let levels = (0..5)
            .map(|i| OrderbookLevel {
                bid_price: bid_prices[i],
                ask_price: ask_prices[i],
                bid_qty:   bid_qtys[i],
                ask_qty:   ask_qtys[i],
            })
            .collect();

        Self {
            symbol: r.symbol,
            last: r.last,
            levels,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OrderbookApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: KisOrderbookRaw,
}

/// 해외주식 호가 조회 (5단계)
///
/// GET /uapi/overseas-price/v1/quotations/inquire-asking-price  (TR: HHDFS76200100)
pub async fn orderbook(
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<OrderbookResponse, KisError> {
    let params = [
        ("AUTH", ""),
        ("EXCD", exchange.as_code()),
        ("SYMB", symbol),
    ];

    let resp: OrderbookApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/inquire-asking-price",
        "HHDFS76200100",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(OrderbookResponse::from(resp.output))
}
```

> `exchange.as_code()` — Plan 3a의 `types.rs`에 정의된 `Exchange` 메서드. 없으면 `exchange_to_4letter_code(exchange)` 헬퍼를 로컬에 정의한다.

### Step 2-4: 테스트 통과 확인

```bash
cargo test -p kis_api rest::overseas::quote::orderbook::tests
```

Expected:
```
test rest::overseas::quote::orderbook::tests::parse_orderbook_response ... ok

test result: ok. 1 passed; 0 failed
```

- [ ] **Step 2-1: 픽스처 JSON 작성**
- [ ] **Step 2-2: 실패하는 테스트 작성 + 실패 확인**
- [ ] **Step 2-3: OrderbookLevel, OrderbookResponse, orderbook() 구현**
- [ ] **Step 2-4: 테스트 통과 확인**
- [ ] **Step 2-5: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/quote/orderbook.rs \
        crates/kis_api/tests/fixtures/overseas/quote/orderbook.json
git commit -m "feat(kis_api): implement overseas orderbook() with 5-level order book"
```

---

## Task 3: daily_chart() + minute_chart() — 차트 데이터

**Endpoints:**
- `GET /uapi/overseas-price/v1/quotations/dailyprice` (TR: `HHDFS76240000`)
- `GET /uapi/overseas-price/v1/quotations/inquire-time-itemchartprice` (TR: `HHDFS76950200`)

**Files:**
- Write: `crates/kis_api/src/rest/overseas/quote/chart.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/daily_chart.json`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/minute_chart.json`

### Step 3-1: 픽스처 JSON 작성

`crates/kis_api/tests/fixtures/overseas/quote/daily_chart.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output1": {
    "rsym": "DNASD  AAPL",
    "last": "181.2300",
    "base": "179.6600"
  },
  "output2": [
    {
      "xymd": "20260321",
      "open": "178.5000",
      "high": "182.3400",
      "low":  "177.9200",
      "clos": "181.2300",
      "vol":  "62345100",
      "diff": "1.5700",
      "rate": "0.87"
    },
    {
      "xymd": "20260320",
      "open": "176.8000",
      "high": "180.1000",
      "low":  "175.5000",
      "clos": "179.6600",
      "vol":  "48123400",
      "diff": "2.8600",
      "rate": "1.62"
    }
  ]
}
```

`crates/kis_api/tests/fixtures/overseas/quote/minute_chart.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output1": {
    "rsym": "DNASD  AAPL"
  },
  "output2": [
    {
      "kymd": "20260321",
      "khms": "143000",
      "open": "181.1000",
      "high": "181.3500",
      "low":  "181.0500",
      "last": "181.2300",
      "evol": "125400"
    },
    {
      "kymd": "20260321",
      "khms": "142900",
      "open": "180.9000",
      "high": "181.1200",
      "low":  "180.8500",
      "last": "181.1000",
      "evol": "98700"
    }
  ]
}
```

### Step 3-2: 실패하는 테스트 작성

```rust
// crates/kis_api/src/rest/overseas/quote/chart.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_daily() -> serde_json::Value {
        let json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/daily_chart.json"
        ))
        .expect("fixture not found");
        serde_json::from_str(&json).expect("invalid json")
    }

    fn load_minute() -> serde_json::Value {
        let json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/minute_chart.json"
        ))
        .expect("fixture not found");
        serde_json::from_str(&json).expect("invalid json")
    }

    #[test]
    fn parse_daily_chart() {
        let bars: Vec<KisCandleRaw> =
            serde_json::from_value(load_daily()["output2"].clone())
                .expect("deserialize failed");

        assert_eq!(bars.len(), 2);
        let bar = CandleBar::from(bars[0].clone());

        assert_eq!(bar.date, "20260321");
        assert_eq!(bar.open,   dec!(178.5000));
        assert_eq!(bar.high,   dec!(182.3400));
        assert_eq!(bar.low,    dec!(177.9200));
        assert_eq!(bar.close,  dec!(181.2300));
        assert_eq!(bar.volume, dec!(62345100));
        assert_eq!(bar.diff,   dec!(1.5700));
        assert_eq!(bar.rate,   dec!(0.87));
    }

    #[test]
    fn parse_minute_chart() {
        let bars: Vec<KisMinuteRaw> =
            serde_json::from_value(load_minute()["output2"].clone())
                .expect("deserialize failed");

        assert_eq!(bars.len(), 2);
        let bar = MinuteBar::from(bars[0].clone());

        assert_eq!(bar.date,   "20260321");
        assert_eq!(bar.time,   "143000");
        assert_eq!(bar.open,   dec!(181.1000));
        assert_eq!(bar.high,   dec!(181.3500));
        assert_eq!(bar.low,    dec!(181.0500));
        assert_eq!(bar.close,  dec!(181.2300));
        assert_eq!(bar.volume, dec!(125400));
    }

    #[test]
    fn chart_period_gubn_code() {
        assert_eq!(ChartPeriod::Day.as_gubn(),   "0");
        assert_eq!(ChartPeriod::Week.as_gubn(),  "1");
        assert_eq!(ChartPeriod::Month.as_gubn(), "2");
    }
}
```

```bash
cargo test -p kis_api rest::overseas::quote::chart::tests 2>&1 | head -20
```

Expected: 컴파일 에러 (미구현)

### Step 3-3: 구현 작성

```rust
// crates/kis_api/src/rest/overseas/quote/chart.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::config::KisConfig;
use crate::error::KisError;
use crate::rest::http::execute;
use crate::rest::overseas::types::Exchange;

// ── 차트 주기 ──────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartPeriod {
    /// 일봉 (GUBN = "0")
    Day,
    /// 주봉 (GUBN = "1")
    Week,
    /// 월봉 (GUBN = "2")
    Month,
}

impl ChartPeriod {
    pub fn as_gubn(&self) -> &'static str {
        match self {
            ChartPeriod::Day   => "0",
            ChartPeriod::Week  => "1",
            ChartPeriod::Month => "2",
        }
    }
}

// ── 일/주/월봉 요청 ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct DailyChartRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub period: ChartPeriod,
    /// 기준일 YYYYMMDD (빈 문자열이면 오늘)
    pub base_date: String,
    /// true = 수정주가 (MODP = "1"), false = 미수정 (MODP = "0")
    pub adjusted: bool,
}

// ── 분봉 요청 ─────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct MinuteChartRequest {
    pub symbol: String,
    pub exchange: Exchange,
    /// 분봉 단위 — 1, 5, 15 중 하나
    pub interval_min: u32,
}

// ── KIS raw 역직렬화 타입 (일봉) ──────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisCandleRaw {
    #[serde(rename = "xymd")]
    pub date: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub open: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub high: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub low: Decimal,
    #[serde(rename = "clos", with = "rust_decimal::serde::str")]
    pub close: Decimal,
    #[serde(rename = "vol", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub diff: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub rate: Decimal,
}

/// 일/주/월봉 캔들
#[derive(Debug, Clone)]
pub struct CandleBar {
    /// 날짜 YYYYMMDD (XYMD)
    pub date: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    /// 전일대비 (DIFF)
    pub diff: Decimal,
    /// 등락률 % (RATE)
    pub rate: Decimal,
}

impl From<KisCandleRaw> for CandleBar {
    fn from(r: KisCandleRaw) -> Self {
        Self {
            date: r.date,
            open: r.open,
            high: r.high,
            low: r.low,
            close: r.close,
            volume: r.volume,
            diff: r.diff,
            rate: r.rate,
        }
    }
}

// ── KIS raw 역직렬화 타입 (분봉) ──────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisMinuteRaw {
    #[serde(rename = "kymd")]
    pub date: String,
    #[serde(rename = "khms")]
    pub time: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub open: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub high: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub low: Decimal,
    #[serde(rename = "last", with = "rust_decimal::serde::str")]
    pub close: Decimal,
    #[serde(rename = "evol", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
}

/// 분봉 캔들
#[derive(Debug, Clone)]
pub struct MinuteBar {
    /// 날짜 YYYYMMDD (KYMD)
    pub date: String,
    /// 시간 HHMMSS (KHMS)
    pub time: String,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

impl From<KisMinuteRaw> for MinuteBar {
    fn from(r: KisMinuteRaw) -> Self {
        Self {
            date: r.date,
            time: r.time,
            open: r.open,
            high: r.high,
            low: r.low,
            close: r.close,
            volume: r.volume,
        }
    }
}

// ── API 래퍼 응답 타입 ────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
struct DailyChartApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output2: Vec<KisCandleRaw>,
}

#[derive(Debug, Deserialize)]
struct MinuteChartApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output2: Vec<KisMinuteRaw>,
}

/// 해외주식 일/주/월봉 차트 조회
///
/// GET /uapi/overseas-price/v1/quotations/dailyprice  (TR: HHDFS76240000)
pub async fn daily_chart(
    config: &KisConfig,
    token: &str,
    req: DailyChartRequest,
) -> Result<Vec<CandleBar>, KisError> {
    let modp = if req.adjusted { "1" } else { "0" };

    let params = [
        ("AUTH", ""),
        ("EXCD", req.exchange.as_code()),
        ("SYMB", req.symbol.as_str()),
        ("GUBN", req.period.as_gubn()),
        ("BYMD", req.base_date.as_str()),
        ("MODP", modp),
        ("KEYB", ""),
    ];

    let resp: DailyChartApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/dailyprice",
        "HHDFS76240000",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(resp.output2.into_iter().map(CandleBar::from).collect())
}

/// 해외주식 분봉 차트 조회 (최대 120개)
///
/// GET /uapi/overseas-price/v1/quotations/inquire-time-itemchartprice  (TR: HHDFS76950200)
pub async fn minute_chart(
    config: &KisConfig,
    token: &str,
    req: MinuteChartRequest,
) -> Result<Vec<MinuteBar>, KisError> {
    let nmin = req.interval_min.to_string();

    let params = [
        ("AUTH", ""),
        ("EXCD", req.exchange.as_code()),
        ("SYMB", req.symbol.as_str()),
        ("NMIN", nmin.as_str()),
        ("PINC", "1"),
        ("NEXT", ""),
        ("NREC", "120"),
        ("FILL", ""),
        ("KEYB", ""),
    ];

    let resp: MinuteChartApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/inquire-time-itemchartprice",
        "HHDFS76950200",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(resp.output2.into_iter().map(MinuteBar::from).collect())
}

#[cfg(test)]
mod tests {
    // ... (위 Step 3-2 테스트 코드)
}
```

### Step 3-4: 테스트 통과 확인

```bash
cargo test -p kis_api rest::overseas::quote::chart::tests
```

Expected:
```
test rest::overseas::quote::chart::tests::parse_daily_chart ... ok
test rest::overseas::quote::chart::tests::parse_minute_chart ... ok
test rest::overseas::quote::chart::tests::chart_period_gubn_code ... ok

test result: ok. 3 passed; 0 failed
```

- [ ] **Step 3-1: 두 픽스처 JSON 작성**
- [ ] **Step 3-2: 실패하는 테스트 작성 + 실패 확인**
- [ ] **Step 3-3: ChartPeriod, CandleBar, MinuteBar + 두 함수 구현**
- [ ] **Step 3-4: 테스트 통과 확인**
- [ ] **Step 3-5: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/quote/chart.rs \
        crates/kis_api/tests/fixtures/overseas/quote/daily_chart.json \
        crates/kis_api/tests/fixtures/overseas/quote/minute_chart.json
git commit -m "feat(kis_api): implement daily_chart() and minute_chart() for overseas stocks"
```

---

## Task 4: search() + symbol_info() — 종목 검색 및 정보

**Endpoints:**
- `GET /uapi/overseas-price/v1/quotations/inquire-search` (TR: `HHDFS76410000`)
- `GET /uapi/overseas-price/v1/quotations/search-info` (TR: `HHDFS76200200`)

**Files:**
- Write: `crates/kis_api/src/rest/overseas/quote/search.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/search.json`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/symbol_info.json`

### Step 4-1: 픽스처 JSON 작성

`crates/kis_api/tests/fixtures/overseas/quote/search.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "excd": "NAS",
      "symb": "AAPL",
      "dnam": "애플",
      "last": "181.2300",
      "diff": "1.5700",
      "rate": "0.87",
      "tvol": "62345100"
    },
    {
      "excd": "NAS",
      "symb": "AAON",
      "dnam": "AAON Inc",
      "last": "72.4500",
      "diff": "-0.3200",
      "rate": "-0.44",
      "tvol": "412300"
    }
  ]
}
```

`crates/kis_api/tests/fixtures/overseas/quote/symbol_info.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": {
    "symb": "AAPL",
    "dnam": "애플",
    "ordy": "Y",
    "mktc": "2800000000000",
    "perx": "29.45",
    "pbrx": "48.23",
    "epsx": "6.14"
  }
}
```

### Step 4-2: 실패하는 테스트 작성

```rust
// crates/kis_api/src/rest/overseas/quote/search.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_search() -> serde_json::Value {
        let json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/search.json"
        ))
        .expect("fixture not found");
        serde_json::from_str(&json).expect("invalid json")
    }

    fn load_symbol_info() -> serde_json::Value {
        let json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/symbol_info.json"
        ))
        .expect("fixture not found");
        serde_json::from_str(&json).expect("invalid json")
    }

    #[test]
    fn parse_search_results() {
        let raw: Vec<KisSearchResultRaw> =
            serde_json::from_value(load_search()["output"].clone())
                .expect("deserialize failed");

        assert_eq!(raw.len(), 2);
        let result = SearchResult::from(raw[0].clone());

        assert_eq!(result.exchange, "NAS");
        assert_eq!(result.symbol,   "AAPL");
        assert_eq!(result.name,     "애플");
        assert_eq!(result.last,  dec!(181.2300));
        assert_eq!(result.diff,  dec!(1.5700));
        assert_eq!(result.rate,  dec!(0.87));
        assert_eq!(result.volume, dec!(62345100));
    }

    #[test]
    fn parse_symbol_info() {
        let raw: KisSymbolInfoRaw =
            serde_json::from_value(load_symbol_info()["output"].clone())
                .expect("deserialize failed");

        let info = SymbolInfo::from(raw);

        assert_eq!(info.symbol, "AAPL");
        assert_eq!(info.name,   "애플");
        assert!(info.orderable);
        assert_eq!(info.market_cap, dec!(2800000000000));
        assert_eq!(info.per, dec!(29.45));
        assert_eq!(info.pbr, dec!(48.23));
        assert_eq!(info.eps, dec!(6.14));
    }
}
```

```bash
cargo test -p kis_api rest::overseas::quote::search::tests 2>&1 | head -20
```

Expected: 컴파일 에러 (미구현)

### Step 4-3: 구현 작성

```rust
// crates/kis_api/src/rest/overseas/quote/search.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::config::KisConfig;
use crate::error::KisError;
use crate::rest::http::execute;
use crate::rest::overseas::types::Exchange;

// ── search() 타입 ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisSearchResultRaw {
    #[serde(rename = "excd")]
    pub exchange: String,
    #[serde(rename = "symb")]
    pub symbol: String,
    #[serde(rename = "dnam")]
    pub name: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub last: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub diff: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub rate: Decimal,
    #[serde(rename = "tvol", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
}

/// 종목 검색 결과 단건
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 거래소 코드 (EXCD)
    pub exchange: String,
    /// 종목 코드 (SYMB)
    pub symbol: String,
    /// 종목명 (DNAM)
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
}

impl From<KisSearchResultRaw> for SearchResult {
    fn from(r: KisSearchResultRaw) -> Self {
        Self {
            exchange: r.exchange,
            symbol: r.symbol,
            name: r.name,
            last: r.last,
            diff: r.diff,
            rate: r.rate,
            volume: r.volume,
        }
    }
}

// ── symbol_info() 타입 ────────────────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisSymbolInfoRaw {
    #[serde(rename = "symb")]
    pub symbol: String,
    #[serde(rename = "dnam")]
    pub name: String,
    #[serde(rename = "ordy")]
    pub ordy: String,
    #[serde(rename = "mktc", with = "rust_decimal::serde::str")]
    pub market_cap: Decimal,
    #[serde(rename = "perx", with = "rust_decimal::serde::str")]
    pub per: Decimal,
    #[serde(rename = "pbrx", with = "rust_decimal::serde::str")]
    pub pbr: Decimal,
    #[serde(rename = "epsx", with = "rust_decimal::serde::str")]
    pub eps: Decimal,
}

/// 종목 기본 정보 (시가총액, PER, PBR, EPS)
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub symbol: String,
    pub name: String,
    /// 주문가능여부 (ORDY == "Y")
    pub orderable: bool,
    /// 시가총액 (MKTC)
    pub market_cap: Decimal,
    /// PER (PERX)
    pub per: Decimal,
    /// PBR (PBRX)
    pub pbr: Decimal,
    /// EPS (EPSX)
    pub eps: Decimal,
}

impl From<KisSymbolInfoRaw> for SymbolInfo {
    fn from(r: KisSymbolInfoRaw) -> Self {
        Self {
            symbol: r.symbol,
            name: r.name,
            orderable: r.ordy == "Y",
            market_cap: r.market_cap,
            per: r.per,
            pbr: r.pbr,
            eps: r.eps,
        }
    }
}

// ── API 래퍼 응답 타입 ────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
struct SearchApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<KisSearchResultRaw>,
}

#[derive(Debug, Deserialize)]
struct SymbolInfoApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: KisSymbolInfoRaw,
}

/// 해외주식 종목 검색
///
/// GET /uapi/overseas-price/v1/quotations/inquire-search  (TR: HHDFS76410000)
pub async fn search(
    config: &KisConfig,
    token: &str,
    keyword: &str,
) -> Result<Vec<SearchResult>, KisError> {
    let params = [
        ("AUTH", ""),
        ("SYMB", keyword),
        ("BYMD", ""),
        ("COMBI", "0"),
    ];

    let resp: SearchApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/inquire-search",
        "HHDFS76410000",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(resp.output.into_iter().map(SearchResult::from).collect())
}

/// 해외주식 종목 기본정보 조회
///
/// GET /uapi/overseas-price/v1/quotations/search-info  (TR: HHDFS76200200)
pub async fn symbol_info(
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<SymbolInfo, KisError> {
    let params = [
        ("AUTH", ""),
        ("EXCD", exchange.as_code()),
        ("SYMB", symbol),
    ];

    let resp: SymbolInfoApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/search-info",
        "HHDFS76200200",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(SymbolInfo::from(resp.output))
}

#[cfg(test)]
mod tests {
    // ... (위 Step 4-2 테스트 코드)
}
```

### Step 4-4: 테스트 통과 확인

```bash
cargo test -p kis_api rest::overseas::quote::search::tests
```

Expected:
```
test rest::overseas::quote::search::tests::parse_search_results ... ok
test rest::overseas::quote::search::tests::parse_symbol_info ... ok

test result: ok. 2 passed; 0 failed
```

- [ ] **Step 4-1: 두 픽스처 JSON 작성**
- [ ] **Step 4-2: 실패하는 테스트 작성 + 실패 확인**
- [ ] **Step 4-3: SearchResult, SymbolInfo + 두 함수 구현**
- [ ] **Step 4-4: 테스트 통과 확인**
- [ ] **Step 4-5: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/quote/search.rs \
        crates/kis_api/tests/fixtures/overseas/quote/search.json \
        crates/kis_api/tests/fixtures/overseas/quote/symbol_info.json
git commit -m "feat(kis_api): implement search() and symbol_info() for overseas stocks"
```

---

## Task 5: news() + dividend() + holidays() — 기업정보

**Endpoints:**
- `GET /uapi/overseas-price/v1/quotations/news-title` (TR: `HHDFS76320000`)
- `GET /uapi/overseas-price/v1/quotations/period-rights` (TR: `HHDFS76520000`)
- `GET /uapi/overseas-price/v1/quotations/countries-holiday` (TR: `HHDFS76270000`)

> 참고: 속보(`brknews-title`, TR: `HHDFS76330000`)는 이 태스크에 포함하지 않는다. 종목 코드 없이 전체 뉴스를 조회하는 독립적 기능이므로 별도 확장 포인트로 남긴다. `news()`는 종목 기준 뉴스(`news-title`)만 구현한다.

**Files:**
- Write: `crates/kis_api/src/rest/overseas/quote/corporate.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/news.json`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/dividend.json`
- Create: `crates/kis_api/tests/fixtures/overseas/quote/holidays.json`

### Step 5-1: 픽스처 JSON 작성

`crates/kis_api/tests/fixtures/overseas/quote/news.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "nwsn": "20260321001234",
      "tmsf": "143022",
      "tlfe": "Apple reports record Q1 earnings, beats estimates"
    },
    {
      "nwsn": "20260321000987",
      "tmsf": "091500",
      "tlfe": "AAPL stock rises after positive analyst note"
    }
  ]
}
```

`crates/kis_api/tests/fixtures/overseas/quote/dividend.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "xddt": "20260215",
      "dvdn": "0.2400",
      "strt": "0.0000"
    },
    {
      "xddt": "20251115",
      "dvdn": "0.2400",
      "strt": "0.0000"
    },
    {
      "xddt": "20240601",
      "dvdn": "0.0000",
      "strt": "0.1000"
    }
  ]
}
```

`crates/kis_api/tests/fixtures/overseas/quote/holidays.json`:

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "bass_dt": "20260101",
      "holc": "1",
      "holc_name": "New Year's Day"
    },
    {
      "bass_dt": "20260119",
      "holc": "1",
      "holc_name": "Martin Luther King Jr. Day"
    }
  ]
}
```

### Step 5-2: 실패하는 테스트 작성

```rust
// crates/kis_api/src/rest/overseas/quote/corporate.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load(filename: &str) -> serde_json::Value {
        let path = format!(
            "{}/tests/fixtures/overseas/quote/{}",
            env!("CARGO_MANIFEST_DIR"),
            filename
        );
        let json = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("fixture not found: {}", path));
        serde_json::from_str(&json).expect("invalid json")
    }

    #[test]
    fn parse_news() {
        let raw: Vec<KisNewsRaw> =
            serde_json::from_value(load("news.json")["output"].clone())
                .expect("deserialize failed");

        assert_eq!(raw.len(), 2);
        let item = NewsItem::from(raw[0].clone());

        assert_eq!(item.id,    "20260321001234");
        assert_eq!(item.time,  "143022");
        assert_eq!(item.title, "Apple reports record Q1 earnings, beats estimates");
    }

    #[test]
    fn parse_dividends() {
        let raw: Vec<KisDividendRaw> =
            serde_json::from_value(load("dividend.json")["output"].clone())
                .expect("deserialize failed");

        assert_eq!(raw.len(), 3);
        let item = DividendItem::from(raw[0].clone());

        assert_eq!(item.ex_date,     "20260215");
        assert_eq!(item.dividend,    dec!(0.2400));
        assert_eq!(item.split_ratio, dec!(0.0000));

        let split = DividendItem::from(raw[2].clone());
        assert_eq!(split.split_ratio, dec!(0.1000));
    }

    #[test]
    fn parse_holidays() {
        let raw: Vec<KisHolidayRaw> =
            serde_json::from_value(load("holidays.json")["output"].clone())
                .expect("deserialize failed");

        assert_eq!(raw.len(), 2);
        let h = Holiday::from(raw[0].clone());

        assert_eq!(h.date, "20260101");
        assert_eq!(h.kind, "1");
        assert_eq!(h.name, "New Year's Day");
    }
}
```

```bash
cargo test -p kis_api rest::overseas::quote::corporate::tests 2>&1 | head -20
```

Expected: 컴파일 에러 (미구현)

### Step 5-3: 구현 작성

```rust
// crates/kis_api/src/rest/overseas/quote/corporate.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::config::KisConfig;
use crate::error::KisError;
use crate::rest::http::execute;
use crate::rest::overseas::types::Exchange;

// ── 뉴스 ──────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisNewsRaw {
    #[serde(rename = "nwsn")]
    pub id: String,
    #[serde(rename = "tmsf")]
    pub time: String,
    #[serde(rename = "tlfe")]
    pub title: String,
}

/// 뉴스 단건
#[derive(Debug, Clone)]
pub struct NewsItem {
    /// 뉴스 번호 (NWSN)
    pub id: String,
    /// 발생 시간 HHMMSS (TMSF)
    pub time: String,
    /// 제목 (TLFE)
    pub title: String,
}

impl From<KisNewsRaw> for NewsItem {
    fn from(r: KisNewsRaw) -> Self {
        Self { id: r.id, time: r.time, title: r.title }
    }
}

// ── 배당 ──────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisDividendRaw {
    #[serde(rename = "xddt")]
    pub ex_date: String,
    #[serde(rename = "dvdn", with = "rust_decimal::serde::str")]
    pub dividend: Decimal,
    #[serde(rename = "strt", with = "rust_decimal::serde::str")]
    pub split_ratio: Decimal,
}

/// 배당/스플릿 이력 단건
#[derive(Debug, Clone)]
pub struct DividendItem {
    /// 배당락일 YYYYMMDD (XDDT)
    pub ex_date: String,
    /// 배당금 (DVDN)
    pub dividend: Decimal,
    /// 스플릿 비율 (STRT, 0이면 배당, 양수면 스플릿)
    pub split_ratio: Decimal,
}

impl From<KisDividendRaw> for DividendItem {
    fn from(r: KisDividendRaw) -> Self {
        Self {
            ex_date: r.ex_date,
            dividend: r.dividend,
            split_ratio: r.split_ratio,
        }
    }
}

// ── 휴장일 ────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KisHolidayRaw {
    #[serde(rename = "bass_dt")]
    pub date: String,
    #[serde(rename = "holc")]
    pub kind: String,
    #[serde(rename = "holc_name")]
    pub name: String,
}

/// 거래소 휴장일 단건
#[derive(Debug, Clone)]
pub struct Holiday {
    /// 날짜 YYYYMMDD (BASS_DT)
    pub date: String,
    /// 휴장 구분 코드 (HOLC)
    pub kind: String,
    /// 휴장명 (HOLC_NAME)
    pub name: String,
}

impl From<KisHolidayRaw> for Holiday {
    fn from(r: KisHolidayRaw) -> Self {
        Self { date: r.date, kind: r.kind, name: r.name }
    }
}

// ── API 래퍼 응답 타입 ────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
struct NewsApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<KisNewsRaw>,
}

#[derive(Debug, Deserialize)]
struct DividendApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<KisDividendRaw>,
}

#[derive(Debug, Deserialize)]
struct HolidayApiResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<KisHolidayRaw>,
}

/// 종목 뉴스 조회
///
/// GET /uapi/overseas-price/v1/quotations/news-title  (TR: HHDFS76320000)
pub async fn news(
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<Vec<NewsItem>, KisError> {
    let params = [
        ("AUTH", ""),
        ("PDNO", symbol),
        ("EXCD", exchange.as_code()),
        ("PTON", "20"),
        ("NWSN", ""),
    ];

    let resp: NewsApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/news-title",
        "HHDFS76320000",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(resp.output.into_iter().map(NewsItem::from).collect())
}

/// 배당/스플릿 이력 조회
///
/// GET /uapi/overseas-price/v1/quotations/period-rights  (TR: HHDFS76520000)
pub async fn dividend(
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<DividendItem>, KisError> {
    let params = [
        ("AUTH", ""),
        ("EXCD", exchange.as_code()),
        ("SYMB", symbol),
        ("STDT", start_date),
        ("ENDT", end_date),
    ];

    let resp: DividendApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/period-rights",
        "HHDFS76520000",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(resp.output.into_iter().map(DividendItem::from).collect())
}

/// 거래소 휴장일 조회
///
/// GET /uapi/overseas-price/v1/quotations/countries-holiday  (TR: HHDFS76270000)
pub async fn holidays(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    year: u32,
) -> Result<Vec<Holiday>, KisError> {
    let yrid = year.to_string();

    let params = [
        ("AUTH", ""),
        ("EXCD", exchange.as_code()),
        ("YRID", yrid.as_str()),
    ];

    let resp: HolidayApiResponse = execute(
        config,
        token,
        "GET",
        "/uapi/overseas-price/v1/quotations/countries-holiday",
        "HHDFS76270000",
        &params,
        None,
    )
    .await?;

    if resp.rt_cd != "0" {
        return Err(KisError::Api {
            code: resp.msg_cd,
            message: resp.msg1,
        });
    }

    Ok(resp.output.into_iter().map(Holiday::from).collect())
}

#[cfg(test)]
mod tests {
    // ... (위 Step 5-2 테스트 코드)
}
```

### Step 5-4: 테스트 통과 확인

```bash
cargo test -p kis_api rest::overseas::quote::corporate::tests
```

Expected:
```
test rest::overseas::quote::corporate::tests::parse_news ... ok
test rest::overseas::quote::corporate::tests::parse_dividends ... ok
test rest::overseas::quote::corporate::tests::parse_holidays ... ok

test result: ok. 3 passed; 0 failed
```

- [ ] **Step 5-1: 세 픽스처 JSON 작성**
- [ ] **Step 5-2: 실패하는 테스트 작성 + 실패 확인**
- [ ] **Step 5-3: NewsItem, DividendItem, Holiday + 세 함수 구현**
- [ ] **Step 5-4: 테스트 통과 확인**
- [ ] **Step 5-5: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/quote/corporate.rs \
        crates/kis_api/tests/fixtures/overseas/quote/news.json \
        crates/kis_api/tests/fixtures/overseas/quote/dividend.json \
        crates/kis_api/tests/fixtures/overseas/quote/holidays.json
git commit -m "feat(kis_api): implement news(), dividend(), holidays() for overseas stocks"
```

---

## Task 6: KisClient 메서드 추가 + lib.rs pub 정리

**Files:**
- Modify: `crates/kis_api/src/client.rs`
- Modify: `crates/kis_api/src/lib.rs`
- Create: `crates/kis_api/tests/integration_quote.rs`

### Step 6-1: client.rs에 시세 메서드 추가

`client.rs`의 `impl KisClient` 블록에 아래 메서드를 추가한다. Plan 2의 `current_price`를 `price`로 대체(또는 `price`를 추가하고 `current_price`는 `#[deprecated]` 표시):

```rust
// crates/kis_api/src/client.rs — impl KisClient에 추가

use crate::rest::overseas::quote::{
    price::{PriceResponse, price as _price},
    orderbook::{OrderbookResponse, orderbook as _orderbook},
    chart::{
        CandleBar, MinuteBar, DailyChartRequest, MinuteChartRequest,
        daily_chart as _daily_chart, minute_chart as _minute_chart,
    },
    search::{
        SearchResult, SymbolInfo,
        search as _search, symbol_info as _symbol_info,
    },
    corporate::{
        NewsItem, DividendItem, Holiday,
        news as _news, dividend as _dividend, holidays as _holidays,
    },
};

impl KisClient {
    // ── 현재가 ────────────────────────────────────────────────────────────────
    /// 해외주식 현재가 조회
    pub async fn price(
        &self,
        symbol: &str,
        exchange: &Exchange,
    ) -> Result<PriceResponse, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _price(&self.inner.config, &token, symbol, exchange).await
    }

    // ── 호가 ──────────────────────────────────────────────────────────────────
    /// 해외주식 호가 조회 (5단계)
    pub async fn orderbook(
        &self,
        symbol: &str,
        exchange: &Exchange,
    ) -> Result<OrderbookResponse, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _orderbook(&self.inner.config, &token, symbol, exchange).await
    }

    // ── 차트 ──────────────────────────────────────────────────────────────────
    /// 해외주식 일/주/월봉 차트 조회
    pub async fn daily_chart(
        &self,
        req: DailyChartRequest,
    ) -> Result<Vec<CandleBar>, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _daily_chart(&self.inner.config, &token, req).await
    }

    /// 해외주식 분봉 차트 조회 (최대 120개)
    pub async fn minute_chart(
        &self,
        req: MinuteChartRequest,
    ) -> Result<Vec<MinuteBar>, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _minute_chart(&self.inner.config, &token, req).await
    }

    // ── 검색 ──────────────────────────────────────────────────────────────────
    /// 해외주식 종목 검색 (키워드)
    pub async fn search(
        &self,
        keyword: &str,
    ) -> Result<Vec<SearchResult>, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _search(&self.inner.config, &token, keyword).await
    }

    /// 해외주식 종목 기본정보 조회
    pub async fn symbol_info(
        &self,
        symbol: &str,
        exchange: &Exchange,
    ) -> Result<SymbolInfo, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _symbol_info(&self.inner.config, &token, symbol, exchange).await
    }

    // ── 기업정보 ──────────────────────────────────────────────────────────────
    /// 종목 뉴스 조회
    pub async fn news(
        &self,
        symbol: &str,
        exchange: &Exchange,
    ) -> Result<Vec<NewsItem>, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _news(&self.inner.config, &token, symbol, exchange).await
    }

    /// 배당/스플릿 이력 조회
    pub async fn dividend(
        &self,
        symbol: &str,
        exchange: &Exchange,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<DividendItem>, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _dividend(&self.inner.config, &token, symbol, exchange, start_date, end_date).await
    }

    /// 거래소 휴장일 조회
    pub async fn holidays(
        &self,
        exchange: &Exchange,
        year: u32,
    ) -> Result<Vec<Holiday>, KisError> {
        let token = self.inner.token_manager.get_token().await?;
        _holidays(&self.inner.config, &token, exchange, year).await
    }
}
```

### Step 6-2: lib.rs pub 정리

```rust
// crates/kis_api/src/lib.rs — 기존 pub use에 추가

// 시세 타입 re-export
pub use rest::overseas::quote::price::PriceResponse;
pub use rest::overseas::quote::orderbook::{OrderbookLevel, OrderbookResponse};
pub use rest::overseas::quote::chart::{
    ChartPeriod, DailyChartRequest, MinuteChartRequest, CandleBar, MinuteBar,
};
pub use rest::overseas::quote::search::{SearchResult, SymbolInfo};
pub use rest::overseas::quote::corporate::{NewsItem, DividendItem, Holiday};
```

### Step 6-3: 통합 테스트 작성 (VTS-safe, read-only)

`crates/kis_api/tests/integration_quote.rs`:

```rust
//! 해외주식 시세 통합 테스트
//!
//! VTS(모의투자) 환경에서 실행되는 read-only 테스트.
//! 실제 네트워크 요청 발생. CI 환경에서는 SKIP_INTEGRATION=1로 비활성화.
//!
//! 실행:
//!   KIS_APP_KEY=... KIS_APP_SECRET=... KIS_ACCOUNT=... cargo test --test integration_quote

use kis_api::{KisConfig, KisClient, KisError};
use kis_api::rest::overseas::types::Exchange;

fn make_client() -> Option<KisClient> {
    let key     = std::env::var("KIS_APP_KEY").ok()?;
    let secret  = std::env::var("KIS_APP_SECRET").ok()?;
    let account = std::env::var("KIS_ACCOUNT").ok()?;

    if std::env::var("SKIP_INTEGRATION").is_ok() {
        return None;
    }

    let config = KisConfig::builder()
        .app_key(key)
        .app_secret(secret)
        .account(account)
        .mock(true)  // VTS 모의투자
        .build()
        .ok()?;

    Some(KisClient::new(config))
}

#[tokio::test]
async fn test_price_aapl() {
    let Some(client) = make_client() else {
        eprintln!("Skipping: KIS credentials not set");
        return;
    };

    let result = client.price("AAPL", &Exchange::NASD).await;
    match result {
        Ok(p) => {
            assert!(p.last > rust_decimal::Decimal::ZERO, "last price must be positive");
            println!("AAPL price: {}", p.last);
        }
        Err(KisError::Api { code, message }) => {
            // VTS에서 해외주식 시세 미지원 시 API 에러 코드 반환 — 테스트 통과 처리
            eprintln!("API error (VTS limitation): {} — {}", code, message);
        }
        Err(e) => panic!("unexpected error: {e}"),
    }
}

#[tokio::test]
async fn test_search_apple() {
    let Some(client) = make_client() else {
        eprintln!("Skipping: KIS credentials not set");
        return;
    };

    let result = client.search("AAPL").await;
    match result {
        Ok(results) => {
            println!("search 'AAPL': {} results", results.len());
            if !results.is_empty() {
                println!("first: {} — {}", results[0].symbol, results[0].name);
            }
        }
        Err(KisError::Api { code, message }) => {
            eprintln!("API error (VTS limitation): {} — {}", code, message);
        }
        Err(e) => panic!("unexpected error: {e}"),
    }
}
```

### Step 6-4: 컴파일 + 단위 테스트 전체 통과

```bash
cargo check -p kis_api
cargo test -p kis_api
```

Expected:
```
running N tests
test rest::overseas::quote::price::tests::parse_price_response ... ok
test rest::overseas::quote::price::tests::exchange_price_codes ... ok
test rest::overseas::quote::orderbook::tests::parse_orderbook_response ... ok
test rest::overseas::quote::chart::tests::parse_daily_chart ... ok
test rest::overseas::quote::chart::tests::parse_minute_chart ... ok
test rest::overseas::quote::chart::tests::chart_period_gubn_code ... ok
test rest::overseas::quote::search::tests::parse_search_results ... ok
test rest::overseas::quote::search::tests::parse_symbol_info ... ok
test rest::overseas::quote::corporate::tests::parse_news ... ok
test rest::overseas::quote::corporate::tests::parse_dividends ... ok
test rest::overseas::quote::corporate::tests::parse_holidays ... ok
(+ Plan 2 및 Plan 3a 기존 테스트들)

test result: ok. N passed; 0 failed
```

### Step 6-5: 통합 테스트 실행 (선택, 실제 자격증명 필요)

```bash
KIS_APP_KEY=... KIS_APP_SECRET=... KIS_ACCOUNT=... \
  cargo test --test integration_quote -- --nocapture
```

- [ ] **Step 6-1: client.rs에 9개 시세 메서드 추가**
- [ ] **Step 6-2: lib.rs pub use 추가**
- [ ] **Step 6-3: integration_quote.rs 작성 (VTS-safe)**
- [ ] **Step 6-4: `cargo test -p kis_api` 전체 통과**
- [ ] **Step 6-5: 통합 테스트 실행 (선택)**
- [ ] **Step 6-6: 최종 커밋**

```bash
git add crates/kis_api/src/client.rs \
        crates/kis_api/src/lib.rs \
        crates/kis_api/tests/integration_quote.rs
git commit -m "feat(kis_api): add KisClient methods for overseas quote (price, orderbook, chart, search, news, dividend, holidays)"
```

---

## WebSocket 이벤트 타입 참조 (Plan 2 KisStream 문서화)

Plan 2의 `KisStream`이 이미 WebSocket 구독을 구현하고 있다. Plan 3b는 신규 WS 구현 없이 아래 이벤트 매핑만 문서화한다.

| TR ID        | 이벤트                  | KisEvent variant       | 설명              |
|--------------|------------------------|------------------------|-------------------|
| `HDFSCNT0`   | 실시간 해외주식 체결     | `KisEvent::Transaction` | 체결가, 체결량    |
| `HDFSASP0`   | 실시간 해외주식 호가 (미국) | `KisEvent::Quote`    | 매수/매도 호가    |
| `HDFSASP1`   | 실시간 해외주식 호가 (아시아) | `KisEvent::Quote`  | 매수/매도 호가    |
| `H0GSCNI0`   | 주문 체결 통보 (실전)    | `KisEvent::OrderConfirm` | 주문ID, 체결가  |
| `H0GSCNI9`   | 주문 체결 통보 (VTS)     | `KisEvent::OrderConfirm` | 주문ID, 체결가  |

구독 방법 (Plan 2 KisStream 사용):

```rust
let stream = client.stream().await?;
let mut receiver = stream.receiver();

// 실시간 체결 구독
stream.subscribe("AAPL", SubscriptionKind::Price).await?;

// 이벤트 수신
while let Ok(event) = receiver.recv().await {
    match event {
        KisEvent::Transaction(tx) => println!("체결: {} @ {}", tx.symbol, tx.price),
        KisEvent::Quote(q) => println!("호가: bid={}, ask={}", q.bid_price, q.ask_price),
        _ => {}
    }
}
```

---

## 전체 태스크 요약

| 태스크 | 내용 | 파일 | 테스트 수 |
|--------|------|------|-----------|
| Task 0 | 디렉터리 재구조화 (price.rs → quote/) | 3파일 수정 | — |
| Task 1 | price() — 현재가 | price.rs + 픽스처 | 2 |
| Task 2 | orderbook() — 호가 5단계 | orderbook.rs + 픽스처 | 1 |
| Task 3 | daily_chart() + minute_chart() | chart.rs + 픽스처 2개 | 3 |
| Task 4 | search() + symbol_info() | search.rs + 픽스처 2개 | 2 |
| Task 5 | news() + dividend() + holidays() | corporate.rs + 픽스처 3개 | 3 |
| Task 6 | KisClient 메서드 + lib.rs + 통합 테스트 | client.rs, lib.rs, tests/ | 2 (통합) |

**총 단위 테스트:** 11개
**총 통합 테스트:** 2개 (VTS-safe, credentials 필요 시만 실행)
**총 픽스처 파일:** 9개
**총 API 엔드포인트:** 11개 (현재가, 호가, 일봉, 분봉, 검색, 종목정보, 뉴스, 배당, 휴장일 + WS 이벤트 문서화)

---

## 의존 관계

```
Plan 1 (workspace)
  └── Plan 2 (KisConfig, KisError, KisClient, KisStream, rest::http::execute)
        └── Plan 3a (Exchange, OrderSide, OrderType, SubscriptionKind in types.rs)
              └── Plan 3b (this plan — overseas quote endpoints)
                    └── Plan 3c (해외주식 시세분석)
```

Plan 3b는 Plan 3a의 `Exchange`를 재사용하며 신규 공유 타입을 추가하지 않는다.
`Exchange::as_code()` 메서드가 Plan 3a에 없으면 `price.rs`의 `exchange_to_price_code()`를 참고해 `types.rs`에 추가한다.
