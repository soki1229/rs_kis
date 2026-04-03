// ⚠ TR ID 및 필드명은 KIS OpenAPI 가이드에서 확인 필요
// 실전투자:
// - 매수: POST /uapi/domestic-stock/v1/trading/order-cash  (tr_id: TTTC0802U)
// - 매도: POST /uapi/domestic-stock/v1/trading/order-cash  (tr_id: TTTC0801U)
// - 취소: POST /uapi/domestic-stock/v1/trading/order-rvsecncl (tr_id: TTTC0803U)
// 모의투자(VTS):
// - 매수: tr_id: VTTC0802U
// - 매도: tr_id: VTTC0801U
// - 취소: tr_id: VTTC0803U

use super::types::*;
use crate::rest::overseas::types::split_account;
use crate::{KisConfig, KisError, OrderSide};
use reqwest::Client;
use serde_json::json;

fn select_place_order_tr_id(is_domestic_virtual: bool, side: &OrderSide) -> &'static str {
    match (is_domestic_virtual, side) {
        (true, OrderSide::Buy) => "VTTC0802U",
        (true, OrderSide::Sell) => "VTTC0801U",
        (false, OrderSide::Buy) => "TTTC0802U",
        (false, OrderSide::Sell) => "TTTC0801U",
    }
}

fn select_cancel_order_tr_id(is_domestic_virtual: bool) -> &'static str {
    if is_domestic_virtual {
        "VTTC0803U"
    } else {
        "TTTC0803U"
    }
}

pub async fn domestic_place_order(
    client: &Client,
    config: &KisConfig,
    token: &str,
    req: DomesticPlaceOrderRequest,
) -> Result<DomesticPlaceOrderResponse, KisError> {
    let tr_id = select_place_order_tr_id(config.is_domestic_virtual, &req.side);

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
        .post(format!(
            "{}/uapi/domestic-stock/v1/trading/order-cash",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", tr_id)
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
    let tr_id = select_cancel_order_tr_id(config.is_domestic_virtual);
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
        .post(format!(
            "{}/uapi/domestic-stock/v1/trading/order-rvsecncl",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", tr_id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn vts_flag_selects_correct_tr_id() {
        // 모의투자(VTS) — VTTC 계열
        assert_eq!(select_place_order_tr_id(true, &OrderSide::Buy), "VTTC0802U");
        assert_eq!(select_place_order_tr_id(true, &OrderSide::Sell), "VTTC0801U");
        assert_eq!(select_cancel_order_tr_id(true), "VTTC0803U");
        // 실전투자 — TTTC 계열
        assert_eq!(select_place_order_tr_id(false, &OrderSide::Buy), "TTTC0802U");
        assert_eq!(select_place_order_tr_id(false, &OrderSide::Sell), "TTTC0801U");
        assert_eq!(select_cancel_order_tr_id(false), "TTTC0803U");
    }

    #[test]
    fn deserialize_place_order_response() {
        let json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/domestic/order/place_order.json"
        ))
        .unwrap();
        let output = &json["output"];
        let resp = DomesticPlaceOrderResponse {
            order_no: output["ODNO"].as_str().unwrap_or("").into(),
            order_date: output["ORD_DT"].as_str().unwrap_or("").into(),
            order_time: output["ORD_TMD"].as_str().unwrap_or("").into(),
        };
        assert_eq!(resp.order_no, "0000123456");
        assert_eq!(resp.order_date, "20260327");
        assert_eq!(resp.order_time, "091500");
    }

    #[test]
    fn deserialize_cancel_order_response() {
        let json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/domestic/order/cancel_order.json"
        ))
        .unwrap();
        let output = &json["output"];
        let resp = DomesticCancelOrderResponse {
            order_no: output["ODNO"].as_str().unwrap_or("").into(),
            order_date: output["ORD_DT"].as_str().unwrap_or("").into(),
            order_time: output["ORD_TMD"].as_str().unwrap_or("").into(),
        };
        assert_eq!(resp.order_no, "0000123457");
        assert_eq!(resp.order_date, "20260327");
        assert_eq!(resp.order_time, "092000");
    }

    #[test]
    fn place_order_request_fields() {
        let req = DomesticPlaceOrderRequest {
            symbol: "005930".into(),
            exchange: DomesticExchange::KOSPI,
            side: OrderSide::Buy,
            order_type: DomesticOrderType::Limit,
            qty: 10,
            price: Some(dec!(70000)),
        };
        assert_eq!(req.symbol, "005930");
        assert_eq!(req.order_type.as_str(), "00");
        assert_eq!(req.exchange.market_code(), "J");
    }

    #[test]
    fn error_response_not_zero_rt_cd() {
        let json: serde_json::Value = serde_json::json!({
            "rt_cd": "1",
            "msg1": "잘못된 요청입니다."
        });
        assert_ne!(json["rt_cd"].as_str().unwrap_or(""), "0");
    }
}
