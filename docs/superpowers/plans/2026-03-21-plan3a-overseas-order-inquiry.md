# Plan 3a: 해외주식 주문/계좌 REST API

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `crates/kis_api`에 해외주식 주문/계좌 REST API 전체를 구현한다 — `place_order`, `cancel_order`, `balance`, `unfilled_orders`, `order_history`, `period_profit`, `buyable_amount` 7개 메서드를 `KisClient`에 추가하고, 공통 타입(`Exchange`, `OrderSide`, `OrderType`, `SubscriptionKind`)을 라이브러리 레벨로 노출한다.

**Architecture:** Plan 2가 확립한 `Arc<Inner>` 패턴(`KisConfig` + `TokenManager`) 위에 기능을 추가한다. 각 엔드포인트는 독립된 파일에 자유함수(`pub async fn`)로 구현하고, `KisClient` 메서드가 이를 위임 호출한다. `serde_json::Value` 반환 금지 — 모든 응답은 강타입 구조체. 금액/수량은 `rust_decimal::Decimal`, 날짜는 `chrono::NaiveDate` / `chrono::DateTime<FixedOffset>` 사용.

**Tech Stack:** Rust 2021, `tokio`, `reqwest`, `serde_json`, `rust_decimal`, `chrono`, `thiserror` (Plan 2 의존성 그대로 사용)

**선행 조건:** Plan 2 완료 — `KisConfig`, `KisError`, `TokenManager`, `KisClient`, `rest::http::execute` 존재

**태스크 순서 원칙:** 각 태스크는 독립적으로 컴파일 가능하고 테스트 가능한 상태로 완료된다. TDD 순서: 픽스처 작성 → 테스트 작성 → `cargo test` 실패 확인 → 구현 → `cargo test` 통과 확인 → 커밋.

---

## File Map

```
crates/kis_api/src/rest/overseas/
├── mod.rs              UPDATE — order, inquiry 서브모듈 선언 추가
├── types.rs            CREATE — Exchange, OrderSide, OrderType, SubscriptionKind, TR ID 함수
├── deposit.rs          KEEP   — 기존 파일 (Plan 2)
├── price.rs            KEEP   — 기존 파일 (Plan 2)
├── order/
│   ├── mod.rs          CREATE — place, cancel 모듈 선언
│   ├── place.rs        CREATE — place_order()
│   └── cancel.rs       CREATE — cancel_order()
└── inquiry/
    ├── mod.rs          CREATE — balance, orders, profit 모듈 선언
    ├── balance.rs      CREATE — balance()
    ├── orders.rs       CREATE — unfilled_orders(), order_history()
    └── profit.rs       CREATE — period_profit(), buyable_amount()

crates/kis_api/src/
├── client.rs           UPDATE — 7개 pub async fn 메서드 추가
└── lib.rs              UPDATE — 신규 pub use 추가

crates/kis_api/tests/
├── fixtures/
│   └── overseas/
│       ├── order/
│       │   ├── place_order.json    CREATE
│       │   └── cancel_order.json   CREATE
│       └── inquiry/
│           ├── balance.json              CREATE
│           ├── unfilled_orders.json      CREATE
│           ├── order_history.json        CREATE
│           ├── period_profit.json        CREATE
│           └── buyable_amount.json       CREATE
└── integration/
    ├── mod.rs          CREATE — skip_unless_integration(), vts_client()
    └── order.rs        CREATE — 통합 테스트 (balance, unfilled_orders)
```

---

## Task 0: 공통 타입 정의 (`types.rs`)

**Files:**
- Create: `crates/kis_api/src/rest/overseas/types.rs`
- Update: `crates/kis_api/src/rest/overseas/mod.rs`
- Update: `crates/kis_api/src/lib.rs`

---

- [ ] **Step 1: `types.rs` 파일 생성 — enum 및 TR ID 함수 작성**

파일: `crates/kis_api/src/rest/overseas/types.rs`

```rust
//! 해외주식 공통 타입 — Exchange, OrderSide, OrderType, SubscriptionKind
//! 및 KIS TR ID 매핑 함수.

use std::fmt;

/// 해외 거래소 코드
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exchange {
    /// 나스닥
    NASD,
    /// 뉴욕증권거래소
    NYSE,
    /// 아멕스
    AMEX,
    /// 홍콩
    SEHK,
    /// 상해
    SHAA,
    /// 심천
    SZAA,
    /// 도쿄
    TKSE,
    /// 하노이
    HASE,
    /// 호치민
    VNSE,
    /// 미지원 거래소 (확장 대응)
    Other(String),
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Exchange::NASD => "NASD",
            Exchange::NYSE => "NYSE",
            Exchange::AMEX => "AMEX",
            Exchange::SEHK => "SEHK",
            Exchange::SHAA => "SHAA",
            Exchange::SZAA => "SZAA",
            Exchange::TKSE => "TKSE",
            Exchange::HASE => "HASE",
            Exchange::VNSE => "VNSE",
            Exchange::Other(s) => s.as_str(),
        };
        write!(f, "{}", s)
    }
}

impl From<&str> for Exchange {
    fn from(s: &str) -> Self {
        match s {
            "NASD" => Exchange::NASD,
            "NYSE" => Exchange::NYSE,
            "AMEX" => Exchange::AMEX,
            "SEHK" => Exchange::SEHK,
            "SHAA" => Exchange::SHAA,
            "SZAA" => Exchange::SZAA,
            "TKSE" => Exchange::TKSE,
            "HASE" => Exchange::HASE,
            "VNSE" => Exchange::VNSE,
            other  => Exchange::Other(other.to_string()),
        }
    }
}

/// 매수/매도 구분
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy  => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
        }
    }
}

/// 주문 유형
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    /// 시장가 ("01")
    Market,
    /// 지정가 ("00")
    Limit,
    /// 조건부지정가 ("32" — US LOC)
    LimitAon,
    /// 장마감시장가 ("34" — US MOC)
    MarketClose,
    /// 미래 KIS 추가 코드 대응
    Other(String),
}

impl OrderType {
    /// KIS API ORD_DVSN 코드 반환
    pub fn to_ord_dvsn(&self) -> &str {
        match self {
            OrderType::Market      => "01",
            OrderType::Limit       => "00",
            OrderType::LimitAon    => "32",
            OrderType::MarketClose => "34",
            OrderType::Other(s)    => s.as_str(),
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ord_dvsn())
    }
}

/// WebSocket 실시간 구독 종류
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionKind {
    /// 실시간 체결가
    Price,
    /// 실시간 호가
    Orderbook,
}

// ─── TR ID 매핑 ──────────────────────────────────────────────────────────────

/// 주문 TR ID 반환.
///
/// | Exchange          | Buy real     | Buy VTS      | Sell real    | Sell VTS     |
/// |-------------------|--------------|--------------|--------------|--------------|
/// | NASD / NYSE / AMEX| TTTT1002U    | VTTT1002U    | TTTT1006U    | VTTT1006U    |
/// | SEHK              | TTTS1002U    | VTTS1002U    | TTTS1001U    | VTTS1001U    |
/// | SHAA              | TTTS0202U    | VTTS0202U    | TTTS1005U    | VTTS1005U    |
/// | SZAA              | TTTS0305U    | VTTS0305U    | TTTS0304U    | VTTS0304U    |
/// | TKSE              | TTTS0308U    | VTTS0308U    | TTTS0307U    | VTTS0307U    |
/// | HASE / VNSE       | TTTS0311U    | VTTS0311U    | TTTS0310U    | VTTS0310U    |
///
/// `Exchange::Other(_)`는 `""` 반환 — 호출 측에서 `KisError::Api`로 변환할 것.
pub fn order_tr_id(exchange: &Exchange, side: &OrderSide, mock: bool) -> &'static str {
    match (exchange, side, mock) {
        // ── 미국 ──────────────────────────────────────────────────────────
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Buy,  false) => "TTTT1002U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Buy,  true)  => "VTTT1002U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Sell, false) => "TTTT1006U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Sell, true)  => "VTTT1006U",
        // ── 홍콩 ──────────────────────────────────────────────────────────
        (Exchange::SEHK, OrderSide::Buy,  false) => "TTTS1002U",
        (Exchange::SEHK, OrderSide::Buy,  true)  => "VTTS1002U",
        (Exchange::SEHK, OrderSide::Sell, false) => "TTTS1001U",
        (Exchange::SEHK, OrderSide::Sell, true)  => "VTTS1001U",
        // ── 상해 ──────────────────────────────────────────────────────────
        (Exchange::SHAA, OrderSide::Buy,  false) => "TTTS0202U",
        (Exchange::SHAA, OrderSide::Buy,  true)  => "VTTS0202U",
        (Exchange::SHAA, OrderSide::Sell, false) => "TTTS1005U",
        (Exchange::SHAA, OrderSide::Sell, true)  => "VTTS1005U",
        // ── 심천 ──────────────────────────────────────────────────────────
        (Exchange::SZAA, OrderSide::Buy,  false) => "TTTS0305U",
        (Exchange::SZAA, OrderSide::Buy,  true)  => "VTTS0305U",
        (Exchange::SZAA, OrderSide::Sell, false) => "TTTS0304U",
        (Exchange::SZAA, OrderSide::Sell, true)  => "VTTS0304U",
        // ── 도쿄 ──────────────────────────────────────────────────────────
        (Exchange::TKSE, OrderSide::Buy,  false) => "TTTS0308U",
        (Exchange::TKSE, OrderSide::Buy,  true)  => "VTTS0308U",
        (Exchange::TKSE, OrderSide::Sell, false) => "TTTS0307U",
        (Exchange::TKSE, OrderSide::Sell, true)  => "VTTS0307U",
        // ── 베트남 ────────────────────────────────────────────────────────
        (Exchange::HASE | Exchange::VNSE, OrderSide::Buy,  false) => "TTTS0311U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Buy,  true)  => "VTTS0311U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Sell, false) => "TTTS0310U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Sell, true)  => "VTTS0310U",
        // ── 미지원 ────────────────────────────────────────────────────────
        _ => "",
    }
}

/// 정정/취소 TR ID 반환.
///
/// - 미국 (NASD/NYSE/AMEX): real=TTTT1004U, VTS=VTTT1004U
/// - 기타: real=TTTS1003U, VTS=VTTS1003U
pub fn cancel_tr_id(exchange: &Exchange, mock: bool) -> &'static str {
    match (exchange, mock) {
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, false) => "TTTT1004U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, true)  => "VTTT1004U",
        (_, false) => "TTTS1003U",
        (_, true)  => "VTTS1003U",
    }
}

// ─── 단위 테스트 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchange_display() {
        assert_eq!(Exchange::NASD.to_string(), "NASD");
        assert_eq!(Exchange::Other("XLON".to_string()).to_string(), "XLON");
    }

    #[test]
    fn exchange_from_str() {
        assert_eq!(Exchange::from("NYSE"), Exchange::NYSE);
        assert_eq!(Exchange::from("XPAR"), Exchange::Other("XPAR".to_string()));
    }

    #[test]
    fn order_tr_id_nasd_buy_real() {
        assert_eq!(order_tr_id(&Exchange::NASD, &OrderSide::Buy, false), "TTTT1002U");
    }

    #[test]
    fn order_tr_id_nasd_buy_vts() {
        assert_eq!(order_tr_id(&Exchange::NASD, &OrderSide::Buy, true), "VTTT1002U");
    }

    #[test]
    fn order_tr_id_nasd_sell_real() {
        assert_eq!(order_tr_id(&Exchange::NASD, &OrderSide::Sell, false), "TTTT1006U");
    }

    #[test]
    fn order_tr_id_nasd_sell_vts() {
        assert_eq!(order_tr_id(&Exchange::NASD, &OrderSide::Sell, true), "VTTT1006U");
    }

    #[test]
    fn order_tr_id_nyse_same_as_nasd() {
        assert_eq!(
            order_tr_id(&Exchange::NYSE, &OrderSide::Buy, false),
            order_tr_id(&Exchange::NASD, &OrderSide::Buy, false)
        );
    }

    #[test]
    fn order_tr_id_sehk() {
        assert_eq!(order_tr_id(&Exchange::SEHK, &OrderSide::Buy,  false), "TTTS1002U");
        assert_eq!(order_tr_id(&Exchange::SEHK, &OrderSide::Sell, false), "TTTS1001U");
        assert_eq!(order_tr_id(&Exchange::SEHK, &OrderSide::Buy,  true),  "VTTS1002U");
    }

    #[test]
    fn order_tr_id_other_returns_empty() {
        assert_eq!(order_tr_id(&Exchange::Other("XLON".to_string()), &OrderSide::Buy, false), "");
    }

    #[test]
    fn cancel_tr_id_us_real() {
        assert_eq!(cancel_tr_id(&Exchange::NASD, false), "TTTT1004U");
        assert_eq!(cancel_tr_id(&Exchange::NYSE, false), "TTTT1004U");
        assert_eq!(cancel_tr_id(&Exchange::AMEX, false), "TTTT1004U");
    }

    #[test]
    fn cancel_tr_id_us_vts() {
        assert_eq!(cancel_tr_id(&Exchange::NASD, true), "VTTT1004U");
    }

    #[test]
    fn cancel_tr_id_non_us() {
        assert_eq!(cancel_tr_id(&Exchange::SEHK, false), "TTTS1003U");
        assert_eq!(cancel_tr_id(&Exchange::SEHK, true),  "VTTS1003U");
        assert_eq!(cancel_tr_id(&Exchange::TKSE, false), "TTTS1003U");
    }

    #[test]
    fn order_type_to_ord_dvsn() {
        assert_eq!(OrderType::Limit.to_ord_dvsn(), "00");
        assert_eq!(OrderType::Market.to_ord_dvsn(), "01");
        assert_eq!(OrderType::LimitAon.to_ord_dvsn(), "32");
        assert_eq!(OrderType::MarketClose.to_ord_dvsn(), "34");
    }
}
```

