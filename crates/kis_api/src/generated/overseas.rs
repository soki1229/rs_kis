#![allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct OverseasCommon(pub(crate) KisClient);

#[allow(dead_code)]
pub struct OverseasTrading(pub(crate) KisClient);

#[allow(dead_code)]
pub struct OverseasQuotations(pub(crate) KisClient);

#[allow(dead_code)]
pub struct OverseasRanking(pub(crate) KisClient);

impl crate::endpoints::Overseas {
    pub fn common(&self) -> OverseasCommon { OverseasCommon(self.0.clone()) }
    pub fn trading(&self) -> OverseasTrading { OverseasTrading(self.0.clone()) }
    pub fn quotations(&self) -> OverseasQuotations { OverseasQuotations(self.0.clone()) }
    pub fn ranking(&self) -> OverseasRanking { OverseasRanking(self.0.clone()) }
}

#[allow(non_snake_case)]
impl OverseasCommon {
    /// 접근토큰발급(P)[인증-001]
    /// - TR_ID: Real= / VTS=
    /// - Endpoint: /oauth2/tokenP
    pub async fn oauth2_token_p(&self, req: AuthOauth2TokenPRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/tokenP", tr_id, req).await
    }

    /// 접근토큰폐기(P)[인증-002]
    /// - TR_ID: Real=None / VTS=None
    /// - Endpoint: /oauth2/revokeP
    pub async fn oauth2_revoke_p(&self, req: AuthOauth2RevokePRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "None",
            crate::client::KisEnv::Vts => "None",
        };
        self.0.post("/oauth2/revokeP", tr_id, req).await
    }

    /// Hashkey
    /// - TR_ID: Real=None / VTS=None
    /// - Endpoint: /uapi/hashkey
    pub async fn hashkey(&self, req: AuthHashkeyRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "None",
            crate::client::KisEnv::Vts => "None",
        };
        self.0.post("/uapi/hashkey", tr_id, req).await
    }

    /// 실시간 (웹소켓) 접속키 발급[실시간-000]
    /// - TR_ID: Real=None / VTS=None
    /// - Endpoint: /oauth2/Approval
    pub async fn oauth2_approval(&self, req: AuthOauth2ApprovalRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "None",
            crate::client::KisEnv::Vts => "None",
        };
        self.0.post("/oauth2/Approval", tr_id, req).await
    }

}

