use reqwest::Method;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::{Value, json};

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;

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

/// 해외주식 체결강도 조회
pub async fn volume_power(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<Vec<VolumePowerItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "SYMB": symbol,
        "KEYB": "",
        "NREC": "30",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/volume-power",
            tr_id: "HHDFS76370100",
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
            Ok(VolumePowerItem {
                date: item["XYMD"].as_str().unwrap_or("").to_string(),
                total_volume: parse_decimal(item, "TVOL")?,
                buy_volume: parse_decimal(item, "MBVOL")?,
                sell_volume: parse_decimal(item, "MSVOL")?,
                power: parse_decimal(item, "POWER")?,
            })
        })
        .collect()
}

/// 해외주식 신고가/신저가 순위 조회
pub async fn new_highlow(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    exchange: &Exchange,
    kind: &HighLowKind,
    count: u32,
) -> Result<Vec<NewHighLowItem>, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "HLGU": kind.to_hlgu(),
        "KEYB": "",
        "NREC": count.to_string(),
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/new-highlow",
            tr_id: "HHDFS76280300",
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
            Ok(NewHighLowItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST")?,
                diff: parse_decimal(item, "DIFF")?,
                rate: parse_decimal(item, "RATE")?,
                volume: parse_decimal(item, "TVOL")?,
                high_price: parse_decimal(item, "XHGP")?,
                low_price: parse_decimal(item, "XLWP")?,
            })
        })
        .collect()
}

/// 해외주식 시가총액 순위 조회
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
        "KEYB": "",
        "NREC": count.to_string(),
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/market-cap",
            tr_id: "HHDFS76260100",
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
            Ok(MarketCapItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST")?,
                diff: parse_decimal(item, "DIFF")?,
                rate: parse_decimal(item, "RATE")?,
                volume: parse_decimal(item, "TVOL")?,
                market_cap: parse_decimal(item, "MKTC")?,
            })
        })
        .collect()
}

/// 해외주식 거래회전율 순위 조회
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
        "KEYB": "",
        "NREC": count.to_string(),
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/trade-turnover",
            tr_id: "HHDFS76370200",
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
            Ok(TradeTurnoverItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST")?,
                diff: parse_decimal(item, "DIFF")?,
                rate: parse_decimal(item, "RATE")?,
                volume: parse_decimal(item, "TVOL")?,
                turnover_rate: parse_decimal(item, "TRNO")?,
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
    fn high_low_kind_to_hlgu() {
        assert_eq!(HighLowKind::High.to_hlgu(), "1");
        assert_eq!(HighLowKind::Low.to_hlgu(), "2");
    }

    #[test]
    fn deserialize_volume_power() {
        let v = load_fixture("volume_power");
        let items: Vec<VolumePowerItem> = v["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| {
                VolumePowerItem {
                    date: item["XYMD"].as_str().unwrap_or("").to_string(),
                    total_volume: parse_decimal(item, "TVOL").unwrap(),
                    buy_volume: parse_decimal(item, "MBVOL").unwrap(),
                    sell_volume: parse_decimal(item, "MSVOL").unwrap(),
                    power: parse_decimal(item, "POWER").unwrap(),
                }
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.date, "20260321");
        assert_eq!(first.buy_volume, dec!(26540200));
        assert_eq!(first.sell_volume, dec!(21779900));
        assert_eq!(first.power, dec!(121.89));
    }

    #[test]
    fn deserialize_new_highlow() {
        let v = load_fixture("new_highlow");
        let items: Vec<NewHighLowItem> = v["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| NewHighLowItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST").unwrap(),
                diff: parse_decimal(item, "DIFF").unwrap(),
                rate: parse_decimal(item, "RATE").unwrap(),
                volume: parse_decimal(item, "TVOL").unwrap(),
                high_price: parse_decimal(item, "XHGP").unwrap(),
                low_price: parse_decimal(item, "XLWP").unwrap(),
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
        let items: Vec<MarketCapItem> = v["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| MarketCapItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST").unwrap(),
                diff: parse_decimal(item, "DIFF").unwrap(),
                rate: parse_decimal(item, "RATE").unwrap(),
                volume: parse_decimal(item, "TVOL").unwrap(),
                market_cap: parse_decimal(item, "MKTC").unwrap(),
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
        let items: Vec<TradeTurnoverItem> = v["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| TradeTurnoverItem {
                exchange: item["EXCD"].as_str().unwrap_or("").to_string(),
                symbol: item["SYMB"].as_str().unwrap_or("").to_string(),
                name: item["DNAM"].as_str().unwrap_or("").to_string(),
                last: parse_decimal(item, "LAST").unwrap(),
                diff: parse_decimal(item, "DIFF").unwrap(),
                rate: parse_decimal(item, "RATE").unwrap(),
                volume: parse_decimal(item, "TVOL").unwrap(),
                turnover_rate: parse_decimal(item, "TRNO").unwrap(),
            })
            .collect();

        assert_eq!(items.len(), 2);
        let first = &items[0];
        assert_eq!(first.symbol, "GME");
        assert_eq!(first.turnover_rate, dec!(18.43));
    }
}
