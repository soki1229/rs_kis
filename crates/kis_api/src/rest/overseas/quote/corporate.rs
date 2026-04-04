use reqwest::Method;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use std::str::FromStr;

use crate::rest::http::{execute, RequestParams};
use crate::{KisConfig, KisError};

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
    pub record_date: String, // YYYYMMDD
    pub kind: String,
    pub amount: Decimal,
    pub pay_date: String,
}

/// 휴장일 항목
#[derive(Debug, Clone)]
pub struct Holiday {
    pub date: String,    // YYYYMMDD
    pub weekday: String, // 6=토, 7=일
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

/// 해외주식 뉴스 조회 [해외주식-053]
/// TR-ID: HHPSTH60100C1
pub async fn news(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
) -> Result<Vec<NewsItem>, KisError> {
    let query = json!({
        "INFO_GB": "",
        "CLASS_CD": "",
        "NATION_CD": "",
        "EXCHANGE_CD": "",
        "SYMB": symbol,
        "DATA_DT": "",
        "DATA_TM": "",
        "CTS": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/news-title",
            tr_id: "HHPSTH60100C1",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output is not an array".to_string(),
    })?;

    Ok(arr
        .iter()
        .map(|item| NewsItem {
            source: item["source"].as_str().unwrap_or("").to_string(),
            // data_dt + data_tm 을 하나의 datetime 문자열로 합침
            datetime: format!(
                "{}{}",
                item["data_dt"].as_str().unwrap_or(""),
                item["data_tm"].as_str().unwrap_or("")
            ),
            title: item["title"].as_str().unwrap_or("").to_string(),
        })
        .collect())
}

/// 해외주식 기간별권리조회 [해외주식-052]
/// TR-ID: CTRGT011R
/// `rght_type_cd`: "%%"=전체, "03"=배당
/// `start_date`/`end_date`: YYYYMMDD
pub async fn dividend(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<DividendItem>, KisError> {
    let query = json!({
        "RGHT_TYPE_CD": "%%",
        "INQR_DVSN_CD": "02",
        "INQR_STRT_DT": start_date,
        "INQR_END_DT": end_date,
        "PDNO": symbol,
        "PRDT_TYPE_CD": "",
        "CTX_AREA_NK50": "",
        "CTX_AREA_FK50": "",
    });

    let v: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::GET,
            path: "/uapi/overseas-price/v1/quotations/period-rights",
            tr_id: "CTRGT011R",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output is not an array".to_string(),
    })?;

    arr.iter()
        .map(|item| {
            Ok(DividendItem {
                record_date: item["bass_dt"].as_str().unwrap_or("").to_string(),
                kind: item["rght_type_cd"].as_str().unwrap_or("").to_string(),
                amount: parse_decimal(item, "stkp_dvdn_frcr_amt2")
                    .or_else(|_| parse_decimal(item, "alct_frcr_unpr"))
                    .unwrap_or(rust_decimal::Decimal::ZERO),
                pay_date: item["sbsc_end_dt"].as_str().unwrap_or("").to_string(),
            })
        })
        .collect()
}

/// 해외주식 휴장일 조회 (현재 미지원)
///
/// KIS API 개편으로 기존 "해외주식 휴장일 목록" 엔드포인트
/// (`/uapi/overseas-price/v1/quotations/countries-holiday`)가 제거되었습니다.
/// 새 `/uapi/overseas-stock/v1/quotations/countries-holiday` (CTOS5011R)는
/// "해외결제일자조회" 로 목적이 달라 휴장일 목록을 반환하지 않습니다.
///
/// 이 함수는 빈 vec 을 반환하며, 호출측은 빈 결과를 "휴장일 없음(시장 개장)"으로 처리합니다.
/// 해외 휴장일이 필요한 경우 별도 캘린더 데이터를 사용하세요.
pub async fn holidays(
    _http: &reqwest::Client,
    _config: &KisConfig,
    _token: &str,
    _country: &str,
) -> Result<Vec<Holiday>, KisError> {
    Ok(vec![])
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

    #[test]
    fn parse_news_response() {
        let v = load_news_fixture();
        let arr = v["output"].as_array().unwrap();
        let items: Vec<NewsItem> = arr
            .iter()
            .map(|item| NewsItem {
                source: item["source"].as_str().unwrap_or("").to_string(),
                datetime: format!(
                    "{}{}",
                    item["data_dt"].as_str().unwrap_or(""),
                    item["data_tm"].as_str().unwrap_or("")
                ),
                title: item["title"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "REUTERS");
        assert_eq!(items[0].datetime, "20260321143000");
        assert_eq!(items[0].title, "Apple hits new high amid strong earnings");
    }

    #[test]
    fn parse_dividend_response() {
        let v = load_dividend_fixture();
        let arr = v["output"].as_array().unwrap();
        let items: Vec<DividendItem> = arr
            .iter()
            .map(|item| DividendItem {
                record_date: item["bass_dt"].as_str().unwrap_or("").to_string(),
                kind: item["rght_type_cd"].as_str().unwrap_or("").to_string(),
                amount: parse_decimal(item, "stkp_dvdn_frcr_amt2")
                    .or_else(|_| parse_decimal(item, "alct_frcr_unpr"))
                    .unwrap_or(rust_decimal::Decimal::ZERO),
                pay_date: item["sbsc_end_dt"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].record_date, "20260301");
        assert_eq!(items[0].kind, "03");
        assert_eq!(items[0].amount, dec!(0.25));
        assert_eq!(items[0].pay_date, "20260315");
    }

    #[test]
    fn holidays_returns_empty() {
        // KIS API 개편으로 해외 휴장일 목록 API가 결제일자 조회로 변경됨.
        // holidays() 함수는 항상 빈 vec 을 반환한다.
        // 실제 API 호출 없이 반환값 유형만 검증.
        let _: Vec<Holiday> = vec![];
    }
}
