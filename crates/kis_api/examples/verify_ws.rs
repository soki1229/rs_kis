use dotenvy::dotenv;
use futures_util::StreamExt;
use kis_api::{KisClient, KisEnv, SubscriptionKind};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let app_key = env::var("KIS_REAL_APP_KEY").expect("KIS_REAL_APP_KEY not set");
    let app_secret = env::var("KIS_REAL_APP_SECRET").expect("KIS_REAL_APP_SECRET not set");

    println!("--- Initializing Real Client for WebSocket ---");
    let client = KisClient::new(&app_key, &app_secret, KisEnv::Real).await?;
    let approval_key = client.approval_key().await?;
    println!("Approval Key: {}", approval_key);

    let ws_url = client.ws_url();
    println!("Connecting to: {}", ws_url);

    let mut ws_stream = tokio_tungstenite::connect_async(ws_url).await?.0;

    // Subscribe to NVDA (US Price)
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
                "tr_key": "NVDA"
            }
        }
    });

    ws_stream
        .send(tokio_tungstenite::tungstenite::Message::Text(
            sub_msg.to_string(),
        ))
        .await?;
    println!("Subscribed to NVDA (HDFSCNT0)");

    let mut count = 0;
    while let Some(Ok(msg)) = ws_stream.next().await {
        if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
            println!("\n[WS RECEIVED] {}", text);
            if text.starts_with('0') || text.starts_with('1') {
                let parts: Vec<&str> = text.split('|').collect();
                if parts.len() > 3 {
                    let data = parts[3];
                    let fields: Vec<&str> = data.split('^').collect();
                    println!("  Parsed Fields Count: {}", fields.len());
                    for (i, f) in fields.iter().enumerate() {
                        println!("    [{}] {}", i, f);
                    }
                }
                count += 1;
            }
        }
        if count >= 3 {
            break;
        }
    }

    Ok(())
}