- [ ] **Step 2: `crates/kis_api/src/rest/overseas/order/mod.rs` 생성**

```rust
pub mod place;
pub mod cancel;
```

- [ ] **Step 3: `crates/kis_api/src/rest/overseas/inquiry/mod.rs` 생성**

```rust
pub mod balance;
pub mod orders;
pub mod profit;
```

- [ ] **Step 4: `crates/kis_api/src/rest/overseas/mod.rs` 업데이트**

기존 `mod.rs`에 아래 선언을 추가한다:

```rust
pub mod types;
pub mod order;
pub mod inquiry;
```

`deposit`, `price` 모듈 선언은 유지한다.

- [ ] **Step 5: `crates/kis_api/src/lib.rs` 업데이트 — 신규 타입 re-export 추가**

```rust
pub use rest::overseas::types::{Exchange, OrderSide, OrderType, SubscriptionKind};
```

- [ ] **Step 6: 컴파일 확인**

```bash
cargo build -p kis_api 2>&1
```

오류 없이 빌드되어야 한다.

- [ ] **Step 7: 테스트 실행**

```bash
cargo test -p kis_api rest::overseas::types 2>&1
```

예상 출력:
```
running 11 tests
test rest::overseas::types::tests::exchange_display ... ok
test rest::overseas::types::tests::exchange_from_str ... ok
test rest::overseas::types::tests::order_tr_id_nasd_buy_real ... ok
test rest::overseas::types::tests::order_tr_id_nasd_buy_vts ... ok
test rest::overseas::types::tests::order_tr_id_nasd_sell_real ... ok
test rest::overseas::types::tests::order_tr_id_nasd_sell_vts ... ok
test rest::overseas::types::tests::order_tr_id_nyse_same_as_nasd ... ok
test rest::overseas::types::tests::order_tr_id_sehk ... ok
test rest::overseas::types::tests::order_tr_id_other_returns_empty ... ok
test rest::overseas::types::tests::cancel_tr_id_us_real ... ok
test rest::overseas::types::tests::cancel_tr_id_us_vts ... ok
test rest::overseas::types::tests::cancel_tr_id_non_us ... ok
test rest::overseas::types::tests::order_type_to_ord_dvsn ... ok
test result: ok. 13 passed; 0 failed
```

- [ ] **Step 8: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/types.rs \
        crates/kis_api/src/rest/overseas/order/mod.rs \
        crates/kis_api/src/rest/overseas/inquiry/mod.rs \
        crates/kis_api/src/rest/overseas/mod.rs \
        crates/kis_api/src/lib.rs
git commit -m "[plan3a/task0] Add overseas common types (Exchange, OrderSide, OrderType, TR ID fns)"
```

---

## Task 1: `place_order` — 매수/매도 주문

**Files:**
- Create: `crates/kis_api/tests/fixtures/overseas/order/place_order.json`
- Create: `crates/kis_api/src/rest/overseas/order/place.rs`
- Update: `crates/kis_api/src/rest/overseas/order/mod.rs`
- Update: `crates/kis_api/src/lib.rs`

**KIS API:** `POST /uapi/overseas-stock/v1/trading/order`

---

- [ ] **Step 1: 픽스처 JSON 작성**

파일: `crates/kis_api/tests/fixtures/overseas/order/place_order.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "주문 전송 완료 되었습니다.",
  "output": {
    "KRX_FWDG_ORD_ORGNO": "91252",
    "ODNO": "0000117057",
    "ORD_TMD": "141922",
    "ORDS_PRCS_DT": "20260321",
    "ORDS_PRCS_DTIME": "20260321141922123456"
  }
}
```

- [ ] **Step 2: `place.rs` 에 타입 및 테스트 작성 (구현 없이)**

파일: `crates/kis_api/src/rest/overseas/order/place.rs`

```rust
use rust_decimal::Decimal;
use crate::{
    config::KisConfig,
    error::KisError,
    rest::overseas::types::{Exchange, OrderSide, OrderType, order_tr_id},
};

/// 해외주식 주문 요청
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub qty: Decimal,
    /// None = 시장가 ("0" 전송)
    pub price: Option<Decimal>,
}

/// 해외주식 주문 응답
pub struct PlaceOrderResponse {
    /// 주문처리일자 (ORDS_PRCS_DT) — YYYYMMDD 문자열
    pub order_date: String,
    /// KRX전송주문조직번호 (KRX_FWDG_ORD_ORGNO)
    pub order_org_no: String,
    /// 주문처리일시 (ORDS_PRCS_DTIME) — YYYYMMDDHHmmssSSSSS 문자열
    pub order_time: String,
}

