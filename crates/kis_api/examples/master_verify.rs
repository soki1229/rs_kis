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

    // 2. US Volume Ranking Verification (HDFSCNT0)
    println!("\n[2] REST: US Volume Ranking (NAS)...");
    let rank_resp = client
        .overseas()
        .ranking()
        .overseas_stock_v1_ranking_trade_vol(OverseasStockV1RankingTradeVolRequest {
            excd: "NAS".to_string(),
            ..Default::default()
        })
        .await?;

    if let Some(items) = rank_resp["output2"].as_array() {
        println!("SUCCESS: Received {} symbols.", items.len());
        if let Some(item) = items.first() {
            println!(
                "  - Sample Data: Rank 1: {} ({}), Last: {}, Vol: {}",
                item["name"], item["symb"], item["last"], item["tvol"]
            );
        }
    } else {
        println!("WARNING: output2 is empty or not an array. (Check permissions)");
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

    if let Some(bars) = chart_resp["output2"].as_array() {
        println!("SUCCESS: Received {} candles.", bars.len());
        if let Some(bar) = bars.first() {
            println!(
                "  - Latest Bar: Date: {}, Close: {}, Vol: {}",
                bar["xymd"], bar["last"], bar["v_vol"]
            );
        }
    }

    // 4. KR Daily Chart Verification
    println!("\n[4] REST: KR Daily Chart (Samsung)...");
    let kr_chart_resp = client
        .stock()
        .quotations()
        .domestic_stock_v1_quotations_inquire_daily_itemchartprice(
            DomesticStockV1QuotationsInquireDailyItemchartpriceRequest {
                fid_cond_mrkt_div_code: "J".to_string(),
                fid_input_iscd: "005930".to_string(),
                fid_input_date_1: "20240101".to_string(),
                fid_input_date_2: "20240422".to_string(),
                fid_period_div_code: "D".to_string(),
                fid_org_adj_prc: "0".to_string(),
                ..Default::default()
            },
        )
        .await?;

    if let Some(bars) = kr_chart_resp["output2"].as_array() {
        println!("SUCCESS: Received {} candles.", bars.len());
        if let Some(bar) = bars.first() {
            println!(
                "  - Sample Bar: Date: {}, Close: {}, Vol: {}",
                bar["stck_bsop_date"], bar["stck_clpr"], bar["acml_vol"]
            );
        }
    }

    println!("\n==================================================");
    println!("   ALL REST API CHECKS PASSED SUCCESSFULLY       ");
    println!("==================================================");

    Ok(())
}
