use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use reqwest::{Client, Method};
use crate::environment;
use crate::core::http;
use crate::error::KisClientError as Error;
use serde::{Serialize, Deserialize};
use log::{info, debug, error};

use crate::types;
use types::TokenInfo;

#[derive(Serialize, Debug)]
struct Request {
    grant_type: String,
    appkey: String,
    secretkey: String,
}

#[derive(Deserialize, Debug)]
pub struct KeyResponse {
    pub approval_key: String,
}

pub async fn issue_oauth_websocket(client: &Client, config: &http::Config) -> Result<KeyResponse, Error> {
    let env = environment::get();

    let body_data = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &env.app_key,
        "secretkey": &env.app_secret,
    });

    let response = http::execute_api_call(
        client,
        config,
        "/oauth2/Approval",
        Method::POST,
        None,
        Some(body_data),
        None,
    ).await?;

    let approval_response: KeyResponse = response.json().await?;
    
    Ok(approval_response)

    // if approval_response.approval_key.is_empty() {
    //     error!("Received empty approval key");
    //     Err(Error::MissingApprovalKey)
    // } else {
    //     debug!("Approval key granted: [{}]", approval_response.approval_key);
    //     Ok(approval_response.approval_key)
    // }
}

pub fn current_time() -> DateTime<Utc> {
    Utc::now() + Duration::hours(9)
}

pub async fn issue_oauth_api(client: &Client, config: &http::Config) -> Result<TokenInfo, Error> {
    let env = environment::get();
    
    let body_data = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &env.app_key,
        "appsecret": &env.app_secret,
    });

    let response = http::execute_api_call(
        client,
        config,
        "/oauth2/tokenP",
        Method::POST,
        None,
        Some(body_data),
        None,
    ).await?;

    let approval_response: TokenInfo = response.json().await?;
    
    Ok(approval_response)

    // if approval_response.access_token.is_empty() {
    //     error!("Received empty approval token");
    //     Err(Error::MissingApprovalKey)
    // } else {
    //     debug!("Approval token granted: [{}]", approval_response.access_token);
    //     Ok(approval_response.access_token)
    // }
}