#[allow(non_snake_case)]
impl OverseasTrading {
    /// 해외주식 주문[v1_해외주식-001]
    /// - TR_ID: Real=(미국매수) TTTT1002U  (미국매도) TTTT1006U (아시아 국가 하단 규격서 참고) / VTS=(미국매수) VTTT1002U  (미국매도) VTTT1001U  (아시아 국가 하단 규격서 참고)
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order
    pub async fn overseas_stock_v1_trading_order(&self, req: OverseasStockV1TradingOrderRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "(미국매수) TTTT1002U  (미국매도) TTTT1006U (아시아 국가 하단 규격서 참고)",
            crate::client::KisEnv::Vts => "(미국매수) VTTT1002U  (미국매도) VTTT1001U  (아시아 국가 하단 규격서 참고)",
        };
        self.0.post("/uapi/overseas-stock/v1/trading/order", tr_id, req).await
    }

    /// 해외주식 정정취소주문[v1_해외주식-003]
    /// - TR_ID: Real=(미국 정정·취소) TTTT1004U (아시아 국가 하단 규격서 참고) / VTS=(미국 정정·취소) VTTT1004U (아시아 국가 하단 규격서 참고)
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-rvsecncl
    pub async fn overseas_stock_v1_trading_order_rvsecncl(&self, req: OverseasStockV1TradingOrderRvsecnclRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "(미국 정정·취소) TTTT1004U (아시아 국가 하단 규격서 참고)",
            crate::client::KisEnv::Vts => "(미국 정정·취소) VTTT1004U (아시아 국가 하단 규격서 참고)",
        };
        self.0.post("/uapi/overseas-stock/v1/trading/order-rvsecncl", tr_id, req).await
    }

    /// 해외주식 예약주문접수[v1_해외주식-002]
    /// - TR_ID: Real=(미국예약매수) TTTT3014U  (미국예약매도) TTTT3016U   (중국/홍콩/일본/베트남 예약주문) TTTS3013U / VTS=(미국예약매수) VTTT3014U  (미국예약매도) VTTT3016U   (중국/홍콩/일본/베트남 예약주문) VTTS3013U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv
    pub async fn overseas_stock_v1_trading_order_resv(&self, req: OverseasStockV1TradingOrderResvRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "(미국예약매수) TTTT3014U  (미국예약매도) TTTT3016U   (중국/홍콩/일본/베트남 예약주문) TTTS3013U",
            crate::client::KisEnv::Vts => "(미국예약매수) VTTT3014U  (미국예약매도) VTTT3016U   (중국/홍콩/일본/베트남 예약주문) VTTS3013U",
        };
        self.0.post("/uapi/overseas-stock/v1/trading/order-resv", tr_id, req).await
    }

    /// 해외주식 예약주문접수취소[v1_해외주식-004]
    /// - TR_ID: Real=(미국 예약주문 취소접수) TTTT3017U (아시아국가 미제공) / VTS=(미국 예약주문 취소접수) VTTT3017U (아시아국가 미제공)
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-ccnl
    pub async fn overseas_stock_v1_trading_order_resv_ccnl(&self, req: OverseasStockV1TradingOrderResvCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "(미국 예약주문 취소접수) TTTT3017U (아시아국가 미제공)",
            crate::client::KisEnv::Vts => "(미국 예약주문 취소접수) VTTT3017U (아시아국가 미제공)",
        };
        self.0.post("/uapi/overseas-stock/v1/trading/order-resv-ccnl", tr_id, req).await
    }

    /// 해외주식 매수가능금액조회[v1_해외주식-014]
    /// - TR_ID: Real=TTTS3007R / VTS=VTTS3007R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-psamount
    pub async fn overseas_stock_v1_trading_inquire_psamount(&self, req: OverseasStockV1TradingInquirePsamountRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3007R",
            crate::client::KisEnv::Vts => "VTTS3007R",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-psamount", tr_id, req).await
    }

    /// 해외주식 미체결내역[v1_해외주식-005]
    /// - TR_ID: Real=TTTS3018R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-nccs
    pub async fn overseas_stock_v1_trading_inquire_nccs(&self, req: OverseasStockV1TradingInquireNccsRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3018R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-nccs", tr_id, req).await
    }

    /// 해외주식 잔고[v1_해외주식-006]
    /// - TR_ID: Real=TTTS3012R / VTS=VTTS3012R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-balance
    pub async fn overseas_stock_v1_trading_inquire_balance(&self, req: OverseasStockV1TradingInquireBalanceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3012R",
            crate::client::KisEnv::Vts => "VTTS3012R",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-balance", tr_id, req).await
    }

    /// 해외주식 주문체결내역[v1_해외주식-007]
    /// - TR_ID: Real=TTTS3035R / VTS=VTTS3035R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-ccnl
    pub async fn overseas_stock_v1_trading_inquire_ccnl(&self, req: OverseasStockV1TradingInquireCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3035R",
            crate::client::KisEnv::Vts => "VTTS3035R",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-ccnl", tr_id, req).await
    }

    /// 해외주식 체결기준현재잔고[v1_해외주식-008]
    /// - TR_ID: Real=CTRP6504R / VTS=VTRP6504R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-present-balance
    pub async fn overseas_stock_v1_trading_inquire_present_balance(&self, req: OverseasStockV1TradingInquirePresentBalanceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRP6504R",
            crate::client::KisEnv::Vts => "VTRP6504R",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-present-balance", tr_id, req).await
    }

    /// 해외주식 예약주문조회[v1_해외주식-013]
    /// - TR_ID: Real=(미국) TTTT3039R (일본/중국/홍콩/베트남) TTTS3014R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-list
    pub async fn overseas_stock_v1_trading_order_resv_list(&self, req: OverseasStockV1TradingOrderResvListRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "(미국) TTTT3039R (일본/중국/홍콩/베트남) TTTS3014R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/order-resv-list", tr_id, req).await
    }

    /// 해외주식 결제기준잔고 [해외주식-064]
    /// - TR_ID: Real=CTRP6010R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance
    pub async fn overseas_stock_v1_trading_inquire_paymt_stdr_balance(&self, req: OverseasStockV1TradingInquirePaymtStdrBalanceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRP6010R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance", tr_id, req).await
    }

    /// 해외주식 일별거래내역 [해외주식-063]
    /// - TR_ID: Real=CTOS4001R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-period-trans
    pub async fn overseas_stock_v1_trading_inquire_period_trans(&self, req: OverseasStockV1TradingInquirePeriodTransRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTOS4001R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-period-trans", tr_id, req).await
    }

    /// 해외주식 기간손익[v1_해외주식-032]
    /// - TR_ID: Real=TTTS3039R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-period-profit
    pub async fn overseas_stock_v1_trading_inquire_period_profit(&self, req: OverseasStockV1TradingInquirePeriodProfitRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3039R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-period-profit", tr_id, req).await
    }

    /// 해외증거금 통화별조회 [해외주식-035]
    /// - TR_ID: Real=TTTC2101R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/foreign-margin
    pub async fn overseas_stock_v1_trading_foreign_margin(&self, req: OverseasStockV1TradingForeignMarginRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC2101R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/foreign-margin", tr_id, req).await
    }

    /// 해외주식 미국주간주문[v1_해외주식-026]
    /// - TR_ID: Real=(주간매수) TTTS6036U (주간매도) TTTS6037U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order
    pub async fn overseas_stock_v1_trading_daytime_order(&self, req: OverseasStockV1TradingDaytimeOrderRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "(주간매수) TTTS6036U (주간매도) TTTS6037U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/uapi/overseas-stock/v1/trading/daytime-order", tr_id, req).await
    }

    /// 해외주식 미국주간정정취소[v1_해외주식-027]
    /// - TR_ID: Real=TTTS6038U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order-rvsecncl
    pub async fn overseas_stock_v1_trading_daytime_order_rvsecncl(&self, req: OverseasStockV1TradingDaytimeOrderRvsecnclRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6038U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/uapi/overseas-stock/v1/trading/daytime-order-rvsecncl", tr_id, req).await
    }

    /// 해외주식 지정가주문번호조회  [해외주식-071]
    /// - TR_ID: Real=TTTS6058R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/algo-ordno
    pub async fn overseas_stock_v1_trading_algo_ordno(&self, req: OverseasStockV1TradingAlgoOrdnoRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6058R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/algo-ordno", tr_id, req).await
    }

    /// 해외주식 지정가체결내역조회 [해외주식-070]
    /// - TR_ID: Real=TTTS6059R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-algo-ccnl
    pub async fn overseas_stock_v1_trading_inquire_algo_ccnl(&self, req: OverseasStockV1TradingInquireAlgoCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6059R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/trading/inquire-algo-ccnl", tr_id, req).await
    }

}

