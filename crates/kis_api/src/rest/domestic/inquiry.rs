// ⚠ TR ID 및 필드명은 KIS OpenAPI 가이드에서 확인 필요
// - 미체결: GET /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl (tr_id: TTTC8036R)
// - 일봉: GET /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice (tr_id: FHKST03010100)
// - 거래량: GET /uapi/domestic-stock/v1/ranking/volume (tr_id: FHPST01710000)

use super::types::*;
use crate::rest::overseas::types::split_account;
use crate::{CandleBar, ChartPeriod, KisConfig, KisError};
use reqwest::Client;
use rust_decimal::Decimal;
use std::str::FromStr;

pub async fn domestic_unfilled_orders(
    client: &Client,
    config: &KisConfig,
    token: &str,
) -> Result<Vec<DomesticUnfilledOrder>, KisError> {
    let (cano, prdt_cd) = split_account(&config.account_num);

    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", "TTTC8036R")
        .header("custtype", "P")
        .query(&[
            ("CANO", cano),
            ("ACNT_PRDT_CD", prdt_cd),
            ("CTX_AREA_FK100", ""),
            ("CTX_AREA_NK100", ""),
            ("INQR_DVSN_1", "0"),
            ("INQR_DVSN_2", "0"),
        ])
        .send()
        .await
        .map_err(KisError::Network)?;

    let json: serde_json::Value = resp.json().await.map_err(KisError::Network)?;
    let rt_cd = json["rt_cd"].as_str().unwrap_or("");
    if rt_cd != "0" {
        return Err(KisError::Api {
            code: rt_cd.into(),
            message: json["msg1"].as_str().unwrap_or("unknown").into(),
        });
    }

    let orders = json["output"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|o| {
                    let qty: u32 = o["ord_qty"].as_str()?.parse().ok()?;
                    let remaining: u32 = o["psbl_qty"].as_str()?.parse().ok()?;
                    let filled: u32 = qty.saturating_sub(remaining);
                    Some(DomesticUnfilledOrder {
                        order_no: o["odno"].as_str()?.into(),
                        symbol: o["pdno"].as_str()?.into(),
                        exchange: o["ord_excg_dvsn_cd"].as_str().unwrap_or("KSC").into(),
                        side_cd: o["sll_buy_dvsn_cd"].as_str()?.into(),
                        qty,
                        price: Decimal::from_str(o["ord_unpr"].as_str()?).ok()?,
                        filled_qty: filled,
                        remaining_qty: remaining,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(orders)
}

pub async fn domestic_daily_chart(
    client: &Client,
    config: &KisConfig,
    token: &str,
    req: DomesticDailyChartRequest,
) -> Result<Vec<CandleBar>, KisError> {
    let period_code = match req.period {
        ChartPeriod::Daily => "D",
        ChartPeriod::Weekly => "W",
        ChartPeriod::Monthly => "M",
    };
    let adj = if req.adj_price { "1" } else { "0" };

    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", "FHKST03010100")
        .query(&[
            ("FID_COND_MRKT_DIV_CODE", "J"),
            ("FID_INPUT_ISCD", req.symbol.as_str()),
            ("FID_INPUT_DATE_1", ""),
            ("FID_INPUT_DATE_2", ""),
            ("FID_PERIOD_DIV_CODE", period_code),
            ("FID_ORG_ADJ_PRC", adj),
        ])
        .send()
        .await
        .map_err(KisError::Network)?;

    let json: serde_json::Value = resp.json().await.map_err(KisError::Network)?;
    let rt_cd = json["rt_cd"].as_str().unwrap_or("");
    if rt_cd != "0" {
        return Err(KisError::Api {
            code: rt_cd.into(),
            message: json["msg1"].as_str().unwrap_or("unknown").into(),
        });
    }

    let bars = json["output2"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|b| {
                    Some(CandleBar {
                        date: b["stck_bsop_date"].as_str()?.into(),
                        open: Decimal::from_str(b["stck_oprc"].as_str()?).ok()?,
                        high: Decimal::from_str(b["stck_hgpr"].as_str()?).ok()?,
                        low: Decimal::from_str(b["stck_lwpr"].as_str()?).ok()?,
                        close: Decimal::from_str(b["stck_clpr"].as_str()?).ok()?,
                        volume: Decimal::from_str(b["acml_vol"].as_str()?).ok()?,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(bars)
}

pub async fn domestic_volume_ranking(
    client: &Client,
    config: &KisConfig,
    token: &str,
    exchange: &DomesticExchange,
    count: u32,
) -> Result<Vec<DomesticRankingItem>, KisError> {
    let mkt_div = match exchange {
        DomesticExchange::KOSPI => "J",
        DomesticExchange::KOSDAQ => "Q",
    };
    let count_str = count.to_string();

    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/ranking/volume",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", "FHPST01710000")
        .query(&[
            ("FID_COND_MRKT_DIV_CODE", mkt_div),
            ("FID_COND_SCR_DIV_CODE", "20171"),
            ("FID_INPUT_ISCD", "0000"),
            ("FID_DIV_CLS_CODE", "0"),
            ("FID_BLNG_CLS_CODE", "0"),
            ("FID_TRGT_CLS_CODE", "111111111"),
            ("FID_TRGT_EXLS_CLS_CODE", "000000000"),
            ("FID_INPUT_PRICE_1", ""),
            ("FID_INPUT_PRICE_2", ""),
            ("FID_VOL_CNT", count_str.as_str()),
            ("FID_INPUT_DATE_1", ""),
        ])
        .send()
        .await
        .map_err(KisError::Network)?;

    let json: serde_json::Value = resp.json().await.map_err(KisError::Network)?;
    let rt_cd = json["rt_cd"].as_str().unwrap_or("");
    if rt_cd != "0" {
        return Err(KisError::Api {
            code: rt_cd.into(),
            message: json["msg1"].as_str().unwrap_or("unknown").into(),
        });
    }

    let items = json["output"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|i| {
                    Some(DomesticRankingItem {
                        symbol: i["mksc_shrn_iscd"].as_str()?.into(),
                        name: i["hts_kor_isnm"].as_str().unwrap_or("").into(),
                        exchange: mkt_div.into(),
                        price: Decimal::from_str(i["stck_prpr"].as_str()?).ok()?,
                        volume: Decimal::from_str(i["acml_vol"].as_str()?).ok()?,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(items)
}
