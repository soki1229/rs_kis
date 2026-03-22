# Plan 3c: 해외주식 시세분석 REST API

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `crates/kis_api`의 `rest/overseas/analysis/` 모듈을 구현해 해외주식 시세분석 REST API 7개 엔드포인트를 완성하고, `KisClient`에 공개 메서드로 노출한다.

**Architecture:** Plan 2가 확립한 `KisConfig`, `KisError`, `KisClient`, `rest::http::execute` 위에서 동작한다. Plan 3a가 제공하는 `rest/overseas/types.rs`의 `Exchange`, `OrderSide`, `OrderType`, `SubscriptionKind`를 공유 타입으로 사용한다. Plan 3b가 확립한 `rest/overseas/quote/` 디렉터리 구조와 `exchange_to_price_code()` 헬퍼를 참조한다. 모든 API 함수는 `&KisConfig`, `&str` (access token), Request 구조체를 받아 `Result<_, KisError>`를 반환하는 자유 함수(free function)이고, `KisClient` 메서드는 이 자유 함수를 래핑한다.

**Tech Stack:** Rust 2021, `rust_decimal` (모든 가격/수량), `serde` + `serde_json`, `thiserror` (`KisError`), `tokio` (async), `reqwest` (`KisConfig` 내부)

**선행 조건:**
- Plan 2 완료: `KisConfig`, `KisError`, `KisClient`, `rest::http::execute` 존재
- Plan 3a 완료: `crates/kis_api/src/rest/overseas/types.rs` — `Exchange`, `OrderSide`, `OrderType`, `SubscriptionKind`
- Plan 3b 완료: `crates/kis_api/src/rest/overseas/quote/mod.rs` — `exchange_to_price_code()` 헬퍼

---

## File Map

```
crates/kis_api/src/rest/overseas/
└── analysis/
    ├── mod.rs          CREATE  — pub mod ranking; pub mod market; + re-exports
    ├── ranking.rs      CREATE  — RankingSort, RankingRequest, RankingItem, VolumeSurgeItem
    │                             + price_ranking(), volume_ranking(), volume_surge()
    └── market.rs       CREATE  — VolumePowerItem, HighLowKind, NewHighLowItem, MarketCapItem,
                                  TradeTurnoverItem
                                  + volume_power(), new_highlow(), market_cap(), trade_turnover()

crates/kis_api/src/rest/overseas/mod.rs
    MODIFY  — pub mod analysis 추가

crates/kis_api/src/client.rs
    MODIFY  — price_ranking, volume_ranking, volume_surge, volume_power,
              new_highlow, market_cap, trade_turnover 메서드 추가

crates/kis_api/src/lib.rs
    MODIFY  — analysis 공개 타입 re-export

crates/kis_api/tests/fixtures/overseas/analysis/
    CREATE  — updown_rate.json, trade_vol.json, volume_surge.json,
              volume_power.json, new_highlow.json, market_cap.json, trade_turnover.json
```

---

## KIS API 엔드포인트 요약

| 메서드 | TR ID | Path |
|--------|-------|------|
| `price_ranking` (등락률/거래량/시총 정렬) | HHDFS76280100 | `/uapi/overseas-price/v1/quotations/updown-rate` |
| `volume_ranking` (거래량 순위) | HHDFS76280200 | `/uapi/overseas-price/v1/quotations/trade-vol` |
| `volume_surge` (거래량 급증) | HHDFS76280400 | `/uapi/overseas-price/v1/quotations/volume-surge` |
| `volume_power` (체결강도) | HHDFS76370100 | `/uapi/overseas-price/v1/quotations/volume-power` |
| `new_highlow` (신고/신저가) | HHDFS76280300 | `/uapi/overseas-price/v1/quotations/new-highlow` |
| `market_cap` (시가총액 순위) | HHDFS76260100 | `/uapi/overseas-price/v1/quotations/market-cap` |
| `trade_turnover` (거래회전율) | HHDFS76370200 | `/uapi/overseas-price/v1/quotations/trade-turnover` |

**공통 패턴:**
- 모든 요청: `GET`, 인증 헤더 (`authorization: Bearer <token>`, `appkey`, `appsecret`, `tr_id`)
- 페이징 파라미터: `KEYB("")` (커서, 첫 요청은 빈 문자열), `NREC("20")` (최대 조회 건수)
- `AUTH("")` 고정값 (빈 문자열)
- 응답 구조: `{ "rt_cd": "0", "msg_cd": "...", "msg1": "...", "output": [...] }`
- `rt_cd != "0"` 이면 `KisError::Api { code, message }` 반환

---

## Task 0: `analysis/` 디렉터리 생성

**Files:**
- Create: `crates/kis_api/src/rest/overseas/analysis/mod.rs`
- Modify: `crates/kis_api/src/rest/overseas/mod.rs`

- [ ] **Step 1: `analysis/mod.rs` 생성 (빈 모듈 골격)**

```rust
// crates/kis_api/src/rest/overseas/analysis/mod.rs
pub mod market;
pub mod ranking;

pub use market::{
    HighLowKind, MarketCapItem, NewHighLowItem, TradeTurnoverItem, VolumePowerItem,
};
pub use ranking::{RankingItem, RankingRequest, RankingSort, VolumeSurgeItem};
```

- [ ] **Step 2: `rest/overseas/mod.rs`에 `pub mod analysis` 추가**

기존 `mod.rs`에 아래 줄을 추가한다 (다른 `pub mod` 선언 아래):

```rust
pub mod analysis;
```

- [ ] **Step 3: `cargo check -p kis_api` — 컴파일 확인**

```bash
cargo check -p kis_api
```

Expected: `analysis/ranking.rs`, `analysis/market.rs` 파일 없음으로 컴파일 에러. 다음 태스크에서 해결.

---

## Task 1: `ranking.rs` — 등락률/거래량/거래량급증 순위

**Files:**
- Create: `crates/kis_api/src/rest/overseas/analysis/ranking.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/updown_rate.json`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/trade_vol.json`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/volume_surge.json`

### Step 1: 픽스처 JSON 파일 생성

