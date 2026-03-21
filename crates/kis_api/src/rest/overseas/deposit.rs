use rust_decimal::Decimal;
use serde::Deserialize;

use crate::rest::http::{execute, RequestParams};
use crate::{KisConfig, KisError};

/// 예수금 상세 현황 응답
#[derive(Debug, Deserialize)]
pub struct DepositInfo {
    /// 외화증거금통화코드
    #[serde(rename = "crcy_cd")]
    pub currency: String,
    /// 외화예수금
    #[serde(rename = "frcr_dncl_amt_2", with = "rust_decimal::serde::str")]
    pub amount: Decimal,
}

/// 해외주식 예수금 상세 현황 조회
pub async fn check_deposit(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
) -> Result<DepositInfo, KisError> {
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "CRCY_CD": "USD",
        "INQR_DVSN_1": "",
        "INQR_DVSN_2": ""
    });

    let tr_id = if config.mock {
        "VTTS3012R"
    } else {
        "TTTS3012R"
    };

    execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-psamount",
            tr_id,
            query: Some(&query),
            body: None,
        },
    )
    .await
}

fn split_account(account: &str) -> (&str, &str) {
    let mut it = account.splitn(2, '-');
    (it.next().unwrap_or(""), it.next().unwrap_or(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_info_deserializes() {
        let json = r#"{"crcy_cd":"USD","frcr_dncl_amt_2":"1234.56","rt_cd":"0","msg_cd":"KISOK","msg1":"success"}"#;
        let info: DepositInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.currency, "USD");
        assert_eq!(info.amount, rust_decimal_macros::dec!(1234.56));
    }
}
