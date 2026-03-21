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

pub struct ProfitItem {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub filled_qty: Decimal,
    pub avg_buy_price: Decimal,
    pub filled_price: Decimal,
    pub realized_pnl: Decimal,
    pub pnl_rate: Decimal,
}

pub struct ProfitSummary {
    pub realized_pnl: Decimal,
    pub total_pnl: Decimal,
}

pub struct PeriodProfitRequest {
    pub start_date: String,
    pub end_date: String,
}

pub struct PeriodProfitResponse {
    pub items: Vec<ProfitItem>,
    pub summary: ProfitSummary,
}

pub struct BuyableAmountRequest {
    pub exchange_cd: String,
    pub symbol: String,
    pub price: String,
    pub order_type_cd: String,
}

pub struct BuyableAmountResponse {
    pub max_qty: Decimal,
    pub exchange_balance: Decimal,
    pub buyable_amount: Decimal,
}

pub async fn period_profit(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: PeriodProfitRequest,
) -> Result<PeriodProfitResponse, KisError> {
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "OVRS_EXCG_CD": "",
        "NATN_CD": "",
        "CRCY_CD": "",
        "PDNO": "",
        "INQR_STRT_DT": req.start_date,
        "INQR_END_DT": req.end_date,
        "WCRC_FRCR_DVSN_CD": "01",
        "CTX_AREA_FK200": "",
        "CTX_AREA_NK200": ""
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-period-profit",
            tr_id: "TTTS3039R",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let items = resp["output1"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| ProfitItem {
            symbol: item["PDNO"].as_str().unwrap_or("").to_string(),
            name: item["PRDT_NAME"].as_str().unwrap_or("").to_string(),
            exchange: item["OVRS_EXCG_CD"].as_str().unwrap_or("").to_string(),
            filled_qty: parse_decimal(item, "CCLD_QTY"),
            avg_buy_price: parse_decimal(item, "PCHS_AVG_PRIC"),
            filled_price: parse_decimal(item, "CCLD_UNPR"),
            realized_pnl: parse_decimal(item, "RLZT_PFLS"),
            pnl_rate: parse_decimal(item, "PFLS_RT"),
        })
        .collect();

    let out2 = &resp["output2"];
    let summary = ProfitSummary {
        realized_pnl: parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
        total_pnl: parse_decimal(out2, "OVRS_TOT_PFLS"),
    };

    Ok(PeriodProfitResponse { items, summary })
}

pub async fn buyable_amount(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: BuyableAmountRequest,
) -> Result<BuyableAmountResponse, KisError> {
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "OVRS_EXCG_CD": req.exchange_cd,
        "OVRS_ORD_UNPR": req.price,
        "ITEM_CD": req.symbol,
        "OVRS_ORD_QTY": "",
        "ORD_DVSN_CD": req.order_type_cd
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-psamount",
            tr_id: "TTTS3007R",
            query: Some(&query),
            body: None,
        },
    )
    .await?;

    let out = &resp["output"];
    Ok(BuyableAmountResponse {
        max_qty: parse_decimal(out, "OVRS_MAX_ORD_PSBL_QTY"),
        exchange_balance: parse_decimal(out, "EXCC_RAMT"),
        buyable_amount: parse_decimal(out, "FRCR_BUY_PSBL_AMT"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_period_profit_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/period_profit.json");
        let v: Value = serde_json::from_str(json).unwrap();

        let items: Vec<ProfitItem> = v["output1"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| ProfitItem {
                symbol: item["PDNO"].as_str().unwrap().to_string(),
                name: item["PRDT_NAME"].as_str().unwrap().to_string(),
                exchange: item["OVRS_EXCG_CD"].as_str().unwrap().to_string(),
                filled_qty: parse_decimal(item, "CCLD_QTY"),
                avg_buy_price: parse_decimal(item, "PCHS_AVG_PRIC"),
                filled_price: parse_decimal(item, "CCLD_UNPR"),
                realized_pnl: parse_decimal(item, "RLZT_PFLS"),
                pnl_rate: parse_decimal(item, "PFLS_RT"),
            })
            .collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].symbol, "NVDA");
        assert_eq!(items[0].exchange, "NASD");
        assert!(items[0].realized_pnl > Decimal::ZERO);
        assert!(items[0].pnl_rate > Decimal::ZERO);

        let out2 = &v["output2"];
        let summary = ProfitSummary {
            realized_pnl: parse_decimal(out2, "OVRS_RLZT_PFLS_AMT"),
            total_pnl: parse_decimal(out2, "OVRS_TOT_PFLS"),
        };
        assert!(summary.realized_pnl > Decimal::ZERO);
        assert_eq!(summary.realized_pnl, summary.total_pnl);
    }

    #[test]
    fn deserialize_buyable_amount_response() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/buyable_amount.json");
        let v: Value = serde_json::from_str(json).unwrap();

        let out = &v["output"];
        let resp = BuyableAmountResponse {
            max_qty: parse_decimal(out, "OVRS_MAX_ORD_PSBL_QTY"),
            exchange_balance: parse_decimal(out, "EXCC_RAMT"),
            buyable_amount: parse_decimal(out, "FRCR_BUY_PSBL_AMT"),
        };

        assert!(resp.max_qty > Decimal::ZERO);
        assert!(resp.exchange_balance > Decimal::ZERO);
        assert_eq!(resp.exchange_balance, resp.buyable_amount);
    }
}
