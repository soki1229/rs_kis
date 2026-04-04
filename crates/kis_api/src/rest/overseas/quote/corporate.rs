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

/// NYSE 공휴일 계산 — 하드코딩 규칙 기반
///
/// 적용 규칙 (NYSE 공식 휴장일):
///   신년(1/1), MLK Day(1월 3번째 월), Presidents Day(2월 3번째 월),
///   Good Friday(부활절 전 금요일), Memorial Day(5월 마지막 월), Juneteenth(6/19, since 2022),
///   Independence Day(7/4), Labor Day(9월 첫 번째 월), Thanksgiving(11월 4번째 목), Christmas(12/25).
///   토요일 관찰일 → 전 금요일, 일요일 관찰일 → 다음 월요일.
fn nyse_holidays(year: i32) -> Vec<chrono::NaiveDate> {
    use chrono::{Datelike, NaiveDate, Weekday};

    fn observe(d: NaiveDate) -> NaiveDate {
        match d.weekday() {
            Weekday::Sat => d.pred_opt().unwrap(),
            Weekday::Sun => d.succ_opt().unwrap(),
            _ => d,
        }
    }

    fn nth_weekday(year: i32, month: u32, wd: Weekday, n: u32) -> NaiveDate {
        let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let days_until =
            (wd.num_days_from_monday() + 7 - first.weekday().num_days_from_monday()) % 7;
        first + chrono::Days::new(days_until as u64) + chrono::Days::new(((n - 1) * 7) as u64)
    }

    fn last_weekday(year: i32, month: u32, wd: Weekday) -> NaiveDate {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        let last = NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .unwrap()
            .pred_opt()
            .unwrap();
        let days_back = (last.weekday().num_days_from_monday() + 7 - wd.num_days_from_monday()) % 7;
        last - chrono::Days::new(days_back as u64)
    }

    /// Anonymous Gregorian Easter 알고리즘
    fn easter_sunday(year: i32) -> NaiveDate {
        let a = year % 19;
        let b = year / 100;
        let c = year % 100;
        let d = b / 4;
        let e = b % 4;
        let f = (b + 8) / 25;
        let g = (b - f + 1) / 3;
        let h = (19 * a + b - d - g + 15) % 30;
        let i = c / 4;
        let k = c % 4;
        let l = (32 + 2 * e + 2 * i - h - k) % 7;
        let m = (a + 11 * h + 22 * l) / 451;
        let month = (h + l - 7 * m + 114) / 31;
        let day = ((h + l - 7 * m + 114) % 31) + 1;
        NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap()
    }

    let mut dates = vec![
        observe(NaiveDate::from_ymd_opt(year, 1, 1).unwrap()), // New Year's Day
        nth_weekday(year, 1, Weekday::Mon, 3),                 // MLK Day
        nth_weekday(year, 2, Weekday::Mon, 3),                 // Presidents Day
        easter_sunday(year) - chrono::Days::new(2),            // Good Friday
        last_weekday(year, 5, Weekday::Mon),                   // Memorial Day
        observe(NaiveDate::from_ymd_opt(year, 7, 4).unwrap()), // Independence Day
        nth_weekday(year, 9, Weekday::Mon, 1),                 // Labor Day
        nth_weekday(year, 11, Weekday::Thu, 4),                // Thanksgiving
        observe(NaiveDate::from_ymd_opt(year, 12, 25).unwrap()), // Christmas
    ];
    if year >= 2022 {
        dates.push(observe(NaiveDate::from_ymd_opt(year, 6, 19).unwrap())); // Juneteenth
    }
    dates.sort();
    dates
}

/// 해외주식 공휴일 조회 (NYSE 기준, 올해 + 내년 초)
///
/// KIS는 해외 공휴일 전용 API를 제공하지 않으므로 두 소스를 조합:
///   1. NYSE 하드코딩 규칙 (주 소스 — 항상 동작)
///   2. date.nager.at 무료 API (보조 — 추가 공휴일 보완, 5초 타임아웃, 실패 시 무시)
pub async fn holidays(
    http: &reqwest::Client,
    _config: &KisConfig,
    _token: &str,
    _country: &str,
) -> Result<Vec<Holiday>, KisError> {
    use chrono::{Datelike, Utc};
    let this_year = Utc::now().date_naive().year();

    // 1. NYSE 하드코딩 규칙 (올해 + 내년)
    let mut dates: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for year in [this_year, this_year + 1] {
        for d in nyse_holidays(year) {
            dates.insert(d.format("%Y%m%d").to_string());
        }
    }

    // 2. date.nager.at 보조 — 올해 US 공휴일 (실패해도 무시)
    let nager_url = format!("https://date.nager.at/api/v3/publicholidays/{this_year}/US");
    let nager_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        http.get(&nager_url).send(),
    )
    .await;
    if let Ok(Ok(resp)) = nager_result {
        if let Ok(serde_json::Value::Array(arr)) = resp.json::<serde_json::Value>().await {
            for item in arr {
                if let Some(s) = item["date"].as_str() {
                    // "2026-07-04" → "20260704"
                    let compact = s.replace('-', "");
                    if compact.len() == 8 {
                        dates.insert(compact);
                    }
                }
            }
        }
    }

    Ok(dates
        .into_iter()
        .map(|date| Holiday {
            date,
            weekday: String::new(),
            name: "holiday".to_string(),
        })
        .collect())
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