- [ ] **`updown_rate.json` (등락률 순위 — TR: HHDFS76280100)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "EXCD": "NAS",
      "SYMB": "NVDA",
      "DNAM": "NVIDIA CORP",
      "LAST": "875.40",
      "DIFF": "43.20",
      "RATE": "5.19",
      "TVOL": "48320100",
      "TAMT": "42281127840",
      "MKTC": "2154000000000"
    },
    {
      "EXCD": "NAS",
      "SYMB": "AMD",
      "DNAM": "ADVANCED MICRO DEVICES",
      "LAST": "182.30",
      "DIFF": "8.10",
      "RATE": "4.65",
      "TVOL": "31200400",
      "TAMT": "5687812920",
      "MKTC": "294000000000"
    }
  ]
}
```

- [ ] **`trade_vol.json` (거래량 순위 — TR: HHDFS76280200)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "EXCD": "NAS",
      "SYMB": "TSLA",
      "DNAM": "TESLA INC",
      "LAST": "248.50",
      "DIFF": "-3.20",
      "RATE": "-1.27",
      "TVOL": "112048300",
      "TAMT": "27843993550"
    },
    {
      "EXCD": "NAS",
      "SYMB": "AAPL",
      "DNAM": "APPLE INC",
      "LAST": "178.90",
      "DIFF": "1.30",
      "RATE": "0.73",
      "TVOL": "98320100",
      "TAMT": "17597526090"
    }
  ]
}
```

- [ ] **`volume_surge.json` (거래량 급증 — TR: HHDFS76280400)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "EXCD": "NAS",
      "SYMB": "META",
      "DNAM": "META PLATFORMS INC",
      "LAST": "510.70",
      "DIFF": "22.40",
      "RATE": "4.59",
      "TVOL": "28340200",
      "PRDY_TVOL": "12102300",
      "TVOL_RATE": "134.17"
    },
    {
      "EXCD": "NAS",
      "SYMB": "AMZN",
      "DNAM": "AMAZON.COM INC",
      "LAST": "185.30",
      "DIFF": "6.80",
      "RATE": "3.81",
      "TVOL": "45028100",
      "PRDY_TVOL": "21304200",
      "TVOL_RATE": "111.35"
    }
  ]
}
```

### Step 2: 실패하는 단위 테스트 작성 (TDD Red)

- [ ] **`ranking.rs` — 테스트 블록 먼저 작성**

```rust
// crates/kis_api/src/rest/overseas/analysis/ranking.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture(name: &str) -> String {
        let path = format!(
            "{}/tests/fixtures/overseas/analysis/{}.json",
            env!("CARGO_MANIFEST_DIR"),
            name
        );
        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("fixture not found: {path}"))
    }

    #[test]
    fn deserialize_ranking_response() {
        let json = load_fixture("updown_rate");
        let resp: RankingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.rt_cd, "0");
        assert_eq!(resp.output.len(), 2);
        let first = &resp.output[0];
        assert_eq!(first.symb, "NVDA");
        assert_eq!(first.dnam, "NVIDIA CORP");
        assert_eq!(first.last, dec!(875.40));
        assert_eq!(first.diff, dec!(43.20));
        assert_eq!(first.rate, dec!(5.19));
        assert_eq!(first.tvol, dec!(48320100));
        assert_eq!(first.tamt, dec!(42281127840));
        assert_eq!(first.mktc, Some(dec!(2154000000000)));
    }

    #[test]
    fn ranking_item_from_raw_with_mktc() {
        let raw = RankingRaw {
            excd: "NAS".into(),
            symb: "NVDA".into(),
            dnam: "NVIDIA CORP".into(),
            last: dec!(875.40),
            diff: dec!(43.20),
            rate: dec!(5.19),
            tvol: dec!(48320100),
            tamt: dec!(42281127840),
            mktc: Some(dec!(2154000000000)),
        };
        let item = RankingItem::from(raw);
        assert_eq!(item.exchange, "NAS");
        assert_eq!(item.symbol, "NVDA");
        assert_eq!(item.name, "NVIDIA CORP");
        assert_eq!(item.market_cap, Some(dec!(2154000000000)));
    }

    #[test]
    fn deserialize_trade_vol_response() {
        let json = load_fixture("trade_vol");
        let resp: RankingResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.output.len(), 2);
        // trade_vol output에는 MKTC 없음 — None으로 역직렬화
        assert_eq!(resp.output[0].mktc, None);
        assert_eq!(resp.output[0].symb, "TSLA");
    }

    #[test]
    fn deserialize_volume_surge_response() {
        let json = load_fixture("volume_surge");
        let resp: VolumeSurgeResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.output.len(), 2);
        let first = &resp.output[0];
        assert_eq!(first.symb, "META");
        assert_eq!(first.prdy_tvol, dec!(12102300));
        assert_eq!(first.tvol_rate, dec!(134.17));
    }

    #[test]
    fn ranking_sort_to_gubn() {
        assert_eq!(RankingSort::ChangeRate.to_gubn(), "1");
        assert_eq!(RankingSort::Volume.to_gubn(), "2");
        assert_eq!(RankingSort::MarketCap.to_gubn(), "3");
    }

    #[test]
    fn ranking_request_default_count() {
        let req = RankingRequest {
            exchange: Exchange::NASD,
            sort: RankingSort::ChangeRate,
            count: 20,
        };
        assert_eq!(req.count, 20);
    }
}
```

- [ ] **Step 3: `cargo test -p kis_api rest::overseas::analysis::ranking::tests` — 컴파일 에러 확인 (Red)**

```bash
cargo test -p kis_api "rest::overseas::analysis::ranking::tests" 2>&1 | head -40
```

Expected: 타입 미정의로 컴파일 에러.

### Step 3: 구현 (TDD Green)

- [ ] **`ranking.rs` 전체 구현 작성**

```rust
// crates/kis_api/src/rest/overseas/analysis/ranking.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    config::KisConfig,
    error::KisError,
    rest::{http::execute, overseas::types::Exchange},
};

// ── 공통 타입 ─────────────────────────────────────────────────────────────────

