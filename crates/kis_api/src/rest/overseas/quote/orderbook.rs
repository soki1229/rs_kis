use reqwest::Method;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::{Value, json};

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;

/// 호가 단계
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
    pub symbol: String,
    pub last: Decimal,
    pub levels: Vec<OrderbookLevel>,
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

/// 해외주식 호가 조회
pub async fn orderbook(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<OrderbookResponse, KisError> {
    let query = json!({
        "AUTH": "",
        "EXCD": exchange.to_string(),
        "SYMB": symbol,
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/inquire-asking-price",
            tr_id: "HHDFS76200100",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let output = &v["output"];

    let symbol_str = output["rsym"].as_str().unwrap_or("").to_string();
    let last = parse_decimal(output, "last")?;

    let suffixes = ["", "2", "3", "4", "5"];
    let mut levels = Vec::with_capacity(5);
    for s in &suffixes {
        let bid_field = format!("bidp{s}");
        let ask_field = format!("askp{s}");
        let bidq_field = format!("bidq{s}");
        let askq_field = format!("askq{s}");
        levels.push(OrderbookLevel {
            bid_price: parse_decimal(output, &bid_field)?,
            ask_price: parse_decimal(output, &ask_field)?,
            bid_qty: parse_decimal(output, &bidq_field)?,
            ask_qty: parse_decimal(output, &askq_field)?,
        });
    }

    Ok(OrderbookResponse { symbol: symbol_str, last, levels })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/orderbook.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn parse_orderbook_response() {
        let v = load_fixture();
        let output = &v["output"];

        let symbol_str = output["rsym"].as_str().unwrap_or("").to_string();
        let last = parse_decimal(output, "last").unwrap();

        let suffixes = ["", "2", "3", "4", "5"];
        let mut levels = Vec::with_capacity(5);
        for s in &suffixes {
            let bid_field = format!("bidp{s}");
            let ask_field = format!("askp{s}");
            let bidq_field = format!("bidq{s}");
            let askq_field = format!("askq{s}");
            levels.push(OrderbookLevel {
                bid_price: parse_decimal(output, &bid_field).unwrap(),
                ask_price: parse_decimal(output, &ask_field).unwrap(),
                bid_qty: parse_decimal(output, &bidq_field).unwrap(),
                ask_qty: parse_decimal(output, &askq_field).unwrap(),
            });
        }

        assert_eq!(symbol_str, "DNASD  AAPL");
        assert_eq!(last, dec!(181.23));
        assert_eq!(levels.len(), 5);

        // Level 1
        assert_eq!(levels[0].bid_price, dec!(181.22));
        assert_eq!(levels[0].ask_price, dec!(181.24));
        assert_eq!(levels[0].bid_qty, dec!(1200));
        assert_eq!(levels[0].ask_qty, dec!(800));

        // Level 5
        assert_eq!(levels[4].bid_price, dec!(181.18));
        assert_eq!(levels[4].ask_price, dec!(181.28));
        assert_eq!(levels[4].bid_qty, dec!(3200));
        assert_eq!(levels[4].ask_qty, dec!(2100));
    }
}
