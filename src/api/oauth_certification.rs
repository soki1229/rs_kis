use crate::configurations::Configurations;
use crate::core::http;
use crate::credentials::AccessToken;
use crate::credentials::{CredentialProvider, Credentials};
use crate::error::KisClientError as Error;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use log::{debug, error, info};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

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

pub async fn issue_oauth_websocket(
    client: &Client,
    config: &Configurations,
    credential: &Credentials,
) -> Result<KeyResponse, Error> {
    let app_key = credential.app_key().to_string();
    let app_secret = credential.app_secret().to_string();

    let body_data = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &app_key,
        "secretkey": &app_secret,
    });

    let response = http::execute_api_call(
        client,
        config,
        "/oauth2/Approval",
        Method::POST,
        None,
        Some(body_data),
        None,
    )
    .await?;

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

pub async fn issue_oauth_api(
    client: &Client,
    config: &Configurations,
    credential: &Credentials,
) -> Result<AccessToken, Error> {
    let app_key = credential.app_key().to_string();
    let app_secret = credential.app_secret().to_string();

    let body_data = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &app_key,
        "appsecret": &app_secret,
    });

    let response = http::execute_api_call(
        client,
        config,
        "/oauth2/tokenP",
        Method::POST,
        None,
        Some(body_data),
        None,
    )
    .await?;

    let approval_response: AccessToken = response.json().await?;

    Ok(approval_response)

    // if approval_response.access_token.is_empty() {
    //     error!("Received empty approval token");
    //     Err(Error::MissingApprovalKey)
    // } else {
    //     debug!("Approval token granted: [{}]", approval_response.access_token);
    //     Ok(approval_response.access_token)
    // }
}