/// 등락률 순위 정렬 기준 (GUBN 파라미터)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingSort {
    /// 등락률 순위 (GUBN = "1")
    ChangeRate,
    /// 거래량 순위 (GUBN = "2")
    Volume,
    /// 시가총액 순위 (GUBN = "3")
    MarketCap,
}

impl RankingSort {
    pub fn to_gubn(self) -> &'static str {
        match self {
            Self::ChangeRate => "1",
            Self::Volume => "2",
            Self::MarketCap => "3",
        }
    }
}

/// `price_ranking()` 요청 파라미터
#[derive(Debug, Clone)]
pub struct RankingRequest {
    pub exchange: Exchange,
    pub sort: RankingSort,
    /// 조회 건수 (1~50, 기본 20)
    pub count: u32,
}

impl Default for RankingRequest {
    fn default() -> Self {
        Self {
            exchange: Exchange::NASD,
            sort: RankingSort::ChangeRate,
            count: 20,
        }
    }
}

/// 등락률/거래량/시가총액 순위 응답 항목
#[derive(Debug, Clone)]
pub struct RankingItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    pub amount: Decimal,
    /// 시가총액 — updown-rate/market-cap에는 있고 trade-vol에는 없음
    pub market_cap: Option<Decimal>,
}

/// 거래량 급증 응답 항목
#[derive(Debug, Clone)]
pub struct VolumeSurgeItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    /// 전일 거래량 (PRDY_TVOL)
    pub prev_volume: Decimal,
    /// 거래량 증가율 (TVOL_RATE)
    pub volume_surge_rate: Decimal,
}

// ── 내부 역직렬화 타입 ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct RankingRaw {
    #[serde(rename = "EXCD")]
    excd: String,
    #[serde(rename = "SYMB")]
    symb: String,
    #[serde(rename = "DNAM")]
    dnam: String,
    #[serde(rename = "LAST", with = "rust_decimal::serde::str")]
    last: Decimal,
    #[serde(rename = "DIFF", with = "rust_decimal::serde::str")]
    diff: Decimal,
    #[serde(rename = "RATE", with = "rust_decimal::serde::str")]
    rate: Decimal,
    #[serde(rename = "TVOL", with = "rust_decimal::serde::str")]
    tvol: Decimal,
    #[serde(rename = "TAMT", with = "rust_decimal::serde::str")]
    tamt: Decimal,
    #[serde(
        rename = "MKTC",
        default,
        deserialize_with = "deserialize_optional_decimal"
    )]
    mktc: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
struct RankingResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<RankingRaw>,
}

#[derive(Debug, Deserialize)]
struct VolumeSurgeRaw {
    #[serde(rename = "EXCD")]
    excd: String,
    #[serde(rename = "SYMB")]
    symb: String,
    #[serde(rename = "DNAM")]
    dnam: String,
    #[serde(rename = "LAST", with = "rust_decimal::serde::str")]
    last: Decimal,
    #[serde(rename = "DIFF", with = "rust_decimal::serde::str")]
    diff: Decimal,
    #[serde(rename = "RATE", with = "rust_decimal::serde::str")]
    rate: Decimal,
    #[serde(rename = "TVOL", with = "rust_decimal::serde::str")]
    tvol: Decimal,
    #[serde(rename = "PRDY_TVOL", with = "rust_decimal::serde::str")]
    prdy_tvol: Decimal,
    #[serde(rename = "TVOL_RATE", with = "rust_decimal::serde::str")]
    tvol_rate: Decimal,
}

#[derive(Debug, Deserialize)]
struct VolumeSurgeResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<VolumeSurgeRaw>,
}

// ── From 변환 ─────────────────────────────────────────────────────────────────

impl From<RankingRaw> for RankingItem {
    fn from(r: RankingRaw) -> Self {
        Self {
            exchange: r.excd,
            symbol: r.symb,
            name: r.dnam,
            last: r.last,
            diff: r.diff,
            rate: r.rate,
            volume: r.tvol,
            amount: r.tamt,
            market_cap: r.mktc,
        }
    }
}

impl From<VolumeSurgeRaw> for VolumeSurgeItem {
    fn from(r: VolumeSurgeRaw) -> Self {
        Self {
            exchange: r.excd,
            symbol: r.symb,
            name: r.dnam,
            last: r.last,
            diff: r.diff,
            rate: r.rate,
            volume: r.tvol,
            prev_volume: r.prdy_tvol,
            volume_surge_rate: r.tvol_rate,
        }
    }
}

// ── 헬퍼 ──────────────────────────────────────────────────────────────────────

/// `""` 빈 문자열이거나 필드가 없으면 `None`, 숫자 문자열이면 `Some(Decimal)`
fn deserialize_optional_decimal<'de, D>(d: D) -> Result<Option<Decimal>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(d)?;
    match s {
        None => Ok(None),
        Some(v) if v.is_empty() => Ok(None),
        Some(v) => v
            .parse::<Decimal>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}

// ── 공개 API 함수 ─────────────────────────────────────────────────────────────

/// 등락률/거래량/시가총액 순위 조회 (TR: HHDFS76280100)
///
/// `req.sort`에 따라 GUBN 파라미터를 자동 설정한다.
pub async fn price_ranking(
    config: &KisConfig,
    token: &str,
    req: RankingRequest,
) -> Result<Vec<RankingItem>, KisError> {
    let excd = req.exchange.to_excd();
    let nrec = req.count.to_string();
    let query = vec![
        ("AUTH", ""),
        ("EXCD", excd),
        ("GUBN", req.sort.to_gubn()),
        ("KEYB", ""),
        ("NREC", &nrec),
    ];

    let body: RankingResponse = execute(
        config,
        token,
        "HHDFS76280100",
        "/uapi/overseas-price/v1/quotations/updown-rate",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body.output.into_iter().map(RankingItem::from).collect())
}

