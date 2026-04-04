use reqwest::Method;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::str::FromStr;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;
use crate::{KisConfig, KisError};

/// 순위 정렬 기준
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankingSort {
    ChangeRate,
    Volume,
    MarketCap,
}

impl RankingSort {
    pub fn to_gubn(&self) -> &'static str {
        match self {
            RankingSort::ChangeRate => "1",
            RankingSort::Volume => "2",
            RankingSort::MarketCap => "3",
        }
    }
}

/// 순위 조회 요청
#[derive(Debug, Clone)]
pub struct RankingRequest {
    pub exchange: Exchange,
    pub sort: RankingSort,
    pub count: u32,
}

/// 순위 항목
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
    pub market_cap: Option<Decimal>,
}

/// 거래량 급증 항목
#[derive(Debug, Clone)]
pub struct VolumeSurgeItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    pub prev_volume: Decimal,
    pub volume_surge_rate: Decimal,
}

fn parse_decimal(v: &Value, field: &str) -> Result<Decimal, KisError> {
    let s = v[field].as_str().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: format!("missing field: {field}"),
    })?;
    Decimal::from_str(s).map_err(|e| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: format!("decimal parse error for {field}: {e}"),
    })
}

fn parse_optional_decimal(v: &Value, field: &str) -> Option<Decimal> {
    v[field]
        .as_str()
        .filter(|s| !s.is_empty())
        .and_then(|s| Decimal::from_str(s).ok())
}

fn parse_ranking_item(item: &Value) -> Result<RankingItem, KisError> {
    let symbol = item["symb"].as_str().unwrap_or("").to_string();
    if symbol.is_empty() {
        return Err(KisError::Api {
            code: "PARSE_ERR".to_string(),
            message: "missing symbol in ranking item".to_string(),
        });
    }
    Ok(RankingItem {
        exchange: item["excd"].as_str().unwrap_or("").to_string(),
        symbol,
        name: item["name"].as_str().unwrap_or("").to_string(),
        // last/diff/rate/volume may be empty strings on weekends/after-hours
        last: parse_optional_decimal(item, "last").unwrap_or(rust_decimal::Decimal::ZERO),
        diff: parse_optional_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
        rate: parse_optional_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
        volume: parse_optional_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
        amount: parse_optional_decimal(item, "tamt").unwrap_or(rust_decimal::Decimal::ZERO),
        market_cap: parse_optional_decimal(item, "tomv")
            .or_else(|| parse_optional_decimal(item, "mcap")),
    })
}

