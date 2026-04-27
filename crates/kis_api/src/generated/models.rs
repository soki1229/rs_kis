#![allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::doc_markdown
)]
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// [접근토큰발급(P)[인증-001]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Oauth2TokenpRequest {
    /// 권한부여 Type (String, 필수)
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// 앱키 (String, 필수)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 앱시크릿키 (String, 필수)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
}

/// [접근토큰폐기(P)[인증-002]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Oauth2RevokepRequest {
    /// 고객 앱Key (String, 필수)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 고객 앱Secret (String, 필수)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
    /// 접근토큰 (String, 필수)
    #[serde(rename = "token")]
    pub token: String,
}

/// [Hashkey] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct HashkeyRequest {
    /// 요청값 (String, 필수)
    #[serde(rename = "JsonBody")]
    pub json_body: String,
}

/// [실시간 (웹소켓) 접속키 발급[실시간-000]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Oauth2ApprovalRequest {
    /// 권한부여타입 (String, 필수)
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// 앱키 (String, 필수)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 시크릿키 (String, 필수)
    #[serde(rename = "secretkey")]
    pub secretkey: String,
}

/// [주식주문(현금)[v1_국내주식-001]] 요청 구조체
/// [국내주식-001 v1] 주식주문(현금)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderCashRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형 (매도주문 시) (String, 선택)
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 조건가격 (String, 선택)
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
    /// 거래소ID구분코드 (String, 선택)
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
}

/// [주식주문(신용)[v1_국내주식-002]] 요청 구조체
/// [국내주식-002 v1] 주식주문(신용)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderCreditRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형 (String, 선택)
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 신용유형 (String, 필수)
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// 대출일자 (String, 필수)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 예약주문여부 (String, 선택)
    #[serde(rename = "RSVN_ORD_YN")]
    pub rsvn_ord_yn: String,
    /// 비상주문여부 (String, 선택)
    #[serde(rename = "EMGC_ORD_YN")]
    pub emgc_ord_yn: String,
    /// 프로그램매매구분 (String, 선택)
    #[serde(rename = "PGTR_DVSN")]
    pub pgtr_dvsn: String,
    /// 운용사지정주문번호 (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 대량거래협상상세번호 (String, 선택)
    #[serde(rename = "LQTY_TR_NGTN_DTL_NO")]
    pub lqty_tr_ngtn_dtl_no: String,
    /// 대량거래협정번호 (String, 선택)
    #[serde(rename = "LQTY_TR_AGMT_NO")]
    pub lqty_tr_agmt_no: String,
    /// 대량거래협상자Id (String, 선택)
    #[serde(rename = "LQTY_TR_NGTN_ID")]
    pub lqty_tr_ngtn_id: String,
    /// LP주문여부 (String, 선택)
    #[serde(rename = "LP_ORD_YN")]
    pub lp_ord_yn: String,
    /// 매체주문번호 (String, 선택)
    #[serde(rename = "MDIA_ODNO")]
    pub mdia_odno: String,
    /// 주문서버구분코드 (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 프로그램호가신고구분코드 (String, 선택)
    #[serde(rename = "PGM_NMPR_STMT_DVSN_CD")]
    pub pgm_nmpr_stmt_dvsn_cd: String,
    /// 반대매매선정사유코드 (String, 선택)
    #[serde(rename = "CVRG_SLCT_RSON_CD")]
    pub cvrg_slct_rson_cd: String,
    /// 반대매매순번 (String, 선택)
    #[serde(rename = "CVRG_SEQ")]
    pub cvrg_seq: String,
    /// 거래소ID구분코드 (String, 선택)
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
    /// 조건가격 (String, 선택)
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
}

/// [주식주문(정정취소)[v1_국내주식-003]] 요청 구조체
/// [국내주식-003 v1] 주식주문(정정취소)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 한국거래소전송주문조직번호 (String, 필수)
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    /// 원주문번호 (String, 필수)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 정정취소구분코드 (String, 필수)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 잔량전부주문여부 (String, 필수)
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 조건가격 (String, 선택)
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
    /// 거래소ID구분코드 (String, 선택)
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
}

/// [주식정정취소가능주문조회[v1_국내주식-004]] 요청 구조체
/// [국내주식-004 v1] 주식정정취소가능주문조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePsblRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 조회구분1 (String, 필수)
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 조회구분2 (String, 필수)
    #[serde(rename = "INQR_DVSN_2")]
    pub inqr_dvsn_2: String,
}

/// [주식일별주문체결조회[v1_국내주식-005]] 요청 구조체
/// [국내주식-005 v1] 주식일별주문체결조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireDailyCcldRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 상품번호 (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문채번지점번호 (String, 필수)
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호 (String, 선택)
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 체결구분 (String, 필수)
    #[serde(rename = "CCLD_DVSN")]
    pub ccld_dvsn: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 조회구분1 (String, 필수)
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 조회구분3 (String, 필수)
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// 거래소ID구분코드 (String, 필수)
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [주식잔고조회[v1_국내주식-006]] 요청 구조체
/// [국내주식-006 v1] 주식잔고조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시간외단일가, 거래소여부 (String, 필수)
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// 오프라인여부 (String, 선택)
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 단가구분 (String, 필수)
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// 펀드결제분포함여부 (String, 필수)
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// 융자금액자동상환여부 (String, 필수)
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// 처리구분 (String, 필수)
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// 연속조회검색조건100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [매수가능조회[v1_국내주식-007]] 요청 구조체
/// [국내주식-007 v1] 매수가능조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePsblOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// CMA평가금액포함여부 (String, 필수)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 해외포함여부 (String, 필수)
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
}

/// [매도가능수량조회 [국내주식-165]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePsblSellRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [신용매수가능조회[v1_국내주식-042]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireCreditPsamountRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 신용유형 (String, 필수)
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// CMA평가금액포함여부 (String, 필수)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 해외포함여부 (String, 필수)
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
}

/// [주식예약주문[v1_국내주식-017]] 요청 구조체
/// [국내주식-017 v1] 주식예약주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderResvRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드(6자리) (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 주문대상잔고구분코드 (String, 필수)
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    /// 대출일자 (String, 선택)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 예약주문종료일자 (String, 선택)
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 대여일자 (String, 선택)
    #[serde(rename = "LDNG_DT")]
    pub ldng_dt: String,
}

/// [주식예약주문정정취소[v1_국내주식-018,019]] 요청 구조체
/// [국내주식-018,019 v1] 주식예약주문정정취소
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderResvRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드(6자리) (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 주문대상잔고구분코드 (String, 필수)
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    /// 대출일자 (String, 선택)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 예약주문종료일자 (String, 선택)
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 연락전화번호 (String, 선택)
    #[serde(rename = "CTAL_TLNO")]
    pub ctal_tlno: String,
    /// 예약주문순번 (String, 필수)
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// 예약주문조직번호 (String, 선택)
    #[serde(rename = "RSVN_ORD_ORGNO")]
    pub rsvn_ord_orgno: String,
    /// 예약주문주문일자 (String, 선택)
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
}

/// [주식예약주문조회[v1_국내주식-020]] 요청 구조체
/// [국내주식-020 v1] 주식예약주문조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderResvCcnlRequest {
    /// 예약주문시작일자 (String, 필수)
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
    /// 예약주문종료일자 (String, 필수)
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 예약주문순번 (String, 필수)
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// 단말매체종류코드 (String, 필수)
    #[serde(rename = "TMNL_MDIA_KIND_CD")]
    pub tmnl_mdia_kind_cd: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 처리구분코드 (String, 필수)
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
    /// 취소여부 (String, 필수)
    #[serde(rename = "CNCL_YN")]
    pub cncl_yn: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [퇴직연금 체결기준잔고[v1_국내주식-032]] 요청 구조체
/// 퇴직연금 체결기준잔고[v1_국내주식-032]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquirePresentBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 사용자구분코드 (String, 필수)
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 처리구분코드 (String, 선택)
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
}

/// [퇴직연금 미체결내역[v1_국내주식-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquireDailyCcldRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 사용자구분코드 (String, 필수)
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분 (String, 필수)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 조회구분3 (String, 필수)
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [퇴직연금 매수가능조회[v1_국내주식-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquirePsblOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 적립금구분코드 (String, 필수)
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// CMA평가금액포함여부 (String, 필수)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
}

/// [퇴직연금 예수금조회[v1_국내주식-035]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquireDepositRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 적립금구분코드 (String, 필수)
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
}

/// [퇴직연금 잔고조회[v1_국내주식-036]] 요청 구조체
/// 퇴직연금 잔고조회[v1_국내주식-036]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquireBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 적립금구분코드 (String, 필수)
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [주식잔고조회_실현손익[v1_국내주식-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireBalanceRlzPlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시간외단일가여부 (String, 필수)
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// 오프라인여부 (String, 필수)
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 단가구분 (String, 필수)
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// 펀드결제포함여부 (String, 필수)
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// 융자금액자동상환여부 (String, 필수)
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// PRCS_DVSN (String, 필수)
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// 비용포함여부 (String, 필수)
    #[serde(rename = "COST_ICLD_YN")]
    pub cost_icld_yn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [투자계좌자산현황조회[v1_국내주식-048]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireAccountBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회구분1 (String, 필수)
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 기준가이전일자적용여부 (String, 필수)
    #[serde(rename = "BSPR_BF_DT_APLY_YN")]
    pub bspr_bf_dt_aply_yn: String,
}

/// [기간별손익일별합산조회[v1_국내주식-052]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePeriodProfitRequest {
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 정렬구분 (String, 필수)
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 잔고구분 (String, 필수)
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [기간별매매손익현황조회[v1_국내주식-060]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePeriodTradeProfitRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 정렬구분 (String, 필수)
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 잔고구분 (String, 필수)
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [주식통합증거금 현황 [국내주식-191]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingIntgrMarginRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// CMA평가금액포함여부 (String, 필수)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 원화외화구분코드 (String, 필수)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 선도환계약외화구분코드 (String, 필수)
    #[serde(rename = "FWEX_CTRT_FRCR_DVSN_CD")]
    pub fwex_ctrt_frcr_dvsn_cd: String,
}

/// [기간별계좌권리현황조회 [국내주식-211]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPeriodRightsRequest {
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 고객실명확인번호25 (String, 필수)
    #[serde(rename = "CUST_RNCNO25")]
    pub cust_rncno25: String,
    /// 홈넷ID (String, 필수)
    #[serde(rename = "HMID")]
    pub hmid: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 권리유형코드 (String, 필수)
    #[serde(rename = "RGHT_TYPE_CD")]
    pub rght_type_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [주식현재가 시세[v1_국내주식-008]] 요청 구조체
/// [국내주식-008 v1] 주식현재가 시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquirePriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 시세2[v1_국내주식-054]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquirePrice2Request {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 체결[v1_국내주식-009]] 요청 구조체
/// [국내주식-009 v1] 주식현재가 체결
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireCcnlRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 일자별[v1_국내주식-010]] 요청 구조체
/// [국내주식-010 v1] 주식현재가 일자별
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyPriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기간 분류 코드 (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 수정주가 원주가 가격 (String, 필수)
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
}

/// [주식현재가 호가/예상체결[v1_국내주식-011]] 요청 구조체
/// [국내주식-011 v1] 주식현재가 호가 예상체결
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireAskingPriceExpCcnRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 투자자[v1_국내주식-012]] 요청 구조체
/// [국내주식-012 v1] 주식현재가 투자자
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireInvestorRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 회원사[v1_국내주식-013]] 요청 구조체
/// [국내주식-013 v1] 주식현재가 회원사
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireMemberRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내주식기간별시세(일/주/월/년)[v1_국내주식-016]] 요청 구조체
/// [국내주식-016 v1] 국내주식기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyItemchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜 1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 날짜 2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 기간분류코드 (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 수정주가 원주가 가격 여부 (String, 필수)
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
}

