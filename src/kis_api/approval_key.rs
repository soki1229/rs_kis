use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct ApprovalResponse {
    approval_key: String,
}

pub(crate) async fn get_websocket_key(app_key: &str, app_secret: &str) -> Result<String, Box<dyn Error>> {
    let client  = reqwest::Client::new();
    let domain  = std::env::var("VTS_DOMAIN").expect("DOMAIN must be set");
    let port    = std::env::var("VTS_PORT").expect("PORT must be set");
    let url     = String::new() + &domain + ":" + &port + "/oauth2/Approval";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": app_key,
        "secretkey": app_secret
    });

    let response: ApprovalResponse = client.post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.approval_key)
}