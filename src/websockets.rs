use crate::config;
mod approval_key;
use approval_key::get_websocket_key;

use chrono::NaiveDateTime;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, protocol::Message as WsMessage};
use thiserror::Error;
use log::{info, error};

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Approval key error: {0}")]
    ApprovalKeyError(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub(crate) async fn connect_websocket() -> Result<(), WebSocketError> {
    let config = config::get();
    let url = &config.ws_url;
    
    loop {
        let approval_key = get_websocket_key().await.map_err(|e| WebSocketError::ApprovalKeyError(e.to_string()))?;
        
        let request = url.into_client_request()?;
        // let ws_stream = connect_with_retry(request).await?;

        let (ws_stream, _) = connect_async(request).await?;

        info!("Handshaking successfully completed");
        
        if let Err(e) = handle_websocket(ws_stream, &approval_key).await {
            error!("WebSocket error: {}. Reconnecting...", e);
            sleep(Duration::from_secs(5)).await;  // Add delay before reconnecting
        }
    }
}

mod message {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub header: Header,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Header {
        pub tr_id: String,
        pub datetime: String,
    }

    impl TryFrom<&str> for Message {
        type Error = serde_json::Error;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            serde_json::from_str(s)
        }
    }

    #[derive(Debug)]
    pub enum TransactionId {
        PingPong,
        Other(String),
    }

    impl FromStr for TransactionId {
        type Err = ();
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "PINGPONG" => TransactionId::PingPong,
                _ => TransactionId::Other(s.to_string()),
            })
        }
    }
}

use message::{Message, TransactionId};

async fn handle_websocket(mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, _approval_key: &str) -> Result<(), WebSocketError> {
    loop {
        select! {
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => handle_message(&mut ws_stream, msg).await?,
                    Some(Err(e)) => {
                        error!("Error while receiving message: {}", e);
                        break;
                    }
                    None => {
                        info!("WebSocket stream ended");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn handle_message(ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, message: WsMessage) -> Result<(), WebSocketError> {
    match message {
        WsMessage::Text(text) => {
            let parsed_message: Message = text.as_str().try_into()?;
            match parsed_message.header.tr_id.parse() {
                Ok(TransactionId::PingPong) => {
                    if let Ok(datetime) = NaiveDateTime::parse_from_str(&parsed_message.header.datetime, "%Y%m%d%H%M%S") {
                        info!("PP {}", datetime);
                    }
                    ws_stream.send(WsMessage::Pong(text.into_bytes())).await?;
                },
                Ok(TransactionId::Other(_)) => on_received_text(&text)?,
                Err(_) => return Err(WebSocketError::Other("Failed to parse transaction ID".to_string())),
            }
        },
        WsMessage::Binary(bin) => {
            info!("Received binary message: {:?}", bin);
        }
        WsMessage::Ping(payload) => {
            info!("Received ping with payload: {:?}", payload);
            ws_stream.send(WsMessage::Pong(payload)).await?;
        }
        WsMessage::Pong(payload) => {
            info!("Received pong with payload: {:?}", payload);
        }
        WsMessage::Frame(frame) => {
            info!("Received frame: {:?}", frame);
        }
        WsMessage::Close(_) => {
            info!("Connection closed by server.");
        }
    }
    Ok(())
}

fn on_received_text(message: &str) -> Result<(), WebSocketError> {
    info!("Received text message: {}", message);
    Ok(())
}


/*
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};


async fn subscribe_stock_price(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, approval_key: &str) {
    let (mut write, _) = ws_stream.split();

    // Example for subscribing real-time stock price of 'Samsung: 005930'
    // Create your message payload
    let message_payload = json!({
        "header": {
            "approval_key": approval_key,
            "custtype": "P",
            "tr_type": "1",
            "content-type": "utf-8"
        },
        "body": {
            "input": {
                "tr_id": "H0STCNT0",
                "tr_key": "005930"
            }
        }
    });

    // Send Subscribe request: *Should be sent if the session wasn't created yet.
    if false {
        write.send(WsMessage::Text(message_payload.to_string())).await.expect("Failed to send message");
        println!("Message sent: {}", message_payload);
    }
}
*/