// - 미체결: GET /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl (tr_id: TTTC0084R)
// - 일봉: GET /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice (tr_id: FHKST03010100)
// - 거래량: GET /uapi/domestic-stock/v1/quotations/volume-rank (tr_id: FHPST01710000)

use super::types::*;
use crate::rest::overseas::inquiry::balance::{BalanceItem, BalanceResponse, BalanceSummary};
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
    let tr_id = if config.is_domestic_virtual {
        "VTTC0084R"
    } else {
        "TTTC0084R"
    };
    let (cano, prdt_cd) = split_account(&config.account_num);

    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", tr_id)
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
        // VTS는 미체결조회를 지원하지 않음 — 빈 목록 반환
        // msg_cd 90000000: 구버전 VTS 미지원 응답
        // "없는 서비스 코드": 신규 TR-ID(VTTC0084R)에 대한 VTS 미지원 응답
        let msg_cd = json["msg_cd"].as_str().unwrap_or("");
        let msg1 = json["msg1"].as_str().unwrap_or("");
        if config.is_domestic_virtual || msg_cd == "90000000" || msg1.contains("없는 서비스 코드")
        {
            return Ok(vec![]);
        }
        return Err(KisError::Api {
            code: rt_cd.into(),
            message: msg1.into(),
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

pub async fn domestic_balance(
    client: &Client,
    config: &KisConfig,
    token: &str,
) -> Result<BalanceResponse, KisError> {
    let tr_id = if config.mock {
        "VTTC8434R"
    } else {
        "TTTC8434R"
    };
    let (cano, prdt_cd) = split_account(&config.account_num);

    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/trading/inquire-balance",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", tr_id)
        .header("custtype", "P")
        .query(&[
            ("CANO", cano),
            ("ACNT_PRDT_CD", prdt_cd),
            ("AFHR_FLPR_YN", "N"),
            ("OFL_YN", ""),
            ("INQR_DVSN", "02"),
            ("UNPR_DVSN", "01"),
            ("FUND_STTL_ICLD_YN", "N"),
            ("FINY_PFTRT_ICLD_YN", "N"),
            ("PRCS_DVSN", "00"),
            ("FNCG_AMT_AUTO_RDPT_YN", "N"),
            ("CTX_AREA_FK100", ""),
            ("CTX_AREA_NK100", ""),
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

    let items = json["output1"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|i| {
            Some(BalanceItem {
                symbol: i["pdno"].as_str()?.into(),
                name: i["prdt_name"].as_str().unwrap_or("").into(),
                exchange: "KSC".into(),
                qty: Decimal::from_str(i["hldg_qty"].as_str()?).ok()?,
                avg_price: Decimal::from_str(i["pchs_avg_pric"].as_str()?).ok()?,
                eval_amount: Decimal::from_str(i["evlu_amt"].as_str().unwrap_or("0"))
                    .unwrap_or(Decimal::ZERO),
                unrealized_pnl: Decimal::from_str(i["evlu_pfls_amt"].as_str().unwrap_or("0"))
                    .unwrap_or(Decimal::ZERO),
                pnl_rate: Decimal::from_str(i["evlu_pfls_rt"].as_str().unwrap_or("0"))
                    .unwrap_or(Decimal::ZERO),
            })
        })
        .collect();

    // output2 is an array with one element for domestic balance
    let out2 = json["output2"]
        .as_array()
        .and_then(|a| a.first())
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let summary = BalanceSummary {
        // dnca_tot_amt: 예수금 총금액 (주문가능금액 근사)
        purchase_amount: Decimal::from_str(out2["dnca_tot_amt"].as_str().unwrap_or("0"))
            .unwrap_or(Decimal::ZERO),
        realized_pnl: Decimal::from_str(out2["rlzt_pfls"].as_str().unwrap_or("0"))
            .unwrap_or(Decimal::ZERO),
        total_pnl: Decimal::from_str(out2["tot_evlu_pfls_amt"].as_str().unwrap_or("0"))
            .unwrap_or(Decimal::ZERO),
    };

    Ok(BalanceResponse { items, summary })
}

pub async fn domestic_order_history(
    client: &Client,
    config: &KisConfig,
    token: &str,
    req: DomesticOrderHistoryRequest,
) -> Result<Vec<DomesticOrderHistoryItem>, KisError> {
    let (cano, prdt_cd) = split_account(&config.account_num);

    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header(
            "tr_id",
            if config.is_domestic_virtual {
                "VTTC0081R"
            } else {
                "TTTC0081R"
            },
        )
        .header("custtype", "P")
        .query(&[
            ("CANO", cano),
            ("ACNT_PRDT_CD", prdt_cd),
            ("INQR_STRT_DT", req.start_date.as_str()),
            ("INQR_END_DT", req.end_date.as_str()),
            ("SLL_BUY_DVSN_CD", "00"),
            ("INQR_DVSN", "00"),
            ("PDNO", ""),
            ("CCLD_DVSN", "01"),
            ("ORD_GNO_BRNO", ""),
            ("ODNO", ""),
            ("INQR_DVSN_3", "00"),
            ("INQR_DVSN_1", ""),
            ("CTX_AREA_FK100", ""),
            ("CTX_AREA_NK100", ""),
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

    let items = json["output1"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|o| {
                    let qty: u32 = o["ord_qty"].as_str()?.parse().ok()?;
                    let filled_qty: u32 = o["tot_ccld_qty"].as_str()?.parse().ok()?;
                    let filled_price =
                        Decimal::from_str(o["avg_prvs"].as_str().unwrap_or("0")).ok()?;
                    Some(DomesticOrderHistoryItem {
                        order_no: o["odno"].as_str()?.into(),
                        symbol: o["pdno"].as_str()?.into(),
                        side_cd: o["sll_buy_dvsn_cd"].as_str()?.into(),
                        qty,
                        filled_qty,
                        filled_price,
                        filled_date: o["ord_dt"].as_str().unwrap_or("").into(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(items)
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
    let mkt_div = req.exchange.market_code();

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
            ("FID_COND_MRKT_DIV_CODE", mkt_div),
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
            "{}/uapi/domestic-stock/v1/quotations/volume-rank",
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

/// 국내휴장일 조회 (TCA0903R)
/// `date`: 기준일자 YYYYMMDD — 해당 날짜의 개장 여부를 반환.
/// `opnd_yn == "N"` 인 날짜를 휴장일로 간주하여 `Holiday` vec 에 담아 반환.
/// `opnd_yn == "Y"` 이면 빈 vec 반환.
pub async fn domestic_check_holiday(
    client: &Client,
    config: &KisConfig,
    token: &str,
    date: &str,
) -> Result<Vec<crate::Holiday>, KisError> {
    // CTCA0903R은 VTS 미지원 — 빈 목록 반환 (개장으로 간주)
    if config.is_domestic_virtual {
        return Ok(vec![]);
    }
    let resp = client
        .get(format!(
            "{}/uapi/domestic-stock/v1/quotations/chk-holiday",
            config.rest_url
        ))
        .header("authorization", format!("Bearer {}", token))
        .header("appkey", &config.app_key)
        .header("appsecret", &config.app_secret)
        .header("tr_id", "CTCA0903R")
        .header("custtype", "P")
        .query(&[("BASS_DT", date), ("CTX_AREA_FK", ""), ("CTX_AREA_NK", "")])
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

    let holidays = json["output"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let d = item["bass_dt"].as_str()?;
                    // opnd_yn = "N" → 개장하지 않는 날 = 휴장일
                    if item["opnd_yn"].as_str().unwrap_or("Y") == "N" {
                        Some(crate::Holiday {
                            date: d.to_string(),
                            weekday: item["wday_dvsn_cd"].as_str().unwrap_or("").to_string(),
                            name: "휴장일".to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(holidays)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn deserialize_unfilled_orders() {
        let json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/domestic/inquiry/unfilled_orders.json"
        ))
        .unwrap();

        let orders: Vec<DomesticUnfilledOrder> = json["output"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|o| {
                let qty: u32 = o["ord_qty"].as_str()?.parse().ok()?;
                let remaining: u32 = o["psbl_qty"].as_str()?.parse().ok()?;
                let filled = qty.saturating_sub(remaining);
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
            .collect();

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_no, "0000123456");
        assert_eq!(orders[0].symbol, "005930");
        assert_eq!(orders[0].exchange, "KSC");
        assert_eq!(orders[0].side_cd, "02");
        assert_eq!(orders[0].qty, 10);
        assert_eq!(orders[0].filled_qty, 0);
        assert_eq!(orders[0].remaining_qty, 10);
        assert_eq!(orders[0].price, dec!(70000));
    }

    #[test]
    fn deserialize_daily_chart() {
        let json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/domestic/inquiry/daily_chart.json"
        ))
        .unwrap();

        let bars: Vec<crate::CandleBar> = json["output2"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|b| {
                Some(crate::CandleBar {
                    date: b["stck_bsop_date"].as_str()?.into(),
                    open: Decimal::from_str(b["stck_oprc"].as_str()?).ok()?,
                    high: Decimal::from_str(b["stck_hgpr"].as_str()?).ok()?,
                    low: Decimal::from_str(b["stck_lwpr"].as_str()?).ok()?,
                    close: Decimal::from_str(b["stck_clpr"].as_str()?).ok()?,
                    volume: Decimal::from_str(b["acml_vol"].as_str()?).ok()?,
                })
            })
            .collect();

        assert_eq!(bars.len(), 2);
        assert_eq!(bars[0].date, "20260326");
        assert_eq!(bars[0].close, dec!(72000));
        assert_eq!(bars[0].volume, dec!(15000000));
        assert_eq!(bars[1].date, "20260325");
        assert_eq!(bars[1].close, dec!(71000));
    }

    #[test]
    fn deserialize_volume_ranking() {
        let json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/domestic/inquiry/volume_ranking.json"
        ))
        .unwrap();

        let items: Vec<DomesticRankingItem> = json["output"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|i| {
                Some(DomesticRankingItem {
                    symbol: i["mksc_shrn_iscd"].as_str()?.into(),
                    name: i["hts_kor_isnm"].as_str().unwrap_or("").into(),
                    exchange: "J".into(),
                    price: Decimal::from_str(i["stck_prpr"].as_str()?).ok()?,
                    volume: Decimal::from_str(i["acml_vol"].as_str()?).ok()?,
                })
            })
            .collect();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].symbol, "005930");
        assert_eq!(items[0].name, "삼성전자");
        assert_eq!(items[0].price, dec!(72000));
        assert_eq!(items[0].volume, dec!(15000000));
        assert_eq!(items[1].symbol, "000660");
    }

    #[test]
    fn deserialize_domestic_balance() {
        let json: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/domestic/inquiry/balance.json"
        ))
        .unwrap();

        let items: Vec<BalanceItem> = json["output1"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|i| {
                Some(BalanceItem {
                    symbol: i["pdno"].as_str()?.into(),
                    name: i["prdt_name"].as_str().unwrap_or("").into(),
                    exchange: "KSC".into(),
                    qty: Decimal::from_str(i["hldg_qty"].as_str()?).ok()?,
                    avg_price: Decimal::from_str(i["pchs_avg_pric"].as_str()?).ok()?,
                    eval_amount: Decimal::from_str(i["evlu_amt"].as_str().unwrap_or("0"))
                        .unwrap_or(Decimal::ZERO),
                    unrealized_pnl: Decimal::from_str(i["evlu_pfls_amt"].as_str().unwrap_or("0"))
                        .unwrap_or(Decimal::ZERO),
                    pnl_rate: Decimal::from_str(i["evlu_pfls_rt"].as_str().unwrap_or("0"))
                        .unwrap_or(Decimal::ZERO),
                })
            })
            .collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].symbol, "005930");
        assert_eq!(items[0].qty, dec!(10));

        let out2 = json["output2"].as_array().and_then(|a| a.first()).unwrap();
        let purchase_amount = Decimal::from_str(out2["dnca_tot_amt"].as_str().unwrap()).unwrap();
        assert_eq!(purchase_amount, dec!(5000000));
    }
}
