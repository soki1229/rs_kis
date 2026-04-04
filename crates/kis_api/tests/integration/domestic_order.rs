//! 국내주식 주문/계좌 통합 테스트 (VTS 모의투자).
//! 실행: KIS_INTEGRATION_TEST=1 cargo test -p kis_api

use kis_api::KisDomesticApi;

#[tokio::test]
async fn test_domestic_unfilled_orders_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_unfilled_orders().await;
    assert!(result.is_ok(), "domestic_unfilled_orders() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_domestic_volume_ranking_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_volume_ranking(&kis_api::DomesticExchange::KOSPI, 5).await;
    assert!(result.is_ok(), "domestic_volume_ranking() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_domestic_holidays_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    // CTCA0903R은 YYYYMMDD 날짜를 파라미터로 받음
    let result = client.domestic_holidays("20260404").await;
    assert!(result.is_ok(), "domestic_holidays() failed: {:?}", result.err());
    println!("holidays count: {}", result.unwrap().len());
}

#[tokio::test]
async fn test_domestic_order_tr_ids_vts() {
    // 주문 TR-ID를 00xx 계열로 변경 후 잔고조회로 API 연결 검증
    // (실제 주문은 VTS에서도 체결될 수 있으므로 조회만 수행)
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_balance().await;
    assert!(result.is_ok(), "domestic_balance() failed: {:?}", result.err());
    let balance = result.unwrap();
    println!("domestic balance items: {}, purchase_amount: {}",
        balance.items.len(), balance.summary.purchase_amount);
}

#[tokio::test]
async fn test_domestic_order_history_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_order_history(kis_api::DomesticOrderHistoryRequest {
        start_date: "20260101".to_string(),
        end_date: "20261231".to_string(),
    }).await;
    assert!(result.is_ok(), "domestic_order_history() failed: {:?}", result.err());
    println!("order history count: {}", result.unwrap().len());
}

#[tokio::test]
async fn test_domestic_unfilled_new_trid_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisDomesticClient::new(config);
    let result = client.domestic_unfilled_orders().await;
    // VTS에서 90000000 msg_cd는 빈 목록으로 처리됨 — OK
    assert!(result.is_ok(), "domestic_unfilled_orders() failed: {:?}", result.err());
    println!("unfilled orders count: {}", result.unwrap().len());
}