/// [주식당일분봉조회[v1_국내주식-022]] 요청 구조체
/// 주식당일분봉조회[v1_국내주식-022]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeItemchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 과거 데이터 포함 여부  (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// 기타 구분 코드 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [주식일별분봉조회 [국내주식-213]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeDailychartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 과거 데이터 포함 여부  (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// 허봉 포함 여부 (String, 선택)
    #[serde(rename = "FID_FAKE_TICK_INCU_YN")]
    pub fid_fake_tick_incu_yn: String,
}

/// [주식현재가 당일시간대별체결[v1_국내주식-023]] 요청 구조체
/// 주식현재가 당일시간대별체결[v1_국내주식-023]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeItemconclusionRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [주식현재가 시간외일자별주가[v1_국내주식-026]] 요청 구조체
/// 주식현재가 시간외일자별주가[v1_국내주식-026]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyOvertimepriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 시간외시간별체결[v1_국내주식-025]] 요청 구조체
/// 주식현재가 시간외시간별체결[v1_국내주식-025]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeOvertimeconclusionRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간 구분 코드 (String, 필수)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
}

/// [국내주식 시간외현재가[국내주식-076]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireOvertimePriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내주식 시간외호가[국내주식-077]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireOvertimeAskingPriceRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 장마감 예상체결가[국내주식-120]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpClosingPriceRequest {
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 소속 구분 코드 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ETF/ETN 현재가[v1_국내주식-068]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EtfetnV1QuotationsInquirePriceRequest {
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [ETF 구성종목시세[국내주식-073]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EtfetnV1QuotationsInquireComponentStockPriceRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
}

/// [NAV 비교추이(종목)[v1_국내주식-069]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EtfetnV1QuotationsNavComparisonTrendRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [NAV 비교추이(일)[v1_국내주식-071]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EtfetnV1QuotationsNavComparisonDailyTrendRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2 (String, 필수)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
}

/// [NAV 비교추이(분)[v1_국내주식-070]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EtfetnV1QuotationsNavComparisonTimeTrendRequest {
    /// FID 시간 구분 코드 (String, 필수)
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [ELW 현재가 시세[v1_국내주식-014]] 요청 구조체
/// ELW 현재가 시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireElwPriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 신규상장종목 [국내주식-181]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsNewlyListedRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 입력종목코드2 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 결재방법 (String, 필수)
    #[serde(rename = "FID_BLNC_CLS_CODE")]
    pub fid_blnc_cls_code: String,
}

/// [ELW 민감도 순위[국내주식-170]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1RankingSensitivityRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 콜풋구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 잔존일수(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 조회기준일 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 결재방법 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 기초자산별 종목시세 [국내주식-186]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsUdrlAssetPriceRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 시장구분코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 거래량수 (String, 필수)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상제외구분코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 입력거래량1 (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 입력거래량2 (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 입력잔존일수1 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 입력잔존일수2 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_2")]
    pub fid_input_rmnn_dynu_2: String,
    /// 옵션 (String, 필수)
    #[serde(rename = "FID_OPTION")]
    pub fid_option: String,
    /// 입력옵션1 (String, 필수)
    #[serde(rename = "FID_INPUT_OPTION_1")]
    pub fid_input_option_1: String,
    /// 입력옵션2 (String, 필수)
    #[serde(rename = "FID_INPUT_OPTION_2")]
    pub fid_input_option_2: String,
}

/// [ELW 종목검색 [국내주식-166]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsCondSearchRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력수1 (String, 필수)
    #[serde(rename = "FID_INPUT_CNT_1")]
    pub fid_input_cnt_1: String,
    /// 순위정렬구분코드2 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_2")]
    pub fid_rank_sort_cls_code_2: String,
    /// 입력수2 (String, 필수)
    #[serde(rename = "FID_INPUT_CNT_2")]
    pub fid_input_cnt_2: String,
    /// 순위정렬구분코드3 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_3")]
    pub fid_rank_sort_cls_code_3: String,
    /// 입력수3 (String, 필수)
    #[serde(rename = "FID_INPUT_CNT_3")]
    pub fid_input_cnt_3: String,
    /// 대상구분코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 시장구분코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 입력종목코드2 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 기타구분코드 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 입력잔존일수1 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 입력잔존일수2 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_2")]
    pub fid_input_rmnn_dynu_2: String,
    /// 현재가수1 (String, 필수)
    #[serde(rename = "FID_PRPR_CNT1")]
    pub fid_prpr_cnt1: String,
    /// 현재가수2 (String, 필수)
    #[serde(rename = "FID_PRPR_CNT2")]
    pub fid_prpr_cnt2: String,
    /// 등락비율1 (String, 필수)
    #[serde(rename = "FID_RSFL_RATE1")]
    pub fid_rsfl_rate1: String,
    /// 등락비율2 (String, 필수)
    #[serde(rename = "FID_RSFL_RATE2")]
    pub fid_rsfl_rate2: String,
    /// 거래량1 (String, 필수)
    #[serde(rename = "FID_VOL1")]
    pub fid_vol1: String,
    /// 거래량2 (String, 필수)
    #[serde(rename = "FID_VOL2")]
    pub fid_vol2: String,
    /// 적용범위가격1 (String, 필수)
    #[serde(rename = "FID_APLY_RANG_PRC_1")]
    pub fid_aply_rang_prc_1: String,
    /// 적용범위가격2 (String, 필수)
    #[serde(rename = "FID_APLY_RANG_PRC_2")]
    pub fid_aply_rang_prc_2: String,
    /// 레버리지값1 (String, 필수)
    #[serde(rename = "FID_LVRG_VAL1")]
    pub fid_lvrg_val1: String,
    /// 레버리지값2 (String, 필수)
    #[serde(rename = "FID_LVRG_VAL2")]
    pub fid_lvrg_val2: String,
    /// 거래량3 (String, 필수)
    #[serde(rename = "FID_VOL3")]
    pub fid_vol3: String,
    /// 거래량4 (String, 필수)
    #[serde(rename = "FID_VOL4")]
    pub fid_vol4: String,
    /// 내재변동성1 (String, 필수)
    #[serde(rename = "FID_INTS_VLTL1")]
    pub fid_ints_vltl1: String,
    /// 내재변동성2 (String, 필수)
    #[serde(rename = "FID_INTS_VLTL2")]
    pub fid_ints_vltl2: String,
    /// 프리미엄값1 (String, 필수)
    #[serde(rename = "FID_PRMM_VAL1")]
    pub fid_prmm_val1: String,
    /// 프리미엄값2 (String, 필수)
    #[serde(rename = "FID_PRMM_VAL2")]
    pub fid_prmm_val2: String,
    /// 기어링1 (String, 필수)
    #[serde(rename = "FID_GEAR1")]
    pub fid_gear1: String,
    /// 기어링2 (String, 필수)
    #[serde(rename = "FID_GEAR2")]
    pub fid_gear2: String,
    /// 손익분기비율1 (String, 필수)
    #[serde(rename = "FID_PRLS_QRYR_RATE1")]
    pub fid_prls_qryr_rate1: String,
    /// 손익분기비율2 (String, 필수)
    #[serde(rename = "FID_PRLS_QRYR_RATE2")]
    pub fid_prls_qryr_rate2: String,
    /// 델타1 (String, 필수)
    #[serde(rename = "FID_DELTA1")]
    pub fid_delta1: String,
    /// 델타2 (String, 필수)
    #[serde(rename = "FID_DELTA2")]
    pub fid_delta2: String,
    /// 행사가1 (String, 필수)
    #[serde(rename = "FID_ACPR1")]
    pub fid_acpr1: String,
    /// 행사가2 (String, 필수)
    #[serde(rename = "FID_ACPR2")]
    pub fid_acpr2: String,
    /// 주식전환비율1 (String, 필수)
    #[serde(rename = "FID_STCK_CNVR_RATE1")]
    pub fid_stck_cnvr_rate1: String,
    /// 주식전환비율2 (String, 필수)
    #[serde(rename = "FID_STCK_CNVR_RATE2")]
    pub fid_stck_cnvr_rate2: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 패리티1 (String, 필수)
    #[serde(rename = "FID_PRIT1")]
    pub fid_prit1: String,
    /// 패리티2 (String, 필수)
    #[serde(rename = "FID_PRIT2")]
    pub fid_prit2: String,
    /// 자본지지점1 (String, 필수)
    #[serde(rename = "FID_CFP1")]
    pub fid_cfp1: String,
    /// 자본지지점2 (String, 필수)
    #[serde(rename = "FID_CFP2")]
    pub fid_cfp2: String,
    /// 지수가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_NMIX_PRICE_1")]
    pub fid_input_nmix_price_1: String,
    /// 지수가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_NMIX_PRICE_2")]
    pub fid_input_nmix_price_2: String,
    /// E기어링값1 (String, 필수)
    #[serde(rename = "FID_EGEA_VAL1")]
    pub fid_egea_val1: String,
    /// E기어링값2 (String, 필수)
    #[serde(rename = "FID_EGEA_VAL2")]
    pub fid_egea_val2: String,
    /// 배당수익율 (String, 필수)
    #[serde(rename = "FID_INPUT_DVDN_ERT")]
    pub fid_input_dvdn_ert: String,
    /// 역사적변동성 (String, 필수)
    #[serde(rename = "FID_INPUT_HIST_VLTL")]
    pub fid_input_hist_vltl: String,
    /// 세타1 (String, 필수)
    #[serde(rename = "FID_THETA1")]
    pub fid_theta1: String,
    /// 세타2 (String, 필수)
    #[serde(rename = "FID_THETA2")]
    pub fid_theta2: String,
}

