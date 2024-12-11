use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use reqwest::{Client, Method};
use crate::environment;
use crate::api::{Config, http};
use crate::error::RestfulError as Error;
use serde::{Serialize, Deserialize};
use log::{info, debug, error};

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

pub async fn issue_oauth_websocket(client: &Client, config: &Config) -> Result<KeyResponse, Error> {
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

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    access_token: String,
    access_token_token_expired: String,
    token_type: String,
    expires_in: u32,
}

impl TokenInfo {
    pub fn new() -> Self {
        TokenInfo {
            access_token: String::new(),
            access_token_token_expired: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            token_type: String::new(),
            expires_in: 0,
        }
    }

    pub fn get_token(&self) -> String {
        self.access_token.clone()
    }

    pub fn is_token_expired(&self) -> bool {
        // Check if the token has already expired based on `expires_in`
        if self.expires_in == 0 {
            return true;
        }

        // Attempt to parse the expiration time
        let naive_dt = match NaiveDateTime::parse_from_str(&self.access_token_token_expired, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => dt,
            Err(_) => {
                error!("Failed to parse access_token_token_expired: {}", self.access_token_token_expired);
                return true; // Assume expired if parsing fails
            }
        };

        // Convert NaiveDateTime to DateTime<Utc>
        let expiry_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);

        // Compare the current time with the expiration time
        Utc::now() >= expiry_time
    }
}

pub async fn issue_oauth_api(client: &Client, config: &Config) -> Result<TokenInfo, Error> {
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