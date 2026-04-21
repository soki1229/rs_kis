#![allow(clippy::doc_lazy_continuation)]
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// [접근토큰발급(P)[인증-001]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOauth2TokenPRequest {
    /// grant_type (String, 선택)
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// appkey (String, 선택)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// appsecret (String, 선택)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// access_token (String, 선택)
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// token_type (String, 선택)
    #[serde(rename = "token_type")]
    pub token_type: String,
    /// expires_in (String, 선택)
    #[serde(rename = "expires_in")]
    pub expires_in: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [접근토큰폐기(P)[인증-002]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOauth2RevokePRequest {
    /// appkey (String, 선택)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// appsecret (String, 선택)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
    /// token (String, 선택)
    #[serde(rename = "token")]
    pub token: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// code (String, 선택)
    #[serde(rename = "code")]
    pub code: String,
    /// message (String, 선택)
    #[serde(rename = "message")]
    pub message: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [Hashkey] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthHashkeyRequest {
    /// ORD_PRCS_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// SHTN_PDNO (String, 선택)
    #[serde(rename = "SHTN_PDNO")]
    pub shtn_pdno: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// UNIT_PRICE (String, 선택)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// NMPR_TYPE_CD (String, 선택)
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// KRX_NMPR_CNDT_CD (String, 선택)
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// FUOP_ITEM_DVSN_CD (String, 선택)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// BODY (String, 선택)
    #[serde(rename = "BODY")]
    pub body: String,
    /// HASH (String, 선택)
    #[serde(rename = "HASH")]
    pub hash: String,
}

/// [실시간 (웹소켓) 접속키 발급[실시간-000]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOauth2ApprovalRequest {
    /// grant_type (String, 선택)
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// appkey (String, 선택)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// secretkey (String, 선택)
    #[serde(rename = "secretkey")]
    pub secretkey: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식주문(현금)[v1_국내주식-001]] 요청 구조체
/// [국내주식-001 v1] 주식주문(현금)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderCashRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식주문(신용)[v1_국내주식-002]] 요청 구조체
/// [국내주식-002 v1] 주식주문(신용)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderCreditRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// CRDT_TYPE (String, 선택)
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// LOAN_DT (String, 선택)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// RSVN_ORD_YN (String, 선택)
    #[serde(rename = "RSVN_ORD_YN")]
    pub rsvn_ord_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// EMGC_ORD_YN (String, 선택)
    #[serde(rename = "EMGC_ORD_YN")]
    pub emgc_ord_yn: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식주문(정정취소)[v1_국내주식-003]] 요청 구조체
/// [국내주식-003 v1] 주식주문(정정취소)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderRvsecnclRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// KRX_FWDG_ORD_ORGNO (String, 선택)
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    /// ORGN_ODNO (String, 선택)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// RVSE_CNCL_DVSN_CD (String, 선택)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// QTY_ALL_ORD_YN (String, 선택)
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식정정취소가능주문조회[v1_국내주식-004]] 요청 구조체
/// [국내주식-004 v1] 주식정정취소가능주문조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePsblRvsecnclRequest {
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// INQR_DVSN_1 (String, 선택)
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// INQR_DVSN_2 (String, 선택)
    #[serde(rename = "INQR_DVSN_2")]
    pub inqr_dvsn_2: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식일별주문체결조회[v1_국내주식-005]] 요청 구조체
/// [국내주식-005 v1] 주식일별주문체결조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireDailyCcldRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// INQR_STRT_DT (String, 선택)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// INQR_END_DT (String, 선택)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// INQR_DVSN (String, 선택)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// CCLD_DVSN (String, 선택)
    #[serde(rename = "CCLD_DVSN")]
    pub ccld_dvsn: String,
    /// ORD_GNO_BRNO (String, 선택)
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// ODNO (String, 선택)
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// INQR_DVSN_3 (String, 선택)
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// INQR_DVSN_1 (String, 선택)
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식잔고조회[v1_국내주식-006]] 요청 구조체
/// [국내주식-006 v1] 주식잔고조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// AFHR_FLPR_YN (String, 선택)
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// OFL_YN (String, 선택)
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// INQR_DVSN (String, 선택)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// UNPR_DVSN (String, 선택)
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// FUND_STTL_ICLD_YN (String, 선택)
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// FNCG_AMT_AUTO_RDPT_YN (String, 선택)
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// PRCS_DVSN (String, 선택)
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [매수가능조회[v1_국내주식-007]] 요청 구조체
/// [국내주식-007 v1] 매수가능조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePsblOrderRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// CMA_EVLU_AMT_ICLD_YN (String, 선택)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// OVRS_ICLD_YN (String, 선택)
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [매도가능수량조회 [국내주식-165]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePsblSellRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [신용매수가능조회[v1_국내주식-042]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireCreditPsamountRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// CRDT_TYPE (String, 선택)
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// CMA_EVLU_AMT_ICLD_YN (String, 선택)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// OVRS_ICLD_YN (String, 선택)
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식예약주문[v1_국내주식-017]] 요청 구조체
/// [국내주식-017 v1] 주식예약주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderResvRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// ORD_OBJT_CBLC_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    /// LOAN_DT (String, 선택)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// RSVN_ORD_END_DT (String, 선택)
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// LDNG_DT (String, 선택)
    #[serde(rename = "LDNG_DT")]
    pub ldng_dt: String,
    /// CUST_LDNG_TR_SEQ (String, 선택)
    #[serde(rename = "CUST_LDNG_TR_SEQ")]
    pub cust_ldng_tr_seq: String,
    /// RSVN_ORD_SEQ (String, 선택)
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// rt_cd (String, 선택)
    #[serde(rename = "rt_cd")]
    pub rt_cd: String,
    /// msg_cd (String, 선택)
    #[serde(rename = "msg_cd")]
    pub msg_cd: String,
    /// msg1 (String, 선택)
    #[serde(rename = "msg1")]
    pub msg1: String,
    /// output (String, 선택)
    #[serde(rename = "output")]
    pub output: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식예약주문정정취소[v1_국내주식-018,019]] 요청 구조체
/// [국내주식-018,019 v1] 주식예약주문정정취소
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderResvRvsecnclRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// _comment (String, 선택)
    #[serde(rename = "_comment")]
    pub _comment: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// ORD_OBJT_CBLC_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    /// LOAN_DT (String, 선택)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// RSVN_ORD_END_DT (String, 선택)
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// RSVN_ORD_SEQ (String, 선택)
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// RSVN_ORD_ORGNO (String, 선택)
    #[serde(rename = "RSVN_ORD_ORGNO")]
    pub rsvn_ord_orgno: String,
    /// RSVN_ORD_ORD_DT (String, 선택)
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
    /// rt_cd (String, 선택)
    #[serde(rename = "rt_cd")]
    pub rt_cd: String,
    /// msg_cd (String, 선택)
    #[serde(rename = "msg_cd")]
    pub msg_cd: String,
    /// msg1 (String, 선택)
    #[serde(rename = "msg1")]
    pub msg1: String,
    /// output (String, 선택)
    #[serde(rename = "output")]
    pub output: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식예약주문조회[v1_국내주식-020]] 요청 구조체