/// [ELW 당일급변종목[국내주식-171]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1RankingQuickChangeRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장구분코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 가격(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 시간구분코드 (String, 필수)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 입력 일 또는 분 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 기준시간(분 선택 시) (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_2")]
    pub fid_input_hour_2: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 결재방법 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 기초자산 목록조회 [국내주식-185]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsUdrlAssetListRequest {
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 비교대상종목조회 [국내주식-183]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsCompareStocksRequest {
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW LP매매추이 [국내주식-182]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsLpTradeTrendRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 투자지표추이(체결) [국내주식-172]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsIndicatorTrendCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 투자지표추이(분별) [국내주식-174]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsIndicatorTrendMinuteRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간구분코드 (String, 필수)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [ELW 투자지표추이(일별) [국내주식-173]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsIndicatorTrendDailyRequest {
    /// 시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종콕코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(틱) [국내주식-180]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsVolatilityTrendTickRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성추이(체결) [국내주식-177]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsVolatilityTrendCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(일별) [국내주식-178]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsVolatilityTrendDailyRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 민감도 추이(체결) [국내주식-175]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsSensitivityTrendCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(분별) [국내주식-179]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsVolatilityTrendMinuteRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간구분코드 (String, 필수)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [ELW 민감도 추이(일별) [국내주식-176]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsSensitivityTrendDailyRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 만기예정/만기종목 [국내주식-184]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1QuotationsExpirationStocksRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 기타구분코드 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행회사코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 결제방법 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 입력옵션1 (String, 필수)
    #[serde(rename = "FID_INPUT_OPTION_1")]
    pub fid_input_option_1: String,
}

/// [ELW 지표순위[국내주식-169]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1RankingIndicatorRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 콜풋구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 결재방법 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 상승률순위[국내주식-167]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1RankingUpdownRateRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 상승율/하락율 구분 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 상승율/하락율 구분 (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [ELW 거래량순위[국내주식-168]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ElwV1RankingVolumeRankRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력잔존일수 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 콜풋구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하) (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 조회기준일 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 소속구분코드 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// LP발행사 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 만기일-최종거래일조회 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [국내업종 현재지수[v1_국내주식-063]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexPriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내업종 일자별지수[v1_국내주식-065]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexDailyPriceRequest {
    /// FID 기간 분류 코드 (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내업종 시간별지수(초)[국내주식-064]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexTickpriceRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내업종 시간별지수(분)[국내주식-119]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexTimepriceRequest {
    /// ?입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [업종 분봉조회[v1_국내주식-045]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeIndexchartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 기타 구분 코드 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// FID 과거 데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]] 요청 구조체
/// [국내주식-021 v1] 업종기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyIndexchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 업종 상세코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회 시작일자 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 조회 종료일자 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// ' 기간분류코드' (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [국내업종 구분별전체시세[v1_국내주식-066]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexCategoryPriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// FID 소속 구분 코드 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [국내주식 예상체결지수 추이[국내주식-121]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpIndexTrendRequest {
    /// 장운영 구분 코드 (String, 필수)
    #[serde(rename = "FID_MKOP_CLS_CODE")]
    pub fid_mkop_cls_code: String,
    /// 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 예상체결 전체지수[국내주식-122]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpTotalIndexRequest {
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "fid_mrkt_cls_code")]
    pub fid_mrkt_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 장운영 구분 코드 (String, 필수)
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
}

/// [변동성완화장치(VI) 현황 [v1_국내주식-055]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireViStatusRequest {
    /// FID 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// FID 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// FID 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 대상 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [금리 종합(국내채권/금리) [국내주식-155]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCompInterestRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE1")]
    pub fid_div_cls_code1: String,
}

/// [종합 시황/공시(제목) [국내주식-141]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsNewsTitleRequest {
    /// 뉴스 제공 업체 코드 (String, 필수)
    #[serde(rename = "FID_NEWS_OFER_ENTP_CODE")]
    pub fid_news_ofer_entp_code: String,
    /// 조건 시장 구분 코드  (String, 필수)
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 제목 내용 (String, 필수)
    #[serde(rename = "FID_TITL_CNTT")]
    pub fid_titl_cntt: String,
    /// 입력 날짜 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 시간 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 일련번호 (String, 필수)
    #[serde(rename = "FID_INPUT_SRNO")]
    pub fid_input_srno: String,
}

/// [국내휴장일조회[국내주식-040]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsChkHolidayRequest {
    /// 기준일자 (String, 필수)
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 연속조회키 (String, 필수)
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// 연속조회검색조건 (String, 필수)
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
}

