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
    ///
    /// - TR_ID: Real=TTTC0011U / VTS=VTTC0011U
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-cash
    ///
    /// 국내주식 현금 주문을 수행하는 API입니다. (매수/매도)
    ///
    /// # Example (Scraped)
    /// 국내주식주문(현금) API 입니다.
    ///
    /// ※ TTC0802U(현금매수) 사용하셔서 미수매수 가능합니다. 단, 거래하시는 계좌가 증거금40%계좌로 신청이 되어있어야 가능합니다.
    /// ※ 신용매수는 별도의 API가 준비되어 있습니다.
    ///
    /// ※ ORD_QTY(주문수량), ORD_UNPR(주문단가) 등을 String으로 전달해야 함에 유의 부탁드립니다.
    ///
    /// ※ ORD_UNPR(주문단가)가 없는 주문은 상한가로 주문금액을 선정하고 이후 체결이되면 체결금액로 정산됩니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
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
    ///
    /// - TR_ID: Real=TTTC0051U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-credit
    ///
    /// [국내주식] 주문/계좌
    /// 주식주문(신용)[v1_국내주식-002]
    /// krx_fwdg_ord_orgno
    /// ord_tmd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식주문(신용) API입니다.
    /// ※ 모의투자는 사용 불가합니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
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
    ///
    /// - TR_ID: Real=TTTC0013U / VTS=VTTC0013U
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-rvsecncl
    ///
    /// [국내주식] 주문/계좌
    /// 주식주문(정정취소)[v1_국내주식-003]
    /// krx_fwdg_ord_orgno
    /// ord_tmd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주문 건에 대하여 정정 및 취소하는 API입니다. 단, 이미 체결된 건은 정정 및 취소가 불가합니다.
    ///
    /// ※ 정정은 원주문에 대한 주문단가 혹은 주문구분을 변경하는 사항으로, 정정이 가능한 수량은 원주문수량을 초과 할 수 없습니다.
    ///
    /// ※ 주식주문(정정취소) 호출 전에 반드시 주식정정취소가능주문조회 호출을 통해 정정취소가능수량(output > psbl_qty)을 확인하신 후 정정취소주문 내시기 바랍니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
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
    ///
    /// - TR_ID: Real=TTTC0084R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl
    ///
    /// [국내주식] 주문/계좌
    /// 주식정정취소가능주문조회[v1_국내주식-004]
    /// ord_gno_brno
    /// orgn_odno
    /// ord_dvsn_name
    /// prdt_name
    /// rvse_cncl_dvsn_name
    /// ord_qty
    /// ord_unpr
    /// ord_tmd
    /// tot_ccld_qty
    /// tot_ccld_amt
    /// psbl_qty
    /// sll_buy_dvsn_cd
    /// ord_dvsn_cd
    /// mgco_aptm_odno
    /// excg_dvsn_cd
    /// excg_id_dvsn_cd
    /// excg_id_dvsn_name
    /// stpm_cndt_pric
    /// stpm_efct_occr_yn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식정정취소가능주문조회 API입니다. 한 번의 호출에 최대 50건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    ///
    /// ※ 주식주문(정정취소) 호출 전에 반드시 주식정정취소가능주문조회 호출을 통해 정정취소가능수량(output > psbl_qty)을 확인하신 후 정정취소주문 내시기 바랍니다.
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
    ///
    /// - TR_ID: Real=TTTC0081R / VTS=VTTC0081R
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-daily-ccld
    ///
    /// [국내주식] 주문/계좌
    /// 주식일별주문체결조회[v1_국내주식-005]
    /// ord_dt
    /// ord_gno_brno
    /// orgn_odno
    /// ord_dvsn_name
    /// sll_buy_dvsn_cd
    /// sll_buy_dvsn_cd_name
    /// prdt_name
    /// ord_qty
    /// ord_unpr
    /// ord_tmd
    /// tot_ccld_qty
    /// avg_prvs
    /// cncl_yn
    /// tot_ccld_amt
    /// loan_dt
    /// ordr_empno
    /// ord_dvsn_cd
    /// cnc_cfrm_qty
    /// rmn_qty
    /// rjct_qty
    /// ccld_cndt_name
    /// inqr_ip_addr
    /// cpbc_ordp_ord_rcit_dvsn_cd
    /// cpbc_ordp_infm_mthd_dvsn_cd
    /// infm_tmd
    /// ctac_tlno
    /// prdt_type_cd
    /// excg_dvsn_cd
    /// cpbc_ordp_mtrl_dvsn_cd
    /// ord_orgno
    /// rsvn_ord_end_dt
    /// excg_id_dvsn_Cd
    /// stpm_cndt_pric
    /// stpm_efct_occr_dtmd
    /// tot_ord_qty
    /// tot_ccld_qty
    /// tot_ccld_amt
    /// prsm_tlex_smtl
    /// pchs_avg_pric
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식일별주문체결조회 API입니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    /// 모의계좌의 경우, 한 번의 호출에 최대 15건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    ///
    /// * 다만, 3개월 이전 체결내역 조회(CTSC9115R) 의 경우,
    /// 장중에는 많은 거래량으로 인해 순간적으로 DB가 밀렸거나 응답을 늦게 받거나 하는 등의 이슈가 있을 수 있어
    /// ① 가급적 장 종료 이후(15:30 이후) 조회하시고
    /// ② 조회기간(INQR_STRT_DT와 INQR_END_DT 사이의 간격)을 보다 짧게 해서 조회하는 것을
    /// 권유드립니다.
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
    ///
    /// - TR_ID: Real=TTTC8434R / VTS=VTTC8434R
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-balance
    ///
    /// [국내주식] 주문/계좌
    /// 주식잔고조회[v1_국내주식-006]
    /// prdt_name
    /// trad_dvsn_name
    /// bfdy_buy_qty
    /// bfdy_sll_qty
    /// thdt_buyqty
    /// thdt_sll_qty
    /// hldg_qty
    /// ord_psbl_qty
    /// pchs_avg_pric
    /// pchs_amt
    /// evlu_amt
    /// evlu_pfls_amt
    /// evlu_pfls_rt
    /// evlu_erng_rt
    /// loan_dt
    /// loan_amt
    /// stln_slng_chgs
    /// expd_dt
    /// fltt_rt
    /// bfdy_cprs_icdc
    /// item_mgna_rt_name
    /// grta_rt_name
    /// sbst_pric
    /// stck_loan_unpr
    /// dnca_tot_amt
    /// nxdy_excc_amt
    /// prvs_rcdl_excc_amt
    /// cma_evlu_amt
    /// bfdy_buy_amt
    /// thdt_buy_amt
    /// nxdy_auto_rdpt_amt
    /// bfdy_sll_amt
    /// thdt_sll_amt
    /// d2_auto_rdpt_amt
    /// bfdy_tlex_amt
    /// thdt_tlex_amt
    /// tot_loan_amt
    /// scts_evlu_amt
    /// tot_evlu_amt
    /// nass_amt
    /// fncg_gld_auto_rdpt_yn
    /// pchs_amt_smtl_amt
    /// evlu_amt_smtl_amt
    /// evlu_pfls_smtl_amt
    /// tot_stln_slng_chgs
    /// bfdy_tot_asst_evlu_amt
    /// asst_icdc_amt
    /// asst_icdc_erng_rt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식 잔고조회 API입니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 50건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    /// 모의계좌의 경우, 한 번의 호출에 최대 20건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    ///
    /// * 당일 전량매도한 잔고도 보유수량 0으로 보여질 수 있으나, 해당 보유수량 0인 잔고는 최종 D-2일 이후에는 잔고에서 사라집니다.
    ///
    /// ※ 중요 : 해당 API는 제공 정보량이 많아 조회속도가 느린 API입니다. 주문 준비를 위해서는 주식매수/매도가능수량 조회 TR 사용을 권장 드립니다.
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
    ///
    /// - TR_ID: Real=TTTC8908R / VTS=VTTC8908R
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-order
    ///
    /// [국내주식] 주문/계좌
    /// 매수가능조회[v1_국내주식-007]
    /// ord_psbl_cash
    /// ord_psbl_sbst
    /// ruse_psbl_amt
    /// fund_rpch_chgs
    /// psbl_qty_calc_unpr
    /// nrcvb_buy_amt
    /// nrcvb_buy_qty
    /// max_buy_amt
    /// max_buy_qty
    /// cma_evlu_amt
    /// ovrs_re_use_amt_wcrc
    /// ord_psbl_frcr_amt_wcrc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 매수가능 조회 API입니다.
    /// 실전계좌/모의계좌의 경우, 한 번의 호출에 최대 1건까지 확인 가능합니다.
    ///
    ///
    /// 1) 매수가능금액 확인
    /// . 미수 사용 X: nrcvb_buy_amt(미수없는매수금액) 확인
    /// . 미수 사용 O: max_buy_amt(최대매수금액) 확인
    ///
    ///
    /// 2) 매수가능수량 확인
    /// . 특정 종목 전량매수 시 가능수량을 확인하실 경우 ORD_DVSN:00(지정가)는 종목증거금율이 반영되지 않습니다.
    /// 따라서 "반드시" ORD_DVSN:01(시장가)로 지정하여 종목증거금율이 반영된 가능수량을 확인하시기 바랍니다.
    ///
    /// (다만, 조건부지정가 등 특정 주문구분(ex.IOC)으로 주문 시 가능수량을 확인할 경우 주문 시와 동일한 주문구분(ex.IOC) 입력하여 가능수량 확인)
    ///
    /// . 미수 사용 X: ORD_DVSN:01(시장가) or 특정 주문구분(ex.IOC)로 지정하여 nrcvb_buy_qty(미수없는매수수량) 확인
    /// . 미수 사용 O: ORD_DVSN:01(시장가) or 특정 주문구분(ex.IOC)로 지정하여 max_buy_qty(최대매수수량) 확인
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
    ///
    /// - TR_ID: Real=TTTC8408R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-psbl-sell
    ///
    /// [국내주식] 주문/계좌
    /// 매도가능수량조회 [국내주식-165]
    /// prdt_name
    /// buy_qty
    /// sll_qty
    /// cblc_qty
    /// nsvg_qty
    /// ord_psbl_qty
    /// pchs_avg_pric
    /// pchs_amt
    /// now_pric
    /// evlu_amt
    /// evlu_pfls_amt
    /// evlu_pfls_rt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 매도가능수량조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0971] 주식 매도 화면에서 종목코드 입력 후 "가능" 클릭 시 매도가능수량이 확인되는 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 특정종목 매도가능수량 확인 시, 매도주문 내시려는 주문종목(PDNO)으로 API 호출 후
    /// output > ord_psbl_qty(주문가능수량) 확인하실 수 있습니다.
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
    ///
    /// - TR_ID: Real=TTTC8909R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-credit-psamount
    ///
    /// [국내주식] 주문/계좌
    /// 신용매수가능조회[v1_국내주식-042]
    /// ord_psbl_cash
    /// ord_psbl_sbst
    /// ruse_psbl_amt
    /// fund_rpch_chgs
    /// psbl_qty_calc_unpr
    /// nrcvb_buy_amt
    /// nrcvb_buy_qty
    /// max_buy_amt
    /// max_buy_qty
    /// cma_evlu_amt
    /// ovrs_re_use_amt_wcrc
    /// ord_psbl_frcr_amt_wcrc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 신용매수가능조회 API입니다.
    /// 신용매수주문 시 주문가능수량과 금액을 확인하실 수 있습니다.
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
    ///
    /// - TR_ID: Real=CTSC0008U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-resv
    ///
    /// [국내주식] 주문/계좌
    /// 주식예약주문[v1_국내주식-017]
    /// rsvn_ord_seq
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 예약주문 매수/매도 API 입니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 유의사항
    /// 1. 예약주문 가능시간 : 15시 40분 ~ 다음 영업일 7시 30분
    /// (단, 서버 초기화 작업 시 예약주문 불가 : 23시 40분 ~ 00시 10분)
    /// ※ 예약주문 처리내역은 통보되지 않으므로 주문처리일 장 시작전에 반드시 주문처리 결과를 확인하시기 바랍니다.
    ///
    /// 2. 예약주문 안내
    /// - 예약종료일 미입력 시 일반예약주문으로 최초 도래하는 영업일에 주문 전송됩니다.
    /// - 예약종료일 입력 시 기간예약주문으로 최초 예약주문수량 중 미체결 된 수량에 대해 예약종료일까지 매 영업일 주문이
    /// 실행됩니다. (예약종료일은 익영업일부터 달력일 기준으로 공휴일 포함하여 최대 30일이 되는 일자까지 입력가능)
    /// - 예약주문 접수 처리순서는 일반/기간예약주문 중 신청일자가 빠른 주문이 우선합니다.
    /// 단, 기간예약주문 자동배치시간(약 15시35분 ~ 15시55분)사이 접수되는 주문의 경우 당일에 한해 순서와 상관없이
    /// 처리될 수 있습니다.
    /// - 기간예약주문 자동배치시간(약 15시35분 ~ 15시55분)에는 예약주문 조회가 제한 될 수 있습니다.
    /// - 기간예약주문은 계좌 당 주문건수 최대 1,000건으로 제한됩니다.
    ///
    /// 3. 예약주문 접수내역 중 아래의 사유 등으로 인해 주문이 거부될 수 있사오니, 주문처리일 장 시작전에 반드시
    /// 주문처리 결과를 확인하시기 바랍니다.
    /// * 주문처리일 기준 : 매수가능금액 부족, 매도가능수량 부족, 주문수량/호가단위 오류, 대주 호가제한,
    /// 신용/대주가능종목 변경, 상/하한폭 변경, 시가형성 종목(신규상장 등)의 시장가, 거래서비스 미신청 등
    ///
    /// 4. 익일 예상 상/하한가는 조회시점의 현재가로 계산되며 익일의 유/무상증자, 배당, 감자, 합병, 액면변경 등에 의해
    /// 변동될 수 있으며 이로 인해 상/하한가를 벗어나 주문이 거부되는 경우가 발생할 수 있사오니, 주문처리일 장 시작전에
    /// 반드시 주문처리결과를 확인하시기 바랍니다.
    ///
    /// 5. 정리매매종목, ELW, 신주인수권증권, 신주인수권증서 등은 가격제한폭(상/하한가) 적용 제외됩니다.
    ///
    /// 6. 영업일 장 시작 후 [기간예약주문] 내역 취소는 해당시점 이후의 예약주문이 취소되는 것으로,
    /// 일반주문으로 이미 전환된 주문에는 영향을 미치지 않습니다. 반드시 장 시작전 주문처리결과를 확인하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=CTSC0009U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-resv-rvsecncl
    ///
    /// [국내주식] 주문/계좌
    /// 주식예약주문정정취소[v1_국내주식-018,019]
    /// nrml_prcs_yn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 예약주문 정정/취소 API 입니다.
    /// *  정정주문은 취소주문에 비해 필수 입력값이 추가 됩니다.
    /// 하단의 입력값을 참조하시기 바랍니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
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
    ///
    /// - TR_ID: Real=CTSC0004R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/order-resv-ccnl
    ///
    /// [국내주식] 주문/계좌
    /// 주식예약주문조회[v1_국내주식-020]
    /// rsvn_ord_seq
    /// rsvn_ord_ord_dt
    /// rsvn_ord_rcit_dt
    /// ord_dvsn_cd
    /// ord_rsvn_qty
    /// tot_ccld_qty
    /// cncl_ord_dt
    /// ord_tmd
    /// ctac_tlno
    /// rjct_rson2
    /// rsvn_ord_rcit_tmd
    /// kor_item_shtn_name
    /// sll_buy_dvsn_cd
    /// ord_rsvn_unpr
    /// tot_ccld_amt
    /// loan_dt
    /// cncl_rcit_tmd
    /// prcs_rslt
    /// ord_dvsn_name
    /// tmnl_mdia_kind_cd
    /// rsvn_end_dt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내예약주문 처리내역 조회 API 입니다.
    /// 실전계좌/모의계좌의 경우, 한 번의 호출에 최대 20건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
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
    ///
    /// - TR_ID: Real=TTTC2202R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-present-balance
    ///
    /// [국내주식] 주문/계좌
    /// 퇴직연금 체결기준잔고[v1_국내주식-032]
    /// cblc_dvsn
    /// cblc_dvsn_name
    /// prdt_name
    /// hldg_qty
    /// slpsb_qty
    /// pchs_avg_pric
    /// evlu_pfls_amt
    /// evlu_pfls_rt
    /// evlu_amt
    /// pchs_amt
    /// cblc_weit
    /// pchs_amt_smtl_amt
    /// evlu_amt_smtl_amt
    /// evlu_pfls_smtl_amt
    /// trad_pfls_smtl
    /// thdt_tot_pfls_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
    /// KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.
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
    ///
    /// - TR_ID: Real=TTTC2201R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld
    ///
    /// [국내주식] 주문/계좌
    /// 퇴직연금 미체결내역[v1_국내주식-033]
    /// ord_gno_brno
    /// sll_buy_dvsn_cd
    /// trad_dvsn_name
    /// prdt_name
    /// ord_unpr
    /// ord_qty
    /// tot_ccld_qty
    /// nccs_qty
    /// ord_dvsn_cd
    /// ord_dvsn_name
    /// orgn_odno
    /// ord_tmd
    /// objt_cust_dvsn_name
    /// pchs_avg_pric
    /// stpm_cndt_pric
    /// stpm_efct_occr_dtmd
    /// stpm_efct_occr_yn
    /// excg_id_dvsn_cd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
    /// KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.
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
    ///
    /// - TR_ID: Real=TTTC0503R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-psbl-order
    ///
    /// [국내주식] 주문/계좌
    /// 퇴직연금 매수가능조회[v1_국내주식-034]
    /// ord_psbl_cash
    /// ruse_psbl_amt
    /// psbl_qty_calc_unpr
    /// max_buy_amt
    /// max_buy_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
    /// KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.
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
    ///
    /// - TR_ID: Real=TTTC0506R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-deposit
    ///
    /// [국내주식] 주문/계좌
    /// 퇴직연금 예수금조회[v1_국내주식-035]
    /// dnca_tota
    /// nxdy_excc_amt
    /// nxdy_sttl_amt
    /// nx2_day_sttl_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
    /// KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.
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
    ///
    /// - TR_ID: Real=TTTC2208R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/pension/inquire-balance
    ///
    /// [국내주식] 주문/계좌
    /// 퇴직연금 잔고조회[v1_국내주식-036]
    /// cblc_dvsn_name
    /// prdt_name
    /// item_dvsn_name
    /// thdt_buyqty
    /// thdt_sll_qty
    /// hldg_qty
    /// ord_psbl_qty
    /// pchs_avg_pric
    /// pchs_amt
    /// evlu_amt
    /// evlu_pfls_amt
    /// evlu_erng_rt
    /// dnca_tot_amt
    /// nxdy_excc_amt
    /// prvs_rcdl_excc_amt
    /// thdt_buy_amt
    /// thdt_sll_amt
    /// thdt_tlex_amt
    /// scts_evlu_amt
    /// tot_evlu_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식, ETF, ETN만 조회 가능하며 펀드는 조회 불가합니다.
    ///
    /// ​※ 55번 계좌(DC가입자계좌)의 경우 해당 API 이용이 불가합니다.
    /// KIS Developers API의 경우 HTS ID에 반드시 연결되어있어야만 API 신청 및 앱정보 발급이 가능한 서비스로 개발되어서 실물계좌가 아닌 55번 계좌는 API 이용이 불가능한 점 양해 부탁드립니다.
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
    ///
    /// - TR_ID: Real=TTTC8494R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-balance-rlz-pl
    ///
    /// [국내주식] 주문/계좌
    /// 주식잔고조회_실현손익[v1_국내주식-041]
    /// prdt_name
    /// trad_dvsn_name
    /// bfdy_buy_qty
    /// bfdy_sll_qty
    /// thdt_buyqty
    /// thdt_sll_qty
    /// hldg_qty
    /// ord_psbl_qty
    /// pchs_avg_pric
    /// pchs_amt
    /// evlu_amt
    /// evlu_pfls_amt
    /// evlu_pfls_rt
    /// evlu_erng_rt
    /// loan_dt
    /// loan_amt
    /// stln_slng_chgs
    /// expd_dt
    /// stck_loan_unpr
    /// bfdy_cprs_icdc
    /// fltt_rt
    /// dnca_tot_amt
    /// nxdy_excc_amt
    /// prvs_rcdl_excc_amt
    /// cma_evlu_amt
    /// bfdy_buy_amt
    /// thdt_buy_amt
    /// nxdy_auto_rdpt_amt
    /// bfdy_sll_amt
    /// thdt_sll_amt
    /// d2_auto_rdpt_amt
    /// bfdy_tlex_amt
    /// thdt_tlex_amt
    /// tot_loan_amt
    /// scts_evlu_amt
    /// tot_evlu_amt
    /// nass_amt
    /// fncg_gld_auto_rdpt_yn
    /// pchs_amt_smtl_amt
    /// evlu_amt_smtl_amt
    /// evlu_pfls_smtl_amt
    /// tot_stln_slng_chgs
    /// bfdy_tot_asst_evlu_amt
    /// asst_icdc_amt
    /// asst_icdc_erng_rt
    /// rlzt_pfls
    /// rlzt_erng_rt
    /// real_evlu_pfls
    /// real_evlu_pfls_erng_rt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식잔고조회_실현손익 API입니다.
    /// 한국투자 HTS(eFriend Plus) [0800] 국내 체결기준잔고 화면을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// (참고: 포럼 - 공지사항 - 신규 API 추가 안내(주식잔고조회_실현손익 외 1건))
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
    ///
    /// - TR_ID: Real=CTRP6548R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-account-balance
    ///
    /// [국내주식] 주문/계좌
    /// 투자계좌자산현황조회[v1_국내주식-048]
    /// pchs_amt
    /// evlu_amt
    /// evlu_pfls_amt
    /// crdt_lnd_amt
    /// real_nass_amt
    /// whol_weit_rt
    /// pchs_amt_smtl
    /// nass_tot_amt
    /// loan_amt_smtl
    /// evlu_pfls_amt_smtl
    /// evlu_amt_smtl
    /// tot_asst_amt
    /// tot_lnda_tot_ulst_lnda
    /// cma_auto_loan_amt
    /// tot_mgln_amt
    /// stln_evlu_amt
    /// crdt_fncg_amt
    /// ocl_apl_loan_amt
    /// pldg_stup_amt
    /// frcr_evlu_tota
    /// tot_dncl_amt
    /// cma_evlu_amt
    /// dncl_amt
    /// tot_sbst_amt
    /// thdt_rcvb_amt
    /// ovrs_stck_evlu_amt1
    /// ovrs_bond_evlu_amt
    /// mmf_cma_mgge_loan_amt
    /// sbsc_dncl_amt
    /// pbst_sbsc_fnds_loan_use_amt
    /// etpr_crdt_grnt_loan_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 투자계좌자산현황조회 API입니다.
    ///
    /// output1은 한국투자 HTS(eFriend Plus) > [0891] 계좌 자산비중(결제기준) 화면 아래 테이블의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=TTTC8708R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-period-profit
    ///
    /// [국내주식] 주문/계좌
    /// 기간별손익일별합산조회[v1_국내주식-052]
    /// trad_dt
    /// buy_amt
    /// sll_amt
    /// rlzt_pfls
    /// loan_int
    /// tl_tax
    /// pfls_rt
    /// sll_qty1
    /// buy_qty1
    /// sll_qty_smtl
    /// sll_tr_amt_smtl
    /// sll_fee_smtl
    /// sll_tltx_smtl
    /// sll_excc_amt_smtl
    /// buy_qty_smtl
    /// buy_tr_amt_smtl
    /// buy_fee_smtl
    /// buy_tax_smtl
    /// buy_excc_amt_smtl
    /// tot_qty
    /// tot_tr_amt
    /// tot_fee
    /// tot_tltx
    /// tot_excc_amt
    /// tot_rlzt_pfls
    /// loan_int
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 기간별손익일별합산조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0856] 기간별 매매손익 화면 에서 "일별" 클릭 시의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=TTTC8715R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/inquire-period-trade-profit
    ///
    /// [국내주식] 주문/계좌
    /// 기간별매매손익현황조회[v1_국내주식-060]
    /// trad_dt
    /// prdt_name
    /// trad_dvsn_name
    /// loan_dt
    /// hldg_qty
    /// pchs_unpr
    /// buy_qty
    /// buy_amt
    /// sll_pric
    /// sll_qty
    /// sll_amt
    /// rlzt_pfls
    /// pfls_rt
    /// tl_tax
    /// loan_int
    /// sll_qty_smtl
    /// sll_tr_amt_smtl
    /// sll_fee_smtl
    /// sll_tltx_smtl
    /// sll_excc_amt_smtl
    /// buyqty_smtl
    /// buy_tr_amt_smtl
    /// buy_fee_smtl
    /// buy_tax_smtl
    /// buy_excc_amt_smtl
    /// tot_qty
    /// tot_tr_amt
    /// tot_fee
    /// tot_tltx
    /// tot_excc_amt
    /// tot_rlzt_pfls
    /// loan_int
    /// tot_pftrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 기간별매매손익현황조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0856] 기간별 매매손익 화면 에서 "종목별" 클릭 시의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=TTTC0869R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/intgr-margin
    ///
    /// [국내주식] 주문/계좌
    /// 주식통합증거금 현황 [국내주식-191]
    /// acmga_rt
    /// acmga_pct100_aptm_rson
    /// stck_cash_objt_amt
    /// stck_sbst_objt_amt
    /// stck_evlu_objt_amt
    /// stck_ruse_psbl_objt_amt
    /// stck_fund_rpch_chgs_objt_amt
    /// stck_fncg_rdpt_objt_atm
    /// bond_ruse_psbl_objt_amt
    /// stck_cash_use_amt
    /// stck_sbst_use_amt
    /// stck_evlu_use_amt
    /// stck_ruse_psbl_amt_use_amt
    /// stck_fund_rpch_chgs_use_amt
    /// stck_fncg_rdpt_amt_use_amt
    /// bond_ruse_psbl_amt_use_amt
    /// stck_cash_ord_psbl_amt
    /// stck_sbst_ord_psbl_amt
    /// stck_evlu_ord_psbl_amt
    /// stck_ruse_psbl_ord_psbl_amt
    /// stck_fund_rpch_ord_psbl_amt
    /// bond_ruse_psbl_ord_psbl_amt
    /// rcvb_amt
    /// stck_loan_grta_ruse_psbl_amt
    /// stck_cash20_max_ord_psbl_amt
    /// stck_cash30_max_ord_psbl_amt
    /// stck_cash40_max_ord_psbl_amt
    /// stck_cash50_max_ord_psbl_amt
    /// stck_cash60_max_ord_psbl_amt
    /// stck_cash100_max_ord_psbl_amt
    /// stck_rsip100_max_ord_psbl_amt
    /// bond_max_ord_psbl_amt
    /// stck_fncg45_max_ord_psbl_amt
    /// stck_fncg50_max_ord_psbl_amt
    /// stck_fncg60_max_ord_psbl_amt
    /// stck_fncg70_max_ord_psbl_amt
    /// stck_stln_max_ord_psbl_amt
    /// lmt_amt
    /// ovrs_stck_itgr_mgna_dvsn_name
    /// usd_objt_amt
    /// usd_use_amt
    /// usd_ord_psbl_amt
    /// hkd_objt_amt
    /// hkd_use_amt
    /// hkd_ord_psbl_amt
    /// jpy_objt_amt
    /// jpy_use_amt
    /// jpy_ord_psbl_amt
    /// cny_objt_amt
    /// cny_use_amt
    /// cny_ord_psbl_amt
    /// usd_ruse_objt_amt
    /// usd_ruse_amt
    /// usd_ruse_ord_psbl_amt
    /// hkd_ruse_objt_amt
    /// hkd_ruse_amt
    /// hkd_ruse_ord_psbl_amt
    /// jpy_ruse_objt_amt
    /// jpy_ruse_amt
    /// jpy_ruse_ord_psbl_amt
    /// cny_ruse_objt_amt
    /// cny_ruse_amt
    /// cny_ruse_ord_psbl_amt
    /// usd_gnrl_ord_psbl_amt
    /// usd_itgr_ord_psbl_amt
    /// hkd_gnrl_ord_psbl_amt
    /// hkd_itgr_ord_psbl_amt
    /// jpy_gnrl_ord_psbl_amt
    /// jpy_itgr_ord_psbl_amt
    /// cny_gnrl_ord_psbl_amt
    /// cny_itgr_ord_psbl_amt
    /// stck_itgr_cash20_ord_psbl_amt
    /// stck_itgr_cash30_ord_psbl_amt
    /// stck_itgr_cash40_ord_psbl_amt
    /// stck_itgr_cash50_ord_psbl_amt
    /// stck_itgr_cash60_ord_psbl_amt
    /// stck_itgr_cash100_ord_psbl_amt
    /// stck_itgr_100_ord_psbl_amt
    /// stck_itgr_fncg45_ord_psbl_amt
    /// stck_itgr_fncg50_ord_psbl_amt
    /// stck_itgr_fncg60_ord_psbl_amt
    /// stck_itgr_fncg70_ord_psbl_amt
    /// stck_itgr_stln_ord_psbl_amt
    /// bond_itgr_ord_psbl_amt
    /// stck_cash_ovrs_use_amt
    /// stck_sbst_ovrs_use_amt
    /// stck_evlu_ovrs_use_amt
    /// stck_re_use_amt_ovrs_use_amt
    /// stck_fund_rpch_ovrs_use_amt
    /// stck_fncg_rdpt_ovrs_use_amt
    /// bond_re_use_ovrs_use_amt
    /// usd_oth_mket_use_amt
    /// jpy_oth_mket_use_amt
    /// cny_oth_mket_use_amt
    /// hkd_oth_mket_use_amt
    /// usd_re_use_oth_mket_use_amt
    /// jpy_re_use_oth_mket_use_amt
    /// cny_re_use_oth_mket_use_amt
    /// hkd_re_use_oth_mket_use_amt
    /// hgkg_cny_re_use_amt
    /// usd_frst_bltn_exrt
    /// hkd_frst_bltn_exrt
    /// jpy_frst_bltn_exrt
    /// cny_frst_bltn_exrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식통합증거금 현황 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0867] 통합증거금조회 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 해당 화면은 일반계좌와 통합증거금 신청계좌에 대해서 국내 및 해외 주문가능금액을 간단하게 조회하는 화면입니다.
    /// ※ 해외 국가별 상세한 증거금현황을 원하시면 [해외주식] 주문/계좌 > 해외증거금 통화별조회 API를 이용하여 주시기 바랍니다.
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
    ///
    /// - TR_ID: Real=CTRGA011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/trading/period-rights
    ///
    /// [국내주식] 주문/계좌
    /// 기간별계좌권리현황조회 [국내주식-211]
    /// acno10
    /// rght_type_cd
    /// bass_dt
    /// rght_cblc_type_cd
    /// rptt_pdno
    /// prdt_type_cd
    /// shtn_pdno
    /// prdt_name
    /// cblc_qty
    /// last_alct_qty
    /// excs_alct_qty
    /// tot_alct_qty
    /// last_ftsk_qty
    /// last_alct_amt
    /// last_ftsk_chgs
    /// rdpt_prca
    /// dlay_int_amt
    /// lstg_dt
    /// sbsc_end_dt
    /// cash_dfrm_dt
    /// rqst_qty
    /// rqst_amt
    /// rqst_dt
    /// rfnd_dt
    /// rfnd_amt
    /// lstg_stqt
    /// tax_amt
    /// sbsc_unpr
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 기간별계좌권리현황조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7344] 권리유형별 현황조회 화면을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST01010100 / VTS=FHKST01010100
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-price
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 시세[v1_국내주식-008]
    /// iscd_stat_cls_code
    /// marg_rate
    /// rprs_mrkt_kor_name
    /// new_hgpr_lwpr_cls_code
    /// bstp_kor_isnm
    /// temp_stop_yn
    /// oprc_rang_cont_yn
    /// clpr_rang_cont_yn
    /// crdt_able_yn
    /// grmn_rate_cls_code
    /// elw_pblc_yn
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_tr_pbmn
    /// acml_vol
    /// prdy_vrss_vol_rate
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// stck_mxpr
    /// stck_llam
    /// stck_sdpr
    /// wghn_avrg_stck_prc
    /// hts_frgn_ehrt
    /// frgn_ntby_qty
    /// pgtr_ntby_qty
    /// pvt_scnd_dmrs_prc
    /// pvt_frst_dmrs_prc
    /// pvt_pont_val
    /// pvt_frst_dmsp_prc
    /// pvt_scnd_dmsp_prc
    /// dmrs_val
    /// dmsp_val
    /// rstc_wdth_prc
    /// stck_fcam
    /// stck_sspr
    /// aspr_unit
    /// hts_deal_qty_unit_val
    /// lstn_stcn
    /// hts_avls
    /// stac_month
    /// vol_tnrt
    /// d250_hgpr
    /// d250_hgpr_date
    /// d250_hgpr_vrss_prpr_rate
    /// d250_lwpr
    /// d250_lwpr_date
    /// d250_lwpr_vrss_prpr_rate
    /// stck_dryy_hgpr
    /// dryy_hgpr_vrss_prpr_rate
    /// dryy_hgpr_date
    /// stck_dryy_lwpr
    /// dryy_lwpr_vrss_prpr_rate
    /// dryy_lwpr_date
    /// w52_hgpr
    /// w52_hgpr_vrss_prpr_ctrt
    /// w52_hgpr_date
    /// w52_lwpr
    /// w52_lwpr_vrss_prpr_ctrt
    /// w52_lwpr_date
    /// whol_loan_rmnd_rate
    /// ssts_yn
    /// stck_shrn_iscd
    /// fcam_cnnm
    /// cpfn_cnnm
    /// apprch_rate
    /// frgn_hldn_qty
    /// vi_cls_code
    /// ovtm_vi_cls_code
    /// last_ssts_cntg_qty
    /// invt_caful_yn
    /// mrkt_warn_cls_code
    /// short_over_yn
    /// sltr_yn
    /// mang_issu_cls_code
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식 현재가 시세 API입니다. 실시간 시세를 원하신다면 웹소켓 API를 활용하세요.
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
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
    ///
    /// - TR_ID: Real=FHPST01010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-price-2
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 시세2[v1_국내주식-054]
    /// rprs_mrkt_kor_name
    /// new_hgpr_lwpr_cls_code
    /// mxpr_llam_cls_code
    /// crdt_able_yn
    /// stck_mxpr
    /// elw_pblc_yn
    /// prdy_clpr_vrss_oprc_rate
    /// crdt_rate
    /// marg_rate
    /// lwpr_vrss_prpr
    /// lwpr_vrss_prpr_sign
    /// prdy_clpr_vrss_lwpr_rate
    /// stck_lwpr
    /// hgpr_vrss_prpr
    /// hgpr_vrss_prpr_sign
    /// prdy_clpr_vrss_hgpr_rate
    /// stck_hgpr
    /// oprc_vrss_prpr
    /// oprc_vrss_prpr_sign
    /// mang_issu_yn
    /// divi_app_cls_code
    /// short_over_yn
    /// mrkt_warn_cls_code
    /// invt_caful_yn
    /// stange_runup_yn
    /// ssts_hot_yn
    /// low_current_yn
    /// vi_cls_code
    /// short_over_cls_code
    /// stck_llam
    /// new_lstn_cls_name
    /// vlnt_deal_cls_name
    /// flng_cls_name
    /// revl_issu_reas_name
    /// mrkt_warn_cls_name
    /// stck_sdpr
    /// bstp_cls_code
    /// stck_prdy_clpr
    /// insn_pbnt_yn
    /// fcam_mod_cls_name
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_tr_pbmn
    /// acml_vol
    /// prdy_vrss_vol_rate
    /// bstp_kor_isnm
    /// sltr_yn
    /// trht_yn
    /// oprc_rang_cont_yn
    /// vlnt_fin_cls_code
    /// stck_oprc
    /// prdy_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 시세2 API입니다.
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
    ///
    /// - TR_ID: Real=FHKST01010300 / VTS=FHKST01010300
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-ccnl
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 체결[v1_국내주식-009]
    /// stck_cntg_hour
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// cntg_vol
    /// tday_rltv
    /// prdy_ctrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 체결 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [010] 현재가 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 더 많은 체결데이터 확인이 필요할 경우 주식현재가 당일시간대별체결 API를 이용하세요
    /// (FID_INPUT_HOUR_1 를 이용하여 과거시간대 체결데이터 확인 가능)
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
    ///
    /// - TR_ID: Real=FHKST01010400 / VTS=FHKST01010400
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-price
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 일자별[v1_국내주식-010]
    /// stck_bsop_date
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// stck_clpr
    /// acml_vol
    /// prdy_vrss_vol_rate
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// hts_frgn_ehrt
    /// frgn_ntby_qty
    /// flng_cls_code
    /// acml_prtt_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 일자별 API입니다. 일/주/월별 주가를 확인할 수 있으며 최근 30일(주,별)로 제한되어 있습니다.
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
    ///
    /// - TR_ID: Real=FHKST01010200 / VTS=FHKST01010200
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 호가/예상체결[v1_국내주식-011]
    /// aspr_acpt_hour
    /// askp10
    /// bidp10
    /// askp_rsqn1
    /// askp_rsqn2
    /// askp_rsqn3
    /// askp_rsqn4
    /// askp_rsqn5
    /// askp_rsqn6
    /// askp_rsqn7
    /// askp_rsqn8
    /// askp_rsqn9
    /// askp_rsqn10
    /// bidp_rsqn1
    /// bidp_rsqn2
    /// bidp_rsqn3
    /// bidp_rsqn4
    /// bidp_rsqn5
    /// bidp_rsqn6
    /// bidp_rsqn7
    /// bidp_rsqn8
    /// bidp_rsqn9
    /// bidp_rsqn10
    /// askp_rsqn_icdc1
    /// askp_rsqn_icdc2
    /// askp_rsqn_icdc3
    /// askp_rsqn_icdc4
    /// askp_rsqn_icdc5
    /// askp_rsqn_icdc6
    /// askp_rsqn_icdc7
    /// askp_rsqn_icdc8
    /// askp_rsqn_icdc9
    /// askp_rsqn_icdc10
    /// bidp_rsqn_icdc1
    /// bidp_rsqn_icdc2
    /// bidp_rsqn_icdc3
    /// bidp_rsqn_icdc4
    /// bidp_rsqn_icdc5
    /// bidp_rsqn_icdc6
    /// bidp_rsqn_icdc7
    /// bidp_rsqn_icdc8
    /// bidp_rsqn_icdc9
    /// bidp_rsqn_icdc10
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// total_askp_rsqn_icdc
    /// total_bidp_rsqn_icdc
    /// ovtm_total_askp_icdc
    /// ovtm_total_bidp_icdc
    /// ovtm_total_askp_rsqn
    /// ovtm_total_bidp_rsqn
    /// ntby_aspr_rsqn
    /// new_mkop_cls_code
    /// antc_mkop_cls_code
    /// stck_prpr
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// stck_sdpr
    /// antc_cnpr
    /// antc_cntg_vrss_sign
    /// antc_cntg_vrss
    /// antc_cntg_prdy_ctrt
    /// antc_vol
    /// stck_shrn_iscd
    /// vi_cls_code
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 호가 예상체결 API입니다. 매수 매도 호가를 확인하실 수 있습니다. 실시간 데이터를 원하신다면 웹소켓 API를 활용하세요.
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
    ///
    /// - TR_ID: Real=FHKST01010900 / VTS=FHKST01010900
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 투자자[v1_국내주식-012]
    /// stck_bsop_date
    /// stck_clpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prsn_ntby_qty
    /// frgn_ntby_qty
    /// orgn_ntby_qty
    /// prsn_ntby_tr_pbmn
    /// frgn_ntby_tr_pbmn
    /// orgn_ntby_tr_pbmn
    /// prsn_shnu_vol
    /// frgn_shnu_vol
    /// orgn_shnu_vol
    /// prsn_shnu_tr_pbmn
    /// frgn_shnu_tr_pbmn
    /// orgn_shnu_tr_pbmn
    /// prsn_seln_vol
    /// frgn_seln_vol
    /// orgn_seln_vol
    /// prsn_seln_tr_pbmn
    /// frgn_seln_tr_pbmn
    /// orgn_seln_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 투자자 API입니다. 개인, 외국인, 기관 등 투자 정보를 확인할 수 있습니다.
    ///
    /// [유의사항]
    /// - 외국인은 외국인(외국인투자등록 고유번호가 있는 경우)+기타 외국인을 지칭합니다.
    /// - 당일 데이터는 장 종료 후 제공됩니다.
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
    ///
    /// - TR_ID: Real=FHKST01010600 / VTS=FHKST01010600
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-member
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 회원사[v1_국내주식-013]
    /// seln_mbcr_no1
    /// seln_mbcr_no2
    /// seln_mbcr_no3
    /// seln_mbcr_no4
    /// seln_mbcr_no5
    /// seln_mbcr_name1
    /// seln_mbcr_name2
    /// seln_mbcr_name3
    /// seln_mbcr_name4
    /// seln_mbcr_name5
    /// total_seln_qty1
    /// total_seln_qty2
    /// total_seln_qty3
    /// total_seln_qty4
    /// total_seln_qty5
    /// seln_mbcr_rlim1
    /// seln_mbcr_rlim2
    /// seln_mbcr_rlim3
    /// seln_mbcr_rlim4
    /// seln_mbcr_rlim5
    /// seln_qty_icdc1
    /// seln_qty_icdc2
    /// seln_qty_icdc3
    /// seln_qty_icdc4
    /// seln_qty_icdc5
    /// shnu_mbcr_no1
    /// shnu_mbcr_no2
    /// shnu_mbcr_no3
    /// shnu_mbcr_no4
    /// shnu_mbcr_no5
    /// shnu_mbcr_name1
    /// shnu_mbcr_name2
    /// shnu_mbcr_name3
    /// shnu_mbcr_name4
    /// shnu_mbcr_name5
    /// total_shnu_qty1
    /// total_shnu_qty2
    /// total_shnu_qty3
    /// total_shnu_qty4
    /// total_shnu_qty5
    /// shnu_mbcr_rlim1
    /// shnu_mbcr_rlim2
    /// shnu_mbcr_rlim3
    /// shnu_mbcr_rlim4
    /// shnu_mbcr_rlim5
    /// shnu_qty_icdc1
    /// shnu_qty_icdc2
    /// shnu_qty_icdc3
    /// shnu_qty_icdc4
    /// shnu_qty_icdc5
    /// glob_total_seln_qty
    /// glob_seln_rlim
    /// glob_ntby_qty
    /// glob_total_shnu_qty
    /// glob_shnu_rlim
    /// seln_mbcr_glob_yn_1
    /// seln_mbcr_glob_yn_2
    /// seln_mbcr_glob_yn_3
    /// seln_mbcr_glob_yn_4
    /// seln_mbcr_glob_yn_5
    /// shnu_mbcr_glob_yn_1
    /// shnu_mbcr_glob_yn_2
    /// shnu_mbcr_glob_yn_3
    /// shnu_mbcr_glob_yn_4
    /// shnu_mbcr_glob_yn_5
    /// glob_total_seln_qty_icdc
    /// glob_total_shnu_qty_icdc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식 현재가 회원사 API입니다. 회원사의 투자 정보를 확인할 수 있습니다.
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
    ///
    /// - TR_ID: Real=FHKST03010100 / VTS=FHKST03010100
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice
    ///
    /// [국내주식] 기본시세
    /// 국내주식기간별시세(일/주/월/년)[v1_국내주식-016]
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// stck_prdy_clpr
    /// acml_vol
    /// acml_tr_pbmn
    /// hts_kor_isnm
    /// stck_prpr
    /// stck_shrn_iscd
    /// prdy_vol
    /// stck_mxpr
    /// stck_llam
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// stck_prdy_oprc
    /// stck_prdy_hgpr
    /// stck_prdy_lwpr
    /// prdy_vrss_vol
    /// vol_tnrt
    /// stck_fcam
    /// lstn_stcn
    /// hts_avls
    /// itewhol_loan_rmnd_ratem
    /// stck_bsop_date
    /// stck_clpr
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// acml_vol
    /// acml_tr_pbmn
    /// flng_cls_code
    /// prtt_rate
    /// mod_yn
    /// prdy_vrss_sign
    /// prdy_vrss
    /// revl_issu_reas
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식기간별시세(일/주/월/년) API입니다.
    /// 실전계좌/모의계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능합니다.
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
    ///
    /// - TR_ID: Real=FHKST03010200 / VTS=FHKST03010200
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-itemchartprice
    ///
    /// [국내주식] 기본시세
    /// 주식당일분봉조회[v1_국내주식-022]
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// stck_prdy_clpr
    /// acml_vol
    /// acml_tr_pbmn
    /// hts_kor_isnm
    /// stck_prpr
    /// stck_bsop_date
    /// stck_cntg_hour
    /// stck_prpr
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// cntg_vol
    /// acml_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식당일분봉조회 API입니다.
    /// 실전계좌/모의계좌의 경우, 한 번의 호출에 최대 30건까지 확인 가능합니다.
    ///
    /// ※ 당일 분봉 데이터만 제공됩니다. (전일자 분봉 미제공)
    ///
    /// ※ input > FID_INPUT_HOUR_1 에 미래일시 입력 시에 현재가로 조회됩니다.
    /// ex) 오전 10시에 113000 입력 시에 오전 10시~11시30분 사이의 데이터가 오전 10시 값으로 조회됨
    ///
    /// ※ output2의 첫번째 배열의 체결량(cntg_vol)은 첫체결이 발생되기 전까지는 이전 분봉의 체결량이 해당 위치에 표시됩니다.
    /// 해당 분봉의 첫 체결이 발생되면 해당 이전분 체결량이 두번째 배열로 이동되면서 새로운 체결량으로 업데이트됩니다.
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
    ///
    /// - TR_ID: Real=FHKST03010230 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice
    ///
    /// [국내주식] 기본시세
    /// 주식일별분봉조회 [국내주식-213]
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// stck_prdy_clpr
    /// acml_vol
    /// acml_tr_pbmn
    /// hts_kor_isnm
    /// stck_prpr
    /// stck_bsop_date
    /// stck_cntg_hour
    /// stck_prpr
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// cntg_vol
    /// acml_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식일별분봉조회 API입니다.
    ///
    /// 실전계좌의 경우, 한 번의 호출에 최대 120건까지 확인 가능하며,
    /// FID_INPUT_DATE_1, FID_INPUT_HOUR_1 이용하여 과거일자 분봉조회 가능합니다.
    ///
    /// ※ 과거 분봉 조회 시, 당사 서버에서 보관하고 있는 만큼의 데이터만 확인이 가능합니다. (최대 1년 분봉 보관)
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
    ///
    /// - TR_ID: Real=FHPST01060000 / VTS=FHPST01060000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 당일시간대별체결[v1_국내주식-023]
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// prdy_vol
    /// rprs_mrkt_kor_name
    /// stck_cntg_hour
    /// stck_pbpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// tday_rltv
    /// acml_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 당일시간대별체결 API입니다.
    ///
    /// * FID_INPUT_HOUR_1 를 이용하여 과거시간대 체결데이터 확인 가능
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
    ///
    /// - TR_ID: Real=FHPST02320000 / VTS=FHPST02320000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 시간외일자별주가[v1_국내주식-026]
    /// ovtm_untp_prpr
    /// ovtm_untp_prdy_vrss
    /// ovtm_untp_prdy_vrss_sign
    /// ovtm_untp_prdy_ctrt
    /// ovtm_untp_vol
    /// ovtm_untp_tr_pbmn
    /// ovtm_untp_mxpr
    /// ovtm_untp_llam
    /// ovtm_untp_oprc
    /// ovtm_untp_hgpr
    /// ovtm_untp_lwpr
    /// ovtm_untp_antc_cnpr
    /// ovtm_untp_antc_cntg_vrss
    /// ovtm_untp_antc_cntg_vrss_sign
    /// ovtm_untp_antc_cntg_ctrt
    /// ovtm_untp_antc_vol
    /// stck_bsop_date
    /// ovtm_untp_prpr
    /// ovtm_untp_prdy_vrss
    /// ovtm_untp_prdy_vrss_sign
    /// ovtm_untp_prdy_ctrt
    /// ovtm_untp_vol
    /// stck_clpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// ovtm_untp_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 시간외일자별주가 API입니다.  (최근일 30건만 조회 가능)
    /// 한국투자 HTS(eFriend Plus) > [0232] 시간외 일자별주가의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST02310000 / VTS=FHPST02310000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion
    ///
    /// [국내주식] 기본시세
    /// 주식현재가 시간외시간별체결[v1_국내주식-025]
    /// ovtm_untp_prpr
    /// ovtm_untp_prdy_vrss
    /// ovtm_untp_prdy_vrss_sign
    /// ovtm_untp_prdy_ctrt
    /// ovtm_untp_vol
    /// ovtm_untp_tr_pbmn
    /// ovtm_untp_mxpr
    /// ovtm_untp_llam
    /// ovtm_untp_oprc
    /// ovtm_untp_hgpr
    /// ovtm_untp_lwpr
    /// ovtm_untp_antc_cnpr
    /// ovtm_untp_antc_cntg_vrss
    /// ovtm_untp_antc_cntg_vrss_sign
    /// ovtm_untp_antc_cntg_ctrt
    /// ovtm_untp_antc_vol
    /// uplm_sign
    /// lslm_sign
    /// stck_cntg_hour
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// cntg_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 시간외시간별체결 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0231] 시간외 시간별체결의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST02300000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-overtime-price
    ///
    /// [국내주식] 기본시세
    /// 국내주식 시간외현재가[국내주식-076]
    /// bstp_kor_isnm
    /// mang_issu_cls_name
    /// ovtm_untp_prpr
    /// ovtm_untp_prdy_vrss
    /// ovtm_untp_prdy_vrss_sign
    /// ovtm_untp_prdy_ctrt
    /// ovtm_untp_vol
    /// ovtm_untp_tr_pbmn
    /// ovtm_untp_mxpr
    /// ovtm_untp_llam
    /// ovtm_untp_oprc
    /// ovtm_untp_hgpr
    /// ovtm_untp_lwpr
    /// marg_rate
    /// ovtm_untp_antc_cnpr
    /// ovtm_untp_antc_cntg_vrss
    /// ovtm_untp_antc_cntg_vrss_sign
    /// ovtm_untp_antc_cntg_ctrt
    /// ovtm_untp_antc_cnqn
    /// crdt_able_yn
    /// new_lstn_cls_name
    /// sltr_yn
    /// mang_issu_yn
    /// mrkt_warn_cls_code
    /// trht_yn
    /// vlnt_deal_cls_name
    /// ovtm_untp_sdpr
    /// mrkt_warn_cls_name
    /// revl_issu_reas_name
    /// insn_pbnt_yn
    /// flng_cls_name
    /// rprs_mrkt_kor_name
    /// ovtm_vi_cls_code
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외현재가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0230] 시간외 현재가 화면의 좌측 상단기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST02300400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-overtime-asking-price
    ///
    /// [국내주식] 기본시세
    /// 국내주식 시간외호가[국내주식-077]
    /// ovtm_untp_last_hour
    /// ovtm_untp_askp1
    /// ovtm_untp_askp2
    /// ovtm_untp_askp3
    /// ovtm_untp_askp4
    /// ovtm_untp_askp5
    /// ovtm_untp_askp6
    /// ovtm_untp_askp7
    /// ovtm_untp_askp8
    /// ovtm_untp_askp9
    /// ovtm_untp_askp10
    /// ovtm_untp_bidp1
    /// ovtm_untp_bidp2
    /// ovtm_untp_bidp3
    /// ovtm_untp_bidp4
    /// ovtm_untp_bidp5
    /// ovtm_untp_bidp6
    /// ovtm_untp_bidp7
    /// ovtm_untp_bidp8
    /// ovtm_untp_bidp9
    /// ovtm_untp_bidp10
    /// ovtm_untp_askp_icdc1
    /// ovtm_untp_askp_icdc2
    /// ovtm_untp_askp_icdc3
    /// ovtm_untp_askp_icdc4
    /// ovtm_untp_askp_icdc5
    /// ovtm_untp_askp_icdc6
    /// ovtm_untp_askp_icdc7
    /// ovtm_untp_askp_icdc8
    /// ovtm_untp_askp_icdc9
    /// ovtm_untp_askp_icdc10
    /// ovtm_untp_bidp_icdc1
    /// ovtm_untp_bidp_icdc2
    /// ovtm_untp_bidp_icdc3
    /// ovtm_untp_bidp_icdc4
    /// ovtm_untp_bidp_icdc5
    /// ovtm_untp_bidp_icdc6
    /// ovtm_untp_bidp_icdc7
    /// ovtm_untp_bidp_icdc8
    /// ovtm_untp_bidp_icdc9
    /// ovtm_untp_bidp_icdc10
    /// ovtm_untp_askp_rsqn1
    /// ovtm_untp_askp_rsqn2
    /// ovtm_untp_askp_rsqn3
    /// ovtm_untp_askp_rsqn4
    /// ovtm_untp_askp_rsqn5
    /// ovtm_untp_askp_rsqn6
    /// ovtm_untp_askp_rsqn7
    /// ovtm_untp_askp_rsqn8
    /// ovtm_untp_askp_rsqn9
    /// ovtm_untp_askp_rsqn10
    /// ovtm_untp_bidp_rsqn1
    /// ovtm_untp_bidp_rsqn2
    /// ovtm_untp_bidp_rsqn3
    /// ovtm_untp_bidp_rsqn4
    /// ovtm_untp_bidp_rsqn5
    /// ovtm_untp_bidp_rsqn6
    /// ovtm_untp_bidp_rsqn7
    /// ovtm_untp_bidp_rsqn8
    /// ovtm_untp_bidp_rsqn9
    /// ovtm_untp_bidp_rsqn10
    /// ovtm_untp_total_askp_rsqn
    /// ovtm_untp_total_bidp_rsqn
    /// ovtm_untp_total_askp_rsqn_icdc
    /// ovtm_untp_total_bidp_rsqn_icdc
    /// ovtm_untp_ntby_bidp_rsqn
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// total_askp_rsqn_icdc
    /// total_bidp_rsqn_icdc
    /// ovtm_total_askp_rsqn
    /// ovtm_total_bidp_rsqn
    /// ovtm_total_askp_icdc
    /// ovtm_total_bidp_icdc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외호가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0230] 시간외 현재가 화면의 '호가' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST117300C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-closing-price
    ///
    /// [국내주식] 기본시세
    /// 국내주식 장마감 예상체결가[국내주식-120]
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// sdpr_vrss_prpr
    /// sdpr_vrss_prpr_rate
    /// cntg_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 장마감 예상체결가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0183] 장마감 예상체결가 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKEW15010000 / VTS=FHKEW15010000
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-elw-price
    ///
    /// [국내주식] ELW 시세
    /// ELW 현재가 시세[v1_국내주식-014]
    /// elw_shrn_iscd
    /// hts_kor_isnm
    /// elw_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// prdy_vrss_vol_rate
    /// unas_shrn_iscd
    /// unas_isnm
    /// unas_prpr
    /// unas_prdy_vrss
    /// unas_prdy_vrss_sign
    /// unas_prdy_ctrt
    /// acml_tr_pbmn
    /// vol_tnrt
    /// elw_oprc
    /// elw_hgpr
    /// elw_lwpr
    /// stck_prdy_clpr
    /// hts_thpr
    /// atm_cls_name
    /// hts_ints_vltl
    /// pvt_scnd_dmrs_prc
    /// pvt_frst_dmrs_prc
    /// pvt_pont_val
    /// pvt_frst_dmsp_prc
    /// pvt_scnd_dmsp_prc
    /// dmsp_val
    /// dmrs_val
    /// elw_sdpr
    /// apprch_rate
    /// tick_conv_prc
    /// invt_epmd_cntt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ELW 현재가 시세 API입니다. ELW 관련 정보를 얻을 수 있습니다.
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
    ///
    /// - TR_ID: Real=FHPUP02100000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-price
    ///
    /// [국내주식] 업종/기타
    /// 국내업종 현재지수[v1_국내주식-063]
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_vol
    /// prdy_vol
    /// acml_tr_pbmn
    /// prdy_tr_pbmn
    /// bstp_nmix_oprc
    /// prdy_nmix_vrss_nmix_oprc
    /// oprc_vrss_prpr_sign
    /// bstp_nmix_oprc_prdy_ctrt
    /// bstp_nmix_hgpr
    /// prdy_nmix_vrss_nmix_hgpr
    /// hgpr_vrss_prpr_sign
    /// bstp_nmix_hgpr_prdy_ctrt
    /// bstp_nmix_lwpr
    /// prdy_clpr_vrss_lwpr
    /// lwpr_vrss_prpr_sign
    /// prdy_clpr_vrss_lwpr_rate
    /// ascn_issu_cnt
    /// uplm_issu_cnt
    /// stnr_issu_cnt
    /// down_issu_cnt
    /// lslm_issu_cnt
    /// dryy_bstp_nmix_hgpr
    /// dryy_hgpr_vrss_prpr_rate
    /// dryy_bstp_nmix_hgpr_date
    /// dryy_bstp_nmix_lwpr
    /// dryy_lwpr_vrss_prpr_rate
    /// dryy_bstp_nmix_lwpr_date
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// seln_rsqn_rate
    /// shnu_rsqn_rate
    /// ntby_rsqn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내업종 현재지수 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0210] 업종 현재지수 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPUP02120000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-daily-price
    ///
    /// [국내주식] 업종/기타
    /// 국내업종 일자별지수[v1_국내주식-065]
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// prdy_vol
    /// ascn_issu_cnt
    /// down_issu_cnt
    /// stnr_issu_cnt
    /// uplm_issu_cnt
    /// lslm_issu_cnt
    /// prdy_tr_pbmn
    /// dryy_bstp_nmix_hgpr_date
    /// dryy_bstp_nmix_hgpr
    /// dryy_bstp_nmix_lwpr
    /// dryy_bstp_nmix_lwpr_date
    /// stck_bsop_date
    /// bstp_nmix_prpr
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_vrss
    /// bstp_nmix_prdy_ctrt
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// acml_vol_rlim
    /// acml_vol
    /// acml_tr_pbmn
    /// invt_new_psdg
    /// d20_dsrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내업종 일자별지수 API입니다. 한 번의 조회에 100건까지 확인 가능합니다.
    /// 한국투자 HTS(eFriend Plus) > [0212] 업종 일자별지수 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPUP02110100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-tickprice
    ///
    /// [국내주식] 업종/기타
    /// 국내업종 시간별지수(초)[국내주식-064]
    /// stck_cntg_hour
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_tr_pbmn
    /// acml_vol
    /// cntg_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내업종 시간별지수(초) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0211] 업종 시간별지수 화면에서 우측 '10초' 선택 시의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPUP02110200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-timeprice
    ///
    /// [국내주식] 업종/기타
    /// 국내업종 시간별지수(분)[국내주식-119]
    /// bsop_hour
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_tr_pbmn
    /// acml_vol
    /// cntg_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내업종 시간별지수(분) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0211] 업종 시간별지수 화면에서 우측 '1분' 선택 시의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKUP03500200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-time-indexchartprice
    ///
    /// [국내주식] 업종/기타
    /// 업종 분봉조회[v1_국내주식-045]
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// prdy_nmix
    /// acml_vol
    /// acml_tr_pbmn
    /// hts_kor_isnm
    /// bstp_nmix_prpr
    /// bstp_cls_code
    /// prdy_vol
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// futs_prdy_oprc
    /// futs_prdy_hgpr
    /// futs_prdy_lwpr
    /// stck_bsop_date
    /// stck_cntg_hour
    /// bstp_nmix_prpr
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// cntg_vol
    /// acml_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 업종 분봉조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0350] 업종 종합차트 화면의 분봉기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 102건까지 확인 가능합니다.
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
    ///
    /// - TR_ID: Real=FHKUP03500100 / VTS=FHKUP03500100
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice
    ///
    /// [국내주식] 업종/기타
    /// 국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// prdy_nmix
    /// acml_vol
    /// acml_tr_pbmn
    /// hts_kor_isnm
    /// bstp_nmix_prpr
    /// bstp_cls_code
    /// prdy_vol
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// futs_prdy_oprc
    /// futs_prdy_hgpr
    /// futs_prdy_lwpr
    /// stck_bsop_date
    /// bstp_nmix_prpr
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// acml_vol
    /// acml_tr_pbmn
    /// mod_yn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 업종기간별시세(일/주/월/년) API입니다.
    /// 실전계좌/모의계좌의 경우, 한 번의 호출에 최대 50건까지 확인 가능합니다.
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
    ///
    /// - TR_ID: Real=FHPUP02140000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-index-category-price
    ///
    /// [국내주식] 업종/기타
    /// 국내업종 구분별전체시세[v1_국내주식-066]
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// prdy_vol
    /// ascn_issu_cnt
    /// down_issu_cnt
    /// stnr_issu_cnt
    /// uplm_issu_cnt
    /// lslm_issu_cnt
    /// prdy_tr_pbmn
    /// dryy_bstp_nmix_hgpr_date
    /// dryy_bstp_nmix_hgpr
    /// dryy_bstp_nmix_lwpr
    /// dryy_bstp_nmix_lwpr_date
    /// bstp_cls_code
    /// hts_kor_isnm
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// acml_vol_rlim
    /// acml_tr_pbmn_rlim
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내업종 구분별전체시세 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0214] 업종 전체시세 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST01840000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-index-trend
    ///
    /// [국내주식] 업종/기타
    /// 국내주식 예상체결지수 추이[국내주식-121]
    /// stck_cntg_hour
    /// bstp_nmix_prpr
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 예상체결지수 추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0184] 예상체결지수 추이 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKUP11750000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-total-index
    ///
    /// [국내주식] 업종/기타
    /// 국내주식 예상체결 전체지수[국내주식-122]
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// ascn_issu_cnt
    /// down_issu_cnt
    /// stnr_issu_cnt
    /// bstp_cls_code
    /// hts_kor_isnm
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// acml_vol
    /// nmix_sdpr
    /// ascn_issu_cnt
    /// stnr_issu_cnt
    /// down_issu_cnt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 예상체결 전체지수 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0185] 예상체결 전체지수 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST01390000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-vi-status
    ///
    /// [국내주식] 업종/기타
    /// 변동성완화장치(VI) 현황 [v1_국내주식-055]
    /// hts_kor_isnm
    /// mksc_shrn_iscd
    /// vi_cls_code
    /// bsop_date
    /// cntg_vi_hour
    /// vi_cncl_hour
    /// vi_kind_code
    /// vi_prc
    /// vi_stnd_prc
    /// vi_dprt
    /// vi_dmc_stnd_prc
    /// vi_dmc_dprt
    /// vi_count
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// HTS(eFriend Plus) [0139] 변동성 완화장치(VI) 현황 데이터를 확인할 수 있는 API입니다.
    ///
    /// 최근 30건까지 확인 가능합니다.
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
    ///
    /// - TR_ID: Real=FHPST07020000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/comp-interest
    ///
    /// [국내주식] 업종/기타
    /// 금리 종합(국내채권/금리) [국내주식-155]
    /// bcdt_code
    /// hts_kor_isnm
    /// bond_mnrt_prpr
    /// prdy_vrss_sign
    /// bond_mnrt_prdy_vrss
    /// prdy_ctrt
    /// stck_bsop_date
    /// bcdt_code
    /// hts_kor_isnm
    /// bond_mnrt_prpr
    /// prdy_vrss_sign
    /// bond_mnrt_prdy_vrss
    /// bstp_nmix_prdy_ctrt
    /// stck_bsop_date
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 금리 종합(국내채권/금리) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0702] 금리 종합 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 11:30 이후에 신규데이터가 수신되는 점 참고하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=FHKST01011800 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/news-title
    ///
    /// [국내주식] 업종/기타
    /// 종합 시황/공시(제목) [국내주식-141]
    /// cntt_usiq_srno
    /// news_ofer_entp_code
    /// data_dt
    /// data_tm
    /// hts_pbnt_titl_cntt
    /// news_lrdv_code
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 종합 시황/공시(제목) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0601] 종합 시황/공시 화면의 "우측 상단 리스트" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=CTCA0903R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/chk-holiday
    ///
    /// [국내주식] 업종/기타
    /// 국내휴장일조회[국내주식-040]
    /// bass_dt
    /// wday_dvsn_cd
    /// bzdy_yn
    /// tr_day_yn
    /// opnd_yn
    /// sttl_day_yn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// (★중요) 국내휴장일조회(TCA0903R) 서비스는 당사 원장서비스와 연관되어 있어
    /// 단시간 내 다수 호출시 서비스에 영향을 줄 수 있어 가급적 1일 1회 호출 부탁드립니다.
    ///
    /// 국내휴장일조회 API입니다.
    /// 영업일, 거래일, 개장일, 결제일 여부를 조회할 수 있습니다.
    /// 주문을 넣을 수 있는지 확인하고자 하실 경우 개장일여부(opnd_yn)을 사용하시면 됩니다.
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
    ///
    /// - TR_ID: Real=HHMCM000002C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/market-time
    ///
    /// [국내주식] 업종/기타
    /// 국내선물 영업일조회 [국내주식-160]
    /// s_time
    /// e_time
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내선물 영업일조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [1938] 시가총액순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// API호출 시 body 혹은 params로 입력하는 사항이 없습니다.
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
    ///
    /// - TR_ID: Real=CTPF1604R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/search-info
    ///
    /// [국내주식] 종목정보
    /// 상품기본조회[v1_국내주식-029]
    /// prdt_type_cd
    /// prdt_name
    /// prdt_name120
    /// prdt_abrv_name
    /// prdt_eng_name
    /// prdt_eng_name120
    /// prdt_eng_abrv_name
    /// std_pdno
    /// shtn_pdno
    /// prdt_sale_stat_cd
    /// prdt_risk_grad_cd
    /// prdt_clsf_cd
    /// prdt_clsf_name
    /// sale_strt_dt
    /// sale_end_dt
    /// wrap_asst_type_cd
    /// ivst_prdt_type_cd
    /// ivst_prdt_type_cd_name
    /// frst_erlm_dt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
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
    ///
    /// - TR_ID: Real=CTPF1002R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/search-stock-info
    ///
    /// [국내주식] 종목정보
    /// 주식기본조회[v1_국내주식-067]
    /// prdt_type_cd
    /// mket_id_cd
    /// scty_grp_id_cd
    /// excg_dvsn_cd
    /// setl_mmdd
    /// lstg_stqt
    /// lstg_cptl_amt
    /// issu_pric
    /// kospi200_item_yn
    /// scts_mket_lstg_dt
    /// scts_mket_lstg_abol_dt
    /// kosdaq_mket_lstg_dt
    /// kosdaq_mket_lstg_abol_dt
    /// frbd_mket_lstg_dt
    /// frbd_mket_lstg_abol_dt
    /// reits_kind_cd
    /// etf_dvsn_cd
    /// oilf_fund_yn
    /// idx_bztp_lcls_cd
    /// idx_bztp_mcls_cd
    /// idx_bztp_scls_cd
    /// stck_kind_cd
    /// mfnd_opng_dt
    /// mfnd_end_dt
    /// dpsi_erlm_cncl_dt
    /// etf_cu_qty
    /// prdt_name
    /// prdt_name120
    /// prdt_abrv_name
    /// std_pdno
    /// prdt_eng_name
    /// prdt_eng_name120
    /// prdt_eng_abrv_name
    /// dpsi_aptm_erlm_yn
    /// etf_txtn_type_cd
    /// etf_type_cd
    /// lstg_abol_dt
    /// nwst_odst_dvsn_cd
    /// sbst_pric
    /// thco_sbst_pric
    /// thco_sbst_pric_chng_dt
    /// tr_stop_yn
    /// admn_item_yn
    /// thdt_clpr
    /// bfdy_clpr
    /// clpr_chng_dt
    /// std_idst_clsf_cd
    /// std_idst_clsf_cd_name
    /// idx_bztp_lcls_cd_name
    /// idx_bztp_mcls_cd_name
    /// idx_bztp_scls_cd_name
    /// ocr_no
    /// crfd_item_yn
    /// elec_scty_yn
    /// issu_istt_cd
    /// etf_chas_erng_rt_dbnb
    /// etf_etn_ivst_heed_item_yn
    /// stln_int_rt_dvsn_cd
    /// frnr_psnl_lmt_rt
    /// lstg_rqsr_issu_istt_cd
    /// lstg_rqsr_item_cd
    /// trst_istt_issu_istt_cd
    /// cptt_trad_tr_psbl_yn
    /// nxt_tr_stop_yn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식기본조회 API입니다.
    /// 국내주식 종목의 종목상세정보를 확인할 수 있습니다.
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
    ///
    /// - TR_ID: Real=FHPST04770000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/credit-by-company
    ///
    /// [국내주식] 종목정보
    /// 국내주식 당사 신용가능종목[국내주식-111]
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// crdt_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 당사 신용가능종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0477] 당사 신용가능 종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 100건 확인 가능하며, 다음 조회가 불가합니다.
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
    ///
    /// - TR_ID: Real=HHKST668300C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/estimate-perform
    ///
    /// [국내주식] 종목정보
    /// 국내주식 종목추정실적 [국내주식-187]
    /// sht_cd
    /// item_kor_nm
    /// estdate
    /// rcmd_name
    /// capital
    /// forn_item_lmtrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 종목추정실적 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0613] 종목추정실적 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 본 화면의 추정실적 및 투자의견은 당월 초의 애널리스트의 의견사항이므로 월중 변동 사항이 있을 수 있음을 유의하시기 바랍니다.
    /// ※ 종목별 수익추정은 리서치본부에서 매월 발표되는 거래소, 코스닥 160여개 기업에 한정합니다. 구체적인 종목 리스트는 추정종목리스트를 참고하기 바랍니다.
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
    ///
    /// - TR_ID: Real=CTSC2702R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/lendable-by-company
    ///
    /// [국내주식] 종목정보
    /// 당사 대주가능 종목 [국내주식-195]
    /// prdt_name
    /// bfdy_clpr
    /// sbst_prvs
    /// tr_stop_dvsn_name
    /// psbl_yn_name
    /// lmt_qty1
    /// use_qty1
    /// trad_psbl_qty2
    /// rght_type_cd
    /// bass_dt
    /// psbl_yn
    /// tot_stup_lmt_qty
    /// brch_lmt_qty
    /// rqst_psbl_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 당사 대주가능 종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0490] 당사 대주가능 종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 본 API는 다음조회가 불가합니다.
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
    ///
    /// - TR_ID: Real=FHKST663300C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/invest-opinion
    ///
    /// [국내주식] 종목정보
    /// 국내주식 종목투자의견 [국내주식-188]
    /// stck_bsop_date
    /// invt_opnn
    /// invt_opnn_cls_code
    /// rgbf_invt_opnn
    /// rgbf_invt_opnn_cls_code
    /// mbcr_name
    /// hts_goal_prc
    /// stck_prdy_clpr
    /// stck_nday_esdg
    /// nday_dprt
    /// stft_esdg
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 종목투자의견 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0605] 종목투자의견 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 한 번의 호출에 100건까지 조회가 가능하기에, 일자 파라미터(FID_INPUT_DATE_1, FID_INPUT_DATE_2)를 조절하여 다음 데이터 조회하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=FHKST663400C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/invest-opbysec
    ///
    /// [국내주식] 종목정보
    /// 국내주식 증권사별 투자의견 [국내주식-189]
    /// stck_bsop_date
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// invt_opnn
    /// invt_opnn_cls_code
    /// rgbf_invt_opnn
    /// rgbf_invt_opnn_cls_code
    /// mbcr_name
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// hts_goal_prc
    /// stck_prdy_clpr
    /// stft_esdg
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 증권사별 투자의견 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0608] 증권사별 투자의견 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 한 번의 호출에 20건까지 조회가 가능하기에, 일자 파라미터(FID_INPUT_DATE_1, FID_INPUT_DATE_2)를 조절하여 다음 데이터 조회하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKST03900300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/psearch-title
    ///
    /// [국내주식] 시세분석
    /// 종목조건검색 목록조회[국내주식-038]
    /// user_id
    /// grp_nm
    /// condition_nm
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API입니다.
    /// 종목조건검색 목록조회 API(/uapi/domestic-stock/v1/quotations/psearch-title)의 output인 'seq'을 종목조건검색조회 API(/uapi/domestic-stock/v1/quotations/psearch-result)의 input으로 사용하시면 됩니다.
    ///
    /// ※ 시스템 안정성을 위해 API로 제공되는 조건검색 결과의 경우 조건당 100건으로 제한을 둔 점 양해 부탁드립니다.
    ///
    /// ※ [0110] 화면의 '대상변경' 설정사항은 HTS [0110] 사용자 조건검색 화면에만 적용됨에 유의 부탁드립니다.
    ///
    /// ※ '조회가 계속 됩니다. (다음을 누르십시오.)' 오류 발생 시 해결방법
    /// → HTS(efriend Plus) [0110] 조건검색 화면에서 조건을 등록하신 후, 왼쪽 하단의 "사용자조건 서버저장" 클릭하셔서 등록한 조건들을 서버로 보낸 후 다시 API 호출 시도 부탁드립니다.
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
    ///
    /// - TR_ID: Real=HHKST03900400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/psearch-result
    ///
    /// [국내주식] 시세분석
    /// 종목조건검색조회 [국내주식-039]
    /// chgrate
    /// acml_vol
    /// trade_amt
    /// change
    /// high52
    /// expprice
    /// expchange
    /// expchggrate
    /// expcvol
    /// chgrate2
    /// expdaebi
    /// recprice
    /// uplmtprice
    /// dnlmtprice
    /// stotprice
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API입니다.
    /// 종목조건검색 목록조회 API(/uapi/domestic-stock/v1/quotations/psearch-title)의 output인 'seq'을 종목조건검색조회 API(/uapi/domestic-stock/v1/quotations/psearch-result)의 input으로 사용하시면 됩니다.
    ///
    /// ※ 시스템 안정성을 위해 API로 제공되는 조건검색 결과의 경우 조건당 100건으로 제한을 둔 점 양해 부탁드립니다.
    ///
    /// ※ [0110] 화면의 '대상변경' 설정사항은 HTS [0110] 사용자 조건검색 화면에만 적용됨에 유의 부탁드립니다.
    ///
    /// ※ '조회가 계속 됩니다. (다음을 누르십시오.)' 오류 발생 시 해결방법
    /// → HTS(efriend Plus) [0110] 조건검색 화면에서 조건을 등록하신 후, 왼쪽 하단의 "사용자조건 서버저장" 클릭하셔서 등록한 조건들을 서버로 보낸 후 다시 API 호출 시도 부탁드립니다.
    ///
    /// ※ {"rt_cd":"1","msg_cd":"MCA05918","msg1":"종목코드 오류입니다."} 메시지 발생 이유
    /// → 조건검색 결과 검색된 종목이 0개인 경우 위 응답값을 수신하게 됩니다.
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
    ///
    /// - TR_ID: Real=HHKCM113004C7 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/intstock-grouplist
    ///
    /// [국내주식] 시세분석
    /// 관심종목 그룹조회 [국내주식-204]
    /// trnm_hour
    /// data_rank
    /// inter_grp_code
    /// inter_grp_name
    /// ask_cnt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 관심종목 그룹조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0161] 관심종목 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ① 관심종목 그룹조회 → ② 관심종목 그룹별 종목조회 → ③ 관심종목(멀티종목) 시세조회 순서대로 호출하셔서 관심종목 시세 조회 가능합니다.
    ///
    /// ※ 한 번의 호출에 최대 30종목의 시세 확인 가능합니다.
    ///
    /// 한국투자증권 Github 에서 관심종목 복수시세조회 파이썬 샘플코드를 참고하실 수 있습니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/rest/get_interest_stocks_price.py
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
    ///
    /// - TR_ID: Real=FHKST11300006 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/intstock-multprice
    ///
    /// [국내주식] 시세분석
    /// 관심종목(멀티종목) 시세조회 [국내주식-205]
    /// kospi_kosdaq_cls_name
    /// mrkt_trtm_cls_name
    /// hour_cls_code
    /// inter_shrn_iscd
    /// inter_kor_isnm
    /// inter2_prpr
    /// inter2_prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// inter2_oprc
    /// inter2_hgpr
    /// inter2_lwpr
    /// inter2_llam
    /// inter2_mxpr
    /// inter2_askp
    /// inter2_bidp
    /// seln_rsqn
    /// shnu_rsqn
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// acml_tr_pbmn
    /// inter2_prdy_clpr
    /// oprc_vrss_hgpr_rate
    /// intr_antc_cntg_vrss
    /// intr_antc_cntg_vrss_sign
    /// intr_antc_cntg_prdy_ctrt
    /// intr_antc_vol
    /// inter2_sdpr
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 관심종목(멀티종목) 시세조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0161] 관심종목 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=HHKCM113004C6 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group
    ///
    /// [국내주식] 시세분석
    /// 관심종목 그룹별 종목조회 [국내주식-203]
    /// data_rank
    /// inter_grp_name
    /// fid_mrkt_cls_code
    /// data_rank
    /// exch_code
    /// jong_code
    /// color_code
    /// hts_kor_isnm
    /// fxdt_ntby_qty
    /// cntg_unpr
    /// cntg_cls_code
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 관심종목 그룹별 종목조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0161] 관심종목 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ① 관심종목 그룹조회 → ② 관심종목 그룹별 종목조회 → ③ 관심종목(멀티종목) 시세조회 순서대로 호출하셔서 관심종목 시세 조회 가능합니다.
    ///
    /// ※ 한 번의 호출에 최대 30종목의 시세 확인 가능합니다.
    ///
    /// 한국투자증권 Github 에서 관심종목 복수시세조회 파이썬 샘플코드를 참고하실 수 있습니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/rest/get_interest_stocks_price.py
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
    ///
    /// - TR_ID: Real=FHPTJ04400000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/foreign-institution-total
    ///
    /// [국내주식] 시세분석
    /// 국내기관_외국인 매매종목가집계[국내주식-037]
    /// hts_kor_isnm
    /// mksc_shrn_iscd
    /// ntby_qty
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// frgn_ntby_qty
    /// orgn_ntby_qty
    /// ivtr_ntby_qty
    /// bank_ntby_qty
    /// insu_ntby_qty
    /// mrbn_ntby_qty
    /// fund_ntby_qty
    /// etc_orgt_ntby_vol
    /// etc_corp_ntby_vol
    /// frgn_ntby_tr_pbmn
    /// orgn_ntby_tr_pbmn
    /// ivtr_ntby_tr_pbmn
    /// bank_ntby_tr_pbmn
    /// insu_ntby_tr_pbmn
    /// mrbn_ntby_tr_pbmn
    /// fund_ntby_tr_pbmn
    /// etc_orgt_ntby_tr_pbmn
    /// etc_corp_ntby_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내기관_외국인 매매종목가집계 API입니다.
    ///
    /// HTS(efriend Plus) [0440] 외국인/기관 매매종목 가집계 화면을 API로 구현한 사항으로 화면을 함께 보시면 기능 이해가 쉽습니다.
    ///
    /// 증권사 직원이 장중에 집계/입력한 자료를 단순 누계한 수치로서,
    /// 입력시간은 외국인 09:30, 11:20, 13:20, 14:30 / 기관종합 10:00, 11:20, 13:20, 14:30 이며,
    /// 입력한 시간은 ±10분정도 차이가 발생할 수 있으며, 장운영 사정에 다라 변동될 수 있습니다.
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
    ///
    /// - TR_ID: Real=FHKST644100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-trade-estimate
    ///
    /// [국내주식] 시세분석
    /// 외국계 매매종목 가집계 [국내주식-161]
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// glob_ntsl_qty
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// glob_total_seln_qty
    /// glob_total_shnu_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 외국계 매매종목 가집계 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0430] 외국계 매매종목 가집계 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPTJ04160001 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily
    ///
    /// [국내주식] 시세분석
    /// 종목별 투자자매매동향(일별)
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// prdy_vol
    /// rprs_mrkt_kor_name
    /// stck_bsop_date
    /// stck_clpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// frgn_ntby_qty
    /// frgn_reg_ntby_qty
    /// frgn_nreg_ntby_qty
    /// prsn_ntby_qty
    /// orgn_ntby_qty
    /// scrt_ntby_qty
    /// ivtr_ntby_qty
    /// pe_fund_ntby_vol
    /// bank_ntby_qty
    /// insu_ntby_qty
    /// mrbn_ntby_qty
    /// fund_ntby_qty
    /// etc_ntby_qty
    /// etc_corp_ntby_vol
    /// etc_orgt_ntby_vol
    /// frgn_reg_ntby_pbmn
    /// frgn_ntby_tr_pbmn
    /// frgn_nreg_ntby_pbmn
    /// prsn_ntby_tr_pbmn
    /// orgn_ntby_tr_pbmn
    /// scrt_ntby_tr_pbmn
    /// pe_fund_ntby_tr_pbmn
    /// ivtr_ntby_tr_pbmn
    /// bank_ntby_tr_pbmn
    /// insu_ntby_tr_pbmn
    /// mrbn_ntby_tr_pbmn
    /// fund_ntby_tr_pbmn
    /// etc_ntby_tr_pbmn
    /// etc_corp_ntby_tr_pbmn
    /// etc_orgt_ntby_tr_pbmn
    /// frgn_seln_vol
    /// frgn_shnu_vol
    /// frgn_seln_tr_pbmn
    /// frgn_shnu_tr_pbmn
    /// frgn_reg_askp_qty
    /// frgn_reg_bidp_qty
    /// frgn_reg_askp_pbmn
    /// frgn_reg_bidp_pbmn
    /// frgn_nreg_askp_qty
    /// frgn_nreg_bidp_qty
    /// frgn_nreg_askp_pbmn
    /// frgn_nreg_bidp_pbmn
    /// prsn_seln_vol
    /// prsn_shnu_vol
    /// prsn_seln_tr_pbmn
    /// prsn_shnu_tr_pbmn
    /// orgn_seln_vol
    /// orgn_shnu_vol
    /// orgn_seln_tr_pbmn
    /// orgn_shnu_tr_pbmn
    /// scrt_seln_vol
    /// scrt_shnu_vol
    /// scrt_seln_tr_pbmn
    /// scrt_shnu_tr_pbmn
    /// ivtr_seln_vol
    /// ivtr_shnu_vol
    /// ivtr_seln_tr_pbmn
    /// ivtr_shnu_tr_pbmn
    /// pe_fund_seln_tr_pbmn
    /// pe_fund_seln_vol
    /// pe_fund_shnu_tr_pbmn
    /// pe_fund_shnu_vol
    /// bank_seln_vol
    /// bank_shnu_vol
    /// bank_seln_tr_pbmn
    /// bank_shnu_tr_pbmn
    /// insu_seln_vol
    /// insu_shnu_vol
    /// insu_seln_tr_pbmn
    /// insu_shnu_tr_pbmn
    /// mrbn_seln_vol
    /// mrbn_shnu_vol
    /// mrbn_seln_tr_pbmn
    /// mrbn_shnu_tr_pbmn
    /// fund_seln_vol
    /// fund_shnu_vol
    /// fund_seln_tr_pbmn
    /// fund_shnu_tr_pbmn
    /// etc_seln_vol
    /// etc_shnu_vol
    /// etc_seln_tr_pbmn
    /// etc_shnu_tr_pbmn
    /// etc_orgt_seln_vol
    /// etc_orgt_shnu_vol
    /// etc_orgt_seln_tr_pbmn
    /// etc_orgt_shnu_tr_pbmn
    /// etc_corp_seln_vol
    /// etc_corp_shnu_vol
    /// etc_corp_seln_tr_pbmn
    /// etc_corp_shnu_tr_pbmn
    /// bold_yn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 종목별 투자자매매동향(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0416] 종목별 일별동향 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 단위 : 금액(백만원) 수량(주)
    ///
    /// 당일 데이터는 15:40이후에 데이터가 가집계 및 산출되어 15:40부터 조회가능하며,
    /// 데이터 산출의 경우 산출 시간대는 일정하지 않을 수 있음을 참고 부탁드립니다.
    /// 추가로 API를 통한 00:00 ~ 15:40 이외의 시간은 당일 조회가 제한되는 점 이용에 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPTJ04030000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market
    ///
    /// [국내주식] 시세분석
    /// 시장별 투자자매매동향(시세)[v1_국내주식-074]
    /// frgn_seln_vol
    /// frgn_shnu_vol
    /// frgn_ntby_qty
    /// frgn_seln_tr_pbmn
    /// frgn_shnu_tr_pbmn
    /// frgn_ntby_tr_pbmn
    /// prsn_seln_vol
    /// prsn_shnu_vol
    /// prsn_ntby_qty
    /// prsn_seln_tr_pbmn
    /// prsn_shnu_tr_pbmn
    /// prsn_ntby_tr_pbmn
    /// orgn_seln_vol
    /// orgn_shnu_vol
    /// orgn_ntby_qty
    /// orgn_seln_tr_pbmn
    /// orgn_shnu_tr_pbmn
    /// orgn_ntby_tr_pbmn
    /// scrt_seln_vol
    /// scrt_shnu_vol
    /// scrt_ntby_qty
    /// scrt_seln_tr_pbmn
    /// scrt_shnu_tr_pbmn
    /// scrt_ntby_tr_pbmn
    /// ivtr_seln_vol
    /// ivtr_shnu_vol
    /// ivtr_ntby_qty
    /// ivtr_seln_tr_pbmn
    /// ivtr_shnu_tr_pbmn
    /// ivtr_ntby_tr_pbmn
    /// pe_fund_seln_tr_pbmn
    /// pe_fund_seln_vol
    /// pe_fund_ntby_vol
    /// pe_fund_shnu_tr_pbmn
    /// pe_fund_shnu_vol
    /// pe_fund_ntby_tr_pbmn
    /// bank_seln_vol
    /// bank_shnu_vol
    /// bank_ntby_qty
    /// bank_seln_tr_pbmn
    /// bank_shnu_tr_pbmn
    /// bank_ntby_tr_pbmn
    /// insu_seln_vol
    /// insu_shnu_vol
    /// insu_ntby_qty
    /// insu_seln_tr_pbmn
    /// insu_shnu_tr_pbmn
    /// insu_ntby_tr_pbmn
    /// mrbn_seln_vol
    /// mrbn_shnu_vol
    /// mrbn_ntby_qty
    /// mrbn_seln_tr_pbmn
    /// mrbn_shnu_tr_pbmn
    /// mrbn_ntby_tr_pbmn
    /// fund_seln_vol
    /// fund_shnu_vol
    /// fund_ntby_qty
    /// fund_seln_tr_pbmn
    /// fund_shnu_tr_pbmn
    /// fund_ntby_tr_pbmn
    /// etc_orgt_seln_vol
    /// etc_orgt_shnu_vol
    /// etc_orgt_ntby_vol
    /// etc_orgt_seln_tr_pbmn
    /// etc_orgt_shnu_tr_pbmn
    /// etc_orgt_ntby_tr_pbmn
    /// etc_corp_seln_vol
    /// etc_corp_shnu_vol
    /// etc_corp_ntby_vol
    /// etc_corp_seln_tr_pbmn
    /// etc_corp_shnu_tr_pbmn
    /// etc_corp_ntby_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 시장별 투자자매매동향(시세성) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0403] 시장별 시간동향 의 상단 표 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPTJ04040000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market
    ///
    /// [국내주식] 시세분석
    /// 시장별 투자자매매동향(일별) [국내주식-075]
    /// stck_bsop_date
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// bstp_nmix_prdy_ctrt
    /// bstp_nmix_oprc
    /// bstp_nmix_hgpr
    /// bstp_nmix_lwpr
    /// stck_prdy_clpr
    /// frgn_ntby_qty
    /// frgn_reg_ntby_qty
    /// frgn_nreg_ntby_qty
    /// prsn_ntby_qty
    /// orgn_ntby_qty
    /// scrt_ntby_qty
    /// ivtr_ntby_qty
    /// pe_fund_ntby_vol
    /// bank_ntby_qty
    /// insu_ntby_qty
    /// mrbn_ntby_qty
    /// fund_ntby_qty
    /// etc_ntby_qty
    /// etc_orgt_ntby_vol
    /// etc_corp_ntby_vol
    /// frgn_ntby_tr_pbmn
    /// frgn_reg_ntby_pbmn
    /// frgn_nreg_ntby_pbmn
    /// prsn_ntby_tr_pbmn
    /// orgn_ntby_tr_pbmn
    /// scrt_ntby_tr_pbmn
    /// ivtr_ntby_tr_pbmn
    /// pe_fund_ntby_tr_pbmn
    /// bank_ntby_tr_pbmn
    /// insu_ntby_tr_pbmn
    /// mrbn_ntby_tr_pbmn
    /// fund_ntby_tr_pbmn
    /// etc_ntby_tr_pbmn
    /// etc_orgt_ntby_tr_pbmn
    /// etc_corp_ntby_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 시장별 투자자매매동향(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0404] 시장별 일별동향 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST644400C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-pchs-trend
    ///
    /// [국내주식] 시세분석
    /// 종목별 외국계 순매수추이 [국내주식-164]
    /// bsop_hour
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// frgn_seln_vol
    /// frgn_shnu_vol
    /// glob_ntby_qty
    /// frgn_ntby_qty_icdc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 종목별 외국계 순매수추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0433] 종목별 외국계 순매수추이 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST04320000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/frgnmem-trade-trend
    ///
    /// [국내주식] 시세분석
    /// 회원사 실시간 매매동향(틱) [국내주식-163]
    /// total_seln_qty
    /// total_shnu_qty
    /// bsop_hour
    /// mbcr_name
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// cntg_vol
    /// acml_ntby_qty
    /// glob_ntby_qty
    /// frgn_ntby_qty_icdc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 회원사 실시간 매매동향(틱) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0432] 회원사 실시간 매매동향 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 최근 100건까지 데이터 조회 가능합니다.
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
    ///
    /// - TR_ID: Real=FHPST04540000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-member-daily
    ///
    /// [국내주식] 시세분석
    /// 주식현재가 회원사 종목매매동향 [국내주식-197]
    /// stck_bsop_date
    /// total_seln_qty
    /// total_shnu_qty
    /// ntby_qty
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 주식현재가 회원사 종목매매동향 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0454] 증권사 종목매매동향 화면을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPPG04650101 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/program-trade-by-stock
    ///
    /// [국내주식] 시세분석
    /// 종목별 프로그램매매추이(체결)[v1_국내주식-044]
    /// bsop_hour
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// whol_smtn_seln_vol
    /// whol_smtn_shnu_vol
    /// whol_smtn_ntby_qty
    /// whol_smtn_seln_tr_pbmn
    /// whol_smtn_shnu_tr_pbmn
    /// whol_smtn_ntby_tr_pbmn
    /// whol_ntby_vol_icdc
    /// whol_ntby_tr_pbmn_icdc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 종목별 프로그램매매추이(체결) API입니다.
    ///
    /// 한국투자 HTS(eFriend Plus) > [0465] 종목별 프로그램 매매추이 화면(혹은 한국투자 MTS > 국내 현재가 > 기타수급 > 프로그램) 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPPG04650201 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily
    ///
    /// [국내주식] 시세분석
    /// 종목별 프로그램매매추이(일별) [국내주식-113]
    /// stck_bsop_date
    /// stck_clpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// whol_smtn_seln_vol
    /// whol_smtn_shnu_vol
    /// whol_smtn_ntby_qty
    /// whol_smtn_seln_tr_pbmn
    /// whol_smtn_shnu_tr_pbmn
    /// whol_smtn_ntby_tr_pbmn
    /// whol_ntby_vol_icdc
    /// whol_ntby_tr_pbmn_icdc2
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 종목별 프로그램매매추이(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0465] 종목별 프로그램 매매추이 화면(혹은 한국투자 MTS > 국내 현재가 > 기타수급 > 프로그램) 의 "일자별" 클릭 시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=HHPTJ04160200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/investor-trend-estimate
    ///
    /// [국내주식] 시세분석
    /// 종목별 외인기관 추정가집계[v1_국내주식-046]
    /// bsop_hour_gb
    /// frgn_fake_ntby_qty
    /// orgn_fake_ntby_qty
    /// sum_fake_ntby_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 종목별 외국인, 기관 추정가집계 API입니다.
    ///
    /// 한국투자 MTS > 국내 현재가 > 투자자 > 투자자동향 탭 > 왼쪽구분을 '추정(주)'로 선택 시 확인 가능한 데이터를 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 증권사 직원이 장중에 집계/입력한 자료를 단순 누계한 수치로서,
    /// 입력시간은 외국인 09:30, 11:20, 13:20, 14:30 / 기관종합 10:00, 11:20, 13:20, 14:30 이며, 사정에 따라 변동될 수 있습니다.
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
    ///
    /// - TR_ID: Real=FHKST03010800 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/inquire-daily-trade-volume
    ///
    /// [국내주식] 시세분석
    /// 종목별일별매수매도체결량 [v1_국내주식-056]
    /// shnu_cnqn_smtn
    /// seln_cnqn_smtn
    /// stck_bsop_date
    /// total_seln_qty
    /// total_shnu_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 종목별일별매수매도체결량 API입니다. 실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능합니다.
    /// 국내주식 종목의 일별 매수체결량, 매도체결량 데이터를 확인할 수 있습니다.
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
    ///
    /// - TR_ID: Real=FHPPG04600101 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/comp-program-trade-today
    ///
    /// [국내주식] 시세분석
    /// 프로그램매매 종합현황(시간) [국내주식-114]
    /// bsop_hour
    /// arbt_smtn_seln_tr_pbmn
    /// arbt_smtm_seln_tr_pbmn_rate
    /// arbt_smtn_shnu_tr_pbmn
    /// arbt_smtm_shun_tr_pbmn_rate
    /// nabt_smtn_seln_tr_pbmn
    /// nabt_smtm_seln_tr_pbmn_rate
    /// nabt_smtn_shnu_tr_pbmn
    /// nabt_smtm_shun_tr_pbmn_rate
    /// arbt_smtn_ntby_tr_pbmn
    /// arbt_smtm_ntby_tr_pbmn_rate
    /// nabt_smtn_ntby_tr_pbmn
    /// nabt_smtm_ntby_tr_pbmn_rate
    /// whol_smtn_ntby_tr_pbmn
    /// whol_ntby_tr_pbmn_rate
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 프로그램매매 종합현황(시간) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0460] 프로그램매매 종합현황 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 장시간(09:00~15:30) 동안의 최근 30분간의 데이터 확인이 가능하며, 다음조회가 불가합니다.
    /// ※ 장시간(09:00~15:30) 이후에는 bsop_hour 에 153000 ~ 170000 까지의 시간데이터가 출력되지만 데이터는 모두 동일한 장마감 데이터인 점 유의 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPPG04600001 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/comp-program-trade-daily
    ///
    /// [국내주식] 시세분석
    /// 프로그램매매 종합현황(일별)[국내주식-115]
    /// stck_bsop_date
    /// nabt_entm_seln_tr_pbmn
    /// nabt_onsl_seln_vol
    /// whol_onsl_seln_tr_pbmn
    /// arbt_smtn_shnu_vol
    /// nabt_smtn_shnu_tr_pbmn
    /// arbt_entm_ntby_qty
    /// nabt_entm_ntby_tr_pbmn
    /// arbt_entm_seln_vol
    /// nabt_entm_seln_vol_rate
    /// nabt_onsl_seln_vol_rate
    /// whol_onsl_seln_tr_pbmn_rate
    /// arbt_smtm_shun_vol_rate
    /// nabt_smtm_shun_tr_pbmn_rate
    /// arbt_entm_ntby_qty_rate
    /// nabt_entm_ntby_tr_pbmn_rate
    /// arbt_entm_seln_vol_rate
    /// nabt_entm_seln_tr_pbmn_rate
    /// nabt_onsl_seln_tr_pbmn
    /// whol_smtn_seln_vol
    /// arbt_smtn_shnu_tr_pbmn
    /// whol_entm_shnu_vol
    /// arbt_entm_ntby_tr_pbmn
    /// nabt_onsl_ntby_qty
    /// arbt_entm_seln_tr_pbmn
    /// nabt_onsl_seln_tr_pbmn_rate
    /// whol_seln_vol_rate
    /// arbt_smtm_shun_tr_pbmn_rate
    /// whol_entm_shnu_vol_rate
    /// arbt_entm_ntby_tr_pbmn_rate
    /// nabt_onsl_ntby_qty_rate
    /// arbt_entm_seln_tr_pbmn_rate
    /// nabt_smtn_seln_vol
    /// whol_smtn_seln_tr_pbmn
    /// nabt_entm_shnu_vol
    /// whol_entm_shnu_tr_pbmn
    /// arbt_onsl_ntby_qty
    /// nabt_onsl_ntby_tr_pbmn
    /// arbt_onsl_seln_tr_pbmn
    /// nabt_smtm_seln_vol_rate
    /// whol_seln_tr_pbmn_rate
    /// nabt_entm_shnu_vol_rate
    /// whol_entm_shnu_tr_pbmn_rate
    /// arbt_onsl_ntby_qty_rate
    /// nabt_onsl_ntby_tr_pbmn_rate
    /// arbt_onsl_seln_tr_pbmn_rate
    /// nabt_smtn_seln_tr_pbmn
    /// arbt_entm_shnu_vol
    /// nabt_entm_shnu_tr_pbmn
    /// whol_onsl_shnu_vol
    /// arbt_onsl_ntby_tr_pbmn
    /// nabt_smtn_ntby_qty
    /// arbt_onsl_seln_vol
    /// nabt_smtm_seln_tr_pbmn_rate
    /// arbt_entm_shnu_vol_rate
    /// nabt_entm_shnu_tr_pbmn_rate
    /// whol_onsl_shnu_tr_pbmn
    /// arbt_onsl_ntby_tr_pbmn_rate
    /// nabt_smtm_ntby_qty_rate
    /// arbt_onsl_seln_vol_rate
    /// whol_entm_seln_vol
    /// arbt_entm_shnu_tr_pbmn
    /// nabt_onsl_shnu_vol
    /// whol_onsl_shnu_tr_pbmn_rate
    /// arbt_smtn_ntby_qty
    /// nabt_smtn_ntby_tr_pbmn
    /// arbt_smtn_seln_vol
    /// whol_entm_seln_tr_pbmn
    /// arbt_entm_shnu_tr_pbmn_rate
    /// nabt_onsl_shnu_vol_rate
    /// whol_onsl_shnu_vol_rate
    /// arbt_smtm_ntby_qty_rate
    /// nabt_smtm_ntby_tr_pbmn_rate
    /// arbt_smtm_seln_vol_rate
    /// whol_entm_seln_vol_rate
    /// arbt_onsl_shnu_vol
    /// nabt_onsl_shnu_tr_pbmn
    /// whol_smtn_shnu_vol
    /// arbt_smtn_ntby_tr_pbmn
    /// whol_entm_ntby_qty
    /// arbt_smtn_seln_tr_pbmn
    /// whol_entm_seln_tr_pbmn_rate
    /// arbt_onsl_shnu_vol_rate
    /// nabt_onsl_shnu_tr_pbmn_rate
    /// whol_shun_vol_rate
    /// arbt_smtm_ntby_tr_pbmn_rate
    /// whol_entm_ntby_qty_rate
    /// arbt_smtm_seln_tr_pbmn_rate
    /// whol_onsl_seln_vol
    /// arbt_onsl_shnu_tr_pbmn
    /// nabt_smtn_shnu_vol
    /// whol_smtn_shnu_tr_pbmn
    /// nabt_entm_ntby_qty
    /// whol_entm_ntby_tr_pbmn
    /// nabt_entm_seln_vol
    /// whol_onsl_seln_vol_rate
    /// arbt_onsl_shnu_tr_pbmn_rate
    /// nabt_smtm_shun_vol_rate
    /// whol_shun_tr_pbmn_rate
    /// nabt_entm_ntby_qty_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 프로그램매매 종합현황(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0460] 프로그램매매 종합현황 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// * 8개월 이상 과거 조회는 불가하며 에러메시지가 발생합니다.
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
    ///
    /// - TR_ID: Real=HHPPG046600C1 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/investor-program-trade-today
    ///
    /// [국내주식] 시세분석
    /// 프로그램매매 투자자매매동향(당일) [국내주식-116]
    /// invr_cls_code
    /// all_seln_qty
    /// all_seln_amt
    /// invr_cls_name
    /// all_shnu_qty
    /// all_shnu_amt
    /// all_ntby_amt
    /// arbt_seln_qty
    /// all_ntby_qty
    /// arbt_shnu_qty
    /// arbt_ntby_qty
    /// arbt_seln_amt
    /// arbt_shnu_amt
    /// arbt_ntby_amt
    /// nabt_seln_qty
    /// nabt_shnu_qty
    /// nabt_ntby_qty
    /// nabt_seln_amt
    /// nabt_shnu_amt
    /// nabt_ntby_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 프로그램매매 투자자매매동향(당일) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0466] 프로그램매매 투자자별 동향 화면 의 "당일동향" 표의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST04760000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/daily-credit-balance
    ///
    /// [국내주식] 시세분석
    /// 국내주식 신용잔고 일별추이[국내주식-110]
    /// deal_date
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// stlm_date
    /// whol_loan_new_stcn
    /// whol_loan_rdmp_stcn
    /// whol_loan_rmnd_stcn
    /// whol_loan_new_amt
    /// whol_loan_rdmp_amt
    /// whol_loan_rmnd_amt
    /// whol_loan_rmnd_rate
    /// whol_loan_gvrt
    /// whol_stln_new_stcn
    /// whol_stln_rdmp_stcn
    /// whol_stln_rmnd_stcn
    /// whol_stln_new_amt
    /// whol_stln_rdmp_amt
    /// whol_stln_rmnd_amt
    /// whol_stln_rmnd_rate
    /// whol_stln_gvrt
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 신용잔고 일별추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0476] 국내주식 신용잔고 일별추이 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 한 번의 호출에 최대 30건 확인 가능하며, fid_input_date_1 을 입력하여 다음 조회가 가능합니다.
    ///
    /// ※ 상환수량은 "매도상환수량+현금상환수량"의 합계 수치입니다.
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
    ///
    /// - TR_ID: Real=FHPST01810000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/exp-price-trend
    ///
    /// [국내주식] 시세분석
    /// 국내주식 예상체결가 추이[국내주식-118]
    /// rprs_mrkt_kor_name
    /// antc_cnpr
    /// antc_cntg_vrss_sign
    /// antc_cntg_vrss
    /// antc_cntg_prdy_ctrt
    /// antc_vol
    /// antc_tr_pbmn
    /// stck_bsop_date
    /// stck_cntg_hour
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 예상체결가 추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0184] 예상체결지수 추이 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
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
    ///
    /// - TR_ID: Real=FHPST04830000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/daily-short-sale
    ///
    /// [국내주식] 시세분석
    /// 국내주식 공매도 일별추이[국내주식-134]
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// prdy_vol
    /// stck_bsop_date
    /// stck_clpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// stnd_vol_smtn
    /// ssts_cntg_qty
    /// ssts_vol_rlim
    /// acml_ssts_cntg_qty
    /// acml_ssts_cntg_qty_rlim
    /// acml_tr_pbmn
    /// stnd_tr_pbmn_smtn
    /// ssts_tr_pbmn
    /// ssts_tr_pbmn_rlim
    /// acml_ssts_tr_pbmn
    /// acml_ssts_tr_pbmn_rlim
    /// stck_oprc
    /// stck_hgpr
    /// stck_lwpr
    /// avrg_prc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
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
    ///
    /// - TR_ID: Real=FHKST111900C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/tradprt-byamt
    ///
    /// [국내주식] 시세분석
    /// 국내주식 체결금액별 매매비중 [국내주식-192]
    /// prpr_name
    /// smtn_avrg_prpr
    /// acml_vol
    /// whol_ntby_qty_rate
    /// ntby_cntg_csnu
    /// seln_cnqn_smtn
    /// whol_seln_vol_rate
    /// seln_cntg_csnu
    /// shnu_cnqn_smtn
    /// whol_shun_vol_rate
    /// shnu_cntg_csnu
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 체결금액별 매매비중 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0135] 체결금액별 매매비중 화면의 "상단 표" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST649100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/mktfunds
    ///
    /// [국내주식] 시세분석
    /// 국내 증시자금 종합 [국내주식-193]
    /// bsop_date
    /// bstp_nmix_prpr
    /// bstp_nmix_prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// hts_avls
    /// cust_dpmn_amt
    /// cust_dpmn_amt_prdy_vrss
    /// amt_tnrt
    /// uncl_amt
    /// crdt_loan_rmnd
    /// futs_tfam_amt
    /// sttp_amt
    /// mxtp_amt
    /// bntp_amt
    /// mmf_amt
    /// secu_lend_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내 증시자금 종합 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0470] 증시자금 종합 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다. (단위: 억원)
    ///
    /// ※ 해당자료는 금융투자협회의 자료를 제공하고 있으며, 오류와 지연이 발생할 수 있습니다.
    /// ※ 위 정보에 의한 투자판단의 최종책임은 정보이용자에게 있으며, 당사와 한국금융투자협회는 어떠한 법적인 책임도 지지 않사오니 투자에 참고로만 이용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHPST074500C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/daily-loan-trans
    ///
    /// [국내주식] 시세분석
    /// 종목별 일별 대차거래추이 [국내주식-135]
    /// bsop_date
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// new_stcn
    /// rdmp_stcn
    /// prdy_rmnd_vrss
    /// rmnd_stcn
    /// rmnd_amt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 종목별 일별 대차거래추이 API입니다.
    /// 한 번의 조회에 최대 100건까지 조회 가능하며, start_date, end_date 를 수정하여 다음 조회가 가능합니다.
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
    ///
    /// - TR_ID: Real=FHKST130000C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/capture-uplowprice
    ///
    /// [국내주식] 시세분석
    /// 국내주식 상하한가 포착 [국내주식-190]
    /// mksc_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// askp_rsqn1
    /// bidp_rsqn1
    /// prdy_vol
    /// seln_cnqn
    /// shnu_cnqn
    /// stck_llam
    /// stck_mxpr
    /// prdy_vrss_vol_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 상하한가 포착 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0917] 실시간 상하한가 포착 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST01130000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/pbar-tratio
    ///
    /// [국내주식] 시세분석
    /// 국내주식 매물대/거래비중 [국내주식-196]
    /// rprs_mrkt_kor_name
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// prdy_vol
    /// wghn_avrg_stck_prc
    /// lstn_stcn
    /// data_rank
    /// stck_prpr
    /// cntg_vol
    /// acml_vol_rlim
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 매물대/거래비중 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0113] 당일가격대별 매물대 화면의 데이터 중 일부를 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST01710000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/quotations/volume-rank
    ///
    /// [국내주식] 순위분석
    /// 거래량순위[v1_국내주식-047]
    /// hts_kor_isnm
    /// mksc_shrn_iscd
    /// data_rank
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// prdy_vol
    /// lstn_stcn
    /// avrg_vol
    /// n_befr_clpr_vrss_prpr_rate
    /// vol_inrt
    /// vol_tnrt
    /// nday_vol_tnrt
    /// avrg_tr_pbmn
    /// tr_pbmn_tnrt
    /// nday_tr_pbmn_tnrt
    /// acml_tr_pbmn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 거래량순위 API입니다.
    ///
    /// 한국투자 HTS(eFriend Plus) > [0171] 거래량 순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    /// +
    /// 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// HTS [0110]에서 여러가지 조건을 설정할 수 있는데, 그 중 거래량 순위(ex. 0봉전 거래량 상위순 100종목) 에 대해서도 설정해서 종목을 검색할 수 있습니다.
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHKST66430100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/balance-sheet
    ///
    /// [국내주식] 종목정보
    /// 국내주식 대차대조표[v1_국내주식-078]
    /// stac_yymm
    /// total_aset
    /// flow_lblt
    /// fix_lblt
    /// total_lblt
    /// cfp_surp
    /// prfi_surp
    /// total_cptl
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 대차대조표 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '1. 대차대조표' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST66430200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/income-statement
    ///
    /// [국내주식] 종목정보
    /// 국내주식 손익계산서[v1_국내주식-079]
    /// stac_yymm
    /// sale_account
    /// sale_cost
    /// sale_totl_prfi
    /// depr_cost
    /// sell_mang
    /// bsop_prti
    /// bsop_non_ernn
    /// bsop_non_expn
    /// op_prfi
    /// spec_prfi
    /// spec_loss
    /// thtr_ntin
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 손익계산서 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '2. 손익계산서' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST66430300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/financial-ratio
    ///
    /// [국내주식] 종목정보
    /// 국내주식 재무비율[v1_국내주식-080]
    /// stac_yymm
    /// bsop_prfi_inrt
    /// ntin_inrt
    /// roe_val
    /// rsrv_rate
    /// lblt_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 재무비율 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 우측의 '재무 비율' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST66430400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/profit-ratio
    ///
    /// [국내주식] 종목정보
    /// 국내주식 수익성비율[v1_국내주식-081]
    /// stac_yymm
    /// cptl_ntin_rate
    /// self_cptl_ntin_inrt
    /// sale_ntin_rate
    /// sale_totl_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 수익성비율 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '4. 수익성비율' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST66430500 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/other-major-ratios
    ///
    /// [국내주식] 종목정보
    /// 국내주식 기타주요비율[v1_국내주식-082]
    /// stac_yymm
    /// payout_rate
    /// ebitda
    /// ev_ebitda
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 기타주요비율 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '9. 기타주요비율' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST66430600 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/stability-ratio
    ///
    /// [국내주식] 종목정보
    /// 국내주식 안정성비율[v1_국내주식-083]
    /// stac_yymm
    /// lblt_rate
    /// bram_depn
    /// crnt_rate
    /// quck_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 안정성비율 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '5. 안정성비율' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHKST66430800 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/finance/growth-ratio
    ///
    /// [국내주식] 종목정보
    /// 국내주식 성장성비율[v1_국내주식-085]
    /// stac_yymm
    /// bsop_prfi_inrt
    /// equt_inrt
    /// totl_aset_inrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 성장성비율 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0635] 재무분석종합 화면의 하단 '7.성장성비율' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=HHKDB669102C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/dividend
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(배당일정)[국내주식-145]
    /// record_date
    /// sht_cd
    /// isin_name
    /// divi_kind
    /// face_val
    /// per_sto_divi_amt
    /// divi_rate
    /// stk_divi_rate
    /// divi_pay_dt
    /// stk_div_pay_dt
    /// odd_pay_dt
    /// stk_kind
    /// high_divi_gb
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(배당일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0658] 배당 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
    /// '주식배당지급일'은 배당주식의 주식교부일자를 말합니다. 배당주식의 계좌입고는 배당주식 상장일인데 일반적으로 주권교부일의 익영업일입니다.
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
    ///
    /// - TR_ID: Real=HHKDB669103C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/purreq
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(주식매수청구일정)[국내주식-146]
    /// record_date
    /// sht_cd
    /// isin_name
    /// stk_kind
    /// opp_opi_rcpt_term
    /// buy_req_rcpt_term
    /// buy_req_price
    /// buy_amt_pay_dt
    /// get_meet_dt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(주식매수청구일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0663] 주식매수청구 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669104C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/merger-split
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(합병/분할일정)[국내주식-147]
    /// record_date
    /// sht_cd
    /// opp_cust_cd
    /// opp_cust_nm
    /// cust_cd
    /// cust_nm
    /// merge_type
    /// merge_rate
    /// td_stop_dt
    /// list_dt
    /// odd_amt_pay_dt
    /// tot_issue_stk_qty
    /// issue_stk_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(합병/분할일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0664] 합병/분할 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669105C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/rev-split
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(액면교체일정)[국내주식-148]
    /// record_date
    /// sht_cd
    /// isin_name
    /// inter_bf_face_amt
    /// inter_af_face_amt
    /// td_stop_dt
    /// list_dt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(액면교체일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0657] 액면교체 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669106C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/cap-dcrs
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(자본감소일정)[국내주식-149]
    /// record_date
    /// sht_cd
    /// isin_name
    /// stk_kind
    /// reduce_cap_type
    /// reduce_cap_rate
    /// comp_way
    /// td_stop_dt
    /// list_dt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(자본감소일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0665] 자본감소 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669107C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/list-info
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(상장정보일정)[국내주식-150]
    /// list_dt
    /// sht_cd
    /// isin_name
    /// stk_kind
    /// issue_type
    /// issue_stk_qty
    /// tot_issue_stk_qty
    /// issue_price
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(상장정보일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0666] 상장정보 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669108C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/pub-offer
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(공모주청약일정)[국내주식-151]
    /// record_date
    /// sht_cd
    /// isin_name
    /// fix_subscr_pri
    /// face_value
    /// subscr_dt
    /// pay_dt
    /// refund_dt
    /// list_dt
    /// lead_mgr
    /// pub_bf_cap
    /// pub_af_cap
    /// assign_stk_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(공모주청약일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0667] 공모주청약 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669109C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/forfeit
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(실권주일정)[국내주식-152]
    /// record_date
    /// sht_cd
    /// isin_name
    /// subscr_dt
    /// subscr_price
    /// subscr_stk_qty
    /// refund_dt
    /// list_dt
    /// lead_mgr
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(실권주일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0668] 실권주 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669110C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/mand-deposit
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(의무예치일정)[국내주식-153]
    /// sht_cd
    /// isin_name
    /// stk_qty
    /// depo_date
    /// depo_reason
    /// tot_issue_qty_per_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(의무예치일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0758] 의무예치 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/paidin-capin
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(유상증자일정) [국내주식-143]
    /// record_date
    /// sht_cd
    /// isin_name
    /// tot_issue_stk_qty
    /// issue_stk_qty
    /// fix_rate
    /// disc_rate
    /// fix_price
    /// right_dt
    /// sub_term_ft
    /// sub_term
    /// list_date
    /// stk_kind
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(유상증자일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0655] 유상증자 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669101C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/bonus-issue
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(무상증자일정) [국내주식-144]
    /// record_date
    /// sht_cd
    /// isin_name
    /// fix_rate
    /// odd_rec_price
    /// right_dt
    /// odd_pay_dt
    /// list_date
    /// tot_issue_stk_qty
    /// issue_stk_qty
    /// stk_kind
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(무상증자일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0656] 무상증자 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=HHKDB669111C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ksdinfo/sharehld-meet
    ///
    /// [국내주식] 종목정보
    /// 예탁원정보(주주총회일정) [국내주식-154]
    /// record_date
    /// sht_cd
    /// isin_name
    /// gen_meet_dt
    /// gen_meet_type
    /// agenda
    /// vote_tot_qty
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 예탁원정보(주주총회일정) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0759] 주주총회 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 예탁원에서 제공한 자료이므로 정보용으로만 사용하시기 바랍니다.
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
    ///
    /// - TR_ID: Real=FHKST11860000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/overtime-exp-trans-fluct
    ///
    /// [국내주식] 시세분석
    /// 국내주식 시간외예상체결등락률 [국내주식-140]
    /// data_rank
    /// iscd_stat_cls_code
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// ovtm_untp_antc_cnpr
    /// ovtm_untp_antc_cntg_vrss
    /// ovtm_untp_antc_cntg_vrsssign
    /// ovtm_untp_antc_cntg_ctrt
    /// ovtm_untp_askp_rsqn1
    /// ovtm_untp_bidp_rsqn1
    /// ovtm_untp_antc_cnqn
    /// itmt_vol
    /// stck_prpr
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외예상체결등락률 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0236] 시간외 예상체결등락률 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
    ///
    /// - TR_ID: Real=FHPST01700000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/fluctuation
    ///
    /// [국내주식] 순위분석
    /// 국내주식 등락률 순위[v1_국내주식-088]
    /// stck_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// stck_hgpr
    /// hgpr_hour
    /// acml_hgpr_date
    /// stck_lwpr
    /// lwpr_hour
    /// acml_lwpr_date
    /// lwpr_vrss_prpr_rate
    /// dsgt_date_clpr_vrss_prpr_rate
    /// cnnt_ascn_dynu
    /// hgpr_vrss_prpr_rate
    /// cnnt_down_dynu
    /// oprc_vrss_prpr_sign
    /// oprc_vrss_prpr
    /// oprc_vrss_prpr_rate
    /// prd_rsfl
    /// prd_rsfl_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 등락률 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0170] 등락률 순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01720000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/quote-balance
    ///
    /// [국내주식] 순위분석
    /// 국내주식 호가잔량 순위[국내주식-089]
    /// mksc_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// total_ntsl_bidp_rsqn
    /// shnu_rsqn_rate
    /// seln_rsqn_rate
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 호가잔량 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0172] 호가잔량 순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01730000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/profit-asset-index
    ///
    /// [국내주식] 순위분석
    /// 국내주식 수익자산지표 순위[v1_국내주식-090]
    /// data_rank
    /// hts_kor_isnm
    /// prdy_vrss_sign
    /// mksc_shrn_iscd
    /// stck_prpr
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// sale_totl_prfi
    /// bsop_prti
    /// op_prfi
    /// thtr_ntin
    /// total_aset
    /// total_lblt
    /// total_cptl
    /// stac_month
    /// stac_month_cls_code
    /// iqry_csnu
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 수익자산지표 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0173] 수익자산지표 순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01740000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/market-cap
    ///
    /// [국내주식] 순위분석
    /// 국내주식 시가총액 상위[v1_국내주식-091]
    /// mksc_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// lstn_stcn
    /// stck_avls
    /// mrkt_whol_avls_rlim
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시가총액 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0174] 시가총액 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01750000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/finance-ratio
    ///
    /// [국내주식] 순위분석
    /// 국내주식 재무비율 순위[v1_국내주식-092]
    /// data_rank
    /// hts_kor_isnm
    /// mksc_shrn_iscd
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// cptl_op_prfi
    /// cptl_ntin_rate
    /// sale_totl_rate
    /// sale_ntin_rate
    /// lblt_rate
    /// bram_depn
    /// rsrv_rate
    /// op_prfi_inrt
    /// bsop_prfi_inrt
    /// ntin_inrt
    /// equt_inrt
    /// cptl_tnrt
    /// sale_bond_tnrt
    /// totl_aset_inrt
    /// stac_month
    /// stac_month_cls_code
    /// iqry_csnu
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 재무비율 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0175] 재무비율순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01760000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/after-hour-balance
    ///
    /// [국내주식] 순위분석
    /// 국내주식 시간외잔량 순위[v1_국내주식-093]
    /// stck_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// ovtm_total_askp_rsqn
    /// ovtm_total_bidp_rsqn
    /// mkob_otcp_vol
    /// mkfa_otcp_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외잔량 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0176] 시간외잔량 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01770000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/prefer-disparate-ratio
    ///
    /// [국내주식] 순위분석
    /// 국내주식 우선주/괴리율 상위[v1_국내주식-094]
    /// mksc_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// acml_vol
    /// prst_iscd
    /// prst_kor_isnm
    /// prst_prpr
    /// prst_prdy_vrss
    /// prst_prdy_vrss_sign
    /// prst_acml_vol
    /// diff_prpr
    /// prdy_ctrt
    /// prst_prdy_ctrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 우선주/괴리율 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0177] 우선주/괴리율 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01780000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/disparity
    ///
    /// [국내주식] 순위분석
    /// 국내주식 이격도 순위[v1_국내주식-095]
    /// mksc_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_ctrt
    /// prdy_vrss_sign
    /// acml_vol
    /// d5_dsrt
    /// d10_dsrt
    /// d20_dsrt
    /// d60_dsrt
    /// d120_dsrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 이격도 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0178] 이격도 순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01790000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/market-value
    ///
    /// [국내주식] 순위분석
    /// 국내주식 시장가치 순위[v1_국내주식-096]
    /// data_rank
    /// hts_kor_isnm
    /// mksc_shrn_iscd
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// ebitda
    /// pv_div_ebitda
    /// ebitda_div_fnnc_expn
    /// stac_month
    /// stac_month_cls_code
    /// iqry_csnu
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시장가치 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0179] 시장가치순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01680000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/volume-power
    ///
    /// [국내주식] 순위분석
    /// 국내주식 체결강도 상위[v1_국내주식-101]
    /// stck_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// tday_rltv
    /// seln_cnqn_smtn
    /// shnu_cnqn_smtn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 체결강도 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0168] 체결강도 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01800000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/top-interest-stock
    ///
    /// [국내주식] 순위분석
    /// 국내주식 관심종목등록 상위[v1_국내주식-102]
    /// mrkt_div_cls_name
    /// mksc_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// data_rank
    /// inter_issu_reg_csnu
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 관심종목등록 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0180] 관심종목등록상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01820000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/exp-trans-updown
    ///
    /// [국내주식] 순위분석
    /// 국내주식 예상체결 상승/하락상위[v1_국내주식-103]
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// stck_sdpr
    /// seln_rsqn
    /// shnu_rsqn
    /// cntg_vol
    /// antc_tr_pbmn
    /// total_askp_rsqn
    /// total_bidp_rsqn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 예상체결 상승/하락상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0182] 예상체결 상승/하락상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01860000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/traded-by-company
    ///
    /// [국내주식] 순위분석
    /// 국내주식 당사매매종목 상위[v1_국내주식-104]
    /// data_rank
    /// mksc_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// seln_cnqn_smtn
    /// shnu_cnqn_smtn
    /// ntby_cnqn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 당사매매종목 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0186] 당사매매종목 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST01870000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/near-new-highlow
    ///
    /// [국내주식] 순위분석
    /// 국내주식 신고/신저근접종목 상위[v1_국내주식-105]
    /// hts_kor_isnm
    /// mksc_shrn_iscd
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// askp_rsqn1
    /// bidp_rsqn1
    /// acml_vol
    /// new_hgpr
    /// hprc_near_rate
    /// new_lwpr
    /// lwpr_near_rate
    /// stck_sdpr
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 신고/신저근접종목 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0187] 신고/신저 근접종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=HHKDB13470100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/dividend-rate
    ///
    /// [국내주식] 순위분석
    /// 국내주식 배당률 상위[국내주식-106]
    /// sht_cd
    /// isin_name
    /// record_date
    /// per_sto_divi_amt
    /// divi_rate
    /// divi_kind
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 배당률 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0188] 배당률 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHKST190900C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/bulk-trans-num
    ///
    /// [국내주식] 순위분석
    /// 국내주식 대량체결건수 상위[국내주식-107]
    /// mksc_shrn_iscd
    /// data_rank
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss_sign
    /// prdy_vrss
    /// prdy_ctrt
    /// acml_vol
    /// shnu_cntg_csnu
    /// seln_cntg_csnu
    /// ntby_cnqn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 대량체결건수 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0169] 대량체결건수 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHKST17010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/credit-balance
    ///
    /// [국내주식] 순위분석
    /// 국내주식 신용잔고 상위[국내주식-109]
    /// bstp_cls_code
    /// hts_kor_isnm
    /// stnd_date1
    /// stnd_date2
    /// mksc_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// whol_loan_rmnd_stcn
    /// whol_loan_rmnd_amt
    /// whol_loan_rmnd_rate
    /// whol_stln_rmnd_stcn
    /// whol_stln_rmnd_amt
    /// whol_stln_rmnd_rate
    /// nday_vrss_loan_rmnd_inrt
    /// nday_vrss_stln_rmnd_inrt
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 신용잔고 상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0475] 신용잔고 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST04820000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/short-sale
    ///
    /// [국내주식] 순위분석
    /// 국내주식 공매도 상위종목[국내주식-133]
    /// mksc_shrn_iscd
    /// hts_kor_isnm
    /// stck_prpr
    /// prdy_vrss
    /// prdy_vrss_sign
    /// prdy_ctrt
    /// acml_vol
    /// acml_tr_pbmn
    /// ssts_cntg_qty
    /// ssts_vol_rlim
    /// ssts_tr_pbmn
    /// ssts_tr_pbmn_rlim
    /// stnd_date1
    /// stnd_date2
    /// avrg_prc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 공매도 상위종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0482] 공매도 상위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
    ///
    /// ※ 30건 이상의 목록 조회가 필요한 경우, 대안으로 종목조건검색 API를 이용해서 원하는 종목 100개까지 검색할 수 있는 기능을 제공하고 있습니다.
    /// 종목조건검색 API는 HTS(efriend Plus) [0110] 조건검색에서 등록 및 서버저장한 나의 조건 목록을 확인할 수 있는 API로,
    /// 자세한 사용 방법은 공지사항 - [조건검색 필독] 조건검색 API 이용안내 참고 부탁드립니다.
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
    ///
    /// - TR_ID: Real=FHPST02340000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/overtime-fluctuation
    ///
    /// [국내주식] 순위분석
    /// 국내주식 시간외등락율순위 [국내주식-138]
    /// ovtm_untp_uplm_issu_cnt
    /// ovtm_untp_ascn_issu_cnt
    /// ovtm_untp_stnr_issu_cnt
    /// ovtm_untp_lslm_issu_cnt
    /// ovtm_untp_down_issu_cnt
    /// ovtm_untp_acml_vol
    /// ovtm_untp_acml_tr_pbmn
    /// ovtm_untp_exch_vol
    /// ovtm_untp_exch_tr_pbmn
    /// ovtm_untp_kosdaq_vol
    /// ovtm_untp_kosdaq_tr_pbmn
    /// mksc_shrn_iscd
    /// hts_kor_isnm
    /// ovtm_untp_prpr
    /// ovtm_untp_prdy_vrss
    /// ovtm_untp_prdy_vrss_sign
    /// ovtm_untp_prdy_ctrt
    /// ovtm_untp_askp1
    /// ovtm_untp_seln_rsqn
    /// ovtm_untp_bidp1
    /// ovtm_untp_shnu_rsqn
    /// ovtm_untp_vol
    /// ovtm_vrss_acml_vol_rlim
    /// stck_prpr
    /// acml_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외등락율순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0234] 시간외 등락률순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
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
    ///
    /// - TR_ID: Real=FHPST02350000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/overtime-volume
    ///
    /// [국내주식] 순위분석
    /// 국내주식 시간외거래량순위 [국내주식-139]
    /// ovtm_untp_exch_vol
    /// ovtm_untp_exch_tr_pbmn
    /// ovtm_untp_kosdaq_vol
    /// ovtm_untp_kosdaq_tr_pbmn
    /// stck_shrn_iscd
    /// hts_kor_isnm
    /// ovtm_untp_prpr
    /// ovtm_untp_prdy_vrss
    /// ovtm_untp_prdy_vrss_sign
    /// ovtm_untp_prdy_ctrt
    /// ovtm_untp_seln_rsqn
    /// ovtm_untp_shnu_rsqn
    /// ovtm_untp_vol
    /// ovtm_vrss_acml_vol_rlim
    /// stck_prpr
    /// acml_vol
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외거래량순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0235] 시간외 거래량순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 최대 30건 확인 가능하며, 다음 조회가 불가합니다.
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
    ///
    /// - TR_ID: Real=HHMCM000100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-stock/v1/ranking/hts-top-view
    ///
    /// [국내주식] 순위분석
    /// HTS조회상위20종목 [국내주식-214]
    /// mrkt_div_cls_code
    /// mksc_shrn_iscd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// HTS조회상위20종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0158] 조회종목상위 화면의 "종목명", "종목코드" 표시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
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