/// 해외주식 매수/매도 주문.
///
/// # TR ID
/// `order_tr_id(exchange, side, config.mock())` 로 결정.
///
/// # Body 필드
/// - CANO: account 앞 8자리
/// - ACNT_PRDT_CD: account 뒤 2자리
/// - OVRS_EXCG_CD: exchange.to_string()
/// - PDNO: symbol
/// - ORD_DVSN: order_type.to_ord_dvsn()
/// - ORD_QTY: qty (소수점 없이 전송 — 정수 변환)
/// - OVRS_ORD_UNPR: price (시장가면 "0")
/// - SLL_TYPE: 매도="00", 매수=""
/// - ORD_SVR_DVSN_CD: "0"
/// - CTAC_TLNO: ""
/// - MGCO_APTM_ODNO: ""
pub async fn place_order(
    config: &KisConfig,
    token: &str,
    req: PlaceOrderRequest,
) -> Result<PlaceOrderResponse, KisError> {
    todo!()
}

// ─── 단위 테스트 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_place_order_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/order/place_order.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        let output = &v["output"];

        let resp = PlaceOrderResponse {
            order_date:    output["ORDS_PRCS_DT"].as_str().unwrap().to_string(),
            order_org_no:  output["KRX_FWDG_ORD_ORGNO"].as_str().unwrap().to_string(),
            order_time:    output["ORDS_PRCS_DTIME"].as_str().unwrap().to_string(),
        };

        assert_eq!(resp.order_date, "20260321");
        assert_eq!(resp.order_org_no, "91252");
        assert!(!resp.order_time.is_empty());
    }

    #[test]
    fn place_order_request_fields() {
        use rust_decimal_macros::dec;
        let req = PlaceOrderRequest {
            symbol: "AAPL".to_string(),
            exchange: Exchange::NASD,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            qty: dec!(10),
            price: Some(dec!(175.50)),
        };
        assert_eq!(req.symbol, "AAPL");
        assert_eq!(req.exchange, Exchange::NASD);
        assert_eq!(req.qty, dec!(10));
    }
}
```

- [ ] **Step 3: 테스트 실패 확인**

```bash
cargo test -p kis_api rest::overseas::order::place 2>&1
```

`todo!()` 패닉 없이 직렬화 테스트만 통과해야 한다:
```
test rest::overseas::order::place::tests::deserialize_place_order_response ... ok
test rest::overseas::order::place::tests::place_order_request_fields ... ok
```

- [ ] **Step 4: `place_order` 함수 구현**

```rust
pub async fn place_order(
    config: &KisConfig,
    token: &str,
    req: PlaceOrderRequest,
) -> Result<PlaceOrderResponse, KisError> {
    let tr_id = order_tr_id(&req.exchange, &req.side, config.mock());
    if tr_id.is_empty() {
        return Err(KisError::Api {
            code: "UNSUPPORTED_EXCHANGE".to_string(),
            message: format!("Exchange {} is not supported for order placement", req.exchange),
        });
    }

    // account 분리: "XXXXXXXX-XX" → cano(8) + acnt_prdt_cd(2)
    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let price_str = req.price
        .map(|p| p.to_string())
        .unwrap_or_else(|| "0".to_string());

    let sll_type = match req.side {
        OrderSide::Sell => "00",
        OrderSide::Buy  => "",
    };

    let body = serde_json::json!({
        "CANO":           cano,
        "ACNT_PRDT_CD":   acnt_prdt_cd,
        "OVRS_EXCG_CD":   req.exchange.to_string(),
        "PDNO":           req.symbol,
        "ORD_DVSN":       req.order_type.to_ord_dvsn(),
        "ORD_QTY":        req.qty.to_string(),
        "OVRS_ORD_UNPR":  price_str,
        "SLL_TYPE":       sll_type,
        "ORD_SVR_DVSN_CD": "0",
        "CTAC_TLNO":      "",
        "MGCO_APTM_ODNO": "",
    });

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), tr_id.to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/order",
        http::Method::POST,
        headers,
        Some(body),
        None,
    ).await?;

    let output = &resp["output"];

    Ok(PlaceOrderResponse {
        order_date:   output["ORDS_PRCS_DT"].as_str().unwrap_or("").to_string(),
        order_org_no: output["KRX_FWDG_ORD_ORGNO"].as_str().unwrap_or("").to_string(),
        order_time:   output["ORDS_PRCS_DTIME"].as_str().unwrap_or("").to_string(),
    })
}
```

> **Note:** `crate::rest::http::execute` 시그니처는 Plan 2 기준. `headers: HashMap<String, String>`, `body: Option<Value>`, `query: Option<HashMap<String,String>>`. 실제 시그니처가 다를 경우 Plan 2 구현에 맞게 조정한다.

- [ ] **Step 5: `order/mod.rs` 업데이트**

```rust
pub mod place;
pub mod cancel;

pub use place::{place_order, PlaceOrderRequest, PlaceOrderResponse};
```

- [ ] **Step 6: `lib.rs` re-export 추가**

```rust
pub use rest::overseas::order::place::{PlaceOrderRequest, PlaceOrderResponse};
```

- [ ] **Step 7: 전체 테스트 통과 확인**

```bash
cargo test -p kis_api rest::overseas::order::place 2>&1
```

- [ ] **Step 8: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/order/place.rs \
        crates/kis_api/src/rest/overseas/order/mod.rs \
        crates/kis_api/tests/fixtures/overseas/order/place_order.json \
        crates/kis_api/src/lib.rs
git commit -m "[plan3a/task1] Implement place_order for overseas stock"
```

---

## Task 2: `cancel_order` — 정정/취소 주문

**Files:**
- Create: `crates/kis_api/tests/fixtures/overseas/order/cancel_order.json`
- Create: `crates/kis_api/src/rest/overseas/order/cancel.rs`
- Update: `crates/kis_api/src/rest/overseas/order/mod.rs`
- Update: `crates/kis_api/src/lib.rs`

**KIS API:** `POST /uapi/overseas-stock/v1/trading/order-rvsecncl`

---

- [ ] **Step 1: 픽스처 JSON 작성**

파일: `crates/kis_api/tests/fixtures/overseas/order/cancel_order.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "주문 전송 완료 되었습니다.",
  "output": {
    "KRX_FWDG_ORD_ORGNO": "91252",
    "ODNO": "0000117080",
    "ORD_TMD": "142230",
    "ORDS_PRCS_DT": "20260321",
    "ORDS_PRCS_DTIME": "20260321142230654321"
  }
}
```

- [ ] **Step 2: `cancel.rs` — 타입 및 테스트 작성**

파일: `crates/kis_api/src/rest/overseas/order/cancel.rs`

```rust
use rust_decimal::Decimal;
use crate::{
    config::KisConfig,
    error::KisError,
    rest::overseas::types::{Exchange, cancel_tr_id},
};

/// 정정(01) 또는 취소(02)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelKind {
    /// 정정 (RVSE_CNCL_DVSN_CD = "01")
    Amend,
    /// 취소 (RVSE_CNCL_DVSN_CD = "02")
    Cancel,
}

impl CancelKind {
    pub fn to_dvsn_cd(&self) -> &str {
        match self {
            CancelKind::Amend  => "01",
            CancelKind::Cancel => "02",
        }
    }
}

/// 해외주식 정정/취소 요청
pub struct CancelOrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    /// 원주문번호 (ORGN_ODNO)
    pub original_order_id: String,
    pub kind: CancelKind,
    pub qty: Decimal,
    /// 정정 시 변경 가격, 취소 시 None
    pub price: Option<Decimal>,
}

/// 해외주식 정정/취소 응답
pub struct CancelOrderResponse {
    /// 주문처리일자
    pub order_date: String,
    /// KRX전송주문조직번호
    pub order_org_no: String,
    /// 주문처리일시
    pub order_time: String,
}

/// 해외주식 주문 정정 또는 취소.
///
/// # TR ID
/// `cancel_tr_id(exchange, config.mock())` 로 결정.
pub async fn cancel_order(
    config: &KisConfig,
    token: &str,
    req: CancelOrderRequest,
) -> Result<CancelOrderResponse, KisError> {
    let tr_id = cancel_tr_id(&req.exchange, config.mock());

    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let price_str = req.price
        .map(|p| p.to_string())
        .unwrap_or_else(|| "0".to_string());

    let body = serde_json::json!({
        "CANO":              cano,
        "ACNT_PRDT_CD":      acnt_prdt_cd,
        "OVRS_EXCG_CD":      req.exchange.to_string(),
        "PDNO":              req.symbol,
        "ORGN_ODNO":         req.original_order_id,
        "RVSE_CNCL_DVSN_CD": req.kind.to_dvsn_cd(),
        "ORD_QTY":           req.qty.to_string(),
        "OVRS_ORD_UNPR":     price_str,
        "CTAC_TLNO":         "",
        "MGCO_APTM_ODNO":    "",
    });

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), tr_id.to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/order-rvsecncl",
        http::Method::POST,
        headers,
        Some(body),
        None,
    ).await?;

    let output = &resp["output"];

    Ok(CancelOrderResponse {
        order_date:   output["ORDS_PRCS_DT"].as_str().unwrap_or("").to_string(),
        order_org_no: output["KRX_FWDG_ORD_ORGNO"].as_str().unwrap_or("").to_string(),
        order_time:   output["ORDS_PRCS_DTIME"].as_str().unwrap_or("").to_string(),
    })
}

// ─── 단위 테스트 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_cancel_order_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/order/cancel_order.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        let output = &v["output"];

        let resp = CancelOrderResponse {
            order_date:   output["ORDS_PRCS_DT"].as_str().unwrap().to_string(),
            order_org_no: output["KRX_FWDG_ORD_ORGNO"].as_str().unwrap().to_string(),
            order_time:   output["ORDS_PRCS_DTIME"].as_str().unwrap().to_string(),
        };

        assert_eq!(resp.order_date, "20260321");
        assert_eq!(resp.order_org_no, "91252");
        assert!(!resp.order_time.is_empty());
    }

    #[test]
    fn cancel_kind_dvsn_cd() {
        assert_eq!(CancelKind::Amend.to_dvsn_cd(), "01");
        assert_eq!(CancelKind::Cancel.to_dvsn_cd(), "02");
    }
}
```

