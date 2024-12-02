use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;
use log::info;

use crate::config;

#[derive(Serialize, Deserialize, Debug)]
struct ApprovalResponse {
    approval_key: String,
}

pub async fn get_websocket_key() -> Result<String, Box<dyn Error>> {
    let config = config::get();

    let client  = reqwest::Client::new();
    let url     = config.rest_url.clone() + "/oauth2/Approval";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": config.app_key,
        "secretkey": config.app_secret
    });

    let response: ApprovalResponse = client.post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

        info!("Allocated approval_key: {}", response.approval_key);

    Ok(response.approval_key)
}