/// 거래량 순위 조회 (TR: HHDFS76280200)
pub async fn volume_ranking(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<RankingItem>, KisError> {
    let excd = exchange.to_excd();
    let nrec = count.to_string();
    let query = vec![("AUTH", ""), ("EXCD", excd), ("KEYB", ""), ("NREC", &nrec)];

    let body: RankingResponse = execute(
        config,
        token,
        "HHDFS76280200",
        "/uapi/overseas-price/v1/quotations/trade-vol",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body.output.into_iter().map(RankingItem::from).collect())
}

/// 거래량 급증 종목 조회 (TR: HHDFS76280400)
pub async fn volume_surge(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<VolumeSurgeItem>, KisError> {
    let excd = exchange.to_excd();
    let nrec = count.to_string();
    let query = vec![("AUTH", ""), ("EXCD", excd), ("KEYB", ""), ("NREC", &nrec)];

    let body: VolumeSurgeResponse = execute(
        config,
        token,
        "HHDFS76280400",
        "/uapi/overseas-price/v1/quotations/volume-surge",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body.output.into_iter().map(VolumeSurgeItem::from).collect())
}
```

- [ ] **Step 4: 테스트 실행 (Green)**

```bash
cargo test -p kis_api "rest::overseas::analysis::ranking::tests"
```

Expected: 6개 PASS.

- [ ] **Step 5: `cargo check -p kis_api`**

```bash
cargo check -p kis_api
```

Expected: 컴파일 성공.

- [ ] **Step 6: 커밋**

```bash
git add \
  crates/kis_api/src/rest/overseas/analysis/mod.rs \
  crates/kis_api/src/rest/overseas/analysis/ranking.rs \
  crates/kis_api/src/rest/overseas/mod.rs \
  crates/kis_api/tests/fixtures/overseas/analysis/updown_rate.json \
  crates/kis_api/tests/fixtures/overseas/analysis/trade_vol.json \
  crates/kis_api/tests/fixtures/overseas/analysis/volume_surge.json
git commit -m "feat(kis_api): add overseas analysis ranking module (price_ranking, volume_ranking, volume_surge)"
```

---

## Task 2: `market.rs` — 체결강도/신고저/시가총액/거래회전율

**Files:**
- Create: `crates/kis_api/src/rest/overseas/analysis/market.rs`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/volume_power.json`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/new_highlow.json`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/market_cap.json`
- Create: `crates/kis_api/tests/fixtures/overseas/analysis/trade_turnover.json`

### Step 1: 픽스처 JSON 파일 생성

- [ ] **`volume_power.json` (체결강도 — TR: HHDFS76370100)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "XYMD": "20260321",
      "TVOL": "48320100",
      "MBVOL": "26540200",
      "MSVOL": "21779900",
      "POWER": "121.89"
    },
    {
      "XYMD": "20260320",
      "TVOL": "35102400",
      "MBVOL": "18204300",
      "MSVOL": "16898100",
      "POWER": "107.73"
    }
  ]
}
```

- [ ] **`new_highlow.json` (신고/신저가 — TR: HHDFS76280300)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "EXCD": "NAS",
      "SYMB": "NVDA",
      "DNAM": "NVIDIA CORP",
      "LAST": "875.40",
      "DIFF": "43.20",
      "RATE": "5.19",
      "TVOL": "48320100",
      "XHGP": "950.00",
      "XLWP": "410.20"
    },
    {
      "EXCD": "NAS",
      "SYMB": "MSFT",
      "DNAM": "MICROSOFT CORP",
      "LAST": "420.80",
      "DIFF": "12.30",
      "RATE": "3.01",
      "TVOL": "22104800",
      "XHGP": "468.35",
      "XLWP": "310.20"
    }
  ]
}
```

- [ ] **`market_cap.json` (시가총액 순위 — TR: HHDFS76260100)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "EXCD": "NAS",
      "SYMB": "AAPL",
      "DNAM": "APPLE INC",
      "LAST": "178.90",
      "DIFF": "1.30",
      "RATE": "0.73",
      "TVOL": "98320100",
      "MKTC": "2750000000000"
    },
    {
      "EXCD": "NAS",
      "SYMB": "MSFT",
      "DNAM": "MICROSOFT CORP",
      "LAST": "420.80",
      "DIFF": "12.30",
      "RATE": "3.01",
      "TVOL": "22104800",
      "MKTC": "3120000000000"
    }
  ]
}
```

- [ ] **`trade_turnover.json` (거래회전율 — TR: HHDFS76370200)**

```json
{
  "rt_cd": "0",
  "msg_cd": "KIOK0000",
  "msg1": "정상처리 되었습니다.",
  "output": [
    {
      "EXCD": "NAS",
      "SYMB": "GME",
      "DNAM": "GAMESTOP CORP",
      "LAST": "12.40",
      "DIFF": "0.80",
      "RATE": "6.90",
      "TVOL": "58204300",
      "TRNO": "18.43"
    },
    {
      "EXCD": "NAS",
      "SYMB": "AMC",
      "DNAM": "AMC ENTERTAINMENT HOLDINGS",
      "LAST": "4.20",
      "DIFF": "0.30",
      "RATE": "7.69",
      "TVOL": "41023800",
      "TRNO": "14.22"
    }
  ]
}
```

### Step 2: 실패하는 단위 테스트 작성 (TDD Red)

- [ ] **`market.rs` — 테스트 블록 먼저 작성**

```rust
// crates/kis_api/src/rest/overseas/analysis/market.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture(name: &str) -> String {
        let path = format!(
            "{}/tests/fixtures/overseas/analysis/{}.json",
            env!("CARGO_MANIFEST_DIR"),
            name
        );
        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("fixture not found: {path}"))
    }

    #[test]
    fn deserialize_volume_power_response() {
        let json = load_fixture("volume_power");
        let resp: VolumePowerResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.rt_cd, "0");
        assert_eq!(resp.output.len(), 2);
        let first = &resp.output[0];
        assert_eq!(first.xymd, "20260321");
        assert_eq!(first.tvol, dec!(48320100));
        assert_eq!(first.mbvol, dec!(26540200));
        assert_eq!(first.msvol, dec!(21779900));
        assert_eq!(first.power, dec!(121.89));
    }

    #[test]
    fn volume_power_item_from_raw() {
        let raw = VolumePowerRaw {
            xymd: "20260321".into(),
            tvol: dec!(48320100),
            mbvol: dec!(26540200),
            msvol: dec!(21779900),
            power: dec!(121.89),
        };
        let item = VolumePowerItem::from(raw);
        assert_eq!(item.date, "20260321");
        assert_eq!(item.buy_volume, dec!(26540200));
        assert_eq!(item.sell_volume, dec!(21779900));
        assert_eq!(item.power, dec!(121.89));
    }

    #[test]
    fn deserialize_new_highlow_response() {
        let json = load_fixture("new_highlow");
        let resp: NewHighLowResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.output.len(), 2);
        let first = &resp.output[0];
        assert_eq!(first.symb, "NVDA");
        assert_eq!(first.xhgp, dec!(950.00));
        assert_eq!(first.xlwp, dec!(410.20));
    }

    #[test]
    fn new_highlow_item_from_raw() {
        let raw = NewHighLowRaw {
            excd: "NAS".into(),
            symb: "NVDA".into(),
            dnam: "NVIDIA CORP".into(),
            last: dec!(875.40),
            diff: dec!(43.20),
            rate: dec!(5.19),
            tvol: dec!(48320100),
            xhgp: dec!(950.00),
            xlwp: dec!(410.20),
        };
        let item = NewHighLowItem::from(raw);
        assert_eq!(item.symbol, "NVDA");
        assert_eq!(item.high_price, dec!(950.00));
        assert_eq!(item.low_price, dec!(410.20));
    }

    #[test]
    fn deserialize_market_cap_response() {
        let json = load_fixture("market_cap");
        let resp: MarketCapResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.output.len(), 2);
        assert_eq!(resp.output[0].symb, "AAPL");
        assert_eq!(resp.output[0].mktc, dec!(2750000000000));
    }

    #[test]
    fn deserialize_trade_turnover_response() {
        let json = load_fixture("trade_turnover");
        let resp: TradeTurnoverResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.output.len(), 2);
        assert_eq!(resp.output[0].symb, "GME");
        assert_eq!(resp.output[0].trno, dec!(18.43));
    }

    #[test]
    fn high_low_kind_to_hlgu() {
        assert_eq!(HighLowKind::High.to_hlgu(), "1");
        assert_eq!(HighLowKind::Low.to_hlgu(), "2");
    }
}
```

- [ ] **Step 3: `cargo test -p kis_api "rest::overseas::analysis::market::tests"` — 컴파일 에러 확인 (Red)**

```bash
cargo test -p kis_api "rest::overseas::analysis::market::tests" 2>&1 | head -40
```

Expected: 타입 미정의로 컴파일 에러.

### Step 3: 구현 (TDD Green)

- [ ] **`market.rs` 전체 구현 작성**

```rust
// crates/kis_api/src/rest/overseas/analysis/market.rs

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{
    config::KisConfig,
    error::KisError,
    rest::{http::execute, overseas::types::Exchange},
};