/// [국내주식-020 v1] 주식예약주문조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingOrderResvCcnlRequest {
    /// RSVN_ORD_ORD_DT (String, 선택)
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
    /// RSVN_ORD_END_DT (String, 선택)
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// RSVN_ORD_SEQ (String, 선택)
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// TMNL_MDIA_KIND_CD (String, 선택)
    #[serde(rename = "TMNL_MDIA_KIND_CD")]
    pub tmnl_mdia_kind_cd: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PRCS_DVSN_CD (String, 선택)
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
    /// CNCL_YN (String, 선택)
    #[serde(rename = "CNCL_YN")]
    pub cncl_yn: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
    /// enabled (String, 선택)
    #[serde(rename = "enabled")]
    pub enabled: String,
    /// byAppKey (String, 선택)
    #[serde(rename = "byAppKey")]
    pub by_app_key: String,
    /// refreshTimeUnit (String, 선택)
    #[serde(rename = "refreshTimeUnit")]
    pub refresh_time_unit: String,
    /// requestLimit (String, 선택)
    #[serde(rename = "requestLimit")]
    pub request_limit: String,
}

/// [퇴직연금 체결기준잔고[v1_국내주식-032]] 요청 구조체
/// 퇴직연금 체결기준잔고[v1_국내주식-032]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquirePresentBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// USER_DVSN_CD (String, 선택)
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
    /// enabled (String, 선택)
    #[serde(rename = "enabled")]
    pub enabled: String,
    /// byAppKey (String, 선택)
    #[serde(rename = "byAppKey")]
    pub by_app_key: String,
    /// refreshTimeUnit (String, 선택)
    #[serde(rename = "refreshTimeUnit")]
    pub refresh_time_unit: String,
    /// requestLimit (String, 선택)
    #[serde(rename = "requestLimit")]
    pub request_limit: String,
}

/// [퇴직연금 미체결내역[v1_국내주식-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquireDailyCcldRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// USER_DVSN_CD (String, 선택)
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// CCLD_NCCS_DVSN (String, 선택)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// INQR_DVSN_3 (String, 선택)
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [퇴직연금 매수가능조회[v1_국내주식-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquirePsblOrderRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_UNPR (String, 선택)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: Decimal,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// CMA_EVLU_AMT_ICLD_YN (String, 선택)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// ACCA_DVSN_CD (String, 선택)
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [퇴직연금 예수금조회[v1_국내주식-035]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquireDepositRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// ACCA_DVSN_CD (String, 선택)
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [퇴직연금 잔고조회[v1_국내주식-036]] 요청 구조체
/// 퇴직연금 잔고조회[v1_국내주식-036]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPensionInquireBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// ACCA_DVSN_CD (String, 선택)
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// INQR_DVSN (String, 선택)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식잔고조회_실현손익[v1_국내주식-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireBalanceRlzPlRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// AFHR_FLPR_YN (String, 선택)
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// OFL_YN (String, 선택)
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// INQR_DVSN (String, 선택)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// UNPR_DVSN (String, 선택)
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// FUND_STTL_ICLD_YN (String, 선택)
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// FNCG_AMT_AUTO_RDPT_YN (String, 선택)
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// PRCS_DVSN (String, 선택)
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// COST_ICLD_YN (String, 선택)
    #[serde(rename = "COST_ICLD_YN")]
    pub cost_icld_yn: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [투자계좌자산현황조회[v1_국내주식-048]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquireAccountBalanceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [기간별손익일별합산조회[v1_국내주식-052]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePeriodProfitRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// INQR_STRT_DT (String, 선택)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// INQR_END_DT (String, 선택)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// SORT_DVSN (String, 선택)
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// INQR_DVSN (String, 선택)
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// CBLC_DVSN (String, 선택)
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [기간별매매손익현황조회[v1_국내주식-060]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingInquirePeriodTradeProfitRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식통합증거금 현황 [국내주식-191]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingIntgrMarginRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [기간별계좌권리현황조회 [국내주식-211]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1TradingPeriodRightsRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 시세[v1_국내주식-008]] 요청 구조체
/// [국내주식-008 v1] 주식현재가 시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquirePriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 시세2[v1_국내주식-054]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquirePrice2Request {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 체결[v1_국내주식-009]] 요청 구조체
/// [국내주식-009 v1] 주식현재가 체결
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireCcnlRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 일자별[v1_국내주식-010]] 요청 구조체
/// [국내주식-010 v1] 주식현재가 일자별
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyPriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_org_adj_prc (String, 선택)
    #[serde(rename = "fid_org_adj_prc")]
    pub fid_org_adj_prc: String,
    /// fid_period_div_code (String, 선택)
    #[serde(rename = "fid_period_div_code")]
    pub fid_period_div_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 호가/예상체결[v1_국내주식-011]] 요청 구조체
/// [국내주식-011 v1] 주식현재가 호가 예상체결
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireAskingPriceExpCcnRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 투자자[v1_국내주식-012]] 요청 구조체
/// [국내주식-012 v1] 주식현재가 투자자
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireInvestorRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 회원사[v1_국내주식-013]] 요청 구조체
/// [국내주식-013 v1] 주식현재가 회원사
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireMemberRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식기간별시세(일/주/월/년)[v1_국내주식-016]] 요청 구조체
/// [국내주식-016 v1] 국내주식기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyItemchartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식당일분봉조회[v1_국내주식-022]] 요청 구조체
/// 주식당일분봉조회[v1_국내주식-022]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeItemchartpriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_etc_cls_code (String, 선택)
    #[serde(rename = "fid_etc_cls_code")]
    pub fid_etc_cls_code: String,
    /// fid_input_hour_1 (String, 선택)
    #[serde(rename = "fid_input_hour_1")]
    pub fid_input_hour_1: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_pw_data_incu_yn (String, 선택)
    #[serde(rename = "fid_pw_data_incu_yn")]
    pub fid_pw_data_incu_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식일별분봉조회 [국내주식-213]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeDailychartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 당일시간대별체결[v1_국내주식-023]] 요청 구조체
