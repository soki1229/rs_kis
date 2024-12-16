use crate::configurations::{ConfigurationProvider, Configurations};
use crate::core::file_io;
use crate::credentials::{AccessToken, CredentialProvider, Credentials};
use crate::error::KisClientError;
use crate::{api, websockets};

use futures::future::OrElse;
use std::error::Error;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

use log::{error, info};
use reqwest::{Client, Response};

static ACCESS_TOKEN_PATH: &str = "~/.soki1229/kis_api/access_token";

trait ClientProvider {
    fn local_access_token(&self) -> String;
}

impl ClientProvider for KisClient {
    fn local_access_token(&self) -> String {
        format!("{}_{}", ACCESS_TOKEN_PATH, self.configuration.trade_type())
    }
}

pub struct KisClient {
    client: reqwest::Client,
    credential: Credentials,
    configuration: Configurations,
    callback: Option<Box<dyn Fn(&str) + Send + Sync>>,
    websocket_manager: Option<Arc<Mutex<websockets::KisSocket>>>,
}
use std::sync::{Arc, Mutex};

impl KisClient {
    pub fn new(app_key: String, app_secret: String, account_num: String) -> Self {
        Self {
            client: Client::new(),
            credential: Credentials::new(app_key, app_secret, account_num),
            configuration: Configurations::new(),
            callback: None,
            websocket_manager: None,
        }
    }

    pub fn mock(mut self) -> Self {
        self.configuration.mockup();
        self
    }

    pub fn callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn create_connection(mut self) -> Self {
        // TODO: is_mock validation
        self.websocket_manager = Some(Arc::new(Mutex::new(websockets::KisSocket::new(
            String::from(self.configuration.socket_endpoint()),
            Self::on_websocket_callback,
        ))));
        self
    }

    // Method to call the callback
    async fn call_callback(&self, message: Result<Response, KisClientError>) {
        if let Some(ref callback) = self.callback {
            if let Ok(rsp) = message {
                callback(&rsp.text().await.unwrap());
            };
        }
    }

    pub async fn disconnect(&mut self) {
        if let Some(websocket_manager) = &self.websocket_manager {
            websocket_manager.lock().unwrap().disconnect().await;
        }
    }

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

    async fn update_oauth_api(&mut self) -> Result<(), KisClientError> {
        if let Ok(Some(json_string)) = file_io::load_token_from_file(&self.local_access_token()) {
            *self.credential.access_token_mut() = serde_json::from_str(&json_string)?;
        }

        if let Ok(true) = self.credential.access_token_mut().is_expired() {
            self.issue_new_token().await
        } else {
            self.credential.access_token_mut().update()
        }
    }

    async fn issue_new_token(&mut self) -> Result<(), KisClientError> {
        let result = api::oauth_certification::issue_oauth_api(
            &self.client,
            &self.configuration,
            &self.credential,
        )
        .await;

        match result {
            Ok(response) => {
                info!("New access_token granted.");

                // Serialize the response to JSON
                let json_string = serde_json::to_string(&response)
                    .map_err(|e| format!("Failed to serialize token: {}", e))?;

                // Save the token to file
                match file_io::save_token_to_file(&json_string, &self.local_access_token()) {
                    Ok(_) => info!("Archived access_token."),
                    Err(e) => error!("Failed to archive: {:?}", e),
                }

                // Update the credentials
                *self.credential.access_token_mut() = response;

                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn check_deposit(&mut self) {
        if let Err(e) = self.update_oauth_api().await {
            info!("Failed updating access_token: {}", e);
        }

        let result = api::overseas::order_deposit::check_deposit(
            &self.client,
            &self.configuration,
            &self.credential,
        )
        .await;

        self.call_callback(result).await;
    }

    pub async fn current_transaction_price(&mut self, symbol: &str) {
        if let Err(e) = self.update_oauth_api().await {
            info!("Failed updating access_token: {}", e);
        }

        let result = api::overseas::stock_price::current_transaction_price(
            &self.client,
            &self.configuration,
            &self.credential,
            symbol,
        )
        .await;

        self.call_callback(result).await;
    }

    // WebSockets API
    async fn update_oauth_websocket(&mut self) {
        *self.credential.approval_key_mut() = match api::oauth_certification::issue_oauth_websocket(
            &self.client,
            &self.configuration,
            &self.credential,
        )
        .await
        {
            Ok(response) => response.approval_key,
            _ => String::new(),
        }
    }

    pub async fn subscribe_transaction(
        &mut self,
        symbol: &str,
        subscribe: bool,
    ) -> Result<(), KisClientError> {
        self.update_oauth_websocket().await;
        websockets::subscribe_transaction(symbol, subscribe, &self.credential.approval_key_mut())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kis() {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
        info!("========================================================= rs_KIS =========================================================");

        let app_key = "";
        let app_secret = "";
        let account_num = "";

        let mut client = KisClient::new(
            String::from(app_key),
            String::from(app_secret),
            String::from(account_num),
        )
        // .mock()
        .callback(|message| info!("Received message: {}", message))
        .create_connection();

        // client.disconnect().await;
        // TODO: need to handle event; Handling requested subscription.

        client.check_deposit().await;

        // client
        //     .subscribe_transaction("NVDA", true)
        //     .await
        //     .expect("Error: NVDA");

        // Keep the main function running to see the logs
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");

        info!("\nReceived Ctrl+C, shutting down gracefully.");
        info!("==========================================================================================================================");
    }
}
