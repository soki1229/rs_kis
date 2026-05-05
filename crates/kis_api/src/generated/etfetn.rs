#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct EtfEtnQuotations(pub(crate) KisClient);

impl crate::endpoints::EtfEtn {
    pub fn quotations(&self) -> EtfEtnQuotations {
        EtfEtnQuotations(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl EtfEtnQuotations {
    /// ETF/ETN 현재가[v1_국내주식-068]
    /// - TR_ID: Real=FHPST02400000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/inquire-price
    pub async fn etfetn_v1_quotations_inquire_price(
        &self,
        req: EtfetnV1QuotationsInquirePriceRequest,
    ) -> Result<EtfetnV1QuotationsInquirePriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02400000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/inquire-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ETF 구성종목시세[국내주식-073]
    /// - TR_ID: Real=FHKST121600C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/inquire-component-stock-price
    pub async fn etfetn_v1_quotations_inquire_component_stock_price(
        &self,
        req: EtfetnV1QuotationsInquireComponentStockPriceRequest,
    ) -> Result<EtfetnV1QuotationsInquireComponentStockPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKST121600C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/inquire-component-stock-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// NAV 비교추이(종목)[v1_국내주식-069]
    /// - TR_ID: Real=FHPST02440000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/nav-comparison-trend
    pub async fn etfetn_v1_quotations_nav_comparison_trend(
        &self,
        req: EtfetnV1QuotationsNavComparisonTrendRequest,
    ) -> Result<EtfetnV1QuotationsNavComparisonTrendResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02440000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/nav-comparison-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// NAV 비교추이(일)[v1_국내주식-071]
    /// - TR_ID: Real=FHPST02440200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/nav-comparison-daily-trend
    pub async fn etfetn_v1_quotations_nav_comparison_daily_trend(
        &self,
        req: EtfetnV1QuotationsNavComparisonDailyTrendRequest,
    ) -> Result<EtfetnV1QuotationsNavComparisonDailyTrendResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => ("FHPST02440200", ""),
            crate::client::KisEnv::Vts => ("모의투자 미지원", ""),
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/nav-comparison-daily-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// NAV 비교추이(분)[v1_국내주식-070]
    /// - TR_ID: Real=FHPST02440100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/nav-comparison-time-trend
    pub async fn etfetn_v1_quotations_nav_comparison_time_trend(
        &self,
        req: EtfetnV1QuotationsNavComparisonTimeTrendRequest,
    ) -> Result<EtfetnV1QuotationsNavComparisonTimeTrendResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPST02440100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/nav-comparison-time-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}
