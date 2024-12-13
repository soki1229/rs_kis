use crate::core::file_io;
use crate::core::http::Config;
use crate::error::KisClientError as Error;
use crate::{api, types, websockets};

use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use types::TokenInfo;

use log::{error, info};
use reqwest::{Client, Response};

pub struct KisClient {
    app_key: String,
    app_secret: String,
    account_num: String,
    socket_key: String,
    token_info: TokenInfo,
    client: reqwest::Client,
    config: Config,
    is_mock: bool,
    websocket_handle: Option<tokio::task::JoinHandle<()>>,
}

impl KisClient {
    pub fn new(app_key: String, app_secret: String, account_num: String) -> Self {
        Self {
            app_key,
            app_secret,
            account_num,
            socket_key: String::new(),
            token_info: TokenInfo::new(),
            client: Client::new(),
            config: Config::new(),
            is_mock: false,
            websocket_handle: None,
        }
    }

    pub fn mock(mut self) -> Self {
        self.is_mock = true;
        self
    }

    pub fn run(mut self) -> Self {
        self.websocket_handle = Some(self.connect_websocket());
        self
    }

    fn connect_websocket(&self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(async move {
            loop {
                match tokio_tungstenite::connect_async(
                    "ws://ops.koreainvestment.com:21000"
                        .to_owned()
                        .into_client_request()
                        .unwrap(),
                )
                .await
                {
                    Ok((ws_stream, _)) => {
                        info!("WebSocket connection established.");

                        if let Err(e) = websockets::handle_websocket(ws_stream).await {
                            error!("WebSocket error: {}. Reconnecting...", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to connect to WebSocket: {}. Retrying...", e);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    }

    async fn update_oauth_api(&mut self) {
        if let Ok(Some(json_string)) =
            file_io::load_token_from_file("~/.soki1229/kis_api/access_token")
        {
            self.token_info = serde_json::from_str(&json_string).unwrap();
        }

        if self.token_info.is_expired() {
            self.issue_new_token().await;
        }

        self.token_info.update();
    }

    async fn issue_new_token(&mut self) {
        self.token_info =
            match api::oauth_certification::issue_oauth_api(&self.client, &self.config).await {
                Ok(response) => {
                    info!("New access_token granted.");
                    match file_io::save_token_to_file(
                        &serde_json::to_string(&response).unwrap(),
                        "~/.soki1229/kis_api/access_token",
                    ) {
                        Ok(_) => info!("Archived access_token."),
                        Err(e) => error!("Failed to archive: {:?}", e),
                    };
                    response
                }
                Err(e) => {
                    error!("Failed to initialize OAuth certification: {:?}", e);
                    TokenInfo::new()
                }
            }
    }

    pub async fn check_deposit(&mut self) -> Result<Response, Error> {
        self.update_oauth_api().await;
        api::overseas::order_deposit::check_deposit(
            &self.client,
            &self.config,
            &self.token_info,
            &self.account_num,
        )
        .await
    }

    pub async fn current_transaction_price(&mut self, symbol: &str) -> Result<Response, Error> {
        self.update_oauth_api().await;
        api::overseas::stock_price::current_transaction_price(
            &self.client,
            &self.config,
            &self.token_info.get_token(),
            symbol,
        )
        .await
    }

    // WebSockets API
    async fn update_oauth_websocket(&mut self) {
        self.socket_key =
            match api::oauth_certification::issue_oauth_websocket(&self.client, &self.config).await
            {
                Ok(response) => response.approval_key,
                _ => String::new(),
            }
    }

    pub async fn subscribe_transaction(
        &mut self,
        symbol: &str,
        subscribe: bool,
    ) -> Result<(), Error> {
        self.update_oauth_websocket().await;
        websockets::subscribe_transaction(symbol, subscribe, &self.socket_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment;
    use crate::logger;

    #[tokio::test]
    async fn test_kis() {
        logger::init_logging();
        println!("========================================================= rs_KIS =========================================================");

        environment::init();
        let env_var = environment::get();

        let mut client = KisClient::new(
            String::from(&env_var.app_key),
            String::from(&env_var.app_secret),
            String::from(&env_var.account_num),
        )
        // .mock()
        .run();

        // TODO: need to handle event; Handling requested subscription.

        // if let Ok(response) = client.check_deposit().await{
        //     let parsed = response.text().await.unwrap();
        //     info!("{}", parsed);
        // } else {
        //     info!("check_depsit returned err");
        // }

        client
            .subscribe_transaction("NVDA", true)
            .await
            .expect("Error: NVDA");

        // Keep the main function running to see the logs
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");

        println!("\nReceived Ctrl+C, shutting down gracefully.");
        println!("==========================================================================================================================");
    }
}
