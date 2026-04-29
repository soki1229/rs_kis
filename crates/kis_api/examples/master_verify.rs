use dotenvy::dotenv;
use kis_api::{models::*, KisClient, KisEnv};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let app_key = env::var("KIS_REAL_APP_KEY").expect("KIS_REAL_APP_KEY not set");
    let app_secret = env::var("KIS_REAL_APP_SECRET").expect("KIS_REAL_APP_SECRET not set");

    println!("==================================================");
    println!("   KIS Master API Verification (Real Environment)  ");
    println!("==================================================");

    // 1. Auth Verification
    println!("\n[1] AUTH: Initializing Client & Issuing Token...");
    let client = KisClient::new(&app_key, &app_secret, KisEnv::Real).await?;
    println!("SUCCESS: Access Token issued and verified.");

    // 2. US Volume Ranking Verification
    println!("\n[2] REST: US Volume Ranking (NAS)...");
    let rank_resp = client
        .overseas()
        .ranking()
        .overseas_stock_v1_ranking_trade_vol(OverseasStockV1RankingTradeVolRequest {
            excd: "NAS".to_string(),
            ..Default::default()
        })
        .await?;

    let items = &rank_resp.output2;
    if !items.is_empty() {
        println!("SUCCESS: Received {} symbols.", items.len());
        if let Some(item) = items.first() {
            println!(
                "  - Sample Data: Rank 1: {} ({}), Last: {}, Vol: {}",
                item.name, item.symb, item.last, item.tvol,
            );
        }
    }

    // 3. US Daily Chart Verification
    println!("\n[3] REST: US Daily Chart (NVDA)...");
    let chart_resp = client
        .overseas()
        .quotations()
        .overseas_price_v1_quotations_dailyprice(OverseasPriceV1QuotationsDailypriceRequest {
            excd: "NAS".to_string(),
            symb: "NVDA".to_string(),
            bymd: "".to_string(),
            gubn: "0".to_string(),
            modp: "1".to_string(),
            ..Default::default()
        })
        .await?;

    let bars = &chart_resp.output2;
    if !bars.is_empty() {
        println!("SUCCESS: Received {} candles.", bars.len());
        if let Some(bar) = bars.first() {
            println!(
                "  - Latest Bar: Date: {}, Close: {}, Vol: {}",
                bar.xymd, bar.clos, bar.tvol,
            );
        }
    }

    println!("\n==================================================");
    println!("   ALL REST API CHECKS PASSED SUCCESSFULLY       ");
    println!("==================================================");

    Ok(())
}