- [ ] **Step 3: 테스트 실행 확인**

```bash
cargo test -p kis_api rest::overseas::order::cancel 2>&1
```

예상:
```
test rest::overseas::order::cancel::tests::deserialize_cancel_order_response ... ok
test rest::overseas::order::cancel::tests::cancel_kind_dvsn_cd ... ok
test result: ok. 2 passed; 0 failed
```

- [ ] **Step 4: `order/mod.rs` 업데이트**

```rust
pub mod place;
pub mod cancel;

pub use place::{place_order, PlaceOrderRequest, PlaceOrderResponse};
pub use cancel::{cancel_order, CancelKind, CancelOrderRequest, CancelOrderResponse};
```

- [ ] **Step 5: `lib.rs` re-export 추가**

```rust
pub use rest::overseas::order::cancel::{CancelKind, CancelOrderRequest, CancelOrderResponse};
```

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/order/cancel.rs \
        crates/kis_api/src/rest/overseas/order/mod.rs \
        crates/kis_api/tests/fixtures/overseas/order/cancel_order.json \
        crates/kis_api/src/lib.rs
git commit -m "[plan3a/task2] Implement cancel_order (amend/cancel) for overseas stock"
```

---

## Task 3: `balance` — 잔고 조회

**Files:**
- Create: `crates/kis_api/tests/fixtures/overseas/inquiry/balance.json`
- Create: `crates/kis_api/src/rest/overseas/inquiry/balance.rs`
- Update: `crates/kis_api/src/rest/overseas/inquiry/mod.rs`
- Update: `crates/kis_api/src/lib.rs`

**KIS API:** `GET /uapi/overseas-stock/v1/trading/inquire-balance`
**TR ID:** real=`TTTS3012R`, VTS=`VTTS3012R`

---

- [ ] **Step 1: 픽스처 JSON 작성**

파일: `crates/kis_api/tests/fixtures/overseas/inquiry/balance.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "조회가 완료되었습니다.",
  "ctx_area_fk200": "",
  "ctx_area_nk200": "",
  "output1": [
    {
      "PDNO": "AAPL",
      "PRDT_NAME": "애플",
      "OVRS_EXCG_CD": "NASD",
      "OVRS_CBLC_QTY": "10",
      "PCHS_AVG_PRIC": "175.3200",
      "OVRS_STCK_EVLU_AMT": "1780.50",
      "FRCR_EVLU_PFLS_AMT": "47.30",
      "EVLU_PFLS_RT": "2.70"
    },
    {
      "PDNO": "TSLA",
      "PRDT_NAME": "테슬라",
      "OVRS_EXCG_CD": "NASD",
      "OVRS_CBLC_QTY": "5",
      "PCHS_AVG_PRIC": "220.0000",
      "OVRS_STCK_EVLU_AMT": "1085.00",
      "FRCR_EVLU_PFLS_AMT": "-15.00",
      "EVLU_PFLS_RT": "-1.36"
    }
  ],
  "output2": {
    "FRCR_PCHS_AMT1": "2853.20",
    "OVRS_RLZT_PFLS_AMT": "0.00",
    "OVRS_TOT_PFLS": "32.30"
  }
}
```

- [ ] **Step 2: `balance.rs` — 타입 및 테스트 작성**

파일: `crates/kis_api/src/rest/overseas/inquiry/balance.rs`

```rust
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::{config::KisConfig, error::KisError};

/// 보유 종목 잔고 항목
pub struct BalanceItem {
    /// 종목코드 (PDNO)
    pub symbol: String,
    /// 종목명 (PRDT_NAME)
    pub name: String,
    /// 거래소코드 (OVRS_EXCG_CD)
    pub exchange: String,
    /// 보유수량 (OVRS_CBLC_QTY)
    pub qty: Decimal,
    /// 매입평균가 (PCHS_AVG_PRIC)
    pub avg_price: Decimal,
    /// 해외주식평가금액 (OVRS_STCK_EVLU_AMT)
    pub eval_amount: Decimal,
    /// 외화평가손익금액 (FRCR_EVLU_PFLS_AMT)
    pub unrealized_pnl: Decimal,
    /// 평가손익율 (EVLU_PFLS_RT)
    pub pnl_rate: Decimal,
}

/// 잔고 요약
pub struct BalanceSummary {
    /// 외화매입금액1 (FRCR_PCHS_AMT1)
    pub purchase_amount: Decimal,
    /// 해외실현손익금액 (OVRS_RLZT_PFLS_AMT)
    pub realized_pnl: Decimal,
    /// 해외총손익금액 (OVRS_TOT_PFLS)
    pub total_pnl: Decimal,
}

/// 잔고 조회 응답
pub struct BalanceResponse {
    pub items: Vec<BalanceItem>,
    pub summary: BalanceSummary,
}

fn parse_decimal(v: &serde_json::Value, key: &str) -> Decimal {
    v[key].as_str()
        .and_then(|s| Decimal::from_str(s).ok())
        .unwrap_or(Decimal::ZERO)
}

/// 해외주식 잔고 조회.
///
/// 페이지네이션은 내부에서 처리하지 않음 (CTX_AREA_FK200/NK200 = "" 로 단일 호출).
/// 필요 시 향후 `balance_all()` 로 래핑.
pub async fn balance(
    config: &KisConfig,
    token: &str,
) -> Result<BalanceResponse, KisError> {
    let tr_id = if config.mock() { "VTTS3012R" } else { "TTTS3012R" };

    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let mut query = std::collections::HashMap::new();
    query.insert("CANO".to_string(),          cano.to_string());
    query.insert("ACNT_PRDT_CD".to_string(),  acnt_prdt_cd.to_string());
    query.insert("OVRS_EXCG_CD".to_string(),  "".to_string());
    query.insert("TR_CRCY_CD".to_string(),    "".to_string());
    query.insert("CTX_AREA_FK200".to_string(), "".to_string());
    query.insert("CTX_AREA_NK200".to_string(), "".to_string());

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), tr_id.to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/inquire-balance",
        http::Method::GET,
        headers,
        None,
        Some(query),
    ).await?;

    let items = resp["output1"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| BalanceItem {
            symbol:         item["PDNO"].as_str().unwrap_or("").to_string(),
            name:           item["PRDT_NAME"].as_str().unwrap_or("").to_string(),
            exchange:       item["OVRS_EXCG_CD"].as_str().unwrap_or("").to_string(),
            qty:            parse_decimal(item, "OVRS_CBLC_QTY"),
            avg_price:      parse_decimal(item, "PCHS_AVG_PRIC"),
            eval_amount:    parse_decimal(item, "OVRS_STCK_EVLU_AMT"),
            unrealized_pnl: parse_decimal(item, "FRCR_EVLU_PFLS_AMT"),
            pnl_rate:       parse_decimal(item, "EVLU_PFLS_RT"),
        })
        .collect();

    let out2 = &resp["output2"];
    let summary = BalanceSummary {
        purchase_amount: parse_decimal(out2, "FRCR_PCHS_AMT1"),
        realized_pnl:    parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
        total_pnl:       parse_decimal(out2, "OVRS_TOT_PFLS"),
    };

    Ok(BalanceResponse { items, summary })
}

// ─── 단위 테스트 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_balance_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/balance.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();

        let items: Vec<BalanceItem> = v["output1"]
            .as_array().unwrap()
            .iter()
            .map(|item| BalanceItem {
                symbol:         item["PDNO"].as_str().unwrap().to_string(),
                name:           item["PRDT_NAME"].as_str().unwrap().to_string(),
                exchange:       item["OVRS_EXCG_CD"].as_str().unwrap().to_string(),
                qty:            parse_decimal(item, "OVRS_CBLC_QTY"),
                avg_price:      parse_decimal(item, "PCHS_AVG_PRIC"),
                eval_amount:    parse_decimal(item, "OVRS_STCK_EVLU_AMT"),
                unrealized_pnl: parse_decimal(item, "FRCR_EVLU_PFLS_AMT"),
                pnl_rate:       parse_decimal(item, "EVLU_PFLS_RT"),
            })
            .collect();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].symbol, "AAPL");
        assert_eq!(items[0].exchange, "NASD");
        assert!(items[0].qty > Decimal::ZERO);
        assert!(items[0].pnl_rate > Decimal::ZERO);
        assert!(items[1].unrealized_pnl < Decimal::ZERO);

        let out2 = &v["output2"];
        let summary = BalanceSummary {
            purchase_amount: parse_decimal(out2, "FRCR_PCHS_AMT1"),
            realized_pnl:    parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
            total_pnl:       parse_decimal(out2, "OVRS_TOT_PFLS"),
        };
        assert!(summary.purchase_amount > Decimal::ZERO);
        assert_eq!(summary.realized_pnl, Decimal::ZERO);
    }
}
```

- [ ] **Step 3: 테스트 실행**

```bash
cargo test -p kis_api rest::overseas::inquiry::balance 2>&1
```

예상:
```
test rest::overseas::inquiry::balance::tests::deserialize_balance_response ... ok
test result: ok. 1 passed; 0 failed
```

- [ ] **Step 4: `inquiry/mod.rs` 업데이트**

```rust
pub mod balance;
pub mod orders;
pub mod profit;

