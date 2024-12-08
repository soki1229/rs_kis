use reqwest::{Client, Method};
use crate::environment;
use crate::api::{Config, http};
use crate::error::RestfulError as Error;
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
struct Oauth2Approval {
    approval_key: String,
}

pub async fn issue_websocket_access_key(client: Arc<Client>, config: Arc<Config>) -> Result<String, Error> {
    let env = environment::get();
    let response = http::execute_api_call(
        client,
        config,
        "/oauth2/Approval",
        Method::POST,
        None,
        Some(serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": env.app_key,
            "secretkey": env.app_secret
        })),
    ).await?;

    let approval_response: Oauth2Approval = response.json().await?;
    
    if approval_response.approval_key.is_empty() {
        error!("Received empty approval key");
        Err(Error::MissingApprovalKey)
    } else {
        info!("Approval key granted: [{}]", approval_response.approval_key);
        Ok(approval_response.approval_key)
    }
}