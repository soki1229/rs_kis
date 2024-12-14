use crate::core::file_io;
use crate::core::http::Config;
use crate::credentials::{AccessToken, CredentialProvider, Credentials};
use crate::error::KisClientError as Error;
use crate::{api, websockets};

use futures::future::OrElse;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

use log::{error, info};
use reqwest::{Client, Response};

pub struct KisClient {
    credentials: Credentials,
    client: reqwest::Client,
    config: Config,
    is_mock: bool,
    websocket_manager: Option<Arc<Mutex<websockets::KisSocket>>>,
}
use std::sync::{Arc, Mutex};

impl KisClient {
    pub fn new(app_key: String, app_secret: String, account_num: String) -> Self {
        Self {
            credentials: Credentials::new(app_key, app_secret, account_num),
            client: Client::new(),
            config: Config::new(),
            is_mock: false,
            websocket_manager: None,
        }
    }

    pub fn mock(mut self) -> Self {
        self.is_mock = true;
        self
    }

    pub fn connect(mut self) -> Self {
        // TODO: is_mock validation
        let url = "ws://ops.koreainvestment.com:21000";
        self.websocket_manager = Some(Arc::new(Mutex::new(websockets::KisSocket::new(
            String::from(url),
            Self::on_websocket_callback,
        ))));
        self
    }

    pub async fn disconnect(&self) {
        if let Some(websocket_manager) = &self.websocket_manager {
            websocket_manager.lock().unwrap().disconnect().await;
        }
    }

    // pub async fn disconnect(self) {
    //     if let Some(handle) = self.websocket_handle {
    //         handle.abort();

    //         match handle.await {
    //             Ok(_) => info!("Task completed successfully"),
    //             Err(e) => {
    //                 if e.is_cancelled() {
    //                     info!("Task was aborted");
    //                 } else {
    //                     info!("Task failed with error: {:?}", e);
    //                 }
    //             }
    //         }
    //     }
    // }

    fn on_websocket_callback(msg: Message, sender: mpsc::Sender<Message>) {
        match msg {
            Message::Text(text) => {
                info!("received text message: {:?}", text);
                // if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                //     on_received_json(&text).await?;
                // } else {
                //     on_received_text(&text, &mut analyzer).await?;
                // }
            }
            Message::Binary(bin) => {
                info!("Received binary message: {:?}", bin);
            }
            Message::Ping(payload) => {
                info!("Received ping with payload: {:?}", payload);
            }
            Message::Pong(payload) => {
                info!("Received pong with payload: {:?}", payload);
            }
            Message::Frame(frame) => {
                info!("Received frame: {:?}", frame);
            }
            Message::Close(_) => {
                info!("Connection closed by server.");
            }
        }
    }

    // fn connect_websocket(&self) -> tokio::task::JoinHandle<()> {
    //     tokio::task::spawn(async move {
    //         loop {
    //             match tokio_tungstenite::connect_async(
    //                 "ws://ops.koreainvestment.com:21000"
    //                     .to_owned()
    //                     .into_client_request()
    //                     .unwrap(),
    //             )
    //             .await
    //             {
    //                 Ok((ws_stream, _)) => {
    //                     info!("WebSocket connection established.");

    //                     if let Err(e) = websockets::handle_websocket(ws_stream).await {
    //                         error!("WebSocket error: {}. Reconnecting...", e);
    //                     }
    //                 }
    //                 Err(e) => {
    //                     error!("Failed to connect to WebSocket: {}. Retrying...", e);
    //                 }
    //             }

    //             tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    //         }
    //     })
    // }

    async fn update_oauth_api(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(Some(json_string)) =
            file_io::load_token_from_file("~/.soki1229/kis_api/access_token")
        {
            *self.credentials.access_token_mut() = serde_json::from_str(&json_string)
                .map_err(|e| format!("Failed to parse token JSON: {}", e))?;
        }

        if self.credentials.access_token_mut().is_expired()? {
            self.issue_new_token().await?;
        } else {
            self.credentials.access_token_mut().update()?;
        }

        Ok(())
    }

    async fn issue_new_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match api::oauth_certification::issue_oauth_api(&self.client, &self.config).await {
            Ok(response) => {
                info!("New access_token granted.");

                // Serialize the response to JSON
                let json_string = serde_json::to_string(&response)
                    .map_err(|e| format!("Failed to serialize token: {}", e))?;

                // Save the token to file
                match file_io::save_token_to_file(&json_string, "~/.soki1229/kis_api/access_token")
                {
                    Ok(_) => info!("Archived access_token."),
                    Err(e) => error!("Failed to archive: {:?}", e),
                }

                // Update the credentials
                *self.credentials.access_token_mut() = response;

                Ok(())
            }
            Err(e) => {
                error!("Failed to initialize OAuth certification: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn check_deposit(&mut self) -> Result<Response, Error> {
        self.update_oauth_api().await;
        api::overseas::order_deposit::check_deposit(
            &self.client,
            &self.config,
            &self.credentials.token(),
            &self.credentials.account_num(),
        )
        .await
    }

    pub async fn current_transaction_price(&mut self, symbol: &str) -> Result<Response, Error> {
        self.update_oauth_api().await;
        api::overseas::stock_price::current_transaction_price(
            &self.client,
            &self.config,
            &self.credentials.token(),
            symbol,
        )
        .await
    }

    // WebSockets API
    async fn update_oauth_websocket(&mut self) {
        *self.credentials.approval_key_mut() =
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
        websockets::subscribe_transaction(symbol, subscribe, &self.credentials.approval_key_mut())
            .await
    }
}

#[cfg(test)]
mod tests {
    use tokio_tungstenite::tungstenite::connect;

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
        .connect();

        // client.disconnect().await;
        // TODO: need to handle event; Handling requested subscription.

        if let Ok(response) = client.check_deposit().await {
            let parsed = response.text().await.unwrap();
            info!("{}", parsed);
        } else {
            info!("check_depsit returned err");
        }

        // client
        //     .subscribe_transaction("NVDA", true)
        //     .await
        //     .expect("Error: NVDA");

        // Keep the main function running to see the logs
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");

        println!("\nReceived Ctrl+C, shutting down gracefully.");
        println!("==========================================================================================================================");
    }
}
