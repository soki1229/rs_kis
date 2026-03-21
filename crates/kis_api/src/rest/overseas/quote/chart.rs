use reqwest::Method;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::str::FromStr;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;
use crate::{KisConfig, KisError};

/// 일봉/주봉/월봉 캔들
#[derive(Debug, Clone)]
pub struct CandleBar {
    pub date: String, // YYYYMMDD
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

/// 분봉 캔들
#[derive(Debug, Clone)]
pub struct MinuteBar {
    pub date: String, // YYYYMMDD
    pub time: String, // HHmmss
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

/// 일/주/월봉 조회 주기
#[derive(Debug, Clone)]
pub enum ChartPeriod {
    Daily,   // "0"
    Weekly,  // "1"
    Monthly, // "2"
}

impl ChartPeriod {
    pub fn as_code(&self) -> &str {
        match self {
            ChartPeriod::Daily => "0",
            ChartPeriod::Weekly => "1",
            ChartPeriod::Monthly => "2",
        }
    }
}

/// 일봉 요청
#[derive(Debug, Clone)]
pub struct DailyChartRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub period: ChartPeriod,
    pub adj_price: bool,
}

/// 분봉 요청
#[derive(Debug, Clone)]
pub struct MinuteChartRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub minutes: u32, // 1, 5, 10, 30, 60
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

/// 해외주식 일/주/월봉 조회
pub async fn daily_chart(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: DailyChartRequest,
) -> Result<Vec<CandleBar>, KisError> {
    let modp = if req.adj_price { "1" } else { "0" };
    let query = json!({
        "AUTH": "",
        "EXCD": req.exchange.to_price_code(),
        "SYMB": req.symbol,
        "GUBN": req.period.as_code(),
        "BYMD": "",
        "MODP": modp,
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/dailyprice",
            tr_id: "HHDFS76240000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output2 is not an array".to_string(),
    })?;

    arr.iter()
        .map(|item| {
            Ok(CandleBar {
                date: item["xymd"].as_str().unwrap_or("").to_string(),
                open: parse_decimal(item, "open")?,
                high: parse_decimal(item, "high")?,
                low: parse_decimal(item, "low")?,
                close: parse_decimal(item, "clos")?,
                volume: parse_decimal(item, "tvol")?,
            })
        })
        .collect()
}

/// 해외주식 분봉 조회
pub async fn minute_chart(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: MinuteChartRequest,
) -> Result<Vec<MinuteBar>, KisError> {
    let minutes_str = req.minutes.to_string();
    let query = json!({
        "AUTH": "",
        "EXCD": req.exchange.to_price_code(),
        "SYMB": req.symbol,
        "NMIN": minutes_str,
        "PINC": "1",
        "NEXT": "",
        "NREC": "120",
        "FILL": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/inquire-time-itemchartprice",
            tr_id: "HHDFS76950200",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output2 is not an array".to_string(),
    })?;

    arr.iter()
        .map(|item| {
            Ok(MinuteBar {
                date: item["kymd"].as_str().unwrap_or("").to_string(),
                time: item["khms"].as_str().unwrap_or("").to_string(),
                open: parse_decimal(item, "open")?,
                high: parse_decimal(item, "high")?,
                low: parse_decimal(item, "low")?,
                close: parse_decimal(item, "last")?,
                volume: parse_decimal(item, "tvol")?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_daily_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/daily_chart.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    fn load_minute_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/minute_chart.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn parse_daily_chart_response() {
        let v = load_daily_fixture();
        let arr = v["output2"].as_array().unwrap();
        let bars: Vec<CandleBar> = arr
            .iter()
            .map(|item| CandleBar {
                date: item["xymd"].as_str().unwrap_or("").to_string(),
                open: parse_decimal(item, "open").unwrap(),
                high: parse_decimal(item, "high").unwrap(),
                low: parse_decimal(item, "low").unwrap(),
                close: parse_decimal(item, "clos").unwrap(),
                volume: parse_decimal(item, "tvol").unwrap(),
            })
            .collect();

        assert_eq!(bars.len(), 2);
        assert_eq!(bars[0].date, "20260321");
        assert_eq!(bars[0].open, dec!(178.5));
        assert_eq!(bars[0].high, dec!(182.34));
        assert_eq!(bars[0].low, dec!(177.92));
        assert_eq!(bars[0].close, dec!(181.23));
        assert_eq!(bars[0].volume, dec!(62345100));

        assert_eq!(bars[1].date, "20260320");
        assert_eq!(bars[1].close, dec!(179.66));
    }

    #[test]
    fn parse_minute_chart_response() {
        let v = load_minute_fixture();
        let arr = v["output2"].as_array().unwrap();
        let bars: Vec<MinuteBar> = arr
            .iter()
            .map(|item| MinuteBar {
                date: item["kymd"].as_str().unwrap_or("").to_string(),
                time: item["khms"].as_str().unwrap_or("").to_string(),
                open: parse_decimal(item, "open").unwrap(),
                high: parse_decimal(item, "high").unwrap(),
                low: parse_decimal(item, "low").unwrap(),
                close: parse_decimal(item, "last").unwrap(),
                volume: parse_decimal(item, "tvol").unwrap(),
            })
            .collect();

        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].date, "20260321");
        assert_eq!(bars[0].time, "143000");
        assert_eq!(bars[0].open, dec!(181.0));
        assert_eq!(bars[0].high, dec!(181.5));
        assert_eq!(bars[0].low, dec!(180.8));
        assert_eq!(bars[0].close, dec!(181.23));
        assert_eq!(bars[0].volume, dec!(123456));
    }

    #[test]
    fn chart_period_codes() {
        assert_eq!(ChartPeriod::Daily.as_code(), "0");
        assert_eq!(ChartPeriod::Weekly.as_code(), "1");
        assert_eq!(ChartPeriod::Monthly.as_code(), "2");
    }
}