pub use balance::{balance, BalanceItem, BalanceSummary, BalanceResponse};
```

- [ ] **Step 5: `lib.rs` re-export 추가**

```rust
pub use rest::overseas::inquiry::balance::{BalanceItem, BalanceSummary, BalanceResponse};
```

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/inquiry/balance.rs \
        crates/kis_api/src/rest/overseas/inquiry/mod.rs \
        crates/kis_api/tests/fixtures/overseas/inquiry/balance.json \
        crates/kis_api/src/lib.rs
git commit -m "[plan3a/task3] Implement balance() for overseas stock inquiry"
```

---

## Task 4: `unfilled_orders` + `order_history`

**Files:**
- Create: `crates/kis_api/tests/fixtures/overseas/inquiry/unfilled_orders.json`
- Create: `crates/kis_api/tests/fixtures/overseas/inquiry/order_history.json`
- Create: `crates/kis_api/src/rest/overseas/inquiry/orders.rs`
- Update: `crates/kis_api/src/rest/overseas/inquiry/mod.rs`
- Update: `crates/kis_api/src/lib.rs`

**KIS API:**
- 미체결: `GET /uapi/overseas-stock/v1/trading/inquire-nccs` TR ID: `TTTS3018R` (real+VTS 동일)
- 체결내역: `GET /uapi/overseas-stock/v1/trading/inquire-ccnl` TR ID: real=`TTTS3035R`, VTS=`VTTS3035R`

---

- [ ] **Step 1: 픽스처 JSON 작성**

파일: `crates/kis_api/tests/fixtures/overseas/inquiry/unfilled_orders.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "조회가 완료되었습니다.",
  "ctx_area_fk200": "",
  "ctx_area_nk200": "",
  "output": [
    {
      "ODNO": "0000117057",
      "ORGN_ODNO": "",
      "PDNO": "AAPL",
      "PRDT_NAME": "애플",
      "OVRS_EXCG_CD": "NASD",
      "SLL_BUY_DVSN_CD": "02",
      "ORD_QTY": "10",
      "OVRS_ORD_UNPR": "175.00",
      "FT_CCLD_QTY": "0",
      "RMND_QTY": "10"
    }
  ]
}
```

파일: `crates/kis_api/tests/fixtures/overseas/inquiry/order_history.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "조회가 완료되었습니다.",
  "ctx_area_fk200": "",
  "ctx_area_nk200": "",
  "output": [
    {
      "ODNO": "0000117001",
      "PDNO": "TSLA",
      "PRDT_NAME": "테슬라",
      "SLL_BUY_DVSN_CD": "01",
      "ORD_QTY": "5",
      "CCLD_QTY": "5",
      "CCLD_UNPR": "222.30",
      "CCLD_DT": "20260320",
      "CCLD_TM": "143500"
    }
  ]
}
```

- [ ] **Step 2: `orders.rs` — 타입, 테스트, 구현 작성**

파일: `crates/kis_api/src/rest/overseas/inquiry/orders.rs`

```rust
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::NaiveDate;
use crate::{config::KisConfig, error::KisError};

fn parse_decimal(v: &serde_json::Value, key: &str) -> Decimal {
    v[key].as_str()
        .and_then(|s| Decimal::from_str(s).ok())
        .unwrap_or(Decimal::ZERO)
}

/// 미체결 주문 항목
pub struct UnfilledOrder {
    /// 주문번호 (ODNO)
    pub order_id: String,
    /// 종목코드 (PDNO)
    pub symbol: String,
    /// 거래소코드 (OVRS_EXCG_CD)
    pub exchange: String,
    /// 매도매수구분코드 ("01"=매도, "02"=매수)
    pub side: String,
    /// 주문수량 (ORD_QTY)
    pub ordered_qty: Decimal,
    /// 해외주문단가 (OVRS_ORD_UNPR)
    pub order_price: Decimal,
    /// 해외체결수량 (FT_CCLD_QTY)
    pub filled_qty: Decimal,
    /// 잔여수량 (RMND_QTY)
    pub remaining_qty: Decimal,
}

/// 체결내역 항목
pub struct OrderHistoryItem {
    /// 주문번호 (ODNO)
    pub order_id: String,
    /// 종목코드 (PDNO)
    pub symbol: String,
    /// 매도매수구분코드 (SLL_BUY_DVSN_CD)
    pub side: String,
    /// 주문수량 (ORD_QTY)
    pub ordered_qty: Decimal,
    /// 체결수량 (CCLD_QTY)
    pub filled_qty: Decimal,
    /// 체결단가 (CCLD_UNPR)
    pub filled_price: Decimal,
    /// 체결일자 (CCLD_DT) — YYYYMMDD
    pub fill_date: String,
    /// 체결시각 (CCLD_TM) — HHmmss
    pub fill_time: String,
}

/// 체결내역 조회 요청 기간
pub struct OrderHistoryRequest {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

/// 해외주식 미체결 조회.
///
/// TR ID: `TTTS3018R` (real/VTS 공통)
pub async fn unfilled_orders(
    config: &KisConfig,
    token: &str,
) -> Result<Vec<UnfilledOrder>, KisError> {
    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let mut query = std::collections::HashMap::new();
    query.insert("CANO".to_string(),          cano.to_string());
    query.insert("ACNT_PRDT_CD".to_string(),  acnt_prdt_cd.to_string());
    query.insert("OVRS_EXCG_CD".to_string(),  "".to_string());
    query.insert("SORT_SQN".to_string(),      "DS".to_string());
    query.insert("CTX_AREA_FK200".to_string(), "".to_string());
    query.insert("CTX_AREA_NK200".to_string(), "".to_string());

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), "TTTS3018R".to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/inquire-nccs",
        http::Method::GET,
        headers,
        None,
        Some(query),
    ).await?;

    let orders = resp["output"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| UnfilledOrder {
            order_id:      item["ODNO"].as_str().unwrap_or("").to_string(),
            symbol:        item["PDNO"].as_str().unwrap_or("").to_string(),
            exchange:      item["OVRS_EXCG_CD"].as_str().unwrap_or("").to_string(),
            side:          item["SLL_BUY_DVSN_CD"].as_str().unwrap_or("").to_string(),
            ordered_qty:   parse_decimal(item, "ORD_QTY"),
            order_price:   parse_decimal(item, "OVRS_ORD_UNPR"),
            filled_qty:    parse_decimal(item, "FT_CCLD_QTY"),
            remaining_qty: parse_decimal(item, "RMND_QTY"),
        })
        .collect();

    Ok(orders)
}

/// 해외주식 체결내역 조회.
///
/// TR ID: real=`TTTS3035R`, VTS=`VTTS3035R`
pub async fn order_history(
    config: &KisConfig,
    token: &str,
    req: OrderHistoryRequest,
) -> Result<Vec<OrderHistoryItem>, KisError> {
    let tr_id = if config.mock() { "VTTS3035R" } else { "TTTS3035R" };

    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let mut query = std::collections::HashMap::new();
    query.insert("CANO".to_string(),          cano.to_string());
    query.insert("ACNT_PRDT_CD".to_string(),  acnt_prdt_cd.to_string());
    query.insert("PDNO".to_string(),          "".to_string());
    query.insert("ORD_STRT_DT".to_string(),   req.from.format("%Y%m%d").to_string());
    query.insert("ORD_END_DT".to_string(),    req.to.format("%Y%m%d").to_string());
    query.insert("SLL_BUY_DVSN".to_string(),  "00".to_string()); // 전체
    query.insert("CCLD_NCCS_DVSN".to_string(), "01".to_string()); // 체결
    query.insert("SORT_SQN".to_string(),      "DS".to_string());
    query.insert("CTX_AREA_FK200".to_string(), "".to_string());
    query.insert("CTX_AREA_NK200".to_string(), "".to_string());

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), tr_id.to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/inquire-ccnl",
        http::Method::GET,
        headers,
        None,
        Some(query),
    ).await?;

    let history = resp["output"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| OrderHistoryItem {
            order_id:     item["ODNO"].as_str().unwrap_or("").to_string(),
            symbol:       item["PDNO"].as_str().unwrap_or("").to_string(),
            side:         item["SLL_BUY_DVSN_CD"].as_str().unwrap_or("").to_string(),
            ordered_qty:  parse_decimal(item, "ORD_QTY"),
            filled_qty:   parse_decimal(item, "CCLD_QTY"),
            filled_price: parse_decimal(item, "CCLD_UNPR"),
            fill_date:    item["CCLD_DT"].as_str().unwrap_or("").to_string(),
            fill_time:    item["CCLD_TM"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    Ok(history)
}

// ─── 단위 테스트 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_unfilled_orders() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/unfilled_orders.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();

        let orders: Vec<UnfilledOrder> = v["output"]
            .as_array().unwrap()
            .iter()
            .map(|item| UnfilledOrder {
                order_id:      item["ODNO"].as_str().unwrap().to_string(),
                symbol:        item["PDNO"].as_str().unwrap().to_string(),
                exchange:      item["OVRS_EXCG_CD"].as_str().unwrap().to_string(),
                side:          item["SLL_BUY_DVSN_CD"].as_str().unwrap().to_string(),
                ordered_qty:   parse_decimal(item, "ORD_QTY"),
                order_price:   parse_decimal(item, "OVRS_ORD_UNPR"),
                filled_qty:    parse_decimal(item, "FT_CCLD_QTY"),
                remaining_qty: parse_decimal(item, "RMND_QTY"),
            })
            .collect();

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].symbol, "AAPL");
        assert_eq!(orders[0].side, "02");
        assert_eq!(orders[0].filled_qty, Decimal::ZERO);
        assert!(orders[0].remaining_qty > Decimal::ZERO);
    }

    #[test]
    fn deserialize_order_history() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/order_history.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();

        let history: Vec<OrderHistoryItem> = v["output"]
            .as_array().unwrap()
            .iter()
            .map(|item| OrderHistoryItem {
                order_id:     item["ODNO"].as_str().unwrap().to_string(),
                symbol:       item["PDNO"].as_str().unwrap().to_string(),
                side:         item["SLL_BUY_DVSN_CD"].as_str().unwrap().to_string(),
                ordered_qty:  parse_decimal(item, "ORD_QTY"),
                filled_qty:   parse_decimal(item, "CCLD_QTY"),
                filled_price: parse_decimal(item, "CCLD_UNPR"),
                fill_date:    item["CCLD_DT"].as_str().unwrap().to_string(),
                fill_time:    item["CCLD_TM"].as_str().unwrap().to_string(),
            })
            .collect();

        assert_eq!(history.len(), 1);
        assert_eq!(history[0].symbol, "TSLA");
        assert_eq!(history[0].side, "01"); // 매도
        assert!(history[0].filled_price > Decimal::ZERO);
        assert_eq!(history[0].fill_date, "20260320");
    }
}
```

