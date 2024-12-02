use crate::config;
mod approval_key;
use approval_key::get_websocket_key;

// mod crypto;
// use crypto::aes_cbc_base64_dec;

use chrono::NaiveDateTime;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, protocol::Message as WsMessage};
use thiserror::Error;
use log::{info, warn, error};

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Approval key error: {0}")]
    ApprovalKeyError(String),
    // #[error("Other error: {0}")]
    // Other(String),
}

pub(crate) async fn connect_websocket() -> Result<(), WebSocketError> {
    let config = config::get();
    let url = &config.ws_url ;
    
    loop {
        let approval_key = get_websocket_key().await.map_err(|e| WebSocketError::ApprovalKeyError(e.to_string()))?;
        
        let request = (url.to_owned() + "/tryitout/HDFSCNT0").into_client_request()?;

        let (ws_stream, _) = connect_async(request).await?;

        info!("Handshaking successfully completed");
        
        if let Err(e) = handle_websocket(ws_stream, &approval_key).await {
            error!("WebSocket error: {}. Reconnecting...", e);
            sleep(Duration::from_secs(5)).await;  // Add delay before reconnecting
        }
    }
}

mod message;
use message::{Message, TransactionId};

async fn handle_websocket(mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, approval_key: &str) -> Result<(), WebSocketError> {
    subscribe_stock_price(&mut ws_stream, approval_key).await;
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
            if let Ok(parsed_message) = serde_json::from_str::<Message>(&text) {
                match parsed_message.header.tr_id.parse() {
                    Ok(TransactionId::PingPong) => {
                        if let Ok(datetime) = NaiveDateTime::parse_from_str(&parsed_message.header.datetime, "%Y%m%d%H%M%S") {
                            info!("PP {}", datetime);
                        }
                        ws_stream.send(WsMessage::Pong(text.into_bytes())).await?;
                    },
                    Ok(TransactionId::LiveCostOverseas) => {
                        on_received_json(&text)?;
                    },
                    Ok(TransactionId::Other(_)) => {
                        on_received_json(&text)?;
                    },
                    Err(_) => {
                        // Ignore the error and log it
                        warn!("Received message is not including tr_id");
                    }
                }
            } else {
                on_received_text(&text).await?;
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

async fn subscribe_stock_price(ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, approval_key: &str) {
    // Example for subscribing real-time stock price of 'Apple: DNASAAPL'
    // Create your message payload
    let message_payload = serde_json::json!({
        "header": {
            "approval_key": approval_key,
            "custtype": "P",
            "tr_type": "1",
            "content-type": "utf-8"
        },
        "body": {
            "input": {
                "tr_id": "HDFSCNT0",
                "tr_key": "DNASAAPL"
            }
        }
    });

    // Send Subscribe request: *Should be sent if the session wasn't created yet.
    ws_stream.send(WsMessage::Text(message_payload.to_string())).await.expect("Failed to send message");
    println!("Message sent: {}", message_payload);
}

async fn on_received_text(message: &str) -> Result<(), WebSocketError> {
    if message.starts_with('0') {
        let recvstr: Vec<&str> = message.split('|').collect();
        let trid0 = recvstr[1];

        match trid0 {
            "H0STASP0" => {
                info!("#### 주식호가 ####");
                //stockhoka_domestic(recvstr[3]);
                sleep(Duration::from_millis(500)).await;
            },
            "H0STCNT0" => {
                info!("#### 주식체결 ####");
                //let data_cnt: i32 = recvstr[2].parse::<i32>().unwrap();
                //stockspurchase_domestic(data_cnt, recvstr[3]);
                sleep(Duration::from_millis(500)).await;
            },
            "HDFSASP1" => {
                info!("#### 해외주식호가 ####");
                //stockhoka_overseas(recvstr[3]);
                sleep(Duration::from_millis(500)).await;
            },
            "HDFSCNT0" => {
                info!("#### 해외주식체결 ####");
                let data_cnt = recvstr[2].parse::<i32>().unwrap();
                stockspurchase_overseas(data_cnt, recvstr[3]);
                sleep(Duration::from_millis(500)).await;
            },
            _ => {}
        }
    } else if message.starts_with('1') {
        let recvstr: Vec<&str> = message.split('|').collect();
        let trid0 = recvstr[1];

        match trid0 {
            "H0STCNI0" | "H0STCNI9" => {
                //stocksigningnotice_domestic(recvstr[3], aes_key, aes_iv);
            },
            "H0GSCNI0" | "H0GSCNI9" => {
                //stocksigningnotice_overseas(recvstr[3], aes_key, aes_iv);
            },
            _ => {}
        }
    }
    
    Ok(())
}

fn stockspurchase_overseas(data_cnt: i32, data: &str) {
    info!("============================================");
    let menulist = "실시간종목코드|종목코드|수수점자리수|현지영업일자|현지일자|현지시간|한국일자|한국시간|시가|고가|저가|현재가|대비구분|전일대비|등락율|매수호가|매도호가|매수잔량|매도잔량|체결량|거래량|거래대금|매도체결량|매수체결량|체결강도|시장구분";
    let menustr: Vec<&str> = menulist.split('|').collect();
    let pvalue: Vec<&str> = data.split('^').collect();

    for cnt in 0..data_cnt {
        info!("### [{} / {}]", cnt + 1, data_cnt);
        for (menu, value) in menustr.iter().zip(pvalue.iter().skip((cnt as usize) * menustr.len())) {
            info!("{:<13}\t[{}]", menu, value);
        }
    }
}

fn on_received_json(message: &str) -> Result<(), WebSocketError> {
    info!("Received json: {}", message);
    //TODO: parse aes iv and key for future usage 

    Ok(())
}