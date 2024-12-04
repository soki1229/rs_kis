use std::sync::Mutex;

use futures::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use lazy_static::lazy_static;
use log::{error, info, warn};
use thiserror::Error;
use tokio::{
    net::TcpStream,
    sync::mpsc,
    time::{sleep, Duration},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        protocol::Message as WsMessage,
        Error as WsError,
    },
    MaybeTlsStream,
    WebSocketStream,
};

use crate::config;
use self::{
    approval_key::get_websocket_key,
    message::{TransactionId, request, response},
};

mod approval_key;
mod message;

// Commented out modules
// mod crypto;
// use crypto::aes_cbc_base64_dec;

lazy_static! {
    static ref GLOBAL_TX: Mutex<Option<mpsc::Sender<WsMessage>>> = Mutex::new(None);
}

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] WsError),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Approval key error: {0}")]
    ApprovalKeyError(String),
    #[error("Error sending message: {0}")]
    SendError(String)
}

pub async fn connect_websocket() -> Result<(), WebSocketError> {    
    loop {
        // Send REST message for granting 'approval_key'
        let approval_key = get_websocket_key().await.map_err(|e| WebSocketError::ApprovalKeyError(e.to_string()))?;
        
        let (ws_stream, _) = connect_async(config::get().ws_url.to_owned().into_client_request()?).await?;
        info!("Handshaking successfully completed");
        
        if let Err(e) = handle_websocket(ws_stream, &approval_key).await {
            error!("WebSocket error: {}. Reconnecting...", e);
            sleep(Duration::from_secs(5)).await;  // Add delay before reconnecting
        }
    }
}

async fn handle_websocket(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>, approval_key: &str) -> Result<(), WebSocketError> {
    let (write, read) = ws_stream.split();
    let (tx, rx) = mpsc::channel::<WsMessage>(100);

    // Set the global sender
    {
        let mut global_tx = GLOBAL_TX.lock().unwrap();
        *global_tx = Some(tx.clone());
    }
    
    // Spawn the send task
    let send_task = tokio::spawn(handle_send_task(write, rx));
    
    // TODO: need to handle event; Handling requested subscription.
    subscribe_transaction("AAPL", true, approval_key).await?;

    // Handle receiving messages (main loop)
    let receive_result = handle_receive_task(read).await;

    // Clean up
    send_task.abort();

    if let Err(e) = receive_result {
        error!("WebSocket error: {}", e);
    }
    info!("WebSocket connection closed");

    // Clear global sender when done
    {
        let mut global_tx = GLOBAL_TX.lock().unwrap();
        *global_tx = None;
    }
    
    Ok(())
}

async fn handle_send_task(mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>, mut rx: mpsc::Receiver<WsMessage>) {
    while let Some(message) = rx.recv().await {
        let mut retry_count = 0;
        let max_retries = 5;
        let mut backoff_duration = Duration::from_millis(100);

        loop {
            match write.send(message.clone()).await {
                Ok(_) => break, // Successfully sent, exit retry loop
                Err(e) => match e {
                    WsError::Io(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                        if retry_count >= max_retries {
                            error!("Max retries reached for sending message");
                            break;
                        }
                        warn!("Send buffer full, applying backpressure (attempt {})", retry_count + 1);
                        sleep(backoff_duration).await;
                        backoff_duration *= 2; // Exponential backoff
                        retry_count += 1;
                    },
                    _ => {
                        error!("Error sending message: {}", e);
                        return; // Exit on other errors
                    }
                }
            }
        }
    }
}

async fn handle_receive_task(mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Result<(), WebSocketError> {
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                match msg {
                    WsMessage::Text(text) => {
                        if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
                            on_received_json(&text).await?;
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
            },
            Err(e) => {
                error!("Error while receiving message: {}", e);
                return Err(WebSocketError::from(e));
            }
        }
    }
    Ok(())
}

async fn on_received_text(message: &str) -> Result<(), WebSocketError> {
    // info!("on_received_text: {}", &message);

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
    let pvalue: Vec<&str> = data.split('^').collect();
    
    for cnt in 0..data_cnt {
        info!("### [{} / {}]", cnt + 1, data_cnt);
        for (menu, value) in GLOBAL_ARRAY.iter().zip(pvalue.iter().skip((cnt as usize) * GLOBAL_ARRAY.len())) {
            info!("{:<13}\t[{}]", menu, value);
        }
    }
}

async fn on_received_json(message: &str) -> Result<(), WebSocketError> {
    let response = serde_json::from_str::<response::Message>(&message)?;
    match response.header.tr_id.parse() {
        Ok(TransactionId::PINGPONG) => {
            let tx = {
                let global_tx = GLOBAL_TX.lock().unwrap();
                global_tx.as_ref().cloned().ok_or(WebSocketError::SendError("SenderNotInitialized".to_string()))?
            };
        
            tx.send(WsMessage::Pong(message.into())).await.map_err(|e| WebSocketError::SendError(e.to_string()))?;
            info!("PINGPONG!");
        },
        // // 실시가 체결가(해외)
        // Ok(TransactionId::HDFSCNT0(s)) => {
        // }
        // // 실시간 호가(미국)
        // Ok(TransactionId::HDFSASP0(s)) => {
        // },
        // // 실시간 호가(아시아)
        // Ok(TransactionId::HDFSASP1(s)) => {
        // },
        // // 실시간 체결 통보(해외)
        // Ok(TransactionId::H0GSCNI0(s)) => {
        // },
        _ => {
            // TODO: parse aes iv and key for future usage
            info!("===============================");
            info!("[header]");
            info!("  - tr_id: {}", response.header.tr_id);
            info!("  - tr_key: {}", response.header.tr_key);
            info!("  - encrypt: {}", response.header.encrypt);
            info!("  - datetime: {}", response.header.datetime);
            info!("[body]");
            info!("  - rt_cd: {}", response.body.rt_cd);
            info!("  - msg_cd: {}", response.body.msg_cd);
            info!("  - msg1: {}", response.body.msg1);
            info!("    [output]");
            info!("     - iv : {}", response.body.output.iv);
            info!("     - key : {}", response.body.output.key);
            info!("===============================");
        }
    }

    Ok(())
}

async fn subscribe_transaction(symbol: &str, subscribe: bool, approval_key: &str) -> Result<(), WebSocketError> {
    // Example for subscribing real-time stock price of 'Apple: DNASAAPL'
    // Create your message payload
    let request = request::Message {
        header: request::Header {
            approval_key:   String::from(approval_key),
            custtype:       String::from("P"),
            tr_type:        String::from(if subscribe {"1"} else {"2"}),
            content_type:   String::from("utf-8"),
        },
        body: request::Body {
            input: request::Input {
                tr_id:      String::from("HDFSCNT0"),
                tr_key:     String::from("DNAS") + symbol,
            },
        },
    };
    
    let tx = {
        let global_tx = GLOBAL_TX.lock().unwrap();
        global_tx.as_ref().cloned().ok_or(WebSocketError::SendError("SenderNotInitialized".to_string()))?
    };

    tx.send(WsMessage::Text(serde_json::to_string_pretty(&request).unwrap())).await.map_err(|e| WebSocketError::SendError(e.to_string()))?;

    // Send Subscribe request: *Should be sent if the session wasn't created yet.

    Ok(())
}