- [ ] **Step 3: 테스트 실행**

```bash
cargo test -p kis_api rest::overseas::inquiry::orders 2>&1
```

예상:
```
test rest::overseas::inquiry::orders::tests::deserialize_unfilled_orders ... ok
test rest::overseas::inquiry::orders::tests::deserialize_order_history ... ok
test result: ok. 2 passed; 0 failed
```

- [ ] **Step 4: `inquiry/mod.rs` 업데이트**

```rust
pub mod balance;
pub mod orders;
pub mod profit;

pub use balance::{balance, BalanceItem, BalanceSummary, BalanceResponse};
pub use orders::{
    unfilled_orders, order_history,
    UnfilledOrder, OrderHistoryItem, OrderHistoryRequest,
};
```

- [ ] **Step 5: `lib.rs` re-export 추가**

```rust
pub use rest::overseas::inquiry::orders::{
    UnfilledOrder, OrderHistoryItem, OrderHistoryRequest,
};
```

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/inquiry/orders.rs \
        crates/kis_api/src/rest/overseas/inquiry/mod.rs \
        crates/kis_api/tests/fixtures/overseas/inquiry/unfilled_orders.json \
        crates/kis_api/tests/fixtures/overseas/inquiry/order_history.json \
        crates/kis_api/src/lib.rs
git commit -m "[plan3a/task4] Implement unfilled_orders() and order_history() for overseas stock"
```

---

## Task 5: `period_profit` + `buyable_amount`

**Files:**
- Create: `crates/kis_api/tests/fixtures/overseas/inquiry/period_profit.json`
- Create: `crates/kis_api/tests/fixtures/overseas/inquiry/buyable_amount.json`
- Create: `crates/kis_api/src/rest/overseas/inquiry/profit.rs`
- Update: `crates/kis_api/src/rest/overseas/inquiry/mod.rs`
- Update: `crates/kis_api/src/lib.rs`

**KIS API:**
- 기간손익: `GET /uapi/overseas-stock/v1/trading/inquire-period-profit` TR ID: `TTTS3039R` (real+VTS 공통)
- 매수가능: `GET /uapi/overseas-stock/v1/trading/inquire-psamount` TR ID: `TTTS3007R` (real+VTS 공통)

---

- [ ] **Step 1: 픽스처 JSON 작성**

파일: `crates/kis_api/tests/fixtures/overseas/inquiry/period_profit.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "조회가 완료되었습니다.",
  "ctx_area_fk200": "",
  "ctx_area_nk200": "",
  "output1": [
    {
      "PDNO": "NVDA",
      "PRDT_NAME": "엔비디아",
      "OVRS_EXCG_CD": "NASD",
      "CCLD_QTY": "3",
      "PCHS_AVG_PRIC": "800.00",
      "CCLD_UNPR": "950.00",
      "RLZT_PFLS": "450.00",
      "PFLS_RT": "18.75"
    }
  ],
  "output2": {
    "OVRS_RLZT_PFLS_AMT": "450.00",
    "OVRS_TOT_PFLS": "450.00"
  }
}
```

파일: `crates/kis_api/tests/fixtures/overseas/inquiry/buyable_amount.json`

```json
{
  "rt_cd": "0",
  "msg_cd": "APBK0013",
  "msg1": "조회가 완료되었습니다.",
  "output": {
    "OVRS_MAX_ORD_PSBL_QTY": "57",
    "EXCC_RAMT": "10050.30",
    "FRCR_BUY_PSBL_AMT": "10050.30"
  }
}
```

- [ ] **Step 2: `profit.rs` — 타입, 테스트, 구현 작성**

파일: `crates/kis_api/src/rest/overseas/inquiry/profit.rs`

```rust
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::NaiveDate;
use crate::{
    config::KisConfig,
    error::KisError,
    rest::overseas::types::Exchange,
};

fn parse_decimal(v: &serde_json::Value, key: &str) -> Decimal {
    v[key].as_str()
        .and_then(|s| Decimal::from_str(s).ok())
        .unwrap_or(Decimal::ZERO)
}

/// 기간손익 종목 항목
pub struct ProfitItem {
    /// 종목코드 (PDNO)
    pub symbol: String,
    /// 종목명 (PRDT_NAME)
    pub name: String,
    /// 거래소코드 (OVRS_EXCG_CD)
    pub exchange: String,
    /// 체결수량 (CCLD_QTY)
    pub filled_qty: Decimal,
    /// 매입평균가 (PCHS_AVG_PRIC)
    pub avg_price: Decimal,
    /// 체결단가 (CCLD_UNPR)
    pub fill_price: Decimal,
    /// 실현손익 (RLZT_PFLS)
    pub realized_pnl: Decimal,
    /// 손익률 (PFLS_RT)
    pub pnl_rate: Decimal,
}

/// 기간손익 요약
pub struct ProfitSummary {
    /// 해외실현손익금액 (OVRS_RLZT_PFLS_AMT)
    pub realized_pnl: Decimal,
    /// 해외총손익금액 (OVRS_TOT_PFLS)
    pub total_pnl: Decimal,
}

/// 기간손익 조회 요청
pub struct PeriodProfitRequest {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

/// 기간손익 조회 응답
pub struct PeriodProfitResponse {
    pub items: Vec<ProfitItem>,
    pub summary: ProfitSummary,
}

/// 매수가능금액 조회 요청
pub struct BuyableAmountRequest {
    pub symbol: String,
    pub exchange: Exchange,
    /// 주문 단가 (지정가: 실제 가격, 시장가: "0")
    pub price: Decimal,
}

/// 매수가능금액 조회 응답
pub struct BuyableAmountResponse {
    /// 해외최대주문가능수량 (OVRS_MAX_ORD_PSBL_QTY)
    pub max_qty: Decimal,
    /// 정산가능금액 (EXCC_RAMT)
    pub buyable_amount: Decimal,
    /// 외화매수가능금액 (FRCR_BUY_PSBL_AMT)
    pub foreign_buyable_amount: Decimal,
}

/// 해외주식 기간손익 조회.
///
/// TR ID: `TTTS3039R` (real/VTS 공통)
pub async fn period_profit(
    config: &KisConfig,
    token: &str,
    req: PeriodProfitRequest,
) -> Result<PeriodProfitResponse, KisError> {
    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let mut query = std::collections::HashMap::new();
    query.insert("CANO".to_string(),            cano.to_string());
    query.insert("ACNT_PRDT_CD".to_string(),    acnt_prdt_cd.to_string());
    query.insert("OVRS_EXCG_CD".to_string(),    "".to_string());
    query.insert("CRCY_CD".to_string(),         "".to_string());
    query.insert("PDNO".to_string(),            "".to_string());
    query.insert("INQR_STRT_DT".to_string(),    req.from.format("%Y%m%d").to_string());
    query.insert("INQR_END_DT".to_string(),     req.to.format("%Y%m%d").to_string());
    query.insert("WCRC_FRCR_DVSN_CD".to_string(), "02".to_string()); // 외화
    query.insert("CTX_AREA_FK200".to_string(),  "".to_string());
    query.insert("CTX_AREA_NK200".to_string(),  "".to_string());

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), "TTTS3039R".to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/inquire-period-profit",
        http::Method::GET,
        headers,
        None,
        Some(query),
    ).await?;

    let items = resp["output1"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| ProfitItem {
            symbol:       item["PDNO"].as_str().unwrap_or("").to_string(),
            name:         item["PRDT_NAME"].as_str().unwrap_or("").to_string(),
            exchange:     item["OVRS_EXCG_CD"].as_str().unwrap_or("").to_string(),
            filled_qty:   parse_decimal(item, "CCLD_QTY"),
            avg_price:    parse_decimal(item, "PCHS_AVG_PRIC"),
            fill_price:   parse_decimal(item, "CCLD_UNPR"),
            realized_pnl: parse_decimal(item, "RLZT_PFLS"),
            pnl_rate:     parse_decimal(item, "PFLS_RT"),
        })
        .collect();

    let out2 = &resp["output2"];
    let summary = ProfitSummary {
        realized_pnl: parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
        total_pnl:    parse_decimal(out2, "OVRS_TOT_PFLS"),
    };

    Ok(PeriodProfitResponse { items, summary })
}

