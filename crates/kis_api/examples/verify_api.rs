use dotenvy::dotenv;
use kis_api::{models::*, KisClient, KisEnv};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let app_key = env::var("KIS_REAL_APP_KEY").expect("KIS_REAL_APP_KEY not set");
    let app_secret = env::var("KIS_REAL_APP_SECRET").expect("KIS_REAL_APP_SECRET not set");

    println!("--- Initializing Real Client ---");
    let client = KisClient::new(&app_key, &app_secret, KisEnv::Real).await?;

    // 1. Test US Volume Ranking (NAS) - 실제 종목 리스트 출력
    println!("\n--- Testing US Volume Ranking (NAS) ---");
    let rank_resp = client
        .overseas()
        .ranking()
        .overseas_stock_v1_ranking_trade_vol(OverseasStockV1RankingTradeVolRequest {
            excd: "NAS".to_string(),
            ..Default::default()
        })
        .await?;

    let output = &rank_resp["output"];
    if let Some(items) = output.as_array() {
        println!("SUCCESS: Received {} symbols", items.len());
        for (i, item) in items.iter().take(5).enumerate() {
            println!(
                "  [Rank {}] Code: {:?}, Name: {:?}, Vol: {:?}",
                i + 1,
                item["symb"],
                item["hts_kor_isnm"],
                item["accum_vol"]
            );
        }
    } else {
        println!(
            "FAILED: Output is not an array. Full response: {}",
            rank_resp
        );
    }

    // 2. Test US Daily Chart (NVDA) - 실제 캔들 데이터 출력
    println!("\n--- Testing US Daily Chart (NVDA) ---");
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

    let bars = &chart_resp["output2"];
    if let Some(items) = bars.as_array() {
        println!("SUCCESS: Received {} bars", items.len());
        if let Some(first) = items.first() {
            println!(
                "  [Latest Bar] Date: {:?}, Close: {:?}, Vol: {:?}",
                first["xymd"], first["close"], first["v_vol"]
            );
        }
    } else {
        println!(
            "FAILED: Output2 is not an array. Full response: {}",
            chart_resp
        );
    }

    Ok(())
}
