// ⚠ TR ID 및 필드명은 KIS OpenAPI 가이드에서 확인 필요
// - 매수: POST /uapi/domestic-stock/v1/trading/order-cash  (tr_id: TTTC0802U)
// - 매도: POST /uapi/domestic-stock/v1/trading/order-cash  (tr_id: TTTC0801U)
// - 취소: POST /uapi/domestic-stock/v1/trading/order-rvsecncl (tr_id: TTTC0803U)

use super::types::*;
use crate::{KisConfig, KisError, OrderSide};
use crate::rest::overseas::types::split_account;
use reqwest::Client;
use serde_json::json;

pub async fn domestic_place_order(
    client: &Client,
    config: &KisConfig,
    token: &str,
    req: DomesticPlaceOrderRequest,
) -> Result<DomesticPlaceOrderResponse, KisError> {
    let tr_id = match req.side {
        OrderSide::Buy => "TTTC0802U",
        OrderSide::Sell => "TTTC0801U",
    };

    let (cano, prdt_cd) = split_account(&config.account_num);

    let body = json!({
        "CANO": cano,
        "ACNT_PRDT_CD": prdt_cd,
        "PDNO": req.symbol,
        "ORD_DVSN": req.order_type.as_str(),
        "ORD_QTY": req.qty.to_string(),
        "ORD_UNPR": req.price.map(|p| p.to_string()).unwrap_or_else(|| "0".into()),
    });

    let resp = client
        .post(format!("{}/uapi/domestic-stock/v1/trading/order-cash", config.rest_url))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", tr_id)
        .header("custtype", "P")
        .json(&body)
        .send()
        .await
        .map_err(KisError::Network)?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(KisError::Network)?;

    let rt_cd = json["rt_cd"].as_str().unwrap_or("");
    if rt_cd != "0" {
        return Err(KisError::Api {
            code: rt_cd.into(),
            message: json["msg1"].as_str().unwrap_or("unknown").into(),
        });
    }

    let output = &json["output"];
    Ok(DomesticPlaceOrderResponse {
        order_no: output["ODNO"].as_str().unwrap_or("").into(),
        order_date: output["ORD_DT"].as_str().unwrap_or("").into(),
        order_time: output["ORD_TMD"].as_str().unwrap_or("").into(),
    })
}

pub async fn domestic_cancel_order(
    client: &Client,
    config: &KisConfig,
    token: &str,
    req: DomesticCancelOrderRequest,
) -> Result<DomesticCancelOrderResponse, KisError> {
    let (cano, prdt_cd) = split_account(&config.account_num);

    let body = json!({
        "CANO": cano,
        "ACNT_PRDT_CD": prdt_cd,
        "KRX_FWDG_ORD_ORGNO": "",
        "ORGN_ODNO": req.original_order_no,
        "ORD_DVSN": "00",
        "RVSE_CNCL_DVSN_CD": "02",
        "ORD_QTY": req.qty.to_string(),
        "ORD_UNPR": req.price.map(|p| p.to_string()).unwrap_or_else(|| "0".into()),
        "QTY_ALL_ORD_YN": "Y",
    });

    let resp = client
        .post(format!("{}/uapi/domestic-stock/v1/trading/order-rvsecncl", config.rest_url))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", "TTTC0803U")
        .header("custtype", "P")
        .json(&body)
        .send()
        .await
        .map_err(KisError::Network)?;

    let json: serde_json::Value = resp.json().await.map_err(KisError::Network)?;
    let rt_cd = json["rt_cd"].as_str().unwrap_or("");
    if rt_cd != "0" {
        return Err(KisError::Api {
            code: rt_cd.into(),
            message: json["msg1"].as_str().unwrap_or("unknown").into(),
        });
    }
    let output = &json["output"];
    Ok(DomesticCancelOrderResponse {
        order_no: output["ODNO"].as_str().unwrap_or("").into(),
        order_date: output["ORD_DT"].as_str().unwrap_or("").into(),
        order_time: output["ORD_TMD"].as_str().unwrap_or("").into(),
    })
}
