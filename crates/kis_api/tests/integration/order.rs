//! 해외주식 주문/계좌 통합 테스트 (VTS 모의투자).
//! 실행: KIS_INTEGRATION_TEST=1 cargo test -p kis_api

#[tokio::test]
async fn test_balance_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisClient::new(config);
    let result = client.balance().await;
    assert!(result.is_ok(), "balance() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_unfilled_orders_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let client = kis_api::KisClient::new(config);
    let result = client.unfilled_orders().await;
    assert!(result.is_ok(), "unfilled_orders() failed: {:?}", result.err());
}

#[tokio::test]
async fn test_check_deposit_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let kis = kis_api::KisClient::new(config.clone());
    let token = kis.token_manager().token().await.expect("token fetch failed");
    let result = kis_api::rest::overseas::deposit::check_deposit(kis.shared_http(), &config, &token).await;
    assert!(result.is_ok(), "check_deposit() failed: {:?}", result.err());
    println!("USD deposit: {:?}", result.unwrap().amount);
}

#[tokio::test]
async fn test_symbol_info_vts() {
    if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" { return; }
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts().expect("VTS credentials not set");
    let kis = kis_api::KisClient::new(config.clone());
    let token = kis.token_manager().token().await.expect("token fetch failed");
    let result = kis_api::rest::overseas::quote::search::symbol_info(
        kis.shared_http(), &config, &token, "AAPL", &kis_api::Exchange::NASD,
    ).await;
    assert!(result.is_ok(), "symbol_info() failed: {:?}", result.err());
    let info = result.unwrap();
    assert_eq!(info.symbol, "AAPL");
    println!("symbol_info: name={}, per={}", info.name, info.per);
}
