use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use kis_api::{KisClient, KisEnv};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let app_key = env::var("KIS_REAL_APP_KEY").expect("KIS_REAL_APP_KEY not set");
    let app_secret = env::var("KIS_REAL_APP_SECRET").expect("KIS_REAL_APP_SECRET not set");

    println!("--- Initializing Real Client for WebSocket ---");
    let client = KisClient::new(&app_key, &app_secret, KisEnv::Real).await?;
    let approval_key = client.approval_key().await?;
    println!("Approval Key: {}", approval_key);

    let ws_url = "ws://ops.koreainvestment.com:21000";
    println!("Connecting to: {}", ws_url);

    let (mut ws_stream, _) = connect_async(ws_url).await?;
    println!("Connected.");

    // Subscribe to NVDA with CORRECT Real Key: DNAS + NVDA
    let sub_msg = serde_json::json!({
        "header": {
            "approval_key": approval_key,
            "custtype": "P",
            "tr_type": "1",
            "content-type": "utf-8"
        },
        "body": {
            "input": {
                "tr_id": "HDFSCNT0",
                "tr_key": "DNASNVDA"
            }
        }
    });

    ws_stream
        .send(Message::Text(sub_msg.to_string().into()))
        .await?;
    println!("Subscribed to NVDA (DNASNVDA)");

    println!("Waiting for ticks (10 seconds)...");
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                if let Some(Ok(Message::Text(text))) = msg {
                    println!("\n[RAW WS DATA RECEIVED] {}", text);
                    if text.contains('|') {
                        println!("SUCCESS: Real-time data is arriving!");
                        return Ok(());
                    }
                }
            }
            _ = &mut timeout => {
                println!("TIMEOUT: No data received in 10 seconds. (Market might be quiet or keys issues)");
                break;
            }
        }
    }

    Ok(())
}
