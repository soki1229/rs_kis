use reqwest::Method;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::{Value, json};

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;

/// 해외주식 현재가 응답
#[derive(Debug, Clone)]
pub struct PriceResponse {
    pub last: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub diff: Decimal,
    pub rate: Decimal,
    pub volume: Decimal,
    pub bid: Decimal,
    pub ask: Decimal,
    pub orderable: bool,
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

/// 해외주식 현재가 조회
pub async fn price(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<PriceResponse, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_price_code(),
        "SYMB": symbol,
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/price",
            tr_id: "HHDFS00000300",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let output = &v["output"];
    Ok(PriceResponse {
        last: parse_decimal(output, "last")?,
        open: parse_decimal(output, "open")?,
        high: parse_decimal(output, "high")?,
        low: parse_decimal(output, "low")?,
        diff: parse_decimal(output, "diff")?,
        rate: parse_decimal(output, "rate")?,
        volume: parse_decimal(output, "tvol")?,
        bid: parse_decimal(output, "pbid")?,
        ask: parse_decimal(output, "pask")?,
        orderable: output["ordy"].as_str().unwrap_or("") == "Y",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/price.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn parse_price_response() {
        let v = load_fixture();
        let output = &v["output"];

        let resp = PriceResponse {
            last: parse_decimal(output, "last").unwrap(),
            open: parse_decimal(output, "open").unwrap(),
            high: parse_decimal(output, "high").unwrap(),
            low: parse_decimal(output, "low").unwrap(),
            diff: parse_decimal(output, "diff").unwrap(),
            rate: parse_decimal(output, "rate").unwrap(),
            volume: parse_decimal(output, "tvol").unwrap(),
            bid: parse_decimal(output, "pbid").unwrap(),
            ask: parse_decimal(output, "pask").unwrap(),
            orderable: output["ordy"].as_str().unwrap_or("") == "Y",
        };

        assert_eq!(resp.last, dec!(181.23));
        assert_eq!(resp.open, dec!(178.5));
        assert_eq!(resp.high, dec!(182.34));
        assert_eq!(resp.low, dec!(177.92));
        assert_eq!(resp.diff, dec!(1.57));
        assert_eq!(resp.rate, dec!(0.87));
        assert_eq!(resp.volume, dec!(62345100));
        assert_eq!(resp.bid, dec!(181.22));
        assert_eq!(resp.ask, dec!(181.24));
        assert!(resp.orderable);
    }

    #[test]
    fn exchange_price_codes() {
        assert_eq!(Exchange::NASD.to_price_code(), "NAS");
        assert_eq!(Exchange::NYSE.to_price_code(), "NYS");
        assert_eq!(Exchange::AMEX.to_price_code(), "AMS");
        assert_eq!(Exchange::SEHK.to_price_code(), "HKS");
        assert_eq!(Exchange::TKSE.to_price_code(), "TSE");
    }
}
