use reqwest::Method;
use rust_decimal::Decimal;
use serde_json::Value;

use crate::rest::http::{execute, RequestParams};
use crate::rest::overseas::types::{Exchange, cancel_tr_id, split_account};
use crate::{KisConfig, KisError};

/// 정정/취소 구분
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelKind {
    Amend,  // "01"
    Cancel, // "02"
}

impl CancelKind {
    pub fn to_dvsn_cd(&self) -> &'static str {
        match self {
            CancelKind::Amend  => "01",
            CancelKind::Cancel => "02",
        }
    }
}

/// 해외주식 정정/취소 주문 요청
#[derive(Debug, Clone)]
pub struct CancelOrderRequest {
    pub symbol: String,
    pub exchange: Exchange,
    pub original_order_id: String,
    pub kind: CancelKind,
    pub qty: Decimal,
    pub price: Option<Decimal>,
}

/// 해외주식 정정/취소 주문 응답
#[derive(Debug, Clone)]
pub struct CancelOrderResponse {
    pub order_date: String,
    pub order_org_no: String,
    pub order_time: String,
}

/// 해외주식 정정/취소 주문 실행
pub async fn cancel_order(
    http: &reqwest::Client,
    config: &KisConfig,
    token: &str,
    req: CancelOrderRequest,
) -> Result<CancelOrderResponse, KisError> {
    let tr_id = cancel_tr_id(&req.exchange, config.mock);
    let (cano, acnt_prdt_cd) = split_account(&config.account_num);

    let price_str = req.price
        .map(|p| p.to_string())
        .unwrap_or_else(|| "0".to_string());

    let body = serde_json::json!({
        "CANO": cano,
        "ACNT_PRDT_CD": acnt_prdt_cd,
        "OVRS_EXCG_CD": req.exchange.to_string(),
        "PDNO": req.symbol,
        "ORGN_ODNO": req.original_order_id,
        "RVSE_CNCL_DVSN_CD": req.kind.to_dvsn_cd(),
        "ORD_QTY": req.qty.to_string(),
        "OVRS_ORD_UNPR": price_str,
        "CTAC_TLNO": "",
        "MGCO_APTM_ODNO": "",
    });

    let resp: Value = execute(
        http,
        config,
        token,
        RequestParams {
            method: Method::POST,
            path: "/uapi/overseas-stock/v1/trading/order-rvsecncl",
            tr_id,
            query: None,
            body: Some(&body),
        },
    )
    .await?;

    parse_cancel_order_response(&resp)
}

fn parse_cancel_order_response(v: &Value) -> Result<CancelOrderResponse, KisError> {
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
    Ok(CancelOrderResponse { order_date, order_org_no, order_time })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn fixture() -> Value {
        let json_str = include_str!("../../../../tests/fixtures/overseas/order/cancel_order.json");
        serde_json::from_str(json_str).expect("valid JSON fixture")
    }

    #[test]
    fn deserialize_cancel_order_response() {
        let v = fixture();
        let resp = parse_cancel_order_response(&v).expect("parse should succeed");
        assert_eq!(resp.order_date, "20260321");
        assert_eq!(resp.order_org_no, "91252");
        assert_eq!(resp.order_time, "142230");
    }

    #[test]
    fn cancel_kind_to_dvsn_cd() {
        assert_eq!(CancelKind::Amend.to_dvsn_cd(), "01");
        assert_eq!(CancelKind::Cancel.to_dvsn_cd(), "02");
    }

    #[test]
    fn cancel_order_request_fields() {
        let req = CancelOrderRequest {
            symbol: "AAPL".to_string(),
            exchange: Exchange::NASD,
            original_order_id: "0000117057".to_string(),
            kind: CancelKind::Cancel,
            qty: dec!(5),
            price: None,
        };
        assert_eq!(req.kind, CancelKind::Cancel);
        assert_eq!(req.original_order_id, "0000117057");
        assert_eq!(req.qty, dec!(5));
    }
}
