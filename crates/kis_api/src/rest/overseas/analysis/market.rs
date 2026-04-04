use reqwest::Method;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::str::FromStr;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;
use crate::{KisConfig, KisError};

/// 체결강도 항목
#[derive(Debug, Clone)]
pub struct VolumePowerItem {
    pub date: String,
    pub total_volume: Decimal,
    pub buy_volume: Decimal,
    pub sell_volume: Decimal,
    pub power: Decimal,
}

/// 신고가/신저가 항목
#[derive(Debug, Clone)]
pub struct NewHighLowItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
}

/// 신고가/신저가 구분
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighLowKind {
    High,
    Low,
}

impl HighLowKind {
    pub fn to_hlgu(&self) -> &'static str {
        match self {
            HighLowKind::High => "1",
            HighLowKind::Low => "2",
        }
    }
}

/// 시가총액 항목
#[derive(Debug, Clone)]
pub struct MarketCapItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    pub market_cap: Decimal,
}

/// 거래회전율 항목
#[derive(Debug, Clone)]
pub struct TradeTurnoverItem {
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub last: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    pub turnover_rate: Decimal,
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

/// 해외주식 체결강도 순위 조회 [해외주식-046]
/// TR-ID: HHDFS76280000
/// (API 개편: 종목별 체결강도→거래소 전체 체결강도 순위로 변경)
pub async fn volume_power(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<VolumePowerItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "NDAY": "0",
        "VOL_RANG": "0",
        "KEYB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/volume-power",
            tr_id: "HHDFS76280000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let mut items: Vec<VolumePowerItem> = arr
        .iter()
        .filter_map(|item| {
            Some(VolumePowerItem {
                date: item["excd"].as_str().unwrap_or("").to_string(),
                total_volume: parse_decimal(item, "tvol").ok()?,
                buy_volume: parse_decimal(item, "asvl").unwrap_or(rust_decimal::Decimal::ZERO),
                sell_volume: parse_decimal(item, "bivl").unwrap_or(rust_decimal::Decimal::ZERO),
                power: parse_decimal(item, "strn")
                    .or_else(|_| parse_decimal(item, "tpow"))
                    .unwrap_or(rust_decimal::Decimal::ZERO),
            })
        })
        .collect();
    items.truncate(count as usize);
    Ok(items)
}

/// 해외주식 신고가/신저가 순위 조회 [해외주식-044]
/// TR-ID: HHDFS76300000
/// `kind`: High=상승(gubn="1"), Low=하락(gubn="0")
pub async fn new_highlow(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    kind: &HighLowKind,
    count: u32,
) -> Result<Vec<NewHighLowItem>, KisError> {
    let gubn = kind.to_hlgu(); // "1"=신고가, "2"=신저가 (재사용)
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "MINX": "0",
        "VOL_RANG": "0",
        "GUBN": gubn,
        "KEYB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/new-highlow",
            tr_id: "HHDFS76300000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let mut items: Vec<NewHighLowItem> = arr
        .iter()
        .filter_map(|item| {
            Some(NewHighLowItem {
                exchange: item["excd"].as_str().unwrap_or("").to_string(),
                symbol: item["symb"].as_str().unwrap_or("").to_string(),
                name: item["name"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "last").ok()?,
                diff: parse_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                rate: parse_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                volume: parse_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                high_price: parse_decimal(item, "nhgh").unwrap_or(rust_decimal::Decimal::ZERO),
                low_price: parse_decimal(item, "nlow").unwrap_or(rust_decimal::Decimal::ZERO),
            })
        })
        .collect();
    items.truncate(count as usize);
    Ok(items)
}

/// 해외주식 시가총액 순위 조회 [해외주식-042]
/// TR-ID: HHDFS76350100
pub async fn market_cap(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<MarketCapItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "VOL_RANG": "0",
        "KEYB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/market-cap",
            tr_id: "HHDFS76350100",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let mut items: Vec<MarketCapItem> = arr
        .iter()
        .filter_map(|item| {
            Some(MarketCapItem {
                exchange: item["excd"].as_str().unwrap_or("").to_string(),
                symbol: item["symb"].as_str().unwrap_or("").to_string(),
                name: item["name"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "last").ok()?,
                diff: parse_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                rate: parse_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                volume: parse_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                market_cap: parse_decimal(item, "tomv")
                    .or_else(|_| parse_decimal(item, "mcap"))
                    .unwrap_or(rust_decimal::Decimal::ZERO),
            })
        })
        .collect();
    items.truncate(count as usize);
    Ok(items)
}

/// 해외주식 거래회전율 순위 조회 [해외주식-048]
/// TR-ID: HHDFS76340000
pub async fn trade_turnover(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    count: u32,
) -> Result<Vec<TradeTurnoverItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "NDAY": "0",
        "VOL_RANG": "0",
        "KEYB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-stock/v1/ranking/trade-turnover",
            tr_id: "HHDFS76340000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "missing output2 array".to_string(),
    })?;

    let mut items: Vec<TradeTurnoverItem> = arr
        .iter()
        .filter_map(|item| {
            Some(TradeTurnoverItem {
                exchange: item["excd"].as_str().unwrap_or("").to_string(),
                symbol: item["symb"].as_str().unwrap_or("").to_string(),
                name: item["name"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "last").ok()?,
                diff: parse_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                rate: parse_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                volume: parse_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                turnover_rate: parse_decimal(item, "tover")
                    .or_else(|_| parse_decimal(item, "trat"))
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
    fn high_low_kind_to_hlgu() {
        assert_eq!(HighLowKind::High.to_hlgu(), "1");
        assert_eq!(HighLowKind::Low.to_hlgu(), "2");
    }

    #[test]
    fn deserialize_volume_power() {
        let v = load_fixture("volume_power");
        let items: Vec<VolumePowerItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| {
                Some(VolumePowerItem {
                    date: item["excd"].as_str().unwrap_or("").to_string(),
                    total_volume: parse_decimal(item, "tvol").ok()?,
                    buy_volume: parse_decimal(item, "asvl").unwrap_or(rust_decimal::Decimal::ZERO),
                    sell_volume: parse_decimal(item, "bivl").unwrap_or(rust_decimal::Decimal::ZERO),
                    power: parse_decimal(item, "strn").unwrap_or(rust_decimal::Decimal::ZERO),
                })
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.date, "NAS");
        assert_eq!(first.buy_volume, dec!(26540200));
        assert_eq!(first.sell_volume, dec!(21779900));
        assert_eq!(first.power, dec!(121.89));
    }

    #[test]
    fn deserialize_new_highlow() {
        let v = load_fixture("new_highlow");
        let items: Vec<NewHighLowItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| {
                Some(NewHighLowItem {
                    exchange: item["excd"].as_str().unwrap_or("").to_string(),
                    symbol: item["symb"].as_str().unwrap_or("").to_string(),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    last: parse_decimal(item, "last").ok()?,
                    diff: parse_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                    rate: parse_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                    volume: parse_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                    high_price: parse_decimal(item, "nhgh").unwrap_or(rust_decimal::Decimal::ZERO),
                    low_price: parse_decimal(item, "nlow").unwrap_or(rust_decimal::Decimal::ZERO),
                })
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "NVDA");
        assert_eq!(first.high_price, dec!(950.00));
        assert_eq!(first.low_price, dec!(410.20));
    }

    #[test]
    fn deserialize_market_cap() {
        let v = load_fixture("market_cap");
        let items: Vec<MarketCapItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| {
                Some(MarketCapItem {
                    exchange: item["excd"].as_str().unwrap_or("").to_string(),
                    symbol: item["symb"].as_str().unwrap_or("").to_string(),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    last: parse_decimal(item, "last").ok()?,
                    diff: parse_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                    rate: parse_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                    volume: parse_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                    market_cap: parse_decimal(item, "tomv").unwrap_or(rust_decimal::Decimal::ZERO),
                })
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "AAPL");
        assert_eq!(first.market_cap, dec!(2750000000000));
    }

    #[test]
    fn deserialize_trade_turnover() {
        let v = load_fixture("trade_turnover");
        let items: Vec<TradeTurnoverItem> = v["output2"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| {
                Some(TradeTurnoverItem {
                    exchange: item["excd"].as_str().unwrap_or("").to_string(),
                    symbol: item["symb"].as_str().unwrap_or("").to_string(),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    last: parse_decimal(item, "last").ok()?,
                    diff: parse_decimal(item, "diff").unwrap_or(rust_decimal::Decimal::ZERO),
                    rate: parse_decimal(item, "rate").unwrap_or(rust_decimal::Decimal::ZERO),
                    volume: parse_decimal(item, "tvol").unwrap_or(rust_decimal::Decimal::ZERO),
                    turnover_rate: parse_decimal(item, "tover")
                        .unwrap_or(rust_decimal::Decimal::ZERO),
                })
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "GME");
        assert_eq!(first.turnover_rate, dec!(18.43));
    }
}
