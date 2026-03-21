use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};

/// 해외주식 현재가 응답 (output 필드)
#[derive(Debug, Deserialize)]
pub struct StockPrice {
    /// 종목 코드
    #[serde(rename = "rsym")]
    pub symbol: String,
    /// 현재가
    #[serde(rename = "last", with = "rust_decimal::serde::str")]
    pub price: Decimal,
    /// 전일 대비 부호 (1=상한, 2=상승, 3=보합, 4=하한, 5=하락)
    #[serde(rename = "diff")]
    pub change_sign: String,
    /// 전일 대비
    #[serde(rename = "rate", with = "rust_decimal::serde::str")]
    pub change_rate: Decimal,
}

/// 해외주식 현재가 상세 조회
pub async fn current_price(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    symbol: &str,
    exchange: &str,
) -> Result<StockPrice, KisError> {
    let query = serde_json::json!({
        "AUTH": "",
        "EXCD": exchange,
        "SYMB": symbol,
    });

    execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-price/v1/quotations/price",
            tr_id: "HHDFS00000300",
            query: Some(&query),
            body: None,
        },
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_price_deserializes() {
        let json = r#"{"rsym":"NASD_AAPL","last":"189.30","diff":"2","rate":"1.50","rt_cd":"0","msg_cd":"KISOK","msg1":"success"}"#;
        let price: StockPrice = serde_json::from_str(json).unwrap();
        assert_eq!(price.symbol, "NASD_AAPL");
        assert_eq!(price.price, rust_decimal_macros::dec!(189.30));
    }
}
