use dotenvy::dotenv;
use kis_api::models::*;
use kis_api::{KisClient, KisEnv};
use std::env;

#[tokio::test]
#[ignore]
async fn test_client_initialization() {
    let client = KisClient::new("app_key", "app_secret", KisEnv::Vts)
        .await
        .expect("Failed to create client");
    assert_eq!(client.env(), KisEnv::Vts);
}

#[tokio::test]
#[ignore]
async fn test_domestic_quotation_interface() {
    dotenv().ok();
    let app_key = env::var("KIS_APP_KEY").unwrap_or_else(|_| "fake_key".to_string());
    let app_secret = env::var("KIS_APP_SECRET").unwrap_or_else(|_| "fake_secret".to_string());

    let client = KisClient::new(&app_key, &app_secret, KisEnv::Vts)
        .await
        .expect("Failed to create client");

    // Using absolute path unique names from generated SDK
    let req = DomesticStockV1QuotationsInquirePriceRequest {
        fid_cond_mrkt_div_code: "J".to_string(),
        fid_input_iscd: "005930".to_string(),
    };

    // This should compile if SDK generation and module mapping are correct
    let _ = client
        .stock()
        .quotations()
        .domestic_stock_v1_quotations_inquire_price(req)
        .await;
}
