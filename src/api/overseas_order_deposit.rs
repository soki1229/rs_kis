use ::http::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method};
use crate::environment;
use crate::api::{Config, http};
use crate::error::RestfulError as Error;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::Arc;

#[derive(Serialize, Debug)]
struct Request {
    CANO: String,
    ACNT_PRDT_CD: String,
    OVRS_EXCG_CD: String,
    TR_CRCY_CD: String,
    CTX_AREA_FK200: String,
    CTX_AREA_NK200: String,
}

pub async fn check_deposit(client: Arc<Client>, config: Arc<Config>, approval_key: &str) -> Result<String, Error> {
    let env = environment::get();

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("authorization"), HeaderValue::from_str(approval_key).unwrap());
    headers.insert(HeaderName::from_static("appkey"), HeaderValue::from_static(&env.app_key));
    headers.insert(HeaderName::from_static("appsecret"), HeaderValue::from_static(&env.app_secret));
    headers.insert(HeaderName::from_static("tr_id"), HeaderValue::from_static("HHDFS00000300"));
    headers.insert(HeaderName::from_static("tr_cont"), HeaderValue::from_static(""));

    let body_data = Request {
        AUTH : String::from(""),
        EXCD : String::from("NAS"),
        SYMB : String::from("AAPL"),
    };

    let response = http::execute_api_call(
        client,
        config,
        "/uapi/overseas-price/v1/quotations/price",
        Method::GET,
        Some(headers),
        Some(serde_json::to_value(body_data).unwrap()),
    ).await?;

    let response_data = response.text().await?;

    // let approval_response: Response = response.json().await?;
    
    if response_data.is_empty() {
        error!("Received empty approval key");
        Err(Error::MissingApprovalKey)
    } else {
        info!("response_data: [{}]", response_data);
        Ok(response_data)
    }
}