use std::sync::Mutex;

use futures::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use tokio::{
    net::TcpStream,
    sync::mpsc,
    time::{sleep, Duration},
};

// use tokio_tungstenite::tungstenite::Message;

use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        client::IntoClientRequest,
        protocol::{frame::coding::CloseCode, CloseFrame},
        Error as WsError, Message,
    },
    MaybeTlsStream, WebSocketStream,
};

use crate::{client::KisClient, environment, error::KisClientError as Error};

mod message;
use self::message::{request, response, TransactionId};

use crate::extentions::analyzer;

// Commented out modules
// mod crypto;
// use crypto::aes_cbc_base64_dec;

// struct KisSocket {
//     websocket_handle: Option<tokio::task::JoinHandle<()>>,
//     stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
// }
use std::sync::Arc;

type Callback = Arc<dyn Fn(Message, mpsc::Sender<Message>) + Send + Sync + 'static>;

pub struct KisSocket {
    tx: mpsc::Sender<Message>,
    callback: Callback,
}

impl KisSocket {
    pub fn new(
        url: String,
        callback: impl Fn(Message, mpsc::Sender<Message>) + Send + Sync + 'static,
    ) -> Self {
        let (tx, rx) = mpsc::channel(100);
        let callback = Arc::new(callback);
        let kis_socket = Self {
            tx: tx.clone(),
            callback: callback.clone(),
        };

        tokio::spawn(Self::run_websocket(rx, url, tx.clone(), callback));

        kis_socket
    }

    pub async fn disconnect(&self) {
        match self.tx.send(Message::Close(None)).await {
            Ok(_) => {
                info!("requested disconnection.")
            }
            Err(_) => {
                info!("Failed to send.")
            }
        };
    }

    async fn run_websocket(
        mut rx: mpsc::Receiver<Message>,
        url: String,
        tx: mpsc::Sender<Message>,
        callback: Callback,
    ) {
        loop {
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    info!("WebSocket connection established.");
                    if let Err(e) = Self::handle_websocket(ws_stream, &mut rx, &tx, &callback).await
                    {
                        warn!("WebSocket error: {}. Reconnecting...", e);
                    }
                }
                Err(e) => error!("Failed to connect: {}. Retrying...", e),
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    async fn handle_websocket(
        ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
        rx: &mut mpsc::Receiver<Message>,
        tx: &mpsc::Sender<Message>,
        callback: &Callback,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (mut write, mut read) = ws_stream.split();

        loop {
            tokio::select! {
                Some(msg) = rx.recv() => {
                    if let Err(e) = write.send(msg).await {
                        error!("Error sending message: {}", e);
                        break;
                    }
                },
                Some(result) = read.next() => match result {
                    Ok(msg) => (callback)(msg, tx.clone()),
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                        break;
                    }
                },
                else => {
                    error!("WebSocket connection closed unexpectedly");
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn send_message(
        &self,
        message: Message,
    ) -> Result<(), mpsc::error::SendError<Message>> {
        self.tx.send(message).await
    }
}
// impl KisSocket {
//     pub fn new() -> Self {
//         Self {
//             websocket_handle: Some(Self::create_connection()),

//         }
//     }

//     fn create_connection() -> tokio::task::JoinHandle<()> {
//         tokio::task::spawn(async move {
//             loop {
//                 match tokio_tungstenite::connect_async(
//                     "ws://ops.koreainvestment.com:21000"
//                         .to_owned()
//                         .into_client_request()
//                         .unwrap(),
//                 )
//                 .await
//                 {
//                     Ok((ws_stream, _)) => {
//                         info!("WebSocket connection established.");

//                         if let Err(e) = handle_websocket(ws_stream).await {
//                             error!("WebSocket error: {}. Reconnecting...", e);
//                         }
//                     }
//                     Err(e) => {
//                         error!("Failed to connect to WebSocket: {}. Retrying...", e);
//                     }
//                 }

//                 tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//             }
//         })
//     }
// }

lazy_static! {
    static ref GLOBAL_TX: Mutex<Option<mpsc::Sender<Message>>> = Mutex::new(None);
}

// pub async fn handle_websocket(
//     ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
// ) -> Result<(), Error> {
//     let (write, read) = ws_stream.split();
//     let (tx, rx) = mpsc::channel::<WebSocketMessage>(100);

//     // Set the global sender
//     {
//         let mut global_tx = GLOBAL_TX.lock().unwrap();
//         *global_tx = Some(tx.clone());
//     }

//     // Spawn the send task
//     let send_task = tokio::spawn(handle_send_task(write, rx));

//     // Handle receiving messages (main loop)
//     let receive_result: Result<(), Error> = handle_receive_task(read).await;

//     // Clean up
//     send_task.abort();

//     if let Err(e) = receive_result {
//         error!("WebSocket error: {}", e);
//     }
//     info!("WebSocket connection closed");

//     // Clear global sender when done
//     {
//         let mut global_tx: std::sync::MutexGuard<'_, Option<mpsc::Sender<WebSocketMessage>>> =
//             GLOBAL_TX.lock().unwrap();
//         *global_tx = None;
//     }

//     Ok(())
// }

async fn handle_send_task(
    mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut rx: mpsc::Receiver<Message>,
) {
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
                        warn!(
                            "Send buffer full, applying backpressure (attempt {})",
                            retry_count + 1
                        );
                        sleep(backoff_duration).await;
                        backoff_duration *= 2; // Exponential backoff
                        retry_count += 1;
                    }
                    _ => {
                        error!("Error sending message: {}", e);
                        return; // Exit on other errors
                    }
                },
            }
        }
    }
}

