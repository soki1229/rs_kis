use rust_decimal::Decimal;
use serde_json::Value;
use std::str::FromStr;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::split_account;
use crate::{KisConfig, KisError};

fn parse_decimal(v: &Value, key: &str) -> Decimal {
    v[key]
        .as_str()
        .and_then(|s| Decimal::from_str(s).ok())
        .unwrap_or(Decimal::ZERO)
}

pub struct BalanceItem {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub qty: Decimal,
    pub avg_price: Decimal,
    pub eval_amount: Decimal,
    pub unrealized_pnl: Decimal,
    pub pnl_rate: Decimal,
}

pub struct BalanceSummary {
    pub purchase_amount: Decimal,
    pub available_cash: Decimal,
    pub realized_pnl: Decimal,
    pub total_pnl: Decimal,
}

pub struct BalanceResponse {
    pub items: Vec<BalanceItem>,
    pub summary: BalanceSummary,
}

pub async fn balance(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
) -> Result<BalanceResponse, KisError> {
    let tr_id = if config.mock {
        "VTTS3012R"
    } else {
        "TTTS3012R"
    };
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "OVRS_EXCG_CD": "",
        "TR_CRCY_CD": "",
        "CTX_AREA_FK200": "",
        "CTX_AREA_NK200": ""
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-balance",
            tr_id,
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let items = resp["output1"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| BalanceItem {
            symbol: item["PDNO"].as_str().unwrap_or("").to_string(),
            name: item["PRDT_NAME"].as_str().unwrap_or("").to_string(),
            exchange: item["OVRS_EXCG_CD"].as_str().unwrap_or("").to_string(),
            qty: parse_decimal(item, "OVRS_CBLC_QTY"),
            avg_price: parse_decimal(item, "PCHS_AVG_PRIC"),
            eval_amount: parse_decimal(item, "OVRS_STCK_EVLU_AMT"),
            unrealized_pnl: parse_decimal(item, "FRCR_EVLU_PFLS_AMT"),
            pnl_rate: parse_decimal(item, "EVLU_PFLS_RT"),
        })
        .collect();

    let out2 = &resp["output2"];
    let summary = BalanceSummary {
        purchase_amount: parse_decimal(out2, "FRCR_PCHS_AMT1"),
        available_cash: parse_decimal(out2, "FRCR_PCHS_AMT1"),
        realized_pnl: parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
        total_pnl: parse_decimal(out2, "OVRS_TOT_PFLS"),
    };

    Ok(BalanceResponse { items, summary })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_balance_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/balance.json");
        let v: Value = serde_json::from_str(json).unwrap();

        let items: Vec<BalanceItem> = v["output1"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| BalanceItem {
                symbol: item["PDNO"].as_str().unwrap().to_string(),
                name: item["PRDT_NAME"].as_str().unwrap().to_string(),
                exchange: item["OVRS_EXCG_CD"].as_str().unwrap().to_string(),
                qty: parse_decimal(item, "OVRS_CBLC_QTY"),
                avg_price: parse_decimal(item, "PCHS_AVG_PRIC"),
                eval_amount: parse_decimal(item, "OVRS_STCK_EVLU_AMT"),
                unrealized_pnl: parse_decimal(item, "FRCR_EVLU_PFLS_AMT"),
                pnl_rate: parse_decimal(item, "EVLU_PFLS_RT"),
            })
            .collect();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].symbol, "AAPL");
        assert_eq!(items[0].exchange, "NASD");
        assert!(items[0].qty > Decimal::ZERO);
        assert!(items[0].pnl_rate > Decimal::ZERO);
        assert!(items[1].unrealized_pnl < Decimal::ZERO);

        let out2 = &v["output2"];
        let summary = BalanceSummary {
            purchase_amount: parse_decimal(out2, "FRCR_PCHS_AMT1"),
            available_cash: parse_decimal(out2, "FRCR_PCHS_AMT1"),
            realized_pnl: parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
            total_pnl: parse_decimal(out2, "OVRS_TOT_PFLS"),
        };
        assert!(summary.purchase_amount > Decimal::ZERO);
        assert_eq!(summary.realized_pnl, Decimal::ZERO);
    }
}