/// 해외주식 매수가능금액 조회.
///
/// TR ID: `TTTS3007R` (real/VTS 공통)
pub async fn buyable_amount(
    config: &KisConfig,
    token: &str,
    req: BuyableAmountRequest,
) -> Result<BuyableAmountResponse, KisError> {
    let parts: Vec<&str> = config.account().splitn(2, '-').collect();
    let (cano, acnt_prdt_cd) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        return Err(KisError::Api {
            code: "INVALID_ACCOUNT".to_string(),
            message: "Account must be in XXXXXXXX-XX format".to_string(),
        });
    };

    let mut query = std::collections::HashMap::new();
    query.insert("CANO".to_string(),           cano.to_string());
    query.insert("ACNT_PRDT_CD".to_string(),   acnt_prdt_cd.to_string());
    query.insert("OVRS_EXCG_CD".to_string(),   req.exchange.to_string());
    query.insert("OVRS_ORD_UNPR".to_string(),  req.price.to_string());
    query.insert("ITEM_CD".to_string(),        req.symbol.clone());

    let mut headers = std::collections::HashMap::new();
    headers.insert("tr_id".to_string(), "TTTS3007R".to_string());

    let resp = crate::rest::http::execute(
        config,
        token,
        "/uapi/overseas-stock/v1/trading/inquire-psamount",
        http::Method::GET,
        headers,
        None,
        Some(query),
    ).await?;

    let output = &resp["output"];

    Ok(BuyableAmountResponse {
        max_qty:                parse_decimal(output, "OVRS_MAX_ORD_PSBL_QTY"),
        buyable_amount:         parse_decimal(output, "EXCC_RAMT"),
        foreign_buyable_amount: parse_decimal(output, "FRCR_BUY_PSBL_AMT"),
    })
}

// ─── 단위 테스트 ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_period_profit_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/period_profit.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();

        let items: Vec<ProfitItem> = v["output1"]
            .as_array().unwrap()
            .iter()
            .map(|item| ProfitItem {
                symbol:       item["PDNO"].as_str().unwrap().to_string(),
                name:         item["PRDT_NAME"].as_str().unwrap().to_string(),
                exchange:     item["OVRS_EXCG_CD"].as_str().unwrap().to_string(),
                filled_qty:   parse_decimal(item, "CCLD_QTY"),
                avg_price:    parse_decimal(item, "PCHS_AVG_PRIC"),
                fill_price:   parse_decimal(item, "CCLD_UNPR"),
                realized_pnl: parse_decimal(item, "RLZT_PFLS"),
                pnl_rate:     parse_decimal(item, "PFLS_RT"),
            })
            .collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].symbol, "NVDA");
        assert!(items[0].realized_pnl > Decimal::ZERO);
        assert!(items[0].pnl_rate > Decimal::ZERO);

        let out2 = &v["output2"];
        let summary = ProfitSummary {
            realized_pnl: parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
            total_pnl:    parse_decimal(out2, "OVRS_TOT_PFLS"),
        };
        assert!(summary.realized_pnl > Decimal::ZERO);
    }

    #[test]
    fn deserialize_buyable_amount_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/buyable_amount.json");
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        let output = &v["output"];

        let resp = BuyableAmountResponse {
            max_qty:                parse_decimal(output, "OVRS_MAX_ORD_PSBL_QTY"),
            buyable_amount:         parse_decimal(output, "EXCC_RAMT"),
            foreign_buyable_amount: parse_decimal(output, "FRCR_BUY_PSBL_AMT"),
        };

        assert!(resp.max_qty > Decimal::ZERO);
        assert!(resp.buyable_amount > Decimal::ZERO);
        assert_eq!(resp.buyable_amount, resp.foreign_buyable_amount);
    }
}
```

- [ ] **Step 3: 테스트 실행**

```bash
cargo test -p kis_api rest::overseas::inquiry::profit 2>&1
```

예상:
```
test rest::overseas::inquiry::profit::tests::deserialize_period_profit_response ... ok
test rest::overseas::inquiry::profit::tests::deserialize_buyable_amount_response ... ok
test result: ok. 2 passed; 0 failed
```

- [ ] **Step 4: `inquiry/mod.rs` 최종 업데이트**

```rust
pub mod balance;
pub mod orders;
pub mod profit;

pub use balance::{balance, BalanceItem, BalanceSummary, BalanceResponse};
pub use orders::{
    unfilled_orders, order_history,
    UnfilledOrder, OrderHistoryItem, OrderHistoryRequest,
};
pub use profit::{
    period_profit, buyable_amount,
    ProfitItem, ProfitSummary, PeriodProfitRequest, PeriodProfitResponse,
    BuyableAmountRequest, BuyableAmountResponse,
};
```

- [ ] **Step 5: `lib.rs` re-export 추가**

```rust
pub use rest::overseas::inquiry::profit::{
    ProfitItem, ProfitSummary, PeriodProfitRequest, PeriodProfitResponse,
    BuyableAmountRequest, BuyableAmountResponse,
};
```

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/inquiry/profit.rs \
        crates/kis_api/src/rest/overseas/inquiry/mod.rs \
        crates/kis_api/tests/fixtures/overseas/inquiry/period_profit.json \
        crates/kis_api/tests/fixtures/overseas/inquiry/buyable_amount.json \
        crates/kis_api/src/lib.rs
git commit -m "[plan3a/task5] Implement period_profit() and buyable_amount() for overseas stock"
```

---

## Task 6: `KisClient` 메서드 연결 + 통합 테스트 헬퍼

**Files:**
- Update: `crates/kis_api/src/client.rs`
- Create: `crates/kis_api/tests/integration/mod.rs`
- Create: `crates/kis_api/tests/integration/order.rs`

---

- [ ] **Step 1: `client.rs` — 7개 메서드 추가**

`KisClient` 의 `impl` 블록에 아래 메서드들을 추가한다. 각 메서드는 `self.inner.token_manager.access_token().await?` 로 토큰을 획득한 후 대응하는 자유함수를 위임 호출한다.

```rust
// ─── 주문 ─────────────────────────────────────────────────────────────────────

use crate::rest::overseas::order::place::{place_order as _place_order, PlaceOrderRequest, PlaceOrderResponse};
use crate::rest::overseas::order::cancel::{cancel_order as _cancel_order, CancelOrderRequest, CancelOrderResponse};
use crate::rest::overseas::inquiry::balance::{balance as _balance, BalanceResponse};
use crate::rest::overseas::inquiry::orders::{
    unfilled_orders as _unfilled_orders, order_history as _order_history,
    UnfilledOrder, OrderHistoryItem, OrderHistoryRequest,
};
use crate::rest::overseas::inquiry::profit::{
    period_profit as _period_profit, buyable_amount as _buyable_amount,
    PeriodProfitRequest, PeriodProfitResponse, BuyableAmountRequest, BuyableAmountResponse,
};

impl KisClient {
    /// 해외주식 매수/매도 주문
    pub async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _place_order(&self.inner.config, &token, req).await
    }

    /// 해외주식 주문 정정/취소
    pub async fn cancel_order(&self, req: CancelOrderRequest) -> Result<CancelOrderResponse, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _cancel_order(&self.inner.config, &token, req).await
    }

    /// 해외주식 잔고 조회
    pub async fn balance(&self) -> Result<BalanceResponse, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _balance(&self.inner.config, &token).await
    }

    /// 해외주식 미체결 주문 조회
    pub async fn unfilled_orders(&self) -> Result<Vec<UnfilledOrder>, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _unfilled_orders(&self.inner.config, &token).await
    }

    /// 해외주식 체결내역 조회
    pub async fn order_history(&self, req: OrderHistoryRequest) -> Result<Vec<OrderHistoryItem>, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _order_history(&self.inner.config, &token, req).await
    }

    /// 해외주식 기간손익 조회
    pub async fn period_profit(&self, req: PeriodProfitRequest) -> Result<PeriodProfitResponse, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _period_profit(&self.inner.config, &token, req).await
    }

    /// 해외주식 매수가능금액 조회
    pub async fn buyable_amount(&self, req: BuyableAmountRequest) -> Result<BuyableAmountResponse, KisError> {
        let token = self.inner.token_manager.access_token().await?;
        _buyable_amount(&self.inner.config, &token, req).await
    }
}
```

> **Note:** `self.inner.token_manager` 와 `self.inner.config` 접근 방식은 Plan 2의 `Arc<Inner>` 구조에 따른다. `Inner` 필드명이 다를 경우 Plan 2 실제 구현에 맞게 조정한다.

- [ ] **Step 2: 컴파일 확인**

```bash
cargo build -p kis_api 2>&1
```

- [ ] **Step 3: 통합 테스트 헬퍼 작성**

파일: `crates/kis_api/tests/integration/mod.rs`

```rust
//! 통합 테스트 헬퍼.
//!
//! `KIS_INTEGRATION_TEST=1` 환경변수가 없으면 테스트를 건너뜀.
//! VTS (모의투자) 계정 정보는 환경변수로 주입한다.

pub mod order;

use kis_api::{KisClient, config::KisConfig};

/// 통합 테스트 skip 매크로.
///
/// ```rust
/// #[tokio::test]
/// async fn test_something() {
///     skip_unless_integration!();
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! skip_unless_integration {
    () => {
        if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" {
            eprintln!("Skipping integration test (KIS_INTEGRATION_TEST != 1)");
            return;
        }
    };
}