/// [상품기본조회[v1_국내주식-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsSearchInfoRequest {
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [주식기본조회[v1_국내주식-067]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsSearchStockInfoRequest {
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [국내주식 대차대조표[v1_국내주식-078]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceBalanceSheetRequest {
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 손익계산서[v1_국내주식-079]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceIncomeStatementRequest {
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 재무비율[v1_국내주식-080]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceFinancialRatioRequest {
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 수익성비율[v1_국내주식-081]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceProfitRatioRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 기타주요비율[v1_국내주식-082]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceOtherMajorRatiosRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 안정성비율[v1_국내주식-083]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceStabilityRatioRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 성장성비율[v1_국내주식-085]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceGrowthRatioRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 당사 신용가능종목[국내주식-111]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCreditByCompanyRequest {
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 선택 여부 (String, 필수)
    #[serde(rename = "fid_slct_yn")]
    pub fid_slct_yn: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [예탁원정보(배당일정)[국내주식-145]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoDividendRequest {
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 고배당여부 (String, 필수)
    #[serde(rename = "HIGH_GB")]
    pub high_gb: String,
}

/// [예탁원정보(주식매수청구일정)[국내주식-146]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoPurreqRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(합병/분할일정)[국내주식-147]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoMergerSplitRequest {
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(액면교체일정)[국내주식-148]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoRevSplitRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 시장구분 (String, 필수)
    #[serde(rename = "MARKET_GB")]
    pub market_gb: String,
}

/// [예탁원정보(자본감소일정)[국내주식-149]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoCapDcrsRequest {
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(상장정보일정)[국내주식-150]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoListInfoRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(공모주청약일정)[국내주식-151]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoPubOfferRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
}

/// [예탁원정보(실권주일정)[국내주식-152]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoForfeitRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(의무예치일정)[국내주식-153]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoMandDepositRequest {
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(유상증자일정) [국내주식-143]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoPaidinCapinRequest {
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(무상증자일정) [국내주식-144]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoBonusIssueRequest {
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(주주총회일정) [국내주식-154]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoSharehldMeetRequest {
    /// CTS (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [국내주식 종목추정실적 [국내주식-187]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsEstimatePerformRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [당사 대주가능 종목 [국내주식-195]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsLendableByCompanyRequest {
    /// 거래소구분코드 (String, 필수)
    #[serde(rename = "EXCG_DVSN_CD")]
    pub excg_dvsn_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 당사대주가능여부 (String, 필수)
    #[serde(rename = "THCO_STLN_PSBL_YN")]
    pub thco_stln_psbl_yn: String,
    /// 조회구분1 (String, 필수)
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [국내주식 종목투자의견 [국내주식-188]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestOpinionRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [국내주식 증권사별 투자의견 [국내주식-189]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestOpbysecRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [종목조건검색 목록조회[국내주식-038]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsPsearchTitleRequest {
    /// 사용자 HTS ID (String, 필수)
    #[serde(rename = "user_id")]
    pub user_id: String,
}

/// [종목조건검색조회 [국내주식-039]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsPsearchResultRequest {
    /// 사용자 HTS ID (String, 필수)
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 사용자조건 키값 (String, 필수)
    #[serde(rename = "seq")]
    pub seq: String,
}

/// [관심종목 그룹조회 [국내주식-204]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsIntstockGrouplistRequest {
    /// 관심종목구분코드                 (String, 필수)
    #[serde(rename = "TYPE")]
    pub r#type: String,
    /// FID 기타 구분 코드  (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 사용자 ID                 (String, 필수)
    #[serde(rename = "USER_ID")]
    pub user_id: String,
}

/// [관심종목(멀티종목) 시세조회 [국내주식-205]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsIntstockMultpriceRequest {
    /// 조건 시장 분류 코드1 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_1")]
    pub fid_cond_mrkt_div_code_1: String,
    /// 입력 종목코드1 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_1")]
    pub fid_input_iscd_1: String,
    /// 조건 시장 분류 코드2 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_2")]
    pub fid_cond_mrkt_div_code_2: String,
    /// 입력 종목코드2 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 조건 시장 분류 코드3 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_3")]
    pub fid_cond_mrkt_div_code_3: String,
    /// 입력 종목코드3 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_3")]
    pub fid_input_iscd_3: String,
    /// 조건 시장 분류 코드4 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_4")]
    pub fid_cond_mrkt_div_code_4: String,
    /// 입력 종목코드4 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_4")]
    pub fid_input_iscd_4: String,
    /// 조건 시장 분류 코드5 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_5")]
    pub fid_cond_mrkt_div_code_5: String,
    /// 입력 종목코드5 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_5")]
    pub fid_input_iscd_5: String,
    /// 조건 시장 분류 코드6 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_6")]
    pub fid_cond_mrkt_div_code_6: String,
    /// 입력 종목코드6 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_6")]
    pub fid_input_iscd_6: String,
    /// 조건 시장 분류 코드7 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_7")]
    pub fid_cond_mrkt_div_code_7: String,
    /// 입력 종목코드7 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_7")]
    pub fid_input_iscd_7: String,
    /// 조건 시장 분류 코드8 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_8")]
    pub fid_cond_mrkt_div_code_8: String,
    /// 입력 종목코드8 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_8")]
    pub fid_input_iscd_8: String,
    /// 조건 시장 분류 코드9 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_9")]
    pub fid_cond_mrkt_div_code_9: String,
    /// 입력 종목코드9 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_9")]
    pub fid_input_iscd_9: String,
    /// 조건 시장 분류 코드10 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_10")]
    pub fid_cond_mrkt_div_code_10: String,
    /// 입력 종목코드10 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_10")]
    pub fid_input_iscd_10: String,
    /// 조건 시장 분류 코드11 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_11")]
    pub fid_cond_mrkt_div_code_11: String,
    /// 입력 종목코드11 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_11")]
    pub fid_input_iscd_11: String,
    /// 조건 시장 분류 코드12 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_12")]
    pub fid_cond_mrkt_div_code_12: String,
    /// 입력 종목코드12 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_12")]
    pub fid_input_iscd_12: String,
    /// 조건 시장 분류 코드13 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_13")]
    pub fid_cond_mrkt_div_code_13: String,
    /// 입력 종목코드13 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_13")]
    pub fid_input_iscd_13: String,
    /// 조건 시장 분류 코드14 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_14")]
    pub fid_cond_mrkt_div_code_14: String,
    /// 입력 종목코드14 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_14")]
    pub fid_input_iscd_14: String,
    /// 조건 시장 분류 코드15 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_15")]
    pub fid_cond_mrkt_div_code_15: String,
    /// 입력 종목코드15 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_15")]
    pub fid_input_iscd_15: String,
    /// 조건 시장 분류 코드16 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_16")]
    pub fid_cond_mrkt_div_code_16: String,
    /// 입력 종목코드16 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_16")]
    pub fid_input_iscd_16: String,
    /// 조건 시장 분류 코드17 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_17")]
    pub fid_cond_mrkt_div_code_17: String,
    /// 입력 종목코드17 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_17")]
    pub fid_input_iscd_17: String,
    /// 조건 시장 분류 코드18 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_18")]
    pub fid_cond_mrkt_div_code_18: String,
    ///  입력 종목코드18 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_18")]
    pub fid_input_iscd_18: String,
    /// 조건 시장 분류 코드19 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_19")]
    pub fid_cond_mrkt_div_code_19: String,
    /// 입력 종목코드19 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_19")]
    pub fid_input_iscd_19: String,
    /// 조건 시장 분류 코드20 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_20")]
    pub fid_cond_mrkt_div_code_20: String,
    /// 입력 종목코드20 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_20")]
    pub fid_input_iscd_20: String,
    /// 조건 시장 분류 코드21 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_21")]
    pub fid_cond_mrkt_div_code_21: String,
    /// 입력 종목코드21 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_21")]
    pub fid_input_iscd_21: String,
    /// 조건 시장 분류 코드22 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_22")]
    pub fid_cond_mrkt_div_code_22: String,
    /// 입력 종목코드22 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_22")]
    pub fid_input_iscd_22: String,
    /// 조건 시장 분류 코드23 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_23")]
    pub fid_cond_mrkt_div_code_23: String,
    /// 입력 종목코드23 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_23")]
    pub fid_input_iscd_23: String,
    /// 조건 시장 분류 코드24 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_24")]
    pub fid_cond_mrkt_div_code_24: String,
    /// 입력 종목코드24 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_24")]
    pub fid_input_iscd_24: String,
    /// 조건 시장 분류 코드25 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_25")]
    pub fid_cond_mrkt_div_code_25: String,
    /// 입력 종목코드25 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_25")]
    pub fid_input_iscd_25: String,
    /// 조건 시장 분류 코드26 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_26")]
    pub fid_cond_mrkt_div_code_26: String,
    /// 입력 종목코드26 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_26")]
    pub fid_input_iscd_26: String,
    /// 조건 시장 분류 코드27 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_27")]
    pub fid_cond_mrkt_div_code_27: String,
    /// 입력 종목코드27 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_27")]
    pub fid_input_iscd_27: String,
    /// 조건 시장 분류 코드28 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_28")]
    pub fid_cond_mrkt_div_code_28: String,
    /// 입력 종목코드28 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_28")]
    pub fid_input_iscd_28: String,
    /// 조건 시장 분류 코드29 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_29")]
    pub fid_cond_mrkt_div_code_29: String,
    /// 입력 종목코드29 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_29")]
    pub fid_input_iscd_29: String,
    /// 조건 시장 분류 코드30 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_30")]
    pub fid_cond_mrkt_div_code_30: String,
    /// 입력 종목코드30 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_30")]
    pub fid_input_iscd_30: String,
}

/// [관심종목 그룹별 종목조회 [국내주식-203]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsIntstockStocklistByGroupRequest {
    /// 관심종목구분코드                 (String, 필수)
    #[serde(rename = "TYPE")]
    pub r#type: String,
    /// 사용자 ID                 (String, 필수)
    #[serde(rename = "USER_ID")]
    pub user_id: String,
    /// 데이터 순위            (String, 필수)
    #[serde(rename = "DATA_RANK")]
    pub data_rank: String,
    /// 관심 그룹 코드        (String, 필수)
    #[serde(rename = "INTER_GRP_CODE")]
    pub inter_grp_code: String,
    /// 관심 그룹 명       (String, 필수)
    #[serde(rename = "INTER_GRP_NAME")]
    pub inter_grp_name: String,
    /// HTS 한글 종목명      (String, 필수)
    #[serde(rename = "HTS_KOR_ISNM")]
    pub hts_kor_isnm: String,
    /// 체결 구분 코드          (String, 필수)
    #[serde(rename = "CNTG_CLS_CODE")]
    pub cntg_cls_code: String,
    /// 기타 구분 코드  (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [국내기관_외국인 매매종목가집계[국내주식-037]] 요청 구조체
/// 국내기관_외국인 매매종목가집계[국내주식-037]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsForeignInstitutionTotalRequest {
    /// 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 기타 구분  정렬 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [외국계 매매종목 가집계 [국내주식-161]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsFrgnmemTradeEstimateRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 순위정렬구분코드2 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_2")]
    pub fid_rank_sort_cls_code_2: String,
}

/// [종목별 투자자매매동향(일별)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestorTradeByStockDailyRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 수정주가 원주가 가격 (String, 필수)
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
    /// 기타 구분 코드 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [시장별 투자자매매동향(시세)[v1_국내주식-074]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireInvestorTimeByMarketRequest {
    /// 시장구분 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 업종구분 (String, 필수)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
}

/// [시장별 투자자매매동향(일별) [국내주식-075]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireInvestorDailyByMarketRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_1")]
    pub fid_input_iscd_1: String,
    /// 입력 날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 하위 분류코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
}

/// [종목별 외국계 순매수추이 [국내주식-164]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsFrgnmemPchsTrendRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 시장구분코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [회원사 실시간 매매동향(틱) [국내주식-163]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsFrgnmemTradeTrendRequest {
    /// 화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 회원사코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 시장구분코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 거래량 (String, 필수)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
}

/// [주식현재가 회원사 종목매매동향 [국내주식-197]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireMemberDailyRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 회원사코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 구간구분코드 (String, 필수)
    #[serde(rename = "FID_SCTN_CLS_CODE")]
    pub fid_sctn_cls_code: String,
}

/// [종목별 프로그램매매추이(체결)[v1_국내주식-044]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsProgramTradeByStockRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [종목별 프로그램매매추이(일별) [국내주식-113]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsProgramTradeByStockDailyRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [종목별 외인기관 추정가집계[v1_국내주식-046]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestorTrendEstimateRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
}

/// [종목별일별매수매도체결량 [v1_국내주식-056]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyTradeVolumeRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// FID 기간 분류 코드 (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [프로그램매매 종합현황(시간) [국내주식-114]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCompProgramTradeTodayRequest {
    /// 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 구간 구분 코드 (String, 필수)
    #[serde(rename = "FID_SCTN_CLS_CODE")]
    pub fid_sctn_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장 분류코드1 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE1")]
    pub fid_cond_mrkt_div_code1: String,
    /// 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [프로그램매매 종합현황(일별)[국내주식-115]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCompProgramTradeDailyRequest {
    /// 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 검색시작일 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 검색종료일 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [프로그램매매 투자자매매동향(당일) [국내주식-116]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestorProgramTradeTodayRequest {
    /// 거래소 구분 코드 (String, 필수)
    #[serde(rename = "EXCH_DIV_CLS_CODE")]
    pub exch_div_cls_code: String,
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "MRKT_DIV_CLS_CODE")]
    pub mrkt_div_cls_code: String,
}

/// [국내주식 신용잔고 일별추이[국내주식-110]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsDailyCreditBalanceRequest {
    /// 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 결제일자 (String, 필수)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 예상체결가 추이[국내주식-118]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpPriceTrendRequest {
    /// 장운영 구분 코드 (String, 필수)
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 공매도 일별추이[국내주식-134]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsDailyShortSaleRequest {
    /// 입력 날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 시간외예상체결등락률 [국내주식-140]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingOvertimeExpTransFluctRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 입력 거래량 (String, 필수)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
}

/// [국내주식 체결금액별 매매비중 [국내주식-192]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsTradprtByamtRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내 증시자금 종합 [국내주식-193]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsMktfundsRequest {
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [종목별 일별 대차거래추이 [국내주식-135]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsDailyLoanTransRequest {
    /// 조회구분 (String, 필수)
    #[serde(rename = "MRKT_DIV_CLS_CODE")]
    pub mrkt_div_cls_code: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE")]
    pub start_date: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "END_DATE")]
    pub end_date: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [국내주식 상하한가 포착 [국내주식-190]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCaptureUplowpriceRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 상하한가 구분코드 (String, 필수)
    #[serde(rename = "FID_PRC_CLS_CODE")]
    pub fid_prc_cls_code: String,
    /// 분류구분코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 대상구분코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상제외구분코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량수 (String, 필수)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
}

/// [국내주식 매물대/거래비중 [국내주식-196]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsPbarTratioRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [거래량순위[v1_국내주식-047]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsVolumeRankRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 등락률 순위[v1_국내주식-088]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingFluctuationRequest {
    /// 등락 비율2 (String, 필수)
    #[serde(rename = "fid_rsfl_rate2")]
    pub fid_rsfl_rate2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 수1 (String, 필수)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// 가격 구분 코드 (String, 필수)
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 등락 비율1 (String, 필수)
    #[serde(rename = "fid_rsfl_rate1")]
    pub fid_rsfl_rate1: String,
}

/// [국내주식 호가잔량 순위[국내주식-089]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingQuoteBalanceRequest {
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 수익자산지표 순위[v1_국내주식-090]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingProfitAssetIndexRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 옵션1 (String, 필수)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2 (String, 필수)
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시가총액 상위[v1_국내주식-091]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingMarketCapRequest {
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
}

/// [국내주식 재무비율 순위[v1_국내주식-092]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingFinanceRatioRequest {
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 옵션1 (String, 필수)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2 (String, 필수)
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시간외잔량 순위[v1_국내주식-093]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingAfterHourBalanceRequest {
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 우선주/괴리율 상위[v1_국내주식-094]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingPreferDisparateRatioRequest {
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 이격도 순위[v1_국내주식-095]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingDisparityRequest {
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 시간 구분 코드 (String, 필수)
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
}

/// [국내주식 시장가치 순위[v1_국내주식-096]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingMarketValueRequest {
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 옵션1 (String, 필수)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2 (String, 필수)
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 체결강도 상위[v1_국내주식-101]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingVolumePowerRequest {
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
}

/// [국내주식 관심종목등록 상위[v1_국내주식-102]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingTopInterestStockRequest {
    /// 입력 필수값2 (String, 필수)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 업종 코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 입력값 (String, 필수)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
}

/// [국내주식 예상체결 상승/하락상위[v1_국내주식-103]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingExpTransUpdownRequest {
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 적용 범위 가격1 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 거래대금 (String, 필수)
    #[serde(rename = "fid_pbmn")]
    pub fid_pbmn: String,
    /// 소속 구분 코드 (String, 필수)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 장운영 구분 코드 (String, 필수)
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
}

