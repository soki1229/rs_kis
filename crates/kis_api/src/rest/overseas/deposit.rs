/// 해외주식 체결기준현재잔고 조회 [해외주식-008]
/// TR-ID: CTRP6504R (실전) / VTRP6504R (모의)
/// Path: /uapi/overseas-stock/v1/trading/inquire-present-balance
use rust_decimal::Decimal;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::split_account;
use crate::{KisConfig, KisError};

/// 해외주식 USD 예수금 정보
#[derive(Debug, Clone)]
pub struct DepositInfo {
    /// 통화코드
    pub currency: String,
    /// 외화예수금액 (frcr_dncl_amt_2)
    pub amount: Decimal,
}

/// 해외주식 체결기준현재잔고에서 USD 예수금을 조회한다.
/// output2[] 배열에서 crcy_cd == "USD" 항목의 frcr_dncl_amt_2 를 반환한다.
pub async fn check_deposit(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
) -> Result<DepositInfo, KisError> {
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "WCRC_FRCR_DVSN_CD": "02",  // 02=외화
        "NATN_CD": "840",           // 840=미국(USD)
        "TR_MKET_CD": "00",         // 00=전체
        "INQR_DVSN_CD": "00",       // 00=전체
    });

    let tr_id = if config.mock { "VTRP6504R" } else { "CTRP6504R" };

    let v: serde_json::Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-present-balance",
            tr_id,
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let arr = v["output2"].as_array().ok_or_else(|| KisError::Api {
        code: "PARSE_ERR".to_string(),
        message: "output2 missing in inquire-present-balance response".to_string(),
    })?;

    let usd = arr
        .iter()
        .find(|item| item["crcy_cd"].as_str().unwrap_or("") == "USD")
        .ok_or_else(|| KisError::Api {
            code: "PARSE_ERR".to_string(),
            message: "USD entry not found in output2".to_string(),
        })?;

    let amount = usd["frcr_dncl_amt_2"]
        .as_str()
        .unwrap_or("0")
        .parse::<Decimal>()
        .unwrap_or(Decimal::ZERO);

    Ok(DepositInfo {
        currency: "USD".to_string(),
        amount,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn deposit_info_parses_from_output2() {
        let json = serde_json::json!({
            "rt_cd": "0",
            "output2": [
                {"crcy_cd": "KRW", "frcr_dncl_amt_2": "0"},
                {"crcy_cd": "USD", "frcr_dncl_amt_2": "1234.56"}
            ]
        });
        let arr = json["output2"].as_array().unwrap();
        let usd = arr
            .iter()
            .find(|i| i["crcy_cd"].as_str().unwrap_or("") == "USD")
            .unwrap();
        let amount: Decimal = usd["frcr_dncl_amt_2"].as_str().unwrap().parse().unwrap();
        assert_eq!(amount, dec!(1234.56));
    }

    #[test]
    fn deposit_info_missing_usd_returns_error_shape() {
        let json = serde_json::json!({
            "rt_cd": "0",
            "output2": [
                {"crcy_cd": "KRW", "frcr_dncl_amt_2": "0"}
            ]
        });
        let arr = json["output2"].as_array().unwrap();
        let found = arr
            .iter()
            .find(|i| i["crcy_cd"].as_str().unwrap_or("") == "USD");
        assert!(found.is_none());
    }
}
