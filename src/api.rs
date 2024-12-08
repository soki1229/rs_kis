use std::time::Duration;
use reqwest::Client;
use crate::error::RestfulError as Error;


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
    pub api_token: String,
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
            api_token: String::new(),
        }
    }

    pub async fn initialize_oauth_certifiaction(&mut self) -> Result<(), Error> {
        self.socket_key = self.issue_oauth_websocket().await?;
        //TODO: check if valid api_token already exists
        self.api_token = self.issue_oauth_api().await?;

        Ok(())
    }

    async fn issue_oauth_websocket(&self) -> Result<String, Error> {
        oauth_certification::issue_oauth_websocket(&self.client, &self.config).await
    }
        
    async fn issue_oauth_api(&self) -> Result<String, Error> {
        oauth_certification::issue_oauth_api(&self.client, &self.config).await
    }

    pub async fn current_transaction_price(&self) -> Result<String, Error> {
        overseas_stock_price::current_transaction_price(&self.client, &self.config, &self.api_token).await
    }
}