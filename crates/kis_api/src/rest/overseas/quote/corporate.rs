use reqwest::Method;
use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::{Value, json};

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::Exchange;

/// 뉴스 항목
#[derive(Debug, Clone)]
pub struct NewsItem {
    pub source: String,
    pub datetime: String,
    pub title: String,
}

/// 배당 항목
#[derive(Debug, Clone)]
pub struct DividendItem {
    pub record_date: String,  // YYYYMMDD
    pub kind: String,
    pub amount: Decimal,
    pub pay_date: String,
}

/// 휴장일 항목
#[derive(Debug, Clone)]
pub struct Holiday {
    pub date: String,       // YYYYMMDD
    pub weekday: String,    // 6=토, 7=일
    pub name: String,
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

/// 해외주식 뉴스 조회
pub async fn news(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
) -> Result<Vec<NewsItem>, KisError> {
    let query = json!({
        "SYMB": symbol,
        "EXCD": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/news-title",
            tr_id: "HHDFS76410100",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output is not an array".to_string(),
    })?;

    Ok(arr.iter().map(|item| NewsItem {
        source: item["news_ofer_entp_cd"].as_str().unwrap_or("").to_string(),
        datetime: item["news_datm"].as_str().unwrap_or("").to_string(),
        title: item["hts_pbls_titl_cntt"].as_str().unwrap_or("").to_string(),
    }).collect())
}

/// 해외주식 배당 조회
pub async fn dividend(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &Exchange,
) -> Result<Vec<DividendItem>, KisError> {
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
            path: "/uapi/overseas-price/v1/quotations/period-rights",
            tr_id: "HHDFS76410200",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output is not an array".to_string(),
    })?;

    arr.iter().map(|item| {
        Ok(DividendItem {
            record_date: item["bass_dt"].as_str().unwrap_or("").to_string(),
            kind: item["rght_clsf_name"].as_str().unwrap_or("").to_string(),
            amount: parse_decimal(item, "dvdn_amt")?,
            pay_date: item["pay_dt"].as_str().unwrap_or("").to_string(),
        })
    }).collect()
}

/// 해외주식 휴장일 조회
pub async fn holidays(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    country: &str,
) -> Result<Vec<Holiday>, KisError> {
    let query = json!({
        "TRAD_DT": "",
        "NATN": country,
        "EXCD": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/countries-holiday",
            tr_id: "HHDFS76230200",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output is not an array".to_string(),
    })?;

    Ok(arr.iter().map(|item| Holiday {
        date: item["bass_dt"].as_str().unwrap_or("").to_string(),
        weekday: item["wday_dvsn_cd"].as_str().unwrap_or("").to_string(),
        name: item["dynm_dvsn_name"].as_str().unwrap_or("").to_string(),
    }).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn load_news_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/news.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    fn load_dividend_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/dividend.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    fn load_holidays_fixture() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/overseas/quote/holidays.json"
        );
        let text = std::fs::read_to_string(path).expect("fixture not found");
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn parse_news_response() {
        let v = load_news_fixture();
        let arr = v["output"].as_array().unwrap();
        let items: Vec<NewsItem> = arr.iter().map(|item| NewsItem {
            source: item["news_ofer_entp_cd"].as_str().unwrap_or("").to_string(),
            datetime: item["news_datm"].as_str().unwrap_or("").to_string(),
            title: item["hts_pbls_titl_cntt"].as_str().unwrap_or("").to_string(),
        }).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "REUTERS");
        assert_eq!(items[0].datetime, "20260321143000");
        assert_eq!(items[0].title, "Apple hits new high amid strong earnings");
    }

    #[test]
    fn parse_dividend_response() {
        let v = load_dividend_fixture();
        let arr = v["output"].as_array().unwrap();
        let items: Vec<DividendItem> = arr.iter().map(|item| DividendItem {
            record_date: item["bass_dt"].as_str().unwrap_or("").to_string(),
            kind: item["rght_clsf_name"].as_str().unwrap_or("").to_string(),
            amount: parse_decimal(item, "dvdn_amt").unwrap(),
            pay_date: item["pay_dt"].as_str().unwrap_or("").to_string(),
        }).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].record_date, "20260301");
        assert_eq!(items[0].kind, "배당");
        assert_eq!(items[0].amount, dec!(0.25));
        assert_eq!(items[0].pay_date, "20260315");
    }

    #[test]
    fn parse_holidays_response() {
        let v = load_holidays_fixture();
        let arr = v["output"].as_array().unwrap();
        let items: Vec<Holiday> = arr.iter().map(|item| Holiday {
            date: item["bass_dt"].as_str().unwrap_or("").to_string(),
            weekday: item["wday_dvsn_cd"].as_str().unwrap_or("").to_string(),
            name: item["dynm_dvsn_name"].as_str().unwrap_or("").to_string(),
        }).collect();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].date, "20260101");
        assert_eq!(items[0].weekday, "7");
        assert_eq!(items[0].name, "New Year's Day");

        assert_eq!(items[1].date, "20260704");
        assert_eq!(items[1].weekday, "6");
    }
}