/// 해외주식 등락률 순위 조회 [해외주식-041]
/// TR-ID: HHDFS76290000
/// `req.sort`: ChangeRate=상승률(gubn="1"), else=하락률(gubn="0")
pub async fn price_ranking(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: RankingRequest,
) -> Result<Vec<RankingItem>, KisError> {
    let gubn = match req.sort {
        RankingSort::ChangeRate => "1",
        _ => "0",
    };
    let query = json!({
        "AUTH": "",
        "EXCD": req.exchange.to_price_code(),
        "NDAY": "0",
        "GUBN": gubn,
        "VOL_RANG": "0",
        "KEYB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/updown-rate",
            tr_id: "HHDFS76290000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let items: Result<Vec<RankingItem>, KisError> = arr.iter().map(parse_ranking_item).collect();
    items.map(|mut v| {
        v.truncate(req.count as usize);
        v
    })
}

/// 해외주식 거래량 순위 조회 [해외주식-043]
/// TR-ID: HHDFS76310010
pub async fn volume_ranking(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<RankingItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_price_code(),
        "NDAY": "0",
        "VOL_RANG": "0",
        "KEYB": "",
        "PRC1": "",
        "PRC2": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/trade-vol",
            tr_id: "HHDFS76310010",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let items: Result<Vec<RankingItem>, KisError> = arr.iter().map(parse_ranking_item).collect();
    items.map(|mut v| {
        v.truncate(count as usize);
        v
    })
}

/// 해외주식 거래량 급증 순위 조회 [해외주식-045]
/// TR-ID: HHDFS76270000
pub async fn volume_surge(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<VolumeSurgeItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_price_code(),
        "MINX": "0",
        "VOL_RANG": "0",
        "KEYB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/volume-surge",
            tr_id: "HHDFS76270000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let mut items: Vec<VolumeSurgeItem> = arr
        .iter()
        .filter_map(|item| {
            Some(VolumeSurgeItem {
                exchange: item["excd"].as_str().unwrap_or("").to_string(),
                symbol: item["symb"].as_str().unwrap_or("").to_string(),
                name: item["name"]
                    .as_str()
                    .or_else(|| item["knam"].as_str())
                    .unwrap_or("")
                    .to_string(),
                last: parse_optional_decimal(item, "last")?,
                diff: parse_optional_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                rate: parse_optional_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                volume: parse_optional_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                prev_volume: parse_optional_decimal(item, "n_tvol")
                    .unwrap_or(rust_decimal::Decimal::ZERO),
                volume_surge_rate: parse_optional_decimal(item, "n_rate")
                    .or_else(|| parse_optional_decimal(item, "trat"))
                    .unwrap_or(rust_decimal::Decimal::ZERO),
            })
        })
        .collect();
    items.truncate(count as usize);
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture(name: &str) -> Value {
        let path = format!(
            "{}/tests/fixtures/overseas/analysis/{}.json",
            env!("CARGO_MANIFEST_DIR"),
            name
        );
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture not found: {path}"));
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn ranking_sort_to_gubn() {
        assert_eq!(RankingSort::ChangeRate.to_gubn(), "1");
        assert_eq!(RankingSort::Volume.to_gubn(), "2");
        assert_eq!(RankingSort::MarketCap.to_gubn(), "3");
    }

    #[test]
    fn deserialize_updown_rate() {
        let v = load_fixture("updown_rate");
        let items: Vec<RankingItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| parse_ranking_item(item).unwrap())
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "NVDA");
        assert_eq!(first.exchange, "NAS");
        assert_eq!(first.last, dec!(875.40));
        assert_eq!(first.diff, dec!(43.20));
        assert_eq!(first.rate, dec!(5.19));
        assert_eq!(first.volume, dec!(48320100));
        assert!(first.market_cap.is_some());
        assert_eq!(first.market_cap.unwrap(), dec!(2154000000000));
    }

    #[test]
    fn deserialize_trade_vol() {
        let v = load_fixture("trade_vol");
        let items: Vec<RankingItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| parse_ranking_item(item).unwrap())
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "TSLA");
        assert_eq!(first.last, dec!(248.50));
        // tomv 필드 없음 → None
        assert!(first.market_cap.is_none());
    }

    #[test]
    fn deserialize_volume_surge() {
        let v = load_fixture("volume_surge");
        let items: Vec<VolumeSurgeItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| {
                Some(VolumeSurgeItem {
                    exchange: item["excd"].as_str().unwrap_or("").to_string(),
                    symbol: item["symb"].as_str().unwrap_or("").to_string(),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    last: parse_optional_decimal(item, "last")?,
                    diff: parse_optional_decimal(item, "diff")
                        .unwrap_or(rust_decimal::Decimal::ZERO),
                    rate: parse_optional_decimal(item, "rate")
                        .unwrap_or(rust_decimal::Decimal::ZERO),
                    volume: parse_optional_decimal(item, "tvol")
                        .unwrap_or(rust_decimal::Decimal::ZERO),
                    prev_volume: parse_optional_decimal(item, "n_tvol")
                        .unwrap_or(rust_decimal::Decimal::ZERO),
                    volume_surge_rate: parse_optional_decimal(item, "n_rate")
                        .unwrap_or(rust_decimal::Decimal::ZERO),
                })
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "META");
        assert_eq!(first.last, dec!(510.70));
        assert_eq!(first.prev_volume, dec!(12102300));
        assert_eq!(first.volume_surge_rate, dec!(134.17));
    }
}
