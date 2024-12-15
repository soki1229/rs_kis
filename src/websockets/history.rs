use log::info;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::config;

#[derive(Serialize, Deserialize, Debug)]
struct ApprovalResponse {
    approval_key: String,
}

pub async fn retrieve_history(symbol: &str, approval_key: &str) -> Result<(), Box<dyn Error>> {
    let config = config::get();

    let client = reqwest::Client::new();
    let url = config.rest_url.clone() + "/uapi/overseas-price/v1/quotations/dailyprice";

    let mut headers = HeaderMap::new();
    // headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("authorization", HeaderValue::from_static(approval_key));
    headers.insert("appkey", HeaderValue::from_static(&config.app_key));
    headers.insert("appsecret", HeaderValue::from_static(&config.app_secret));
    headers.insert("tr_id", HeaderValue::from_static("HHDFS76240000"));

    let body = serde_json::json!({
        "AUTH": "",
        "EXCD": "NAS",
        "SYMB": symbol,
        "GUBN": "0",
        "BYMD": "",
        "MODP": "0"
    });

    let resp = client
        .get(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .text()
        .await?;
    // .await?
    // .json()

    // debug!("approval_key granted: [{}]", response.approval_key);
    info!("Raw response body: {}", resp);

    Ok(())
}