/// [국내주식 당사매매종목 상위[v1_국내주식-104]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingTradedByCompanyRequest {
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 날짜1 (String, 필수)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// 입력 날짜2 (String, 필수)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 적용 범위 거래량 (String, 필수)
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: Decimal,
    /// 적용 범위 가격2 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// 적용 범위 가격1 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
}

/// [국내주식 신고/신저근접종목 상위[v1_국내주식-105]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingNearNewHighlowRequest {
    /// 적용 범위 거래량 (String, 필수)
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: Decimal,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 수1 (String, 필수)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// 입력 수2 (String, 필수)
    #[serde(rename = "fid_input_cnt_2")]
    pub fid_input_cnt_2: String,
    /// 가격 구분 코드 (String, 필수)
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    ///  입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 적용 범위 가격1 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 적용 범위 가격2 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
}

/// [국내주식 배당률 상위[국내주식-106]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingDividendRateRequest {
    /// CTS_AREA (String, 필수)
    #[serde(rename = "CTS_AREA")]
    pub cts_area: String,
    /// KOSPI (String, 필수)
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 업종구분 (String, 필수)
    #[serde(rename = "UPJONG")]
    pub upjong: String,
    /// 종목선택 (String, 필수)
    #[serde(rename = "GB2")]
    pub gb2: String,
    /// 배당구분 (String, 필수)
    #[serde(rename = "GB3")]
    pub gb3: String,
    /// 기준일From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 기준일To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 결산/중간배당 (String, 필수)
    #[serde(rename = "GB4")]
    pub gb4: String,
}

/// [국내주식 대량체결건수 상위[국내주식-107]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingBulkTransNumRequest {
    /// 적용 범위 가격2 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 적용 범위 가격1 (String, 필수)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 입력 종목코드2 (String, 필수)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    ///  거래량 수 (String, 필수)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
}

/// [국내주식 신용잔고 상위[국내주식-109]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingCreditBalanceRequest {
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 증가율기간 (String, 필수)
    #[serde(rename = "FID_OPTION")]
    pub fid_option: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
}

/// [국내주식 공매도 상위종목[국내주식-133]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingShortSaleRequest {
    /// FID 적용 범위 거래량 (String, 필수)
    #[serde(rename = "FID_APLY_RANG_VOL")]
    pub fid_aply_rang_vol: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회구분 (일/월) (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 조회가간(일수 (String, 필수)
    #[serde(rename = "FID_INPUT_CNT_1")]
    pub fid_input_cnt_1: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// FID 대상 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID 적용 범위 가격1 (String, 필수)
    #[serde(rename = "FID_APLY_RANG_PRC_1")]
    pub fid_aply_rang_prc_1: String,
    /// FID 적용 범위 가격2 (String, 필수)
    #[serde(rename = "FID_APLY_RANG_PRC_2")]
    pub fid_aply_rang_prc_2: String,
}

/// [국내주식 시간외등락율순위 [국내주식-138]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingOvertimeFluctuationRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시간외거래량순위 [국내주식-139]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingOvertimeVolumeRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 가격1 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 실시간체결가 (KRX) [실시간-003]] 요청 구조체
/// [실시간-003] 국내주식 실시간체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (KRX) [실시간-004]] 요청 구조체
/// [실시간-004] 국내주식 실시간호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결통보 [실시간-005]] 요청 구조체
/// [실시간-005] 국내주식 실시간체결통보
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stcni0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (KRX) [실시간-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (KRX) [실시간-047]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stmbc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (KRX) [실시간-048]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stpgm0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (KRX) [실시간-049]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stmko0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간호가 (KRX) [실시간-025]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stoaa0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간체결가 (KRX) [실시간-042]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stoup0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값  (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간예상체결 (KRX) [실시간-024]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stoac0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간체결 [실시간-026]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0upcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간예상체결 [실시간-027]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0upanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간프로그램매매 [실시간-028]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0uppgm0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간호가 [실시간-062]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ewasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간체결가 [실시간-061]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ewcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간예상체결 [실시간-063]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ewanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내ETF NAV추이 [실시간-051]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0stnav0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결가 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0uncnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0unasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0unanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0unmbc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0unpgm0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0unmko0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결가 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0nxcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0nxasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0nxanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0nxmbc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0nxpgm0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0nxmko0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [선물옵션 주문[v1_국내선물-001]] 요청 구조체
/// [국내선물-001 v1] 선물옵션주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingOrderRequest {
    /// 주문처리구분코드 (String, 필수)
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 단축상품번호 (String, 필수)
    #[serde(rename = "SHTN_PDNO")]
    pub shtn_pdno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문가격1 (String, 필수)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드 (String, 선택)
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// 한국거래소호가조건코드 (String, 선택)
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// 연락전화번호 (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 선물옵션종목구분코드 (String, 선택)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [선물옵션 정정취소주문[v1_국내선물-002]] 요청 구조체
/// [국내선물-002 v1] 선물옵션정정취소주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingOrderRvsecnclRequest {
    /// 주문처리구분코드 (String, 필수)
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 정정취소구분코드 (String, 필수)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 원주문번호 (String, 필수)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문가격1 (String, 필수)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드 (String, 필수)
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// 한국거래소호가조건코드 (String, 필수)
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// 잔여수량여부 (String, 필수)
    #[serde(rename = "RMN_QTY_YN")]
    pub rmn_qty_yn: String,
    /// 선물옵션종목구분코드 (String, 선택)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [선물옵션 주문체결내역조회[v1_국내선물-003]] 요청 구조체
/// [국내선물-003 v1] 선물옵션주문체결내역조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireCcnlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작주문일자 (String, 필수)
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// 종료주문일자 (String, 필수)
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분 (String, 필수)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 정렬순서 (String, 필수)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 시작주문번호 (String, 필수)
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 시장ID코드 (String, 필수)
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 잔고현황[v1_국내선물-004]] 요청 구조체
/// [국내선물-004 v1] 선물옵션 잔고현황
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금 구분 (String, 필수)
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드 (String, 필수)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 주문가능[v1_국내선물-005]] 요청 구조체
/// [국내선물-005 v1] 선물옵션 주문가능
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquirePsblOrderRequest {
    /// 종합계좌번호 (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문가격1 (String, 선택)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 주문구분코드 (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [(야간)선물옵션 주문체결 내역조회 [국내선물-009]] 요청 구조체
/// (야간)선물옵션 주문체결 내역조회 [국내선물-009]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireNgtCcnlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작주문일자 (String, 필수)
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// 종료주문일자 (String, 필수)
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분 (String, 필수)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 정렬순서 (String, 필수)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 시작주문번호 (String, 필수)
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 시장ID코드 (String, 필수)
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// 선물옵션구분코드 (String, 필수)
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// 화면구분 (String, 필수)
    #[serde(rename = "SCRN_DVSN")]
    pub scrn_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [(야간)선물옵션 잔고현황 [국내선물-010]] 요청 구조체
/// (야간)선물옵션 잔고현황 [국내선물-010]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireNgtBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 계좌비밀번호 (String, 필수)
    #[serde(rename = "ACNT_PWD")]
    pub acnt_pwd: String,
    /// 증거금구분 (String, 필수)
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드 (String, 필수)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [(야간)선물옵션 주문가능 조회 [국내선물-011]] 요청 구조체
/// (야간)선물옵션 주문가능 조회 [국내선물-011]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquirePsblNgtOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문가격1 (String, 필수)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 주문구분코드 (String, 필수)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [(야간)선물옵션 증거금 상세 [국내선물-024]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingNgtMarginDetailRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금 구분코드 (String, 필수)
    #[serde(rename = "MGNA_DVSN_CD")]
    pub mgna_dvsn_cd: String,
}

/// [선물옵션 잔고정산손익내역[v1_국내선물-013]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireBalanceSettlementPlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회일자 (String, 필수)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 총자산현황[v1_국내선물-014]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireDepositRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
}

/// [선물옵션 잔고평가손익내역[v1_국내선물-015]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireBalanceValuationPlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금구분 (String, 필수)
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드 (String, 필수)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 기준일체결내역[v1_국내선물-016]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireCcnlBstimeRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자 (String, 필수)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 선물옵션거래시작시각 (String, 필수)
    #[serde(rename = "FUOP_TR_STRT_TMD")]
    pub fuop_tr_strt_tmd: String,
    /// 선물옵션거래종료시각 (String, 필수)
    #[serde(rename = "FUOP_TR_END_TMD")]
    pub fuop_tr_end_tmd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션기간약정수수료일별[v1_국내선물-017]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireDailyAmountFeeRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일 (String, 필수)
    #[serde(rename = "INQR_STRT_DAY")]
    pub inqr_strt_day: String,
    /// 조회종료일 (String, 필수)
    #[serde(rename = "INQR_END_DAY")]
    pub inqr_end_day: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 증거금률] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsMarginRateRequest {
    /// 기준일자 (String, 필수)
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 기초자산ID (String, 필수)
    #[serde(rename = "BAST_ID")]
    pub bast_id: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 시세[v1_국내선물-006]] 요청 구조체
/// [국내선물-006 v1] 선물옵션 시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquirePriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [선물옵션 시세호가[v1_국내선물-007]] 요청 구조체
/// [국내선물-007 v1] 선물옵션 시세호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquireAskingPriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]] 요청 구조체
/// [국내선물-008 v1] 선물옵션기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquireDailyFuopchartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회 시작일자 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 조회 종료일자  (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 기간분류코드 (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [선물옵션 분봉조회[v1_국내선물-012]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquireTimeFuopchartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 시간 구분 코드 (String, 필수)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// FID 과거 데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// FID 허봉 포함 여부 (String, 필수)
    #[serde(rename = "FID_FAKE_TICK_INCU_YN")]
    pub fid_fake_tick_incu_yn: String,
    /// FID 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [국내옵션전광판_옵션월물리스트[국내선물-020]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardOptionListRequest {
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [국내선물 기초자산 시세[국내선물-021]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardTopRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE1")]
    pub fid_cond_mrkt_div_code1: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 만기 수 (String, 필수)
    #[serde(rename = "FID_MTRT_CNT")]
    pub fid_mtrt_cnt: String,
    /// 조건 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [국내옵션전광판_콜풋[국내선물-022]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardCallputRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 만기 수 (String, 필수)
    #[serde(rename = "FID_MTRT_CNT")]
    pub fid_mtrt_cnt: String,
    /// 조건 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_MRKT_CLS_CODE1")]
    pub fid_mrkt_cls_code1: String,
}

