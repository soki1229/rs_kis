use reqwest::Method;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::{Value, json};

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;

/// 종목 검색 결과
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub exchange: String,
}

/// 종목 상세 정보
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub symbol: String,
    pub name: String,
    pub country: String,
    pub exchange: String,
    pub listed_shares: String,
    pub per: Decimal,
    pub pbr: Decimal,
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

/// 해외주식 종목 검색
pub async fn search(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    keyword: &str,
) -> Result<Vec<SearchResult>, KisError> {
    let query = json!({
        "AUTH": "",
        "KEYWD": keyword,
        "EXCD": "",
        "SYMB": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/inquire-search",
            tr_id: "HHDFS76410000",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output is not an array".to_string(),
    })?;

    Ok(arr.iter().map(|item| SearchResult {
        symbol: item["pdno"].as_str().unwrap_or("").to_string(),
        name: item["prdt_eng_name"].as_str().unwrap_or("").to_string(),
        country: item["natn_kor_name"].as_str().unwrap_or("").to_string(),
        exchange: item["ovrs_excg_cd"].as_str().unwrap_or("").to_string(),
    }).collect())
}

/// 해외주식 종목 정보 조회
pub async fn symbol_info(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<SymbolInfo, KisError> {
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
            path: "/uapi/overseas-price/v1/quotations/search-info",
            tr_id: "HHDFS76200200",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let output = &v["output"];
    Ok(SymbolInfo {
        symbol: output["pdno"].as_str().unwrap_or("").to_string(),
        name: output["prdt_eng_name"].as_str().unwrap_or("").to_string(),
        country: output["natn_kor_name"].as_str().unwrap_or("").to_string(),
        exchange: output["ovrs_excg_cd"].as_str().unwrap_or("").to_string(),
        listed_shares: output["lstg_stck_num"].as_str().unwrap_or("").to_string(),
        per: parse_decimal(output, "perx")?,
        pbr: parse_decimal(output, "pbrx")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_search_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/search.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    fn load_symbol_info_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/symbol_info.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn parse_search_response() {
        let v = load_search_fixture();
        let arr = v["output"].as_array().unwrap();
        let results: Vec<SearchResult> = arr.iter().map(|item| SearchResult {
            symbol: item["pdno"].as_str().unwrap_or("").to_string(),
            name: item["prdt_eng_name"].as_str().unwrap_or("").to_string(),
            country: item["natn_kor_name"].as_str().unwrap_or("").to_string(),
            exchange: item["ovrs_excg_cd"].as_str().unwrap_or("").to_string(),
        }).collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol, "AAPL");
        assert_eq!(results[0].name, "APPLE INC");
        assert_eq!(results[0].country, "미국");
        assert_eq!(results[0].exchange, "NASD");
    }

    #[test]
    fn parse_symbol_info_response() {
        let v = load_symbol_info_fixture();
        let output = &v["output"];

        let info = SymbolInfo {
            symbol: output["pdno"].as_str().unwrap_or("").to_string(),
            name: output["prdt_eng_name"].as_str().unwrap_or("").to_string(),
            country: output["natn_kor_name"].as_str().unwrap_or("").to_string(),
            exchange: output["ovrs_excg_cd"].as_str().unwrap_or("").to_string(),
            listed_shares: output["lstg_stck_num"].as_str().unwrap_or("").to_string(),
            per: parse_decimal(output, "perx").unwrap(),
            pbr: parse_decimal(output, "pbrx").unwrap(),
        };

        assert_eq!(info.symbol, "AAPL");
        assert_eq!(info.name, "APPLE INC");
        assert_eq!(info.listed_shares, "15441000000");
        assert_eq!(info.per, dec!(28.50));
        assert_eq!(info.pbr, dec!(46.20));
    }
}