#[allow(non_snake_case)]
impl OverseasQuotations {
    /// 해외주식 현재가상세[v1_해외주식-029]
    /// - TR_ID: Real=HHDFS76200200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/price-detail
    pub async fn overseas_price_v1_quotations_price_detail(&self, req: OverseasPriceV1QuotationsPriceDetailRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76200200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/price-detail", tr_id, req).await
    }

    /// 해외주식 현재가 호가 [해외주식-033]
    /// - TR_ID: Real=HHDFS76200100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-asking-price
    pub async fn overseas_price_v1_quotations_inquire_asking_price(&self, req: OverseasPriceV1QuotationsInquireAskingPriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76200100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/inquire-asking-price", tr_id, req).await
    }

    /// 해외주식 현재체결가[v1_해외주식-009]
    /// - TR_ID: Real=HHDFS00000300 / VTS=HHDFS00000300
    /// - Endpoint: /uapi/overseas-price/v1/quotations/price
    pub async fn overseas_price_v1_quotations_price(&self, req: OverseasPriceV1QuotationsPriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS00000300",
            crate::client::KisEnv::Vts => "HHDFS00000300",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/price", tr_id, req).await
    }

    /// 해외주식 체결추이[해외주식-037]
    /// - TR_ID: Real=HHDFS76200300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-ccnl
    pub async fn overseas_price_v1_quotations_inquire_ccnl(&self, req: OverseasPriceV1QuotationsInquireCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76200300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/inquire-ccnl", tr_id, req).await
    }

    /// 해외주식분봉조회[v1_해외주식-030]
    /// - TR_ID: Real=HHDFS76950200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-time-itemchartprice
    pub async fn overseas_price_v1_quotations_inquire_time_itemchartprice(&self, req: OverseasPriceV1QuotationsInquireTimeItemchartpriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76950200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/inquire-time-itemchartprice", tr_id, req).await
    }

