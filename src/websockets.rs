use crate::config;
mod approval_key;
use approval_key::get_websocket_key;
// mod crypto;
// use crypto::aes_cbc_base64_dec;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
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
    #[error("Error sending message: {0}")]
    SendError(String)
}

pub(crate) async fn connect_websocket() -> Result<(), WebSocketError> {    
    loop {
        let approval_key = get_websocket_key().await.map_err(|e| WebSocketError::ApprovalKeyError(e.to_string()))?;
        
        let request = config::get().ws_url.to_owned().into_client_request()?;

        let (ws_stream, _) = connect_async(request).await?;

        info!("Handshaking successfully completed");
        
        if let Err(e) = handle_websocket(ws_stream, &approval_key).await {
            error!("WebSocket error: {}. Reconnecting...", e);
            sleep(Duration::from_secs(5)).await;  // Add delay before reconnecting
        }
    }
}

mod message;
use message::{Response, TransactionId};

async fn handle_websocket(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, approval_key: &str) -> Result<(), WebSocketError> {
    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<WsMessage>(100);
    
    // Spawn a task to handle sending messages
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = write.send(message).await {
                error!("Error sending message: {}", e);
                return;
            }
        }
    });
    
    // Clone tx for use in the loop
    let tx_clone = tx.clone();
    
    subscribe_stock_price(tx_clone.clone(), approval_key).await?;
    
    // Main loop for receiving messages
    let receive_result = async {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => handle_message(msg, tx_clone.clone()).await?,
                Err(e) => {
                    error!("Error while receiving message: {}", e);
                    return Err(WebSocketError::from(e));
                }
            }
        }
        Ok(())
    }.await;

    // Clean up
    send_task.abort();
    if let Err(e) = receive_result {
        error!("WebSocket error: {}", e);
    }
    info!("WebSocket connection closed");

    Ok(())
}

async fn handle_message(message: WsMessage, tx: mpsc::Sender<WsMessage>) -> Result<(), WebSocketError> {
    match message {
        WsMessage::Text(text) => {
            if let Ok(parsed_message) = serde_json::from_str::<Response>(&text) {
                match parsed_message.header.tr_id.parse() {
                    Ok(TransactionId::PINGPONG) => {
                        tx.send(WsMessage::Pong(text.into_bytes())).await.map_err(|e| WebSocketError::SendError(e.to_string()))?;
                    },
                    Ok(tr_id) => on_received_json(tr_id, &text)?,
                    Err(_) => on_received_json(TransactionId::UnKnown, &text)?,
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

async fn subscribe_stock_price(tx: mpsc::Sender<WsMessage>, approval_key: &str) -> Result<(), WebSocketError> {
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
    tx.send(WsMessage::Text(message_payload.to_string())).await.map_err(|e| WebSocketError::SendError(e.to_string()))?;

    Ok(())
}

async fn on_received_text(message: &str) -> Result<(), WebSocketError> {
    if message.starts_with('0') {
        let recvstr: Vec<&str> = message.split('|').collect();
        let trid0 = recvstr[1];
        let data_cnt = recvstr[2].parse::<i32>().unwrap();

        match trid0 {
            "HDFSASP0"|"HDFSASP1" => {
                info!("#### 해외주식호가 ####");
                stocks_call_overseas(data_cnt, recvstr[3]);
                sleep(Duration::from_millis(500)).await;
            },
            "HDFSCNT0" => {
                info!("#### 해외주식체결 ####");
                stocks_purchase_overseas(data_cnt, recvstr[3]);
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

static GLOBAL_ARRAY: [&str; 26] = ["실시간종목코드", "종목코드", "수수점자리수", "현지영업일자", "현지일자", "현지시간", "한국일자", "한국시간", "시가", "고가", "저가", "현재가", "대비구분", "전일대비", "등락율", "매수호가", "매도호가", "매수잔량", "매도잔량", "체결량", "거래량", "거래대금", "매도체결량", "매수체결량", "체결강도", "시장구분"];

fn stocks_call_overseas(data_cnt: i32, data: &str) {
    info!("============================================");
    // let menulist = "실시간종목코드|종목코드|수수점자리수|현지영업일자|현지일자|현지시간|한국일자|한국시간|시가|고가|저가|현재가|대비구분|전일대비|등락율|매수호가|매도호가|매수잔량|매도잔량|체결량|거래량|거래대금|매도체결량|매수체결량|체결강도|시장구분";
    // let menustr: Vec<&str> = menulist.split('|').collect();
    let pvalue: Vec<&str> = data.split('^').collect();

    for cnt in 0..data_cnt {
        info!("### [{} / {}]", cnt + 1, data_cnt);
        for (menu, value) in GLOBAL_ARRAY.iter().zip(pvalue.iter().skip((cnt as usize) * GLOBAL_ARRAY.len())) {
            info!("{:<13}\t[{}]", menu, value);
        }
    }
}

fn stocks_purchase_overseas(data_cnt: i32, data: &str) {
    info!("============================================");
    // let menulist = "실시간종목코드|종목코드|수수점자리수|현지영업일자|현지일자|현지시간|한국일자|한국시간|시가|고가|저가|현재가|대비구분|전일대비|등락율|매수호가|매도호가|매수잔량|매도잔량|체결량|거래량|거래대금|매도체결량|매수체결량|체결강도|시장구분";
    // let menustr: Vec<&str> = menulist.split('|').collect();
    let pvalue: Vec<&str> = data.split('^').collect();

    for cnt in 0..data_cnt {
        info!("### [{} / {}]", cnt + 1, data_cnt);
        for (menu, value) in GLOBAL_ARRAY.iter().zip(pvalue.iter().skip((cnt as usize) * GLOBAL_ARRAY.len())) {
            info!("{:<13}\t[{}]", menu, value);
        }
    }
}

fn on_received_json(tr_id: TransactionId, message: &str) -> Result<(), WebSocketError> {
    match tr_id {
        // 실시가 체결가(해외)
        TransactionId::HDFSCNT0(s) => {
            info!("tr_id: {} \n{}", s, message);
        }
        // 실시간 호가(미국)
        TransactionId::HDFSASP0(s) => {
            info!("tr_id: {} \n{}", s, message);
        },
        // 실시간 호가(아시아)
        TransactionId::HDFSASP1(s) => {
            info!("tr_id: {} \n{}", s, message);
        },
        // 실시간 체결 통보(해외)
        TransactionId::H0GSCNI0(s) => {
            info!("tr_id: {} \n{}", s, message);
        },
        // 미지정
        _ => {
            info!("Received json: {}", message);
        }
    }
    //TODO: parse aes iv and key for future usage 

    Ok(())
}