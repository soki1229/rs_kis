//! 국내주식 주문/계좌 통합 테스트 (VTS 모의투자).
//! 실행: KIS_INTEGRATION_TEST=1 cargo test -p kis_api

use kis_api::KisDomesticApi;

#[tokio::test]
async fn test_domestic_unfilled_orders_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_unfilled_orders().await;
    assert!(result.is_ok(), "domestic_unfilled_orders() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_domestic_volume_ranking_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_volume_ranking(&kis_api::DomesticExchange::KOSPI, 5).await;
    assert!(result.is_ok(), "domestic_volume_ranking() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_domestic_holidays_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_holidays("KOR").await;
    assert!(result.is_ok(), "domestic_holidays() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_domestic_order_tr_ids_vts() {
    // 주문 TR-ID를 00xx 계열로 변경 후 잔고조회로 API 연결 검증
    // (실제 주문은 VTS에서도 체결될 수 있으므로 조회만 수행)
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_balance().await;
    assert!(result.is_ok(), "domestic_balance() failed: {:?}", result.err());
    let balance = result.unwrap();
    println!("domestic balance items: {}, purchase_amount: {}",
        balance.items.len(), balance.summary.purchase_amount);
}
