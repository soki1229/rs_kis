use rust_decimal::Decimal;
use std::str::FromStr;
use serde_json::Value;

use crate::{KisConfig, KisError};
use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::split_account;

fn parse_decimal(v: &Value, key: &str) -> Decimal {
    v[key].as_str()
        .and_then(|s| Decimal::from_str(s).ok())
        .unwrap_or(Decimal::ZERO)
}

pub struct UnfilledOrder {
    pub order_no: String,
    pub orig_order_no: String,
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    /// "01" = 매도, "02" = 매수
    pub side_cd: String,
    pub qty: Decimal,
    pub price: Decimal,
    pub filled_qty: Decimal,
    pub remaining_qty: Decimal,
}

pub struct OrderHistoryItem {
    pub order_no: String,
    pub symbol: String,
    pub name: String,
    /// "01" = 매도, "02" = 매수
    pub side_cd: String,
    pub qty: Decimal,
    pub filled_qty: Decimal,
    pub filled_price: Decimal,
    pub filled_date: String,
    pub filled_time: String,
}

pub struct OrderHistoryRequest {
    pub start_date: String,
    pub end_date: String,
    pub exchange_cd: String,
}

pub async fn unfilled_orders(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
) -> Result<Vec<UnfilledOrder>, KisError> {
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "OVRS_EXCG_CD": "",
        "SORT_SQN": "DS",
        "CTX_AREA_FK200": "",
        "CTX_AREA_NK200": ""
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-nccs",
            tr_id: "TTTS3018R",
            query: Some(&query),
            body: None,
        },
    ).await?;

    let orders = resp["output"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| UnfilledOrder {
            order_no: item["ODNO"].as_str().unwrap_or("").to_string(),
            orig_order_no: item["ORGN_ODNO"].as_str().unwrap_or("").to_string(),
            symbol: item["PDNO"].as_str().unwrap_or("").to_string(),
            name: item["PRDT_NAME"].as_str().unwrap_or("").to_string(),
            exchange: item["OVRS_EXCG_CD"].as_str().unwrap_or("").to_string(),
            side_cd: item["SLL_BUY_DVSN_CD"].as_str().unwrap_or("").to_string(),
            qty: parse_decimal(item, "ORD_QTY"),
            price: parse_decimal(item, "OVRS_ORD_UNPR"),
            filled_qty: parse_decimal(item, "FT_CCLD_QTY"),
            remaining_qty: parse_decimal(item, "RMND_QTY"),
        })
        .collect();

    Ok(orders)
}

pub async fn order_history(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: OrderHistoryRequest,
) -> Result<Vec<OrderHistoryItem>, KisError> {
    let tr_id = if config.mock { "VTTS3035R" } else { "TTTS3035R" };
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let query = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "PDNO": "",
        "OVRS_EXCG_CD": req.exchange_cd,
        "OVRS_ORD_UNPR_SUM": "",
        "OVRS_RLZT_PFLS_AMT_SUM": "",
        "CTX_AREA_NK200": "",
        "CTX_AREA_FK200": "",
        "INQR_STRT_DT": req.start_date,
        "INQR_END_DT": req.end_date,
        "SORT_SQN": "DS",
        "ORD_QTY_SUM": ""
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: reqwest::Method::GET,
            path: "/uapi/overseas-stock/v1/trading/inquire-ccnl",
            tr_id,
            query: Some(&query),
            body: None,
        },
    ).await?;

    let items = resp["output"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| OrderHistoryItem {
            order_no: item["ODNO"].as_str().unwrap_or("").to_string(),
            symbol: item["PDNO"].as_str().unwrap_or("").to_string(),
            name: item["PRDT_NAME"].as_str().unwrap_or("").to_string(),
            side_cd: item["SLL_BUY_DVSN_CD"].as_str().unwrap_or("").to_string(),
            qty: parse_decimal(item, "ORD_QTY"),
            filled_qty: parse_decimal(item, "CCLD_QTY"),
            filled_price: parse_decimal(item, "CCLD_UNPR"),
            filled_date: item["CCLD_DT"].as_str().unwrap_or("").to_string(),
            filled_time: item["CCLD_TM"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_unfilled_orders() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/unfilled_orders.json");
        let v: Value = serde_json::from_str(json).unwrap();

        let orders: Vec<UnfilledOrder> = v["output"].as_array().unwrap().iter().map(|item| UnfilledOrder {
            order_no: item["ODNO"].as_str().unwrap().to_string(),
            orig_order_no: item["ORGN_ODNO"].as_str().unwrap_or("").to_string(),
            symbol: item["PDNO"].as_str().unwrap().to_string(),
            name: item["PRDT_NAME"].as_str().unwrap().to_string(),
            exchange: item["OVRS_EXCG_CD"].as_str().unwrap().to_string(),
            side_cd: item["SLL_BUY_DVSN_CD"].as_str().unwrap().to_string(),
            qty: parse_decimal(item, "ORD_QTY"),
            price: parse_decimal(item, "OVRS_ORD_UNPR"),
            filled_qty: parse_decimal(item, "FT_CCLD_QTY"),
            remaining_qty: parse_decimal(item, "RMND_QTY"),
        }).collect();

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_no, "0000117057");
        assert_eq!(orders[0].symbol, "AAPL");
        assert_eq!(orders[0].exchange, "NASD");
        assert_eq!(orders[0].side_cd, "02");
        assert!(orders[0].qty > Decimal::ZERO);
        assert_eq!(orders[0].filled_qty, Decimal::ZERO);
        assert_eq!(orders[0].remaining_qty, orders[0].qty);
    }

    #[test]
    fn deserialize_order_history() {
        let json = include_str!("../../../../tests/fixtures/overseas/inquiry/order_history.json");
        let v: Value = serde_json::from_str(json).unwrap();

        let items: Vec<OrderHistoryItem> = v["output"].as_array().unwrap().iter().map(|item| OrderHistoryItem {
            order_no: item["ODNO"].as_str().unwrap().to_string(),
            symbol: item["PDNO"].as_str().unwrap().to_string(),
            name: item["PRDT_NAME"].as_str().unwrap().to_string(),
            side_cd: item["SLL_BUY_DVSN_CD"].as_str().unwrap().to_string(),
            qty: parse_decimal(item, "ORD_QTY"),
            filled_qty: parse_decimal(item, "CCLD_QTY"),
            filled_price: parse_decimal(item, "CCLD_UNPR"),
            filled_date: item["CCLD_DT"].as_str().unwrap().to_string(),
            filled_time: item["CCLD_TM"].as_str().unwrap().to_string(),
        }).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].order_no, "0000117001");
        assert_eq!(items[0].symbol, "TSLA");
        assert_eq!(items[0].side_cd, "01");
        assert!(items[0].filled_price > Decimal::ZERO);
        assert_eq!(items[0].filled_date, "20260320");
        assert_eq!(items[0].filled_time, "143500");
    }
}
