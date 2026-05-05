#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct OverseasFutureOptionTrading(pub(crate) KisClient);

#[allow(dead_code)]
pub struct OverseasFutureOptionQuotations(pub(crate) KisClient);

impl crate::endpoints::OverseasFutureOption {
    pub fn trading(&self) -> OverseasFutureOptionTrading {
        OverseasFutureOptionTrading(self.0.clone())
    }
    pub fn quotations(&self) -> OverseasFutureOptionQuotations {
        OverseasFutureOptionQuotations(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl OverseasFutureOptionTrading {
    /// 해외선물옵션 주문 [v1_해외선물-001]
    /// - TR_ID: Real=OTFM3001U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/order
    pub async fn overseas_futureoption_v1_trading_order(
        &self,
        req: OverseasFutureoptionV1TradingOrderRequest,
    ) -> Result<OverseasFutureoptionV1TradingOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3001U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/overseas-futureoption/v1/trading/order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 정정취소주문 [v1_해외선물-002, 003]
    /// - TR_ID: Real=OTFM3002U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/order-rvsecncl
    pub async fn overseas_futureoption_v1_trading_order_rvsecncl_modify(
        &self,
        req: OverseasFutureoptionV1TradingOrderRvsecnclRequest,
    ) -> Result<OverseasFutureoptionV1TradingOrderRvsecnclResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3002U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/overseas-futureoption/v1/trading/order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 정정취소주문 [v1_해외선물-002, 003]
    /// - TR_ID: Real=OTFM3003U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/order-rvsecncl
    pub async fn overseas_futureoption_v1_trading_order_rvsecncl_cancel(
        &self,
        req: OverseasFutureoptionV1TradingOrderRvsecnclRequest,
    ) -> Result<OverseasFutureoptionV1TradingOrderRvsecnclResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3003U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/overseas-futureoption/v1/trading/order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 당일주문내역조회 [v1_해외선물-004]
    /// - TR_ID: Real=OTFM3116R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-ccld
    pub async fn overseas_futureoption_v1_trading_inquire_ccld(
        &self,
        req: OverseasFutureoptionV1TradingInquireCcldRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquireCcldResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3116R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-ccld",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]
    /// - TR_ID: Real=OTFM1412R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-unpd
    pub async fn overseas_futureoption_v1_trading_inquire_unpd(
        &self,
        req: OverseasFutureoptionV1TradingInquireUnpdRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquireUnpdResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM1412R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-unpd",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 주문가능조회 [v1_해외선물-006]
    /// - TR_ID: Real=OTFM3304R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-psamount
    pub async fn overseas_futureoption_v1_trading_inquire_psamount(
        &self,
        req: OverseasFutureoptionV1TradingInquirePsamountRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquirePsamountResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3304R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-psamount",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 기간계좌손익 일별[해외선물-010]
    /// - TR_ID: Real=OTFM3118R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-period-ccld
    pub async fn overseas_futureoption_v1_trading_inquire_period_ccld(
        &self,
        req: OverseasFutureoptionV1TradingInquirePeriodCcldRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquirePeriodCcldResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3118R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-period-ccld",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 일별 체결내역[해외선물-011]
    /// - TR_ID: Real=OTFM3122R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-daily-ccld
    pub async fn overseas_futureoption_v1_trading_inquire_daily_ccld(
        &self,
        req: OverseasFutureoptionV1TradingInquireDailyCcldRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquireDailyCcldResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3122R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-daily-ccld",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 예수금현황[해외선물-012]
    /// - TR_ID: Real=OTFM1411R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-deposit
    pub async fn overseas_futureoption_v1_trading_inquire_deposit(
        &self,
        req: OverseasFutureoptionV1TradingInquireDepositRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquireDepositResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM1411R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-deposit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 일별 주문내역[해외선물-013]
    /// - TR_ID: Real=OTFM3120R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-daily-order
    pub async fn overseas_futureoption_v1_trading_inquire_daily_order(
        &self,
        req: OverseasFutureoptionV1TradingInquireDailyOrderRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquireDailyOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3120R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-daily-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 기간계좌거래내역[해외선물-014]
    /// - TR_ID: Real=OTFM3114R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-period-trans
    pub async fn overseas_futureoption_v1_trading_inquire_period_trans(
        &self,
        req: OverseasFutureoptionV1TradingInquirePeriodTransRequest,
    ) -> Result<OverseasFutureoptionV1TradingInquirePeriodTransResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3114R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-period-trans",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 증거금상세 [해외선물-032]
    /// - TR_ID: Real=OTFM3115R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/margin-detail
    pub async fn overseas_futureoption_v1_trading_margin_detail(
        &self,
        req: OverseasFutureoptionV1TradingMarginDetailRequest,
    ) -> Result<OverseasFutureoptionV1TradingMarginDetailResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM3115R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/margin-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl OverseasFutureOptionQuotations {
    /// 해외선물종목현재가 [v1_해외선물-009]
    /// - TR_ID: Real=HHDFC55010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-price
    pub async fn overseas_futureoption_v1_quotations_inquire_price(
        &self,
        req: OverseasFutureoptionV1QuotationsInquirePriceRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsInquirePriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55010000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물종목상세 [v1_해외선물-008]
    /// - TR_ID: Real=HHDFC55010100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/stock-detail
    pub async fn overseas_futureoption_v1_quotations_stock_detail(
        &self,
        req: OverseasFutureoptionV1QuotationsStockDetailRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsStockDetailResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55010100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/stock-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 호가 [해외선물-031]
    /// - TR_ID: Real=HHDFC86000000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-asking-price
    pub async fn overseas_futureoption_v1_quotations_inquire_asking_price(
        &self,
        req: OverseasFutureoptionV1QuotationsInquireAskingPriceRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsInquireAskingPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC86000000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-asking-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 분봉조회[해외선물-016]
    /// - TR_ID: Real=HHDFC55020400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice
    pub async fn overseas_futureoption_v1_quotations_inquire_time_futurechartprice(
        &self,
        req: OverseasFutureoptionV1QuotationsInquireTimeFuturechartpriceRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsInquireTimeFuturechartpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55020400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(틱)[해외선물-019]
    /// - TR_ID: Real=HHDFC55020200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/tick-ccnl
    pub async fn overseas_futureoption_v1_quotations_tick_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsTickCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsTickCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55020200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/tick-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(주간)[해외선물-017]
    /// - TR_ID: Real=HHDFC55020000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/weekly-ccnl
    pub async fn overseas_futureoption_v1_quotations_weekly_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsWeeklyCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsWeeklyCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55020000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/weekly-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(일간)[해외선물-018]
    /// - TR_ID: Real=HHDFC55020100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/daily-ccnl
    pub async fn overseas_futureoption_v1_quotations_daily_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsDailyCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsDailyCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55020100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/daily-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(월간)[해외선물-020]
    /// - TR_ID: Real=HHDFC55020300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/monthly-ccnl
    pub async fn overseas_futureoption_v1_quotations_monthly_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsMonthlyCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsMonthlyCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55020300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/monthly-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 상품기본정보 [해외선물-023]
    /// - TR_ID: Real=HHDFC55200000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/search-contract-detail
    pub async fn overseas_futureoption_v1_quotations_search_contract_detail(
        &self,
        req: OverseasFutureoptionV1QuotationsSearchContractDetailRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsSearchContractDetailResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFC55200000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/search-contract-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물 미결제추이 [해외선물-029]
    /// - TR_ID: Real=HHDDB95030000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/investor-unpd-trend
    pub async fn overseas_futureoption_v1_quotations_investor_unpd_trend(
        &self,
        req: OverseasFutureoptionV1QuotationsInvestorUnpdTrendRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsInvestorUnpdTrendResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDDB95030000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/investor-unpd-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션종목현재가 [해외선물-035]
    /// - TR_ID: Real=HHDFO55010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-price
    pub async fn overseas_futureoption_v1_quotations_opt_price(
        &self,
        req: OverseasFutureoptionV1QuotationsOptPriceRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55010000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션종목상세 [해외선물-034]
    /// - TR_ID: Real=HHDFO55010100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-detail
    pub async fn overseas_futureoption_v1_quotations_opt_detail(
        &self,
        req: OverseasFutureoptionV1QuotationsOptDetailRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptDetailResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55010100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 호가 [해외선물-033]
    /// - TR_ID: Real=HHDFO86000000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-asking-price
    pub async fn overseas_futureoption_v1_quotations_opt_asking_price(
        &self,
        req: OverseasFutureoptionV1QuotationsOptAskingPriceRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptAskingPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO86000000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-asking-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 분봉조회 [해외선물-040]
    /// - TR_ID: Real=HHDFO55020400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-time-optchartprice
    pub async fn overseas_futureoption_v1_quotations_inquire_time_optchartprice(
        &self,
        req: OverseasFutureoptionV1QuotationsInquireTimeOptchartpriceRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsInquireTimeOptchartpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55020400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-time-optchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(틱) [해외선물-038]
    /// - TR_ID: Real=HHDFO55020200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl
    pub async fn overseas_futureoption_v1_quotations_opt_tick_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsOptTickCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptTickCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55020200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(일간) [해외선물-037]
    /// - TR_ID: Real=HHDFO55020100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-daily-ccnl
    pub async fn overseas_futureoption_v1_quotations_opt_daily_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsOptDailyCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptDailyCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55020100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-daily-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(주간) [해외선물-036]
    /// - TR_ID: Real=HHDFO55020000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-weekly-ccnl
    pub async fn overseas_futureoption_v1_quotations_opt_weekly_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsOptWeeklyCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptWeeklyCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55020000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-weekly-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(월간) [해외선물-039]
    /// - TR_ID: Real=HHDFO55020300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-monthly-ccnl
    pub async fn overseas_futureoption_v1_quotations_opt_monthly_ccnl(
        &self,
        req: OverseasFutureoptionV1QuotationsOptMonthlyCcnlRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsOptMonthlyCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55020300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-monthly-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외옵션 상품기본정보 [해외선물-041]
    /// - TR_ID: Real=HHDFO55200000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/search-opt-detail
    pub async fn overseas_futureoption_v1_quotations_search_opt_detail(
        &self,
        req: OverseasFutureoptionV1QuotationsSearchOptDetailRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsSearchOptDetailResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("HHDFO55200000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/search-opt-detail",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 해외선물옵션 장운영시간 [해외선물-030]
    /// - TR_ID: Real=OTFM2229R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/market-time
    pub async fn overseas_futureoption_v1_quotations_market_time(
        &self,
        req: OverseasFutureoptionV1QuotationsMarketTimeRequest,
    ) -> Result<OverseasFutureoptionV1QuotationsMarketTimeResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("OTFM2229R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/market-time",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}
