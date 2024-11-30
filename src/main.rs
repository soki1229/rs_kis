mod config;
mod kis_api;

use kis_api::approval_key::get_websocket_key;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, protocol::Message as WsMessage};
use futures::{SinkExt, StreamExt};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    tr_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    header: Header,
    // Add other fields as necessary
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // .env 파일에서 환경 변수 로드
    config::init();
    let config = config::get();

    // 웹소켓 접속키 발급
    let websocket_key = get_websocket_key(&config.app_key, &config.app_secret).await?;
    
    println!("웹소켓 접속키: {}", websocket_key);

    // 웹소켓 연결
    tokio::spawn(async move {
        if let Err(e) = connect_websocket(&websocket_key).await {
            eprintln!("WebSocket error: {}", e);
        }
    });

    // Keep the main function running to see the logs
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    println!("Received Ctrl+C, shutting down gracefully.");
    
    Ok(())
}


async fn connect_websocket(token: &str) -> Result<(), Box<dyn Error>> {
    println!("Connecting WebSocket...");

    let request = "ws://ops.koreainvestment.com:31000/".into_client_request().unwrap();
    // Connect to the WebSocket
    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");

    println!("WebSocket handshake has been successfully completed");

    // Example for subscribing real-time stock price of 'Samsung: 005930'
    // Create your message payload
    let message_payload = json!({
        "header": {
            "approval_key": token,
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

    // Send the message over the WebSocket
    let (mut write, mut read) = ws_stream.split();

    write.send(WsMessage::Ping(vec![])).await.expect("Failed to send ping");

    // Send Subscribe request: *Should be sent if the session wasn't created yet.
    if false {
        write.send(WsMessage::Text(message_payload.to_string())).await.expect("Failed to send message");
        println!("Message sent: {}", message_payload);
    }

    // Receive messages from the WebSocket
    while let Some(message) = read.next().await {
        match message {
            Ok(WsMessage::Text(text)) => {
                println!("Received text message: {}", text);
                
                // Attempt to parse the text as JSON
                match serde_json::from_str::<Message>(&text) {
                    Ok(parsed_message) => {
                        // Check if the header contains the specific tr_id
                        if parsed_message.header.tr_id == "PINGPONG" {
                            // Send a ping message
                            write.send(WsMessage::Pong(text.into())).await.expect("Failed to send pong");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message: {}", e);
                        // Optionally handle different types of errors here
                    }
                }
            }
            Ok(WsMessage::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
            }
            Ok(WsMessage::Close(_)) => {
                println!("Connection closed by server.");
                break;
            }
            Err(e) => {
                eprintln!("Error while receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }

    println!("Exiting...");

    Ok(())
}