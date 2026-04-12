#![allow(clippy::doc_lazy_continuation)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct Trading(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Quotations(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Finance(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Ksdinfo(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Ranking(pub(crate) KisClient);

impl crate::endpoints::Stock {
    pub fn trading(&self) -> Trading {
        Trading(self.0.clone())
    }
    pub fn quotations(&self) -> Quotations {
        Quotations(self.0.clone())
    }
    pub fn finance(&self) -> Finance {
        Finance(self.0.clone())
    }
    pub fn ksdinfo(&self) -> Ksdinfo {
        Ksdinfo(self.0.clone())
    }
    pub fn ranking(&self) -> Ranking {
        Ranking(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl Trading {
    /// 주식주문(현금)
    /// 주식주문(현금)[v1_국내주식-001]
    pub async fn order_cash(&self, req: OrderCashRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0011U",
            crate::client::KisEnv::Vts => "VTTC0011U",
        };
        self.0
            .post("/uapi/domestic-stock/v1/trading/order-cash", tr_id, req)
            .await
    }

    /// 주식주문(신용)
    /// 주식주문(신용)[v1_국내주식-002]
    pub async fn order_credit(
        &self,
        req: OrderCreditRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0051U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/domestic-stock/v1/trading/order-credit", tr_id, req)
            .await
    }

    /// 주식주문(정정취소)
    /// 주식주문(정정취소)[v1_국내주식-003]
    pub async fn order_rvsecncl(
        &self,
        req: OrderRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0013U",
            crate::client::KisEnv::Vts => "VTTC0013U",
        };
        self.0
            .post("/uapi/domestic-stock/v1/trading/order-rvsecncl", tr_id, req)
            .await
    }

    /// 주식정정취소가능주문조회
    /// 주식정정취소가능주문조회[v1_국내주식-004]
    pub async fn inquire_psbl_rvsecncl(
        &self,
        req: InquirePsblRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0084R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl",
                tr_id,
                req,
            )
            .await
    }

