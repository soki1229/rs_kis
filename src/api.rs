use std::time::Duration;
use oauth_certification::TokenInfo;
use reqwest::{Client, Response};
use crate::error::RestfulError as Error;
use log::{info, warn, error};

mod oauth_certification;
mod overseas_stock_price;

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
    pub socket_key: String,
    pub token_info: TokenInfo,
}

impl Api {
    pub fn new(domain: &str) -> Self {
        Self {
            client: Client::new(),
            config: Config {
                domain: String::from(domain),
                default_timeout: Duration::from_secs(30),
                max_retries: 3,
            },
            socket_key: String::new(),
            token_info: TokenInfo::new(),
        }
    }

    pub async fn initialize_oauth_certifiaction(&mut self) -> Result<(), Error> {
        self.update_oauth_websocket().await;
        self.update_oauth_api().await;

        Ok(())
    }

    async fn update_oauth_websocket(&mut self) {
        self.socket_key = match oauth_certification::issue_oauth_websocket(&self.client, &self.config).await {
            Ok(response) => response.approval_key,
            _ => String::new(),
        }
    }

    async fn update_oauth_api(&mut self) {
        if let Ok(Some(token_info)) = load_token_from_file("~/.soki1229/kis_api/access_token") {
            self.token_info = token_info;
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
                match save_token_to_file(&response, "~/.soki1229/kis_api/access_token") {
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

    pub async fn request(&mut self) -> Result<Response, Error> {
        self.update_oauth_api().await;
        overseas_stock_price::current_transaction_price(&self.client, &self.config, &self.token_info.get_token()).await
    }
}

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use shellexpand;

fn save_token_to_file(token_info: &TokenInfo, file_path: &str) -> std::io::Result<()> {
    let expanded_path = shellexpand::tilde(file_path).into_owned();
    let path = PathBuf::from(&expanded_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string(token_info)?;
    let mut file = File::create(&expanded_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_token_from_file(file_path: &str) -> std::io::Result<Option<TokenInfo>> {
    let expanded_path = shellexpand::tilde(file_path);
    let mut file = match File::open(expanded_path.as_ref()) {
        Ok(file) => file,
        Err(_) => return Ok(None),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let token_info: TokenInfo = serde_json::from_str(&contents)?;
    Ok(Some(token_info))
}