// async fn handle_receive_task(
//     mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
// ) -> Result<(), Error> {
//     let mut analyzer = analyzer::StockAnalyzer::new(50);

//     while let Some(msg) = read.next().await {
//         match msg {
//             Ok(msg) => match msg {
//                 WebSocketMessage::Text(text) => {
//                     if serde_json::from_str::<serde_json::Value>(&text).is_ok() {
//                         on_received_json(&text).await?;
//                     } else {
//                         on_received_text(&text, &mut analyzer).await?;
//                     }
//                 }
//                 WebSocketMessage::Binary(bin) => {
//                     info!("Received binary message: {:?}", bin);
//                 }
//                 WebSocketMessage::Ping(payload) => {
//                     info!("Received ping with payload: {:?}", payload);
//                 }
//                 WebSocketMessage::Pong(payload) => {
//                     info!("Received pong with payload: {:?}", payload);
//                 }
//                 WebSocketMessage::Frame(frame) => {
//                     info!("Received frame: {:?}", frame);
//                 }
//                 WebSocketMessage::Close(_) => {
//                     info!("Connection closed by server.");
//                 }
//             },
//             Err(e) => {
//                 error!("Error while receiving message: {}", e);
//                 return Err(Error::from(e));
//             }
//         }
//     }
//     Ok(())
// }

async fn on_received_text(
    message: &str,
    analyzer: &mut analyzer::StockAnalyzer,
) -> Result<(), Error> {
    // info!("on_received_text: {}", &message);

    if message.starts_with('0') {
        let recvstr: Vec<&str> = message.split('|').collect();
        let trid0 = recvstr[1];
        let data_cnt = recvstr[2].parse::<i32>().unwrap();

        match trid0 {
            "HDFSASP0" | "HDFSASP1" => {
                info!("#### 해외주식호가 ####");
                stocks_call_overseas(data_cnt, recvstr[3]);
                sleep(Duration::from_millis(500)).await;
            }
            "HDFSCNT0" => {
                info!("#### 해외주식체결 ####");
                stocks_purchase_overseas(data_cnt, recvstr[3], analyzer);
                sleep(Duration::from_millis(500)).await;
            }
            _ => {}
        }
    } else if message.starts_with('1') {
        let recvstr: Vec<&str> = message.split('|').collect();
        let trid0 = recvstr[1];

        match trid0 {
            "H0STCNI0" | "H0STCNI9" => {
                //stocksigningnotice_domestic(recvstr[3], aes_key, aes_iv);
            }
            "H0GSCNI0" | "H0GSCNI9" => {
                //stocksigningnotice_overseas(recvstr[3], aes_key, aes_iv);
            }
            _ => {}
        }
    }

    Ok(())
}

static GLOBAL_ARRAY: [&str; 26] = [
    "실시간종목코드",
    "종목코드",
    "수수점자리수",
    "현지영업일자",
    "현지일자",
    "현지시간",
    "한국일자",
    "한국시간",
    "시가",
    "고가",
    "저가",
    "현재가",
    "대비구분",
    "전일대비",
    "등락율",
    "매수호가",
    "매도호가",
    "매수잔량",
    "매도잔량",
    "체결량",
    "거래량",
    "거래대금",
    "매도체결량",
    "매수체결량",
    "체결강도",
    "시장구분",
];