/// [국내옵션전광판_선물[국내선물-023]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardFuturesRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 구분 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [선물옵션 일중예상체결추이[국내선물-018]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsExpPriceTrendRequest {
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [지수선물 실시간호가[실시간-011]] 요청 구조체
/// [실시간-011] 지수선물 실시간호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ifasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수선물 실시간체결가[실시간-010]] 요청 구조체
/// [실시간-010] 지수선물 실시간체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ifcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수옵션 실시간호가[실시간-015]] 요청 구조체
/// [실시간-015] 지수옵션 실시간호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ioasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수옵션  실시간체결가[실시간-014]] 요청 구조체
/// [실시간-014] 지수옵션 실시간체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0iocnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [선물옵션 실시간체결통보[실시간-012]] 요청 구조체
/// [실시간-012] 지수선물옵션 실시간체결 통보
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0ifcni0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [상품선물 실시간호가[실시간-023]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0cfasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [상품선물 실시간체결가[실시간-022]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0cfcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간호가 [실시간-030]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0zfasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간체결가 [실시간-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0zfcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간예상체결 [실시간-031]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0zfanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간호가 [실시간-045]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0zoasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간체결가 [실시간-044]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0zocnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간예상체결 [실시간-046]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0zoanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션 실시간호가 [실시간-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0euasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션 실시간체결가 [실시간-032]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0eucnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션실시간예상체결 [실시간-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0euanc0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션실시간체결통보 [실시간-067]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0eucni0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간호가 [실시간-065]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0mfasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간종목체결 [실시간-064]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0mfcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간체결통보 [실시간-066]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0mfcni0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 주문[v1_해외주식-001]] 요청 구조체
/// [해외주식-001 v1] 해외주식 주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호 (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호 (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 판매유형 (String, 선택)
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 주문서버구분코드 (String, 필수)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 시작시간 (String, 선택)
    #[serde(rename = "START_TIME")]
    pub start_time: String,
    /// 종료시간 (String, 선택)
    #[serde(rename = "END_TIME")]
    pub end_time: String,
    /// 알고리즘주문시간구분코드 (String, 선택)
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD")]
    pub algo_ord_tmd_dvsn_cd: String,
}

/// [해외주식 정정취소주문[v1_해외주식-003]] 요청 구조체
/// [해외주식-003 v1] 해외주식 정정취소주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호 (String, 필수)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 정정취소구분코드 (String, 필수)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 운용사지정주문번호 (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

/// [해외주식 예약주문접수[v1_해외주식-002]] 요청 구조체
/// [해외주식-002 v1] 해외주식 예약주문접수
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderResvRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 매도매수구분코드 (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 정정취소구분코드 (String, 필수)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// FT주문수량 (String, 필수)
    #[serde(rename = "FT_ORD_QTY")]
    pub ft_ord_qty: String,
    /// FT주문단가3 (String, 필수)
    #[serde(rename = "FT_ORD_UNPR3")]
    pub ft_ord_unpr3: String,
    /// 주문서버구분코드 (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 예약주문접수일자 (String, 선택)
    #[serde(rename = "RSVN_ORD_RCIT_DT")]
    pub rsvn_ord_rcit_dt: String,
    /// 주문구분 (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 해외예약주문번호 (String, 선택)
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
    /// 알고리즘주문시간구분코드 (String, 선택)
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD")]
    pub algo_ord_tmd_dvsn_cd: String,
}

/// [해외주식 예약주문접수취소[v1_해외주식-004]] 요청 구조체
/// [해외주식-004 v1] 해외주식 예약주문접수취소
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderResvCcnlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외주문접수일자 (String, 필수)
    #[serde(rename = "RSYN_ORD_RCIT_DT")]
    pub rsyn_ord_rcit_dt: String,
    /// 해외예약주문번호 (String, 필수)
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
}

/// [해외주식 매수가능금액조회[v1_해외주식-014]] 요청 구조체
/// [v1_해외주식-014]해외주식 매수가능금액조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePsamountRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 해외주문단가 (String, 필수)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "ITEM_CD")]
    pub item_cd: String,
}

/// [해외주식 미체결내역[v1_해외주식-005]] 요청 구조체
/// [해외주식-005 v1] 해외주식 미체결내역
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireNccsRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 정렬순서 (String, 필수)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 잔고[v1_해외주식-006]] 요청 구조체
/// [해외주식-006 v1] 해외주식 잔고
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 거래통화코드 (String, 필수)
    #[serde(rename = "TR_CRCY_CD")]
    pub tr_crcy_cd: String,
    /// 연속조회검색조건200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 주문체결내역[v1_해외주식-007]] 요청 구조체
/// [해외주식-007 v1] 해외주식 주문체결내역
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireCcnlRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문시작일자 (String, 필수)
    #[serde(rename = "ORD_STRT_DT")]
    pub ord_strt_dt: String,
    /// 주문종료일자 (String, 필수)
    #[serde(rename = "ORD_END_DT")]
    pub ord_end_dt: String,
    /// 매도매수구분 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN")]
    pub sll_buy_dvsn: String,
    /// 체결미체결구분 (String, 필수)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 정렬순서 (String, 필수)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 주문일자 (String, 필수)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문채번지점번호 (String, 필수)
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호 (String, 필수)
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 체결기준현재잔고[v1_해외주식-008]] 요청 구조체
/// [해외주식-008 v1] 해외주식 체결기준현재잔고
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePresentBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 원화외화구분코드 (String, 필수)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 국가코드 (String, 필수)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 거래시장코드 (String, 필수)
    #[serde(rename = "TR_MKET_CD")]
    pub tr_mket_cd: String,
    /// 조회구분코드 (String, 필수)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
}

/// [해외주식 예약주문조회[v1_해외주식-013]] 요청 구조체
/// [v1_해외주식-013]해외주식 예약주문조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderResvListRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 조회구분코드 (String, 필수)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 결제기준잔고 [해외주식-064]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePaymtStdrBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 기준일자 (String, 필수)
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 원화외화구분코드 (String, 필수)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 조회구분코드 (String, 필수)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
}

/// [해외주식 일별거래내역 [해외주식-063]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePeriodTransRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 등록시작일자 (String, 필수)
    #[serde(rename = "ERLM_STRT_DT")]
    pub erlm_strt_dt: String,
    /// 등록종료일자 (String, 필수)
    #[serde(rename = "ERLM_END_DT")]
    pub erlm_end_dt: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 대출구분코드 (String, 필수)
    #[serde(rename = "LOAN_DVSN_CD")]
    pub loan_dvsn_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외주식 기간손익[v1_해외주식-032]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePeriodProfitRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 국가코드 (String, 필수)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 통화코드 (String, 필수)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 원화외화구분코드 (String, 필수)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외증거금 통화별조회 [해외주식-035]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingForeignMarginRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드  (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
}

/// [해외주식 미국주간주문[v1_해외주식-026]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingDaytimeOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호 (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호 (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
}

/// [해외주식 미국주간정정취소[v1_해외주식-027]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingDaytimeOrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호 (String, 필수)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 정정취소구분코드 (String, 필수)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호 (String, 필수)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호 (String, 필수)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

/// [해외주식 지정가주문번호조회  [해외주식-071]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingAlgoOrdnoRequest {
    /// 거래일자 (String, 필수)
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// 계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 연속조회키200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회조건200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 지정가체결내역조회 [해외주식-070]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireAlgoCcnlRequest {
    /// 계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자 (String, 필수)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문채번지점번호 (String, 선택)
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호 (String, 필수)
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 집계포함여부 (String, 선택)
    #[serde(rename = "TTLZ_ICLD_YN")]
    pub ttlz_icld_yn: String,
    /// 연속조회키200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회조건200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 현재가상세[v1_해외주식-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsPriceDetailRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소명 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 현재가 호가 [해외주식-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireAskingPriceRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 현재체결가[v1_해외주식-009]] 요청 구조체
