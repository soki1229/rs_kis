use crate::config;
mod approval_key;
use approval_key::get_websocket_key;

use chrono::NaiveDateTime;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, protocol::Message as WsMessage};

pub(crate) async fn connect_websocket() -> Result<(), Box<dyn Error>> {
    // Retrieve configured environment variables
    let config = config::get();
    let url = &config.ws_url;
    
    // TODO: WebSocket should be able to disconnected or connected depends on needs
    loop {
        // Obtain WebSocket connection token from server (REST)
        let approval_key = get_websocket_key().await?;
        
        // Connect to the WebSocket
        let request = url.into_client_request()?;
        let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
        
        println!("[WebSocket] Handshaking successfully completed");
        
        handle_websocket(ws_stream, &approval_key).await;   
    }
    // Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    header: Header,
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    tr_id: String,
    datetime: String,
}

#[warn(unused_variables)]
async fn handle_websocket(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, _approval_key: &str) {    
    let (mut write, mut read) = ws_stream.split();

    // Receive messages from the WebSocket
    while let Some(message) = read.next().await {
        match message {
            Ok(WsMessage::Text(text)) => {
                if let Ok(parsed_message) = serde_json::from_str::<Message>(&text) {
                    match parsed_message.header.tr_id.as_str() {
                        "PINGPONG" => {
                            if let Ok(datetime) = NaiveDateTime::parse_from_str(parsed_message.header.datetime.as_str(), "%Y%m%d%H%M%S") {
                                println!("PP {}", datetime);
                            }

                            write.send(WsMessage::Pong(text.into())).await.expect("Failed to send pong");
                        },
                        _ => on_received_text(&text),
                    }
                } else {
                    eprintln!("Failed to parse JSON message");
                }
            }
            Ok(WsMessage::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
            }
            Ok(WsMessage::Ping(payload)) => {
                println!("Received ping with payload: {:?}", payload);
                write.send(WsMessage::Pong(payload)).await.expect("Failed to send pong");
            }
            Ok(WsMessage::Pong(payload)) => {
                println!("Received pong with payload: {:?}", payload);
            }
            Ok(WsMessage::Frame(frame)) => {
                println!("Received frame: {:?}", frame);
            }
            Ok(WsMessage::Close(_)) => {
                println!("Connection closed by server.");
                break;
            }
            Err(e) => {
                eprintln!("Error while receiving message: {}", e);
                break;
            }
        }
    }
}

fn on_received_text(message: &str) {
    println!("Received text message: {}", message);
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