/// 주식현재가 당일시간대별체결[v1_국내주식-023]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeItemconclusionRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 시간외일자별주가[v1_국내주식-026]] 요청 구조체
/// 주식현재가 시간외일자별주가[v1_국내주식-026]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyOvertimepriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 시간외시간별체결[v1_국내주식-025]] 요청 구조체
/// 주식현재가 시간외시간별체결[v1_국내주식-025]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeOvertimeconclusionRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외현재가[국내주식-076]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireOvertimePriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외호가[국내주식-077]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireOvertimeAskingPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 장마감 예상체결가[국내주식-120]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpClosingPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ETF/ETN 현재가[v1_국내주식-068]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthEtfetnV1QuotationsInquirePriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ETF 구성종목시세[국내주식-073]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthEtfetnV1QuotationsInquireComponentStockPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [NAV 비교추이(종목)[v1_국내주식-069]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthEtfetnV1QuotationsNavComparisonTrendRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [NAV 비교추이(일)[v1_국내주식-071]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthEtfetnV1QuotationsNavComparisonDailyTrendRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_input_date_1 (String, 선택)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// fid_input_date_2 (String, 선택)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [NAV 비교추이(분)[v1_국내주식-070]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthEtfetnV1QuotationsNavComparisonTimeTrendRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_hour_cls_code (String, 선택)
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 현재가 시세[v1_국내주식-014]] 요청 구조체
/// ELW 현재가 시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireElwPriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 신규상장종목 [국내주식-181]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsNewlyListedRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 민감도 순위[국내주식-170]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1RankingSensitivityRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 기초자산별 종목시세 [국내주식-186]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsUdrlAssetPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 종목검색 [국내주식-166]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsCondSearchRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 당일급변종목[국내주식-171]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1RankingQuickChangeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 기초자산 목록조회 [국내주식-185]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsUdrlAssetListRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 비교대상종목조회 [국내주식-183]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsCompareStocksRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW LP매매추이 [국내주식-182]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsLpTradeTrendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 투자지표추이(체결) [국내주식-172]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsIndicatorTrendCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 투자지표추이(분별) [국내주식-174]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsIndicatorTrendMinuteRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 투자지표추이(일별) [국내주식-173]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsIndicatorTrendDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 변동성 추이(틱) [국내주식-180]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsVolatilityTrendTickRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 변동성추이(체결) [국내주식-177]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsVolatilityTrendCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 변동성 추이(일별) [국내주식-178]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsVolatilityTrendDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 민감도 추이(체결) [국내주식-175]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsSensitivityTrendCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 변동성 추이(분별) [국내주식-179]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsVolatilityTrendMinuteRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 민감도 추이(일별) [국내주식-176]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsSensitivityTrendDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 만기예정/만기종목 [국내주식-184]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1QuotationsExpirationStocksRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 지표순위[국내주식-169]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1RankingIndicatorRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 상승률순위[국내주식-167]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1RankingUpdownRateRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 거래량순위[국내주식-168]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthElwV1RankingVolumeRankRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내업종 현재지수[v1_국내주식-063]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내업종 일자별지수[v1_국내주식-065]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexDailyPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내업종 시간별지수(초)[국내주식-064]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexTickpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내업종 시간별지수(분)[국내주식-119]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexTimepriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [업종 분봉조회[v1_국내주식-045]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireTimeIndexchartpriceRequest {
    /// FID_COND_MRKT_DIV_CODE (String, 선택)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID_INPUT_ISCD (String, 선택)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID_INPUT_HOUR_1 (String, 선택)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// FID_PW_DATA_INCU_YN (String, 선택)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// FID_ETC_CLS_CODE (String, 선택)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식업종기간별시세(일/주/월/년)[v1_국내주식-021]] 요청 구조체
