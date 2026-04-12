use dotenvy::dotenv;
use kis_api::{InquirePriceRequest, KisClient, KisEnv};
use std::env;
use std::path::PathBuf;

#[tokio::test]
async fn test_token_and_quote_real() {
    dotenv().ok();

    let app_key = match env::var("VTS_APP_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("[!] Skipping test: VTS_APP_KEY not set");
            return;
        }
    };
    let app_secret = match env::var("VTS_APP_SECRET") {
        Ok(val) => val,
        Err(_) => {
            println!("[!] Skipping test: VTS_APP_SECRET not set");
            return;
        }
    };

    println!("[*] Starting Verification with VTS Key (and Caching)...");

    // TPS 제한 회피를 위한 대기
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    // 캐시 파일 사용하여 토큰 재사용
    let cache_path = PathBuf::from(".token_cache.vts.json");
    let client = KisClient::with_cache(&app_key, &app_secret, KisEnv::Vts, Some(cache_path))
        .await
        .expect("Failed to create client and fetch/load token");

    println!("[√] Token Engine: Ready");

    let req = InquirePriceRequest {
        fid_cond_mrkt_div_code: "J".to_string(),
        fid_input_iscd: "005930".to_string(),
    };

    let quote = client
        .stock()
        .quotations()
        .inquire_price(req)
        .await
        .expect("Failed to fetch quote");

    println!("[√] API Interface (InquirePrice): Success");
    println!("삼성전자 현재가 응답 데이터: {:?}", quote);
}