    /// 주식일별주문체결조회
    /// 주식일별주문체결조회[v1_국내주식-005]
    pub async fn inquire_daily_ccld(
        &self,
        req: InquireDailyCcldRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0081R",
            crate::client::KisEnv::Vts => "VTTC0081R",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
                tr_id,
                req,
            )
            .await
    }

    /// 주식잔고조회
    /// 2021-12-29 11:22:33
    pub async fn inquire_balance(
        &self,
        req: InquireBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8434R",
            crate::client::KisEnv::Vts => "VTTC8434R",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 매수가능조회
    /// 2021-12-29 11:22:33
    pub async fn inquire_psbl_order(
        &self,
        req: InquirePsblOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8908R",
            crate::client::KisEnv::Vts => "VTTC8908R",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-psbl-order",
                tr_id,
                req,
            )
            .await
    }

    /// 매도가능수량조회
    /// 2021-12-29 11:22:33
    pub async fn inquire_psbl_sell(
        &self,
        req: InquirePsblSellRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8408R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-psbl-sell",
                tr_id,
                req,
            )
            .await
    }

    /// 신용매수가능조회
    /// 신용매수가능조회[v1_국내주식-042]
    pub async fn inquire_credit_psamount(
        &self,
        req: InquireCreditPsamountRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8909R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-credit-psamount",
                tr_id,
                req,
            )
            .await
    }

    /// 주식예약주문
    /// 2021-12-29 11:22:33
    pub async fn order_resv(&self, req: OrderResvRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC0008U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/domestic-stock/v1/trading/order-resv", tr_id, req)
            .await
    }

    /// 주식예약주문정정취소
    /// 주식예약주문정정취소[v1_국내주식-018,019]
    pub async fn order_resv_rvsecncl(
        &self,
        req: OrderResvRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC0009U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post(
                "/uapi/domestic-stock/v1/trading/order-resv-rvsecncl",
                tr_id,
                req,
            )
            .await
    }

    /// 주식예약주문조회
    /// 주식예약주문조회[v1_국내주식-020]
    pub async fn order_resv_ccnl(
        &self,
        req: OrderResvCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC0004R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/order-resv-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 퇴직연금 체결기준잔고
    /// 퇴직연금 체결기준잔고[v1_국내주식-032]
    pub async fn inquire_present_balance(
        &self,
        req: InquirePresentBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC2202R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-present-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 퇴직연금 미체결내역
    /// 퇴직연금 미체결내역[v1_국내주식-033]
    pub async fn inquire_daily_ccld_next(
        &self,
        req: InquireDailyCcldNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC2201R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld",
                tr_id,
                req,
            )
            .await
    }

    /// 퇴직연금 매수가능조회
    /// 퇴직연금 매수가능조회[v1_국내주식-034]
    pub async fn inquire_psbl_order_next(
        &self,
        req: InquirePsblOrderNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0503R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-psbl-order",
                tr_id,
                req,
            )
            .await
    }

    /// 퇴직연금 예수금조회
    /// 퇴직연금 예수금조회[v1_국내주식-035]
    pub async fn inquire_deposit(
        &self,
        req: InquireDepositRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0506R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-deposit",
                tr_id,
                req,
            )
            .await
    }

    /// 퇴직연금 잔고조회
    /// 퇴직연금 잔고조회[v1_국내주식-036]
    pub async fn inquire_balance_next(
        &self,
        req: InquireBalanceNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC2208R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/pension/inquire-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 주식잔고조회_실현손익
    /// 주식잔고조회_실현손익[v1_국내주식-041]
    pub async fn inquire_balance_rlz_pl(
        &self,
        req: InquireBalanceRlzPlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8494R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl",
                tr_id,
                req,
            )
            .await
    }

    /// 투자계좌자산현황조회
    /// 투자계좌자산현황조회[v1_국내주식-048]
    pub async fn inquire_account_balance(
        &self,
        req: InquireAccountBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRP6548R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-account-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 기간별손익일별합산조회
    /// 기간별손익일별합산조회[v1_국내주식-052]
    pub async fn inquire_period_profit(
        &self,
        req: InquirePeriodProfitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8708R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-period-profit",
                tr_id,
                req,
            )
            .await
    }

    /// 기간별매매손익현황조회
    /// 기간별매매손익현황조회[v1_국내주식-060]
    pub async fn inquire_period_trade_profit(
        &self,
        req: InquirePeriodTradeProfitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8715R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/trading/inquire-period-trade-profit",
                tr_id,
                req,
            )
            .await
    }

    /// 주식통합증거금 현황
    /// 주식통합증거금 현황 [국내주식-191]
    pub async fn intgr_margin(
        &self,
        req: IntgrMarginRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0869R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/trading/intgr-margin", tr_id, req)
            .await
    }

    /// 기간별계좌권리현황조회
    /// 기간별계좌권리현황조회 [국내주식-211]
    pub async fn period_rights(
        &self,
        req: PeriodRightsRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRGA011R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/trading/period-rights", tr_id, req)
            .await
    }
}