/// [해외주식-009 v1] 해외주식 현재체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsPriceRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 체결추이[해외주식-037]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireCcnlRequest {
    /// 거래소명 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 당일전일구분 (String, 필수)
    #[serde(rename = "TDAY")]
    pub tday: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식분봉조회[v1_해외주식-030]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireTimeItemchartpriceRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 분갭 (String, 필수)
    #[serde(rename = "NMIN")]
    pub nmin: String,
    /// 전일포함여부 (String, 필수)
    #[serde(rename = "PINC")]
    pub pinc: String,
    /// 다음여부 (String, 필수)
    #[serde(rename = "NEXT")]
    pub next: String,
    /// 요청갯수 (String, 필수)
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// 미체결채움구분 (String, 필수)
    #[serde(rename = "FILL")]
    pub fill: String,
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외지수분봉조회[v1_해외주식-031]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireTimeIndexchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간 구분 코드 (String, 필수)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거 데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [해외주식 기간별시세[v1_해외주식-010]] 요청 구조체
/// [해외주식-010 v1] 해외주식 기간별시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsDailypriceRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 일/주/월구분 (String, 필수)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// 조회기준일자 (String, 필수)
    #[serde(rename = "BYMD")]
    pub bymd: String,
    /// 수정주가반영여부 (String, 필수)
    #[serde(rename = "MODP")]
    pub modp: String,
    /// NEXT KEY BUFF (String, 선택)
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]] 요청 구조체
/// [v1_해외주식-012] 해외지수/환율기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireDailyChartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// FID 기간 분류 코드 (String, 필수)
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [해외주식조건검색[v1_해외주식-015]] 요청 구조체
/// 해외주식조건검색[v1_해외주식-015]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireSearchRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 현재가선택조건 (String, 선택)
    #[serde(rename = "CO_YN_PRICECUR")]
    pub co_yn_pricecur: String,
    /// 현재가시작범위가 (String, 선택)
    #[serde(rename = "CO_ST_PRICECUR")]
    pub co_st_pricecur: String,
    /// 현재가끝범위가 (String, 선택)
    #[serde(rename = "CO_EN_PRICECUR")]
    pub co_en_pricecur: String,
    /// 등락율선택조건 (String, 선택)
    #[serde(rename = "CO_YN_RATE")]
    pub co_yn_rate: String,
    /// 등락율시작율 (String, 선택)
    #[serde(rename = "CO_ST_RATE")]
    pub co_st_rate: String,
    /// 등락율끝율 (String, 선택)
    #[serde(rename = "CO_EN_RATE")]
    pub co_en_rate: String,
    /// 시가총액선택조건 (String, 선택)
    #[serde(rename = "CO_YN_VALX")]
    pub co_yn_valx: String,
    /// 시가총액시작액 (String, 선택)
    #[serde(rename = "CO_ST_VALX")]
    pub co_st_valx: String,
    /// 시가총액끝액 (String, 선택)
    #[serde(rename = "CO_EN_VALX")]
    pub co_en_valx: String,
    /// 발행주식수선택조건 (String, 선택)
    #[serde(rename = "CO_YN_SHAR")]
    pub co_yn_shar: String,
    /// 발행주식시작수 (String, 선택)
    #[serde(rename = "CO_ST_SHAR")]
    pub co_st_shar: String,
    /// 발행주식끝수 (String, 선택)
    #[serde(rename = "CO_EN_SHAR")]
    pub co_en_shar: String,
    /// 거래량선택조건 (String, 선택)
    #[serde(rename = "CO_YN_VOLUME")]
    pub co_yn_volume: String,
    /// 거래량시작량 (String, 선택)
    #[serde(rename = "CO_ST_VOLUME")]
    pub co_st_volume: String,
    /// 거래량끝량 (String, 선택)
    #[serde(rename = "CO_EN_VOLUME")]
    pub co_en_volume: String,
    /// 거래대금선택조건 (String, 선택)
    #[serde(rename = "CO_YN_AMT")]
    pub co_yn_amt: String,
    /// 거래대금시작금 (String, 선택)
    #[serde(rename = "CO_ST_AMT")]
    pub co_st_amt: String,
    /// 거래대금끝금 (String, 선택)
    #[serde(rename = "CO_EN_AMT")]
    pub co_en_amt: String,
    /// EPS선택조건 (String, 선택)
    #[serde(rename = "CO_YN_EPS")]
    pub co_yn_eps: String,
    /// EPS시작 (String, 선택)
    #[serde(rename = "CO_ST_EPS")]
    pub co_st_eps: String,
    /// EPS끝 (String, 선택)
    #[serde(rename = "CO_EN_EPS")]
    pub co_en_eps: String,
    /// PER선택조건 (String, 선택)
    #[serde(rename = "CO_YN_PER")]
    pub co_yn_per: String,
    /// PER시작 (String, 선택)
    #[serde(rename = "CO_ST_PER")]
    pub co_st_per: String,
    /// PER끝 (String, 선택)
    #[serde(rename = "CO_EN_PER")]
    pub co_en_per: String,
    /// NEXT KEY BUFF (String, 선택)
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외결제일자조회[해외주식-017]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1QuotationsCountriesHolidayRequest {
    /// 기준일자 (String, 필수)
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// 연속조회키 (String, 필수)
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// 연속조회검색조건 (String, 필수)
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
}

/// [해외주식 상품기본정보[v1_해외주식-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsSearchInfoRequest {
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [해외주식 업종별시세[해외주식-048]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsIndustryThemeRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 업종코드 (String, 필수)
    #[serde(rename = "ICOD")]
    pub icod: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 업종별코드조회[해외주식-049]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsIndustryPriceRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
}

/// [해외주식 복수종목 시세조회] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsMultpriceRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 종목요청개수 (String, 필수)
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD_01 ~ 10")]
    pub excd_01_10: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB_01 ~ 10")]
    pub symb_01_10: String,
}

/// [해외주식 가격급등락[해외주식-038]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingPriceFluctRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 급등/급락구분 (String, 필수)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// N분전콤보값 (String, 필수)
    #[serde(rename = "MINX")]
    pub minx: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래량급증[해외주식-039]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingVolumeSurgeRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N분전콤보값 (String, 필수)
    #[serde(rename = "MINX")]
    pub minx: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 매수체결강도상위[해외주식-040]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingVolumePowerRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 상승율/하락율[해외주식-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingUpdownRateRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 상승율/하락율 구분 (String, 필수)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 신고/신저가[해외주식-042]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingNewHighlowRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 신고/신저 구분 (String, 필수)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// 일시돌파/돌파 구분 (String, 필수)
    #[serde(rename = "GUBN2")]
    pub gubn2: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래량순위[해외주식-043]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradeVolRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 현재가 필터범위 1 (String, 필수)
    #[serde(rename = "PRC1")]
    pub prc1: String,
    /// 현재가 필터범위 2 (String, 필수)
    #[serde(rename = "PRC2")]
    pub prc2: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래대금순위[해외주식-044]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradePbmnRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
    /// 현재가 필터범위 1 (String, 필수)
    #[serde(rename = "PRC1")]
    pub prc1: String,
    /// 현재가 필터범위 2 (String, 필수)
    #[serde(rename = "PRC2")]
    pub prc2: String,
}

/// [해외주식 거래증가율순위[해외주식-045]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradeGrowthRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래회전율순위[해외주식-046]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradeTurnoverRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 시가총액순위[해외주식-047]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingMarketCapRequest {
    /// NEXT KEY BUFF (String, 필수)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 거래량조건 (String, 필수)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 기간별권리조회 [해외주식-052]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsPeriodRightsRequest {
    /// 권리유형코드 (String, 필수)
    #[serde(rename = "RGHT_TYPE_CD")]
    pub rght_type_cd: String,
    /// 조회구분코드 (String, 필수)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 연속조회키50 (String, 필수)
    #[serde(rename = "CTX_AREA_NK50")]
    pub ctx_area_nk50: String,
    /// 연속조회검색조건50 (String, 필수)
    #[serde(rename = "CTX_AREA_FK50")]
    pub ctx_area_fk50: String,
}

/// [해외뉴스종합(제목) [해외주식-053]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsNewsTitleRequest {
    /// 뉴스구분 (String, 필수)
    #[serde(rename = "INFO_GB")]
    pub info_gb: String,
    /// 중분류 (String, 필수)
    #[serde(rename = "CLASS_CD")]
    pub class_cd: String,
    /// 국가코드 (String, 필수)
    #[serde(rename = "NATION_CD")]
    pub nation_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCHANGE_CD")]
    pub exchange_cd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 조회일자 (String, 필수)
    #[serde(rename = "DATA_DT")]
    pub data_dt: String,
    /// 조회시간 (String, 필수)
    #[serde(rename = "DATA_TM")]
    pub data_tm: String,
    /// 다음키 (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [해외주식 권리종합 [해외주식-050]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsRightsByIceRequest {
    /// 국가코드 (String, 필수)
    #[serde(rename = "NCOD")]
    pub ncod: String,
    /// 심볼 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 일자 시작일 (String, 필수)
    #[serde(rename = "ST_YMD")]
    pub st_ymd: String,
    /// 일자 종료일 (String, 필수)
    #[serde(rename = "ED_YMD")]
    pub ed_ymd: String,
}

/// [당사 해외주식담보대출 가능 종목 [해외주식-051]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsColableByCompanyRequest {
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 국가코드 (String, 필수)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 조회순서구분 (String, 필수)
    #[serde(rename = "INQR_SQN_DVSN")]
    pub inqr_sqn_dvsn: String,
    /// 비율구분코드 (String, 필수)
    #[serde(rename = "RT_DVSN_CD")]
    pub rt_dvsn_cd: String,
    /// 비율 (String, 필수)
    #[serde(rename = "RT")]
    pub rt: String,
    /// 대출가능여부 (String, 필수)
    #[serde(rename = "LOAN_PSBL_YN")]
    pub loan_psbl_yn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외속보(제목) [해외주식-055]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsBrknewsTitleRequest {
    /// 뉴스제공업체코드 (String, 필수)
    #[serde(rename = "FID_NEWS_OFER_ENTP_CODE")]
    pub fid_news_ofer_entp_code: String,
    /// 조건시장구분코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 제목내용 (String, 필수)
    #[serde(rename = "FID_TITL_CNTT")]
    pub fid_titl_cntt: String,
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력시간1 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 순위정렬구분코드 (String, 필수)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력일련번호 (String, 필수)
    #[serde(rename = "FID_INPUT_SRNO")]
    pub fid_input_srno: String,
    /// 조건화면분류코드 (String, 필수)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
}

/// [해외주식 실시간호가[실시간-021]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfsasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// R거래소명종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 지연호가(아시아)[실시간-008]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfsasp1Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// D거래소명종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 실시간지연체결가[실시간-007]] 요청 구조체
/// [실시간-007] 해외주식 실시간지연체결통보
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfscnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// D거래소명종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 실시간체결통보[실시간-009]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0gscni0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 주문 [v1_해외선물-001]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외선물FX상품번호 (String, 필수)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM청산미결제체결일자 (String, 선택)
    #[serde(rename = "FM_LQD_USTL_CCLD_DT")]
    pub fm_lqd_ustl_ccld_dt: String,
    /// FM청산미결제체결번호 (String, 선택)
    #[serde(rename = "FM_LQD_USTL_CCNO")]
    pub fm_lqd_ustl_ccno: String,
    /// 가격구분코드 (String, 필수)
    #[serde(rename = "PRIC_DVSN_CD")]
    pub pric_dvsn_cd: String,
    /// FMLIMIT주문가격 (String, 필수)
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    /// FMSTOP주문가격 (String, 필수)
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    /// FM주문수량 (String, 필수)
    #[serde(rename = "FM_ORD_QTY")]
    pub fm_ord_qty: String,
    /// FM청산LIMIT주문가격 (String, 선택)
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: String,
    /// FM청산STOP주문가격 (String, 선택)
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: String,
    /// 체결조건코드 (String, 필수)
    #[serde(rename = "CCLD_CNDT_CD")]
    pub ccld_cndt_cd: String,
    /// 복합주문구분코드 (String, 필수)
    #[serde(rename = "CPLX_ORD_DVSN_CD")]
    pub cplx_ord_dvsn_cd: String,
    /// 행사예약주문여부 (String, 필수)
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
    /// FM_HEDGE주문화면여부 (String, 필수)
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
}

/// [해외선물옵션 정정취소주문 [v1_해외선물-002, 003]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingOrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 원주문일자 (String, 필수)
    #[serde(rename = "ORGN_ORD_DT")]
    pub orgn_ord_dt: String,
    /// 원주문번호 (String, 필수)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// FMLIMIT주문가격 (String, 선택)
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    /// FMSTOP주문가격 (String, 선택)
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    /// FM청산LIMIT주문가격 (String, 선택)
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: String,
    /// FM청산STOP주문가격 (String, 선택)
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: String,
    /// FM_HEDGE주문화면여부 (String, 필수)
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    /// FM시장가전환여부 (String, 선택)
    #[serde(rename = "FM_MKPR_CVSN_YN")]
    pub fm_mkpr_cvsn_yn: String,
}

