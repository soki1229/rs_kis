use reqwest::{Client, Method};
use crate::environment;
use crate::api::{Config, http};
use crate::error::RestfulError as Error;
use serde::{Deserialize, Serialize};
use log::{debug, error};

#[derive(Serialize, Debug)]
struct Request {
    grant_type: String,
    appkey: String,
    secretkey: String,
}

#[derive(Deserialize, Debug)]
struct KeyResponse {
    approval_key: String,
}

pub async fn issue_oauth_websocket(client: &Client, config: &Config) -> Result<String, Error> {
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
    
    if approval_response.approval_key.is_empty() {
        error!("Received empty approval key");
        Err(Error::MissingApprovalKey)
    } else {
        debug!("Approval key granted: [{}]", approval_response.approval_key);
        Ok(approval_response.approval_key)
    }
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    access_token_token_expired: String,
}

pub async fn issue_oauth_api(client: &Client, config: &Config) -> Result<String, Error> {
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

    let approval_response: TokenResponse = response.json().await?;
    
    if approval_response.access_token.is_empty() {
        error!("Received empty approval token");
        Err(Error::MissingApprovalKey)
    } else {
        debug!("Approval token granted: [{}]", approval_response.access_token);
        Ok(approval_response.access_token)
    }
}