// ── 공통 타입 ─────────────────────────────────────────────────────────────────

/// 신고/신저가 구분 (HLGU 파라미터)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighLowKind {
    /// 신고가 (HLGU = "1")
    High,
    /// 신저가 (HLGU = "2")
    Low,
}

impl HighLowKind {
    pub fn to_hlgu(self) -> &'static str {
        match self {
            Self::High => "1",
            Self::Low => "2",
        }
    }
}

/// 체결강도 응답 항목
#[derive(Debug, Clone)]
pub struct VolumePowerItem {
    /// 날짜 (XYMD, "YYYYMMDD" 형식)
    pub date: String,
    /// 총 거래량 (TVOL)
    pub volume: Decimal,
    /// 매수 체결량 (MBVOL)
    pub buy_volume: Decimal,
    /// 매도 체결량 (MSVOL)
    pub sell_volume: Decimal,
    /// 체결강도 (POWER)
    pub power: Decimal,
}

/// 신고/신저가 응답 항목
#[derive(Debug, Clone)]
pub struct NewHighLowItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    /// 신고가 (XHGP)
    pub high_price: Decimal,
    /// 신저가 (XLWP)
    pub low_price: Decimal,
}

/// 시가총액 순위 응답 항목
#[derive(Debug, Clone)]
pub struct MarketCapItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    /// 시가총액 (MKTC)
    pub market_cap: Decimal,
}

/// 거래회전율 응답 항목
#[derive(Debug, Clone)]
pub struct TradeTurnoverItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    /// 거래회전율 (TRNO)
    pub turnover_rate: Decimal,
}

// ── 내부 역직렬화 타입 ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct VolumePowerRaw {
    #[serde(rename = "XYMD")]
    xymd: String,
    #[serde(rename = "TVOL", with = "rust_decimal::serde::str")]
    tvol: Decimal,
    #[serde(rename = "MBVOL", with = "rust_decimal::serde::str")]
    mbvol: Decimal,
    #[serde(rename = "MSVOL", with = "rust_decimal::serde::str")]
    msvol: Decimal,
    #[serde(rename = "POWER", with = "rust_decimal::serde::str")]
    power: Decimal,
}

#[derive(Debug, Deserialize)]
struct VolumePowerResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<VolumePowerRaw>,
}

#[derive(Debug, Deserialize)]
struct NewHighLowRaw {
    #[serde(rename = "EXCD")]
    excd: String,
    #[serde(rename = "SYMB")]
    symb: String,
    #[serde(rename = "DNAM")]
    dnam: String,
    #[serde(rename = "LAST", with = "rust_decimal::serde::str")]
    last: Decimal,
    #[serde(rename = "DIFF", with = "rust_decimal::serde::str")]
    diff: Decimal,
    #[serde(rename = "RATE", with = "rust_decimal::serde::str")]
    rate: Decimal,
    #[serde(rename = "TVOL", with = "rust_decimal::serde::str")]
    tvol: Decimal,
    #[serde(rename = "XHGP", with = "rust_decimal::serde::str")]
    xhgp: Decimal,
    #[serde(rename = "XLWP", with = "rust_decimal::serde::str")]
    xlwp: Decimal,
}

#[derive(Debug, Deserialize)]
struct NewHighLowResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<NewHighLowRaw>,
}

