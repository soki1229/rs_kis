#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
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
    pub fn common(&self) -> OverseasCommon {
        OverseasCommon(self.0.clone())
    }
    pub fn trading(&self) -> OverseasTrading {
        OverseasTrading(self.0.clone())
    }
    pub fn quotations(&self) -> OverseasQuotations {
        OverseasQuotations(self.0.clone())
    }
    pub fn ranking(&self) -> OverseasRanking {
        OverseasRanking(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl OverseasCommon {
    /// 접근토큰발급(P)[인증-001]
    /// - TR_ID: Real= / VTS=모의투자 미지원
    /// - Endpoint: /oauth2/tokenP
    pub async fn oauth2_token_p(
        &self,
        req: Oauth2TokenpRequest,
    ) -> Result<Oauth2TokenpResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => ("", "https://openapi.koreainvestment.com:9443"),
            crate::client::KisEnv::Vts => (
                "모의투자 미지원",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0.post("/oauth2/tokenP", tr_id, base_url, req).await
    }

    /// 접근토큰폐기(P)[인증-002]
    /// - TR_ID: Real= / VTS=모의투자 미지원
    /// - Endpoint: /oauth2/revokeP
    pub async fn oauth2_revoke_p(
        &self,
        req: Oauth2RevokepRequest,
    ) -> Result<Oauth2RevokepResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => ("", "https://openapi.koreainvestment.com:9443"),
            crate::client::KisEnv::Vts => (
                "모의투자 미지원",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0.post("/oauth2/revokeP", tr_id, base_url, req).await
    }

    /// Hashkey
    /// - TR_ID: Real= / VTS=모의투자 미지원
    /// - Endpoint: /uapi/hashkey
    pub async fn hashkey(&self, req: HashkeyRequest) -> Result<HashkeyResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => ("", "https://openapi.koreainvestment.com:9443"),
            crate::client::KisEnv::Vts => (
                "모의투자 미지원",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0.post("/uapi/hashkey", tr_id, base_url, req).await
    }

    /// 실시간 (웹소켓) 접속키 발급[실시간-000]
    /// - TR_ID: Real= / VTS=모의투자 미지원
    /// - Endpoint: /oauth2/Approval
    pub async fn oauth2_approval(
        &self,
        req: Oauth2ApprovalRequest,
    ) -> Result<Oauth2ApprovalResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => ("", "https://openapi.koreainvestment.com:9443"),
            crate::client::KisEnv::Vts => (
                "모의투자 미지원",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0.post("/oauth2/Approval", tr_id, base_url, req).await
    }
}

#[allow(non_snake_case)]
impl OverseasTrading {
    /// 해외주식 주문[v1_해외주식-001]
    /// - TR_ID: Real=TTTT1002U / VTS=VTTT1002U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order
    pub async fn overseas_stock_v1_trading_order_buy(
        &self,
        req: OverseasStockV1TradingOrderRequest,
    ) -> Result<OverseasStockV1TradingOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTT1002U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTT1002U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 주문[v1_해외주식-001]
    /// - TR_ID: Real=TTTT1006U / VTS=VTTT1001U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order
    pub async fn overseas_stock_v1_trading_order_sell(
        &self,
        req: OverseasStockV1TradingOrderRequest,
    ) -> Result<OverseasStockV1TradingOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTT1006U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTT1001U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 정정취소주문[v1_해외주식-003]
    /// - TR_ID: Real=TTTT1004U / VTS=VTTT1004U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-rvsecncl
    pub async fn overseas_stock_v1_trading_order_rvsecncl(
        &self,
        req: OverseasStockV1TradingOrderRvsecnclRequest,
    ) -> Result<OverseasStockV1TradingOrderRvsecnclResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTT1004U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTT1004U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 예약주문접수[v1_해외주식-002]
    /// - TR_ID: Real=TTTT3014U / VTS=VTTT3014U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv
    pub async fn overseas_stock_v1_trading_order_resv_us_buy_resv(
        &self,
        req: OverseasStockV1TradingOrderResvRequest,
    ) -> Result<OverseasStockV1TradingOrderResvResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTT3014U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTT3014U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order-resv",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 예약주문접수[v1_해외주식-002]
    /// - TR_ID: Real=TTTT3016U / VTS=VTTT3016U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv
    pub async fn overseas_stock_v1_trading_order_resv_us_sell_resv(
        &self,
        req: OverseasStockV1TradingOrderResvRequest,
    ) -> Result<OverseasStockV1TradingOrderResvResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTT3016U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTT3016U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order-resv",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 예약주문접수[v1_해외주식-002]
    /// - TR_ID: Real=TTTS3013U / VTS=VTTS3013U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv
    pub async fn overseas_stock_v1_trading_order_resv_asia_resv(
        &self,
        req: OverseasStockV1TradingOrderResvRequest,
    ) -> Result<OverseasStockV1TradingOrderResvResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3013U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTS3013U", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order-resv",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 예약주문접수취소[v1_해외주식-004]
    /// - TR_ID: Real=모의투자 미지원 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-ccnl
    pub async fn overseas_stock_v1_trading_order_resv_ccnl(
        &self,
        req: OverseasStockV1TradingOrderResvCcnlRequest,
    ) -> Result<OverseasStockV1TradingOrderResvCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => (
                "모의투자 미지원",
                "https://openapi.koreainvestment.com:9443",
            ),
            crate::client::KisEnv::Vts => (
                "모의투자 미지원",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order-resv-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 매수가능금액조회[v1_해외주식-014]
    /// - TR_ID: Real=TTTS3007R / VTS=VTTS3007R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-psamount
    pub async fn overseas_stock_v1_trading_inquire_psamount(
        &self,
        req: OverseasStockV1TradingInquirePsamountRequest,
    ) -> Result<OverseasStockV1TradingInquirePsamountResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3007R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTS3007R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-psamount",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 미체결내역[v1_해외주식-005]
    /// - TR_ID: Real=TTTS3018R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-nccs
    pub async fn overseas_stock_v1_trading_inquire_nccs(
        &self,
        req: OverseasStockV1TradingInquireNccsRequest,
    ) -> Result<OverseasStockV1TradingInquireNccsResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3018R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-nccs",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 잔고[v1_해외주식-006]
    /// - TR_ID: Real=TTTS3012R / VTS=VTTS3012R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-balance
    pub async fn overseas_stock_v1_trading_inquire_balance(
        &self,
        req: OverseasStockV1TradingInquireBalanceRequest,
    ) -> Result<OverseasStockV1TradingInquireBalanceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3012R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTS3012R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 주문체결내역[v1_해외주식-007]
    /// - TR_ID: Real=TTTS3035R / VTS=VTTS3035R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-ccnl
    pub async fn overseas_stock_v1_trading_inquire_ccnl(
        &self,
        req: OverseasStockV1TradingInquireCcnlRequest,
    ) -> Result<OverseasStockV1TradingInquireCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3035R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTTS3035R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 체결기준현재잔고[v1_해외주식-008]
    /// - TR_ID: Real=CTRP6504R / VTS=VTRP6504R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-present-balance
    pub async fn overseas_stock_v1_trading_inquire_present_balance(
        &self,
        req: OverseasStockV1TradingInquirePresentBalanceRequest,
    ) -> Result<OverseasStockV1TradingInquirePresentBalanceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTRP6504R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => {
                ("VTRP6504R", "https://openapivts.koreainvestment.com:29443")
            }
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-present-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 예약주문조회[v1_해외주식-013]
    /// - TR_ID: Real=TTTT3039R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-list
    pub async fn overseas_stock_v1_trading_order_resv_list_us(
        &self,
        req: OverseasStockV1TradingOrderResvListRequest,
    ) -> Result<OverseasStockV1TradingOrderResvListResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTT3039R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/order-resv-list",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 예약주문조회[v1_해외주식-013]
    /// - TR_ID: Real=TTTS3014R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-list
    pub async fn overseas_stock_v1_trading_order_resv_list_asia(
        &self,
        req: OverseasStockV1TradingOrderResvListRequest,
    ) -> Result<OverseasStockV1TradingOrderResvListResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3014R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/order-resv-list",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 결제기준잔고 [해외주식-064]
    /// - TR_ID: Real=CTRP6010R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance
    pub async fn overseas_stock_v1_trading_inquire_paymt_stdr_balance(
        &self,
        req: OverseasStockV1TradingInquirePaymtStdrBalanceRequest,
    ) -> Result<OverseasStockV1TradingInquirePaymtStdrBalanceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTRP6010R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 일별거래내역 [해외주식-063]
    /// - TR_ID: Real=CTOS4001R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-period-trans
    pub async fn overseas_stock_v1_trading_inquire_period_trans(
        &self,
        req: OverseasStockV1TradingInquirePeriodTransRequest,
    ) -> Result<OverseasStockV1TradingInquirePeriodTransResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTOS4001R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-period-trans",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 기간손익[v1_해외주식-032]
    /// - TR_ID: Real=TTTS3039R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-period-profit
    pub async fn overseas_stock_v1_trading_inquire_period_profit(
        &self,
        req: OverseasStockV1TradingInquirePeriodProfitRequest,
    ) -> Result<OverseasStockV1TradingInquirePeriodProfitResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS3039R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-period-profit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외증거금 통화별조회 [해외주식-035]
    /// - TR_ID: Real=TTTC2101R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/foreign-margin
    pub async fn overseas_stock_v1_trading_foreign_margin(
        &self,
        req: OverseasStockV1TradingForeignMarginRequest,
    ) -> Result<OverseasStockV1TradingForeignMarginResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC2101R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/foreign-margin",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 미국주간주문[v1_해외주식-026]
    /// - TR_ID: Real=TTTS6036U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order
    pub async fn overseas_stock_v1_trading_daytime_order_buy(
        &self,
        req: OverseasStockV1TradingDaytimeOrderRequest,
    ) -> Result<OverseasStockV1TradingDaytimeOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS6036U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/daytime-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 미국주간주문[v1_해외주식-026]
    /// - TR_ID: Real=TTTS6037U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order
    pub async fn overseas_stock_v1_trading_daytime_order_sell(
        &self,
        req: OverseasStockV1TradingDaytimeOrderRequest,
    ) -> Result<OverseasStockV1TradingDaytimeOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS6037U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/daytime-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 미국주간정정취소[v1_해외주식-027]
    /// - TR_ID: Real=TTTS6038U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order-rvsecncl
    pub async fn overseas_stock_v1_trading_daytime_order_rvsecncl(
        &self,
        req: OverseasStockV1TradingDaytimeOrderRvsecnclRequest,
    ) -> Result<OverseasStockV1TradingDaytimeOrderRvsecnclResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS6038U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/daytime-order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 지정가주문번호조회  [해외주식-071]
    /// - TR_ID: Real=TTTS6058R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/algo-ordno
    pub async fn overseas_stock_v1_trading_algo_ordno(
        &self,
        req: OverseasStockV1TradingAlgoOrdnoRequest,
    ) -> Result<OverseasStockV1TradingAlgoOrdnoResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS6058R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/algo-ordno",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 지정가체결내역조회 [해외주식-070]
    /// - TR_ID: Real=TTTS6059R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-algo-ccnl
    pub async fn overseas_stock_v1_trading_inquire_algo_ccnl(
        &self,
        req: OverseasStockV1TradingInquireAlgoCcnlRequest,
    ) -> Result<OverseasStockV1TradingInquireAlgoCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTS6059R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-algo-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl OverseasQuotations {
    /// 해외주식 현재가상세[v1_해외주식-029]
    /// - TR_ID: Real=HHDFS76200200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/price-detail
    pub async fn overseas_price_v1_quotations_price_detail(
        &self,
        req: OverseasPriceV1QuotationsPriceDetailRequest,
    ) -> Result<OverseasPriceV1QuotationsPriceDetailResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76200200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/price-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 현재가 호가 [해외주식-033]
    /// - TR_ID: Real=HHDFS76200100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-asking-price
    pub async fn overseas_price_v1_quotations_inquire_asking_price(
        &self,
        req: OverseasPriceV1QuotationsInquireAskingPriceRequest,
    ) -> Result<OverseasPriceV1QuotationsInquireAskingPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76200100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-asking-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 현재체결가[v1_해외주식-009]
    /// - TR_ID: Real=HHDFS00000300 / VTS=HHDFS00000300
    /// - Endpoint: /uapi/overseas-price/v1/quotations/price
    pub async fn overseas_price_v1_quotations_price(
        &self,
        req: OverseasPriceV1QuotationsPriceRequest,
    ) -> Result<OverseasPriceV1QuotationsPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS00000300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "HHDFS00000300",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 체결추이[해외주식-037]
    /// - TR_ID: Real=HHDFS76200300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-ccnl
    pub async fn overseas_price_v1_quotations_inquire_ccnl(
        &self,
        req: OverseasPriceV1QuotationsInquireCcnlRequest,
    ) -> Result<OverseasPriceV1QuotationsInquireCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76200300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식분봉조회[v1_해외주식-030]
    /// - TR_ID: Real=HHDFS76950200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-time-itemchartprice
    pub async fn overseas_price_v1_quotations_inquire_time_itemchartprice(
        &self,
        req: OverseasPriceV1QuotationsInquireTimeItemchartpriceRequest,
    ) -> Result<OverseasPriceV1QuotationsInquireTimeItemchartpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76950200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-time-itemchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외지수분봉조회[v1_해외주식-031]
    /// - TR_ID: Real=FHKST03030200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-time-indexchartprice
    pub async fn overseas_price_v1_quotations_inquire_time_indexchartprice(
        &self,
        req: OverseasPriceV1QuotationsInquireTimeIndexchartpriceRequest,
    ) -> Result<OverseasPriceV1QuotationsInquireTimeIndexchartpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST03030200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-time-indexchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 기간별시세[v1_해외주식-010]
    /// - TR_ID: Real=HHDFS76240000 / VTS=HHDFS76240000
    /// - Endpoint: /uapi/overseas-price/v1/quotations/dailyprice
    pub async fn overseas_price_v1_quotations_dailyprice(
        &self,
        req: OverseasPriceV1QuotationsDailypriceRequest,
    ) -> Result<OverseasPriceV1QuotationsDailypriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76240000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "HHDFS76240000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/dailyprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]
    /// - TR_ID: Real=FHKST03030100 / VTS=FHKST03030100
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-daily-chartprice
    pub async fn overseas_price_v1_quotations_inquire_daily_chartprice(
        &self,
        req: OverseasPriceV1QuotationsInquireDailyChartpriceRequest,
    ) -> Result<OverseasPriceV1QuotationsInquireDailyChartpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST03030100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "FHKST03030100",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-daily-chartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식조건검색[v1_해외주식-015]
    /// - TR_ID: Real=HHDFS76410000 / VTS=HHDFS76410000
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-search
    pub async fn overseas_price_v1_quotations_inquire_search(
        &self,
        req: OverseasPriceV1QuotationsInquireSearchRequest,
    ) -> Result<OverseasPriceV1QuotationsInquireSearchResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76410000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => (
                "HHDFS76410000",
                "https://openapivts.koreainvestment.com:29443",
            ),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-search",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외결제일자조회[해외주식-017]
    /// - TR_ID: Real=CTOS5011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/quotations/countries-holiday
    pub async fn overseas_stock_v1_quotations_countries_holiday(
        &self,
        req: OverseasStockV1QuotationsCountriesHolidayRequest,
    ) -> Result<OverseasStockV1QuotationsCountriesHolidayResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTOS5011R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/quotations/countries-holiday",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 상품기본정보[v1_해외주식-034]
    /// - TR_ID: Real=CTPF1702R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/search-info
    pub async fn overseas_price_v1_quotations_search_info(
        &self,
        req: OverseasPriceV1QuotationsSearchInfoRequest,
    ) -> Result<OverseasPriceV1QuotationsSearchInfoResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTPF1702R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/search-info",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 업종별시세[해외주식-048]
    /// - TR_ID: Real=HHDFS76370000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/industry-theme
    pub async fn overseas_price_v1_quotations_industry_theme(
        &self,
        req: OverseasPriceV1QuotationsIndustryThemeRequest,
    ) -> Result<OverseasPriceV1QuotationsIndustryThemeResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76370000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/industry-theme",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 업종별코드조회[해외주식-049]
    /// - TR_ID: Real=HHDFS76370100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/industry-price
    pub async fn overseas_price_v1_quotations_industry_price(
        &self,
        req: OverseasPriceV1QuotationsIndustryPriceRequest,
    ) -> Result<OverseasPriceV1QuotationsIndustryPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76370100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/industry-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 복수종목 시세조회
    /// - TR_ID: Real=HHDFS76220000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/multprice
    pub async fn overseas_price_v1_quotations_multprice(
        &self,
        req: OverseasPriceV1QuotationsMultpriceRequest,
    ) -> Result<OverseasPriceV1QuotationsMultpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76220000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/multprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 기간별권리조회 [해외주식-052]
    /// - TR_ID: Real=CTRGT011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/period-rights
    pub async fn overseas_price_v1_quotations_period_rights(
        &self,
        req: OverseasPriceV1QuotationsPeriodRightsRequest,
    ) -> Result<OverseasPriceV1QuotationsPeriodRightsResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTRGT011R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/period-rights",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외뉴스종합(제목) [해외주식-053]
    /// - TR_ID: Real=HHPSTH60100C1 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/news-title
    pub async fn overseas_price_v1_quotations_news_title(
        &self,
        req: OverseasPriceV1QuotationsNewsTitleRequest,
    ) -> Result<OverseasPriceV1QuotationsNewsTitleResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHPSTH60100C1", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/news-title",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 권리종합 [해외주식-050]
    /// - TR_ID: Real=HHDFS78330900 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/rights-by-ice
    pub async fn overseas_price_v1_quotations_rights_by_ice(
        &self,
        req: OverseasPriceV1QuotationsRightsByIceRequest,
    ) -> Result<OverseasPriceV1QuotationsRightsByIceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS78330900", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/rights-by-ice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 당사 해외주식담보대출 가능 종목 [해외주식-051]
    /// - TR_ID: Real=CTLN4050R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/colable-by-company
    pub async fn overseas_price_v1_quotations_colable_by_company(
        &self,
        req: OverseasPriceV1QuotationsColableByCompanyRequest,
    ) -> Result<OverseasPriceV1QuotationsColableByCompanyResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTLN4050R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/colable-by-company",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외속보(제목) [해외주식-055]
    /// - TR_ID: Real=FHKST01011801 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/brknews-title
    pub async fn overseas_price_v1_quotations_brknews_title(
        &self,
        req: OverseasPriceV1QuotationsBrknewsTitleRequest,
    ) -> Result<OverseasPriceV1QuotationsBrknewsTitleResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST01011801", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/brknews-title",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl OverseasRanking {
    /// 해외주식 가격급등락[해외주식-038]
    /// - TR_ID: Real=HHDFS76260000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/price-fluct
    pub async fn overseas_stock_v1_ranking_price_fluct(
        &self,
        req: OverseasStockV1RankingPriceFluctRequest,
    ) -> Result<OverseasStockV1RankingPriceFluctResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76260000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/price-fluct",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 거래량급증[해외주식-039]
    /// - TR_ID: Real=HHDFS76270000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/volume-surge
    pub async fn overseas_stock_v1_ranking_volume_surge(
        &self,
        req: OverseasStockV1RankingVolumeSurgeRequest,
    ) -> Result<OverseasStockV1RankingVolumeSurgeResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76270000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/volume-surge",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 매수체결강도상위[해외주식-040]
    /// - TR_ID: Real=HHDFS76280000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/volume-power
    pub async fn overseas_stock_v1_ranking_volume_power(
        &self,
        req: OverseasStockV1RankingVolumePowerRequest,
    ) -> Result<OverseasStockV1RankingVolumePowerResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76280000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/volume-power",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 상승율/하락율[해외주식-041]
    /// - TR_ID: Real=HHDFS76290000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/updown-rate
    pub async fn overseas_stock_v1_ranking_updown_rate(
        &self,
        req: OverseasStockV1RankingUpdownRateRequest,
    ) -> Result<OverseasStockV1RankingUpdownRateResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76290000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/updown-rate",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 신고/신저가[해외주식-042]
    /// - TR_ID: Real=HHDFS76300000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/new-highlow
    pub async fn overseas_stock_v1_ranking_new_highlow(
        &self,
        req: OverseasStockV1RankingNewHighlowRequest,
    ) -> Result<OverseasStockV1RankingNewHighlowResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76300000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/new-highlow",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 거래량순위[해외주식-043]
    /// - TR_ID: Real=HHDFS76310010 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-vol
    pub async fn overseas_stock_v1_ranking_trade_vol(
        &self,
        req: OverseasStockV1RankingTradeVolRequest,
    ) -> Result<OverseasStockV1RankingTradeVolResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76310010", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/trade-vol",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 거래대금순위[해외주식-044]
    /// - TR_ID: Real=HHDFS76320010 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-pbmn
    pub async fn overseas_stock_v1_ranking_trade_pbmn(
        &self,
        req: OverseasStockV1RankingTradePbmnRequest,
    ) -> Result<OverseasStockV1RankingTradePbmnResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76320010", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/trade-pbmn",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 거래증가율순위[해외주식-045]
    /// - TR_ID: Real=HHDFS76330000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-growth
    pub async fn overseas_stock_v1_ranking_trade_growth(
        &self,
        req: OverseasStockV1RankingTradeGrowthRequest,
    ) -> Result<OverseasStockV1RankingTradeGrowthResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76330000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/trade-growth",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 거래회전율순위[해외주식-046]
    /// - TR_ID: Real=HHDFS76340000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-turnover
    pub async fn overseas_stock_v1_ranking_trade_turnover(
        &self,
        req: OverseasStockV1RankingTradeTurnoverRequest,
    ) -> Result<OverseasStockV1RankingTradeTurnoverResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76340000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/trade-turnover",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외주식 시가총액순위[해외주식-047]
    /// - TR_ID: Real=HHDFS76350100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/market-cap
    pub async fn overseas_stock_v1_ranking_market_cap(
        &self,
        req: OverseasStockV1RankingMarketCapRequest,
    ) -> Result<OverseasStockV1RankingMarketCapResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFS76350100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/ranking/market-cap",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}