/// [해외선물옵션 당일주문내역조회 [v1_해외선물-004]] 요청 구조체
/// 해외선물옵션 당일주문내역조회 [v1_해외선물-004]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquireCcldRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 체결미체결구분 (String, 필수)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 선물옵션구분 (String, 필수)
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]] 요청 구조체
/// 해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquireUnpdRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 선물옵션구분 (String, 필수)
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외선물옵션 주문가능조회 [v1_해외선물-006]] 요청 구조체
/// 해외선물옵션 주문가능조회 [v1_해외선물-006]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquirePsamountRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외선물FX상품번호 (String, 필수)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM주문가격 (String, 필수)
    #[serde(rename = "FM_ORD_PRIC")]
    pub fm_ord_pric: String,
    /// 행사예약주문여부 (String, 필수)
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
}

/// [해외선물옵션 기간계좌손익 일별[해외선물-010]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquirePeriodCcldRequest {
    /// 조회기간FROM일자 (String, 필수)
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// 조회기간TO일자 (String, 필수)
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드 (String, 필수)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 전체환산여부 (String, 필수)
    #[serde(rename = "WHOL_TRSL_YN")]
    pub whol_trsl_yn: String,
    /// 선물옵션구분 (String, 필수)
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 일별 체결내역[해외선물-011]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquireDailyCcldRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작일자 (String, 필수)
    #[serde(rename = "STRT_DT")]
    pub strt_dt: String,
    /// 종료일자 (String, 필수)
    #[serde(rename = "END_DT")]
    pub end_dt: String,
    /// 선물옵션구분코드 (String, 필수)
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// FM상품군코드 (String, 필수)
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// 통화코드 (String, 필수)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// FM종목합산여부 (String, 필수)
    #[serde(rename = "FM_ITEM_FTNG_YN")]
    pub fm_item_ftng_yn: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 예수금현황[해외선물-012]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquireDepositRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드 (String, 필수)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 조회일자 (String, 필수)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
}

/// [해외선물옵션 일별 주문내역[해외선물-013]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquireDailyOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작일자 (String, 필수)
    #[serde(rename = "STRT_DT")]
    pub strt_dt: String,
    /// 종료일자 (String, 필수)
    #[serde(rename = "END_DT")]
    pub end_dt: String,
    /// FM상품군코드 (String, 필수)
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// 체결미체결구분 (String, 필수)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 선물옵션구분 (String, 필수)
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 기간계좌거래내역[해외선물-014]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingInquirePeriodTransRequest {
    /// 조회기간FROM일자 (String, 필수)
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// 조회기간TO일자 (String, 필수)
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 계좌거래유형코드 (String, 필수)
    #[serde(rename = "ACNT_TR_TYPE_CD")]
    pub acnt_tr_type_cd: String,
    /// 통화코드 (String, 필수)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 비밀번호체크여부 (String, 필수)
    #[serde(rename = "PWD_CHK_YN")]
    pub pwd_chk_yn: String,
}

/// [해외선물옵션 증거금상세 [해외선물-032]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1TradingMarginDetailRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드 (String, 필수)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 조회일자 (String, 필수)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
}

/// [해외선물종목현재가 [v1_해외선물-009]] 요청 구조체
/// 해외선물종목현재가 [v1_해외선물-009]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsInquirePriceRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물종목상세 [v1_해외선물-008]] 요청 구조체
/// 해외선물종목상세 [v1_해외선물-008]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsStockDetailRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물 호가 [해외선물-031]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsInquireAskingPriceRequest {
    /// 종목명 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물 분봉조회[해외선물-016]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsInquireTimeFuturechartpriceRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(틱)[해외선물-019]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsTickCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(주간)[해외선물-017]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsWeeklyCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(일간)[해외선물-018]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsDailyCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(월간)[해외선물-020]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsMonthlyCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 상품기본정보 [해외선물-023]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsSearchContractDetailRequest {
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 품목종류 (String, 필수)
    #[serde(rename = "SRS_CD_01")]
    pub srs_cd_01: String,
    /// 품목종류… (String, 필수)
    #[serde(rename = "SRS_CD_02…")]
    pub srs_cd_02: String,
    /// 품목종류 (String, 필수)
    #[serde(rename = "SRS_CD_32")]
    pub srs_cd_32: String,
}

/// [해외선물 미결제추이 [해외선물-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsInvestorUnpdTrendRequest {
    /// 상품 (String, 필수)
    #[serde(rename = "PROD_ISCD")]
    pub prod_iscd: String,
    /// 일자 (String, 필수)
    #[serde(rename = "BSOP_DATE")]
    pub bsop_date: String,
    /// 구분 (String, 필수)
    #[serde(rename = "UPMU_GUBUN")]
    pub upmu_gubun: String,
    /// CTS_KEY (String, 필수)
    #[serde(rename = "CTS_KEY")]
    pub cts_key: String,
}

/// [해외옵션종목현재가 [해외선물-035]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptPriceRequest {
    /// 종목명 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션종목상세 [해외선물-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptDetailRequest {
    /// 종목명 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션 호가 [해외선물-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptAskingPriceRequest {
    /// 종목명 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션 분봉조회 [해외선물-040]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsInquireTimeOptchartpriceRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(틱) [해외선물-038]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptTickCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(일간) [해외선물-037]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptDailyCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(주간) [해외선물-036]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptWeeklyCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(월간) [해외선물-039]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsOptMonthlyCcnlRequest {
    /// 종목코드 (String, 필수)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 상품기본정보 [해외선물-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsSearchOptDetailRequest {
    /// 요청개수 (String, 필수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 종목코드1 (String, 필수)
    #[serde(rename = "SRS_CD_01")]
    pub srs_cd_01: String,
    /// 종목코드2 (String, 필수)
    #[serde(rename = "SRS_CD_02...")]
    pub srs_cd_02: String,
    /// 종목코드30 (String, 필수)
    #[serde(rename = "SRS_CD_30")]
    pub srs_cd_30: String,
}

/// [해외선물옵션 장운영시간 [해외선물-030]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasFutureoptionV1QuotationsMarketTimeRequest {
    /// FM상품군코드 (String, 필수)
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// FM클래스코드 (String, 필수)
    #[serde(rename = "FM_CLAS_CD")]
    pub fm_clas_cd: String,
    /// FM거래소코드 (String, 필수)
    #[serde(rename = "FM_EXCG_CD")]
    pub fm_excg_cd: String,
    /// 옵션여부 (String, 필수)
    #[serde(rename = "OPT_YN")]
    pub opt_yn: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외선물옵션 실시간체결가[실시간-017]] 요청 구조체
/// 해외선물옵션체결[실시간-017]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfff020Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간호가[실시간-018]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfff010Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간주문내역통보[실시간-019]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfff1c0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간체결내역통보[실시간-020]] 요청 구조체
/// 해외선물옵션체결내역통보[실시간-020]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutHdfff2c0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [장내채권 매수주문 [국내주식-124]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingBuyRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량2 (String, 필수)
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 채권주문단가 (String, 필수)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 소액시장참여여부 (String, 필수)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// 채권소매시장여부 (String, 필수)
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// 유치자직원번호 (String, 필수)
    #[serde(rename = "IDCR_STFNO")]
    pub idcr_stfno: String,
    /// 운용사지정주문번호 (String, 필수)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호 (String, 필수)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [장내채권 매도주문 [국내주식-123]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingSellRequest {
    ///  종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문구분 (String, 필수)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    ///  상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량2 (String, 필수)
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 분리과세여부 (String, 필수)
    #[serde(rename = "SPRX_YN")]
    pub sprx_yn: String,
    /// 매수일자 (String, 필수)
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// 매수순번 (String, 필수)
    #[serde(rename = "BUY_SEQ")]
    pub buy_seq: String,
    /// 소액시장참여여부 (String, 필수)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// 매도대행사반대매도여부 (String, 필수)
    #[serde(rename = "SLL_AGCO_OPPS_SLL_YN")]
    pub sll_agco_opps_sll_yn: String,
    /// 채권소매시장여부 (String, 필수)
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// 운용사지정주문번호 (String, 필수)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호 (String, 필수)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [장내채권 정정취소주문 [국내주식-125]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingOrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호 (String, 필수)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문수량2 (String, 필수)
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 채권주문단가 (String, 필수)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 잔량전부주문여부 (String, 필수)
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 정정취소구분코드 (String, 필수)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 운용사지정주문번호 (String, 필수)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호 (String, 필수)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [채권정정취소가능주문조회  [국내주식-126]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingInquirePsblRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자 (String, 필수)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문번호 (String, 필수)
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [장내채권 주문체결내역 [국내주식-127]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingInquireDailyCcldRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 정렬순서구분 (String, 필수)
    #[serde(rename = "SORT_SQN_DVSN")]
    pub sort_sqn_dvsn: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 미체결여부 (String, 필수)
    #[serde(rename = "NCCS_YN")]
    pub nccs_yn: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [장내채권 잔고조회  [국내주식-198]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingInquireBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회조건 (String, 필수)
    #[serde(rename = "INQR_CNDT")]
    pub inqr_cndt: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매수일자 (String, 필수)
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [장내채권 매수가능조회 [국내주식-199]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1TradingInquirePsblOrderRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 채권주문단가 (String, 필수)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 소액시장참여여부 (String, 필수)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
}

/// [장내채권현재가(호가) [국내주식-132]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsInquireAskingPriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(시세) [국내주식-200]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsInquirePriceRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(체결) [국내주식-201]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsInquireCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(일별) [국내주식-202]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsInquireDailyPriceRequest {
    /// 조건시장분류코드 (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권 기간별시세(일) [국내주식-159]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsInquireDailyItemchartpriceRequest {
    /// 조건 시장 구분 코드  (String, 필수)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권 평균단가조회 [국내주식-158]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsAvgUnitRequest {
    /// 조회시작일자 (String, 필수)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 검증종류코드 (String, 필수)
    #[serde(rename = "VRFC_KIND_CD")]
    pub vrfc_kind_cd: String,
    /// 연속조회키30 (String, 필수)
    #[serde(rename = "CTX_AREA_NK30")]
    pub ctx_area_nk30: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [장내채권 발행정보[국내주식-156]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsIssueInfoRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [장내채권 기본조회 [국내주식-129]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticBondV1QuotationsSearchBondInfoRequest {
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [일반채권 실시간체결가 [실시간-052]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0bjcnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [일반채권 실시간호가 [실시간-053]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0bjasp0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [채권지수 실시간체결가 [실시간-060]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TryitoutH0bicnt0Request {
    /// 거래ID (String, 필수)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}