/// [국내주식-021 v1] 업종기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyIndexchartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내업종 구분별전체시세[v1_국내주식-066]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireIndexCategoryPriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_mrkt_cls_code (String, 선택)
    #[serde(rename = "fid_mrkt_cls_code")]
    pub fid_mrkt_cls_code: String,
    /// fid_blng_cls_code (String, 선택)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 예상체결지수 추이[국내주식-121]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpIndexTrendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 예상체결 전체지수[국내주식-122]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpTotalIndexRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_mkop_cls_code (String, 선택)
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
    /// fid_mrkt_cls_code (String, 선택)
    #[serde(rename = "fid_mrkt_cls_code")]
    pub fid_mrkt_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [변동성완화장치(VI) 현황 [v1_국내주식-055]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireViStatusRequest {
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_mrkt_cls_code (String, 선택)
    #[serde(rename = "fid_mrkt_cls_code")]
    pub fid_mrkt_cls_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_input_date_1 (String, 선택)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [금리 종합(국내채권/금리) [국내주식-155]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCompInterestRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종합 시황/공시(제목) [국내주식-141]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsNewsTitleRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내휴장일조회[국내주식-040]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsChkHolidayRequest {
    /// BASS_DT (String, 선택)
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// CTX_AREA_NK (String, 선택)
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// CTX_AREA_FK (String, 선택)
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내선물 영업일조회 [국내주식-160]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsMarketTimeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [상품기본조회[v1_국내주식-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsSearchInfoRequest {
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// PRDT_TYPE_CD (String, 선택)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식기본조회[v1_국내주식-067]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsSearchStockInfoRequest {
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// PRDT_TYPE_CD (String, 선택)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 대차대조표[v1_국내주식-078]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceBalanceSheetRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 손익계산서[v1_국내주식-079]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceIncomeStatementRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 재무비율[v1_국내주식-080]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceFinancialRatioRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 수익성비율[v1_국내주식-081]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceProfitRatioRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 기타주요비율[v1_국내주식-082]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceOtherMajorRatiosRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 안정성비율[v1_국내주식-083]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceStabilityRatioRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 성장성비율[v1_국내주식-085]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1FinanceGrowthRatioRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 당사 신용가능종목[국내주식-111]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCreditByCompanyRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_slct_yn (String, 선택)
    #[serde(rename = "fid_slct_yn")]
    pub fid_slct_yn: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(배당일정)[국내주식-145]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoDividendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(주식매수청구일정)[국내주식-146]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoPurreqRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(합병/분할일정)[국내주식-147]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoMergerSplitRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(액면교체일정)[국내주식-148]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoRevSplitRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(자본감소일정)[국내주식-149]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoCapDcrsRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(상장정보일정)[국내주식-150]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoListInfoRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(공모주청약일정)[국내주식-151]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoPubOfferRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(실권주일정)[국내주식-152]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoForfeitRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(의무예치일정)[국내주식-153]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoMandDepositRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(유상증자일정) [국내주식-143]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoPaidinCapinRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(무상증자일정) [국내주식-144]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoBonusIssueRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [예탁원정보(주주총회일정) [국내주식-154]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1KsdinfoSharehldMeetRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 종목추정실적 [국내주식-187]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsEstimatePerformRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [당사 대주가능 종목 [국내주식-195]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsLendableByCompanyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 종목투자의견 [국내주식-188]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestOpinionRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 증권사별 투자의견 [국내주식-189]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestOpbysecRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목조건검색 목록조회[국내주식-038]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsPsearchTitleRequest {
    /// user_id (String, 선택)
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목조건검색조회 [국내주식-039]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsPsearchResultRequest {
    /// user_id (String, 선택)
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// seq (String, 선택)
    #[serde(rename = "seq")]
    pub seq: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [관심종목 그룹조회 [국내주식-204]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsIntstockGrouplistRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [관심종목(멀티종목) 시세조회 [국내주식-205]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsIntstockMultpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [관심종목 그룹별 종목조회 [국내주식-203]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsIntstockStocklistByGroupRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내기관_외국인 매매종목가집계[국내주식-037]] 요청 구조체
/// 국내기관_외국인 매매종목가집계[국내주식-037]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsForeignInstitutionTotalRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [외국계 매매종목 가집계 [국내주식-161]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsFrgnmemTradeEstimateRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별 투자자매매동향(일별)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestorTradeByStockDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [시장별 투자자매매동향(시세)[v1_국내주식-074]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireInvestorTimeByMarketRequest {
    /// FID_INPUT_ISCD (String, 선택)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID_INPUT_ISCD_2 (String, 선택)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [시장별 투자자매매동향(일별) [국내주식-075]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireInvestorDailyByMarketRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별 외국계 순매수추이 [국내주식-164]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsFrgnmemPchsTrendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [회원사 실시간 매매동향(틱) [국내주식-163]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsFrgnmemTradeTrendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식현재가 회원사 종목매매동향 [국내주식-197]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireMemberDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별 프로그램매매추이(체결)[v1_국내주식-044]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsProgramTradeByStockRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별 프로그램매매추이(일별) [국내주식-113]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsProgramTradeByStockDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별 외인기관 추정가집계[v1_국내주식-046]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestorTrendEstimateRequest {
    /// MKSC_SHRN_ISCD (String, 선택)
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별일별매수매도체결량 [v1_국내주식-056]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInquireDailyTradeVolumeRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_input_date_1 (String, 선택)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// fid_input_date_2 (String, 선택)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
    /// fid_period_div_code (String, 선택)
    #[serde(rename = "fid_period_div_code")]
    pub fid_period_div_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [프로그램매매 종합현황(시간) [국내주식-114]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCompProgramTradeTodayRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [프로그램매매 종합현황(일별)[국내주식-115]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCompProgramTradeDailyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [프로그램매매 투자자매매동향(당일) [국내주식-116]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsInvestorProgramTradeTodayRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 신용잔고 일별추이[국내주식-110]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsDailyCreditBalanceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_input_date_1 (String, 선택)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 예상체결가 추이[국내주식-118]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsExpPriceTrendRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_mkop_cls_code (String, 선택)
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 공매도 일별추이[국내주식-134]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsDailyShortSaleRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외예상체결등락률 [국내주식-140]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingOvertimeExpTransFluctRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 체결금액별 매매비중 [국내주식-192]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsTradprtByamtRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내 증시자금 종합 [국내주식-193]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsMktfundsRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [종목별 일별 대차거래추이 [국내주식-135]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsDailyLoanTransRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 상하한가 포착 [국내주식-190]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsCaptureUplowpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 매물대/거래비중 [국내주식-196]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsPbarTratioRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [거래량순위[v1_국내주식-047]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1QuotationsVolumeRankRequest {
    /// FID_COND_MRKT_DIV_CODE (String, 선택)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID_COND_SCR_DIV_CODE (String, 선택)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID_INPUT_ISCD (String, 선택)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID_DIV_CLS_CODE (String, 선택)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// FID_BLNG_CLS_CODE (String, 선택)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// FID_TRGT_CLS_CODE (String, 선택)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID_TRGT_EXLS_CLS_CODE (String, 선택)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// FID_INPUT_PRICE_1 (String, 선택)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// FID_INPUT_PRICE_2 (String, 선택)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// FID_VOL_CNT (String, 선택)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: i64,
    /// FID_INPUT_DATE_1 (String, 선택)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 등락률 순위[v1_국내주식-088]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingFluctuationRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_input_cnt_1 (String, 선택)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// fid_prc_cls_code (String, 선택)
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_rsfl_rate1 (String, 선택)
    #[serde(rename = "fid_rsfl_rate1")]
    pub fid_rsfl_rate1: String,
    /// fid_rsfl_rate2 (String, 선택)
    #[serde(rename = "fid_rsfl_rate2")]
    pub fid_rsfl_rate2: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 호가잔량 순위[국내주식-089]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingQuoteBalanceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 수익자산지표 순위[v1_국내주식-090]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingProfitAssetIndexRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시가총액 상위[v1_국내주식-091]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingMarketCapRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 재무비율 순위[v1_국내주식-092]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingFinanceRatioRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// fid_input_option_1 (String, 선택)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// fid_input_option_2 (String, 선택)
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_blng_cls_code (String, 선택)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외잔량 순위[v1_국내주식-093]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingAfterHourBalanceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 우선주/괴리율 상위[v1_국내주식-094]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingPreferDisparateRatioRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 이격도 순위[v1_국내주식-095]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingDisparityRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_hour_cls_code (String, 선택)
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시장가치 순위[v1_국내주식-096]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingMarketValueRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// fid_input_option_1 (String, 선택)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// fid_input_option_2 (String, 선택)
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_blng_cls_code (String, 선택)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 체결강도 상위[v1_국내주식-101]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingVolumePowerRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 관심종목등록 상위[v1_국내주식-102]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingTopInterestStockRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_input_price_1 (String, 선택)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// fid_input_price_2 (String, 선택)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_input_iscd_2 (String, 선택)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// fid_input_cnt_1 (String, 선택)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 예상체결 상승/하락상위[v1_국내주식-103]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingExpTransUpdownRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_aply_rang_prc_1 (String, 선택)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// fid_vol_cnt (String, 선택)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// fid_pbmn (String, 선택)
    #[serde(rename = "fid_pbmn")]
    pub fid_pbmn: String,
    /// fid_blng_cls_code (String, 선택)
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// fid_mkop_cls_code (String, 선택)
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 당사매매종목 상위[v1_국내주식-104]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingTradedByCompanyRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_rank_sort_cls_code (String, 선택)
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// fid_input_date_1 (String, 선택)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// fid_input_date_2 (String, 선택)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_aply_rang_prc_1 (String, 선택)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// fid_aply_rang_prc_2 (String, 선택)
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// fid_aply_rang_vol (String, 선택)
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: Decimal,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 신고/신저근접종목 상위[v1_국내주식-105]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingNearNewHighlowRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_cond_scr_div_code (String, 선택)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// fid_div_cls_code (String, 선택)
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// fid_input_cnt_1 (String, 선택)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// fid_input_cnt_2 (String, 선택)
    #[serde(rename = "fid_input_cnt_2")]
    pub fid_input_cnt_2: String,
    /// fid_prc_cls_code (String, 선택)
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// fid_trgt_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// fid_trgt_exls_cls_code (String, 선택)
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// fid_aply_rang_prc_1 (String, 선택)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// fid_aply_rang_prc_2 (String, 선택)
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// fid_aply_rang_vol (String, 선택)
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: Decimal,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 배당률 상위[국내주식-106]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingDividendRateRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 대량체결건수 상위[국내주식-107]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingBulkTransNumRequest {
    /// output (String, 선택)
    #[serde(rename = "output")]
    pub output: String,
    /// rt_cd (String, 선택)
    #[serde(rename = "rt_cd")]
    pub rt_cd: String,
    /// msg_cd (String, 선택)
    #[serde(rename = "msg_cd")]
    pub msg_cd: String,
    /// msg1 (String, 선택)
    #[serde(rename = "msg1")]
    pub msg1: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 신용잔고 상위[국내주식-109]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingCreditBalanceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 공매도 상위종목[국내주식-133]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingShortSaleRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외등락율순위 [국내주식-138]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingOvertimeFluctuationRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외거래량순위 [국내주식-139]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingOvertimeVolumeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [HTS조회상위20종목 [국내주식-214]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticStockV1RankingHtsTopViewRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간체결가 (KRX) [실시간-003]] 요청 구조체
/// [실시간-003] 국내주식 실시간체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// appkey (String, 선택)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// appsecret (String, 선택)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
    /// personalseckey (String, 선택)
    #[serde(rename = "personalseckey")]
    pub personalseckey: String,
    /// custtype (String, 선택)
    #[serde(rename = "custtype")]
    pub custtype: String,
    /// tr_type (String, 선택)
    #[serde(rename = "tr_type")]
    pub tr_type: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간호가 (KRX) [실시간-004]] 요청 구조체
/// [실시간-004] 국내주식 실시간호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간체결통보 [실시간-005]] 요청 구조체
/// [실시간-005] 국내주식 실시간체결통보
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STCNI0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간예상체결 (KRX) [실시간-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STANC0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간회원사 (KRX) [실시간-047]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STMBC0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
}

/// [국내주식 실시간프로그램매매 (KRX) [실시간-048]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STPGM0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 장운영정보 (KRX) [실시간-049]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STMKO0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외 실시간호가 (KRX) [실시간-025]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STOAA0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외 실시간체결가 (KRX) [실시간-042]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STOUP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 시간외 실시간예상체결 (KRX) [실시간-024]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STOAC0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내지수 실시간체결 [실시간-026]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UPCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내지수 실시간예상체결 [실시간-027]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UPANC0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내지수 실시간프로그램매매 [실시간-028]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UPPGM0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 실시간호가 [실시간-062]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EWASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 실시간체결가 [실시간-061]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EWCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [ELW 실시간예상체결 [실시간-063]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EWANC0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내ETF NAV추이 [실시간-051]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0STNAV0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간체결가 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UNCNT0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간호가 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UNASP0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간예상체결 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UNANC0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간회원사 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UNMBC0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간프로그램매매 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UNPGM0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 장운영정보 (통합)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0UNMKO0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간체결가 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0NXCNT0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간호가 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0NXASP0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간예상체결 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0NXANC0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간회원사 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0NXMBC0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 실시간프로그램매매 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0NXPGM0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내주식 장운영정보 (NXT)] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0NXMKO0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 주문[v1_국내선물-001]] 요청 구조체
/// [국내선물-001 v1] 선물옵션주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingOrderRequest {
    /// ORD_PRCS_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// SHTN_PDNO (String, 선택)
    #[serde(rename = "SHTN_PDNO")]
    pub shtn_pdno: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// UNIT_PRICE (String, 선택)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// NMPR_TYPE_CD (String, 선택)
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// KRX_NMPR_CNDT_CD (String, 선택)
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// FUOP_ITEM_DVSN_CD (String, 선택)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 정정취소주문[v1_국내선물-002]] 요청 구조체
/// [국내선물-002 v1] 선물옵션정정취소주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingOrderRvsecnclRequest {
    /// ORD_PRCS_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// RVSE_CNCL_DVSN_CD (String, 선택)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// ORGN_ODNO (String, 선택)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// UNIT_PRICE (String, 선택)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// NMPR_TYPE_CD (String, 선택)
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// KRX_NMPR_CNDT_CD (String, 선택)
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// RMN_QTY_YN (String, 선택)
    #[serde(rename = "RMN_QTY_YN")]
    pub rmn_qty_yn: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// FUOP_ITEM_DVSN_CD (String, 선택)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 주문체결내역조회[v1_국내선물-003]] 요청 구조체
/// [국내선물-003 v1] 선물옵션주문체결내역조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireCcnlRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// STRT_ORD_DT (String, 선택)
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// END_ORD_DT (String, 선택)
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// CCLD_NCCS_DVSN (String, 선택)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// SORT_SQN (String, 선택)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// STRT_ODNO (String, 선택)
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// MKET_ID_CD (String, 선택)
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 잔고현황[v1_국내선물-004]] 요청 구조체
/// [국내선물-004 v1] 선물옵션 잔고현황
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// MGNA_DVSN (String, 선택)
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// EXCC_STAT_CD (String, 선택)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 주문가능[v1_국내선물-005]] 요청 구조체
/// [국내선물-005 v1] 선물옵션 주문가능
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquirePsblOrderRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// UNIT_PRICE (String, 선택)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [(야간)선물옵션 주문체결 내역조회 [국내선물-009]] 요청 구조체
/// (야간)선물옵션 주문체결 내역조회 [국내선물-009]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireNgtCcnlRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// STRT_ORD_DT (String, 선택)
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// END_ORD_DT (String, 선택)
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// CCLD_NCCS_DVSN (String, 선택)
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// SORT_SQN (String, 선택)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// STRT_ODNO (String, 선택)
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// MKET_ID_CD (String, 선택)
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// FUOP_DVSN_CD (String, 선택)
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// SCRN_DVSN (String, 선택)
    #[serde(rename = "SCRN_DVSN")]
    pub scrn_dvsn: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [(야간)선물옵션 잔고현황 [국내선물-010]] 요청 구조체
/// (야간)선물옵션 잔고현황 [국내선물-010]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireNgtBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// ACNT_PWD (String, 선택)
    #[serde(rename = "ACNT_PWD")]
    pub acnt_pwd: String,
    /// MGNA_DVSN (String, 선택)
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// EXCC_STAT_CD (String, 선택)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [(야간)선물옵션 주문가능 조회 [국내선물-011]] 요청 구조체
/// (야간)선물옵션 주문가능 조회 [국내선물-011]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquirePsblNgtOrderRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// PRDT_TYPE_CD (String, 선택)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// UNIT_PRICE (String, 선택)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// ORD_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [(야간)선물옵션 증거금 상세 [국내선물-024]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingNgtMarginDetailRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 잔고정산손익내역[v1_국내선물-013]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireBalanceSettlementPlRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// INQR_DT (String, 선택)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 총자산현황[v1_국내선물-014]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireDepositRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 잔고평가손익내역[v1_국내선물-015]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireBalanceValuationPlRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// MGNA_DVSN (String, 선택)
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// EXCC_STAT_CD (String, 선택)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 기준일체결내역[v1_국내선물-016]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireCcnlBstimeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션기간약정수수료일별[v1_국내선물-017]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1TradingInquireDailyAmountFeeRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// INQR_STRT_DAY (String, 선택)
    #[serde(rename = "INQR_STRT_DAY")]
    pub inqr_strt_day: String,
    /// INQR_END_DAY (String, 선택)
    #[serde(rename = "INQR_END_DAY")]
    pub inqr_end_day: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 증거금률] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsMarginRateRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
    /// enabled (String, 선택)
    #[serde(rename = "enabled")]
    pub enabled: String,
    /// byAppKey (String, 선택)
    #[serde(rename = "byAppKey")]
    pub by_app_key: String,
    /// refreshTimeUnit (String, 선택)
    #[serde(rename = "refreshTimeUnit")]
    pub refresh_time_unit: String,
    /// requestLimit (String, 선택)
    #[serde(rename = "requestLimit")]
    pub request_limit: String,
}

/// [선물옵션 시세[v1_국내선물-006]] 요청 구조체
/// [국내선물-006 v1] 선물옵션 시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquirePriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 시세호가[v1_국내선물-007]] 요청 구조체
/// [국내선물-007 v1] 선물옵션 시세호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquireAskingPriceRequest {
    /// fid_cond_mrkt_div_code (String, 선택)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// fid_input_iscd (String, 선택)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]] 요청 구조체
/// [국내선물-008 v1] 선물옵션기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquireDailyFuopchartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 분봉조회[v1_국내선물-012]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsInquireTimeFuopchartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
    /// enabled (String, 선택)
    #[serde(rename = "enabled")]
    pub enabled: String,
    /// byAppKey (String, 선택)
    #[serde(rename = "byAppKey")]
    pub by_app_key: String,
    /// refreshTimeUnit (String, 선택)
    #[serde(rename = "refreshTimeUnit")]
    pub refresh_time_unit: String,
    /// requestLimit (String, 선택)
    #[serde(rename = "requestLimit")]
    pub request_limit: String,
}

/// [국내옵션전광판_옵션월물리스트[국내선물-020]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardOptionListRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내선물 기초자산 시세[국내선물-021]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardTopRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내옵션전광판_콜풋[국내선물-022]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardCallputRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [국내옵션전광판_선물[국내선물-023]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsDisplayBoardFuturesRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 일중예상체결추이[국내선물-018]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DomesticFutureoptionV1QuotationsExpPriceTrendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [지수선물 실시간호가[실시간-011]] 요청 구조체
/// [실시간-011] 지수선물 실시간호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0IFASP0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [지수선물 실시간체결가[실시간-010]] 요청 구조체
/// [실시간-010] 지수선물 실시간체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0IFCNT0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [지수옵션 실시간호가[실시간-015]] 요청 구조체
/// [실시간-015] 지수옵션 실시간호가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0IOASP0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [지수옵션  실시간체결가[실시간-014]] 요청 구조체
/// [실시간-014] 지수옵션 실시간체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0IOCNT0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [선물옵션 실시간체결통보[실시간-012]] 요청 구조체
/// [실시간-012] 지수선물옵션 실시간체결 통보
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0IFCNI0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [상품선물 실시간호가[실시간-023]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0CFASP0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [상품선물 실시간체결가[실시간-022]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0CFCNT0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식선물 실시간호가 [실시간-030]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0ZFASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식선물 실시간체결가 [실시간-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0ZFCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식선물 실시간예상체결 [실시간-031]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0ZFANC0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식옵션 실시간호가 [실시간-045]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0ZOASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식옵션 실시간체결가 [실시간-044]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0ZOCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [주식옵션 실시간예상체결 [실시간-046]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0ZOANC0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간옵션 실시간호가 [실시간-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EUASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간옵션 실시간체결가 [실시간-032]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EUCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간옵션실시간예상체결 [실시간-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EUANC0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간옵션실시간체결통보 [실시간-067]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0EUCNI0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간선물 실시간호가 [실시간-065]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0MFASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간선물 실시간종목체결 [실시간-064]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0MFCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [KRX야간선물 실시간체결통보 [실시간-066]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0MFCNI0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 주문[v1_해외주식-001]] 요청 구조체
/// [해외주식-001 v1] 해외주식 주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_EXCG_CD (String, 선택)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// OVRS_ORD_UNPR (String, 선택)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: Decimal,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// rt_cd (String, 선택)
    #[serde(rename = "rt_cd")]
    pub rt_cd: String,
    /// msg_cd (String, 선택)
    #[serde(rename = "msg_cd")]
    pub msg_cd: String,
    /// msg1 (String, 선택)
    #[serde(rename = "msg1")]
    pub msg1: String,
    /// output (String, 선택)
    #[serde(rename = "output")]
    pub output: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 정정취소주문[v1_해외주식-003]] 요청 구조체
/// [해외주식-003 v1] 해외주식 정정취소주문
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderRvsecnclRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_EXCG_CD (String, 선택)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORGN_ODNO (String, 선택)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// RVSE_CNCL_DVSN_CD (String, 선택)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// OVRS_ORD_UNPR (String, 선택)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: Decimal,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 예약주문접수[v1_해외주식-002]] 요청 구조체
/// [해외주식-002 v1] 해외주식 예약주문접수
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderResvRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// OVRS_EXCG_CD (String, 선택)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// FT_ORD_QTY (String, 선택)
    #[serde(rename = "FT_ORD_QTY")]
    pub ft_ord_qty: Decimal,
    /// FT_ORD_UNPR3 (String, 선택)
    #[serde(rename = "FT_ORD_UNPR3")]
    pub ft_ord_unpr3: Decimal,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 예약주문접수취소[v1_해외주식-004]] 요청 구조체
/// [해외주식-004 v1] 해외주식 예약주문접수취소
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderResvCcnlRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// RSVN_ORD_RCIT_DT (String, 선택)
    #[serde(rename = "RSVN_ORD_RCIT_DT")]
    pub rsvn_ord_rcit_dt: String,
    /// OVRS_RSVN_ODNO (String, 선택)
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 매수가능금액조회[v1_해외주식-014]] 요청 구조체
/// [v1_해외주식-014]해외주식 매수가능금액조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePsamountRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// echm_af_ord_psbl_amt (String, 선택)
    #[serde(rename = "echm_af_ord_psbl_amt")]
    pub echm_af_ord_psbl_amt: Decimal,
    /// echm_af_ord_psbl_qty (String, 선택)
    #[serde(rename = "echm_af_ord_psbl_qty")]
    pub echm_af_ord_psbl_qty: Decimal,
    /// exrt (String, 선택)
    #[serde(rename = "exrt")]
    pub exrt: String,
    /// frcr_ord_psbl_amt1 (String, 선택)
    #[serde(rename = "frcr_ord_psbl_amt1")]
    pub frcr_ord_psbl_amt1: Decimal,
    /// max_ord_psbl_qty (String, 선택)
    #[serde(rename = "max_ord_psbl_qty")]
    pub max_ord_psbl_qty: Decimal,
    /// ord_psbl_frcr_amt (String, 선택)
    #[serde(rename = "ord_psbl_frcr_amt")]
    pub ord_psbl_frcr_amt: Decimal,
    /// ord_psbl_qty (String, 선택)
    #[serde(rename = "ord_psbl_qty")]
    pub ord_psbl_qty: Decimal,
    /// ovrs_max_ord_psbl_qty (String, 선택)
    #[serde(rename = "ovrs_max_ord_psbl_qty")]
    pub ovrs_max_ord_psbl_qty: Decimal,
    /// ovrs_ord_psbl_amt (String, 선택)
    #[serde(rename = "ovrs_ord_psbl_amt")]
    pub ovrs_ord_psbl_amt: Decimal,
    /// sll_ruse_psbl_amt (String, 선택)
    #[serde(rename = "sll_ruse_psbl_amt")]
    pub sll_ruse_psbl_amt: Decimal,
    /// tr_crcy_cd (String, 선택)
    #[serde(rename = "tr_crcy_cd")]
    pub tr_crcy_cd: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 미체결내역[v1_해외주식-005]] 요청 구조체
/// [해외주식-005 v1] 해외주식 미체결내역
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireNccsRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_EXCG_CD (String, 선택)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// SORT_SQN (String, 선택)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 잔고[v1_해외주식-006]] 요청 구조체
/// [해외주식-006 v1] 해외주식 잔고
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_EXCG_CD (String, 선택)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// TR_CRCY_CD (String, 선택)
    #[serde(rename = "TR_CRCY_CD")]
    pub tr_crcy_cd: String,
    /// CTX_AREA_FK200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// CTX_AREA_NK200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 주문체결내역[v1_해외주식-007]] 요청 구조체
/// [해외주식-007 v1] 해외주식 주문체결내역
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 체결기준현재잔고[v1_해외주식-008]] 요청 구조체
/// [해외주식-008 v1] 해외주식 체결기준현재잔고
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePresentBalanceRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// WCRC_FRCR_DVSN_CD (String, 선택)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// TR_MKET_CD (String, 선택)
    #[serde(rename = "TR_MKET_CD")]
    pub tr_mket_cd: String,
    /// NATN_CD (String, 선택)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// INQR_DVSN_CD (String, 선택)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 예약주문조회[v1_해외주식-013]] 요청 구조체
/// [v1_해외주식-013]해외주식 예약주문조회
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingOrderResvListRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 결제기준잔고 [해외주식-064]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePaymtStdrBalanceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 일별거래내역 [해외주식-063]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePeriodTransRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 기간손익[v1_해외주식-032]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquirePeriodProfitRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외증거금 통화별조회 [해외주식-035]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingForeignMarginRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 미국주간주문[v1_해외주식-026]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingDaytimeOrderRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 미국주간정정취소[v1_해외주식-027]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingDaytimeOrderRvsecnclRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_EXCG_CD (String, 선택)
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORGN_ODNO (String, 선택)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// RVSE_CNCL_DVSN_CD (String, 선택)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// ORD_QTY (String, 선택)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: Decimal,
    /// OVRS_ORD_UNPR (String, 선택)
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: Decimal,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 지정가주문번호조회  [해외주식-071]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingAlgoOrdnoRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 지정가체결내역조회 [해외주식-070]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1TradingInquireAlgoCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 현재가상세[v1_해외주식-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsPriceDetailRequest {
    /// AUTH (String, 선택)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// EXCD (String, 선택)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// SYMB (String, 선택)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 현재가 호가 [해외주식-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireAskingPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 현재체결가[v1_해외주식-009]] 요청 구조체
/// [해외주식-009 v1] 해외주식 현재체결가
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsPriceRequest {
    /// AUTH (String, 선택)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// EXCD (String, 선택)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// SYMB (String, 선택)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 체결추이[해외주식-037]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식분봉조회[v1_해외주식-030]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireTimeItemchartpriceRequest {
    /// AUTH (String, 선택)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// EXCD (String, 선택)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// SYMB (String, 선택)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// NMIN (String, 선택)
    #[serde(rename = "NMIN")]
    pub nmin: String,
    /// PINC (String, 선택)
    #[serde(rename = "PINC")]
    pub pinc: String,
    /// NEXT (String, 선택)
    #[serde(rename = "NEXT")]
    pub next: String,
    /// NREC (String, 선택)
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// FILL (String, 선택)
    #[serde(rename = "FILL")]
    pub fill: String,
    /// KEYB (String, 선택)
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외지수분봉조회[v1_해외주식-031]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireTimeIndexchartpriceRequest {
    /// FID_COND_MRKT_DIV_CODE (String, 선택)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID_INPUT_ISCD (String, 선택)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID_HOUR_CLS_CODE (String, 선택)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// FID_PW_DATA_INCU_YN (String, 선택)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 기간별시세[v1_해외주식-010]] 요청 구조체
/// [해외주식-010 v1] 해외주식 기간별시세
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsDailypriceRequest {
    /// AUTH (String, 선택)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// EXCD (String, 선택)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// SYMB (String, 선택)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// GUBN (String, 선택)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// BYMD (String, 선택)
    #[serde(rename = "BYMD")]
    pub bymd: String,
    /// MODP (String, 선택)
    #[serde(rename = "MODP")]
    pub modp: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]] 요청 구조체
/// [v1_해외주식-012] 해외지수/환율기간별시세(일/주/월/년)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireDailyChartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// output1 (String, 선택)
    #[serde(rename = "output1")]
    pub output1: String,
    /// output2 (String, 선택)
    #[serde(rename = "output2")]
    pub output2: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식조건검색[v1_해외주식-015]] 요청 구조체
/// 해외주식조건검색[v1_해외주식-015]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsInquireSearchRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// output1 (String, 선택)
    #[serde(rename = "output1")]
    pub output1: String,
    /// output2 (String, 선택)
    #[serde(rename = "output2")]
    pub output2: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외결제일자조회[해외주식-017]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1QuotationsCountriesHolidayRequest {
    /// TRAD_DT (String, 선택)
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// CTX_AREA_NK (String, 선택)
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// CTX_AREA_FK (String, 선택)
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 상품기본정보[v1_해외주식-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsSearchInfoRequest {
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// PRDT_TYPE_CD (String, 선택)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 업종별시세[해외주식-048]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsIndustryThemeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 업종별코드조회[해외주식-049]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsIndustryPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 복수종목 시세조회] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsMultpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 가격급등락[해외주식-038]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingPriceFluctRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 거래량급증[해외주식-039]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingVolumeSurgeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 매수체결강도상위[해외주식-040]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingVolumePowerRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 상승율/하락율[해외주식-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingUpdownRateRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 신고/신저가[해외주식-042]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingNewHighlowRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 거래량순위[해외주식-043]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradeVolRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 거래대금순위[해외주식-044]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradePbmnRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 거래증가율순위[해외주식-045]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradeGrowthRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 거래회전율순위[해외주식-046]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingTradeTurnoverRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 시가총액순위[해외주식-047]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasStockV1RankingMarketCapRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 기간별권리조회 [해외주식-052]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsPeriodRightsRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외뉴스종합(제목) [해외주식-053]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsNewsTitleRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 권리종합 [해외주식-050]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsRightsByIceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [당사 해외주식담보대출 가능 종목 [해외주식-051]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsColableByCompanyRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외속보(제목) [해외주식-055]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OverseasPriceV1QuotationsBrknewsTitleRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 실시간호가[실시간-021]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFSASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 지연호가(아시아)[실시간-008]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFSASP1Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 실시간지연체결가[실시간-007]] 요청 구조체
/// [실시간-007] 해외주식 실시간지연체결통보
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFSCNT0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외주식 실시간체결통보[실시간-009]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0GSCNI0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 주문 [v1_해외선물-001]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingOrderRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_FUTR_FX_PDNO (String, 선택)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM_LQD_USTL_CCLD_DT (String, 선택)
    #[serde(rename = "FM_LQD_USTL_CCLD_DT")]
    pub fm_lqd_ustl_ccld_dt: String,
    /// FM_LQD_USTL_CCNO (String, 선택)
    #[serde(rename = "FM_LQD_USTL_CCNO")]
    pub fm_lqd_ustl_ccno: String,
    /// PRIC_DVSN_CD (String, 선택)
    #[serde(rename = "PRIC_DVSN_CD")]
    pub pric_dvsn_cd: String,
    /// FM_LIMIT_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: Decimal,
    /// FM_STOP_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: Decimal,
    /// FM_ORD_QTY (String, 선택)
    #[serde(rename = "FM_ORD_QTY")]
    pub fm_ord_qty: Decimal,
    /// FM_LQD_LMT_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: Decimal,
    /// FM_LQD_STOP_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: Decimal,
    /// CCLD_CNDT_CD (String, 선택)
    #[serde(rename = "CCLD_CNDT_CD")]
    pub ccld_cndt_cd: String,
    /// CPLX_ORD_DVSN_CD (String, 선택)
    #[serde(rename = "CPLX_ORD_DVSN_CD")]
    pub cplx_ord_dvsn_cd: String,
    /// ECIS_RSVN_ORD_YN (String, 선택)
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
    /// FM_HDGE_ORD_SCRN_YN (String, 선택)
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// rt_cd (String, 선택)
    #[serde(rename = "rt_cd")]
    pub rt_cd: String,
    /// msg_cd (String, 선택)
    #[serde(rename = "msg_cd")]
    pub msg_cd: String,
    /// msg1 (String, 선택)
    #[serde(rename = "msg1")]
    pub msg1: String,
    /// output (String, 선택)
    #[serde(rename = "output")]
    pub output: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 정정취소주문 [v1_해외선물-002, 003]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingOrderRvsecnclRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// ORGN_ORD_DT (String, 선택)
    #[serde(rename = "ORGN_ORD_DT")]
    pub orgn_ord_dt: String,
    /// ORGN_ODNO (String, 선택)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// FM_MKPR_CVSN_YN (String, 선택)
    #[serde(rename = "FM_MKPR_CVSN_YN")]
    pub fm_mkpr_cvsn_yn: String,
    /// FM_HDGE_ORD_SCRN_YN (String, 선택)
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// FM_LIMIT_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: Decimal,
    /// FM_STOP_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: Decimal,
    /// FM_LQD_LMT_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: Decimal,
    /// FM_LQD_STOP_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: Decimal,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// rt_cd (String, 선택)
    #[serde(rename = "rt_cd")]
    pub rt_cd: String,
    /// msg_cd (String, 선택)
    #[serde(rename = "msg_cd")]
    pub msg_cd: String,
    /// msg1 (String, 선택)
    #[serde(rename = "msg1")]
    pub msg1: String,
    /// output (String, 선택)
    #[serde(rename = "output")]
    pub output: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 당일주문내역조회 [v1_해외선물-004]] 요청 구조체
/// 해외선물옵션 당일주문내역조회 [v1_해외선물-004]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquireCcldRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]] 요청 구조체
/// 해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquireUnpdRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 주문가능조회 [v1_해외선물-006]] 요청 구조체
/// 해외선물옵션 주문가능조회 [v1_해외선물-006]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquirePsamountRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// OVRS_FUTR_FX_PDNO (String, 선택)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// SLL_BUY_DVSN_CD (String, 선택)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM_ORD_PRIC (String, 선택)
    #[serde(rename = "FM_ORD_PRIC")]
    pub fm_ord_pric: Decimal,
    /// ECIS_RSVN_ORD_YN (String, 선택)
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 기간계좌손익 일별[해외선물-010]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquirePeriodCcldRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 일별 체결내역[해외선물-011]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquireDailyCcldRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 예수금현황[해외선물-012]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquireDepositRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 일별 주문내역[해외선물-013]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquireDailyOrderRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 기간계좌거래내역[해외선물-014]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingInquirePeriodTransRequest {
    /// INQR_TERM_FROM_DT (String, 선택)
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// INQR_TERM_TO_DT (String, 선택)
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// ACNT_TR_TYPE_CD (String, 선택)
    #[serde(rename = "ACNT_TR_TYPE_CD")]
    pub acnt_tr_type_cd: String,
    /// CRCY_CD (String, 선택)
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// CTX_AREA_FK100 (String, 선택)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// CTX_AREA_NK100 (String, 선택)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// PWD_CHK_YN (String, 선택)
    #[serde(rename = "PWD_CHK_YN")]
    pub pwd_chk_yn: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 증거금상세 [해외선물-032]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1TradingMarginDetailRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물종목현재가 [v1_해외선물-009]] 요청 구조체
/// 해외선물종목현재가 [v1_해외선물-009]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsInquirePriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물종목상세 [v1_해외선물-008]] 요청 구조체
/// 해외선물종목상세 [v1_해외선물-008]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsStockDetailRequest {
    /// SRS_CD (String, 선택)
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 호가 [해외선물-031]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsInquireAskingPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 분봉조회[해외선물-016]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsInquireTimeFuturechartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 체결추이(틱)[해외선물-019]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsTickCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 체결추이(주간)[해외선물-017]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsWeeklyCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 체결추이(일간)[해외선물-018]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsDailyCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 체결추이(월간)[해외선물-020]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsMonthlyCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 상품기본정보 [해외선물-023]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsSearchContractDetailRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물 미결제추이 [해외선물-029]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsInvestorUnpdTrendRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션종목현재가 [해외선물-035]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션종목상세 [해외선물-034]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptDetailRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 호가 [해외선물-033]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptAskingPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 분봉조회 [해외선물-040]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsInquireTimeOptchartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 체결추이(틱) [해외선물-038]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptTickCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 체결추이(일간) [해외선물-037]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptDailyCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 체결추이(주간) [해외선물-036]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptWeeklyCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 체결추이(월간) [해외선물-039]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsOptMonthlyCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외옵션 상품기본정보 [해외선물-041]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsSearchOptDetailRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 장운영시간 [해외선물-030]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOverseasFutureoptionV1QuotationsMarketTimeRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 실시간체결가[실시간-017]] 요청 구조체
/// 해외선물옵션체결[실시간-017]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFFF020Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 실시간호가[실시간-018]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFFF010Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 실시간주문내역통보[실시간-019]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFFF1C0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [해외선물옵션 실시간체결내역통보[실시간-020]] 요청 구조체
/// 해외선물옵션체결내역통보[실시간-020]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutHDFFF2C0Request {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 매수주문 [국내주식-124]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingBuyRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_QTY2 (String, 선택)
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// BOND_ORD_UNPR (String, 선택)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: Decimal,
    /// SAMT_MKET_PTCI_YN (String, 선택)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// BOND_RTL_MKET_YN (String, 선택)
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// IDCR_STFNO (String, 선택)
    #[serde(rename = "IDCR_STFNO")]
    pub idcr_stfno: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 매도주문 [국내주식-123]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingSellRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// ORD_DVSN (String, 선택)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORD_QTY2 (String, 선택)
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// BOND_ORD_UNPR (String, 선택)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: Decimal,
    /// SPRX_YN (String, 선택)
    #[serde(rename = "SPRX_YN")]
    pub sprx_yn: String,
    /// BUY_DT (String, 선택)
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// BUY_SEQ (String, 선택)
    #[serde(rename = "BUY_SEQ")]
    pub buy_seq: String,
    /// SAMT_MKET_PTCI_YN (String, 선택)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// SLL_AGCO_OPPS_SLL_YN (String, 선택)
    #[serde(rename = "SLL_AGCO_OPPS_SLL_YN")]
    pub sll_agco_opps_sll_yn: String,
    /// BOND_RTL_MKET_YN (String, 선택)
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 정정취소주문 [국내주식-125]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingOrderRvsecnclRequest {
    /// CANO (String, 선택)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// ACNT_PRDT_CD (String, 선택)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// PDNO (String, 선택)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// ORGN_ODNO (String, 선택)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// ORD_QTY2 (String, 선택)
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// BOND_ORD_UNPR (String, 선택)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: Decimal,
    /// QTY_ALL_ORD_YN (String, 선택)
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// RVSE_CNCL_DVSN_CD (String, 선택)
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// MGCO_APTM_ODNO (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// ORD_SVR_DVSN_CD (String, 선택)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// CTAC_TLNO (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [채권정정취소가능주문조회  [국내주식-126]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingInquirePsblRvsecnclRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 주문체결내역 [국내주식-127]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingInquireDailyCcldRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 잔고조회  [국내주식-198]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingInquireBalanceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 매수가능조회 [국내주식-199]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1TradingInquirePsblOrderRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권현재가(호가) [국내주식-132]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsInquireAskingPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권현재가(시세) [국내주식-200]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsInquirePriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권현재가(체결) [국내주식-201]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsInquireCcnlRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권현재가(일별) [국내주식-202]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsInquireDailyPriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 기간별시세(일) [국내주식-159]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsInquireDailyItemchartpriceRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 평균단가조회 [국내주식-158]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsAvgUnitRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 발행정보[국내주식-156]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsIssueInfoRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [장내채권 기본조회 [국내주식-129]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthDomesticBondV1QuotationsSearchBondInfoRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [일반채권 실시간체결가 [실시간-052]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0BJCNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [일반채권 실시간호가 [실시간-053]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0BJASP0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [채권지수 실시간체결가 [실시간-060]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthTryitoutH0BICNT0Request {
    /// header (String, 선택)
    #[serde(rename = "header")]
    pub header: String,
    /// body (String, 선택)
    #[serde(rename = "body")]
    pub body: String,
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}