    /// 해외지수분봉조회[v1_해외주식-031]
    /// - TR_ID: Real=FHKST03030200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-time-indexchartprice
    pub async fn overseas_price_v1_quotations_inquire_time_indexchartprice(&self, req: OverseasPriceV1QuotationsInquireTimeIndexchartpriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03030200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/inquire-time-indexchartprice", tr_id, req).await
    }

    /// 해외주식 기간별시세[v1_해외주식-010]
    /// - TR_ID: Real=HHDFS76240000 / VTS=HHDFS76240000
    /// - Endpoint: /uapi/overseas-price/v1/quotations/dailyprice
    pub async fn overseas_price_v1_quotations_dailyprice(&self, req: OverseasPriceV1QuotationsDailypriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76240000",
            crate::client::KisEnv::Vts => "HHDFS76240000",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/dailyprice", tr_id, req).await
    }

    /// 해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]
    /// - TR_ID: Real=FHKST03030100 / VTS=FHKST03030100
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-daily-chartprice
    pub async fn overseas_price_v1_quotations_inquire_daily_chartprice(&self, req: OverseasPriceV1QuotationsInquireDailyChartpriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03030100",
            crate::client::KisEnv::Vts => "FHKST03030100",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/inquire-daily-chartprice", tr_id, req).await
    }

    /// 해외주식조건검색[v1_해외주식-015]
    /// - TR_ID: Real=HHDFS76410000 / VTS=HHDFS76410000
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-search
    pub async fn overseas_price_v1_quotations_inquire_search(&self, req: OverseasPriceV1QuotationsInquireSearchRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76410000",
            crate::client::KisEnv::Vts => "HHDFS76410000",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/inquire-search", tr_id, req).await
    }

    /// 해외결제일자조회[해외주식-017]
    /// - TR_ID: Real=CTOS5011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/quotations/countries-holiday
    pub async fn overseas_stock_v1_quotations_countries_holiday(&self, req: OverseasStockV1QuotationsCountriesHolidayRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTOS5011R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/quotations/countries-holiday", tr_id, req).await
    }

    /// 해외주식 상품기본정보[v1_해외주식-034]
    /// - TR_ID: Real=CTPF1702R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/search-info
    pub async fn overseas_price_v1_quotations_search_info(&self, req: OverseasPriceV1QuotationsSearchInfoRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF1702R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/search-info", tr_id, req).await
    }

    /// 해외주식 업종별시세[해외주식-048]
    /// - TR_ID: Real=HHDFS76370000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/industry-theme
    pub async fn overseas_price_v1_quotations_industry_theme(&self, req: OverseasPriceV1QuotationsIndustryThemeRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76370000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/industry-theme", tr_id, req).await
    }

    /// 해외주식 업종별코드조회[해외주식-049]
    /// - TR_ID: Real=HHDFS76370100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/industry-price
    pub async fn overseas_price_v1_quotations_industry_price(&self, req: OverseasPriceV1QuotationsIndustryPriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76370100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/industry-price", tr_id, req).await
    }

    /// 해외주식 복수종목 시세조회
    /// - TR_ID: Real=HHDFS76220000  / VTS=미지원 
    /// - Endpoint: /uapi/overseas-price/v1/quotations/multprice
    pub async fn overseas_price_v1_quotations_multprice(&self, req: OverseasPriceV1QuotationsMultpriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76220000 ",
            crate::client::KisEnv::Vts => "미지원 ",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/multprice", tr_id, req).await
    }

    /// 해외주식 기간별권리조회 [해외주식-052]
    /// - TR_ID: Real=CTRGT011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/period-rights
    pub async fn overseas_price_v1_quotations_period_rights(&self, req: OverseasPriceV1QuotationsPeriodRightsRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRGT011R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/period-rights", tr_id, req).await
    }