fn stocks_call_overseas(data_cnt: i32, data: &str) {
    info!("===================================================================");
    let pvalue: Vec<&str> = data.split('^').collect();

    // ifo!("{} : {}({})", data[1], data_cnt[11], data_cnt[14]);
    for cnt in 0..data_cnt {
        info!("### [{} / {}]", cnt + 1, data_cnt);
        for (menu, value) in GLOBAL_ARRAY
            .iter()
            .zip(pvalue.iter().skip((cnt as usize) * GLOBAL_ARRAY.len()))
        {
            info!("{:<13}\t[{}]", menu, value);
        }
    }
}

fn stocks_purchase_overseas(data_cnt: i32, data: &str, analyzer: &mut analyzer::StockAnalyzer) {
    info!("============================================");
    let pvalue: Vec<&str> = data.split('^').collect();

    let stock_data = analyzer::StockData {
        종목코드: pvalue[1].to_string(),
        현재가: pvalue[11].parse().unwrap(),
        매수잔량: pvalue[17].parse().unwrap(),
        매도잔량: pvalue[18].parse().unwrap(),
        거래량: pvalue[20].parse().unwrap(),
        매수체결량: pvalue[23].parse().unwrap(),
        매도체결량: pvalue[22].parse().unwrap(),
    };

    analyzer.add_data(stock_data);

    info!("{}", analyzer.analyze());

    // for cnt in 0..data_cnt {
    //     info!("### [{} / {}]", cnt + 1, data_cnt);
    //     for (menu, value) in GLOBAL_ARRAY.iter().zip(pvalue.iter().skip((cnt as usize) * GLOBAL_ARRAY.len())) {
    //         info!("{:<13}\t[{}]", menu, value);
    //     }
    // }
}

async fn on_received_json(message: &str) -> Result<(), Error> {
    let response = serde_json::from_str::<response::Message>(&message)?;
    match response.header.tr_id.parse() {
        Ok(TransactionId::PINGPONG) => {
            let tx: mpsc::Sender<Message> = {
                let global_tx = GLOBAL_TX.lock().unwrap();
                global_tx
                    .as_ref()
                    .cloned()
                    .ok_or(Error::SendError("SenderNotInitialized".to_string()))?
            };

            tx.send(Message::Pong(message.into()))
                .await
                .map_err(|e| Error::SendError(e.to_string()))?;
            debug!("ping-pong");
        }
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
            info!("===================================================");
            info!("<header>");
            info!(" tr_id: {}", response.header.tr_id);
            info!(" tr_key: {}", response.header.tr_key);
            info!(" encrypt: {}", response.header.encrypt);
            info!(" datetime: {}", response.header.datetime);
            info!("<body>");
            info!(" rt_cd: {}", response.body.rt_cd);
            info!(" msg_cd: {}", response.body.msg_cd);
            info!(" msg1: {}", response.body.msg1);
            info!("  <output>");
            info!("   iv : {}", response.body.output.iv);
            info!("   key : {}", response.body.output.key);
            info!("===================================================");
        }
    }

    Ok(())
}

pub async fn subscribe_transaction(
    symbol: &str,
    subscribe: bool,
    approval_key: &str,
) -> Result<(), Error> {
    // Example for subscribing real-time stock price of 'Apple: DNASAAPL'
    // Create your message payload
    let request = request::Message {
        header: request::Header {
            approval_key: String::from(approval_key),
            custtype: String::from("P"),
            tr_type: String::from(if subscribe { "1" } else { "2" }),
            content_type: String::from("utf-8"),
        },
        body: request::Body {
            input: request::Input {
                tr_id: String::from("HDFSCNT0"),
                tr_key: String::from("DNAS") + symbol,
            },
        },
    };

    let tx = {
        let global_tx = GLOBAL_TX.lock().unwrap();
        global_tx
            .as_ref()
            .cloned()
            .ok_or(Error::SendError("SenderNotInitialized".to_string()))?
    };

    tx.send(Message::Text(
        serde_json::to_string_pretty(&request).unwrap(),
    ))
    .await
    .map_err(|e| Error::SendError(e.to_string()))?;

    // Send Subscribe request: *Should be sent if the session wasn't created yet.

    Ok(())
}