#[derive(Debug, Deserialize)]
struct MarketCapRaw {
    #[serde(rename = "EXCD")]
    excd: String,
    #[serde(rename = "SYMB")]
    symb: String,
    #[serde(rename = "DNAM")]
    dnam: String,
    #[serde(rename = "LAST", with = "rust_decimal::serde::str")]
    last: Decimal,
    #[serde(rename = "DIFF", with = "rust_decimal::serde::str")]
    diff: Decimal,
    #[serde(rename = "RATE", with = "rust_decimal::serde::str")]
    rate: Decimal,
    #[serde(rename = "TVOL", with = "rust_decimal::serde::str")]
    tvol: Decimal,
    #[serde(rename = "MKTC", with = "rust_decimal::serde::str")]
    mktc: Decimal,
}

#[derive(Debug, Deserialize)]
struct MarketCapResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<MarketCapRaw>,
}

#[derive(Debug, Deserialize)]
struct TradeTurnoverRaw {
    #[serde(rename = "EXCD")]
    excd: String,
    #[serde(rename = "SYMB")]
    symb: String,
    #[serde(rename = "DNAM")]
    dnam: String,
    #[serde(rename = "LAST", with = "rust_decimal::serde::str")]
    last: Decimal,
    #[serde(rename = "DIFF", with = "rust_decimal::serde::str")]
    diff: Decimal,
    #[serde(rename = "RATE", with = "rust_decimal::serde::str")]
    rate: Decimal,
    #[serde(rename = "TVOL", with = "rust_decimal::serde::str")]
    tvol: Decimal,
    #[serde(rename = "TRNO", with = "rust_decimal::serde::str")]
    trno: Decimal,
}

#[derive(Debug, Deserialize)]
struct TradeTurnoverResponse {
    rt_cd: String,
    msg_cd: String,
    msg1: String,
    output: Vec<TradeTurnoverRaw>,
}

// ── From 변환 ─────────────────────────────────────────────────────────────────

impl From<VolumePowerRaw> for VolumePowerItem {
    fn from(r: VolumePowerRaw) -> Self {
        Self {
            date: r.xymd,
            volume: r.tvol,
            buy_volume: r.mbvol,
            sell_volume: r.msvol,
            power: r.power,
        }
    }
}

impl From<NewHighLowRaw> for NewHighLowItem {
    fn from(r: NewHighLowRaw) -> Self {
        Self {
            exchange: r.excd,
            symbol: r.symb,
            name: r.dnam,
            last: r.last,
            diff: r.diff,
            rate: r.rate,
            volume: r.tvol,
            high_price: r.xhgp,
            low_price: r.xlwp,
        }
    }
}

impl From<MarketCapRaw> for MarketCapItem {
    fn from(r: MarketCapRaw) -> Self {
        Self {
            exchange: r.excd,
            symbol: r.symb,
            name: r.dnam,
            last: r.last,
            diff: r.diff,
            rate: r.rate,
            volume: r.tvol,
            market_cap: r.mktc,
        }
    }
}

impl From<TradeTurnoverRaw> for TradeTurnoverItem {
    fn from(r: TradeTurnoverRaw) -> Self {
        Self {
            exchange: r.excd,
            symbol: r.symb,
            name: r.dnam,
            last: r.last,
            diff: r.diff,
            rate: r.rate,
            volume: r.tvol,
            turnover_rate: r.trno,
        }
    }
}

// ── 공개 API 함수 ─────────────────────────────────────────────────────────────

/// 체결강도 조회 (TR: HHDFS76370100)
///
/// 특정 종목의 날짜별 체결강도 이력을 반환한다. KEYB는 빈 문자열 고정.
pub async fn volume_power(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    symbol: &str,
) -> Result<Vec<VolumePowerItem>, KisError> {
    let excd = exchange.to_excd();
    let query = vec![("AUTH", ""), ("EXCD", excd), ("SYMB", symbol), ("KEYB", "")];

    let body: VolumePowerResponse = execute(
        config,
        token,
        "HHDFS76370100",
        "/uapi/overseas-price/v1/quotations/volume-power",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body.output.into_iter().map(VolumePowerItem::from).collect())
}

/// 신고/신저가 순위 조회 (TR: HHDFS76280300)
pub async fn new_highlow(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    kind: HighLowKind,
    count: u32,
) -> Result<Vec<NewHighLowItem>, KisError> {
    let excd = exchange.to_excd();
    let nrec = count.to_string();
    let query = vec![
        ("AUTH", ""),
        ("EXCD", excd),
        ("HLGU", kind.to_hlgu()),
        ("KEYB", ""),
        ("NREC", &nrec),
    ];

    let body: NewHighLowResponse = execute(
        config,
        token,
        "HHDFS76280300",
        "/uapi/overseas-price/v1/quotations/new-highlow",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body.output.into_iter().map(NewHighLowItem::from).collect())
}

