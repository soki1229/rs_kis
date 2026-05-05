#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct DomesticBondTrading(pub(crate) KisClient);

#[allow(dead_code)]
pub struct DomesticBondQuotations(pub(crate) KisClient);

impl crate::endpoints::DomesticBond {
    pub fn trading(&self) -> DomesticBondTrading {
        DomesticBondTrading(self.0.clone())
    }
    pub fn quotations(&self) -> DomesticBondQuotations {
        DomesticBondQuotations(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl DomesticBondTrading {
    /// 장내채권 매수주문 [국내주식-124]
    /// - TR_ID: Real=TTTC0952U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/buy
    pub async fn domestic_bond_v1_trading_buy(
        &self,
        req: DomesticBondV1TradingBuyRequest,
    ) -> Result<DomesticBondV1TradingBuyResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0952U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post("/uapi/domestic-bond/v1/trading/buy", tr_id, base_url, req)
            .await
    }

    /// 장내채권 매도주문 [국내주식-123]
    /// - TR_ID: Real=TTTC0958U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/sell
    pub async fn domestic_bond_v1_trading_sell(
        &self,
        req: DomesticBondV1TradingSellRequest,
    ) -> Result<DomesticBondV1TradingSellResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0958U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post("/uapi/domestic-bond/v1/trading/sell", tr_id, base_url, req)
            .await
    }

    /// 장내채권 정정취소주문 [국내주식-125]
    /// - TR_ID: Real=TTTC0953U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/order-rvsecncl
    pub async fn domestic_bond_v1_trading_order_rvsecncl(
        &self,
        req: DomesticBondV1TradingOrderRvsecnclRequest,
    ) -> Result<DomesticBondV1TradingOrderRvsecnclResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC0953U", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .post(
                "/uapi/domestic-bond/v1/trading/order-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 채권정정취소가능주문조회  [국내주식-126]
    /// - TR_ID: Real=CTSC8035R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-psbl-rvsecncl
    pub async fn domestic_bond_v1_trading_inquire_psbl_rvsecncl(
        &self,
        req: DomesticBondV1TradingInquirePsblRvsecnclRequest,
    ) -> Result<DomesticBondV1TradingInquirePsblRvsecnclResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTSC8035R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-psbl-rvsecncl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 주문체결내역 [국내주식-127]
    /// - TR_ID: Real=CTSC8013R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-daily-ccld
    pub async fn domestic_bond_v1_trading_inquire_daily_ccld(
        &self,
        req: DomesticBondV1TradingInquireDailyCcldRequest,
    ) -> Result<DomesticBondV1TradingInquireDailyCcldResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTSC8013R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-daily-ccld",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 잔고조회  [국내주식-198]
    /// - TR_ID: Real=CTSC8407R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-balance
    pub async fn domestic_bond_v1_trading_inquire_balance(
        &self,
        req: DomesticBondV1TradingInquireBalanceRequest,
    ) -> Result<DomesticBondV1TradingInquireBalanceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTSC8407R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-balance",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 매수가능조회 [국내주식-199]
    /// - TR_ID: Real=TTTC8910R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-psbl-order
    pub async fn domestic_bond_v1_trading_inquire_psbl_order(
        &self,
        req: DomesticBondV1TradingInquirePsblOrderRequest,
    ) -> Result<DomesticBondV1TradingInquirePsblOrderResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("TTTC8910R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-psbl-order",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl DomesticBondQuotations {
    /// 장내채권현재가(호가) [국내주식-132]
    /// - TR_ID: Real=FHKBJ773401C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-asking-price
    pub async fn domestic_bond_v1_quotations_inquire_asking_price(
        &self,
        req: DomesticBondV1QuotationsInquireAskingPriceRequest,
    ) -> Result<DomesticBondV1QuotationsInquireAskingPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKBJ773401C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-asking-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권현재가(시세) [국내주식-200]
    /// - TR_ID: Real=FHKBJ773400C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-price
    pub async fn domestic_bond_v1_quotations_inquire_price(
        &self,
        req: DomesticBondV1QuotationsInquirePriceRequest,
    ) -> Result<DomesticBondV1QuotationsInquirePriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKBJ773400C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권현재가(체결) [국내주식-201]
    /// - TR_ID: Real=FHKBJ773403C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-ccnl
    pub async fn domestic_bond_v1_quotations_inquire_ccnl(
        &self,
        req: DomesticBondV1QuotationsInquireCcnlRequest,
    ) -> Result<DomesticBondV1QuotationsInquireCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKBJ773403C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권현재가(일별) [국내주식-202]
    /// - TR_ID: Real=FHKBJ773404C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-daily-price
    pub async fn domestic_bond_v1_quotations_inquire_daily_price(
        &self,
        req: DomesticBondV1QuotationsInquireDailyPriceRequest,
    ) -> Result<DomesticBondV1QuotationsInquireDailyPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKBJ773404C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-daily-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 기간별시세(일) [국내주식-159]
    /// - TR_ID: Real=FHKBJ773701C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-daily-itemchartprice
    pub async fn domestic_bond_v1_quotations_inquire_daily_itemchartprice(
        &self,
        req: DomesticBondV1QuotationsInquireDailyItemchartpriceRequest,
    ) -> Result<DomesticBondV1QuotationsInquireDailyItemchartpriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKBJ773701C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-daily-itemchartprice",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 평균단가조회 [국내주식-158]
    /// - TR_ID: Real=CTPF2005R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/avg-unit
    pub async fn domestic_bond_v1_quotations_avg_unit(
        &self,
        req: DomesticBondV1QuotationsAvgUnitRequest,
    ) -> Result<DomesticBondV1QuotationsAvgUnitResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTPF2005R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/avg-unit",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 발행정보[국내주식-156]
    /// - TR_ID: Real=CTPF1101R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/issue-info
    pub async fn domestic_bond_v1_quotations_issue_info(
        &self,
        req: DomesticBondV1QuotationsIssueInfoRequest,
    ) -> Result<DomesticBondV1QuotationsIssueInfoResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTPF1101R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/issue-info",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// 장내채권 기본조회 [국내주식-129]
    /// - TR_ID: Real=CTPF1114R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/search-bond-info
    pub async fn domestic_bond_v1_quotations_search_bond_info(
        &self,
        req: DomesticBondV1QuotationsSearchBondInfoRequest,
    ) -> Result<DomesticBondV1QuotationsSearchBondInfoResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("CTPF1114R", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/search-bond-info",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}
