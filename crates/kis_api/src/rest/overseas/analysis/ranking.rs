use reqwest::Method;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::{Value, json};

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;

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
    Ok(RankingItem {
        exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
        symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
        name: item["DNAM"].as_str().unwrap_or("").to_string(),
        last: parse_decimal(item, "LAST")?,
        diff: parse_decimal(item, "DIFF")?,
        rate: parse_decimal(item, "RATE")?,
        volume: parse_decimal(item, "TVOL")?,
        amount: parse_decimal(item, "TAMT")?,
        market_cap: parse_optional_decimal(item, "MKTC"),
    })
}

/// 해외주식 등락률 순위 조회
pub async fn price_ranking(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: RankingRequest,
) -> Result<Vec<RankingItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": req.exchange.to_string(),
        "GUBN": req.sort.to_gubn(),
        "KEYB": "",
        "NREC": req.count.to_string(),
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/updown-rate",
            tr_id: "HHDFS76280100",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    v["output"]
        .as_array()
        .ok_or_else(|| KisError::Api {
            code: "PARSE_ERR".to_string(),
            message: "missing output array".to_string(),
        })?
        .iter()
        .map(parse_ranking_item)
        .collect()
}

/// 해외주식 거래량 순위 조회
pub async fn volume_ranking(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<RankingItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "KEYB": "",
        "NREC": count.to_string(),
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/trade-vol",
            tr_id: "HHDFS76280200",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    v["output"]
        .as_array()
        .ok_or_else(|| KisError::Api {
            code: "PARSE_ERR".to_string(),
            message: "missing output array".to_string(),
        })?
        .iter()
        .map(parse_ranking_item)
        .collect()
}

/// 해외주식 거래량 급증 순위 조회
pub async fn volume_surge(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<VolumeSurgeItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "KEYB": "",
        "NREC": count.to_string(),
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/volume-surge",
            tr_id: "HHDFS76280400",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    v["output"]
        .as_array()
        .ok_or_else(|| KisError::Api {
            code: "PARSE_ERR".to_string(),
            message: "missing output array".to_string(),
        })?
        .iter()
        .map(|item| {
            Ok(VolumeSurgeItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST")?,
                diff: parse_decimal(item, "DIFF")?,
                rate: parse_decimal(item, "RATE")?,
                volume: parse_decimal(item, "TVOL")?,
                prev_volume: parse_decimal(item, "PRDY_TVOL")?,
                volume_surge_rate: parse_decimal(item, "TVOL_RATE")?,
            })
        })
        .collect()
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
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("fixture not found: {path}"));
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
        let items: Vec<RankingItem> = v["output"]
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
        let items: Vec<RankingItem> = v["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| parse_ranking_item(item).unwrap())
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "TSLA");
        assert_eq!(first.last, dec!(248.50));
        // MKTC 필드 없음 → None
        assert!(first.market_cap.is_none());
    }

    #[test]
    fn deserialize_volume_surge() {
        let v = load_fixture("volume_surge");
        let items: Vec<VolumeSurgeItem> = v["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| {
                Ok::<VolumeSurgeItem, KisError>(VolumeSurgeItem {
                    exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                    symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                    name: item["DNAM"].as_str().unwrap_or("").to_string(),
                    last: parse_decimal(item, "LAST").unwrap(),
                    diff: parse_decimal(item, "DIFF").unwrap(),
                    rate: parse_decimal(item, "RATE").unwrap(),
                    volume: parse_decimal(item, "TVOL").unwrap(),
                    prev_volume: parse_decimal(item, "PRDY_TVOL").unwrap(),
                    volume_surge_rate: parse_decimal(item, "TVOL_RATE").unwrap(),
                })
            })
            .map(|r| r.unwrap())
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "META");
        assert_eq!(first.last, dec!(510.70));
        assert_eq!(first.prev_volume, dec!(12102300));
        assert_eq!(first.volume_surge_rate, dec!(134.17));
    }
}