#[allow(non_snake_case)]
impl Quotations {
    /// 주식현재가 시세
    /// 주식현재가 시세[v1_국내주식-008]
    pub async fn inquire_price(
        &self,
        req: InquirePriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01010100",
            crate::client::KisEnv::Vts => "FHKST01010100",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-price",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 시세2
    /// 주식현재가 시세2[v1_국내주식-054]
    pub async fn inquire_price_2(
        &self,
        req: InquirePrice2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01010000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-price-2",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 체결
    /// 주식현재가 체결[v1_국내주식-009]
    pub async fn inquire_ccnl(
        &self,
        req: InquireCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01010300",
            crate::client::KisEnv::Vts => "FHKST01010300",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 일자별
    /// 주식현재가 일자별[v1_국내주식-010]
    pub async fn inquire_daily_price(
        &self,
        req: InquireDailyPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01010400",
            crate::client::KisEnv::Vts => "FHKST01010400",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-price",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 호가/예상체결
    /// 주식현재가 호가/예상체결[v1_국내주식-011]
    pub async fn inquire_asking_price_exp_ccn(
        &self,
        req: InquireAskingPriceExpCcnRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01010200",
            crate::client::KisEnv::Vts => "FHKST01010200",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 투자자
    /// 주식현재가 투자자[v1_국내주식-012]
    pub async fn inquire_investor(
        &self,
        req: InquireInvestorRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01010900",
            crate::client::KisEnv::Vts => "FHKST01010900",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-investor",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 회원사
    /// 주식현재가 회원사[v1_국내주식-013]
    pub async fn inquire_member(
        &self,
        req: InquireMemberRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01010600",
            crate::client::KisEnv::Vts => "FHKST01010600",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-member",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식기간별시세(일/주/월/년)
    /// 국내주식기간별시세(일/주/월/년)[v1_국내주식-016]
    pub async fn inquire_daily_itemchartprice(
        &self,
        req: InquireDailyItemchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03010100",
            crate::client::KisEnv::Vts => "FHKST03010100",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 주식당일분봉조회
    /// 주식당일분봉조회[v1_국내주식-022]
    pub async fn inquire_time_itemchartprice(
        &self,
        req: InquireTimeItemchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03010200",
            crate::client::KisEnv::Vts => "FHKST03010200",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-itemchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 주식일별분봉조회
    /// 2021-12-29 11:22:33
    pub async fn inquire_time_dailychartprice(
        &self,
        req: InquireTimeDailychartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03010230",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 당일시간대별체결
    /// 주식현재가 당일시간대별체결[v1_국내주식-023]
    pub async fn inquire_time_itemconclusion(
        &self,
        req: InquireTimeItemconclusionRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01060000",
            crate::client::KisEnv::Vts => "FHPST01060000",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 시간외일자별주가
    /// 주식현재가 시간외일자별주가[v1_국내주식-026]
    pub async fn inquire_daily_overtimeprice(
        &self,
        req: InquireDailyOvertimepriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02320000",
            crate::client::KisEnv::Vts => "FHPST02320000",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 시간외시간별체결
    /// 주식현재가 시간외시간별체결[v1_국내주식-025]
    pub async fn inquire_time_overtimeconclusion(
        &self,
        req: InquireTimeOvertimeconclusionRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02310000",
            crate::client::KisEnv::Vts => "FHPST02310000",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 시간외현재가
    /// 국내주식 시간외현재가[국내주식-076]
    pub async fn inquire_overtime_price(
        &self,
        req: InquireOvertimePriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02300000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-overtime-price",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 시간외호가
    /// 국내주식 시간외호가[국내주식-077]
    pub async fn inquire_overtime_asking_price(
        &self,
        req: InquireOvertimeAskingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02300400",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-overtime-asking-price",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 장마감 예상체결가
    /// 국내주식 장마감 예상체결가[국내주식-120]
    pub async fn exp_closing_price(
        &self,
        req: ExpClosingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST117300C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-closing-price",
                tr_id,
                req,
            )
            .await
    }

    /// ELW 현재가 시세
    /// ELW 현재가 시세[v1_국내주식-014]
    pub async fn inquire_elw_price(
        &self,
        req: InquireElwPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW15010000",
            crate::client::KisEnv::Vts => "FHKEW15010000",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-elw-price",
                tr_id,
                req,
            )
            .await
    }

    /// 국내업종 현재지수
    /// 국내업종 현재지수[v1_국내주식-063]
    pub async fn inquire_index_price(
        &self,
        req: InquireIndexPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPUP02100000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-price",
                tr_id,
                req,
            )
            .await
    }

    /// 국내업종 일자별지수
    /// 국내업종 일자별지수[v1_국내주식-065]
    pub async fn inquire_index_daily_price(
        &self,
        req: InquireIndexDailyPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPUP02120000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-daily-price",
                tr_id,
                req,
            )
            .await
    }

    /// 국내업종 시간별지수(초)
    /// 국내업종 시간별지수(초)[국내주식-064]
    pub async fn inquire_index_tickprice(
        &self,
        req: InquireIndexTickpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPUP02110100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-tickprice",
                tr_id,
                req,
            )
            .await
    }

    /// 국내업종 시간별지수(분)
    /// 국내업종 시간별지수(분)[국내주식-119]
    pub async fn inquire_index_timeprice(
        &self,
        req: InquireIndexTimepriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPUP02110200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-timeprice",
                tr_id,
                req,
            )
            .await
    }

    /// 업종 분봉조회
    /// 업종 분봉조회[v1_국내주식-045]
    pub async fn inquire_time_indexchartprice(
        &self,
        req: InquireTimeIndexchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKUP03500200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-time-indexchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식업종기간별시세(일/주/월/년)
    /// 국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]
    pub async fn inquire_daily_indexchartprice(
        &self,
        req: InquireDailyIndexchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKUP03500100",
            crate::client::KisEnv::Vts => "FHKUP03500100",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 국내업종 구분별전체시세
    /// 국내업종 구분별전체시세[v1_국내주식-066]
    pub async fn inquire_index_category_price(
        &self,
        req: InquireIndexCategoryPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPUP02140000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-index-category-price",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 예상체결지수 추이
    /// 국내주식 예상체결지수 추이[국내주식-121]
    pub async fn exp_index_trend(
        &self,
        req: ExpIndexTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01840000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-index-trend",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 예상체결 전체지수
    /// 국내주식 예상체결 전체지수[국내주식-122]
    pub async fn exp_total_index(
        &self,
        req: ExpTotalIndexRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKUP11750000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-total-index",
                tr_id,
                req,
            )
            .await
    }

    /// 변동성완화장치(VI) 현황
    /// 변동성완화장치(VI) 현황 [v1_국내주식-055]
    pub async fn inquire_vi_status(
        &self,
        req: InquireViStatusRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01390000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-vi-status",
                tr_id,
                req,
            )
            .await
    }

    /// 금리 종합(국내채권/금리)
    /// 금리 종합(국내채권/금리) [국내주식-155]
    pub async fn comp_interest(
        &self,
        req: CompInterestRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST07020000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/comp-interest",
                tr_id,
                req,
            )
            .await
    }

    /// 종합 시황/공시(제목)
    /// 종합 시황/공시(제목) [국내주식-141]
    pub async fn news_title(&self, req: NewsTitleRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01011800",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/news-title", tr_id, req)
            .await
    }

    /// 국내휴장일조회
    /// 2021-12-29 11:22:33
    pub async fn chk_holiday(&self, req: ChkHolidayRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTCA0903R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/chk-holiday", tr_id, req)
            .await
    }

    /// 국내선물 영업일조회
    /// 국내선물 영업일조회 [국내주식-160]
    pub async fn market_time(&self, req: ()) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHMCM000002C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/market-time", tr_id, req)
            .await
    }

    /// 상품기본조회
    /// 2021-12-29 11:22:33
    pub async fn search_info(&self, req: SearchInfoRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF1604R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/search-info", tr_id, req)
            .await
    }

    /// 주식기본조회
    /// 2021-12-29 11:22:33
    pub async fn search_stock_info(
        &self,
        req: SearchStockInfoRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF1002R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/search-stock-info",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 당사 신용가능종목
    /// 국내주식 당사 신용가능종목[국내주식-111]
    pub async fn credit_by_company(
        &self,
        req: CreditByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST04770000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/credit-by-company",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 종목추정실적
    /// 국내주식 종목추정실적 [국내주식-187]
    pub async fn estimate_perform(
        &self,
        req: EstimatePerformRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKST668300C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/estimate-perform",
                tr_id,
                req,
            )
            .await
    }

    /// 당사 대주가능 종목
    /// 당사 대주가능 종목 [국내주식-195]
    pub async fn lendable_by_company(
        &self,
        req: LendableByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC2702R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/lendable-by-company",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 종목투자의견
    /// 국내주식 종목투자의견 [국내주식-188]
    pub async fn invest_opinion(
        &self,
        req: InvestOpinionRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST663300C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/invest-opinion",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 증권사별 투자의견
    /// 국내주식 증권사별 투자의견 [국내주식-189]
    pub async fn invest_opbysec(
        &self,
        req: InvestOpbysecRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST663400C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/invest-opbysec",
                tr_id,
                req,
            )
            .await
    }

    /// 종목조건검색 목록조회
    /// 종목조건검색 목록조회[국내주식-038]
    pub async fn psearch_title(
        &self,
        req: PsearchTitleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKST03900300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/psearch-title",
                tr_id,
                req,
            )
            .await
    }

    /// 종목조건검색조회
    /// 2021-12-29 11:22:33
    pub async fn psearch_result(
        &self,
        req: PsearchResultRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKST03900400",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/psearch-result",
                tr_id,
                req,
            )
            .await
    }

    /// 관심종목 그룹조회
    /// 관심종목 그룹조회 [국내주식-204]
    pub async fn intstock_grouplist(
        &self,
        req: IntstockGrouplistRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKCM113004C7",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/intstock-grouplist",
                tr_id,
                req,
            )
            .await
    }

    /// 관심종목(멀티종목) 시세조회
    /// 관심종목(멀티종목) 시세조회 [국내주식-205]
    pub async fn intstock_multprice(
        &self,
        req: IntstockMultpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST11300006",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/intstock-multprice",
                tr_id,
                req,
            )
            .await
    }

    /// 관심종목 그룹별 종목조회
    /// 관심종목 그룹별 종목조회 [국내주식-203]
    pub async fn intstock_stocklist_by_group(
        &self,
        req: IntstockStocklistByGroupRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKCM113004C6",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group",
                tr_id,
                req,
            )
            .await
    }

    /// 국내기관_외국인 매매종목가집계
    /// 국내기관_외국인 매매종목가집계[국내주식-037]
    pub async fn foreign_institution_total(
        &self,
        req: ForeignInstitutionTotalRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPTJ04400000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/foreign-institution-total",
                tr_id,
                req,
            )
            .await
    }

    /// 외국계 매매종목 가집계
    /// 외국계 매매종목 가집계 [국내주식-161]
    pub async fn frgnmem_trade_estimate(
        &self,
        req: FrgnmemTradeEstimateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST644100C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/frgnmem-trade-estimate",
                tr_id,
                req,
            )
            .await
    }

    /// 종목별 투자자매매동향(일별)
    /// 2021-12-29 11:22:33
    pub async fn investor_trade_by_stock_daily(
        &self,
        req: InvestorTradeByStockDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPTJ04160001",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily",
                tr_id,
                req,
            )
            .await
    }

    /// 시장별 투자자매매동향(시세)
    /// 시장별 투자자매매동향(시세)[v1_국내주식-074]
    pub async fn inquire_investor_time_by_market(
        &self,
        req: InquireInvestorTimeByMarketRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPTJ04030000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market",
                tr_id,
                req,
            )
            .await
    }

    /// 시장별 투자자매매동향(일별)
    /// 시장별 투자자매매동향(일별) [국내주식-075]
    pub async fn inquire_investor_daily_by_market(
        &self,
        req: InquireInvestorDailyByMarketRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPTJ04040000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market",
                tr_id,
                req,
            )
            .await
    }

