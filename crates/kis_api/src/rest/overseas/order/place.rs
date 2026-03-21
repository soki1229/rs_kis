use reqwest::Method;
use rust_decimal::Decimal;
use serde_json::Value;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::{Exchange, OrderSide, OrderType, order_tr_id, split_account};
use crate::{KisConfig, KisError};

/// 해외주식 주문 요청
#[derive(Debug, Clone)]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub qty: Decimal,
    pub price: Option<Decimal>,
}

/// 해외주식 주문 응답
#[derive(Debug, Clone)]
pub struct PlaceOrderResponse {
    pub order_date: String,
    pub order_org_no: String,
    pub order_time: String,
}

fn parse_place_order_response(v: &Value) -> Result<PlaceOrderResponse, KisError> {
    let output = &v["output"];
    let order_date = output["ORDS_PRCS_DT"]
        .as_str()
        .ok_or_else(|| KisError::Api {
            code: "PARSE".into(),
            message: "ORDS_PRCS_DT missing".into(),
        })?
        .to_string();
    let order_org_no = output["KRX_FWDG_ORD_ORGNO"]
        .as_str()
        .ok_or_else(|| KisError::Api {
            code: "PARSE".into(),
            message: "KRX_FWDG_ORD_ORGNO missing".into(),
        })?
        .to_string();
    let order_time = output["ORD_TMD"]
        .as_str()
        .ok_or_else(|| KisError::Api {
            code: "PARSE".into(),
            message: "ORD_TMD missing".into(),
        })?
        .to_string();
    Ok(PlaceOrderResponse { order_date, order_org_no, order_time })
}

/// 해외주식 주문 실행
pub async fn place_order(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: PlaceOrderRequest,
) -> Result<PlaceOrderResponse, KisError> {
    let tr_id = order_tr_id(&req.exchange, &req.side, config.mock);
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let price_str = req.price
        .map(|p| p.to_string())
        .unwrap_or_else(|| "0".to_string());

    let body = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "OVRS_EXCG_CD": req.exchange.to_string(),
        "PDNO": req.symbol,
        "ORD_DVSN": req.order_type.to_ord_dvsn(),
        "ORD_QTY": req.qty.to_string(),
        "OVRS_ORD_UNPR": price_str,
        "CTAC_TLNO": "",
        "MGCO_APTM_EDNO": "",
        "ORD_SVR_DVSN_CD": "0",
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::POST,
            path: "/uapi/overseas-stock/v1/trading/order",
            tr_id,
            query: None,
            body: Some(&body),
        },
    )
    .await?;

    parse_place_order_response(&resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn fixture() -> Value {
        let json_str = include_str!("../../../../tests/fixtures/overseas/order/place_order.json");
        serde_json::from_str(json_str).expect("valid JSON fixture")
    }

    #[test]
    fn deserialize_place_order_response() {
        let v = fixture();
        let resp = parse_place_order_response(&v).expect("parse should succeed");
        assert_eq!(resp.order_date, "20260321");
        assert_eq!(resp.order_org_no, "91252");
        assert_eq!(resp.order_time, "141922");
    }

    #[test]
    fn place_order_request_fields() {
        let req = PlaceOrderRequest {
            symbol: "AAPL".to_string(),
            exchange: Exchange::NASD,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            qty: dec!(10),
            price: Some(dec!(150.00)),
        };
        assert_eq!(req.symbol, "AAPL");
        assert_eq!(req.exchange, Exchange::NASD);
        assert_eq!(req.side, OrderSide::Buy);
        assert_eq!(req.order_type, OrderType::Limit);
        assert_eq!(req.qty, dec!(10));
        assert_eq!(req.price, Some(dec!(150.00)));
    }
}
