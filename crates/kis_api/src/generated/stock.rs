#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct StockTrading(pub(crate) KisClient);

#[allow(dead_code)]
pub struct StockQuotations(pub(crate) KisClient);

#[allow(dead_code)]
pub struct StockCommon(pub(crate) KisClient);

#[allow(dead_code)]
pub struct StockRanking(pub(crate) KisClient);

impl crate::endpoints::Stock {
    pub fn trading(&self) -> StockTrading {
        StockTrading(self.0.clone())
    }
    pub fn quotations(&self) -> StockQuotations {
        StockQuotations(self.0.clone())
    }
    pub fn common(&self) -> StockCommon {
        StockCommon(self.0.clone())
    }
    pub fn ranking(&self) -> StockRanking {
        StockRanking(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl StockTrading {
    /// 주식주문(현금)[v1_국내주식-001]
    /// - TR_ID: Real=(매도) TTTC0011U (매수) TTTC0012U / VTS=(매도) VTTC0011U (매수) VTTC0012U
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-cash
    pub async fn domestic_stock_v1_trading_order_cash(
        &self,
        req: DomesticStockV1TradingOrderCashRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(매도) TTTC0011U (매수) TTTC0012U",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => (
                "(매도) VTTC0011U (매수) VTTC0012U",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .post(
                "/uapi/domestic-stock/v1/trading/order-cash",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식주문(신용)[v1_국내주식-002]
    /// - TR_ID: Real=(매도) TTTC0051U (매수) TTTC0052U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-credit
    pub async fn domestic_stock_v1_trading_order_credit(
        &self,
        req: DomesticStockV1TradingOrderCreditRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(매도) TTTC0051U (매수) TTTC0052U",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/domestic-stock/v1/trading/order-credit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식주문(정정취소)[v1_국내주식-003]
    /// - TR_ID: Real=TTTC0013U / VTS=VTTC0013U
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-rvsecncl
    pub async fn domestic_stock_v1_trading_order_rvsecncl(
        &self,
        req: DomesticStockV1TradingOrderRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0013U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTC0013U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/domestic-stock/v1/trading/order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식정정취소가능주문조회[v1_국내주식-004]
    /// - TR_ID: Real=TTTC0084R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl
    pub async fn domestic_stock_v1_trading_inquire_psbl_rvsecncl(
        &self,
        req: DomesticStockV1TradingInquirePsblRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0084R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식일별주문체결조회[v1_국내주식-005]
    /// - TR_ID: Real=(3개월이내) TTTC0081R (3개월이전) CTSC9215R / VTS=(3개월이내) VTTC0081R (3개월이전) VTSC9215R
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-daily-ccld
    pub async fn domestic_stock_v1_trading_inquire_daily_ccld(
        &self,
        req: DomesticStockV1TradingInquireDailyCcldRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(3개월이내) TTTC0081R (3개월이전) CTSC9215R",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => (
                "(3개월이내) VTTC0081R (3개월이전) VTSC9215R",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식잔고조회[v1_국내주식-006]
    /// - TR_ID: Real=TTTC8434R / VTS=VTTC8434R
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-balance
    pub async fn domestic_stock_v1_trading_inquire_balance(
        &self,
        req: DomesticStockV1TradingInquireBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8434R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTC8434R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 매수가능조회[v1_국내주식-007]
    /// - TR_ID: Real=TTTC8908R / VTS=VTTC8908R
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-order
    pub async fn domestic_stock_v1_trading_inquire_psbl_order(
        &self,
        req: DomesticStockV1TradingInquirePsblOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8908R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTC8908R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-psbl-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 매도가능수량조회 [국내주식-165]
    /// - TR_ID: Real=TTTC8408R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-sell
    pub async fn domestic_stock_v1_trading_inquire_psbl_sell(
        &self,
        req: DomesticStockV1TradingInquirePsblSellRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8408R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-psbl-sell",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 신용매수가능조회[v1_국내주식-042]
    /// - TR_ID: Real=TTTC8909R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-credit-psamount
    pub async fn domestic_stock_v1_trading_inquire_credit_psamount(
        &self,
        req: DomesticStockV1TradingInquireCreditPsamountRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8909R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-credit-psamount",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식예약주문[v1_국내주식-017]
    /// - TR_ID: Real=CTSC0008U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-resv
    pub async fn domestic_stock_v1_trading_order_resv(
        &self,
        req: DomesticStockV1TradingOrderResvRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTSC0008U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/domestic-stock/v1/trading/order-resv",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식예약주문정정취소[v1_국내주식-018,019]
    /// - TR_ID: Real=(예약취소) CTSC0009U (예약정정) CTSC0013U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-resv-rvsecncl
    pub async fn domestic_stock_v1_trading_order_resv_rvsecncl(
        &self,
        req: DomesticStockV1TradingOrderResvRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(예약취소) CTSC0009U (예약정정) CTSC0013U",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/domestic-stock/v1/trading/order-resv-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식예약주문조회[v1_국내주식-020]
    /// - TR_ID: Real=CTSC0004R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-resv-ccnl
    pub async fn domestic_stock_v1_trading_order_resv_ccnl(
        &self,
        req: DomesticStockV1TradingOrderResvCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTSC0004R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/order-resv-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 퇴직연금 체결기준잔고[v1_국내주식-032]
    /// - TR_ID: Real=TTTC2202R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-present-balance
    pub async fn domestic_stock_v1_trading_pension_inquire_present_balance(
        &self,
        req: DomesticStockV1TradingPensionInquirePresentBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC2202R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-present-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 퇴직연금 미체결내역[v1_국내주식-033]
    /// - TR_ID: Real=TTTC2201R(기존 KRX만 가능), TTTC2210R (KRX,NXT/SOR) / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld
    pub async fn domestic_stock_v1_trading_pension_inquire_daily_ccld(
        &self,
        req: DomesticStockV1TradingPensionInquireDailyCcldRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "TTTC2201R(기존 KRX만 가능), TTTC2210R (KRX,NXT/SOR)",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 퇴직연금 매수가능조회[v1_국내주식-034]
    /// - TR_ID: Real=TTTC0503R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-psbl-order
    pub async fn domestic_stock_v1_trading_pension_inquire_psbl_order(
        &self,
        req: DomesticStockV1TradingPensionInquirePsblOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0503R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-psbl-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 퇴직연금 예수금조회[v1_국내주식-035]
    /// - TR_ID: Real=TTTC0506R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-deposit
    pub async fn domestic_stock_v1_trading_pension_inquire_deposit(
        &self,
        req: DomesticStockV1TradingPensionInquireDepositRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0506R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-deposit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 퇴직연금 잔고조회[v1_국내주식-036]
    /// - TR_ID: Real=TTTC2208R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-balance
    pub async fn domestic_stock_v1_trading_pension_inquire_balance(
        &self,
        req: DomesticStockV1TradingPensionInquireBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC2208R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식잔고조회_실현손익[v1_국내주식-041]
    /// - TR_ID: Real=TTTC8494R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl
    pub async fn domestic_stock_v1_trading_inquire_balance_rlz_pl(
        &self,
        req: DomesticStockV1TradingInquireBalanceRlzPlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8494R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 투자계좌자산현황조회[v1_국내주식-048]
    /// - TR_ID: Real=CTRP6548R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-account-balance
    pub async fn domestic_stock_v1_trading_inquire_account_balance(
        &self,
        req: DomesticStockV1TradingInquireAccountBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTRP6548R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-account-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 기간별손익일별합산조회[v1_국내주식-052]
    /// - TR_ID: Real=TTTC8708R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-period-profit
    pub async fn domestic_stock_v1_trading_inquire_period_profit(
        &self,
        req: DomesticStockV1TradingInquirePeriodProfitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8708R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-period-profit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 기간별매매손익현황조회[v1_국내주식-060]
    /// - TR_ID: Real=TTTC8715R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-period-trade-profit
    pub async fn domestic_stock_v1_trading_inquire_period_trade_profit(
        &self,
        req: DomesticStockV1TradingInquirePeriodTradeProfitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8715R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-period-trade-profit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식통합증거금 현황 [국내주식-191]
    /// - TR_ID: Real=TTTC0869R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/intgr-margin
    pub async fn domestic_stock_v1_trading_intgr_margin(
        &self,
        req: DomesticStockV1TradingIntgrMarginRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0869R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/intgr-margin",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 기간별계좌권리현황조회 [국내주식-211]
    /// - TR_ID: Real=CTRGA011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/period-rights
    pub async fn domestic_stock_v1_trading_period_rights(
        &self,
        req: DomesticStockV1TradingPeriodRightsRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTRGA011R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/period-rights",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 주문[v1_국내선물-001]
    /// - TR_ID: Real=(주간 매수/매도) TTTO1101U (야간 매수/매도) (구) JTCE1001U (신) STTN1101U / VTS=(주간 매수/매도) VTTO1101U (야간은 모의투자 미제공)
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/order
    pub async fn domestic_futureoption_v1_trading_order(
        &self,
        req: DomesticFutureoptionV1TradingOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(주간 매수/매도) TTTO1101U (야간 매수/매도) (구) JTCE1001U (신) STTN1101U",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => (
                "(주간 매수/매도) VTTO1101U (야간은 모의투자 미제공)",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .post(
                "/uapi/domestic-futureoption/v1/trading/order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 정정취소주문[v1_국내선물-002]
    /// - TR_ID: Real=(주간 정정/취소) TTTO1103U (야간 정정/취소) (구) JTCE1002U (신) STTN1103U / VTS=(주간 정정/취소) VTTO1103U (야간은 모의투자 미제공)
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/order-rvsecncl
    pub async fn domestic_futureoption_v1_trading_order_rvsecncl(
        &self,
        req: DomesticFutureoptionV1TradingOrderRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(주간 정정/취소) TTTO1103U (야간 정정/취소) (구) JTCE1002U (신) STTN1103U",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => (
                "(주간 정정/취소) VTTO1103U (야간은 모의투자 미제공)",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .post(
                "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 주문체결내역조회[v1_국내선물-003]
    /// - TR_ID: Real=TTTO5201R / VTS=VTTO5201R
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ccnl
    pub async fn domestic_futureoption_v1_trading_inquire_ccnl(
        &self,
        req: DomesticFutureoptionV1TradingInquireCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTO5201R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTO5201R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 잔고현황[v1_국내선물-004]
    /// - TR_ID: Real=CTFO6118R / VTS=VTFO6118R
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance
    pub async fn domestic_futureoption_v1_trading_inquire_balance(
        &self,
        req: DomesticFutureoptionV1TradingInquireBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTFO6118R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTFO6118R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 주문가능[v1_국내선물-005]
    /// - TR_ID: Real=TTTO5105R / VTS=VTTO5105R
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-psbl-order
    pub async fn domestic_futureoption_v1_trading_inquire_psbl_order(
        &self,
        req: DomesticFutureoptionV1TradingInquirePsblOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTO5105R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTO5105R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-psbl-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// (야간)선물옵션 주문체결 내역조회 [국내선물-009]
    /// - TR_ID: Real=(구) JTCE5005R (신) STTN5201R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl
    pub async fn domestic_futureoption_v1_trading_inquire_ngt_ccnl(
        &self,
        req: DomesticFutureoptionV1TradingInquireNgtCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(구) JTCE5005R (신) STTN5201R",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// (야간)선물옵션 잔고현황 [국내선물-010]
    /// - TR_ID: Real=(구) JTCE6001R (신) CTFN6118R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ngt-balance
    pub async fn domestic_futureoption_v1_trading_inquire_ngt_balance(
        &self,
        req: DomesticFutureoptionV1TradingInquireNgtBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(구) JTCE6001R (신) CTFN6118R",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ngt-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// (야간)선물옵션 주문가능 조회 [국내선물-011]
    /// - TR_ID: Real=(구) JTCE1004R (신) STTN5105R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order
    pub async fn domestic_futureoption_v1_trading_inquire_psbl_ngt_order(
        &self,
        req: DomesticFutureoptionV1TradingInquirePsblNgtOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(구) JTCE1004R (신) STTN5105R",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// (야간)선물옵션 증거금 상세 [국내선물-024]
    /// - TR_ID: Real=(구) JTCE6003R (신) CTFN7107R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/ngt-margin-detail
    pub async fn domestic_futureoption_v1_trading_ngt_margin_detail(
        &self,
        req: DomesticFutureoptionV1TradingNgtMarginDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "(구) JTCE6003R (신) CTFN7107R",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/ngt-margin-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 잔고정산손익내역[v1_국내선물-013]
    /// - TR_ID: Real=CTFO6117R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl
    pub async fn domestic_futureoption_v1_trading_inquire_balance_settlement_pl(
        &self,
        req: DomesticFutureoptionV1TradingInquireBalanceSettlementPlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTFO6117R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 총자산현황[v1_국내선물-014]
    /// - TR_ID: Real=CTRP6550R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-deposit
    pub async fn domestic_futureoption_v1_trading_inquire_deposit(
        &self,
        req: DomesticFutureoptionV1TradingInquireDepositRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTRP6550R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-deposit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 잔고평가손익내역[v1_국내선물-015]
    /// - TR_ID: Real=CTFO6159R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl
    pub async fn domestic_futureoption_v1_trading_inquire_balance_valuation_pl(
        &self,
        req: DomesticFutureoptionV1TradingInquireBalanceValuationPlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTFO6159R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 기준일체결내역[v1_국내선물-016]
    /// - TR_ID: Real=CTFO5139R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime
    pub async fn domestic_futureoption_v1_trading_inquire_ccnl_bstime(
        &self,
        req: DomesticFutureoptionV1TradingInquireCcnlBstimeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTFO5139R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션기간약정수수료일별[v1_국내선물-017]
    /// - TR_ID: Real=CTFO6119R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee
    pub async fn domestic_futureoption_v1_trading_inquire_daily_amount_fee(
        &self,
        req: DomesticFutureoptionV1TradingInquireDailyAmountFeeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTFO6119R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl StockQuotations {
    /// 주식현재가 시세[v1_국내주식-008]
    /// - TR_ID: Real=FHKST01010100 / VTS=FHKST01010100
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-price
    pub async fn domestic_stock_v1_quotations_inquire_price(
        &self,
        req: DomesticStockV1QuotationsInquirePriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01010100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST01010100",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 시세2[v1_국내주식-054]
    /// - TR_ID: Real=FHPST01010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-price-2
    pub async fn domestic_stock_v1_quotations_inquire_price_2(
        &self,
        req: DomesticStockV1QuotationsInquirePrice2Request,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01010000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-price-2",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 체결[v1_국내주식-009]
    /// - TR_ID: Real=FHKST01010300 / VTS=FHKST01010300
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-ccnl
    pub async fn domestic_stock_v1_quotations_inquire_ccnl(
        &self,
        req: DomesticStockV1QuotationsInquireCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01010300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST01010300",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 일자별[v1_국내주식-010]
    /// - TR_ID: Real=FHKST01010400 / VTS=FHKST01010400
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-price
    pub async fn domestic_stock_v1_quotations_inquire_daily_price(
        &self,
        req: DomesticStockV1QuotationsInquireDailyPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01010400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST01010400",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 호가/예상체결[v1_국내주식-011]
    /// - TR_ID: Real=FHKST01010200 / VTS=FHKST01010200
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn
    pub async fn domestic_stock_v1_quotations_inquire_asking_price_exp_ccn(
        &self,
        req: DomesticStockV1QuotationsInquireAskingPriceExpCcnRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01010200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST01010200",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 투자자[v1_국내주식-012]
    /// - TR_ID: Real=FHKST01010900 / VTS=FHKST01010900
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor
    pub async fn domestic_stock_v1_quotations_inquire_investor(
        &self,
        req: DomesticStockV1QuotationsInquireInvestorRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01010900", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST01010900",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-investor",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 회원사[v1_국내주식-013]
    /// - TR_ID: Real=FHKST01010600 / VTS=FHKST01010600
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-member
    pub async fn domestic_stock_v1_quotations_inquire_member(
        &self,
        req: DomesticStockV1QuotationsInquireMemberRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01010600", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST01010600",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-member",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식기간별시세(일/주/월/년)[v1_국내주식-016]
    /// - TR_ID: Real=FHKST03010100 / VTS=FHKST03010100
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice
    pub async fn domestic_stock_v1_quotations_inquire_daily_itemchartprice(
        &self,
        req: DomesticStockV1QuotationsInquireDailyItemchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST03010100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST03010100",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식당일분봉조회[v1_국내주식-022]
    /// - TR_ID: Real=FHKST03010200 / VTS=FHKST03010200
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-itemchartprice
    pub async fn domestic_stock_v1_quotations_inquire_time_itemchartprice(
        &self,
        req: DomesticStockV1QuotationsInquireTimeItemchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST03010200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST03010200",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-itemchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식일별분봉조회 [국내주식-213]
    /// - TR_ID: Real=FHKST03010230 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice
    pub async fn domestic_stock_v1_quotations_inquire_time_dailychartprice(
        &self,
        req: DomesticStockV1QuotationsInquireTimeDailychartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST03010230", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 당일시간대별체결[v1_국내주식-023]
    /// - TR_ID: Real=FHPST01060000 / VTS=FHPST01060000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion
    pub async fn domestic_stock_v1_quotations_inquire_time_itemconclusion(
        &self,
        req: DomesticStockV1QuotationsInquireTimeItemconclusionRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01060000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHPST01060000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 시간외일자별주가[v1_국내주식-026]
    /// - TR_ID: Real=FHPST02320000 / VTS=FHPST02320000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice
    pub async fn domestic_stock_v1_quotations_inquire_daily_overtimeprice(
        &self,
        req: DomesticStockV1QuotationsInquireDailyOvertimepriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02320000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHPST02320000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 시간외시간별체결[v1_국내주식-025]
    /// - TR_ID: Real=FHPST02310000 / VTS=FHPST02310000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion
    pub async fn domestic_stock_v1_quotations_inquire_time_overtimeconclusion(
        &self,
        req: DomesticStockV1QuotationsInquireTimeOvertimeconclusionRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02310000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHPST02310000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시간외현재가[국내주식-076]
    /// - TR_ID: Real=FHPST02300000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-overtime-price
    pub async fn domestic_stock_v1_quotations_inquire_overtime_price(
        &self,
        req: DomesticStockV1QuotationsInquireOvertimePriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02300000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-overtime-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시간외호가[국내주식-077]
    /// - TR_ID: Real=FHPST02300400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-overtime-asking-price
    pub async fn domestic_stock_v1_quotations_inquire_overtime_asking_price(
        &self,
        req: DomesticStockV1QuotationsInquireOvertimeAskingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02300400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-overtime-asking-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 장마감 예상체결가[국내주식-120]
    /// - TR_ID: Real=FHKST117300C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-closing-price
    pub async fn domestic_stock_v1_quotations_exp_closing_price(
        &self,
        req: DomesticStockV1QuotationsExpClosingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST117300C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-closing-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 현재가 시세[v1_국내주식-014]
    /// - TR_ID: Real=FHKEW15010000 / VTS=FHKEW15010000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-elw-price
    pub async fn domestic_stock_v1_quotations_inquire_elw_price(
        &self,
        req: DomesticStockV1QuotationsInquireElwPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW15010000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKEW15010000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-elw-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내업종 현재지수[v1_국내주식-063]
    /// - TR_ID: Real=FHPUP02100000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-price
    pub async fn domestic_stock_v1_quotations_inquire_index_price(
        &self,
        req: DomesticStockV1QuotationsInquireIndexPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPUP02100000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내업종 일자별지수[v1_국내주식-065]
    /// - TR_ID: Real=FHPUP02120000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-daily-price
    pub async fn domestic_stock_v1_quotations_inquire_index_daily_price(
        &self,
        req: DomesticStockV1QuotationsInquireIndexDailyPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPUP02120000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-daily-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내업종 시간별지수(초)[국내주식-064]
    /// - TR_ID: Real=FHPUP02110100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-tickprice
    pub async fn domestic_stock_v1_quotations_inquire_index_tickprice(
        &self,
        req: DomesticStockV1QuotationsInquireIndexTickpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPUP02110100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-tickprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내업종 시간별지수(분)[국내주식-119]
    /// - TR_ID: Real=FHPUP02110200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-timeprice
    pub async fn domestic_stock_v1_quotations_inquire_index_timeprice(
        &self,
        req: DomesticStockV1QuotationsInquireIndexTimepriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPUP02110200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-timeprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 업종 분봉조회[v1_국내주식-045]
    /// - TR_ID: Real=FHKUP03500200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-indexchartprice
    pub async fn domestic_stock_v1_quotations_inquire_time_indexchartprice(
        &self,
        req: DomesticStockV1QuotationsInquireTimeIndexchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKUP03500200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-indexchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]
    /// - TR_ID: Real=FHKUP03500100 / VTS=FHKUP03500100
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice
    pub async fn domestic_stock_v1_quotations_inquire_daily_indexchartprice(
        &self,
        req: DomesticStockV1QuotationsInquireDailyIndexchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKUP03500100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKUP03500100",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내업종 구분별전체시세[v1_국내주식-066]
    /// - TR_ID: Real=FHPUP02140000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-category-price
    pub async fn domestic_stock_v1_quotations_inquire_index_category_price(
        &self,
        req: DomesticStockV1QuotationsInquireIndexCategoryPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPUP02140000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-category-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 예상체결지수 추이[국내주식-121]
    /// - TR_ID: Real=FHPST01840000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-index-trend
    pub async fn domestic_stock_v1_quotations_exp_index_trend(
        &self,
        req: DomesticStockV1QuotationsExpIndexTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01840000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-index-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 예상체결 전체지수[국내주식-122]
    /// - TR_ID: Real=FHKUP11750000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-total-index
    pub async fn domestic_stock_v1_quotations_exp_total_index(
        &self,
        req: DomesticStockV1QuotationsExpTotalIndexRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKUP11750000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-total-index",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 변동성완화장치(VI) 현황 [v1_국내주식-055]
    /// - TR_ID: Real=FHPST01390000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-vi-status
    pub async fn domestic_stock_v1_quotations_inquire_vi_status(
        &self,
        req: DomesticStockV1QuotationsInquireViStatusRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01390000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-vi-status",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 금리 종합(국내채권/금리) [국내주식-155]
    /// - TR_ID: Real=FHPST07020000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/comp-interest
    pub async fn domestic_stock_v1_quotations_comp_interest(
        &self,
        req: DomesticStockV1QuotationsCompInterestRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST07020000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/comp-interest",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종합 시황/공시(제목) [국내주식-141]
    /// - TR_ID: Real=FHKST01011800 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/news-title
    pub async fn domestic_stock_v1_quotations_news_title(
        &self,
        req: DomesticStockV1QuotationsNewsTitleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01011800", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/news-title",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내휴장일조회[국내주식-040]
    /// - TR_ID: Real=CTCA0903R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/chk-holiday
    pub async fn domestic_stock_v1_quotations_chk_holiday(
        &self,
        req: DomesticStockV1QuotationsChkHolidayRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTCA0903R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/chk-holiday",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내선물 영업일조회 [국내주식-160]
    /// - TR_ID: Real=HHMCM000002C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/market-time
    pub async fn domestic_stock_v1_quotations_market_time(
        &self,
        req: DomesticStockV1QuotationsMarketTimeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHMCM000002C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/market-time",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 상품기본조회[v1_국내주식-029]
    /// - TR_ID: Real=CTPF1604R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/search-info
    pub async fn domestic_stock_v1_quotations_search_info(
        &self,
        req: DomesticStockV1QuotationsSearchInfoRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTPF1604R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/search-info",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식기본조회[v1_국내주식-067]
    /// - TR_ID: Real=CTPF1002R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/search-stock-info
    pub async fn domestic_stock_v1_quotations_search_stock_info(
        &self,
        req: DomesticStockV1QuotationsSearchStockInfoRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTPF1002R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/search-stock-info",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 당사 신용가능종목[국내주식-111]
    /// - TR_ID: Real=FHPST04770000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/credit-by-company
    pub async fn domestic_stock_v1_quotations_credit_by_company(
        &self,
        req: DomesticStockV1QuotationsCreditByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST04770000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/credit-by-company",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 종목추정실적 [국내주식-187]
    /// - TR_ID: Real=HHKST668300C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/estimate-perform
    pub async fn domestic_stock_v1_quotations_estimate_perform(
        &self,
        req: DomesticStockV1QuotationsEstimatePerformRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKST668300C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/estimate-perform",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 당사 대주가능 종목 [국내주식-195]
    /// - TR_ID: Real=CTSC2702R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/lendable-by-company
    pub async fn domestic_stock_v1_quotations_lendable_by_company(
        &self,
        req: DomesticStockV1QuotationsLendableByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTSC2702R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/lendable-by-company",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 종목투자의견 [국내주식-188]
    /// - TR_ID: Real=FHKST663300C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/invest-opinion
    pub async fn domestic_stock_v1_quotations_invest_opinion(
        &self,
        req: DomesticStockV1QuotationsInvestOpinionRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST663300C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/invest-opinion",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 증권사별 투자의견 [국내주식-189]
    /// - TR_ID: Real=FHKST663400C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/invest-opbysec
    pub async fn domestic_stock_v1_quotations_invest_opbysec(
        &self,
        req: DomesticStockV1QuotationsInvestOpbysecRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST663400C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/invest-opbysec",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목조건검색 목록조회[국내주식-038]
    /// - TR_ID: Real=HHKST03900300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/psearch-title
    pub async fn domestic_stock_v1_quotations_psearch_title(
        &self,
        req: DomesticStockV1QuotationsPsearchTitleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKST03900300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/psearch-title",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목조건검색조회 [국내주식-039]
    /// - TR_ID: Real=HHKST03900400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/psearch-result
    pub async fn domestic_stock_v1_quotations_psearch_result(
        &self,
        req: DomesticStockV1QuotationsPsearchResultRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKST03900400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/psearch-result",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 관심종목 그룹조회 [국내주식-204]
    /// - TR_ID: Real=HHKCM113004C7 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/intstock-grouplist
    pub async fn domestic_stock_v1_quotations_intstock_grouplist(
        &self,
        req: DomesticStockV1QuotationsIntstockGrouplistRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKCM113004C7", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/intstock-grouplist",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 관심종목(멀티종목) 시세조회 [국내주식-205]
    /// - TR_ID: Real=FHKST11300006 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/intstock-multprice
    pub async fn domestic_stock_v1_quotations_intstock_multprice(
        &self,
        req: DomesticStockV1QuotationsIntstockMultpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST11300006", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/intstock-multprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 관심종목 그룹별 종목조회 [국내주식-203]
    /// - TR_ID: Real=HHKCM113004C6 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group
    pub async fn domestic_stock_v1_quotations_intstock_stocklist_by_group(
        &self,
        req: DomesticStockV1QuotationsIntstockStocklistByGroupRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKCM113004C6", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내기관_외국인 매매종목가집계[국내주식-037]
    /// - TR_ID: Real=FHPTJ04400000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/foreign-institution-total
    pub async fn domestic_stock_v1_quotations_foreign_institution_total(
        &self,
        req: DomesticStockV1QuotationsForeignInstitutionTotalRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPTJ04400000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/foreign-institution-total",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 외국계 매매종목 가집계 [국내주식-161]
    /// - TR_ID: Real=FHKST644100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-trade-estimate
    pub async fn domestic_stock_v1_quotations_frgnmem_trade_estimate(
        &self,
        req: DomesticStockV1QuotationsFrgnmemTradeEstimateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST644100C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/frgnmem-trade-estimate",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별 투자자매매동향(일별)
    /// - TR_ID: Real=FHPTJ04160001 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily
    pub async fn domestic_stock_v1_quotations_investor_trade_by_stock_daily(
        &self,
        req: DomesticStockV1QuotationsInvestorTradeByStockDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPTJ04160001", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 시장별 투자자매매동향(시세)[v1_국내주식-074]
    /// - TR_ID: Real=FHPTJ04030000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market
    pub async fn domestic_stock_v1_quotations_inquire_investor_time_by_market(
        &self,
        req: DomesticStockV1QuotationsInquireInvestorTimeByMarketRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPTJ04030000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 시장별 투자자매매동향(일별) [국내주식-075]
    /// - TR_ID: Real=FHPTJ04040000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market
    pub async fn domestic_stock_v1_quotations_inquire_investor_daily_by_market(
        &self,
        req: DomesticStockV1QuotationsInquireInvestorDailyByMarketRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPTJ04040000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별 외국계 순매수추이 [국내주식-164]
    /// - TR_ID: Real=FHKST644400C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-pchs-trend
    pub async fn domestic_stock_v1_quotations_frgnmem_pchs_trend(
        &self,
        req: DomesticStockV1QuotationsFrgnmemPchsTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST644400C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/frgnmem-pchs-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 회원사 실시간 매매동향(틱) [국내주식-163]
    /// - TR_ID: Real=FHPST04320000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-trade-trend
    pub async fn domestic_stock_v1_quotations_frgnmem_trade_trend(
        &self,
        req: DomesticStockV1QuotationsFrgnmemTradeTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST04320000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/frgnmem-trade-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 주식현재가 회원사 종목매매동향 [국내주식-197]
    /// - TR_ID: Real=FHPST04540000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-member-daily
    pub async fn domestic_stock_v1_quotations_inquire_member_daily(
        &self,
        req: DomesticStockV1QuotationsInquireMemberDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST04540000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-member-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별 프로그램매매추이(체결)[v1_국내주식-044]
    /// - TR_ID: Real=FHPPG04650101 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/program-trade-by-stock
    pub async fn domestic_stock_v1_quotations_program_trade_by_stock(
        &self,
        req: DomesticStockV1QuotationsProgramTradeByStockRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPPG04650101", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/program-trade-by-stock",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별 프로그램매매추이(일별) [국내주식-113]
    /// - TR_ID: Real=FHPPG04650201 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily
    pub async fn domestic_stock_v1_quotations_program_trade_by_stock_daily(
        &self,
        req: DomesticStockV1QuotationsProgramTradeByStockDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPPG04650201", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별 외인기관 추정가집계[v1_국내주식-046]
    /// - TR_ID: Real=HHPTJ04160200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/investor-trend-estimate
    pub async fn domestic_stock_v1_quotations_investor_trend_estimate(
        &self,
        req: DomesticStockV1QuotationsInvestorTrendEstimateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHPTJ04160200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/investor-trend-estimate",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별일별매수매도체결량 [v1_국내주식-056]
    /// - TR_ID: Real=FHKST03010800 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-trade-volume
    pub async fn domestic_stock_v1_quotations_inquire_daily_trade_volume(
        &self,
        req: DomesticStockV1QuotationsInquireDailyTradeVolumeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST03010800", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-trade-volume",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 프로그램매매 종합현황(시간) [국내주식-114]
    /// - TR_ID: Real=FHPPG04600101 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/comp-program-trade-today
    pub async fn domestic_stock_v1_quotations_comp_program_trade_today(
        &self,
        req: DomesticStockV1QuotationsCompProgramTradeTodayRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPPG04600101", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/comp-program-trade-today",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 프로그램매매 종합현황(일별)[국내주식-115]
    /// - TR_ID: Real=FHPPG04600001 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/comp-program-trade-daily
    pub async fn domestic_stock_v1_quotations_comp_program_trade_daily(
        &self,
        req: DomesticStockV1QuotationsCompProgramTradeDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPPG04600001", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/comp-program-trade-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 프로그램매매 투자자매매동향(당일) [국내주식-116]
    /// - TR_ID: Real=HHPPG046600C1 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/investor-program-trade-today
    pub async fn domestic_stock_v1_quotations_investor_program_trade_today(
        &self,
        req: DomesticStockV1QuotationsInvestorProgramTradeTodayRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHPPG046600C1", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/investor-program-trade-today",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 신용잔고 일별추이[국내주식-110]
    /// - TR_ID: Real=FHPST04760000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/daily-credit-balance
    pub async fn domestic_stock_v1_quotations_daily_credit_balance(
        &self,
        req: DomesticStockV1QuotationsDailyCreditBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST04760000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/daily-credit-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 예상체결가 추이[국내주식-118]
    /// - TR_ID: Real=FHPST01810000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-price-trend
    pub async fn domestic_stock_v1_quotations_exp_price_trend(
        &self,
        req: DomesticStockV1QuotationsExpPriceTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01810000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-price-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 공매도 일별추이[국내주식-134]
    /// - TR_ID: Real=FHPST04830000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/daily-short-sale
    pub async fn domestic_stock_v1_quotations_daily_short_sale(
        &self,
        req: DomesticStockV1QuotationsDailyShortSaleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST04830000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/daily-short-sale",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 체결금액별 매매비중 [국내주식-192]
    /// - TR_ID: Real=FHKST111900C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/tradprt-byamt
    pub async fn domestic_stock_v1_quotations_tradprt_byamt(
        &self,
        req: DomesticStockV1QuotationsTradprtByamtRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST111900C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/tradprt-byamt",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내 증시자금 종합 [국내주식-193]
    /// - TR_ID: Real=FHKST649100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/mktfunds
    pub async fn domestic_stock_v1_quotations_mktfunds(
        &self,
        req: DomesticStockV1QuotationsMktfundsRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST649100C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/mktfunds",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 종목별 일별 대차거래추이 [국내주식-135]
    /// - TR_ID: Real=HHPST074500C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/daily-loan-trans
    pub async fn domestic_stock_v1_quotations_daily_loan_trans(
        &self,
        req: DomesticStockV1QuotationsDailyLoanTransRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHPST074500C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/daily-loan-trans",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 상하한가 포착 [국내주식-190]
    /// - TR_ID: Real=FHKST130000C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/capture-uplowprice
    pub async fn domestic_stock_v1_quotations_capture_uplowprice(
        &self,
        req: DomesticStockV1QuotationsCaptureUplowpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST130000C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/capture-uplowprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 매물대/거래비중 [국내주식-196]
    /// - TR_ID: Real=FHPST01130000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/pbar-tratio
    pub async fn domestic_stock_v1_quotations_pbar_tratio(
        &self,
        req: DomesticStockV1QuotationsPbarTratioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01130000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/pbar-tratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 거래량순위[v1_국내주식-047]
    /// - TR_ID: Real=FHPST01710000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/volume-rank
    pub async fn domestic_stock_v1_quotations_volume_rank(
        &self,
        req: DomesticStockV1QuotationsVolumeRankRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01710000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/volume-rank",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 증거금률
    /// - TR_ID: Real=TTTO6032R / VTS=미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/margin-rate
    pub async fn domestic_futureoption_v1_quotations_margin_rate(
        &self,
        req: DomesticFutureoptionV1QuotationsMarginRateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTO6032R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/margin-rate",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 시세[v1_국내선물-006]
    /// - TR_ID: Real=FHMIF10000000 / VTS=FHMIF10000000
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-price
    pub async fn domestic_futureoption_v1_quotations_inquire_price(
        &self,
        req: DomesticFutureoptionV1QuotationsInquirePriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHMIF10000000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHMIF10000000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 시세호가[v1_국내선물-007]
    /// - TR_ID: Real=FHMIF10010000 / VTS=FHMIF10010000
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-asking-price
    pub async fn domestic_futureoption_v1_quotations_inquire_asking_price(
        &self,
        req: DomesticFutureoptionV1QuotationsInquireAskingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHMIF10010000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHMIF10010000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-asking-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]
    /// - TR_ID: Real=FHKIF03020100 / VTS=FHKIF03020100
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice
    pub async fn domestic_futureoption_v1_quotations_inquire_daily_fuopchartprice(
        &self,
        req: DomesticFutureoptionV1QuotationsInquireDailyFuopchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKIF03020100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKIF03020100",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 분봉조회[v1_국내선물-012]
    /// - TR_ID: Real=FHKIF03020200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice
    pub async fn domestic_futureoption_v1_quotations_inquire_time_fuopchartprice(
        &self,
        req: DomesticFutureoptionV1QuotationsInquireTimeFuopchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKIF03020200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내옵션전광판_옵션월물리스트[국내선물-020]
    /// - TR_ID: Real=FHPIO056104C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-option-list
    pub async fn domestic_futureoption_v1_quotations_display_board_option_list(
        &self,
        req: DomesticFutureoptionV1QuotationsDisplayBoardOptionListRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPIO056104C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-option-list",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내선물 기초자산 시세[국내선물-021]
    /// - TR_ID: Real=FHPIF05030000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-top
    pub async fn domestic_futureoption_v1_quotations_display_board_top(
        &self,
        req: DomesticFutureoptionV1QuotationsDisplayBoardTopRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPIF05030000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-top",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내옵션전광판_콜풋[국내선물-022]
    /// - TR_ID: Real=FHPIF05030100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-callput
    pub async fn domestic_futureoption_v1_quotations_display_board_callput(
        &self,
        req: DomesticFutureoptionV1QuotationsDisplayBoardCallputRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPIF05030100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-callput",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내옵션전광판_선물[국내선물-023]
    /// - TR_ID: Real=FHPIF05030200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-futures
    pub async fn domestic_futureoption_v1_quotations_display_board_futures(
        &self,
        req: DomesticFutureoptionV1QuotationsDisplayBoardFuturesRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPIF05030200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-futures",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 선물옵션 일중예상체결추이[국내선물-018]
    /// - TR_ID: Real=FHPIF05110100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/exp-price-trend
    pub async fn domestic_futureoption_v1_quotations_exp_price_trend(
        &self,
        req: DomesticFutureoptionV1QuotationsExpPriceTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPIF05110100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/exp-price-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl StockCommon {
    /// 국내주식 대차대조표[v1_국내주식-078]
    /// - TR_ID: Real=FHKST66430100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/balance-sheet
    pub async fn domestic_stock_v1_finance_balance_sheet(
        &self,
        req: DomesticStockV1FinanceBalanceSheetRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/balance-sheet",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 손익계산서[v1_국내주식-079]
    /// - TR_ID: Real=FHKST66430200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/income-statement
    pub async fn domestic_stock_v1_finance_income_statement(
        &self,
        req: DomesticStockV1FinanceIncomeStatementRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", ""),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/income-statement",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 재무비율[v1_국내주식-080]
    /// - TR_ID: Real=FHKST66430300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/financial-ratio
    pub async fn domestic_stock_v1_finance_financial_ratio(
        &self,
        req: DomesticStockV1FinanceFinancialRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/financial-ratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 수익성비율[v1_국내주식-081]
    /// - TR_ID: Real=FHKST66430400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/profit-ratio
    pub async fn domestic_stock_v1_finance_profit_ratio(
        &self,
        req: DomesticStockV1FinanceProfitRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/profit-ratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 기타주요비율[v1_국내주식-082]
    /// - TR_ID: Real=FHKST66430500 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/other-major-ratios
    pub async fn domestic_stock_v1_finance_other_major_ratios(
        &self,
        req: DomesticStockV1FinanceOtherMajorRatiosRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430500", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/other-major-ratios",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 안정성비율[v1_국내주식-083]
    /// - TR_ID: Real=FHKST66430600 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/stability-ratio
    pub async fn domestic_stock_v1_finance_stability_ratio(
        &self,
        req: DomesticStockV1FinanceStabilityRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430600", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/stability-ratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 성장성비율[v1_국내주식-085]
    /// - TR_ID: Real=FHKST66430800 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/growth-ratio
    pub async fn domestic_stock_v1_finance_growth_ratio(
        &self,
        req: DomesticStockV1FinanceGrowthRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST66430800", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/growth-ratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(배당일정)[국내주식-145]
    /// - TR_ID: Real=HHKDB669102C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/dividend
    pub async fn domestic_stock_v1_ksdinfo_dividend(
        &self,
        req: DomesticStockV1KsdinfoDividendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669102C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/dividend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(주식매수청구일정)[국내주식-146]
    /// - TR_ID: Real=HHKDB669103C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/purreq
    pub async fn domestic_stock_v1_ksdinfo_purreq(
        &self,
        req: DomesticStockV1KsdinfoPurreqRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669103C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/purreq",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(합병/분할일정)[국내주식-147]
    /// - TR_ID: Real=HHKDB669104C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/merger-split
    pub async fn domestic_stock_v1_ksdinfo_merger_split(
        &self,
        req: DomesticStockV1KsdinfoMergerSplitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669104C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/merger-split",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(액면교체일정)[국내주식-148]
    /// - TR_ID: Real=HHKDB669105C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/rev-split
    pub async fn domestic_stock_v1_ksdinfo_rev_split(
        &self,
        req: DomesticStockV1KsdinfoRevSplitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669105C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/rev-split",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(자본감소일정)[국내주식-149]
    /// - TR_ID: Real=HHKDB669106C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/cap-dcrs
    pub async fn domestic_stock_v1_ksdinfo_cap_dcrs(
        &self,
        req: DomesticStockV1KsdinfoCapDcrsRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669106C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/cap-dcrs",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(상장정보일정)[국내주식-150]
    /// - TR_ID: Real=HHKDB669107C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/list-info
    pub async fn domestic_stock_v1_ksdinfo_list_info(
        &self,
        req: DomesticStockV1KsdinfoListInfoRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669107C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/list-info",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(공모주청약일정)[국내주식-151]
    /// - TR_ID: Real=HHKDB669108C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/pub-offer
    pub async fn domestic_stock_v1_ksdinfo_pub_offer(
        &self,
        req: DomesticStockV1KsdinfoPubOfferRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669108C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/pub-offer",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(실권주일정)[국내주식-152]
    /// - TR_ID: Real=HHKDB669109C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/forfeit
    pub async fn domestic_stock_v1_ksdinfo_forfeit(
        &self,
        req: DomesticStockV1KsdinfoForfeitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669109C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/forfeit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(의무예치일정)[국내주식-153]
    /// - TR_ID: Real=HHKDB669110C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/mand-deposit
    pub async fn domestic_stock_v1_ksdinfo_mand_deposit(
        &self,
        req: DomesticStockV1KsdinfoMandDepositRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669110C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/mand-deposit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(유상증자일정) [국내주식-143]
    /// - TR_ID: Real=HHKDB669100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/paidin-capin
    pub async fn domestic_stock_v1_ksdinfo_paidin_capin(
        &self,
        req: DomesticStockV1KsdinfoPaidinCapinRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669100C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/paidin-capin",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(무상증자일정) [국내주식-144]
    /// - TR_ID: Real=HHKDB669101C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/bonus-issue
    pub async fn domestic_stock_v1_ksdinfo_bonus_issue(
        &self,
        req: DomesticStockV1KsdinfoBonusIssueRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669101C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/bonus-issue",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 예탁원정보(주주총회일정) [국내주식-154]
    /// - TR_ID: Real=HHKDB669111C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/sharehld-meet
    pub async fn domestic_stock_v1_ksdinfo_sharehld_meet(
        &self,
        req: DomesticStockV1KsdinfoSharehldMeetRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB669111C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ksdinfo/sharehld-meet",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl StockRanking {
    /// 국내주식 시간외예상체결등락률 [국내주식-140]
    /// - TR_ID: Real=FHKST11860000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/overtime-exp-trans-fluct
    pub async fn domestic_stock_v1_ranking_overtime_exp_trans_fluct(
        &self,
        req: DomesticStockV1RankingOvertimeExpTransFluctRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST11860000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/overtime-exp-trans-fluct",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 등락률 순위[v1_국내주식-088]
    /// - TR_ID: Real=FHPST01700000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/fluctuation
    pub async fn domestic_stock_v1_ranking_fluctuation(
        &self,
        req: DomesticStockV1RankingFluctuationRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01700000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/fluctuation",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 호가잔량 순위[국내주식-089]
    /// - TR_ID: Real=FHPST01720000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/quote-balance
    pub async fn domestic_stock_v1_ranking_quote_balance(
        &self,
        req: DomesticStockV1RankingQuoteBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01720000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/quote-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 수익자산지표 순위[v1_국내주식-090]
    /// - TR_ID: Real=FHPST01730000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/profit-asset-index
    pub async fn domestic_stock_v1_ranking_profit_asset_index(
        &self,
        req: DomesticStockV1RankingProfitAssetIndexRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01730000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/profit-asset-index",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시가총액 상위[v1_국내주식-091]
    /// - TR_ID: Real=FHPST01740000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/market-cap
    pub async fn domestic_stock_v1_ranking_market_cap(
        &self,
        req: DomesticStockV1RankingMarketCapRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01740000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/market-cap",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 재무비율 순위[v1_국내주식-092]
    /// - TR_ID: Real=FHPST01750000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/finance-ratio
    pub async fn domestic_stock_v1_ranking_finance_ratio(
        &self,
        req: DomesticStockV1RankingFinanceRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01750000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/finance-ratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시간외잔량 순위[v1_국내주식-093]
    /// - TR_ID: Real=FHPST01760000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/after-hour-balance
    pub async fn domestic_stock_v1_ranking_after_hour_balance(
        &self,
        req: DomesticStockV1RankingAfterHourBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01760000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/after-hour-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 우선주/괴리율 상위[v1_국내주식-094]
    /// - TR_ID: Real=FHPST01770000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/prefer-disparate-ratio
    pub async fn domestic_stock_v1_ranking_prefer_disparate_ratio(
        &self,
        req: DomesticStockV1RankingPreferDisparateRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01770000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/prefer-disparate-ratio",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 이격도 순위[v1_국내주식-095]
    /// - TR_ID: Real=FHPST01780000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/disparity
    pub async fn domestic_stock_v1_ranking_disparity(
        &self,
        req: DomesticStockV1RankingDisparityRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01780000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/disparity",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시장가치 순위[v1_국내주식-096]
    /// - TR_ID: Real=FHPST01790000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/market-value
    pub async fn domestic_stock_v1_ranking_market_value(
        &self,
        req: DomesticStockV1RankingMarketValueRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01790000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/market-value",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 체결강도 상위[v1_국내주식-101]
    /// - TR_ID: Real=FHPST01680000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/volume-power
    pub async fn domestic_stock_v1_ranking_volume_power(
        &self,
        req: DomesticStockV1RankingVolumePowerRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01680000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/volume-power",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 관심종목등록 상위[v1_국내주식-102]
    /// - TR_ID: Real=FHPST01800000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/top-interest-stock
    pub async fn domestic_stock_v1_ranking_top_interest_stock(
        &self,
        req: DomesticStockV1RankingTopInterestStockRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01800000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/top-interest-stock",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 예상체결 상승/하락상위[v1_국내주식-103]
    /// - TR_ID: Real=FHPST01820000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/exp-trans-updown
    pub async fn domestic_stock_v1_ranking_exp_trans_updown(
        &self,
        req: DomesticStockV1RankingExpTransUpdownRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01820000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/exp-trans-updown",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 당사매매종목 상위[v1_국내주식-104]
    /// - TR_ID: Real=FHPST01860000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/traded-by-company
    pub async fn domestic_stock_v1_ranking_traded_by_company(
        &self,
        req: DomesticStockV1RankingTradedByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01860000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/traded-by-company",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 신고/신저근접종목 상위[v1_국내주식-105]
    /// - TR_ID: Real=FHPST01870000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/near-new-highlow
    pub async fn domestic_stock_v1_ranking_near_new_highlow(
        &self,
        req: DomesticStockV1RankingNearNewHighlowRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST01870000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/near-new-highlow",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 배당률 상위[국내주식-106]
    /// - TR_ID: Real=HHKDB13470100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/dividend-rate
    pub async fn domestic_stock_v1_ranking_dividend_rate(
        &self,
        req: DomesticStockV1RankingDividendRateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHKDB13470100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/dividend-rate",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 대량체결건수 상위[국내주식-107]
    /// - TR_ID: Real=FHKST190900C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/bulk-trans-num
    pub async fn domestic_stock_v1_ranking_bulk_trans_num(
        &self,
        req: DomesticStockV1RankingBulkTransNumRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST190900C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/bulk-trans-num",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 신용잔고 상위[국내주식-109]
    /// - TR_ID: Real=FHKST17010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/credit-balance
    pub async fn domestic_stock_v1_ranking_credit_balance(
        &self,
        req: DomesticStockV1RankingCreditBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST17010000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/credit-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 공매도 상위종목[국내주식-133]
    /// - TR_ID: Real=FHPST04820000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/short-sale
    pub async fn domestic_stock_v1_ranking_short_sale(
        &self,
        req: DomesticStockV1RankingShortSaleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST04820000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/short-sale",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시간외등락율순위 [국내주식-138]
    /// - TR_ID: Real=FHPST02340000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/overtime-fluctuation
    pub async fn domestic_stock_v1_ranking_overtime_fluctuation(
        &self,
        req: DomesticStockV1RankingOvertimeFluctuationRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02340000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/overtime-fluctuation",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 국내주식 시간외거래량순위 [국내주식-139]
    /// - TR_ID: Real=FHPST02350000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/overtime-volume
    pub async fn domestic_stock_v1_ranking_overtime_volume(
        &self,
        req: DomesticStockV1RankingOvertimeVolumeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02350000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/overtime-volume",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// HTS조회상위20종목 [국내주식-214]
    /// - TR_ID: Real=HHMCM000100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/hts-top-view
    pub async fn domestic_stock_v1_ranking_hts_top_view(
        &self,
        req: DomesticStockV1RankingHtsTopViewRequest,
    ) -> Result<serde_json::Value, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHMCM000100C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/hts-top-view",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}