/// VTS 모의투자 클라이언트 생성.
///
/// 필요 환경변수:
/// - `VTS_APP_KEY`
/// - `VTS_APP_SECRET`
/// - `VTS_ACCOUNT_NUM` (형식: XXXXXXXX-XX)
/// - `VTS_REST_URL`
pub async fn vts_client() -> KisClient {
    let config = KisConfig::builder()
        .app_key(&std::env::var("VTS_APP_KEY").expect("VTS_APP_KEY not set"))
        .app_secret(&std::env::var("VTS_APP_SECRET").expect("VTS_APP_SECRET not set"))
        .account(&std::env::var("VTS_ACCOUNT_NUM").expect("VTS_ACCOUNT_NUM not set"))
        .rest_url(&std::env::var("VTS_REST_URL").expect("VTS_REST_URL not set"))
        .mock(true)
        .build()
        .expect("Failed to build VTS KisConfig");
    KisClient::new(config)
}
```

파일: `crates/kis_api/tests/integration/order.rs`

```rust
//! 해외주식 주문/계좌 통합 테스트 (VTS 모의투자 기준).
//!
//! 실행: KIS_INTEGRATION_TEST=1 cargo test -p kis_api integration::order

use crate::skip_unless_integration;
use crate::integration::vts_client;
use kis_api::rest::overseas::inquiry::orders::OrderHistoryRequest;
use chrono::NaiveDate;

/// 잔고 조회 — VTS에서 빈 배열 또는 항목 반환, 오류 없어야 함.
#[tokio::test]
async fn test_balance_vts() {
    skip_unless_integration!();
    let client = vts_client().await;
    let result = client.balance().await;
    assert!(result.is_ok(), "balance() failed: {:?}", result.err());
    let resp = result.unwrap();
    // 잔고가 없어도 오류가 아님
    println!("Balance items: {}", resp.items.len());
}

/// 미체결 조회 — VTS에서 빈 배열 또는 항목 반환, 오류 없어야 함.
#[tokio::test]
async fn test_unfilled_orders_vts() {
    skip_unless_integration!();
    let client = vts_client().await;
    let result = client.unfilled_orders().await;
    assert!(result.is_ok(), "unfilled_orders() failed: {:?}", result.err());
    println!("Unfilled orders: {}", result.unwrap().len());
}

/// 체결내역 조회 — 지난 7일 기간 조회, 오류 없어야 함.
#[tokio::test]
async fn test_order_history_vts() {
    skip_unless_integration!();
    let client = vts_client().await;
    let to   = chrono::Local::now().date_naive();
    let from = to - chrono::Duration::days(7);
    let result = client.order_history(OrderHistoryRequest { from, to }).await;
    assert!(result.is_ok(), "order_history() failed: {:?}", result.err());
    println!("Order history items: {}", result.unwrap().len());
}

/// 기간손익 조회 — 지난 30일 기간 조회, 오류 없어야 함.
#[tokio::test]
async fn test_period_profit_vts() {
    skip_unless_integration!();
    let client = vts_client().await;
    let to   = chrono::Local::now().date_naive();
    let from = to - chrono::Duration::days(30);
    let req = kis_api::rest::overseas::inquiry::profit::PeriodProfitRequest { from, to };
    let result = client.period_profit(req).await;
    assert!(result.is_ok(), "period_profit() failed: {:?}", result.err());
}

/// place_order 실제 주문은 VTS에서만 — 잘못된 데이터로 API 오류 수신 확인.
/// 실제 주문을 내지 않고 유효하지 않은 종목코드로 API 오류 응답을 확인함.
#[tokio::test]
#[ignore = "Places real VTS order — run manually"]
async fn test_place_order_vts_invalid_symbol() {
    skip_unless_integration!();
    let client = vts_client().await;
    use kis_api::{PlaceOrderRequest, rest::overseas::types::{Exchange, OrderSide, OrderType}};
    use rust_decimal_macros::dec;
    let req = PlaceOrderRequest {
        symbol:     "INVALID_SYMBOL_XYZ".to_string(),
        exchange:   Exchange::NASD,
        side:       OrderSide::Buy,
        order_type: OrderType::Limit,
        qty:        dec!(1),
        price:      Some(dec!(1.00)),
    };
    let result = client.place_order(req).await;
    // 유효하지 않은 종목이므로 KisError::Api 오류 예상
    assert!(result.is_err(), "Expected error for invalid symbol");
}
```

- [ ] **Step 4: 단위 테스트 전체 실행 확인**

```bash
cargo test -p kis_api 2>&1
```

예상 출력 패턴 (통합 테스트는 skip):
```
running N tests
test rest::overseas::types::tests::... ok
test rest::overseas::order::place::tests::... ok
test rest::overseas::order::cancel::tests::... ok
test rest::overseas::inquiry::balance::tests::... ok
test rest::overseas::inquiry::orders::tests::... ok
test rest::overseas::inquiry::profit::tests::... ok
test result: ok. N passed; 0 failed; 0 ignored
```

- [ ] **Step 5: 통합 테스트 선택 실행 (환경변수 있을 때만)**

```bash
KIS_INTEGRATION_TEST=1 \
VTS_APP_KEY=... \
VTS_APP_SECRET=... \
VTS_ACCOUNT_NUM=XXXXXXXX-XX \
VTS_REST_URL=https://openapivts.koreainvestment.com:29443 \
cargo test -p kis_api integration::order::test_balance_vts -- --nocapture 2>&1
```

- [ ] **Step 6: 최종 커밋**

```bash
git add crates/kis_api/src/client.rs \
        crates/kis_api/tests/integration/mod.rs \
        crates/kis_api/tests/integration/order.rs
git commit -m "[plan3a/task6] Wire 7 methods to KisClient + add integration test helpers"
```

---

## 전체 플랜 완료 검증

모든 태스크 완료 후 아래 명령으로 최종 확인한다.

```bash
# 전체 단위 테스트
cargo test -p kis_api 2>&1

# 전체 빌드 (워크스페이스)
cargo build 2>&1

# 클리피
cargo clippy -p kis_api -- -D warnings 2>&1
```

### 구현 완료 체크리스트

| 파일 | 상태 |
|------|------|
| `rest/overseas/types.rs` | Exchange, OrderSide, OrderType, SubscriptionKind, TR ID 함수 |
| `rest/overseas/order/place.rs` | PlaceOrderRequest/Response, place_order() |
| `rest/overseas/order/cancel.rs` | CancelKind, CancelOrderRequest/Response, cancel_order() |
| `rest/overseas/inquiry/balance.rs` | BalanceItem, BalanceSummary, BalanceResponse, balance() |
| `rest/overseas/inquiry/orders.rs` | UnfilledOrder, OrderHistoryItem, OrderHistoryRequest, unfilled_orders(), order_history() |
| `rest/overseas/inquiry/profit.rs` | ProfitItem, ProfitSummary, PeriodProfitRequest/Response, BuyableAmountRequest/Response, period_profit(), buyable_amount() |
| `client.rs` | 7개 pub async fn 추가 |
| `lib.rs` | 모든 신규 타입 pub use |
| `tests/integration/mod.rs` | skip_unless_integration!(), vts_client() |
| `tests/integration/order.rs` | 통합 테스트 4개 |

### 픽스처 파일 체크리스트

| 파일 | 키 필드 |
|------|---------|
| `tests/fixtures/overseas/order/place_order.json` | output.ORDS_PRCS_DT, KRX_FWDG_ORD_ORGNO, ORDS_PRCS_DTIME |
| `tests/fixtures/overseas/order/cancel_order.json` | output.ORDS_PRCS_DT, KRX_FWDG_ORD_ORGNO, ORDS_PRCS_DTIME |
| `tests/fixtures/overseas/inquiry/balance.json` | output1[], output2{} |
| `tests/fixtures/overseas/inquiry/unfilled_orders.json` | output[] |
| `tests/fixtures/overseas/inquiry/order_history.json` | output[] |
| `tests/fixtures/overseas/inquiry/period_profit.json` | output1[], output2{} |
| `tests/fixtures/overseas/inquiry/buyable_amount.json` | output{} |

---

## 주의사항 및 Plan 2 의존 지점

1. **`rest::http::execute` 시그니처** — Plan 2 구현의 실제 파라미터 순서/타입에 맞게 각 호출부를 조정한다. 이 플랜은 `(config, token, path, method, headers, body, query)` 순서를 가정한다.

2. **`KisClient::Inner` 필드명** — `self.inner.config` 와 `self.inner.token_manager` 접근은 Plan 2 `Arc<Inner>` 실제 구조에 따른다. `inner.config` 가 아닌 `inner.cfg` 같은 다른 이름일 경우 Task 6 Step 1을 그에 맞게 수정한다.

3. **`http` crate import** — `http::Method::GET/POST` 사용을 위해 `Cargo.toml`에 `http = "1"` 이 있어야 한다 (Plan 2 Cargo.toml에 포함됨).

4. **`rust_decimal_macros`** — 테스트에서 `dec!()` 매크로 사용. Plan 2 Cargo.toml에 `rust_decimal_macros = "1"` 이 있어야 한다.

5. **페이지네이션 미처리** — `balance`, `unfilled_orders`, `order_history`, `period_profit` 모두 단일 페이지만 반환한다. `CTX_AREA_FK200/NK200` 반환값으로 다음 페이지가 있을 경우 반복 호출이 필요하나, 이는 별도 `_all()` 변형 함수로 향후 추가한다.

6. **`Exchange::Other` 처리** — `order_tr_id()` 가 `""` 를 반환하는 경우 `place_order()` 에서 `KisError::Api` 로 변환한다. 이는 미지원 거래소에 대한 명시적 오류 처리이다.
