mod config;
mod kis_api;

use kis_api::approval_key::get_websocket_key;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // .env 파일에서 환경 변수 로드
    config::init();
    let config = config::get();

    // 웹소켓 접속키 발급
    let websocket_key = get_websocket_key(&config.app_key, &config.app_secret).await?;
    
    println!("웹소켓 접속키: {}", websocket_key);

    // TODO: 이후 웹소켓 연결 로직 구현...

    Ok(())
}