    /// 종목별 외국계 순매수추이
    /// 종목별 외국계 순매수추이 [국내주식-164]
    pub async fn frgnmem_pchs_trend(
        &self,
        req: FrgnmemPchsTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST644400C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/frgnmem-pchs-trend",
                tr_id,
                req,
            )
            .await
    }

    /// 회원사 실시간 매매동향(틱)
    /// 회원사 실시간 매매동향(틱) [국내주식-163]
    pub async fn frgnmem_trade_trend(
        &self,
        req: FrgnmemTradeTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST04320000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/frgnmem-trade-trend",
                tr_id,
                req,
            )
            .await
    }

    /// 주식현재가 회원사 종목매매동향
    /// 주식현재가 회원사 종목매매동향 [국내주식-197]
    pub async fn inquire_member_daily(
        &self,
        req: InquireMemberDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST04540000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-member-daily",
                tr_id,
                req,
            )
            .await
    }

    /// 종목별 프로그램매매추이(체결)
    /// 종목별 프로그램매매추이(체결)[v1_국내주식-044]
    pub async fn program_trade_by_stock(
        &self,
        req: ProgramTradeByStockRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPPG04650101",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/program-trade-by-stock",
                tr_id,
                req,
            )
            .await
    }

    /// 종목별 프로그램매매추이(일별)
    /// 종목별 프로그램매매추이(일별) [국내주식-113]
    pub async fn program_trade_by_stock_daily(
        &self,
        req: ProgramTradeByStockDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPPG04650201",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily",
                tr_id,
                req,
            )
            .await
    }

    /// 종목별 외인기관 추정가집계
    /// 종목별 외인기관 추정가집계[v1_국내주식-046]
    pub async fn investor_trend_estimate(
        &self,
        req: InvestorTrendEstimateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHPTJ04160200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/investor-trend-estimate",
                tr_id,
                req,
            )
            .await
    }

    /// 종목별일별매수매도체결량
    /// 종목별일별매수매도체결량 [v1_국내주식-056]
    pub async fn inquire_daily_trade_volume(
        &self,
        req: InquireDailyTradeVolumeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03010800",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/inquire-daily-trade-volume",
                tr_id,
                req,
            )
            .await
    }

    /// 프로그램매매 종합현황(시간)
    /// 프로그램매매 종합현황(시간) [국내주식-114]
    pub async fn comp_program_trade_today(
        &self,
        req: CompProgramTradeTodayRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPPG04600101",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/comp-program-trade-today",
                tr_id,
                req,
            )
            .await
    }

    /// 프로그램매매 종합현황(일별)
    /// 프로그램매매 종합현황(일별)[국내주식-115]
    pub async fn comp_program_trade_daily(
        &self,
        req: CompProgramTradeDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPPG04600001",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/comp-program-trade-daily",
                tr_id,
                req,
            )
            .await
    }

    /// 프로그램매매 투자자매매동향(당일)
    /// 프로그램매매 투자자매매동향(당일) [국내주식-116]
    pub async fn investor_program_trade_today(
        &self,
        req: InvestorProgramTradeTodayRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHPPG046600C1",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/investor-program-trade-today",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 신용잔고 일별추이
    /// 국내주식 신용잔고 일별추이[국내주식-110]
    pub async fn daily_credit_balance(
        &self,
        req: DailyCreditBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST04760000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/daily-credit-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 예상체결가 추이
    /// 국내주식 예상체결가 추이[국내주식-118]
    pub async fn exp_price_trend(
        &self,
        req: ExpPriceTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01810000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/exp-price-trend",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 공매도 일별추이
    /// 국내주식 공매도 일별추이[국내주식-134]
    pub async fn daily_short_sale(
        &self,
        req: DailyShortSaleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST04830000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/daily-short-sale",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 체결금액별 매매비중
    /// 국내주식 체결금액별 매매비중 [국내주식-192]
    pub async fn tradprt_byamt(
        &self,
        req: TradprtByamtRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST111900C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/tradprt-byamt",
                tr_id,
                req,
            )
            .await
    }

    /// 국내 증시자금 종합
    /// 국내 증시자금 종합 [국내주식-193]
    pub async fn mktfunds(&self, req: MktfundsRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST649100C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/mktfunds", tr_id, req)
            .await
    }

    /// 종목별 일별 대차거래추이
    /// 종목별 일별 대차거래추이 [국내주식-135]
    pub async fn daily_loan_trans(
        &self,
        req: DailyLoanTransRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHPST074500C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/daily-loan-trans",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 상하한가 포착
    /// 국내주식 상하한가 포착 [국내주식-190]
    pub async fn capture_uplowprice(
        &self,
        req: CaptureUplowpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST130000C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/quotations/capture-uplowprice",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 매물대/거래비중
    /// 국내주식 매물대/거래비중 [국내주식-196]
    pub async fn pbar_tratio(&self, req: PbarTratioRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01130000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/pbar-tratio", tr_id, req)
            .await
    }

    /// 거래량순위
    /// 2021-12-29 11:22:33
    pub async fn volume_rank(
        &self,
        req: VolumeRankNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01710000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/quotations/volume-rank", tr_id, req)
            .await
    }
}

#[allow(non_snake_case)]
impl Finance {
    /// 국내주식 대차대조표
    /// 국내주식 대차대조표[v1_국내주식-078]
    pub async fn balance_sheet(
        &self,
        req: BalanceSheetRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/finance/balance-sheet", tr_id, req)
            .await
    }

    /// 국내주식 손익계산서
    /// 국내주식 손익계산서[v1_국내주식-079]
    pub async fn income_statement(
        &self,
        req: IncomeStatementRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/income-statement",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 재무비율
    /// 국내주식 재무비율[v1_국내주식-080]
    pub async fn financial_ratio(
        &self,
        req: FinancialRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/financial-ratio",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 수익성비율
    /// 국내주식 수익성비율[v1_국내주식-081]
    pub async fn profit_ratio(
        &self,
        req: ProfitRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430400",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/finance/profit-ratio", tr_id, req)
            .await
    }

    /// 국내주식 기타주요비율
    /// 국내주식 기타주요비율[v1_국내주식-082]
    pub async fn other_major_ratios(
        &self,
        req: OtherMajorRatiosRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430500",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/other-major-ratios",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 안정성비율
    /// 국내주식 안정성비율[v1_국내주식-083]
    pub async fn stability_ratio(
        &self,
        req: StabilityRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430600",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/finance/stability-ratio",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 성장성비율
    /// 국내주식 성장성비율[v1_국내주식-085]
    pub async fn growth_ratio(
        &self,
        req: GrowthRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST66430800",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/finance/growth-ratio", tr_id, req)
            .await
    }
}

#[allow(non_snake_case)]
impl Ksdinfo {
    /// 예탁원정보(배당일정)
    /// 예탁원정보(배당일정)[국내주식-145]
    pub async fn dividend(&self, req: DividendRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669102C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/dividend", tr_id, req)
            .await
    }

    /// 예탁원정보(주식매수청구일정)
    /// 예탁원정보(주식매수청구일정)[국내주식-146]
    pub async fn purreq(&self, req: PurreqRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669103C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/purreq", tr_id, req)
            .await
    }

    /// 예탁원정보(합병/분할일정)
    /// 예탁원정보(합병/분할일정)[국내주식-147]
    pub async fn merger_split(
        &self,
        req: MergerSplitRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669104C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/merger-split", tr_id, req)
            .await
    }

    /// 예탁원정보(액면교체일정)
    /// 예탁원정보(액면교체일정)[국내주식-148]
    pub async fn rev_split(&self, req: RevSplitRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669105C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/rev-split", tr_id, req)
            .await
    }

    /// 예탁원정보(자본감소일정)
    /// 예탁원정보(자본감소일정)[국내주식-149]
    pub async fn cap_dcrs(&self, req: CapDcrsRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669106C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/cap-dcrs", tr_id, req)
            .await
    }

    /// 예탁원정보(상장정보일정)
    /// 예탁원정보(상장정보일정)[국내주식-150]
    pub async fn list_info(&self, req: ListInfoRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669107C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/list-info", tr_id, req)
            .await
    }

    /// 예탁원정보(공모주청약일정)
    /// 예탁원정보(공모주청약일정)[국내주식-151]
    pub async fn pub_offer(&self, req: PubOfferRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669108C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/pub-offer", tr_id, req)
            .await
    }

    /// 예탁원정보(실권주일정)
    /// 예탁원정보(실권주일정)[국내주식-152]
    pub async fn forfeit(&self, req: ForfeitRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669109C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/forfeit", tr_id, req)
            .await
    }

    /// 예탁원정보(의무예치일정)
    /// 예탁원정보(의무예치일정)[국내주식-153]
    pub async fn mand_deposit(
        &self,
        req: MandDepositRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669110C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/mand-deposit", tr_id, req)
            .await
    }

    /// 예탁원정보(유상증자일정)
    /// 예탁원정보(유상증자일정) [국내주식-143]
    pub async fn paidin_capin(
        &self,
        req: PaidinCapinRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669100C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/paidin-capin", tr_id, req)
            .await
    }

    /// 예탁원정보(무상증자일정)
    /// 예탁원정보(무상증자일정) [국내주식-144]
    pub async fn bonus_issue(&self, req: BonusIssueRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669101C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/bonus-issue", tr_id, req)
            .await
    }

    /// 예탁원정보(주주총회일정)
    /// 예탁원정보(주주총회일정) [국내주식-154]
    pub async fn sharehld_meet(
        &self,
        req: SharehldMeetRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB669111C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ksdinfo/sharehld-meet", tr_id, req)
            .await
    }
}

#[allow(non_snake_case)]
impl Ranking {
    /// 국내주식 시간외예상체결등락률
    /// 국내주식 시간외예상체결등락률 [국내주식-140]
    pub async fn overtime_exp_trans_fluct(
        &self,
        req: OvertimeExpTransFluctRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST11860000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/overtime-exp-trans-fluct",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 등락률 순위
    /// 국내주식 등락률 순위[v1_국내주식-088]
    pub async fn fluctuation(
        &self,
        req: FluctuationRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01700000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/fluctuation", tr_id, req)
            .await
    }

    /// 국내주식 호가잔량 순위
    /// 국내주식 호가잔량 순위[국내주식-089]
    pub async fn quote_balance(
        &self,
        req: QuoteBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01720000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/quote-balance", tr_id, req)
            .await
    }

    /// 국내주식 수익자산지표 순위
    /// 국내주식 수익자산지표 순위[v1_국내주식-090]
    pub async fn profit_asset_index(
        &self,
        req: ProfitAssetIndexRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01730000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/profit-asset-index",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 시가총액 상위
    /// 국내주식 시가총액 상위[v1_국내주식-091]
    pub async fn market_cap(&self, req: MarketCapRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01740000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/market-cap", tr_id, req)
            .await
    }

    /// 국내주식 재무비율 순위
    /// 국내주식 재무비율 순위[v1_국내주식-092]
    pub async fn finance_ratio(
        &self,
        req: FinanceRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01750000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/finance-ratio", tr_id, req)
            .await
    }

    /// 국내주식 시간외잔량 순위
    /// 국내주식 시간외잔량 순위[v1_국내주식-093]
    pub async fn after_hour_balance(
        &self,
        req: AfterHourBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01760000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/after-hour-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 우선주/괴리율 상위
    /// 국내주식 우선주/괴리율 상위[v1_국내주식-094]
    pub async fn prefer_disparate_ratio(
        &self,
        req: PreferDisparateRatioRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01770000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/prefer-disparate-ratio",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 이격도 순위
    /// 국내주식 이격도 순위[v1_국내주식-095]
    pub async fn disparity(&self, req: DisparityRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01780000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/disparity", tr_id, req)
            .await
    }

    /// 국내주식 시장가치 순위
    /// 국내주식 시장가치 순위[v1_국내주식-096]
    pub async fn market_value(
        &self,
        req: MarketValueRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01790000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/market-value", tr_id, req)
            .await
    }

    /// 국내주식 체결강도 상위
    /// 국내주식 체결강도 상위[v1_국내주식-101]
    pub async fn volume_power(
        &self,
        req: VolumePowerRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01680000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/volume-power", tr_id, req)
            .await
    }

    /// 국내주식 관심종목등록 상위
    /// 국내주식 관심종목등록 상위[v1_국내주식-102]
    pub async fn top_interest_stock(
        &self,
        req: TopInterestStockRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01800000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/top-interest-stock",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 예상체결 상승/하락상위
    /// 국내주식 예상체결 상승/하락상위[v1_국내주식-103]
    pub async fn exp_trans_updown(
        &self,
        req: ExpTransUpdownRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01820000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/exp-trans-updown",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 당사매매종목 상위
    /// 국내주식 당사매매종목 상위[v1_국내주식-104]
    pub async fn traded_by_company(
        &self,
        req: TradedByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01860000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/traded-by-company",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 신고/신저근접종목 상위
    /// 국내주식 신고/신저근접종목 상위[v1_국내주식-105]
    pub async fn near_new_highlow(
        &self,
        req: NearNewHighlowRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST01870000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/near-new-highlow",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 배당률 상위
    /// 국내주식 배당률 상위[국내주식-106]
    pub async fn dividend_rate(
        &self,
        req: DividendRateRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHKDB13470100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/dividend-rate", tr_id, req)
            .await
    }

    /// 국내주식 대량체결건수 상위
    /// 국내주식 대량체결건수 상위[국내주식-107]
    pub async fn bulk_trans_num(
        &self,
        req: BulkTransNumRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST190900C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/bulk-trans-num", tr_id, req)
            .await
    }

    /// 국내주식 신용잔고 상위
    /// 국내주식 신용잔고 상위[국내주식-109]
    pub async fn credit_balance(
        &self,
        req: CreditBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST17010000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/credit-balance", tr_id, req)
            .await
    }

    /// 국내주식 공매도 상위종목
    /// 국내주식 공매도 상위종목[국내주식-133]
    pub async fn short_sale(&self, req: ShortSaleRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST04820000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/short-sale", tr_id, req)
            .await
    }

    /// 국내주식 시간외등락율순위
    /// 국내주식 시간외등락율순위 [국내주식-138]
    pub async fn overtime_fluctuation(
        &self,
        req: OvertimeFluctuationRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02340000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/overtime-fluctuation",
                tr_id,
                req,
            )
            .await
    }

    /// 국내주식 시간외거래량순위
    /// 국내주식 시간외거래량순위 [국내주식-139]
    pub async fn overtime_volume(
        &self,
        req: OvertimeVolumeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02350000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-stock/v1/ranking/overtime-volume",
                tr_id,
                req,
            )
            .await
    }

    /// HTS조회상위20종목
    /// HTS조회상위20종목 [국내주식-214]
    pub async fn hts_top_view(&self, req: ()) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHMCM000100C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-stock/v1/ranking/hts-top-view", tr_id, req)
            .await
    }
}