    /// 해외뉴스종합(제목) [해외주식-053]
    /// - TR_ID: Real=HHPSTH60100C1 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/news-title
    pub async fn overseas_price_v1_quotations_news_title(&self, req: OverseasPriceV1QuotationsNewsTitleRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHPSTH60100C1",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/news-title", tr_id, req).await
    }

    /// 해외주식 권리종합 [해외주식-050]
    /// - TR_ID: Real=HHDFS78330900 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/rights-by-ice
    pub async fn overseas_price_v1_quotations_rights_by_ice(&self, req: OverseasPriceV1QuotationsRightsByIceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS78330900",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/rights-by-ice", tr_id, req).await
    }

    /// 당사 해외주식담보대출 가능 종목 [해외주식-051]
    /// - TR_ID: Real=CTLN4050R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/colable-by-company
    pub async fn overseas_price_v1_quotations_colable_by_company(&self, req: OverseasPriceV1QuotationsColableByCompanyRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTLN4050R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/colable-by-company", tr_id, req).await
    }

    /// 해외속보(제목) [해외주식-055]
    /// - TR_ID: Real=FHKST01011801 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/brknews-title
    pub async fn overseas_price_v1_quotations_brknews_title(&self, req: OverseasPriceV1QuotationsBrknewsTitleRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01011801",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-price/v1/quotations/brknews-title", tr_id, req).await
    }

}

#[allow(non_snake_case)]
impl OverseasRanking {
    /// 해외주식 가격급등락[해외주식-038]
    /// - TR_ID: Real=HHDFS76260000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/price-fluct
    pub async fn overseas_stock_v1_ranking_price_fluct(&self, req: OverseasStockV1RankingPriceFluctRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76260000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/price-fluct", tr_id, req).await
    }

    /// 해외주식 거래량급증[해외주식-039]
    /// - TR_ID: Real=HHDFS76270000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/volume-surge
    pub async fn overseas_stock_v1_ranking_volume_surge(&self, req: OverseasStockV1RankingVolumeSurgeRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76270000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/volume-surge", tr_id, req).await
    }

    /// 해외주식 매수체결강도상위[해외주식-040]
    /// - TR_ID: Real=HHDFS76280000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/volume-power
    pub async fn overseas_stock_v1_ranking_volume_power(&self, req: OverseasStockV1RankingVolumePowerRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76280000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/volume-power", tr_id, req).await
    }

    /// 해외주식 상승율/하락율[해외주식-041]
    /// - TR_ID: Real=HHDFS76290000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/updown-rate
    pub async fn overseas_stock_v1_ranking_updown_rate(&self, req: OverseasStockV1RankingUpdownRateRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76290000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/updown-rate", tr_id, req).await
    }

    /// 해외주식 신고/신저가[해외주식-042]
    /// - TR_ID: Real=HHDFS76300000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/new-highlow
    pub async fn overseas_stock_v1_ranking_new_highlow(&self, req: OverseasStockV1RankingNewHighlowRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76300000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/new-highlow", tr_id, req).await
    }

    /// 해외주식 거래량순위[해외주식-043]
    /// - TR_ID: Real=HHDFS76310010 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-vol
    pub async fn overseas_stock_v1_ranking_trade_vol(&self, req: OverseasStockV1RankingTradeVolRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76310010",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/trade-vol", tr_id, req).await
    }

    /// 해외주식 거래대금순위[해외주식-044]
    /// - TR_ID: Real=HHDFS76320010 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-pbmn
    pub async fn overseas_stock_v1_ranking_trade_pbmn(&self, req: OverseasStockV1RankingTradePbmnRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76320010",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/trade-pbmn", tr_id, req).await
    }

    /// 해외주식 거래증가율순위[해외주식-045]
    /// - TR_ID: Real=HHDFS76330000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-growth
    pub async fn overseas_stock_v1_ranking_trade_growth(&self, req: OverseasStockV1RankingTradeGrowthRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76330000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/trade-growth", tr_id, req).await
    }

    /// 해외주식 거래회전율순위[해외주식-046]
    /// - TR_ID: Real=HHDFS76340000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-turnover
    pub async fn overseas_stock_v1_ranking_trade_turnover(&self, req: OverseasStockV1RankingTradeTurnoverRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76340000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/trade-turnover", tr_id, req).await
    }

    /// 해외주식 시가총액순위[해외주식-047]
    /// - TR_ID: Real=HHDFS76350100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/market-cap
    pub async fn overseas_stock_v1_ranking_market_cap(&self, req: OverseasStockV1RankingMarketCapRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76350100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.get("/uapi/overseas-stock/v1/ranking/market-cap", tr_id, req).await
    }

}
