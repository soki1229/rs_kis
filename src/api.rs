use std::time::Duration;
use oauth_certification::TokenInfo;
use reqwest::{Client, Response};
use crate::error::KisClientError as Error;
use crate::websockets;
use crate::core::file_io;
use log::{info, warn, error};

mod oauth_certification;
mod overseas;

mod http;

#[derive(Clone, Debug, Default)]
pub struct Config {
    domain: String,
    default_timeout: Duration,
    max_retries: u32,
}

pub struct Api {
    client: Client,
    config: Config,
    account_num: String,
    pub socket_key: String,
    pub token_info: TokenInfo,
}

impl Api {
    pub fn new(account_num: &str, domain: &str) -> Self {
        Self {
            client: Client::new(),
            config: Config {
                domain: String::from(domain),
                default_timeout: Duration::from_secs(30),
                max_retries: 3,
            },
            account_num: String::from(account_num),
            socket_key: String::new(),
            token_info: TokenInfo::new(),
        }
    }

    // REST API
    async fn update_oauth_api(&mut self) {
        if let Ok(Some(json_string)) = file_io::load_token_from_file("~/.soki1229/kis_api/access_token") {
            self.token_info = serde_json::from_str(&json_string).unwrap();
        }

        if self.token_info.is_expired() {
            self.issue_new_token().await;
        }

        self.token_info.update();
    }

    async fn issue_new_token(&mut self) {
        self.token_info = match oauth_certification::issue_oauth_api(&self.client, &self.config).await {
            Ok(response) => {
                info!("New access_token granted.");
                match file_io::save_token_to_file(&serde_json::to_string(&response).unwrap(), "~/.soki1229/kis_api/access_token") {
                    Ok(_) => info!("Archived access_token."),
                    Err(e) => error!("Failed to archive: {:?}", e),
                };
                response
            },
            Err(e) => {
                error!("Failed to initialize OAuth certification: {:?}", e);
                TokenInfo::new()
            }
        }
    }

    pub async fn check_deposit(&mut self) -> Result<Response, Error> {
        self.update_oauth_api().await;
        overseas::order_deposit::check_deposit(&self.client, &self.config, &self.token_info, &self.account_num).await
    }

    pub async fn current_transaction_price(&mut self, symbol: &str) -> Result<Response, Error> {
        self.update_oauth_api().await;
        overseas::stock_price::current_transaction_price(&self.client, &self.config, &self.token_info.get_token(), symbol).await
    }

    // WebSockets API
    async fn update_oauth_websocket(&mut self) {
        self.socket_key = match oauth_certification::issue_oauth_websocket(&self.client, &self.config).await {
            Ok(response) => response.approval_key,
            _ => String::new(),
        }
    }

    pub async fn subscribe_transaction(&mut self, symbol: &str, subscribe: bool) -> Result<(), Error> {
        self.update_oauth_websocket().await;
        websockets::subscribe_transaction(symbol, subscribe, &self.socket_key).await
    }

}