/// 시가총액 순위 조회 (TR: HHDFS76260100)
pub async fn market_cap(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<MarketCapItem>, KisError> {
    let excd = exchange.to_excd();
    let nrec = count.to_string();
    let query = vec![("AUTH", ""), ("EXCD", excd), ("KEYB", ""), ("NREC", &nrec)];

    let body: MarketCapResponse = execute(
        config,
        token,
        "HHDFS76260100",
        "/uapi/overseas-price/v1/quotations/market-cap",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body.output.into_iter().map(MarketCapItem::from).collect())
}

/// 거래회전율 순위 조회 (TR: HHDFS76370200)
pub async fn trade_turnover(
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<TradeTurnoverItem>, KisError> {
    let excd = exchange.to_excd();
    let nrec = count.to_string();
    let query = vec![("AUTH", ""), ("EXCD", excd), ("KEYB", ""), ("NREC", &nrec)];

    let body: TradeTurnoverResponse = execute(
        config,
        token,
        "HHDFS76370200",
        "/uapi/overseas-price/v1/quotations/trade-turnover",
        &query,
    )
    .await?;

    if body.rt_cd != "0" {
        return Err(KisError::Api {
            code: body.msg_cd,
            message: body.msg1,
        });
    }

    Ok(body
        .output
        .into_iter()
        .map(TradeTurnoverItem::from)
        .collect())
}
```

- [ ] **Step 4: 테스트 실행 (Green)**

```bash
cargo test -p kis_api "rest::overseas::analysis::market::tests"
```

Expected: 7개 PASS.

- [ ] **Step 5: analysis 모듈 전체 테스트**

```bash
cargo test -p kis_api "rest::overseas::analysis"
```

Expected: ranking 6개 + market 7개 = 13개 PASS.

- [ ] **Step 6: 커밋**

```bash
git add \
  crates/kis_api/src/rest/overseas/analysis/market.rs \
  crates/kis_api/tests/fixtures/overseas/analysis/volume_power.json \
  crates/kis_api/tests/fixtures/overseas/analysis/new_highlow.json \
  crates/kis_api/tests/fixtures/overseas/analysis/market_cap.json \
  crates/kis_api/tests/fixtures/overseas/analysis/trade_turnover.json
git commit -m "feat(kis_api): add overseas analysis market module (volume_power, new_highlow, market_cap, trade_turnover)"
```

---

## Task 3: `KisClient` 메서드 추가 + 통합 테스트 + lib.rs 공개

**Files:**
- Modify: `crates/kis_api/src/client.rs`
- Modify: `crates/kis_api/src/lib.rs`
- Create: `crates/kis_api/tests/integration_analysis.rs`

### Step 1: `KisClient` 메서드 추가

- [ ] **`client.rs`에 analysis 메서드 블록 추가**

`KisClient` impl 블록에 아래 메서드를 추가한다. 각 메서드는 `TokenManager`에서 토큰을 얻어 자유 함수로 위임한다.

```rust
// client.rs — impl KisClient 블록 내부에 추가

use crate::rest::overseas::analysis::{
    market::{self, HighLowKind, MarketCapItem, NewHighLowItem, TradeTurnoverItem, VolumePowerItem},
    ranking::{self, RankingItem, RankingRequest, VolumeSurgeItem},
};

impl KisClient {
    // ── 시세분석 ─────────────────────────────────────────────────────────────

    /// 등락률/거래량/시가총액 순위 (TR: HHDFS76280100)
    pub async fn price_ranking(
        &self,
        req: RankingRequest,
    ) -> Result<Vec<RankingItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        ranking::price_ranking(&self.inner.config, &token, req).await
    }

    /// 거래량 순위 (TR: HHDFS76280200)
    pub async fn volume_ranking(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<RankingItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        ranking::volume_ranking(&self.inner.config, &token, exchange, count).await
    }

    /// 거래량 급증 (TR: HHDFS76280400)
    pub async fn volume_surge(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<VolumeSurgeItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        ranking::volume_surge(&self.inner.config, &token, exchange, count).await
    }

    /// 체결강도 (TR: HHDFS76370100)
    pub async fn volume_power(
        &self,
        exchange: &Exchange,
        symbol: &str,
    ) -> Result<Vec<VolumePowerItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        market::volume_power(&self.inner.config, &token, exchange, symbol).await
    }

    /// 신고/신저가 순위 (TR: HHDFS76280300)
    pub async fn new_highlow(
        &self,
        exchange: &Exchange,
        kind: HighLowKind,
        count: u32,
    ) -> Result<Vec<NewHighLowItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        market::new_highlow(&self.inner.config, &token, exchange, kind, count).await
    }

    /// 시가총액 순위 (TR: HHDFS76260100)
    pub async fn market_cap(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<MarketCapItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        market::market_cap(&self.inner.config, &token, exchange, count).await
    }

    /// 거래회전율 순위 (TR: HHDFS76370200)
    pub async fn trade_turnover(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<TradeTurnoverItem>, KisError> {
        let token = self.inner.token_manager.get_token(&self.inner.config).await?;
        market::trade_turnover(&self.inner.config, &token, exchange, count).await
    }
}
```

- [ ] **Step 2: `cargo check -p kis_api`**

```bash
cargo check -p kis_api
```

Expected: 컴파일 성공.

### Step 2: `lib.rs` 공개 타입 추가

- [ ] **`lib.rs`에 analysis 타입 re-export 추가**

```rust
// lib.rs — 기존 pub use 블록에 추가

pub use rest::overseas::analysis::{
    market::{HighLowKind, MarketCapItem, NewHighLowItem, TradeTurnoverItem, VolumePowerItem},
    ranking::{RankingItem, RankingRequest, RankingSort, VolumeSurgeItem},
};
```

### Step 3: 통합 테스트 작성

- [ ] **`tests/integration_analysis.rs` 생성**

```rust
// crates/kis_api/tests/integration_analysis.rs
//
// 통합 테스트 — KIS_INTEGRATION_TEST=1 일 때만 실행
// VTS(모의투자)가 시세분석 API를 지원하지 않으므로 실전 환경 또는 #[ignore] 처리.

macro_rules! skip_unless_integration {
    () => {
        if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" {
            return;
        }
    };
}

/// 실전 환경이 필요한 테스트 — VTS 미지원
macro_rules! skip_unless_prod_integration {
    () => {
        if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" {
            return;
        }
        if std::env::var("KIS_USE_PROD").unwrap_or_default() != "1" {
            eprintln!("Skipping: KIS_USE_PROD=1 required (VTS does not support analysis APIs)");
            return;
        }
    };
}

#[tokio::test]
async fn integration_price_ranking_nasd() {
    skip_unless_prod_integration!();

    use kis_api::{KisClient, KisConfig, RankingRequest, RankingSort};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let req = RankingRequest {
        exchange: Exchange::NASD,
        sort: RankingSort::ChangeRate,
        count: 5,
    };

    let result = client.price_ranking(req).await;
    assert!(result.is_ok(), "price_ranking failed: {:?}", result.err());

    let items = result.unwrap();
    assert!(!items.is_empty(), "expected at least one ranking item");

    let first = &items[0];
    assert!(!first.symbol.is_empty());
    assert!(!first.name.is_empty());
    // 가격은 양수
    assert!(first.last > rust_decimal::Decimal::ZERO);
}

