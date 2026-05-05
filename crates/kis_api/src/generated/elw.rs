#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct ElwQuotations(pub(crate) KisClient);

#[allow(dead_code)]
pub struct ElwRanking(pub(crate) KisClient);

impl crate::endpoints::Elw {
    pub fn quotations(&self) -> ElwQuotations {
        ElwQuotations(self.0.clone())
    }
    pub fn ranking(&self) -> ElwRanking {
        ElwRanking(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl ElwQuotations {
    /// ELW 신규상장종목 [국내주식-181]
    /// - TR_ID: Real=FHKEW154800C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/newly-listed
    pub async fn elw_v1_quotations_newly_listed(
        &self,
        req: ElwV1QuotationsNewlyListedRequest,
    ) -> Result<ElwV1QuotationsNewlyListedResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW154800C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/quotations/newly-listed", tr_id, base_url, req)
            .await
    }

    /// ELW 기초자산별 종목시세 [국내주식-186]
    /// - TR_ID: Real=FHKEW154101C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/udrl-asset-price
    pub async fn elw_v1_quotations_udrl_asset_price(
        &self,
        req: ElwV1QuotationsUdrlAssetPriceRequest,
    ) -> Result<ElwV1QuotationsUdrlAssetPriceResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW154101C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/udrl-asset-price",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 종목검색 [국내주식-166]
    /// - TR_ID: Real=FHKEW15100000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/cond-search
    pub async fn elw_v1_quotations_cond_search(
        &self,
        req: ElwV1QuotationsCondSearchRequest,
    ) -> Result<ElwV1QuotationsCondSearchResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW15100000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/quotations/cond-search", tr_id, base_url, req)
            .await
    }

    /// ELW 기초자산 목록조회 [국내주식-185]
    /// - TR_ID: Real=FHKEW154100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/udrl-asset-list
    pub async fn elw_v1_quotations_udrl_asset_list(
        &self,
        req: ElwV1QuotationsUdrlAssetListRequest,
    ) -> Result<ElwV1QuotationsUdrlAssetListResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW154100C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/udrl-asset-list",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 비교대상종목조회 [국내주식-183]
    /// - TR_ID: Real=FHKEW151701C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/compare-stocks
    pub async fn elw_v1_quotations_compare_stocks(
        &self,
        req: ElwV1QuotationsCompareStocksRequest,
    ) -> Result<ElwV1QuotationsCompareStocksResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW151701C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/compare-stocks",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW LP매매추이 [국내주식-182]
    /// - TR_ID: Real=FHPEW03760000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/lp-trade-trend
    pub async fn elw_v1_quotations_lp_trade_trend(
        &self,
        req: ElwV1QuotationsLpTradeTrendRequest,
    ) -> Result<ElwV1QuotationsLpTradeTrendResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW03760000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/lp-trade-trend",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 투자지표추이(체결) [국내주식-172]
    /// - TR_ID: Real=FHPEW02740100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/indicator-trend-ccnl
    pub async fn elw_v1_quotations_indicator_trend_ccnl(
        &self,
        req: ElwV1QuotationsIndicatorTrendCcnlRequest,
    ) -> Result<ElwV1QuotationsIndicatorTrendCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02740100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/indicator-trend-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 투자지표추이(분별) [국내주식-174]
    /// - TR_ID: Real=FHPEW02740300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/indicator-trend-minute
    pub async fn elw_v1_quotations_indicator_trend_minute(
        &self,
        req: ElwV1QuotationsIndicatorTrendMinuteRequest,
    ) -> Result<ElwV1QuotationsIndicatorTrendMinuteResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02740300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/indicator-trend-minute",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 투자지표추이(일별) [국내주식-173]
    /// - TR_ID: Real=FHPEW02740200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/indicator-trend-daily
    pub async fn elw_v1_quotations_indicator_trend_daily(
        &self,
        req: ElwV1QuotationsIndicatorTrendDailyRequest,
    ) -> Result<ElwV1QuotationsIndicatorTrendDailyResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02740200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/indicator-trend-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 변동성 추이(틱) [국내주식-180]
    /// - TR_ID: Real=FHPEW02840400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-tick
    pub async fn elw_v1_quotations_volatility_trend_tick(
        &self,
        req: ElwV1QuotationsVolatilityTrendTickRequest,
    ) -> Result<ElwV1QuotationsVolatilityTrendTickResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02840400", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/volatility-trend-tick",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 변동성추이(체결) [국내주식-177]
    /// - TR_ID: Real=FHPEW02840100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-ccnl
    pub async fn elw_v1_quotations_volatility_trend_ccnl(
        &self,
        req: ElwV1QuotationsVolatilityTrendCcnlRequest,
    ) -> Result<ElwV1QuotationsVolatilityTrendCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02840100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/volatility-trend-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 변동성 추이(일별) [국내주식-178]
    /// - TR_ID: Real=FHPEW02840200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-daily
    pub async fn elw_v1_quotations_volatility_trend_daily(
        &self,
        req: ElwV1QuotationsVolatilityTrendDailyRequest,
    ) -> Result<ElwV1QuotationsVolatilityTrendDailyResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02840200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/volatility-trend-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 민감도 추이(체결) [국내주식-175]
    /// - TR_ID: Real=FHPEW02830100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/sensitivity-trend-ccnl
    pub async fn elw_v1_quotations_sensitivity_trend_ccnl(
        &self,
        req: ElwV1QuotationsSensitivityTrendCcnlRequest,
    ) -> Result<ElwV1QuotationsSensitivityTrendCcnlResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02830100", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/sensitivity-trend-ccnl",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 변동성 추이(분별) [국내주식-179]
    /// - TR_ID: Real=FHPEW02840300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-minute
    pub async fn elw_v1_quotations_volatility_trend_minute(
        &self,
        req: ElwV1QuotationsVolatilityTrendMinuteRequest,
    ) -> Result<ElwV1QuotationsVolatilityTrendMinuteResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02840300", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/volatility-trend-minute",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 민감도 추이(일별) [국내주식-176]
    /// - TR_ID: Real=FHPEW02830200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/sensitivity-trend-daily
    pub async fn elw_v1_quotations_sensitivity_trend_daily(
        &self,
        req: ElwV1QuotationsSensitivityTrendDailyRequest,
    ) -> Result<ElwV1QuotationsSensitivityTrendDailyResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02830200", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/sensitivity-trend-daily",
                tr_id,
                base_url,
                req,
            )
            .await
    }

    /// ELW 만기예정/만기종목 [국내주식-184]
    /// - TR_ID: Real=FHKEW154700C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/expiration-stocks
    pub async fn elw_v1_quotations_expiration_stocks(
        &self,
        req: ElwV1QuotationsExpirationStocksRequest,
    ) -> Result<ElwV1QuotationsExpirationStocksResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHKEW154700C0", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "미지원"),
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/expiration-stocks",
                tr_id,
                base_url,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl ElwRanking {
    /// ELW 민감도 순위[국내주식-170]
    /// - TR_ID: Real=FHPEW02850000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/sensitivity
    pub async fn elw_v1_ranking_sensitivity(
        &self,
        req: ElwV1RankingSensitivityRequest,
    ) -> Result<ElwV1RankingSensitivityResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02850000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/ranking/sensitivity", tr_id, base_url, req)
            .await
    }

    /// ELW 당일급변종목[국내주식-171]
    /// - TR_ID: Real=FHPEW02870000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/quick-change
    pub async fn elw_v1_ranking_quick_change(
        &self,
        req: ElwV1RankingQuickChangeRequest,
    ) -> Result<ElwV1RankingQuickChangeResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02870000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/ranking/quick-change", tr_id, base_url, req)
            .await
    }

    /// ELW 지표순위[국내주식-169]
    /// - TR_ID: Real=FHPEW02790000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/indicator
    pub async fn elw_v1_ranking_indicator(
        &self,
        req: ElwV1RankingIndicatorRequest,
    ) -> Result<ElwV1RankingIndicatorResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02790000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/ranking/indicator", tr_id, base_url, req)
            .await
    }

    /// ELW 상승률순위[국내주식-167]
    /// - TR_ID: Real=FHPEW02770000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/updown-rate
    pub async fn elw_v1_ranking_updown_rate(
        &self,
        req: ElwV1RankingUpdownRateRequest,
    ) -> Result<ElwV1RankingUpdownRateResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02770000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/ranking/updown-rate", tr_id, base_url, req)
            .await
    }

    /// ELW 거래량순위[국내주식-168]
    /// - TR_ID: Real=FHPEW02780000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/volume-rank
    pub async fn elw_v1_ranking_volume_rank(
        &self,
        req: ElwV1RankingVolumeRankRequest,
    ) -> Result<ElwV1RankingVolumeRankResponse, KisError> {
        let (tr_id, base_url) = match self.0.env() {
            crate::client::KisEnv::Real => {
                ("FHPEW02780000", "https://openapi.koreainvestment.com:9443")
            }
            crate::client::KisEnv::Vts => ("모의투자 미지원", "모의투자 미지원"),
        };
        self.0
            .get("/uapi/elw/v1/ranking/volume-rank", tr_id, base_url, req)
            .await
    }
}
