use std::time::Duration;
use reqwest::Client;
use crate::error::RestfulError as Error;


mod oauth_certification;
mod http;

use std::sync::Arc;

use crate::environment;

#[derive(Clone, Debug)]
pub struct Config {
    domain_restful: String,
    default_timeout: Duration,
    max_retries: u32,
}

pub fn initialize() -> (Arc<Client>, Arc<Config>) {
    let env = environment::get();

    let client = Arc::new(Client::new());
    let config = Arc::new(Config {
        domain_restful: String::from(&env.domain_restful),
        default_timeout: Duration::from_secs(30),
        max_retries: 3,
    });

    (client, config)
}

pub async fn issue_websocket_access_key(client: Arc<Client>, config: Arc<Config>) -> Result<String, Error> {
    oauth_certification::issue_websocket_access_key(client, config).await
}