#[tokio::test]
async fn integration_volume_ranking_nasd() {
    skip_unless_prod_integration!();

    use kis_api::{KisClient, KisConfig};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let result = client.volume_ranking(&Exchange::NASD, 5).await;
    assert!(result.is_ok(), "volume_ranking failed: {:?}", result.err());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
#[ignore = "VTS does not support volume_surge API"]
async fn integration_volume_surge_nasd() {
    use kis_api::{KisClient, KisConfig};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let result = client.volume_surge(&Exchange::NASD, 5).await;
    assert!(result.is_ok(), "volume_surge failed: {:?}", result.err());
}

#[tokio::test]
#[ignore = "VTS does not support volume_power API"]
async fn integration_volume_power_aapl() {
    use kis_api::{KisClient, KisConfig};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let result = client.volume_power(&Exchange::NASD, "AAPL").await;
    assert!(result.is_ok(), "volume_power failed: {:?}", result.err());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
#[ignore = "VTS does not support new_highlow API"]
async fn integration_new_highlow_nasd() {
    use kis_api::{HighLowKind, KisClient, KisConfig};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let result = client.new_highlow(&Exchange::NASD, HighLowKind::High, 5).await;
    assert!(result.is_ok(), "new_highlow failed: {:?}", result.err());
}

#[tokio::test]
#[ignore = "VTS does not support market_cap API"]
async fn integration_market_cap_nasd() {
    use kis_api::{KisClient, KisConfig};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let result = client.market_cap(&Exchange::NASD, 5).await;
    assert!(result.is_ok(), "market_cap failed: {:?}", result.err());
}

#[tokio::test]
#[ignore = "VTS does not support trade_turnover API"]
async fn integration_trade_turnover_nasd() {
    use kis_api::{KisClient, KisConfig};
    use kis_api::rest::overseas::types::Exchange;

    let config = KisConfig::from_env().expect("KisConfig::from_env failed");
    let client = KisClient::new(config);

    let result = client.trade_turnover(&Exchange::NASD, 5).await;
    assert!(result.is_ok(), "trade_turnover failed: {:?}", result.err());
}
```

### Step 4: 전체 단위 테스트 실행

- [ ] **단위 테스트 전체 실행**

```bash
cargo test -p kis_api "rest::overseas::analysis"
```

Expected: 13개 PASS (ranking 6 + market 7).

- [ ] **통합 테스트 구조 확인 (실행 안 함)**

```bash
cargo test -p kis_api --test integration_analysis -- --list
```

Expected: 7개 테스트 항목 목록 출력 (실행 안 됨).

### Step 5: 커밋

- [ ] **최종 커밋**

```bash
git add \
  crates/kis_api/src/client.rs \
  crates/kis_api/src/lib.rs \
  crates/kis_api/tests/integration_analysis.rs
git commit -m "feat(kis_api): expose analysis methods on KisClient, add integration tests"
```

---

## 전체 파일 체크리스트

```
신규 생성:
  crates/kis_api/src/rest/overseas/analysis/mod.rs
  crates/kis_api/src/rest/overseas/analysis/ranking.rs
  crates/kis_api/src/rest/overseas/analysis/market.rs
  crates/kis_api/tests/fixtures/overseas/analysis/updown_rate.json
  crates/kis_api/tests/fixtures/overseas/analysis/trade_vol.json
  crates/kis_api/tests/fixtures/overseas/analysis/volume_surge.json
  crates/kis_api/tests/fixtures/overseas/analysis/volume_power.json
  crates/kis_api/tests/fixtures/overseas/analysis/new_highlow.json
  crates/kis_api/tests/fixtures/overseas/analysis/market_cap.json
  crates/kis_api/tests/fixtures/overseas/analysis/trade_turnover.json
  crates/kis_api/tests/integration_analysis.rs

수정:
  crates/kis_api/src/rest/overseas/mod.rs  (pub mod analysis 추가)
  crates/kis_api/src/client.rs             (7개 메서드 추가)
  crates/kis_api/src/lib.rs                (analysis 타입 re-export)
```

## 테스트 커맨드 요약

```bash
# Task 1 단위 테스트
cargo test -p kis_api "rest::overseas::analysis::ranking::tests"

# Task 2 단위 테스트
cargo test -p kis_api "rest::overseas::analysis::market::tests"

# analysis 모듈 전체 단위 테스트
cargo test -p kis_api "rest::overseas::analysis"

# 통합 테스트 목록 확인 (실행 안 됨)
cargo test -p kis_api --test integration_analysis -- --list

# 통합 테스트 실행 (실전 환경)
KIS_INTEGRATION_TEST=1 KIS_USE_PROD=1 cargo test -p kis_api --test integration_analysis

# #[ignore] 포함 통합 테스트 실행 (실전 환경)
KIS_INTEGRATION_TEST=1 KIS_USE_PROD=1 cargo test -p kis_api --test integration_analysis -- --ignored
```

## 설계 노트

**`Exchange::to_excd()` 의존성**

`Exchange` enum의 `to_excd() -> &str` 메서드는 Plan 3a에서 구현되어야 한다. 이 메서드는 `Exchange::NASD` → `"NAS"`, `Exchange::NYSE` → `"NYS"` 등 KIS API 코드로 변환한다. Plan 3b의 `exchange_to_price_code()` 헬퍼와 역할이 겹치면 `to_excd()`로 통일하거나 헬퍼를 재사용한다.

**`MKTC` 필드 옵셔널 처리**

`RankingItem.market_cap`이 `Option<Decimal>`인 이유: `updown-rate` (GUBN="1","2")와 `trade-vol` 응답에는 `MKTC` 필드가 없거나 빈 문자열로 올 수 있다. `deserialize_optional_decimal` 헬퍼로 통일 처리한다.

**체결강도 (`volume_power`) 시그니처**

KIS API는 `SYMB` 파라미터를 요구하므로 다른 함수와 달리 `symbol: &str`을 받는다. `KisClient::volume_power(&exchange, symbol)`로 노출.

**VTS 미지원 처리**

시세분석 API (`analysis/`) 전체는 VTS 모의투자 환경에서 동작이 보장되지 않는다. 통합 테스트는 `KIS_USE_PROD=1` 환경변수 추가 확인으로 이중 보호한다. CI 환경에서 자동 실행되지 않도록 `#[ignore]`를 기본으로 적용한다.
