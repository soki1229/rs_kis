mod config;
mod websockets;
use websockets::connect_websocket;
use std::error::Error;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init_logging();

    println!("========================================================= RUSTy KIS =========================================================");
    
    // .env 파일에서 환경 변수 로드
    config::init();
    
    // 웹소켓 연결
    tokio::spawn(async move {
        if let Err(e) = connect_websocket().await {
            eprintln!("WebSocket error: {}", e);
        }
    });

    // Keep the main function running to see the logs
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    println!("\nReceived Ctrl+C, shutting down gracefully.");    
    println!("=============================================================================================================================");

    Ok(())
}