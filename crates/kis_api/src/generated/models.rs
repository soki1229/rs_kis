#![allow(clippy::doc_lazy_continuation)]
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// [접근토큰발급(P)] 요청 구조체
/// OAuth인증
/// 접근토큰발급(P)[인증-001]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TokenPRequest {
    /// 권한부여 Type (String, 필수)
    /// client_credentials
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// 앱키 (String, 필수)
    /// 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 앱시크릿키 (String, 필수)
    /// 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
}

/// [접근토큰폐기(P)] 요청 구조체
/// OAuth인증
/// 접근토큰폐기(P)[인증-002]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct RevokePRequest {
    /// 고객 앱Key (String, 필수)
    /// 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 고객 앱Secret (String, 필수)
    /// 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
    /// 접근토큰 (String, 필수)
    /// OAuth 토큰이 필요한 API 경우 발급한 Access token
    /// 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)
    /// 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)
    #[serde(rename = "token")]
    pub token: String,
}

/// [Hashkey] 요청 구조체
/// OAuth인증
/// Hashkey
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct HashkeyRequest {
    /// 요청값 (Object, 필수)
    /// POST로 보낼 body값
    ///
    /// ex)
    /// datas = {
    /// "CANO": '00000000',
    /// "ACNT_PRDT_CD": "01",
    /// "OVRS_EXCG_CD": "SHAA"
    /// }
    #[serde(rename = "JsonBody")]
    pub json_body: String,
}

/// [실시간 (웹소켓) 접속키 발급] 요청 구조체
/// OAuth인증
/// 실시간 (웹소켓) 접속키 발급[실시간-000]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ApprovalRequest {
    /// 권한부여타입 (, 필수)
    /// "client_credentials"
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// 앱키 (, 필수)
    /// 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 시크릿키 (, 필수)
    /// 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.)
    /// * 주의 : appsecret와 secretkey는 동일하오니 착오없으시기 바랍니다. (용어가 다른점 양해 부탁드립니다.)
    #[serde(rename = "secretkey")]
    pub secretkey: String,
}

/// [주식주문(현금)] 요청 구조체
/// 국내주식 현금 주문을 수행하는 API입니다. (매수/매도)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderCashRequest {
    /// 종합계좌번호 (String, 필수)
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 상품유형코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목코드(6자리) , ETN의 경우 7자리 입력
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형 (매도주문 시) (String, 선택)
    /// 01@일반매도
    /// 02@임의매매
    /// 05@대차매도
    /// → 미입력시 01 일반매도로 진행
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 주문구분 (String, 필수)
    /// [KRX]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 05 : 장전 시간외
    /// 06 : 장후 시간외
    /// 07 : 시간외 단일가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 21 : 중간가
    /// 22 : 스톱지정가
    /// 23 : 중간가IOC
    /// 24 : 중간가FOK
    ///
    /// [NXT]
    /// 00 : 지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 21 : 중간가
    /// 22 : 스톱지정가
    /// 23 : 중간가IOC
    /// 24 : 중간가FOK
    ///
    /// [SOR]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문수량 (String, 필수)
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    /// 주문단가
    /// 시장가 등 주문시, "0"으로 입력
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 조건가격 (String, 선택)
    /// 스탑지정가호가 주문 (ORD_DVSN이 22) 사용 시에만 필수
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
    /// 거래소ID구분코드 (String, 선택)
    /// 한국거래소 : KRX
    /// 대체거래소 (넥스트레이드) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// → 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
}

/// [주식주문(신용)] 요청 구조체
/// [국내주식] 주문/계좌
/// 주식주문(신용)[v1_국내주식-002]
/// krx_fwdg_ord_orgno
/// ord_tmd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderCreditRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목코드(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형 (String, 선택)
    /// 공란 입력
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 신용유형 (String, 필수)
    /// [매도] 22 : 유통대주신규, 24 : 자기대주신규, 25 : 자기융자상환, 27 : 유통융자상환
    /// [매수] 21 : 자기융자신규, 23 : 유통융자신규 , 26 : 유통대주상환, 28 : 자기대주상환
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// 대출일자 (String, 필수)
    /// [신용매수]
    /// 신규 대출로, 오늘날짜(yyyyMMdd)) 입력
    ///
    /// [신용매도]
    /// 매도할 종목의 대출일자(yyyyMMdd)) 입력
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 주문구분 (String, 필수)
    /// [KRX]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 05 : 장전 시간외
    /// 06 : 장후 시간외
    /// 07 : 시간외 단일가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 21 : 중간가
    /// 22 : 스톱지정가
    /// 23 : 중간가IOC
    /// 24 : 중간가FOK
    ///
    /// [NXT]
    /// 00 : 지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 21 : 중간가
    /// 22 : 스톱지정가
    /// 23 : 중간가IOC
    /// 24 : 중간가FOK
    ///
    /// [SOR]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    /// 1주당 가격
    /// * 장전 시간외, 장후 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 예약주문여부 (String, 선택)
    /// 정규 증권시장이 열리지 않는 시간 (15:10분 ~ 익일 7:30분) 에 주문을 미리 설정 하여 다음 영업일 또는 설정한 기간 동안 아침 동시 호가에 주문하는 것
    /// Y : 예약주문
    /// N : 신용주문
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
    /// 한국거래소 : KRX
    /// 대체거래소 (넥스트레이드) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// → 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
    /// 조건가격 (String, 선택)
    /// 스탑지정가호가에서 사용
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
}

/// [주식주문(정정취소)] 요청 구조체
/// [국내주식] 주문/계좌
/// 주식주문(정정취소)[v1_국내주식-003]
/// krx_fwdg_ord_orgno
/// ord_tmd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 상품유형코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 한국거래소전송주문조직번호 (String, 필수)
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    /// 원주문번호 (String, 필수)
    /// 원주문번호
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문구분 (String, 필수)
    /// [KRX]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 05 : 장전 시간외
    /// 06 : 장후 시간외
    /// 07 : 시간외 단일가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 21 : 중간가
    /// 22 : 스톱지정가
    /// 23 : 중간가IOC
    /// 24 : 중간가FOK
    ///
    /// [NXT]
    /// 00 : 지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 21 : 중간가
    /// 22 : 스톱지정가
    /// 23 : 중간가IOC
    /// 24 : 중간가FOK
    ///
    /// [SOR]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 정정취소구분코드 (String, 필수)
    /// 01@정정
    /// 02@취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (String, 필수)
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    /// 주문단가
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 잔량전부주문여부 (String, 필수)
    /// 'Y@전량
    /// N@일부'
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 조건가격 (String, 선택)
    /// 스탑지정가호가에서 사용
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
    /// 거래소ID구분코드 (String, 선택)
    /// 한국거래소 : KRX
    /// 대체거래소 (넥스트레이드) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// → 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
}

/// [주식정정취소가능주문조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    /// '공란 : 최초 조회시는
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    /// '공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 조회구분1 (String, 필수)
    /// '0 주문
    /// 1 종목'
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 조회구분2 (String, 필수)
    /// '0 전체
    /// 1 매도
    /// 2 매수'
    #[serde(rename = "INQR_DVSN_2")]
    pub inqr_dvsn_2: String,
}

/// [주식일별주문체결조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    /// YYYYMMDD
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// YYYYMMDD
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 매도매수구분코드 (String, 필수)
    /// 00 : 전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 상품번호 (String, 선택)
    /// 종목번호(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문채번지점번호 (String, 필수)
    /// 주문시 한국투자증권 시스템에서 지정된 영업점코드
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호 (String, 선택)
    /// 주문시 한국투자증권 시스템에서 채번된 주문번호
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 체결구분 (String, 필수)
    /// '00 전체
    /// 01 체결
    /// 02 미체결'
    #[serde(rename = "CCLD_DVSN")]
    pub ccld_dvsn: String,
    /// 조회구분 (String, 필수)
    /// '00 역순
    /// 01 정순'
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 조회구분1 (String, 필수)
    /// '없음: 전체
    /// 1: ELW
    /// 2: 프리보드'
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 조회구분3 (String, 필수)
    /// '00 전체
    /// 01 현금
    /// 02 신용
    /// 03 담보
    /// 04 대주
    /// 05 대여
    /// 06 자기융자신규/상환
    /// 07 유통융자신규/상환'
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// 거래소ID구분코드 (String, 필수)
    /// 한국거래소 : KRX
    /// 대체거래소 (NXT) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// ALL : 전체
    /// ※ 모의투자는 KRX만 제공
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    /// '공란 : 최초 조회시는
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    /// '공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [주식잔고조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시간외단일가, 거래소여부 (String, 필수)
    /// N : 기본값,
    /// Y : 시간외단일가,
    /// X : NXT 정규장 (프리마켓, 메인, 애프터마켓)
    /// ※ NXT 선택 시 : NXT 거래종목만 시세 등 정보가 NXT 기준으로 변동됩니다. KRX 종목들은 그대로 유지
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// 오프라인여부 (String, 선택)
    /// 공란(Default)
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// 조회구분 (String, 필수)
    /// 01 : 대출일별
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 단가구분 (String, 필수)
    /// 01 : 기본값
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// 펀드결제분포함여부 (String, 필수)
    /// N : 포함하지 않음
    /// Y : 포함
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// 융자금액자동상환여부 (String, 필수)
    /// N : 기본값
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// 처리구분 (String, 필수)
    /// 00 : 전일매매포함
    /// 01 : 전일매매미포함
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// 연속조회검색조건100 (String, 선택)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 선택)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [매수가능조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목번호(6자리)
    /// * PDNO, ORD_UNPR 공란 입력 시, 매수수량 없이 매수금액만 조회됨
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문단가 (String, 필수)
    /// 1주당 가격
    /// * 시장가(ORD_DVSN:01)로 조회 시, 공란으로 입력
    /// * PDNO, ORD_UNPR 공란 입력 시, 매수수량 없이 매수금액만 조회됨
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 주문구분 (String, 필수)
    /// * 특정 종목 전량매수 시 가능수량을 확인할 경우
    /// 00:지정가는 증거금율이 반영되지 않으므로
    /// 증거금율이 반영되는 01: 시장가로 조회
    /// * 다만, 조건부지정가 등 특정 주문구분(ex.IOC)으로 주문 시 가능수량을 확인할 경우 주문 시와 동일한 주문구분(ex.IOC) 입력하여 가능수량 확인
    /// * 종목별 매수가능수량 조회 없이 매수금액만 조회하고자 할 경우 임의값(00) 입력
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 05 : 장전 시간외
    /// 06 : 장후 시간외
    /// 07 : 시간외 단일가
    /// 08 : 자기주식
    /// 09 : 자기주식S-Option
    /// 10 : 자기주식금전신탁
    /// 11 : IOC지정가 (즉시체결,잔량취소)
    /// 12 : FOK지정가 (즉시체결,전량취소)
    /// 13 : IOC시장가 (즉시체결,잔량취소)
    /// 14 : FOK시장가 (즉시체결,전량취소)
    /// 15 : IOC최유리 (즉시체결,잔량취소)
    /// 16 : FOK최유리 (즉시체결,전량취소)
    /// 51 : 장중대량
    /// 52 : 장중바스켓
    /// 62 : 장개시전 시간외대량
    /// 63 : 장개시전 시간외바스켓
    /// 67 : 장개시전 금전신탁자사주
    /// 69 : 장개시전 자기주식
    /// 72 : 시간외대량
    /// 77 : 시간외자사주신탁
    /// 79 : 시간외대량자기주식
    /// 80 : 바스켓
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// CMA평가금액포함여부 (String, 필수)
    /// Y : 포함
    /// N : 포함하지 않음
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 해외포함여부 (String, 필수)
    /// Y : 포함
    /// N : 포함하지 않음
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
}

/// [매도가능수량조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblSellRequest {
    /// 종합계좌번호 (String, 필수)
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목번호 (String, 필수)
    /// 보유종목 코드 ex)000660
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [신용매수가능조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCreditPsamountRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목코드(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문단가 (String, 필수)
    /// 1주당 가격
    /// * 장전 시간외, 장후 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 주문구분 (String, 필수)
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 03 : 최유리지정가
    /// 04 : 최우선지정가
    /// 05 : 장전 시간외
    /// 06 : 장후 시간외
    /// 07 : 시간외 단일가 등
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 신용유형 (String, 필수)
    /// 21 : 자기융자신규
    /// 23 : 유통융자신규
    /// 26 : 유통대주상환
    /// 28 : 자기대주상환
    /// 25 : 자기융자상환
    /// 27 : 유통융자상환
    /// 22 : 유통대주신규
    /// 24 : 자기대주신규
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// CMA평가금액포함여부 (String, 필수)
    /// Y/N
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 해외포함여부 (String, 필수)
    /// Y/N
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
}

/// [주식예약주문] 요청 구조체
/// [국내주식] 주문/계좌
/// 주식예약주문[v1_국내주식-017]
/// rsvn_ord_seq
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드(6자리) (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    /// 주문주식수
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    /// 1주당 가격
    /// * 장전 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 매도매수구분코드 (String, 필수)
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 05 : 장전 시간외
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 주문대상잔고구분코드 (String, 필수)
    /// [매도매수구분코드 01:매도/02:매수시 사용]
    /// 10 : 현금
    ///
    /// [매도매수구분코드 01:매도시 사용]
    /// 12 : 주식담보대출
    /// 14 : 대여상환
    /// 21 : 자기융자신규
    /// 22 : 유통대주신규
    /// 23 : 유통융자신규
    /// 24 : 자기대주신규
    /// 25 : 자기융자상환
    /// 26 : 유통대주상환
    /// 27 : 유통융자상환
    /// 28 : 자기대주상환
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    /// 대출일자 (String, 선택)
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 예약주문종료일자 (String, 선택)
    /// (YYYYMMDD) 현재 일자보다 이후로 설정해야 함
    /// * RSVN_ORD_END_DT(예약주문종료일자)를 안 넣으면 다음날 주문처리되고 예약주문은 종료됨
    /// * RSVN_ORD_END_DT(예약주문종료일자)는 익영업일부터 달력일 기준으로 공휴일 포함하여 최대 30일이 되는 일자까지 입력 가능
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 대여일자 (String, 선택)
    #[serde(rename = "LDNG_DT")]
    pub ldng_dt: String,
}

/// [주식예약주문정정취소] 요청 구조체
/// [국내주식] 주문/계좌
/// 주식예약주문정정취소[v1_국내주식-018,019]
/// nrml_prcs_yn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    /// [정정/취소] 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// [정정/취소] 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드(6자리) (String, 필수)
    /// [정정]
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    /// [정정] 주문주식수
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가 (String, 필수)
    /// [정정] 1주당 가격
    /// * 장전 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 매도매수구분코드 (String, 필수)
    /// [정정]
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    /// [정정]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 05 : 장전 시간외
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 주문대상잔고구분코드 (String, 필수)
    /// [정정]
    /// 10 : 현금
    /// 12 : 주식담보대출
    /// 14 : 대여상환
    /// 21 : 자기융자신규
    /// 22 : 유통대주신규
    /// 23 : 유통융자신규
    /// 24 : 자기대주신규
    /// 25 : 자기융자상환
    /// 26 : 유통대주상환
    /// 27 : 유통융자상환
    /// 28 : 자기대주상환
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    /// 대출일자 (String, 선택)
    /// [정정]
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 예약주문종료일자 (String, 선택)
    /// [정정]
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 연락전화번호 (String, 선택)
    /// [정정]
    #[serde(rename = "CTAL_TLNO")]
    pub ctal_tlno: String,
    /// 예약주문순번 (String, 필수)
    /// [정정/취소]
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// 예약주문조직번호 (String, 선택)
    /// [정정/취소]
    #[serde(rename = "RSVN_ORD_ORGNO")]
    pub rsvn_ord_orgno: String,
    /// 예약주문주문일자 (String, 선택)
    /// [정정/취소]
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
}

/// [주식예약주문조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvCcnlRequest {
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
    /// "00" 입력
    #[serde(rename = "TMNL_MDIA_KIND_CD")]
    pub tmnl_mdia_kind_cd: String,
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 처리구분코드 (String, 필수)
    /// 0: 전체
    /// 1: 처리내역
    /// 2: 미처리내역
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
    /// 취소여부 (String, 필수)
    /// "Y" 유효한 주문만 조회
    #[serde(rename = "CNCL_YN")]
    pub cncl_yn: String,
    /// 상품번호 (String, 필수)
    /// 종목코드(6자리) (공백 입력 시 전체 조회)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (String, 필수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 다음 페이지 조회시 사용
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 다음 페이지 조회시 사용
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [퇴직연금 체결기준잔고] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePresentBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 사용자구분코드 (String, 필수)
    /// 00
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 처리구분코드 (String, 선택)
    /// 00 : 보유 주식 전체 조회
    /// 01 : 보유 주식 중 0주 주식 숨김
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
}

/// [퇴직연금 미체결내역] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldV2Request {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 사용자구분코드 (String, 필수)
    /// %%
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// 매도매수구분코드 (String, 필수)
    /// 00 : 전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분 (String, 필수)
    /// %% : 전체 / 01 : 체결 / 02 : 미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 조회구분3 (String, 필수)
    /// 00 : 전체
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [퇴직연금 매수가능조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderV2Request {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 적립금구분코드 (String, 필수)
    /// 00
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// CMA평가금액포함여부 (String, 필수)
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 주문구분 (String, 필수)
    /// 00 : 지정가 / 01 : 시장가
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
}

/// [퇴직연금 예수금조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDepositRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 적립금구분코드 (String, 필수)
    /// 00
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
}

/// [퇴직연금 잔고조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceV2Request {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 적립금구분코드 (String, 필수)
    /// 00
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// 조회구분 (String, 필수)
    /// 00 : 전체
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [주식잔고조회_실현손익] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceRlzPlRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시간외단일가여부 (String, 필수)
    /// 'N : 기본값
    /// Y : 시간외단일가'
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// 오프라인여부 (String, 필수)
    /// 공란
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// 조회구분 (String, 필수)
    /// 00 : 전체
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 단가구분 (String, 필수)
    /// 01 : 기본값
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// 펀드결제포함여부 (String, 필수)
    /// N : 포함하지 않음
    /// Y : 포함
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// 융자금액자동상환여부 (String, 필수)
    /// N : 기본값
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// PRCS_DVSN (String, 필수)
    /// 00 : 전일매매포함
    /// 01 : 전일매매미포함
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// 비용포함여부 (String, 필수)
    #[serde(rename = "COST_ICLD_YN")]
    pub cost_icld_yn: String,
    /// 연속조회검색조건100 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [투자계좌자산현황조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAccountBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회구분1 (String, 필수)
    /// 공백입력
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 기준가이전일자적용여부 (String, 필수)
    /// 공백입력
    #[serde(rename = "BSPR_BF_DT_APLY_YN")]
    pub bspr_bf_dt_aply_yn: String,
}

/// [기간별손익일별합산조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodProfitRequest {
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
    /// ""공란입력 시, 전체
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 조회종료일자 (String, 필수)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 정렬구분 (String, 필수)
    /// 00: 최근 순, 01: 과거 순, 02: 최근 순
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// 조회구분 (String, 필수)
    /// 00 입력
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 잔고구분 (String, 필수)
    /// 00: 전체
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [기간별매매손익현황조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodTradeProfitRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 정렬구분 (String, 필수)
    /// 00: 최근 순, 01: 과거 순, 02: 최근 순
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// ""공란입력 시, 전체
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
    /// 00: 전체
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [주식통합증거금 현황] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntgrMarginRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// CMA평가금액포함여부 (String, 필수)
    /// N 입력
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 원화외화구분코드 (String, 필수)
    /// 01(외화기준),02(원화기준)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 선도환계약외화구분코드 (String, 필수)
    /// 01(외화기준),02(원화기준)
    #[serde(rename = "FWEX_CTRT_FRCR_DVSN_CD")]
    pub fwex_ctrt_frcr_dvsn_cd: String,
}

/// [기간별계좌권리현황조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PeriodRightsRequest {
    /// 조회구분 (String, 필수)
    /// 03 입력
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 고객실명확인번호25 (String, 필수)
    /// 공란
    #[serde(rename = "CUST_RNCNO25")]
    pub cust_rncno25: String,
    /// 홈넷ID (String, 필수)
    /// 공란
    #[serde(rename = "HMID")]
    pub hmid: String,
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 8자리 입력 (ex.12345678)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 상품계좌번호 2자리 입력(ex. 01 or 22)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    /// 조회시작일자(YYYYMMDD)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// 조회종료일자(YYYYMMDD)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 권리유형코드 (String, 필수)
    /// 공란
    #[serde(rename = "RGHT_TYPE_CD")]
    pub rght_type_cd: String,
    /// 상품번호 (String, 필수)
    /// 공란
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// 공란
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 연속조회키100 (String, 필수)
    /// 다음조회시 입력
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 연속조회검색조건100 (String, 필수)
    /// 다음조회시 입력
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [주식현재가 시세] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자) // ETN은 종목코드 6자리 앞에 Q 입력 필수
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 시세2] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePrice2Request {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 000660
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 체결] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 일자별] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyPriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기간 분류 코드 (String, 필수)
    /// 'D : (일)최근 30거래일
    /// W : (주)최근 30주
    /// M : (월)최근 30개월'
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 수정주가 원주가 가격 (String, 필수)
    /// '0 : 수정주가미반영
    /// 1 : 수정주가반영
    /// * 수정주가는 액면분할/액면병합 등 권리 발생 시 과거 시세를 현재 주가에 맞게 보정한 가격'
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
}

/// [주식현재가 호가/예상체결] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceExpCcnRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 투자자] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireInvestorRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J : KRX, NX : NXT, UN : 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 회원사] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireMemberRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내주식기간별시세(일/주/월/년)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyItemchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜 1 (String, 필수)
    /// 조회 시작일자
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 날짜 2 (String, 필수)
    /// 조회 종료일자 (최대 100개)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 기간분류코드 (String, 필수)
    /// D:일봉 W:주봉, M:월봉, Y:년봉
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 수정주가 원주가 가격 여부 (String, 필수)
    /// 0:수정주가 1:원주가
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
}

/// [주식당일분봉조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeItemchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1 (String, 필수)
    /// 입력시간
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 과거 데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// 기타 구분 코드 (String, 필수)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [주식일별분봉조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeDailychartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1 (String, 필수)
    /// 입력 시간(ex 13시 130000)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 날짜1 (String, 필수)
    /// 입력 날짜(20241023)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 과거 데이터 포함 여부 (String, 필수)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// 허봉 포함 여부 (String, 선택)
    /// 공백 필수 입력
    #[serde(rename = "FID_FAKE_TICK_INCU_YN")]
    pub fid_fake_tick_incu_yn: String,
}

/// [주식현재가 당일시간대별체결] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeItemconclusionRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1 (String, 필수)
    /// 입력시간
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [주식현재가 시간외일자별주가] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyOvertimepriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J : 주식, ETF, ETN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 시간외시간별체결] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeOvertimeconclusionRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J : 주식, ETF, ETN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간 구분 코드 (String, 필수)
    /// 1 : 시간외 (Default)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
}

/// [국내주식 시간외현재가] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireOvertimePriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내주식 시간외호가] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireOvertimeAskingPriceRequest {
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 장마감 예상체결가] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpClosingPriceRequest {
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:전체, 1:상한가마감예상, 2:하한가마감예상, 3:직전대비상승률상위 ,4:직전대비하락률상위
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(11173)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 소속 구분 코드 (String, 필수)
    /// 0:전체, 1:종가범위연장
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ETF/ETN 현재가] 요청 구조체
/// [국내주식] 기본시세
/// ETF/ETN 현재가[v1_국내주식-068]
/// stck_prpr
/// prdy_vrss_sign
/// prdy_vrss
/// prdy_ctrt
/// acml_vol
/// prdy_vol
/// stck_mxpr
/// stck_llam
/// stck_prdy_clpr
/// stck_oprc
/// prdy_clpr_vrss_oprc_rate
/// stck_hgpr
/// prdy_clpr_vrss_hgpr_rate
/// stck_lwpr
/// prdy_clpr_vrss_lwpr_rate
/// prdy_last_nav
/// nav_prdy_vrss
/// nav_prdy_vrss_sign
/// nav_prdy_ctrt
/// trc_errt
/// stck_sdpr
/// stck_sspr
/// nmix_ctrt
/// etf_crcl_stcn
/// etf_ntas_ttam
/// etf_frcr_ntas_ttam
/// frgn_limt_rate
/// frgn_oder_able_qty
/// etf_cu_unit_scrt_cnt
/// etf_cnfg_issu_cnt
/// etf_dvdn_cycl
/// etf_crcl_ntas_ttam
/// etf_frcr_crcl_ntas_ttam
/// etf_frcr_last_ntas_wrth_val
/// lp_oder_able_cls_code
/// stck_dryy_hgpr
/// dryy_hgpr_vrss_prpr_rate
/// dryy_hgpr_date
/// stck_dryy_lwpr
/// dryy_lwpr_vrss_prpr_rate
/// dryy_lwpr_date
/// bstp_kor_isnm
/// vi_cls_code
/// lstn_stcn
/// frgn_hldn_qty
/// frgn_hldn_qty_rate
/// etf_trc_ert_mltp
/// mbcr_name
/// stck_lstn_date
/// mtrt_date
/// shrg_type_code
/// lp_hldn_rate
/// etf_trgt_nmix_bstp_code
/// etf_div_name
/// etf_rprs_bstp_kor_isnm
/// lp_hldn_vol
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceV2Request {
    /// FID 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [ETF 구성종목시세] 요청 구조체
/// [국내주식] 기본시세
/// ETF 구성종목시세[국내주식-073]
/// stck_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// etf_cnfg_issu_avls
/// nav_prdy_vrss_sign
/// nav_prdy_vrss
/// nav_prdy_ctrt
/// etf_ntas_ttam
/// prdy_clpr_nav
/// oprc_nav
/// hprc_nav
/// lprc_nav
/// etf_cu_unit_scrt_cnt
/// etf_cnfg_issu_cnt
/// stck_shrn_iscd
/// hts_kor_isnm
/// stck_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// acml_tr_pbmn
/// tday_rsfl_rate
/// prdy_vrss_vol
/// tr_pbmn_tnrt
/// hts_avls
/// etf_cnfg_issu_avls
/// etf_cnfg_issu_rlim
/// etf_vltn_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireComponentStockPriceRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드 (String, 필수)
    /// Unique key( 11216 )
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
}

/// [NAV 비교추이(종목)] 요청 구조체
/// [국내주식] 기본시세
/// NAV 비교추이(종목)[v1_국내주식-069]
/// stck_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// acml_tr_pbmn
/// stck_prdy_clpr
/// stck_oprc
/// stck_hgpr
/// stck_lwpr
/// stck_mxpr
/// stck_llam
/// nav_prdy_vrss_sign
/// nav_prdy_vrss
/// nav_prdy_ctrt
/// prdy_clpr_nav
/// oprc_nav
/// hprc_nav
/// lprc_nav
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NavComparisonTrendRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [NAV 비교추이(일)] 요청 구조체
/// [국내주식] 기본시세
/// NAV 비교추이(일)[v1_국내주식-071]
/// stck_bsop_date
/// stck_clpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// cntg_vol
/// nav_vrss_prpr
/// nav_prdy_vrss_sign
/// nav_prdy_vrss
/// nav_prdy_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NavComparisonDailyTrendRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J 입력
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목코드 (6자리)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    /// 조회 시작일자 (ex. 20240101)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2 (String, 필수)
    /// 조회 종료일자 (ex. 20240220)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
}

/// [NAV 비교추이(분)] 요청 구조체
/// [국내주식] 기본시세
/// NAV 비교추이(분)[v1_국내주식-070]
/// bsop_hour
/// nav_prdy_vrss_sign
/// nav_prdy_vrss
/// nav_prdy_ctrt
/// nav_vrss_prpr
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NavComparisonTimeTrendRequest {
    /// FID 시간 구분 코드 (String, 필수)
    /// 1분 :60, 3분: 180 … 120분:7200
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// E - 고정값
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [ELW 현재가 시세] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireElwPriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// W
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목번호 (6자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 신규상장종목] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 신규상장종목 [국내주식-181]
/// stck_lstn_date
/// elw_kor_isnm
/// elw_shrn_iscd
/// unas_isnm
/// pblc_co_name
/// lstn_stcn
/// stck_last_tr_date
/// elw_ko_barrier
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewlyListedRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Unique key(11548)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 분류구분코드 (String, 필수)
    /// 전체(02), 콜(00), 풋(01)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    /// 'ex) 000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 입력종목코드2 (String, 필수)
    /// '00003(한국투자증권), 00017(KB증권),
    /// 00005(미래에셋증권)'
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 입력날짜1 (String, 필수)
    /// 날짜 (ex) 20240402)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 결재방법 (String, 필수)
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNC_CLS_CODE")]
    pub fid_blnc_cls_code: String,
}

/// [ELW 민감도 순위] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 민감도 순위[국내주식-170]
/// elw_shrn_iscd
/// elw_kor_isnm
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// hts_thpr
/// delta_val
/// hts_ints_vltl
/// d90_hist_vltl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SensitivityRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Unique key(20285)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 입력종목코드 (String, 필수)
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 콜풋구분코드 (String, 필수)
    /// 0(전체), 1(콜), 2(풋)
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
    /// '0(이론가), 1(델타), 2(감마), 3(로), 4(베가) , 5(로)
    /// , 6(내재변동성), 7(90일변동성)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 잔존일수(이상) (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 조회기준일 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 결재방법 (String, 필수)
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 기초자산별 종목시세] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 기초자산별 종목시세 [국내주식-186]
/// elw_shrn_iscd
/// hts_kor_isnm
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// prls_qryr_stpr_prc
/// hts_rmnn_dynu
/// hts_ints_vltl
/// stck_cnvr_rate
/// lp_hvol
/// lp_rlim
/// lvrg_val
/// delta_val
/// prls_qryr_rate
/// invl_val
/// tmvl_val
/// hts_thpr
/// stck_lstn_date
/// stck_last_tr_date
/// lp_ntby_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UdrlAssetPriceRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분(W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Uniquekey(11541)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 시장구분코드 (String, 필수)
    /// 전체(A),콜(C),풋(P)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 입력종목코드 (String, 필수)
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 거래량수 (String, 필수)
    /// 전일거래량(정수량미만)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상제외구분코드 (String, 필수)
    /// 거래불가종목제외(0:미체크,1:체크)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력가격1 (String, 필수)
    /// 가격~원이상
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력가격2 (String, 필수)
    /// 가격~월이하
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 입력거래량1 (String, 필수)
    /// 거래량~계약이상
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 입력거래량2 (String, 필수)
    /// 거래량~계약이하
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 입력잔존일수1 (String, 필수)
    /// 잔존일(~일이상)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 입력잔존일수2 (String, 필수)
    /// 잔존일(~일이하)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_2")]
    pub fid_input_rmnn_dynu_2: String,
    /// 옵션 (String, 필수)
    /// 옵션상태(0:없음,1:ATM,2:ITM,3:OTM)
    #[serde(rename = "FID_OPTION")]
    pub fid_option: String,
    /// 입력옵션1 (String, 필수)
    #[serde(rename = "FID_INPUT_OPTION_1")]
    pub fid_input_option_1: String,
    /// 입력옵션2 (String, 필수)
    #[serde(rename = "FID_INPUT_OPTION_2")]
    pub fid_input_option_2: String,
}

/// [ELW 종목검색] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 종목검색 [국내주식-166]
/// bond_shrn_iscd
/// hts_kor_isnm
/// rght_type_name
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// stck_cnvr_rate
/// stck_lstn_date
/// stck_last_tr_date
/// hts_rmnn_dynu
/// unas_isnm
/// unas_prpr
/// unas_prdy_vrss
/// unas_prdy_vrss_sign
/// unas_prdy_ctrt
/// unas_acml_vol
/// moneyness
/// atm_cls_name
/// delta_val
/// hts_ints_vltl
/// tmvl_val
/// lvrg_val
/// prls_qryr_rate
/// lstn_stcn
/// pblc_co_name
/// lp_mbcr_name
/// lp_hldn_rate
/// elw_rght_form
/// elw_ko_barrier
/// apprch_rate
/// unas_shrn_iscd
/// mtrt_date
/// prmm_val
/// stck_lp_fin_date
/// tick_conv_prc
/// prls_qryr_stpr_prc
/// lp_hvol
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CondSearchRequest {
    /// 조건시장분류코드 (String, 필수)
    /// ELW(W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// 화면번호(11510)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 순위정렬구분코드 (String, 필수)
    /// '정렬1정렬안함(0)종목코드(1)현재가(2)대비율(3)거래량(4)행사가격(5)
    /// 전환비율(6)상장일(7)만기일(8)잔존일수(9)레버리지(10)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력수1 (String, 필수)
    /// 정렬1기준 - 상위(1)하위(2)
    #[serde(rename = "FID_INPUT_CNT_1")]
    pub fid_input_cnt_1: String,
    /// 순위정렬구분코드2 (String, 필수)
    /// 정렬2
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_2")]
    pub fid_rank_sort_cls_code_2: String,
    /// 입력수2 (String, 필수)
    /// 정렬2기준 - 상위(1)하위(2)
    #[serde(rename = "FID_INPUT_CNT_2")]
    pub fid_input_cnt_2: String,
    /// 순위정렬구분코드3 (String, 필수)
    /// 정렬3
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_3")]
    pub fid_rank_sort_cls_code_3: String,
    /// 입력수3 (String, 필수)
    /// 정렬3기준 - 상위(1)하위(2)
    #[serde(rename = "FID_INPUT_CNT_3")]
    pub fid_input_cnt_3: String,
    /// 대상구분코드 (String, 필수)
    /// 0:발행회사종목코드,1:기초자산종목코드,2:FID시장구분코드,3:FID입력날짜1(상장일),
    /// 4:FID입력날짜2(만기일),5:LP회원사종목코드,6:행사가기초자산비교>=(1) <=(2),
    /// 7:잔존일 이상 이하, 8:현재가, 9:전일대비율, 10:거래량, 11:최종거래일, 12:레버리지
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 입력종목코드 (String, 필수)
    /// 발행사종목코드전체(00000)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기초자산입력종목코드 (String, 필수)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 시장구분코드 (String, 필수)
    /// 권리유형전체(A)콜(CO)풋(PO)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 입력날짜1 (String, 필수)
    /// 상장일전체(0)금일(1)7일이하(2)8~30일(3)31~90일(4)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    /// 만기일전체(0)1개월(1)1~2(2)2~3(3)3~6(4)6~9(5)9~12(6)12이상(7)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 입력종목코드2 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 기타구분코드 (String, 필수)
    /// 행사가전체(0)>=(1)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 입력잔존일수1 (String, 필수)
    /// 잔존일이상
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 입력잔존일수2 (String, 필수)
    /// 잔존일이하
    #[serde(rename = "FID_INPUT_RMNN_DYNU_2")]
    pub fid_input_rmnn_dynu_2: String,
    /// 현재가수1 (String, 필수)
    /// 현재가이상
    #[serde(rename = "FID_PRPR_CNT1")]
    pub fid_prpr_cnt1: String,
    /// 현재가수2 (String, 필수)
    /// 현재가이하
    #[serde(rename = "FID_PRPR_CNT2")]
    pub fid_prpr_cnt2: String,
    /// 등락비율1 (String, 필수)
    /// 전일대비율이상
    #[serde(rename = "FID_RSFL_RATE1")]
    pub fid_rsfl_rate1: String,
    /// 등락비율2 (String, 필수)
    /// 전일대비율이하
    #[serde(rename = "FID_RSFL_RATE2")]
    pub fid_rsfl_rate2: String,
    /// 거래량1 (String, 필수)
    /// 거래량이상
    #[serde(rename = "FID_VOL1")]
    pub fid_vol1: String,
    /// 거래량2 (String, 필수)
    /// 거래량이하
    #[serde(rename = "FID_VOL2")]
    pub fid_vol2: String,
    /// 적용범위가격1 (String, 필수)
    /// 최종거래일from
    #[serde(rename = "FID_APLY_RANG_PRC_1")]
    pub fid_aply_rang_prc_1: String,
    /// 적용범위가격2 (String, 필수)
    /// 최종거래일to
    #[serde(rename = "FID_APLY_RANG_PRC_2")]
    pub fid_aply_rang_prc_2: String,
    /// 레버리지값1 (String, 필수)
    #[serde(rename = "FID_LVRG_VAL1")]
    pub fid_lvrg_val1: String,
    /// 레버리지값2 (String, 필수)
    #[serde(rename = "FID_LVRG_VAL2")]
    pub fid_lvrg_val2: String,
    /// 거래량3 (String, 필수)
    /// LP종료일from
    #[serde(rename = "FID_VOL3")]
    pub fid_vol3: String,
    /// 거래량4 (String, 필수)
    /// LP종료일to
    #[serde(rename = "FID_VOL4")]
    pub fid_vol4: String,
    /// 내재변동성1 (String, 필수)
    /// 내재변동성이상
    #[serde(rename = "FID_INTS_VLTL1")]
    pub fid_ints_vltl1: String,
    /// 내재변동성2 (String, 필수)
    /// 내재변동성이하
    #[serde(rename = "FID_INTS_VLTL2")]
    pub fid_ints_vltl2: String,
    /// 프리미엄값1 (String, 필수)
    /// 프리미엄이상
    #[serde(rename = "FID_PRMM_VAL1")]
    pub fid_prmm_val1: String,
    /// 프리미엄값2 (String, 필수)
    /// 프리미엄이하
    #[serde(rename = "FID_PRMM_VAL2")]
    pub fid_prmm_val2: String,
    /// 기어링1 (String, 필수)
    /// 기어링이상
    #[serde(rename = "FID_GEAR1")]
    pub fid_gear1: String,
    /// 기어링2 (String, 필수)
    /// 기어링이하
    #[serde(rename = "FID_GEAR2")]
    pub fid_gear2: String,
    /// 손익분기비율1 (String, 필수)
    /// 손익분기이상
    #[serde(rename = "FID_PRLS_QRYR_RATE1")]
    pub fid_prls_qryr_rate1: String,
    /// 손익분기비율2 (String, 필수)
    /// 손익분기이하
    #[serde(rename = "FID_PRLS_QRYR_RATE2")]
    pub fid_prls_qryr_rate2: String,
    /// 델타1 (String, 필수)
    /// 델타이상
    #[serde(rename = "FID_DELTA1")]
    pub fid_delta1: String,
    /// 델타2 (String, 필수)
    /// 델타이하
    #[serde(rename = "FID_DELTA2")]
    pub fid_delta2: String,
    /// 행사가1 (String, 필수)
    #[serde(rename = "FID_ACPR1")]
    pub fid_acpr1: String,
    /// 행사가2 (String, 필수)
    #[serde(rename = "FID_ACPR2")]
    pub fid_acpr2: String,
    /// 주식전환비율1 (String, 필수)
    /// 전환비율이상
    #[serde(rename = "FID_STCK_CNVR_RATE1")]
    pub fid_stck_cnvr_rate1: String,
    /// 주식전환비율2 (String, 필수)
    /// 전환비율이하
    #[serde(rename = "FID_STCK_CNVR_RATE2")]
    pub fid_stck_cnvr_rate2: String,
    /// 분류구분코드 (String, 필수)
    /// 0:전체,1:일반,2:조기종료
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 패리티1 (String, 필수)
    /// 패리티이상
    #[serde(rename = "FID_PRIT1")]
    pub fid_prit1: String,
    /// 패리티2 (String, 필수)
    /// 패리티이하
    #[serde(rename = "FID_PRIT2")]
    pub fid_prit2: String,
    /// 자본지지점1 (String, 필수)
    /// 배리어이상
    #[serde(rename = "FID_CFP1")]
    pub fid_cfp1: String,
    /// 자본지지점2 (String, 필수)
    /// 배리어이하
    #[serde(rename = "FID_CFP2")]
    pub fid_cfp2: String,
    /// 지수가격1 (String, 필수)
    /// LP보유비율이상
    #[serde(rename = "FID_INPUT_NMIX_PRICE_1")]
    pub fid_input_nmix_price_1: String,
    /// 지수가격2 (String, 필수)
    /// LP보유비율이하
    #[serde(rename = "FID_INPUT_NMIX_PRICE_2")]
    pub fid_input_nmix_price_2: String,
    /// E기어링값1 (String, 필수)
    /// 접근도이상
    #[serde(rename = "FID_EGEA_VAL1")]
    pub fid_egea_val1: String,
    /// E기어링값2 (String, 필수)
    /// 접근도이하
    #[serde(rename = "FID_EGEA_VAL2")]
    pub fid_egea_val2: String,
    /// 배당수익율 (String, 필수)
    /// 손익분기점이상
    #[serde(rename = "FID_INPUT_DVDN_ERT")]
    pub fid_input_dvdn_ert: String,
    /// 역사적변동성 (String, 필수)
    /// 손익분기점이하
    #[serde(rename = "FID_INPUT_HIST_VLTL")]
    pub fid_input_hist_vltl: String,
    /// 세타1 (String, 필수)
    /// MONEYNESS이상
    #[serde(rename = "FID_THETA1")]
    pub fid_theta1: String,
    /// 세타2 (String, 필수)
    /// MONEYNESS이하
    #[serde(rename = "FID_THETA2")]
    pub fid_theta2: String,
}

/// [ELW 당일급변종목] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 당일급변종목[국내주식-171]
/// elw_shrn_iscd
/// elw_kor_isnm
/// elw_prpr
/// prdy_vrss_sign
/// prdy_vrss
/// prdy_ctrt
/// total_askp_rsqn
/// total_bidp_rsqn
/// acml_vol
/// stnd_val
/// stnd_val_vrss
/// stnd_val_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct QuickChangeRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Unique key(20287)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사 (String, 필수)
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장구분코드 (String, 필수)
    /// Unique key(A)
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
    /// 1(분), 2(일)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 입력 일 또는 분 (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 기준시간(분 선택 시) (String, 필수)
    #[serde(rename = "FID_INPUT_HOUR_2")]
    pub fid_input_hour_2: String,
    /// 순위정렬구분코드 (String, 필수)
    /// '1(가격급등), 2(가격급락), 3(거래량급증)
    /// , 4(매수잔량급증), 5(매도잔량급증)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 결재방법 (String, 필수)
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 기초자산 목록조회] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 기초자산 목록조회 [국내주식-185]
/// unas_shrn_iscd
/// unas_isnm
/// unas_prpr
/// unas_prdy_vrss
/// unas_prdy_vrss_sign
/// unas_prdy_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UdrlAssetListRequest {
    /// 조건화면분류코드 (String, 필수)
    /// 11541(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 순위정렬구분코드 (String, 필수)
    /// 0(종목명순), 1(콜발행종목순), 2(풋발행종목순), 3(전일대비 상승율순), 4(전일대비 하락율순), 5(현재가 크기순), 6(종목코드순)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력종목코드 (String, 필수)
    /// 00000(전체), 00003(한국투자증권), 00017(KB증권), 00005(미래에셋)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 비교대상종목조회] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 비교대상종목조회 [국내주식-183]
/// elw_shrn_iscd
/// elw_kor_isnm
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompareStocksRequest {
    /// 조건화면분류코드 (String, 필수)
    /// 11517(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 종목코드(ex)005930(삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW LP매매추이] 요청 구조체
/// [국내주식] ELW 시세
/// ELW LP매매추이 [국내주식-182]
/// elw_prpr
/// prdy_vrss_sign
/// prdy_vrss
/// prdy_ctrt
/// acml_vol
/// prdy_vol
/// stck_cnvr_rate
/// lvrg_val
/// prls_qryr_rate
/// invl_val
/// tmvl_val
/// elw_ko_barrier
/// stck_bsop_date
/// elw_prpr
/// prdy_vrss_sign
/// prdy_vrss
/// prdy_ctrt
/// lp_seln_qty
/// lp_seln_avrg_unpr
/// lp_shnu_qty
/// lp_shnu_avrg_unpr
/// lp_hvol
/// lp_hldn_rate
/// prsn_deal_qty
/// apprch_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct LpTradeTrendRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분(W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 입력종목코드(ex 52K577(미래 K577KOSDAQ150콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 투자지표추이(체결)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 투자지표추이(체결) [국내주식-172]
/// stck_cntg_hour
/// elw_prpr
/// prdy_vrss_sign
/// prdy_vrss
/// prdy_ctrt
/// acml_vol
/// lvrg_val
/// tmvl_val
/// invl_val
/// apprch_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorTrendCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 투자지표추이(분별)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 투자지표추이(분별) [국내주식-174]
/// stck_bsop_date
/// stck_cntg_hour
/// elw_prpr
/// elw_oprc
/// elw_hgpr
/// elw_lwpr
/// lvrg_val
/// prmm_val
/// invl_val
/// acml_vol
/// cntg_vol
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorTrendMinuteRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간구분코드 (String, 필수)
    /// '60(1분), 180(3분), 300(5분), 600(10분), 1800(30분), 3600(60분), 7200(60분)
    /// '
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거데이터 포함 여부 (String, 필수)
    /// N(과거데이터포함X),Y(과거데이터포함O)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [ELW 투자지표추이(일별)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 투자지표추이(일별) [국내주식-173]
/// stck_bsop_date
/// elw_prpr
/// prdy_vrss_sign
/// prdy_vrss
/// prdy_ctrt
/// acml_vol
/// lvrg_val
/// tmvl_val
/// invl_val
/// elw_oprc
/// elw_hgpr
/// elw_lwpr
/// apprch_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorTrendDailyRequest {
    /// 시장분류코드 (String, 필수)
    /// W
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종콕코드 (String, 필수)
    /// ex. 57K281
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(틱)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 변동성 추이(틱) [국내주식-180]
/// bsop_date
/// stck_cntg_hour
/// elw_prpr
/// hts_ints_vltl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendTickRequest {
    /// 조건시장분류코드 (String, 필수)
    /// W(Unique key)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성추이(체결)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 변동성추이(체결) [국내주식-177]
/// stck_cntg_hour
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// hts_ints_vltl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    /// W(Unique key)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(일별)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 변동성 추이(일별) [국내주식-178]
/// stck_bsop_date
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// elw_oprc
/// elw_hgpr
/// elw_lwpr
/// acml_vol
/// d10_hist_vltl
/// d20_hist_vltl
/// d30_hist_vltl
/// d60_hist_vltl
/// d90_hist_vltl
/// hts_ints_vltl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendDailyRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 민감도 추이(체결)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 민감도 추이(체결) [국내주식-175]
/// stck_cntg_hour
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// hts_thpr
/// delta_val
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SensitivityTrendCcnlRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(분별)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 변동성 추이(분별) [국내주식-179]
/// stck_bsop_date
/// stck_cntg_hour
/// stck_prpr
/// elw_oprc
/// elw_hgpr
/// elw_lwpr
/// hts_ints_vltl
/// hist_vltl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendMinuteRequest {
    /// 조건시장분류코드 (String, 필수)
    /// W(Unique key)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간구분코드 (String, 필수)
    /// '60(1분), 180(3분), 300(5분), 600(10분), 1800(30분), 3600(60분)
    /// '
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거데이터 포함 여부 (String, 필수)
    /// N(과거데이터포함X),Y(과거데이터포함O)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [ELW 민감도 추이(일별)] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 민감도 추이(일별) [국내주식-176]
/// stck_bsop_date
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// hts_thpr
/// delta_val
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SensitivityTrendDailyRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// ex)(58J438(KBJ438삼성전자풋)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 만기예정/만기종목] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 만기예정/만기종목 [국내주식-184]
/// elw_shrn_iscd
/// elw_kor_isnm
/// unas_isnm
/// unas_prpr
/// stck_cnvr_rate
/// elw_prpr
/// stck_lstn_date
/// stck_last_tr_date
/// total_rdmp_amt
/// rdmp_amt
/// lstn_stcn
/// lp_hvol
/// ccls_paym_prc
/// mtrt_vltn_amt
/// evnt_prd_fin_date
/// stlm_date
/// pblc_prc
/// unas_shrn_iscd
/// stnd_iscd
/// rdmp_ask_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpirationStocksRequest {
    /// 조건시장분류코드 (String, 필수)
    /// W 입력
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// 11547 입력
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력날짜1 (String, 필수)
    /// 입력날짜 ~ (ex) 20240402)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    /// ~입력날짜 (ex) 20240408)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 분류구분코드 (String, 필수)
    /// 0(콜),1(풋),2(전체)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 기타구분코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    /// 000000(전체), 2001(KOSPI 200), 기초자산코드(종목코드 ex. 삼성전자-005930)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행회사코드 (String, 필수)
    /// 00000(전체), 00003(한국투자증권), 00017(KB증권), 00005(미래에셋증권)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 결제방법 (String, 필수)
    /// 0(전체),1(일반),2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 입력옵션1 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_INPUT_OPTION_1")]
    pub fid_input_option_1: String,
}

/// [ELW 지표순위] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 지표순위[국내주식-169]
/// elw_shrn_iscd
/// elw_kor_isnm
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// stck_cnvr_rate
/// lvrg_val
/// tmvl_val
/// invl_val
/// elw_ko_barrier
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Unique key(20279)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사 (String, 필수)
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 콜풋구분코드 (String, 필수)
    /// 0(전체), 1(콜), 2(풋)
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
    /// 0(전환비율), 1(레버리지), 2(행사가 ), 3(내재가치), 4(시간가치)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 결재방법 (String, 필수)
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 상승률순위] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 상승률순위[국내주식-167]
/// hts_kor_isnm
/// elw_shrn_iscd
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// stck_sdpr
/// sdpr_vrss_prpr_sign
/// sdpr_vrss_prpr
/// sdpr_vrss_prpr_rate
/// stck_oprc
/// oprc_vrss_prpr_sign
/// oprc_vrss_prpr
/// oprc_vrss_prpr_rate
/// stck_hgpr
/// stck_lwpr
/// prd_rsfl_sign
/// prd_rsfl
/// prd_rsfl_rate
/// stck_cnvr_rate
/// hts_rmnn_dynu
/// unas_isnm
/// unas_shrn_iscd
/// lp_hldn_rate
/// prls_qryr_stpr_prc
/// delta_val
/// prls_qryr_rate
/// stck_lstn_date
/// stck_last_tr_date
/// hts_ints_vltl
/// lvrg_val
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UpdownRateRequest {
    /// 사용자권한정보 (String, 필수)
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 거래소코드 (String, 필수)
    /// Unique key(20277)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 상승율/하락율 구분 (String, 필수)
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// N일자값 (String, 필수)
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 거래량조건 (String, 필수)
    /// '0(전체), 1(1개월이하), 2(1개월~2개월),
    /// 3(2개월~3개월), 4(3개월~6개월),
    /// 5(6개월~9개월),6(9개월~12개월), 7(12개월이상)'
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// NEXT KEY BUFF (String, 필수)
    /// 0(전체), 1(콜), 2(풋)
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
    /// '0(상승율), 1(하락율), 2(시가대비상승율)
    /// , 3(시가대비하락율), 4(변동율)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 사용자권한정보 (String, 필수)
    /// 0(전체)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 거래소코드 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [ELW 거래량순위] 요청 구조체
/// [국내주식] ELW 시세
/// ELW 거래량순위[국내주식-168]
/// elw_kor_isnm
/// elw_shrn_iscd
/// elw_prpr
/// prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// lstn_stcn
/// acml_vol
/// n_prdy_vol
/// n_prdy_vol_vrss
/// vol_inrt
/// vol_tnrt
/// nday_vol_tnrt
/// acml_tr_pbmn
/// n_prdy_tr_pbmn
/// n_prdy_tr_pbmn_vrss
/// total_askp_rsqn
/// total_bidp_rsqn
/// ntsl_rsqn
/// ntby_rsqn
/// seln_rsqn_rate
/// shnu_rsqn_rate
/// stck_cnvr_rate
/// hts_rmnn_dynu
/// invl_val
/// tmvl_val
/// lp_mbcr_name
/// unas_isnm
/// stck_last_tr_date
/// unas_shrn_iscd
/// prdy_vol
/// lp_hldn_rate
/// prls_qryr_stpr_prc
/// delta_val
/// prls_qryr_rate
/// stck_lstn_date
/// hts_ints_vltl
/// lvrg_val
/// lp_ntby_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumeRankRequest {
    /// 조건시장분류코드 (String, 필수)
    /// W
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// 20278
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드 (String, 필수)
    /// 000000
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사 (String, 필수)
    /// 00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력잔존일수 (String, 필수)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 콜풋구분코드 (String, 필수)
    /// 0(전체), 1(콜), 2(풋)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상) (String, 필수)
    /// 거래가격1(이상)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하) (String, 필수)
    /// 거래가격1(이하)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상) (String, 필수)
    /// 거래량1(이상)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하) (String, 필수)
    /// 거래량1(이하)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 조회기준일 (String, 필수)
    /// 입력날짜(기준가 조회기준)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 순위정렬구분코드 (String, 필수)
    /// 0: 거래량순 1: 평균거래증가율 2: 평균거래회전율 3:거래금액순 4: 순매수잔량순 5: 순매도잔량순
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 소속구분코드 (String, 필수)
    /// 0: 전체
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// LP발행사 (String, 필수)
    /// 0000
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 만기일-최종거래일조회 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [국내업종 현재지수] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexPriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// 업종(U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 코스피(0001), 코스닥(1001), 코스피200(2001)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내업종 일자별지수] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexDailyPriceRequest {
    /// FID 기간 분류 코드 (String, 필수)
    /// 일/주/월 구분코드 ( D:일별 , W:주별, M:월별 )
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 코스피(0001), 코스닥(1001), 코스피200(2001)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    /// 입력 날짜(ex. 20240223)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내업종 시간별지수(초)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexTickpriceRequest {
    /// 입력 종목코드 (String, 필수)
    /// 0001:거래소, 1001:코스닥, 2001:코스피200, 3003:KSQ150
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내업종 시간별지수(분)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexTimepriceRequest {
    /// ?입력 시간1 (String, 필수)
    /// 초단위, 60(1분), 300(5분), 600(10분)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 종목코드 (String, 필수)
    /// 0001:거래소, 1001:코스닥, 2001:코스피200, 3003:KSQ150
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [업종 분봉조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeIndexchartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// U
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 기타 구분 코드 (String, 필수)
    /// 0: 기본 1:장마감,시간외 제외
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 0001 : 종합
    /// 0002 : 대형주
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 시간1 (String, 필수)
    /// 30, 60 -> 1분, 600-> 10분, 3600 -> 1시간
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// FID 과거 데이터 포함 여부 (String, 필수)
    /// Y (과거) / N (당일)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [국내주식업종기간별시세(일/주/월/년)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyIndexchartpriceRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 업종 : U
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 업종 상세코드 (String, 필수)
    /// '0001 : 종합
    /// 0002 : 대형주
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회 시작일자 (String, 필수)
    /// 조회 시작일자 (ex. 20220501)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 조회 종료일자 (String, 필수)
    /// 조회 종료일자 (ex. 20220530)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// ' 기간분류코드' (String, 필수)
    /// ' D:일봉 W:주봉, M:월봉, Y:년봉'
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [국내업종 구분별전체시세] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexCategoryPriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 코스피(0001), 코스닥(1001), 코스피200(2001)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20214 )
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 시장 구분 코드 (String, 필수)
    /// 시장구분코드(K:거래소, Q:코스닥, K2:코스피200)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// FID 소속 구분 코드 (String, 필수)
    /// 시장구분코드에 따라 아래와 같이 입력
    /// 시장구분코드(K:거래소) 0:전업종, 1:기타구분, 2:자본금구분 3:상업별구분
    /// 시장구분코드(Q:코스닥) 0:전업종, 1:기타구분, 2:벤처구분 3:일반구분
    /// 시장구분코드(K2:코스닥) 0:전업종
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [국내주식 예상체결지수 추이] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpIndexTrendRequest {
    /// 장운영 구분 코드 (String, 필수)
    /// 1: 장시작전, 2: 장마감
    #[serde(rename = "FID_MKOP_CLS_CODE")]
    pub fid_mkop_cls_code: String,
    /// 입력 시간1 (String, 필수)
    /// 10(10초), 30(30초), 60(1분), 600(10분)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:코스피, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 예상체결 전체지수] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpTotalIndexRequest {
    /// 시장 구분 코드 (String, 필수)
    /// 0:전체 K:거래소 Q:코스닥
    #[serde(rename = "fid_mrkt_cls_code")]
    pub fid_mrkt_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (업종 U)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(11175)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 장운영 구분 코드 (String, 필수)
    /// 1:장시작전, 2:장마감
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
}

/// [변동성완화장치(VI) 현황] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireViStatusRequest {
    /// FID 분류 구분 코드 (String, 필수)
    /// 0:전체 1:상승 2:하락
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// FID 조건 화면 분류 코드 (String, 필수)
    /// 20139
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 시장 구분 코드 (String, 필수)
    /// 0:전체 K:거래소 Q:코스닥
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// FID 입력 종목코드 (String, 필수)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 순위 정렬 구분 코드 (String, 필수)
    /// 0:전체1:정적2:동적3:정적&동적
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// FID 입력 날짜1 (String, 필수)
    /// 영업일
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 대상 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID 대상 제외 구분 코드 (String, 필수)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [금리 종합(국내채권/금리)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompInterestRequest {
    /// 조건시장분류코드 (String, 필수)
    /// Unique key(I)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Unique key(20702)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 분류구분코드 (String, 필수)
    /// 1: 해외금리지표
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 분류구분코드 (String, 필수)
    /// 공백 : 전체
    #[serde(rename = "FID_DIV_CLS_CODE1")]
    pub fid_div_cls_code1: String,
}

/// [종합 시황/공시(제목)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewsTitleRequest {
    /// 뉴스 제공 업체 코드 (String, 필수)
    /// 공백 필수 입력
    #[serde(rename = "FID_NEWS_OFER_ENTP_CODE")]
    pub fid_news_ofer_entp_code: String,
    /// 조건 시장 구분 코드 (String, 필수)
    /// 공백 필수 입력
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 공백: 전체, 종목코드 : 해당코드가 등록된 뉴스
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 제목 내용 (String, 필수)
    /// 공백 필수 입력
    #[serde(rename = "FID_TITL_CNTT")]
    pub fid_titl_cntt: String,
    /// 입력 날짜 (String, 필수)
    /// 공백: 현재기준, 조회일자(ex 00YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 시간 (String, 필수)
    /// 공백: 현재기준, 조회시간(ex 0000HHMMSS)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 공백 필수 입력
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 일련번호 (String, 필수)
    /// 공백 필수 입력
    #[serde(rename = "FID_INPUT_SRNO")]
    pub fid_input_srno: String,
}

/// [국내휴장일조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ChkHolidayRequest {
    /// 기준일자 (String, 필수)
    /// 기준일자(YYYYMMDD)
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 연속조회키 (String, 필수)
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// 연속조회검색조건 (String, 필수)
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
}

/// [상품기본조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchInfoRequest {
    /// 상품번호 (String, 필수)
    /// '주식(하이닉스) : 000660 (코드 : 300)
    /// 선물(101S12) : KR4101SC0009 (코드 : 301)
    /// 미국(AAPL) : AAPL (코드 : 512)'
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// '300 주식
    /// 301 선물옵션
    /// 302 채권
    /// 512 미국 나스닥 / 513 미국 뉴욕 / 529 미국 아멕스
    /// 515 일본
    /// 501 홍콩 / 543 홍콩CNY / 558 홍콩USD
    /// 507 베트남 하노이 / 508 베트남 호치민
    /// 551 중국 상해A / 552 중국 심천A'
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [주식기본조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchStockInfoRequest {
    /// 상품유형코드 (String, 필수)
    /// 300: 주식, ETF, ETN, ELW
    /// 301 : 선물옵션
    /// 302 : 채권
    /// 306 : ELS'
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [국내주식 대차대조표] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BalanceSheetRequest {
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 손익계산서] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IncomeStatementRequest {
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    ///
    /// ※ 분기데이터는 연단위 누적합산
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 재무비율] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FinancialRatioRequest {
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 수익성비율] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProfitRatioRequest {
    /// 입력 종목코드 (String, 필수)
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 기타주요비율] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OtherMajorRatiosRequest {
    /// 입력 종목코드 (String, 필수)
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 안정성비율] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct StabilityRatioRequest {
    /// 입력 종목코드 (String, 필수)
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 성장성비율] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct GrowthRatioRequest {
    /// 입력 종목코드 (String, 필수)
    /// ex : 000660
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 년, 1: 분기
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 당사 신용가능종목] 요청 구조체
/// [국내주식] 종목정보
/// 국내주식 당사 신용가능종목[국내주식-111]
/// stck_shrn_iscd
/// hts_kor_isnm
/// crdt_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CreditByCompanyRequest {
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:코드순, 1:이름순
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 선택 여부 (String, 필수)
    /// 0:신용주문가능, 1: 신용주문불가
    #[serde(rename = "fid_slct_yn")]
    pub fid_slct_yn: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20477)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [예탁원정보(배당일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DividendRequest {
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회구분 (String, 필수)
    /// 0:배당전체, 1:결산배당, 2:중간배당
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 고배당여부 (String, 필수)
    /// 공백
    #[serde(rename = "HIGH_GB")]
    pub high_gb: String,
}

/// [예탁원정보(주식매수청구일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PurreqRequest {
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(합병/분할일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MergerSplitRequest {
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(액면교체일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct RevSplitRequest {
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 시장구분 (String, 필수)
    /// 0:전체, 1:코스피, 2:코스닥
    #[serde(rename = "MARKET_GB")]
    pub market_gb: String,
}

/// [예탁원정보(자본감소일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CapDcrsRequest {
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(상장정보일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ListInfoRequest {
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(공모주청약일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PubOfferRequest {
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
}

/// [예탁원정보(실권주일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ForfeitRequest {
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(의무예치일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MandDepositRequest {
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(유상증자일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PaidinCapinRequest {
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회구분 (String, 필수)
    /// 1(청약일별), 2(기준일별)
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백(전체), 특정종목 조회시(종목코드)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(무상증자일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BonusIssueRequest {
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(주주총회일정)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SharehldMeetRequest {
    /// CTS (String, 필수)
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From (String, 필수)
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To (String, 필수)
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [국내주식 종목추정실적] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EstimatePerformRequest {
    /// 종목코드 (String, 필수)
    /// ex) 265520
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [당사 대주가능 종목] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct LendableByCompanyRequest {
    /// 거래소구분코드 (String, 필수)
    /// 00(전체), 02(거래소), 03(코스닥)
    #[serde(rename = "EXCG_DVSN_CD")]
    pub excg_dvsn_cd: String,
    /// 상품번호 (String, 필수)
    /// 공백 : 전체조회, 종목코드 입력 시 해당종목만 조회
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 당사대주가능여부 (String, 필수)
    /// Y
    #[serde(rename = "THCO_STLN_PSBL_YN")]
    pub thco_stln_psbl_yn: String,
    /// 조회구분1 (String, 필수)
    /// 0 : 전체조회, 1: 종목코드순 정렬
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 미입력 (다음조회 불가)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키100 (String, 필수)
    /// 미입력 (다음조회 불가)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [국내주식 종목투자의견] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestOpinionRequest {
    /// 조건시장분류코드 (String, 필수)
    /// J(시장 구분 코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// 16633(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 종목코드(ex) 005930(삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력날짜1 (String, 필수)
    /// 이후 ~(ex) 0020231113)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    /// ~ 이전(ex) 0020240513)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [국내주식 증권사별 투자의견] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestOpbysecRequest {
    /// 조건시장분류코드 (String, 필수)
    /// J(시장 구분 코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// 16634(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 회원사코드 (kis developers 포탈 사이트 포럼-> FAQ -> 종목정보 다운로드(국내) 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류구분코드 (String, 필수)
    /// 전체(0) 매수(1) 중립(2) 매도(3)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력날짜1 (String, 필수)
    /// 이후 ~
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    /// ~ 이전
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [종목조건검색 목록조회] 요청 구조체
/// [국내주식] 시세분석
/// 종목조건검색 목록조회[국내주식-038]
/// user_id
/// grp_nm
/// condition_nm
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PsearchTitleRequest {
    /// 사용자 HTS ID (String, 필수)
    #[serde(rename = "user_id")]
    pub user_id: String,
}

/// [종목조건검색조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PsearchResultRequest {
    /// 사용자 HTS ID (String, 필수)
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 사용자조건 키값 (String, 필수)
    /// 종목조건검색 목록조회 API의 output인 'seq'을 이용
    /// (0 부터 시작)
    #[serde(rename = "seq")]
    pub seq: String,
}

/// [관심종목 그룹조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntstockGrouplistRequest {
    /// 관심종목구분코드 (String, 필수)
    /// Unique key(1)
    #[serde(rename = "TYPE")]
    pub r#type: String,
    /// FID 기타 구분 코드 (String, 필수)
    /// Unique key(00)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 사용자 ID (String, 필수)
    /// HTS_ID 입력
    #[serde(rename = "USER_ID")]
    pub user_id: String,
}

/// [관심종목(멀티종목) 시세조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntstockMultpriceRequest {
    /// 조건 시장 분류 코드1 (String, 필수)
    /// 그룹별종목조회 결과 fid_mrkt_cls_code(시장구분) 1 입력
    /// J: KRX, NX: NXT, UN: 통합
    /// ex) J
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_1")]
    pub fid_cond_mrkt_div_code_1: String,
    /// 입력 종목코드1 (String, 필수)
    /// 그룹별종목조회 결과 jong_code(종목코드) 1 입력
    /// ex) 005930
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
    /// 입력 종목코드18 (String, 필수)
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

/// [관심종목 그룹별 종목조회] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntstockStocklistByGroupRequest {
    /// 관심종목구분코드 (String, 필수)
    /// Unique key(1)
    #[serde(rename = "TYPE")]
    pub r#type: String,
    /// 사용자 ID (String, 필수)
    /// HTS_ID 입력
    #[serde(rename = "USER_ID")]
    pub user_id: String,
    /// 데이터 순위 (String, 필수)
    /// 공백
    #[serde(rename = "DATA_RANK")]
    pub data_rank: String,
    /// 관심 그룹 코드 (String, 필수)
    /// 관심그룹 조회 결과의 그룹 값 입력
    #[serde(rename = "INTER_GRP_CODE")]
    pub inter_grp_code: String,
    /// 관심 그룹 명 (String, 필수)
    /// 공백
    #[serde(rename = "INTER_GRP_NAME")]
    pub inter_grp_name: String,
    /// HTS 한글 종목명 (String, 필수)
    /// 공백
    #[serde(rename = "HTS_KOR_ISNM")]
    pub hts_kor_isnm: String,
    /// 체결 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "CNTG_CLS_CODE")]
    pub cntg_cls_code: String,
    /// 기타 구분 코드 (String, 필수)
    /// Unique key(4)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [국내기관_외국인 매매종목가집계] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ForeignInstitutionTotalRequest {
    /// 시장 분류 코드 (String, 필수)
    /// V(Default)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// 16449(Default)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:코스피, 1001:코스닥
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 수량정열, 1: 금액정열
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0: 순매수상위, 1: 순매도상위
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 기타 구분 정렬 (String, 필수)
    /// 0:전체 1:외국인 2:기관계 3:기타
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [외국계 매매종목 가집계] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FrgnmemTradeEstimateRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분코드 (J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Uniquekey (16441)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 0000(전체), 1001(코스피), 2001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위정렬구분코드 (String, 필수)
    /// 0(금액순), 1(수량순)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 순위정렬구분코드2 (String, 필수)
    /// 0(매수순), 1(매도순)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_2")]
    pub fid_rank_sort_cls_code_2: String,
}

/// [종목별 투자자매매동향(일별)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorTradeByStockDailyRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목번호 (6자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    /// 입력 날짜(20250812) (해당일 조회는 장 종료 후 정상 조회 가능)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 수정주가 원주가 가격 (String, 필수)
    /// 공란 입력
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
    /// 기타 구분 코드 (String, 필수)
    /// "1" 입력
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [시장별 투자자매매동향(시세)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireInvestorTimeByMarketRequest {
    /// 시장구분 (String, 필수)
    /// 코스피: KSP, 코스닥:KSQ,
    /// 선물,콜옵션,풋옵션 : K2I, 주식선물:999,
    /// ETF: ETF, ELW:ELW, ETN: ETN,
    /// 미니: MKI, 위클리월 : WKM, 위클리목: WKI
    /// 코스닥150: KQI
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 업종구분 (String, 필수)
    /// - fid_input_iscd: KSP(코스피) 혹은 KSQ(코스닥)인 경우
    /// 코스피(0001_종합, .…0027_제조업 )
    /// 코스닥(1001_종합, …. 1041_IT부품)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    ///
    /// - fid_input_iscd가 K2I인 경우
    /// F001(선물)
    /// OC01(콜옵션)
    /// OP01(풋옵션)
    ///
    /// - fid_input_iscd가 999인 경우
    /// S001(주식선물)
    ///
    /// - fid_input_iscd가 ETF인 경우
    /// T000(ETF)
    ///
    /// - fid_input_iscd가 ELW인 경우
    /// W000(ELW)
    ///
    /// - fid_input_iscd가 ETN인 경우
    /// E199(ETN)
    ///
    /// - fid_input_iscd가 MKI인 경우
    /// F004(미니선물)
    /// OC02(미니콜옵션)
    /// OP02(미니풋옵션)
    ///
    /// - fid_input_iscd가 WKM인 경우
    /// OC05(위클리콜(월))
    /// OP05(위클리풋(월))
    ///
    /// - fid_input_iscd가 WKI인 경우
    /// OC04(위클리콜(목))
    /// OP04(위클리풋(목))
    ///
    /// - fid_input_iscd가 KQI인 경우
    /// F002(코스닥150선물)
    /// OC03(코스닥150콜옵션)
    /// OP03(코스닥150풋옵션)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
}

/// [시장별 투자자매매동향(일별)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireInvestorDailyByMarketRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 코스피, 코스닥 : 업종분류코드 (종목정보파일 - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    /// ex. 20240517
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 종목코드 (String, 필수)
    /// 코스피(KSP), 코스닥(KSQ)
    #[serde(rename = "FID_INPUT_ISCD_1")]
    pub fid_input_iscd_1: String,
    /// 입력 날짜2 (String, 필수)
    /// 입력 날짜1과 동일날짜 입력
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 하위 분류코드 (String, 필수)
    /// 코스피, 코스닥 : 업종분류코드 (종목정보파일 - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
}

/// [종목별 외국계 순매수추이] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FrgnmemPchsTrendRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 종목코드(ex) 005930(삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드 (String, 필수)
    /// 외국계 전체(99999)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 시장구분코드 (String, 필수)
    /// J (KRX만 지원)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [회원사 실시간 매매동향(틱)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FrgnmemTradeTrendRequest {
    /// 화면분류코드 (String, 필수)
    /// 20432(primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J 고정 입력
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드 (String, 필수)
    /// ex. 005930(삼성전자)
    ///
    /// ※ FID_INPUT_ISCD(종목코드) 혹은 FID_MRKT_CLS_CODE(시장구분코드) 둘 중 하나만 입력
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 회원사코드 (String, 필수)
    /// ex. 99999(전체)
    ///
    /// ※ 회원사코드 (kis developers 포탈 사이트 포럼-> FAQ -> 종목정보 다운로드(국내) 참조)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 시장구분코드 (String, 필수)
    /// A(전체),K(코스피), Q(코스닥), K2(코스피200), W(ELW)
    ///
    /// ※ FID_INPUT_ISCD(종목코드) 혹은 FID_MRKT_CLS_CODE(시장구분코드) 둘 중 하나만 입력
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 거래량 (String, 필수)
    /// 거래량 ~
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
}

/// [주식현재가 회원사 종목매매동향] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireMemberDailyRequest {
    /// 조건시장분류코드 (String, 필수)
    /// J: KRX, NX: NXT, UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 주식종목코드입력
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 회원사코드 (String, 필수)
    /// 회원사코드 (kis developers 포탈 사이트 포럼-> FAQ -> 종목정보 다운로드(국내) > 회원사 참조)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 입력날짜1 (String, 필수)
    /// 날짜 ~
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2 (String, 필수)
    /// ~ 날짜
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 구간구분코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_SCTN_CLS_CODE")]
    pub fid_sctn_cls_code: String,
}

/// [종목별 프로그램매매추이(체결)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProgramTradeByStockRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// KRX : J , NXT : NX, 통합 : UN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [종목별 프로그램매매추이(일별)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProgramTradeByStockDailyRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// KRX : J , NXT : NX, 통합 : UN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    /// 기준일 (ex 0020240308), 미입력시 당일부터 조회
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [종목별 외인기관 추정가집계] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorTrendEstimateRequest {
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
}

/// [종목별일별매수매도체결량] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyTradeVolumeRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// J: KRX, NX: NXT, UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 005930
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    /// from
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2 (String, 필수)
    /// to
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// FID 기간 분류 코드 (String, 필수)
    /// D
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [프로그램매매 종합현황(시간)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompProgramTradeTodayRequest {
    /// 시장 분류 코드 (String, 필수)
    /// KRX : J , NXT : NX, 통합 : UN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    /// K:코스피, Q:코스닥
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 구간 구분 코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_SCTN_CLS_CODE")]
    pub fid_sctn_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장 분류코드1 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_COND_MRKT_DIV_CODE1")]
    pub fid_cond_mrkt_div_code1: String,
    /// 입력 시간1 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [프로그램매매 종합현황(일별)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompProgramTradeDailyRequest {
    /// 시장 분류 코드 (String, 필수)
    /// J : KRX, NX : NXT, UN : 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    /// K:코스피, Q:코스닥
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 검색시작일 (String, 필수)
    /// 공백 입력, 입력 시 ~ 입력일자까지 조회됨
    /// * 8개월 이상 과거 조회 불가
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 검색종료일 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [프로그램매매 투자자매매동향(당일)] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorProgramTradeTodayRequest {
    /// 거래소 구분 코드 (String, 필수)
    /// J : KRX, NX : NXT, UN : 통합
    #[serde(rename = "EXCH_DIV_CLS_CODE")]
    pub exch_div_cls_code: String,
    /// 시장 구분 코드 (String, 필수)
    /// 1:코스피, 4:코스닥
    #[serde(rename = "MRKT_DIV_CLS_CODE")]
    pub mrkt_div_cls_code: String,
}

/// [국내주식 신용잔고 일별추이] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyCreditBalanceRequest {
    /// 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 화면 분류 코드 (String, 필수)
    /// Unique key(20476)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 종목코드 (String, 필수)
    /// 종목코드 (ex 005930)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 결제일자 (String, 필수)
    /// 결제일자 (ex 20240313)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 예상체결가 추이] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpPriceTrendRequest {
    /// 장운영 구분 코드 (String, 필수)
    /// 0:전체, 4:체결량 0 제외
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드(ex. 005930)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 공매도 일별추이] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyShortSaleRequest {
    /// 입력 날짜2 (String, 필수)
    /// ~ 누적
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1 (String, 필수)
    /// 공백시 전체 (기간 ~)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 시간외예상체결등락률] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OvertimeExpTransFluctRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J: 주식)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(11186)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000(전체), 0001(코스피), 1001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0(상승률), 1(상승폭), 2(보합), 3(하락률), 4(하락폭)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// '0(전체), 1(관리종목), 2(투자주의), 3(투자경고),
    /// 4(투자위험예고), 5(투자위험), 6(보통주), 7(우선주)'
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 입력 거래량 (String, 필수)
    /// 거래량 ~
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
}

/// [국내주식 체결금액별 매매비중] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradprtByamtRequest {
    /// 조건시장분류코드 (String, 필수)
    /// J: KRX, NX: NXT, UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// Uniquekey(11119)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 종목코드(ex)(005930 (삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내 증시자금 종합] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MktfundsRequest {
    /// 입력날짜1 (String, 필수)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [종목별 일별 대차거래추이] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyLoanTransRequest {
    /// 조회구분 (String, 필수)
    /// 1(코스피), 2(코스닥), 3(종목)
    #[serde(rename = "MRKT_DIV_CLS_CODE")]
    pub mrkt_div_cls_code: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
    /// 조회시작일시 (String, 필수)
    /// 조회기간 ~
    #[serde(rename = "START_DATE")]
    pub start_date: String,
    /// 조회종료일시 (String, 필수)
    /// ~ 조회기간
    #[serde(rename = "END_DATE")]
    pub end_date: String,
    /// 이전조회KEY (String, 필수)
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [국내주식 상하한가 포착] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CaptureUplowpriceRequest {
    /// 조건시장분류코드 (String, 필수)
    /// 시장구분(J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드 (String, 필수)
    /// 11300(Unique key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 상하한가 구분코드 (String, 필수)
    /// 0(상한가),1(하한가)
    #[serde(rename = "FID_PRC_CLS_CODE")]
    pub fid_prc_cls_code: String,
    /// 분류구분코드 (String, 필수)
    /// '0(상하한가종목),6(8%상하한가 근접), 5(10%상하한가 근접), 1(15%상하한가 근접),2(20%상하한가 근접),
    /// 3(25%상하한가 근접)'
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력종목코드 (String, 필수)
    /// 전체(0000), 코스피(0001),코스닥(1001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 대상구분코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상제외구분코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력가격1 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력가격2 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량수 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
}

/// [국내주식 매물대/거래비중] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PbarTratioRequest {
    /// 조건시장분류코드 (String, 필수)
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 주식단축종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드 (String, 필수)
    /// Uniquekey(20113)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력시간1 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [거래량순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumeRankV2Request {
    /// 조건 시장 분류 코드 (String, 필수)
    /// J:KRX, NX:NXT
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// 20171
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000(전체) 기타(업종코드)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0(전체) 1(보통주) 2(우선주)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    /// 0 : 평균거래량 1:거래증가율 2:평균거래회전율 3:거래금액순 4:평균거래금액회전율
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    /// 1 or 0 9자리 (차례대로 증거금 30% 40% 50% 60% 100% 신용보증금 30% 40% 50% 60%)
    /// ex) "111111111"
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 1 or 0 10자리 (차례대로 투자위험/경고/주의 관리종목 정리매매 불성실공시 우선주 거래정지 ETF ETN 신용주문불가 SPAC)
    /// ex) "0000000000"
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 가격 ~
    /// ex) "0"
    ///
    /// 전체 가격 대상 조회 시 FID_INPUT_PRICE_1, FID_INPUT_PRICE_2 모두 ""(공란) 입력
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// ~ 가격
    /// ex) "1000000"
    ///
    /// 전체 가격 대상 조회 시 FID_INPUT_PRICE_1, FID_INPUT_PRICE_2 모두 ""(공란) 입력
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 거래량 ~
    /// ex) "100000"
    ///
    /// 전체 거래량 대상 조회 시 FID_VOL_CNT ""(공란) 입력
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 입력 날짜1 (String, 필수)
    /// ""(공란) 입력
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 등락률 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FluctuationRequest {
    /// 등락 비율2 (String, 필수)
    /// 공백 입력 시 전체 (~ 비율
    #[serde(rename = "fid_rsfl_rate2")]
    pub fid_rsfl_rate2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20170 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000(전체) 코스피(0001), 코스닥(1001), 코스피200(2001)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:상승율순 1:하락율순 2:시가대비상승율 3:시가대비하락율 4:변동율
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 수1 (String, 필수)
    /// 0:전체 , 누적일수 입력
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// 가격 구분 코드 (String, 필수)
    /// 'fid_rank_sort_cls_code :0 상승율 순일때 (0:저가대비, 1:종가대비)
    /// fid_rank_sort_cls_code :1 하락율 순일때 (0:고가대비, 1:종가대비)
    /// fid_rank_sort_cls_code : 기타 (0:전체)'
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 공백 입력 시 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 공백 입력 시 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 공백 입력 시 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 대상 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 등락 비율1 (String, 필수)
    /// 공백 입력 시 전체 (비율 ~)
    #[serde(rename = "fid_rsfl_rate1")]
    pub fid_rsfl_rate1: String,
}

/// [국내주식 호가잔량 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct QuoteBalanceRequest {
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20172 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000(전체) 코스피(0001), 코스닥(1001), 코스피200(2001)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0: 순매수잔량순, 1:순매도잔량순, 2:매수비율순, 3:매도비율순
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 수익자산지표 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProfitAssetIndexRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20173 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 옵션1 (String, 필수)
    /// 회계연도 (2023)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2 (String, 필수)
    /// 0: 1/4분기 , 1: 반기, 2: 3/4분기, 3: 결산
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:매출이익 1:영업이익 2:경상이익 3:당기순이익 4:자산총계 5:부채총계 6:자본총계
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시가총액 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketCapRequest {
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20174 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 전체, 1:보통주, 2:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
}

/// [국내주식 재무비율 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FinanceRatioRequest {
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20175 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 옵션1 (String, 필수)
    /// 회계년도 입력 (ex 2023)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2 (String, 필수)
    /// 0: 1/4분기 , 1: 반기, 2: 3/4분기, 3: 결산
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 7: 수익성 분석, 11 : 안정성 분석, 15: 성장성 분석, 20: 활동성 분석
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    /// 0
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시간외잔량 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AfterHourBalanceRequest {
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20176 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 1: 장전 시간외, 2: 장후 시간외, 3:매도잔량, 4:매수잔량
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 우선주/괴리율 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PreferDisparateRatioRequest {
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20177 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 이격도 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisparityRequest {
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20178 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보톧주, 7:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0: 이격도상위순, 1:이격도하위순
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 시간 구분 코드 (String, 필수)
    /// 5:이격도5, 10:이격도10, 20:이격도20, 60:이격도60, 120:이격도120
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
}

/// [국내주식 시장가치 순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketValueRequest {
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20179 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보톧주, 7:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 입력 옵션1 (String, 필수)
    /// 회계연도 입력 (ex 2023)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2 (String, 필수)
    /// 0: 1/4분기 , 1: 반기, 2: 3/4분기, 3: 결산
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// '가치분석(23:PER, 24:PBR, 25:PCR, 26:PSR, 27: EPS, 28:EVA,
    /// 29: EBITDA, 30: EV/EBITDA, 31:EBITDA/금융비율'
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 체결강도 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumePowerRequest {
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key( 20168 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 전체, 1: 보통주 2: 우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
}

/// [국내주식 관심종목등록 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TopInterestStockRequest {
    /// 입력 필수값2 (String, 필수)
    /// 000000 : 필수입력값
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20180)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 업종 코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 분류 구분 코드 (String, 필수)
    /// 0: 전체 1: 관리종목 2: 투자주의 3: 투자경고 4: 투자위험예고 5: 투자위험 6: 보통주 7: 우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 입력값 (String, 필수)
    /// 순위검색 입력값(1: 1위부터, 10:10위부터)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
}

/// [국내주식 예상체결 상승/하락상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpTransUpdownRequest {
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:상승률1:상승폭2:보합3:하락율4:하락폭5:체결량6:거래대금
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20182)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체 1:보통주 2:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 적용 범위 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
    /// 거래대금 (String, 필수)
    /// 입력값 없을때 전체 (거래대금 ~) 천원단위
    #[serde(rename = "fid_pbmn")]
    pub fid_pbmn: String,
    /// 소속 구분 코드 (String, 필수)
    /// 0: 전체
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 장운영 구분 코드 (String, 필수)
    /// 0:장전예상1:장마감예상
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
}

/// [국내주식 당사매매종목 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradedByCompanyRequest {
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0: 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20186)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보통주, 7:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:매도상위,1:매수상위
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 날짜1 (String, 필수)
    /// 기간~
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// 입력 날짜2 (String, 필수)
    /// ~기간
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0: 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 적용 범위 거래량 (String, 필수)
    /// 0: 전체, 100: 100주 이상
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: Decimal,
    /// 적용 범위 가격2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// 적용 범위 가격1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
}

/// [국내주식 신고/신저근접종목 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NearNewHighlowRequest {
    /// 적용 범위 거래량 (String, 필수)
    /// 0: 전체, 100: 100주 이상
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: Decimal,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20187)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체, 1:관리종목, 2:투자주의, 3:투자경고
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 수1 (String, 필수)
    /// 괴리율 최소
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// 입력 수2 (String, 필수)
    /// 괴리율 최대
    #[serde(rename = "fid_input_cnt_2")]
    pub fid_input_cnt_2: String,
    /// 가격 구분 코드 (String, 필수)
    /// 0:신고근접, 1:신저근접
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0: 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0:전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보통주, 7:우선주
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 적용 범위 가격1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 적용 범위 가격2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
}

/// [국내주식 배당률 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DividendRateRequest {
    /// CTS_AREA (String, 필수)
    /// 공백
    #[serde(rename = "CTS_AREA")]
    pub cts_area: String,
    /// KOSPI (String, 필수)
    /// 0:전체, 1:코스피, 2: 코스피200, 3: 코스닥,
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 업종구분 (String, 필수)
    /// '코스피(0001:종합, 0002:대형주.…0027:제조업 ),
    /// 코스닥(1001:종합, …. 1041:IT부품
    /// 코스피200 (2001:KOSPI200, 2007:KOSPI100, 2008:KOSPI50)'
    #[serde(rename = "UPJONG")]
    pub upjong: String,
    /// 종목선택 (String, 필수)
    /// 0:전체, 6:보통주, 7:우선주
    #[serde(rename = "GB2")]
    pub gb2: String,
    /// 배당구분 (String, 필수)
    /// 1:주식배당, 2: 현금배당
    #[serde(rename = "GB3")]
    pub gb3: String,
    /// 기준일From (String, 필수)
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 기준일To (String, 필수)
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 결산/중간배당 (String, 필수)
    /// 0:전체, 1:결산배당, 2:중간배당
    #[serde(rename = "GB4")]
    pub gb4: String,
}

/// [국내주식 대량체결건수 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BulkTransNumRequest {
    /// 적용 범위 가격2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(11909)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0:매수상위, 1:매도상위
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 건별금액 ~
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 적용 범위 가격1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 입력 종목코드2 (String, 필수)
    /// 공백:전체종목, 개별종목 조회시 종목코드 (000660)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 대상 구분 코드 (String, 필수)
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 거래량 수 (String, 필수)
    /// 거래량 ~
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: i64,
}

/// [국내주식 신용잔고 상위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CreditBalanceRequest {
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(11701)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200,
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 증가율기간 (String, 필수)
    /// 2~999
    #[serde(rename = "FID_OPTION")]
    pub fid_option: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// '(융자)0:잔고비율 상위, 1: 잔고수량 상위, 2: 잔고금액 상위, 3: 잔고비율 증가상위, 4: 잔고비율 감소상위
    /// (대주)5:잔고비율 상위, 6: 잔고수량 상위, 7: 잔고금액 상위, 8: 잔고비율 증가상위, 9: 잔고비율 감소상위 '
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
}

/// [국내주식 공매도 상위종목] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ShortSaleRequest {
    /// FID 적용 범위 거래량 (String, 필수)
    /// 공백
    #[serde(rename = "FID_APLY_RANG_VOL")]
    pub fid_aply_rang_vol: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20482)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000:전체, 0001:코스피, 1001:코스닥, 2001:코스피200, 4001: KRX100, 3003: 코스닥150
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회구분 (일/월) (String, 필수)
    /// 조회구분 (일/월) D: 일, M:월
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 조회가간(일수 (String, 필수)
    /// '조회가간(일수):
    /// 조회구분(D) 0:1일, 1:2일, 2:3일, 3:4일, 4:1주일, 9:2주일, 14:3주일,
    /// 조회구분(M) 1:1개월, 2:2개월, 3:3개월'
    #[serde(rename = "FID_INPUT_CNT_1")]
    pub fid_input_cnt_1: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// FID 대상 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID 적용 범위 가격1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "FID_APLY_RANG_PRC_1")]
    pub fid_aply_rang_prc_1: String,
    /// FID 적용 범위 가격2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "FID_APLY_RANG_PRC_2")]
    pub fid_aply_rang_prc_2: String,
}

/// [국내주식 시간외등락율순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OvertimeFluctuationRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J: 주식)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20234)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000(전체), 0001(코스피), 1001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드 (String, 필수)
    /// 1(상한가), 2(상승률), 3(보합),4(하한가),5(하락률)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 공백 입력
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시간외거래량순위] 요청 구조체
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OvertimeVolumeRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (J: 주식)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20235)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 0000(전체), 0001(코스피), 1001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드 (String, 필수)
    /// 0(매수잔량), 1(매도잔량), 2(거래량)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 가격1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수 (String, 필수)
    /// 거래량 ~
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 실시간체결가 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간체결가 (KRX) [실시간-003]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stcnt0Request {
    /// 거래ID (String, 필수)
    /// [실전/모의투자]
    /// H0STCNT0 : 실시간 주식 체결가
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간호가 (KRX) [실시간-004]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stasp0Request {
    /// 거래ID (String, 필수)
    /// [실전/모의투자]
    /// H0STASP0 : 주식호가
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결통보] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간체결통보 [실시간-005]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stcni0Request {
    /// 거래ID (String, 필수)
    /// '[실전/모의투자]
    /// H0STCNI0 : 국내주식 실시간체결통보
    /// H0STCNI9 : 모의투자 실시간 체결통보
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// HTS ID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간예상체결 (KRX) [실시간-041]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stanc0Request {
    /// 거래ID (String, 필수)
    /// H0STANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간회원사 (KRX) [실시간-047]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stmbc0Request {
    /// 거래ID (String, 필수)
    /// H0STMBC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간프로그램매매 (KRX) [실시간-048]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stpgm0Request {
    /// 거래ID (String, 필수)
    /// H0STPGM0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 장운영정보 (KRX) [실시간-049]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stmko0Request {
    /// 거래ID (String, 필수)
    /// H0STMKO0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간호가 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 시간외 실시간호가 (KRX) [실시간-025]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stoaa0Request {
    /// 거래ID (String, 필수)
    /// H0STOAA0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간체결가 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 시간외 실시간체결가 (KRX) [실시간-042]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stoup0Request {
    /// 거래ID (String, 필수)
    /// H0STOUP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간예상체결 (KRX)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 시간외 실시간예상체결 (KRX) [실시간-024]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stoac0Request {
    /// 거래ID (String, 필수)
    /// H0STOAC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간체결] 요청 구조체
/// [국내주식] 실시간시세
/// 국내지수 실시간체결 [실시간-026]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Upcnt0Request {
    /// 거래ID (String, 필수)
    /// H0UPCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 업종구분코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간예상체결] 요청 구조체
/// [국내주식] 실시간시세
/// 국내지수 실시간예상체결 [실시간-027]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Upanc0Request {
    /// 거래ID (String, 필수)
    /// H0UPANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 업종구분코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간프로그램매매] 요청 구조체
/// [국내주식] 실시간시세
/// 국내지수 실시간프로그램매매 [실시간-028]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Uppgm0Request {
    /// 거래ID (String, 필수)
    /// H0UPPGM0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 업종구분코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간호가] 요청 구조체
/// [국내주식] 실시간시세
/// ELW 실시간호가 [실시간-062]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ewasp0Request {
    /// 거래ID (String, 필수)
    /// H0EWASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// ELW 종목코드(ex. 57LA24)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간체결가] 요청 구조체
/// [국내주식] 실시간시세
/// ELW 실시간체결가 [실시간-061]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ewcnt0Request {
    /// 거래ID (String, 필수)
    /// H0EWCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// ELW 종목코드(ex. 57LA24)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간예상체결] 요청 구조체
/// [국내주식] 실시간시세
/// ELW 실시간예상체결 [실시간-063]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ewanc0Request {
    /// 거래ID (String, 필수)
    /// H0EWANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// ELW 종목코드(ex. 57LA24)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내ETF NAV추이] 요청 구조체
/// [국내주식] 실시간시세
/// 국내ETF NAV추이 [실시간-051]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stnav0Request {
    /// 거래ID (String, 필수)
    /// H0STNAV0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex. 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결가 (통합)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간체결가 (통합)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Uncnt0Request {
    /// 거래ID (String, 필수)
    /// H0UNCNT0 : 실시간 주식 체결가 통합
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (통합)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간호가 (통합)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unasp0Request {
    /// 거래ID (String, 필수)
    /// H0UNASP0 : 실시간 주식 체결가 통합
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (통합)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간예상체결 (통합)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unanc0Request {
    /// 거래ID (String, 필수)
    /// [실전투자]
    /// H0UNANC0 : 국내주식 실시간예상체결 (통합)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (통합)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간회원사 (통합)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unmbc0Request {
    /// 거래ID (String, 필수)
    /// H0UNMBC0 : 국내주식 주식종목회원사 (통합)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (통합)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간프로그램매매 (통합)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unpgm0Request {
    /// 거래ID (String, 필수)
    /// H0UNPGM0 : 실시간 주식종목프로그램매매 통합
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (통합)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 장운영정보 (통합)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unmko0Request {
    /// 거래ID (String, 필수)
    /// H0UNMKO0 : 국내주식 장운영정보 (통합)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결가 (NXT)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간체결가 (NXT)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxcnt0Request {
    /// 거래ID (String, 필수)
    /// H0NXCNT0 : 주식종목체결 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (NXT)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간호가 (NXT)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxasp0Request {
    /// 거래ID (String, 필수)
    /// H0NXASP0 : 실시간 주식 호가 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (NXT)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간예상체결 (NXT)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxanc0Request {
    /// 거래ID (String, 필수)
    /// H0NXANC0 : 국내주식 실시간예상체결 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (NXT)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간회원사 (NXT)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxmbc0Request {
    /// 거래ID (String, 필수)
    /// H0NXMBC0 : 국내주식 주식종목회원사 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (NXT)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 실시간프로그램매매 (NXT)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxpgm0Request {
    /// 거래ID (String, 필수)
    /// H0NXPGM0 : 실시간 주식프로그램매매 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (NXT)] 요청 구조체
/// [국내주식] 실시간시세
/// 국내주식 장운영정보 (NXT)
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxmko0Request {
    /// 거래ID (String, 필수)
    /// H0NXMKO0 : 국내주식 장운영정보 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [선물옵션 주문] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 주문[v1_국내선물-001]
/// ACNT_NAME
/// TRAD_DVSN_NAME
/// ITEM_NAME
/// ORD_TMD
/// ORD_GNO_BRNO
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRequest {
    /// 주문처리구분코드 (String, 필수)
    /// 02 : 주문전송
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 매도매수구분코드 (String, 필수)
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 단축상품번호 (String, 필수)
    /// 종목번호
    /// 선물 6자리 (예: A01603)
    /// 옵션 9자리 (예: B01603955)
    #[serde(rename = "SHTN_PDNO")]
    pub shtn_pdno: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문가격1 (String, 필수)
    /// 시장가나 최유리 지정가인 경우 0으로 입력
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드 (String, 선택)
    /// ※ ORD_DVSN_CD(주문구분코드)를 입력한 경우 ""(공란)으로 입력해도 됨
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// 한국거래소호가조건코드 (String, 선택)
    /// ※ ORD_DVSN_CD(주문구분코드)를 입력한 경우 ""(공란)으로 입력해도 됨
    /// 0 : 없음
    /// 3 : IOC
    /// 4 : FOK
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// 연락전화번호 (String, 선택)
    /// 고객의 연락 가능한 전화번호
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 선물옵션종목구분코드 (String, 선택)
    /// 공란(Default)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리,
    /// 10 : 지정가(IOC)
    /// 11 : 지정가(FOK)
    /// 12 : 시장가(IOC)
    /// 13 : 시장가(FOK)
    /// 14 : 최유리(IOC)
    /// 15 : 최유리(FOK)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [선물옵션 정정취소주문] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 정정취소주문[v1_국내선물-002]
/// ACNT_NAME
/// TRAD_DVSN_NAME
/// ITEM_NAME
/// ORD_TMD
/// ORD_GNO_BRNO
/// ORGN_ODNO
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclV2Request {
    /// 주문처리구분코드 (String, 필수)
    /// 02 : 주문전송
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 정정취소구분코드 (String, 필수)
    /// 01 : 정정
    /// 02 : 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 원주문번호 (String, 필수)
    /// 정정 혹은 취소할 주문의 번호
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문수량 (String, 필수)
    /// [Header tr_id TTTO1103U(선물옵션 정정취소 주간)]
    /// 전량일경우 0으로 입력
    ///
    /// [Header tr_id JTCE1002U(선물옵션 정정취소 야간)]
    /// 일부수량 정정 및 취소 불가, 주문수량 반드시 입력 (공백 불가)
    /// 일부 미체결 시 잔량 전체에 대해서 취소 가능
    /// EX) 2개 매수주문 후 1개 체결, 1개 미체결인 상태에서 취소주문 시 ORD_QTY는 1로 입력
    ///
    /// ※ 모의계좌의 경우, 주문수량 반드시 입력 (공백 불가)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문가격1 (String, 필수)
    /// 시장가나 최유리의 경우 0으로 입력 (취소 시에도 0 입력)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드 (String, 필수)
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// 한국거래소호가조건코드 (String, 필수)
    /// 취소시 0으로 입력
    /// 정정시
    /// 0 : 없음
    /// 3 : IOC
    /// 4 : FOK
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// 잔여수량여부 (String, 필수)
    /// Y : 전량
    /// N : 일부
    #[serde(rename = "RMN_QTY_YN")]
    pub rmn_qty_yn: String,
    /// 선물옵션종목구분코드 (String, 선택)
    /// [Header tr_id TTTO1103U(선물옵션 정정취소 주간)]
    /// 공란(Default)
    ///
    /// [Header tr_id JTCE1002U(선물옵션 정정취소 야간)]
    /// 01 : 선물
    /// 02 : 콜옵션
    /// 03 : 풋옵션
    /// 04 : 스프레드
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// 주문구분코드 (String, 필수)
    /// [정정]
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리,
    /// 10 : 지정가(IOC)
    /// 11 : 지정가(FOK)
    /// 12 : 시장가(IOC)
    /// 13 : 시장가(FOK)
    /// 14 : 최유리(IOC)
    /// 15 : 최유리(FOK)
    ///
    /// [취소]
    /// 01 로 입력
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [선물옵션 주문체결내역조회] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 주문체결내역조회[v1_국내선물-003]
/// ord_gno_brno
/// csac_name
/// acnt_prdt_cd
/// ord_dt
/// orgn_odno
/// sll_buy_dvsn_cd
/// trad_dvsn_name
/// nmpr_type_cd
/// nmpr_type_name
/// prdt_name
/// prdt_type_cd
/// ord_qty
/// ord_idx
/// ord_tmd
/// tot_ccld_qty
/// avg_idx
/// tot_ccld_amt
/// rjct_qty
/// ingr_trad_rjct_rson_cd
/// ingr_trad_rjct_rson_name
/// ord_stfno
/// sprd_item_yn
/// ord_ip_addr
/// tot_ord_qty
/// tot_ccld_amt_smtl
/// tot_ccld_qty_smtl
/// fee_smtl
/// ctac_tlno
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작주문일자 (String, 필수)
    /// 주문내역 조회 시작 일자, YYYYMMDD
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// 종료주문일자 (String, 필수)
    /// 주문내역 조회 마지막 일자, YYYYMMDD
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// 매도매수구분코드 (String, 필수)
    /// 00 : 전체
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분 (String, 필수)
    /// 00 : 전체
    /// 01 : 체결
    /// 02 : 미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 정렬순서 (String, 필수)
    /// AS : 정순
    /// DS : 역순
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 시작주문번호 (String, 필수)
    /// 조회 시작 번호 입력
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// 상품번호 (String, 필수)
    /// 공란 시, 전체 조회
    /// 선물 6자리 (예: 101S03)
    /// 옵션 9자리 (예: 201S03370)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 시장ID코드 (String, 필수)
    /// 공란(Default)
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 잔고현황] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 잔고현황[v1_국내선물-004]
/// acnt_prdt_cd
/// prdt_type_cd
/// shtn_pdno
/// prdt_name
/// sll_buy_dvsn_name
/// cblc_qty
/// excc_unpr
/// ccld_avg_unpr1
/// idx_clpr
/// pchs_amt
/// evlu_amt
/// evlu_pfls_amt
/// trad_pfls_amt
/// lqd_psbl_qty
/// dnca_cash
/// frcr_dncl_amt
/// dnca_sbst
/// tot_dncl_amt
/// tot_ccld_amt
/// cash_mgna
/// sbst_mgna
/// mgna_tota
/// opt_dfpa
/// thdt_dfpa
/// rnwl_dfpa
/// nxdy_dnca
/// nxdy_dncl_amt
/// prsm_dpast
/// prsm_dpast_amt
/// pprt_ord_psbl_cash
/// add_mgna_cash
/// add_mgna_tota
/// futr_trad_pfls_amt
/// opt_trad_pfls_amt
/// futr_evlu_pfls_amt
/// opt_evlu_pfls_amt
/// trad_pfls_amt_smtl
/// evlu_pfls_amt_smtl
/// wdrw_psbl_tot_amt
/// ord_psbl_cash
/// ord_psbl_sbst
/// ord_psbl_tota
/// pchs_amt_smtl
/// evlu_amt_smtl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceV3Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금 구분 (String, 필수)
    /// 01 : 개시
    /// 02 : 유지
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드 (String, 필수)
    /// 1 : 정산 (정산가격으로 잔고 조회)
    /// 2 : 본정산 (매입가격으로 잔고 조회)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 주문가능] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 주문가능[v1_국내선물-005]
/// tot_psbl_qty
/// lqd_psbl_qty1
/// ord_psbl_qty
/// bass_idx
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderV3Request {
    /// 종합계좌번호 (String, 선택)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 선택)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 선택)
    /// 선물옵션종목코드
    /// 선물 6자리 (예: 101S03)
    /// 옵션 9자리 (예: 201S03370)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (String, 선택)
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문가격1 (String, 선택)
    /// 주문가격
    /// ※ 주문가격 '0'일 경우
    /// - 옵션매수 : 현재가
    /// - 그 이외 : 기준가
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 주문구분코드 (String, 선택)
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리,
    /// 10 : 지정가(IOC)
    /// 11 : 지정가(FOK)
    /// 12 : 시장가(IOC)
    /// 13 : 시장가(FOK)
    /// 14 : 최유리(IOC)
    /// 15 : 최유리(FOK)
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [(야간)선물옵션 주문체결 내역조회] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// (야간)선물옵션 주문체결 내역조회 [국내선물-009]
/// tot_ord_qty
/// tot_ccld_qty
/// tot_ccld_qty_SMTL
/// tot_ccld_amt
/// tot_ccld_amt_SMTL
/// ctac_tlno
/// ord_gno_brno
/// csac_name
/// acnt_prdt_cd
/// ord_dt
/// orgn_odno
/// sll_buy_dvsn_cd
/// trad_dvsn_name
/// nmpr_type_name
/// prdt_name
/// prdt_type_cd
/// ord_qty
/// ord_idx4
/// ord_tmd
/// tot_ccld_qty
/// avg_idx
/// tot_ccld_amt
/// rjct_qty
/// ingr_trad_rjct_rson_cd
/// ingr_trad_rjct_rson_name
/// ord_stfno
/// sprd_item_yn
/// ord_ip_addr
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireNgtCcnlRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작주문일자 (String, 필수)
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// 종료주문일자 (String, 필수)
    /// 조회하려는 마지막 일자 다음일자로 조회
    /// (ex. 20221011 까지의 내역을 조회하고자 할 경우,
    /// 20221012로 종료주문일자 설정)
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// 매도매수구분코드 (String, 필수)
    /// 공란 : default (00: 전체 ,01 : 매도, 02 : 매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분 (String, 필수)
    /// 00 : 전체
    /// 01 : 체결
    /// 02 : 미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 정렬순서 (String, 필수)
    /// 공란 : default (DS : 정순, 그외 : 역순)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 시작주문번호 (String, 필수)
    /// 공란 : default
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// 상품번호 (String, 필수)
    /// 공란 : default
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 시장ID코드 (String, 필수)
    /// 공란 : default
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// 선물옵션구분코드 (String, 필수)
    /// 공란 : 전체, 01 : 선물, 02 : 옵션
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// 화면구분 (String, 필수)
    /// 02(Default)
    #[serde(rename = "SCRN_DVSN")]
    pub scrn_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [(야간)선물옵션 잔고현황] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// (야간)선물옵션 잔고현황 [국내선물-010]
/// dnca_cash
/// frcr_dncl_amt
/// dnca_sbst
/// tot_dncl_amt
/// cash_mgna
/// sbst_mgna
/// mgna_tota
/// opt_dfpa
/// thdt_dfpa
/// rnwl_dfpa
/// nxdy_dnca
/// nxdy_dncl_amt
/// prsm_dpast
/// pprt_ord_psbl_cash
/// add_mgna_cash
/// add_mgna_tota
/// futr_trad_pfls_amt
/// opt_trad_pfls_amt
/// futr_evlu_pfls_amt
/// opt_evlu_pfls_amt
/// trad_pfls_amt_smtl
/// evlu_pfls_amt_smtl
/// wdrw_psbl_tot_amt
/// ord_psbl_cash
/// ord_psbl_sbst
/// ord_psbl_tota
/// mmga_tot_amt
/// mmga_cash_amt
/// mtnc_rt
/// isfc_amt
/// pchs_amt_smtl
/// evlu_amt_smtl
/// acnt_prdt_cd
/// prdt_type_cd
/// shtn_pdno
/// prdt_name
/// sll_buy_dvsn_name
/// sll_buy_dvsn_cd
/// trad_dvsn_name
/// cblc_qty
/// excc_unpr
/// ccld_avg_unpr1
/// idx_clpr
/// pchs_amt
/// evlu_amt
/// evlu_pfls_amt
/// trad_pfls_amt
/// lqd_psbl_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireNgtBalanceRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 계좌비밀번호 (String, 필수)
    /// 공란("")으로 조회
    #[serde(rename = "ACNT_PWD")]
    pub acnt_pwd: String,
    /// 증거금구분 (String, 필수)
    /// 01 : 개시, 02 : 유지
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드 (String, 필수)
    /// 1 : 정산 (정산가격으로 잔고 조회)
    /// 2 : 본정산 (매입가격으로 잔고 조회)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [(야간)선물옵션 주문가능 조회] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// (야간)선물옵션 주문가능 조회 [국내선물-011]
/// max_ord_psbl_qty
/// tot_psbl_qty
/// lqd_psbl_qty
/// lqd_psbl_qty_1
/// ord_psbl_qty
/// bass_idx
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblNgtOrderRequest {
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
    /// 301 : 선물옵션
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 매도매수구분코드 (String, 필수)
    /// 01 : 매도 , 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문가격1 (String, 필수)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 주문구분코드 (String, 필수)
    /// '01 : 지정가 02 : 시장가
    /// 03 : 조건부 04 : 최유리,
    /// 10 : 지정가(IOC) 11 : 지정가(FOK)
    /// 12 : 시장가(IOC) 13 : 시장가(FOK)
    /// 14 : 최유리(IOC) 15 : 최유리(FOK)'
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [(야간)선물옵션 증거금 상세] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// (야간)선물옵션 증거금 상세 [국내선물-024]
/// cash_amt
/// tot_amt
/// cash_amt
/// sbst_amt
/// tot_amt
/// base_dpsa_gdat_grad_cd
/// bfdy_sbst_sll_ccld_amt
/// bfdy_sbst_sll_sbst_amt
/// excc_dfpa
/// fee_amt
/// nxdy_dncl_amt
/// opt_base_dpsa_gdat_grad_cd
/// opt_buy_exus_acnt_yn
/// opt_dfpa
/// prsm_dpast_amt
/// thdt_sbst_sll_ccld_amt
/// thdt_sbst_sll_sbst_amt
/// futr_new_mgn_amt
/// futr_sprd_ord_mgna
/// opt_sll_new_mgn_amt
/// opt_buy_new_mgn_amt
/// new_mgn_amt
/// opt_pric_mgna
/// fuop_pric_altr_mgna
/// futr_sprd_mgna
/// uwdl_mgna
/// ctrt_per_min_mgna
/// tot_risk_mgna
/// netrisk_brkg_mgna
/// opt_sll_chgs
/// opt_buy_chgs
/// futr_loss_amt
/// futr_prft_amt
/// thdt_ccld_net_loss_amt
/// brkg_mgna
/// futr_new_mgn_amt
/// futr_sprd_ord_mgna
/// opt_sll_new_mgn_amt
/// opt_buy_new_mgn_amt
/// new_mgn_amt
/// opt_pric_mgna
/// fuop_pric_altr_mgna
/// futr_sprd_mgna
/// uwdl_mgna
/// ctrt_per_min_mgna
/// tot_risk_mgna
/// netrisk_brkg_mgna
/// opt_sll_chgs
/// opt_buy_chgs
/// futr_loss_amt
/// futr_prft_amt
/// thdt_ccld_net_loss_amt
/// brkg_mgna
/// dnca_cash
/// dnca_sbst
/// dnca_tota
/// wdrw_psbl_cash_amt
/// wdrw_psbl_sbsa
/// wdrw_psbl_tot_amt
/// ord_psbl_cash_amt
/// ord_psbl_sbsa
/// ord_psbl_tot_amt
/// brkg_mgna_cash_amt
/// brkg_mgna_sbst
/// brkg_mgna_tot_amt
/// add_mgna_cash_amt
/// add_mgna_sbsa
/// add_mgna_tot_amt
/// bfdy_sbst_sll_sbst_amt
/// thdt_sbst_sll_sbst_amt
/// bfdy_sbst_sll_ccld_amt
/// thdt_sbst_sll_ccld_amt
/// opt_dfpa
/// excc_dfpa
/// fee_amt
/// nxdy_dncl_amt
/// prsm_dpast_amt
/// opt_buy_exus_acnt_yn
/// base_dpsa_gdat_grad_cd
/// opt_base_dpsa_gdat_grad_cd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NgtMarginDetailRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금 구분코드 (String, 필수)
    /// 위탁(01), 유지(02)
    #[serde(rename = "MGNA_DVSN_CD")]
    pub mgna_dvsn_cd: String,
}

/// [선물옵션 잔고정산손익내역] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 잔고정산손익내역[v1_국내선물-013]
/// nxdy_dnca
/// mmga_cash
/// brkg_mgna_cash
/// opt_buy_chgs
/// opt_lqd_evlu_amt
/// dnca_sbst
/// mmga_tota
/// brkg_mgna_tota
/// opt_sll_chgs
/// thdt_dfpa
/// rnwl_dfpa
/// dnca_cash
/// prdt_name
/// trad_dvsn_name
/// bfdy_cblc_qty
/// new_qty
/// mnpl_rpch_qty
/// cblc_qty
/// cblc_amt
/// trad_pfls_amt
/// evlu_amt
/// evlu_pfls_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceSettlementPlRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회일자 (String, 필수)
    /// 조회일자(YYYYMMDD)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 총자산현황] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 총자산현황[v1_국내선물-014]
/// dnca_tota
/// bfdy_chck_amt
/// thdt_chck_amt
/// rlth_uwdl_dpos_amt
/// brkg_mgna_cash
/// wdrw_psbl_tot_amt
/// ord_psbl_cash
/// ord_psbl_tota
/// dnca_sbst
/// scts_sbst_amt
/// frcr_evlu_amt
/// brkg_mgna_sbst
/// sbst_rlse_psbl_amt
/// mtnc_rt
/// add_mgna_tota
/// add_mgna_cash
/// futr_trad_pfls
/// opt_trad_pfls_amt
/// trad_pfls_smtl
/// futr_evlu_pfls_amt
/// opt_evlu_pfls_amt
/// evlu_pfls_smtl
/// excc_dfpa
/// opt_dfpa
/// brkg_fee
/// nxdy_dnca
/// prsm_dpast_amt
/// cash_mntn_amt
/// hack_acdt_acnt_move_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDepositV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
}

/// [선물옵션 잔고평가손익내역] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 잔고평가손익내역[v1_국내선물-015]
/// dnca_cash
/// frcr_dncl_amt
/// dnca_sbst
/// tot_dncl_amt
/// tot_ccld_amt
/// cash_mgna
/// sbst_mgna
/// mgna_tota
/// opt_dfpa
/// thdt_dfpa
/// rnwl_dfpa
/// nxdy_dnca
/// nxdy_dncl_amt
/// prsm_dpast
/// prsm_dpast_amt
/// pprt_ord_psbl_cash
/// add_mgna_cash
/// add_mgna_tota
/// futr_trad_pfls_amt
/// opt_trad_pfls_amt
/// futr_evlu_pfls_amt
/// opt_evlu_pfls_amt
/// trad_pfls_amt_smtl
/// evlu_pfls_amt_smtl
/// wdrw_psbl_tot_amt
/// ord_psbl_cash
/// ord_psbl_sbst
/// ord_psbl_tota
/// acnt_prdt_cd
/// prdt_type_cd
/// shtn_pdno
/// prdt_name
/// sll_buy_dvsn_name
/// cblc_qty1
/// excc_unpr
/// ccld_avg_unpr1
/// idx_clpr
/// pchs_amt
/// evlu_amt
/// evlu_pfls_amt
/// trad_pfls_amt
/// lqd_psbl_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceValuationPlRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금구분 (String, 필수)
    /// 01 : 개시, 02 : 유지
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드 (String, 필수)
    /// 1 : 정산 (정산가격으로 잔고 조회)
    /// 2 : 본정산 (매입가격으로 잔고 조회)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 기준일체결내역] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 기준일체결내역[v1_국내선물-016]
/// prdt_name
/// tr_type_name
/// last_sttldt
/// ccld_idx
/// ccld_qty
/// trad_amt
/// ccld_btwn
/// tot_ccld_qty_smtl
/// tot_ccld_amt_smtl
/// fee_adjt
/// fee_smtl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlBstimeRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자 (String, 필수)
    /// 주문일자(YYYYMMDD)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 선물옵션거래시작시각 (String, 필수)
    /// 선물옵션거래시작시간(HHMMSS)
    #[serde(rename = "FUOP_TR_STRT_TMD")]
    pub fuop_tr_strt_tmd: String,
    /// 선물옵션거래종료시각 (String, 필수)
    /// 선물옵션거래종료시간(HHMMSS)
    #[serde(rename = "FUOP_TR_END_TMD")]
    pub fuop_tr_end_tmd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션기간약정수수료일별] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션기간약정수수료일별[v1_국내선물-017]
/// ord_dt
/// item_name
/// sll_agrm_amt
/// sll_fee
/// buy_agrm_amt
/// buy_fee
/// tot_fee_smtl
/// trad_pfls
/// futr_agrm
/// futr_agrm_amt
/// futr_agrm_amt_smtl
/// futr_sll_fee_smtl
/// futr_buy_fee_smtl
/// futr_fee_smtl
/// opt_agrm
/// opt_agrm_amt
/// opt_agrm_amt_smtl
/// opt_sll_fee_smtl
/// opt_buy_fee_smtl
/// opt_fee_smtl
/// prdt_futr_agrm
/// prdt_fuop
/// prdt_futr_evlu_amt
/// futr_fee
/// opt_fee
/// sll_agrm_amt
/// buy_agrm_amt
/// agrm_amt_smtl
/// sll_fee
/// buy_fee
/// fee_smtl
/// trad_pfls_smtl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyAmountFeeRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일 (String, 필수)
    /// 조회시작일(YYYYMMDD)
    #[serde(rename = "INQR_STRT_DAY")]
    pub inqr_strt_day: String,
    /// 조회종료일 (String, 필수)
    /// 조회종료일(YYYYMMDD)
    #[serde(rename = "INQR_END_DAY")]
    pub inqr_end_day: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 증거금률] 요청 구조체
/// [국내선물옵션] 주문/계좌
/// 선물옵션 증거금률
/// bast_id
/// bast_name
/// brkg_mgna_rt
/// tr_mgna_rt
/// bast_pric
/// tr_mtpl_idx
/// ctrt_per_futr_mgna
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarginRateRequest {
    /// 기준일자 (String, 필수)
    /// 날짜 입력) ex) 20260313
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 기초자산ID (String, 필수)
    /// 공백 입력
    #[serde(rename = "BAST_ID")]
    pub bast_id: String,
    /// 연속조회키200 (String, 필수)
    /// 다음 조회 시 필요, 입력 후 header tr_cont : N 설정 필수
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 시세] 요청 구조체
/// [국내선물옵션] 기본시세
/// 선물옵션 시세[v1_국내선물-006]
/// hts_kor_isnm
/// futs_prpr
/// futs_prdy_vrss
/// prdy_vrss_sign
/// futs_prdy_clpr
/// futs_prdy_ctrt
/// acml_vol
/// acml_tr_pbmn
/// hts_otst_stpl_qty
/// otst_stpl_qty_icdc
/// futs_oprc
/// futs_hgpr
/// futs_lwpr
/// futs_mxpr
/// futs_llam
/// futs_sdpr
/// hts_thpr
/// crbr_aply_mxpr
/// crbr_aply_llam
/// futs_last_tr_date
/// hts_rmnn_dynu
/// futs_lstn_medm_hgpr
/// futs_lstn_medm_lwpr
/// delta_val
/// hist_vltl
/// hts_ints_vltl
/// mrkt_basis
/// bstp_cls_code
/// hts_kor_isnm
/// bstp_nmix_prpr
/// prdy_vrss_sign
/// bstp_nmix_prdy_vrss
/// bstp_nmix_prdy_ctrt
/// bstp_cls_code
/// hts_kor_isnm
/// bstp_nmix_prpr
/// prdy_vrss_sign
/// bstp_nmix_prdy_vrss
/// bstp_nmix_prdy_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceV3Request {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목코드 (예: 101S03)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [선물옵션 시세호가] 요청 구조체
/// [국내선물옵션] 기본시세
/// 선물옵션 시세호가[v1_국내선물-007]
/// hts_kor_isnm
/// futs_prpr
/// prdy_vrss_sign
/// futs_prdy_vrss
/// futs_prdy_ctrt
/// acml_vol
/// futs_prdy_clpr
/// futs_shrn_iscd
/// futs_askp1
/// futs_askp2
/// futs_askp3
/// futs_askp4
/// futs_askp5
/// futs_bidp1
/// futs_bidp2
/// futs_bidp3
/// futs_bidp4
/// futs_bidp5
/// askp_rsqn1
/// askp_rsqn2
/// askp_rsqn3
/// askp_rsqn4
/// askp_rsqn5
/// bidp_rsqn1
/// bidp_rsqn2
/// bidp_rsqn3
/// bidp_rsqn4
/// bidp_rsqn5
/// askp_csnu1
/// askp_csnu2
/// askp_csnu3
/// askp_csnu4
/// askp_csnu5
/// bidp_csnu1
/// bidp_csnu2
/// bidp_csnu3
/// bidp_csnu4
/// bidp_csnu5
/// total_askp_rsqn
/// total_bidp_rsqn
/// total_askp_csnu
/// total_bidp_csnu
/// aspr_acpt_hour
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목코드 (예: 101S03)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [선물옵션기간별시세(일/주/월/년)] 요청 구조체
/// [국내선물옵션] 기본시세
/// 선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyFuopchartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션,
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드 (String, 필수)
    /// 종목번호 (지수선물:6자리, 지수옵션 9자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회 시작일자 (String, 필수)
    /// 조회 시작일자 (ex. 20220401)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 조회 종료일자 (String, 필수)
    /// 조회 종료일자 (ex. 20220524)
    ///
    /// ※ 주(W), 월(M), 년(Y) 봉 조회 시에 아래 참고
    /// ㅁ FID_INPUT_DATE_2 가 현재일 까지일때
    /// . 주봉 조회 : 해당 주의 첫번째 영업일이 포함되어야함
    /// . 월봉 조회 : 해당 월의 전월 일자로 시작되어야함
    /// . 년봉 조회 : 해당 년의 전년도 일자로 시작되어야함
    /// ㅁ FID_INPUT_DATE_2 가 현재일보다 이전일 때
    /// . 주봉 조회 : 해당 주의 첫번째 영업일이 포함되어야함
    /// . 월봉 조회 : 해당 월의 영업일이 포함되어야함
    /// . 년봉 조회 : 해당 년의 영업일이 포함되어야함
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 기간분류코드 (String, 필수)
    /// D:일봉 W:주봉, M:월봉, Y:년봉
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [선물옵션 분봉조회] 요청 구조체
/// [국내선물옵션] 기본시세
/// 선물옵션 분봉조회[v1_국내선물-012]
/// futs_prdy_vrss
/// prdy_vrss_sign
/// futs_prdy_ctrt
/// futs_prdy_clpr
/// prdy_nmix
/// acml_vol
/// acml_tr_pbmn
/// hts_kor_isnm
/// futs_prpr
/// futs_shrn_iscd
/// prdy_vol
/// futs_mxpr
/// futs_llam
/// futs_oprc
/// futs_hgpr
/// futs_lwpr
/// futs_prdy_oprc
/// futs_prdy_hgpr
/// futs_prdy_lwpr
/// futs_askp
/// futs_bidp
/// kospi200_nmix
/// kospi200_prdy_vrss
/// kospi200_prdy_ctrt
/// kospi200_prdy_vrss_sign
/// hts_otst_stpl_qty
/// otst_stpl_qty_icdc
/// tday_rltv
/// hts_thpr
/// stck_bsop_date
/// stck_cntg_hour
/// futs_prpr
/// futs_oprc
/// futs_hgpr
/// futs_lwpr
/// cntg_vol
/// acml_tr_pbmn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeFuopchartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션,
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목번호 (지수선물:6자리, 지수옵션 9자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 시간 구분 코드 (String, 필수)
    /// FID 시간 구분 코드(30: 30초, 60: 1분, 3600: 1시간)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// FID 과거 데이터 포함 여부 (String, 필수)
    /// Y(과거) / N (당일)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// FID 허봉 포함 여부 (String, 필수)
    /// N으로 입력
    #[serde(rename = "FID_FAKE_TICK_INCU_YN")]
    pub fid_fake_tick_incu_yn: String,
    /// FID 입력 날짜1 (String, 필수)
    /// 입력 날짜 기준으로 이전 기간 조회(YYYYMMDD)
    /// ex) 20230908 입력 시, 2023년 9월 8일부터 일자 역순으로 조회
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 시간1 (String, 필수)
    /// 입력 시간 기준으로 이전 시간 조회(HHMMSS)
    /// ex) 093000 입력 시, 오전 9시 30분부터 역순으로 분봉 조회
    ///
    /// * CM(야간선물), EU(야간옵션)인 경우, 자정 이후 시간은 +24시간으로 입력
    /// ex) 253000 입력 시, 새벽 1시 30분부터 역순으로 분봉 조회
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [국내옵션전광판_옵션월물리스트] 요청 구조체
/// [국내선물옵션] 기본시세
/// 국내옵션전광판_옵션월물리스트[국내선물-020]
/// mtrt_yymm_code
/// mtrt_yymm
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardOptionListRequest {
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(509)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 시장 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [국내선물 기초자산 시세] 요청 구조체
/// [국내선물옵션] 기본시세
/// 국내선물 기초자산 시세[국내선물-021]
/// unas_prpr
/// unas_prdy_vrss
/// unas_prdy_vrss_sign
/// unas_prdy_ctrt
/// unas_acml_vol
/// hts_kor_isnm
/// futs_prpr
/// futs_prdy_vrss
/// prdy_vrss_sign
/// futs_prdy_ctrt
/// hts_rmnn_dynu
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardTopRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (F: 선물)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 선물최근월물 ex)(101V06)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_COND_MRKT_DIV_CODE1")]
    pub fid_cond_mrkt_div_code1: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 만기 수 (String, 필수)
    /// 공백
    #[serde(rename = "FID_MTRT_CNT")]
    pub fid_mtrt_cnt: String,
    /// 조건 시장 구분 코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [국내옵션전광판_콜풋] 요청 구조체
/// [국내선물옵션] 기본시세
/// 국내옵션전광판_콜풋[국내선물-022]
/// unch_prpr
/// optn_shrn_iscd
/// optn_prpr
/// optn_prdy_vrss
/// prdy_vrss_sign
/// optn_prdy_ctrt
/// optn_bidp
/// optn_askp
/// tmvl_val
/// nmix_sdpr
/// acml_vol
/// seln_rsqn
/// shnu_rsqn
/// acml_tr_pbmn
/// hts_otst_stpl_qty
/// otst_stpl_qty_icdc
/// delta_val
/// hts_ints_vltl
/// invl_val
/// hist_vltl
/// hts_thpr
/// optn_oprc
/// optn_hgpr
/// optn_lwpr
/// optn_mxpr
/// optn_llam
/// atm_cls_name
/// rgbf_vrss_icdc
/// total_askp_rsqn
/// total_bidp_rsqn
/// futs_antc_cnpr
/// futs_antc_cntg_vrss
/// antc_cntg_vrss_sign
/// antc_cntg_prdy_ctrt
/// unch_prpr
/// optn_shrn_iscd
/// optn_prpr
/// optn_prdy_vrss
/// prdy_vrss_sign
/// optn_prdy_ctrt
/// optn_bidp
/// optn_askp
/// tmvl_val
/// nmix_sdpr
/// acml_vol
/// seln_rsqn
/// shnu_rsqn
/// acml_tr_pbmn
/// hts_otst_stpl_qty
/// otst_stpl_qty_icdc
/// delta_val
/// hts_ints_vltl
/// invl_val
/// hist_vltl
/// hts_thpr
/// optn_oprc
/// optn_hgpr
/// optn_lwpr
/// optn_mxpr
/// optn_llam
/// atm_cls_name
/// rgbf_vrss_icdc
/// total_askp_rsqn
/// total_bidp_rsqn
/// futs_antc_cnpr
/// futs_antc_cntg_vrss
/// antc_cntg_vrss_sign
/// antc_cntg_prdy_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardCallputRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (O: 옵션)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20503)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 시장 구분 코드 (String, 필수)
    /// 시장구분코드 (CO: 콜옵션)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 만기 수 (String, 필수)
    /// - FID_COND_MRKT_CLS_CODE : 공백(KOSPI200), MKI(미니KOSPI200), KQI(KOSDAQ150) 인 경우
    /// : 만기년월(YYYYMM) 입력 (ex. 202407)
    ///
    /// - FID_COND_MRKT_CLS_CODE : WKM(KOSPI200위클리(월)), WKI(KOSPI200위클리(목)) 인 경우
    /// : 만기년월주차(YYMMWW) 입력
    /// (ex. 2024년도 7월 3주차인 경우, 240703 입력)
    #[serde(rename = "FID_MTRT_CNT")]
    pub fid_mtrt_cnt: String,
    /// 조건 시장 구분 코드 (String, 필수)
    /// 공백: KOSPI200
    /// MKI: 미니KOSPI200
    /// WKM: KOSPI200위클리(월)
    /// WKI: KOSPI200위클리(목)
    /// KQI: KOSDAQ150
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 시장 구분 코드 (String, 필수)
    /// 시장구분코드 (PO: 풋옵션)
    #[serde(rename = "FID_MRKT_CLS_CODE1")]
    pub fid_mrkt_cls_code1: String,
}

/// [국내옵션전광판_선물] 요청 구조체
/// [국내선물옵션] 기본시세
/// 국내옵션전광판_선물[국내선물-023]
/// futs_shrn_iscd
/// hts_kor_isnm
/// futs_prpr
/// futs_prdy_vrss
/// prdy_vrss_sign
/// futs_prdy_ctrt
/// hts_thpr
/// acml_vol
/// futs_askp
/// futs_bidp
/// hts_otst_stpl_qty
/// futs_hgpr
/// futs_lwpr
/// hts_rmnn_dynu
/// total_askp_rsqn
/// total_bidp_rsqn
/// futs_antc_cnpr
/// futs_antc_cntg_vrss
/// antc_cntg_vrss_sign
/// antc_cntg_prdy_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardFuturesRequest {
    /// 조건 시장 분류 코드 (String, 필수)
    /// 시장구분코드 (F: 선물)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드 (String, 필수)
    /// Unique key(20503)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 구분 코드 (String, 필수)
    /// 공백: KOSPI200
    /// MKI: 미니KOSPI200
    /// WKM: KOSPI200위클리(월)
    /// WKI: KOSPI200위클리(목)
    /// KQI: KOSDAQ150
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [선물옵션 일중예상체결추이] 요청 구조체
/// [국내선물옵션] 기본시세
/// 선물옵션 일중예상체결추이[국내선물-018]
/// hts_kor_isnm
/// futs_antc_cnpr
/// antc_cntg_vrss_sign
/// futs_antc_cntg_vrss
/// antc_cntg_prdy_ctrt
/// futs_sdpr
/// stck_cntg_hour
/// futs_antc_cnpr
/// antc_cntg_vrss_sign
/// futs_antc_cntg_vrss
/// antc_cntg_prdy_ctrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpPriceTrendV2Request {
    /// 입력 종목코드 (String, 필수)
    /// 종목번호 (지수선물:6자리, 지수옵션 9자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드 (String, 필수)
    /// F : 지수선물, O : 지수옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [지수선물 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 지수선물 실시간호가[실시간-011]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ifasp0Request {
    /// 거래ID (String, 필수)
    /// H0IFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    /// 예:101S12
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수선물 실시간체결가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 지수선물 실시간체결가[실시간-010]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ifcnt0Request {
    /// 거래ID (String, 필수)
    /// H0IFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    /// 예:101S12
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수옵션 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 지수옵션 실시간호가[실시간-015]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ioasp0Request {
    /// 거래ID (String, 필수)
    /// H0IOASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    /// 예:201S11305
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수옵션 실시간체결가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 지수옵션 실시간체결가[실시간-014]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Iocnt0Request {
    /// 거래ID (String, 필수)
    /// H0IOCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    /// 예:201S11305
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [선물옵션 실시간체결통보] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 선물옵션 실시간체결통보[실시간-012]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ifcni0Request {
    /// 거래ID (String, 필수)
    /// [실전투자]
    /// H0IFCNI0 : 실시간 선물옵션 체결통보
    ///
    /// [모의투자]
    /// H0IFCNI9 : 실시간 선물옵션 체결통보
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드 (String, 필수)
    /// 예:101S12
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [상품선물 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 상품선물 실시간호가[실시간-023]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Cfasp0Request {
    /// 거래ID (String, 필수)
    /// H0CFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [상품선물 실시간체결가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 상품선물 실시간체결가[실시간-022]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Cfcnt0Request {
    /// 거래ID (String, 필수)
    /// H0CFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 주식선물 실시간호가 [실시간-030]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zfasp0Request {
    /// 거래ID (String, 필수)
    /// H0ZFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간체결가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 주식선물 실시간체결가 [실시간-029]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zfcnt0Request {
    /// 거래ID (String, 필수)
    /// H0ZFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간예상체결] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 주식선물 실시간예상체결 [실시간-031]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zfanc0Request {
    /// 거래ID (String, 필수)
    /// H0ZFANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 주식선물 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 주식옵션 실시간호가 [실시간-045]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zoasp0Request {
    /// 거래ID (String, 필수)
    /// H0ZOASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간체결가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 주식옵션 실시간체결가 [실시간-044]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zocnt0Request {
    /// 거래ID (String, 필수)
    /// H0ZOCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간예상체결] 요청 구조체
/// [국내선물옵션] 실시간시세
/// 주식옵션 실시간예상체결 [실시간-046]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zoanc0Request {
    /// 거래ID (String, 필수)
    /// H0ZOANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 주식옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간옵션 실시간호가 [실시간-033]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Euasp0Request {
    /// 거래ID (String, 필수)
    /// H0EUASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 야간옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션 실시간체결가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간옵션 실시간체결가 [실시간-032]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Eucnt0Request {
    /// 거래ID (String, 필수)
    /// H0EUCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 야간옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션실시간예상체결] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간옵션실시간예상체결 [실시간-034]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Euanc0Request {
    /// 거래ID (String, 필수)
    /// H0EUANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 야간옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션실시간체결통보] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간옵션실시간체결통보 [실시간-067]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Eucni0Request {
    /// 거래ID (String, 필수)
    /// H0MFCNI0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// HTS ID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간호가] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간선물 실시간호가 [실시간-065]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Mfasp0Request {
    /// 거래ID (String, 필수)
    /// H0MFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 야간선물 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간종목체결] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간선물 실시간종목체결 [실시간-064]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Mfcnt0Request {
    /// 거래ID (String, 필수)
    /// H0MFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 야간선물 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간체결통보] 요청 구조체
/// [국내선물옵션] 실시간시세
/// KRX야간선물 실시간체결통보 [실시간-066]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Mfcni0Request {
    /// 거래ID (String, 필수)
    /// H0MFCNI0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// HTS ID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 주문] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 주문[v1_해외주식-001]
/// KRX_FWDG_ORD_ORGNO
/// ORD_TMD
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// NASD : 나스닥
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    /// SEHK : 홍콩
    /// SHAA : 중국상해
    /// SZAA : 중국심천
    /// TKSE : 일본
    /// HASE : 베트남 하노이
    /// VNSE : 베트남 호치민
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (String, 필수)
    /// 주문수량 (해외거래소 별 최소 주문수량 및 주문단위 확인 필요)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    /// 1주당 가격
    /// * 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호 (String, 선택)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호 (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 판매유형 (String, 선택)
    /// 제거 : 매수
    /// 00 : 매도
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 주문서버구분코드 (String, 필수)
    /// "0"(Default)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분 (String, 필수)
    /// [Header tr_id TTTT1002U(미국 매수 주문)]
    /// 00 : 지정가
    /// 32 : LOO(장개시지정가)
    /// 34 : LOC(장마감지정가)
    /// 35 : TWAP (시간가중평균)
    /// 36 : VWAP (거래량가중평균)
    /// * 모의투자 VTTT1002U(미국 매수 주문)로는 00:지정가만 가능
    /// * TWAP, VWAP 주문은 분할시간 주문 입력 필수
    ///
    /// [Header tr_id TTTT1006U(미국 매도 주문)]
    /// 00 : 지정가
    /// 31 : MOO(장개시시장가)
    /// 32 : LOO(장개시지정가)
    /// 33 : MOC(장마감시장가)
    /// 34 : LOC(장마감지정가)
    /// 35 : TWAP (시간가중평균)
    /// 36 : VWAP (거래량가중평균)
    /// * 모의투자 VTTT1006U(미국 매도 주문)로는 00:지정가만 가능
    /// * TWAP, VWAP 주문은 분할시간 주문 입력 필수
    ///
    /// [Header tr_id TTTS1001U(홍콩 매도 주문)]
    /// 00 : 지정가
    /// 50 : 단주지정가
    /// * 모의투자 VTTS1001U(홍콩 매도 주문)로는 00:지정가만 가능
    ///
    /// [그외 tr_id]
    /// 제거
    ///
    /// ※ TWAP, VWAP 주문은 정정 불가
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 시작시간 (String, 선택)
    /// ※ TWAP, VWAP 주문유형이고 알고리즘주문시간구분코드가 00일때 사용
    /// ※ YYMMDD 형태로 입력
    /// ※ 시간 입력 시 정규장 종료 5분전까지 입력 가능
    #[serde(rename = "START_TIME")]
    pub start_time: String,
    /// 종료시간 (String, 선택)
    /// ※ TWAP, VWAP 주문유형이고 알고리즘주문시간구분코드가 00일때 사용
    /// ※ YYMMDD 형태로 입력
    /// ※ 시간 입력 시 정규장 종료 5분전까지 입력 가능
    #[serde(rename = "END_TIME")]
    pub end_time: String,
    /// 알고리즘주문시간구분코드 (String, 선택)
    /// 00 : 분할주문 시간 직접입력 , 02 : 정규장 종료시까지
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD")]
    pub algo_ord_tmd_dvsn_cd: String,
}

/// [해외주식 정정취소주문] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 정정취소주문[v1_해외주식-003]
/// KRX_FWDG_ORD_ORGNO
/// ORD_TMD
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclV3Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// NASD : 나스닥
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    /// SEHK : 홍콩
    /// SHAA : 중국상해
    /// SZAA : 중국심천
    /// TKSE : 일본
    /// HASE : 베트남 하노이
    /// VNSE : 베트남 호치민
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호 (String, 필수)
    /// 정정 또는 취소할 원주문번호
    /// (해외주식_주문 API ouput ODNO
    /// or 해외주식 미체결내역 API output ODNO 참고)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 정정취소구분코드 (String, 필수)
    /// 01 : 정정
    /// 02 : 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    /// 취소주문 시, "0" 입력
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 운용사지정주문번호 (String, 선택)
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 선택)
    /// "0"(Default)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

/// [해외주식 예약주문접수] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 예약주문접수[v1_해외주식-002]
/// RSVN_ORD_RCIT_DT
/// OVRS_RSVN_ODNO
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 매도매수구분코드 (String, 선택)
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 정정취소구분코드 (String, 필수)
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    /// 00 : "매도/매수 주문"시 필수 항목
    /// 02 : 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    /// 515 : 일본
    /// 501 : 홍콩 / 543 : 홍콩CNY / 558 : 홍콩USD
    /// 507 : 베트남 하노이거래소 / 508 : 베트남 호치민거래소
    /// 551 : 중국 상해A / 552 : 중국 심천A
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// NASD : 나스닥
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    /// SEHK : 홍콩
    /// SHAA : 중국상해
    /// SZAA : 중국심천
    /// TKSE : 일본
    /// HASE : 베트남 하노이
    /// VNSE : 베트남 호치민
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// FT주문수량 (String, 필수)
    #[serde(rename = "FT_ORD_QTY")]
    pub ft_ord_qty: String,
    /// FT주문단가3 (String, 필수)
    #[serde(rename = "FT_ORD_UNPR3")]
    pub ft_ord_unpr3: String,
    /// 주문서버구분코드 (String, 선택)
    /// "0"(Default)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 예약주문접수일자 (String, 선택)
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    #[serde(rename = "RSVN_ORD_RCIT_DT")]
    pub rsvn_ord_rcit_dt: String,
    /// 주문구분 (String, 선택)
    /// tr_id가 TTTT3014U(미국 예약 매수 주문)인 경우만 사용
    /// 00 : 지정가
    /// 35 : TWAP
    /// 36 : VWAP
    ///
    /// tr_id가 TTTT3016U(미국 예약 매도 주문)인 경우만 사용
    /// 00 : 지정가
    /// 31 : MOO(장개시시장가)
    /// 35 : TWAP
    /// 36 : VWAP
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 해외예약주문번호 (String, 선택)
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
    /// 알고리즘주문시간구분코드 (String, 선택)
    /// ※ TWAP, VWAP 주문에서만 사용. 예약주문은 시간입력 불가하여 02로 값 고정
    /// ※ 정규장 종료 10분전까지 가능
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD")]
    pub algo_ord_tmd_dvsn_cd: String,
}

/// [해외주식 예약주문접수취소] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 예약주문접수취소[v1_해외주식-004]
/// OVRS_RSVN_ODNO
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvCcnlV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외주문접수일자 (String, 필수)
    #[serde(rename = "RSYN_ORD_RCIT_DT")]
    pub rsyn_ord_rcit_dt: String,
    /// 해외예약주문번호 (String, 필수)
    /// 해외주식_예약주문접수 API Output ODNO(주문번호) 참고
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
}

/// [해외주식 매수가능금액조회] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 매수가능금액조회[v1_해외주식-014]
/// tr_crcy_cd
/// ord_psbl_frcr_amt
/// sll_ruse_psbl_amt
/// ovrs_ord_psbl_amt
/// max_ord_psbl_qty
/// echm_af_ord_psbl_amt
/// echm_af_ord_psbl_qty
/// ord_psbl_qty
/// frcr_ord_psbl_amt1
/// ovrs_max_ord_psbl_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsamountRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// NASD : 나스닥 / NYSE : 뉴욕 / AMEX : 아멕스
    /// SEHK : 홍콩 / SHAA : 중국상해 / SZAA : 중국심천
    /// TKSE : 일본 / HASE : 하노이거래소 / VNSE : 호치민거래소
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 해외주문단가 (String, 필수)
    /// 해외주문단가 (23.8) 정수부분 23자리, 소수부분 8자리
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "ITEM_CD")]
    pub item_cd: String,
}

/// [해외주식 미체결내역] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 미체결내역[v1_해외주식-005]
/// ord_dt
/// ord_gno_brno
/// orgn_odno
/// prdt_name
/// sll_buy_dvsn_cd
/// sll_buy_dvsn_cd_name
/// rvse_cncl_dvsn_cd
/// rvse_cncl_dvsn_cd_name
/// rjct_rson
/// rjct_rson_name
/// ord_tmd
/// tr_mket_name
/// tr_crcy_cd
/// natn_cd
/// natn_kor_name
/// ft_ord_qty
/// ft_ccld_qty
/// nccs_qty
/// ft_ord_unpr3
/// ft_ccld_unpr3
/// ft_ccld_amt3
/// ovrs_excg_cd
/// prcs_stat_name
/// loan_type_cd
/// loan_dt
/// usa_amk_exts_rqst_yn
/// splt_buy_attr_name
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireNccsRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// NASD : 나스닥
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    /// SEHK : 홍콩
    /// SHAA : 중국상해
    /// SZAA : 중국심천
    /// TKSE : 일본
    /// HASE : 베트남 하노이
    /// VNSE : 베트남 호치민
    ///
    /// * NASD 인 경우만 미국전체로 조회되며 나머지 거래소 코드는 해당 거래소만 조회됨
    /// * 공백 입력 시 다음조회가 불가능하므로, 반드시 거래소코드 입력해야 함
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 정렬순서 (String, 필수)
    /// DS : 정순
    /// 그외 : 역순
    ///
    /// [header tr_id: TTTS3018R]
    /// ""(공란)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 잔고] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 잔고[v1_해외주식-006]
/// acnt_prdt_cd
/// prdt_type_cd
/// ovrs_pdno
/// ovrs_item_name
/// frcr_evlu_pfls_amt
/// evlu_pfls_rt
/// pchs_avg_pric
/// ovrs_cblc_qty
/// ord_psbl_qty
/// frcr_pchs_amt1
/// ovrs_stck_evlu_amt
/// now_pric2
/// tr_crcy_cd
/// ovrs_excg_cd
/// loan_type_cd
/// loan_dt
/// expd_dt
/// frcr_pchs_amt1
/// ovrs_rlzt_pfls_amt
/// ovrs_tot_pfls
/// rlzt_erng_rt
/// tot_evlu_pfls_amt
/// tot_pftrt
/// frcr_buy_amt_smtl1
/// ovrs_rlzt_pfls_amt2
/// frcr_buy_amt_smtl2
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceV4Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// [모의]
    /// NASD : 나스닥
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    ///
    /// [실전]
    /// NASD : 미국전체
    /// NAS : 나스닥
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    ///
    /// [모의/실전 공통]
    /// SEHK : 홍콩
    /// SHAA : 중국상해
    /// SZAA : 중국심천
    /// TKSE : 일본
    /// HASE : 베트남 하노이
    /// VNSE : 베트남 호치민
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 거래통화코드 (String, 필수)
    /// USD : 미국달러
    /// HKD : 홍콩달러
    /// CNY : 중국위안화
    /// JPY : 일본엔화
    /// VND : 베트남동
    #[serde(rename = "TR_CRCY_CD")]
    pub tr_crcy_cd: String,
    /// 연속조회검색조건200 (String, 선택)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 선택)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 주문체결내역] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 주문체결내역[v1_해외주식-007]
/// ord_dt
/// ord_gno_brno
/// orgn_odno
/// sll_buy_dvsn_cd
/// sll_buy_dvsn_cd_name
/// rvse_cncl_dvsn
/// rvse_cncl_dvsn_name
/// prdt_name
/// ft_ord_qty
/// ft_ord_unpr3
/// ft_ccld_qty
/// ft_ccld_unpr3
/// ft_ccld_amt3
/// nccs_qty
/// prcs_stat_name
/// rjct_rson
/// rjct_rson_name
/// ord_tmd
/// tr_mket_name
/// tr_natn
/// tr_natn_name
/// ovrs_excg_cd
/// tr_crcy_cd
/// dmst_ord_dt
/// thco_ord_tmd
/// loan_type_cd
/// loan_dt
/// mdia_dvsn_name
/// usa_amk_exts_rqst_yn
/// splt_buy_attr_name
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlV3Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// 전종목일 경우 "%" 입력
    /// ※ 모의투자계좌의 경우 ""(전체 조회)만 가능
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문시작일자 (String, 필수)
    /// YYYYMMDD 형식 (현지시각 기준)
    #[serde(rename = "ORD_STRT_DT")]
    pub ord_strt_dt: String,
    /// 주문종료일자 (String, 필수)
    /// YYYYMMDD 형식 (현지시각 기준)
    #[serde(rename = "ORD_END_DT")]
    pub ord_end_dt: String,
    /// 매도매수구분 (String, 필수)
    /// 00 : 전체
    /// 01 : 매도
    /// 02 : 매수
    /// ※ 모의투자계좌의 경우 "00"(전체 조회)만 가능
    #[serde(rename = "SLL_BUY_DVSN")]
    pub sll_buy_dvsn: String,
    /// 체결미체결구분 (String, 필수)
    /// 00 : 전체
    /// 01 : 체결
    /// 02 : 미체결
    /// ※ 모의투자계좌의 경우 "00"(전체 조회)만 가능
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 해외거래소코드 (String, 필수)
    /// 전종목일 경우 "%" 입력
    /// NASD : 미국시장 전체(나스닥, 뉴욕, 아멕스)
    /// NYSE : 뉴욕
    /// AMEX : 아멕스
    /// SEHK : 홍콩
    /// SHAA : 중국상해
    /// SZAA : 중국심천
    /// TKSE : 일본
    /// HASE : 베트남 하노이
    /// VNSE : 베트남 호치민
    /// ※ 모의투자계좌의 경우 ""(전체 조회)만 가능
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 정렬순서 (String, 필수)
    /// DS : 정순
    /// AS : 역순
    /// ※ 모의투자계좌의 경우 정렬순서 사용불가(Default : DS(정순))
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 주문일자 (String, 필수)
    /// "" (Null 값 설정)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문채번지점번호 (String, 필수)
    /// "" (Null 값 설정)
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호 (String, 필수)
    /// "" (Null 값 설정)
    /// ※ 주문번호로 검색 불가능합니다. 반드시 ""(Null 값 설정) 바랍니다.
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 체결기준현재잔고] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 체결기준현재잔고[v1_해외주식-008]
/// prdt_name
/// cblc_qty13
/// thdt_buy_ccld_qty1
/// thdt_sll_ccld_qty1
/// ccld_qty_smtl1
/// ord_psbl_qty1
/// frcr_pchs_amt
/// frcr_evlu_amt2
/// evlu_pfls_amt2
/// evlu_pfls_rt1
/// bass_exrt
/// buy_crcy_cd
/// ovrs_now_pric1
/// avg_unpr3
/// tr_mket_name
/// natn_kor_name
/// pchs_rmnd_wcrc_amt
/// thdt_buy_ccld_frcr_amt
/// thdt_sll_ccld_frcr_amt
/// unit_amt
/// std_pdno
/// prdt_type_cd
/// scts_dvsn_name
/// loan_rmnd
/// loan_dt
/// loan_expd_dt
/// ovrs_excg_cd
/// item_lnkg_excg_cd
/// crcy_cd
/// crcy_cd_name
/// frcr_buy_amt_smtl
/// frcr_sll_amt_smtl
/// frcr_dncl_amt_2
/// frst_bltn_exrt
/// frcr_buy_mgn_amt
/// frcr_etc_mgna
/// frcr_drwg_psbl_amt_1
/// frcr_evlu_amt2
/// acpl_cstd_crcy_yn
/// nxdy_frcr_drwg_psbl_amt
/// pchs_amt_smtl
/// evlu_amt_smtl
/// evlu_pfls_amt_smtl
/// dncl_amt
/// cma_evlu_amt
/// tot_dncl_amt
/// etc_mgna
/// wdrw_psbl_tot_amt
/// frcr_evlu_tota
/// evlu_erng_rt1
/// pchs_amt_smtl_amt
/// evlu_amt_smtl_amt
/// tot_evlu_pfls_amt
/// tot_asst_amt
/// buy_mgn_amt
/// mgna_tota
/// frcr_use_psbl_amt
/// ustl_sll_amt_smtl
/// ustl_buy_amt_smtl
/// tot_frcr_cblc_smtl
/// tot_loan_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePresentBalanceV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 원화외화구분코드 (String, 필수)
    /// 01 : 원화
    /// 02 : 외화
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 국가코드 (String, 필수)
    /// 000 전체
    /// 840 미국
    /// 344 홍콩
    /// 156 중국
    /// 392 일본
    /// 704 베트남
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 거래시장코드 (String, 필수)
    /// [Request body NATN_CD 000 설정]
    /// 00 : 전체
    ///
    /// [Request body NATN_CD 840 설정]
    /// 00 : 전체
    /// 01 : 나스닥(NASD)
    /// 02 : 뉴욕거래소(NYSE)
    /// 03 : 미국(PINK SHEETS)
    /// 04 : 미국(OTCBB)
    /// 05 : 아멕스(AMEX)
    ///
    /// [Request body NATN_CD 156 설정]
    /// 00 : 전체
    /// 01 : 상해B
    /// 02 : 심천B
    /// 03 : 상해A
    /// 04 : 심천A
    ///
    /// [Request body NATN_CD 392 설정]
    /// 01 : 일본
    ///
    /// [Request body NATN_CD 704 설정]
    /// 01 : 하노이거래
    /// 02 : 호치민거래소
    ///
    /// [Request body NATN_CD 344 설정]
    /// 01 : 홍콩
    /// 02 : 홍콩CNY
    /// 03 : 홍콩USD
    #[serde(rename = "TR_MKET_CD")]
    pub tr_mket_cd: String,
    /// 조회구분코드 (String, 필수)
    /// 00 : 전체
    /// 01 : 일반해외주식
    /// 02 : 미니스탁
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
}

/// [해외주식 예약주문조회] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 예약주문조회[v1_해외주식-013]
/// cncl_yn
/// rsvn_ord_rcit_dt
/// ovrs_rsvn_odno
/// ord_dt
/// ord_gno_brno
/// sll_buy_dvsn_cd
/// sll_buy_dvsn_cd_name
/// ovrs_rsvn_ord_stat_cd
/// ovrs_rsvn_ord_stat_cd_name
/// prdt_type_cd
/// prdt_name
/// ord_rcit_tmd
/// ord_fwdg_tmd
/// tr_dvsn_name
/// ovrs_excg_cd
/// tr_mket_name
/// ord_stfno
/// ft_ord_qty
/// ft_ord_unpr3
/// ft_ccld_qty
/// nprc_rson_text
/// splt_buy_attr_name
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvListRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    /// 조회시작일자(YYYYMMDD)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// 조회종료일자(YYYYMMDD)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 조회구분코드 (String, 필수)
    /// 00 : 전체
    /// 01 : 일반해외주식
    /// 02 : 미니스탁
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// 상품유형코드 (String, 필수)
    /// [tr_id=TTTT3039R인 경우]
    /// 공백 입력 시 미국주식 전체조회
    /// [tr_id=TTTS3014R인 경우]
    /// 공백 입력 시 아시아주식 전체조회
    ///
    /// 512 : 미국 나스닥 / 513 : 미국 뉴욕거래소 / 529 : 미국 아멕스
    /// 515 : 일본
    /// 501 : 홍콩 / 543 : 홍콩CNY / 558 : 홍콩USD
    /// 507 : 베트남 하노이거래소 / 508 : 베트남 호치민거래소
    /// 551 : 중국 상해A / 552 : 중국 심천A
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// [tr_id=TTTT3039R인 경우]
    /// 공백 입력 시 미국주식 전체조회
    /// [tr_id=TTTS3014R인 경우]
    /// 공백 입력 시 아시아주식 전체조회
    ///
    /// NASD : 나스닥 / NYSE : 뉴욕 / AMEX : 아멕스
    /// SEHK : 홍콩 / SHAA : 중국상해 / SZAA : 중국심천
    /// TKSE : 일본 / HASE : 하노이거래소 / VNSE : 호치민거래소
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 결제기준잔고] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 결제기준잔고 [해외주식-064]
/// prdt_name
/// cblc_qty13
/// ord_psbl_qty1
/// avg_unpr3
/// ovrs_now_pric1
/// frcr_pchs_amt
/// frcr_evlu_amt2
/// evlu_pfls_amt2
/// bass_exrt
/// oprt_dtl_dtime
/// buy_crcy_cd
/// thdt_sll_ccld_qty1
/// thdt_buy_ccld_qty1
/// evlu_pfls_rt1
/// tr_mket_name
/// natn_kor_name
/// std_pdno
/// mgge_qty
/// loan_rmnd
/// prdt_type_cd
/// ovrs_excg_cd
/// scts_dvsn_name
/// ldng_cblc_qty
/// crcy_cd
/// crcy_cd_name
/// frcr_dncl_amt_2
/// frst_bltn_exrt
/// frcr_evlu_amt2
/// pchs_amt_smtl_amt
/// tot_evlu_pfls_amt
/// evlu_erng_rt1
/// tot_dncl_amt
/// wcrc_evlu_amt_smtl
/// tot_asst_amt2
/// frcr_cblc_wcrc_evlu_amt_smtl
/// tot_loan_amt
/// tot_ldng_evlu_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePaymtStdrBalanceRequest {
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
    /// 01(원화기준),02(외화기준)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 조회구분코드 (String, 필수)
    /// 00(전체), 01(일반), 02(미니스탁)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
}

/// [해외주식 일별거래내역] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 일별거래내역 [해외주식-063]
/// trad_dt
/// sttl_dt
/// sll_buy_dvsn_cd
/// sll_buy_dvsn_name
/// ovrs_item_name
/// ccld_qty
/// amt_unit_ccld_qty
/// ft_ccld_unpr2
/// ovrs_stck_ccld_unpr
/// tr_frcr_amt2
/// tr_amt
/// frcr_excc_amt_1
/// wcrc_excc_amt
/// dmst_frcr_fee1
/// frcr_fee1
/// dmst_wcrc_fee
/// ovrs_wcrc_fee
/// crcy_cd
/// std_pdno
/// erlm_exrt
/// loan_dvsn_cd
/// loan_dvsn_name
/// frcr_buy_amt_smtl
/// frcr_sll_amt_smtl
/// dmst_fee_smtl
/// ovrs_fee_smtl
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodTransRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 등록시작일자 (String, 필수)
    /// 입력날짜 ~ (ex) 20240420)
    #[serde(rename = "ERLM_STRT_DT")]
    pub erlm_strt_dt: String,
    /// 등록종료일자 (String, 필수)
    /// ~입력날짜 (ex) 20240520)
    #[serde(rename = "ERLM_END_DT")]
    pub erlm_end_dt: String,
    /// 해외거래소코드 (String, 필수)
    /// 공백
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    /// 공백 (전체조회), 개별종목 조회는 상품번호입력
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드 (String, 필수)
    /// 00(전체), 01(매도), 02(매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 대출구분코드 (String, 필수)
    /// 공백
    #[serde(rename = "LOAN_DVSN_CD")]
    pub loan_dvsn_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외주식 기간손익] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 기간손익[v1_해외주식-032]
/// trad_day
/// ovrs_pdno
/// ovrs_item_name
/// slcl_qty
/// pchs_avg_pric
/// frcr_pchs_amt1
/// avg_sll_unpr
/// frcr_sll_amt_smtl1
/// stck_sll_tlex
/// ovrs_rlzt_pfls_amt
/// ovrs_excg_cd
/// frst_bltn_exrt
/// stck_sll_amt_smtl
/// stck_buy_amt_smtl
/// smtl_fee1
/// excc_dfrm_amt
/// ovrs_rlzt_pfls_tot_amt
/// tot_pftrt
/// bass_dt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodProfitV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// 공란 : 전체,
    /// NASD : 미국, SEHK : 홍콩,
    /// SHAA : 중국, TKSE : 일본, HASE : 베트남
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 국가코드 (String, 필수)
    /// 공란(Default)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 통화코드 (String, 필수)
    /// 공란 : 전체
    /// USD : 미국달러, HKD : 홍콩달러,
    /// CNY : 중국위안화, JPY : 일본엔화, VND : 베트남동
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 상품번호 (String, 필수)
    /// 공란 : 전체
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 조회시작일자 (String, 필수)
    /// YYYYMMDD
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// YYYYMMDD
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 원화외화구분코드 (String, 필수)
    /// 01 : 외화, 02 : 원화
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외증거금 통화별조회] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외증거금 통화별조회 [해외주식-035]
/// natn_name
/// crcy_cd
/// frcr_dncl_amt1
/// ustl_buy_amt
/// ustl_sll_amt
/// frcr_rcvb_amt
/// frcr_mgn_amt
/// frcr_gnrl_ord_psbl_amt
/// frcr_ord_psbl_amt1
/// itgr_ord_psbl_amt
/// bass_exrt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ForeignMarginRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
}

/// [해외주식 미국주간주문] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 미국주간주문[v1_해외주식-026]
/// KRX_FWDG_ORD_ORGNO
/// ORD_TMD
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DaytimeOrderRequest {
    /// 종합계좌번호 (, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (, 필수)
    /// NASD:나스닥 / NYSE:뉴욕 / AMEX:아멕스
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (, 필수)
    /// 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량 (, 필수)
    /// 해외거래소 별 최소 주문수량 및 주문단위 확인 필요
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (, 필수)
    /// 소수점 포함, 1주당 가격
    /// * 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호 (, 선택)
    /// " "
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호 (, 선택)
    /// " "
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (, 필수)
    /// "0"
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분 (, 필수)
    /// [미국 매수/매도 주문]
    /// 00 : 지정가
    /// * 주간거래는 지정가만 가능
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
}

/// [해외주식 미국주간정정취소] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 미국주간정정취소[v1_해외주식-027]
/// KRX_FWDG_ORD_ORGNO
/// ORD_TMD
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DaytimeOrderRvsecnclRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드 (String, 필수)
    /// NASD:나스닥 / NYSE:뉴욕 / AMEX:아멕스
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호 (String, 필수)
    /// 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호 (String, 필수)
    /// '정정 또는 취소할 원주문번호(매매 TR의 주문번호)
    /// - 해외주식 주문체결내역api (/uapi/overseas-stock/v1/trading/inquire-nccs)에서 odno(주문번호) 참조'
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 정정취소구분코드 (String, 필수)
    /// '01 : 정정
    /// 02 : 취소'
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량 (String, 필수)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가 (String, 필수)
    /// 소수점 포함, 1주당 가격
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호 (String, 필수)
    /// " "
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호 (String, 필수)
    /// " "
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    /// "0"
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

/// [해외주식 지정가주문번호조회] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 지정가주문번호조회 [해외주식-071]
/// trad_dvsn_name
/// item_name
/// ft_ord_qty
/// ft_ord_unpr3
/// splt_buy_attr_name
/// ft_ccld_qty
/// ord_gno_brno
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AlgoOrdnoRequest {
    /// 거래일자 (String, 필수)
    /// YYYYMMDD
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// 계좌번호 (String, 필수)
    /// 종합계좌번호 (8자리)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌상품코드 (2자리) : 주식계좌는 01
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 연속조회키200 (String, 선택)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회조건200 (String, 선택)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 지정가체결내역조회] 요청 구조체
/// [해외주식] 주문/계좌
/// 해외주식 지정가체결내역조회 [해외주식-070]
/// CCLD_SEQ
/// CCLD_BTWN
/// ITEM_NAME
/// FT_CCLD_QTY
/// FT_CCLD_UNPR3
/// FT_CCLD_AMT3
/// TRAD_DVSN_NAME
/// ITEM_NAME
/// FT_ORD_QTY
/// FT_ORD_UNPR3
/// ORD_TMD
/// SPLT_BUY_ATTR_NAME
/// FT_CCLD_QTY
/// TR_CRCY
/// FT_CCLD_UNPR3
/// FT_CCLD_AMT3
/// CCLD_CNT
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAlgoCcnlRequest {
    /// 계좌번호 (String, 필수)
    /// 종합계좌번호 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 상품코드 2자리 (주식계좌 : 01)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자 (String, 필수)
    /// 주문일자 (YYYYMMDD)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문채번지점번호 (String, 선택)
    /// TTS6058R 조회 시 해당 주문번호(odno)의 ord_gno_brno 입력
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호 (String, 필수)
    /// 지정가주문번호 (TTTS6058R)에서 조회된 주문번호 입력
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 집계포함여부 (String, 선택)
    #[serde(rename = "TTLZ_ICLD_YN")]
    pub ttlz_icld_yn: String,
    /// 연속조회키200 (String, 선택)
    /// 연속조회 시 사용
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회조건200 (String, 선택)
    /// 연속조회 시 사용
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 현재가상세] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 현재가상세[v1_해외주식-029]
/// t_xprc
/// t_xdif
/// t_xrat
/// p_xprc
/// p_xdif
/// p_xrat
/// t_rate
/// p_rate
/// t_xsgn
/// p_xsng
/// e_ordyn
/// e_hogau
/// e_icod
/// e_parp
/// etyp_nm
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PriceDetailRequest {
    /// 사용자권한정보 (String, 필수)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소명 (String, 필수)
    /// HKS : 홍콩
    /// NYS : 뉴욕
    /// NAS : 나스닥
    /// AMS : 아멕스
    /// TSE : 도쿄
    /// SHS : 상해
    /// SZS : 심천
    /// SHI : 상해지수
    /// SZI : 심천지수
    /// HSX : 호치민
    /// HNX : 하노이
    /// BAY : 뉴욕(주간)
    /// BAQ : 나스닥(주간)
    /// BAA : 아멕스(주간)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 현재가 호가] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 현재가 호가 [해외주식-033]
/// rclose
/// pbid10
/// pask10
/// vbid10
/// vask10
/// dbid10
/// dask10
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceV2Request {
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// NYS : 뉴욕
    /// NAS : 나스닥
    /// AMS : 아멕스
    /// HKS : 홍콩
    /// SHS : 상해
    /// SZS : 심천
    /// HSX : 호치민
    /// HNX : 하노이
    /// TSE : 도쿄
    /// BAY : 뉴욕(주간)
    /// BAQ : 나스닥(주간)
    /// BAA : 아멕스(주간)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    /// 종목코드 예)TSLA
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 현재체결가] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 현재체결가[v1_해외주식-009]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PriceRequest {
    /// 사용자권한정보 (String, 필수)
    /// "" (Null 값 설정)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// HKS : 홍콩
    /// NYS : 뉴욕
    /// NAS : 나스닥
    /// AMS : 아멕스
    /// TSE : 도쿄
    /// SHS : 상해
    /// SZS : 심천
    /// SHI : 상해지수
    /// SZI : 심천지수
    /// HSX : 호치민
    /// HNX : 하노이
    /// BAY : 뉴욕(주간)
    /// BAQ : 나스닥(주간)
    /// BAA : 아멕스(주간)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 체결추이] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 체결추이[해외주식-037]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlV4Request {
    /// 거래소명 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 당일전일구분 (String, 필수)
    /// 0:전일, 1:당일
    #[serde(rename = "TDAY")]
    pub tday: String,
    /// 종목코드 (String, 필수)
    /// 해외종목코드
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식분봉조회] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식분봉조회[v1_해외주식-030]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeItemchartpriceV2Request {
    /// 사용자권한정보 (String, 필수)
    /// "" 공백으로 입력
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// NYS : 뉴욕
    /// NAS : 나스닥
    /// AMS : 아멕스
    /// HKS : 홍콩
    /// SHS : 상해
    /// SZS : 심천
    /// HSX : 호치민
    /// HNX : 하노이
    /// TSE : 도쿄
    ///
    /// ※ 주간거래는 최대 1일치 분봉만 조회 가능
    /// BAY : 뉴욕(주간)
    /// BAQ : 나스닥(주간)
    /// BAA : 아멕스(주간)
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    /// 종목코드(ex. TSLA)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 분갭 (String, 필수)
    /// 분단위(1: 1분봉, 2: 2분봉, ...)
    #[serde(rename = "NMIN")]
    pub nmin: String,
    /// 전일포함여부 (String, 필수)
    /// 0:당일 1:전일포함
    /// ※ 다음조회 시 반드시 "1"로 입력
    #[serde(rename = "PINC")]
    pub pinc: String,
    /// 다음여부 (String, 필수)
    /// 처음조회 시, "" 공백 입력
    /// 다음조회 시, "1" 입력
    #[serde(rename = "NEXT")]
    pub next: String,
    /// 요청갯수 (String, 필수)
    /// 레코드요청갯수 (최대 120)
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// 미체결채움구분 (String, 필수)
    /// "" 공백으로 입력
    #[serde(rename = "FILL")]
    pub fill: String,
    /// NEXT KEY BUFF (String, 필수)
    /// 처음 조회 시, "" 공백 입력
    /// 다음 조회 시, 이전 조회 결과의 마지막 분봉 데이터를 이용하여, 1분 전 혹은 n분 전의 시간을 입력
    /// (형식: YYYYMMDDHHMMSS, ex. 20241014140100)
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외지수분봉조회] 요청 구조체
/// [해외주식] 기본시세
/// 해외지수분봉조회[v1_해외주식-031]
/// ovrs_nmix_prdy_vrss
/// prdy_vrss_sign
/// hts_kor_isnm
/// prdy_ctrt
/// ovrs_nmix_prdy_clpr
/// acml_vol
/// ovrs_nmix_prpr
/// stck_shrn_iscd
/// ovrs_prod_oprc
/// ovrs_prod_hgpr
/// ovrs_prod_lwpr
/// stck_bsop_date
/// stck_cntg_hour
/// optn_prpr
/// optn_oprc
/// optn_hgpr
/// optn_lwpr
/// cntg_vol
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeIndexchartpriceV2Request {
    /// 조건 시장 분류 코드 (String, 필수)
    /// N 해외지수
    /// X 환율
    /// KX 원화환율
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목번호(ex. TSLA)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간 구분 코드 (String, 필수)
    /// 0: 정규장, 1: 시간외
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거 데이터 포함 여부 (String, 필수)
    /// Y/N
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [해외주식 기간별시세] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 기간별시세[v1_해외주식-010]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailypriceRequest {
    /// 사용자권한정보 (String, 필수)
    /// "" (Null 값 설정)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// HKS : 홍콩
    /// NYS : 뉴욕
    /// NAS : 나스닥
    /// AMS : 아멕스
    /// TSE : 도쿄
    /// SHS : 상해
    /// SZS : 심천
    /// SHI : 상해지수
    /// SZI : 심천지수
    /// HSX : 호치민
    /// HNX : 하노이
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 종목코드 (String, 필수)
    /// 종목코드 (ex. TSLA)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 일/주/월구분 (String, 필수)
    /// 0 : 일
    /// 1 : 주
    /// 2 : 월
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// 조회기준일자 (String, 필수)
    /// 조회기준일자(YYYYMMDD)
    /// ※ 공란 설정 시, 기준일 오늘 날짜로 설정
    #[serde(rename = "BYMD")]
    pub bymd: String,
    /// 수정주가반영여부 (String, 필수)
    /// 0 : 미반영
    /// 1 : 반영
    #[serde(rename = "MODP")]
    pub modp: String,
    /// NEXT KEY BUFF (String, 선택)
    /// 응답시 다음값이 있으면 값이 셋팅되어 있으므로 다음 조회시 응답값 그대로 셋팅
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외주식 종목/지수/환율기간별시세(일/주/월/년)] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]
/// ovrs_nmix_prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// ovrs_nmix_prdy_clpr
/// acml_vol
/// hts_kor_isnm
/// ovrs_nmix_prpr
/// stck_shrn_iscd
/// prdy_vol
/// ovrs_prod_oprc
/// ovrs_prod_hgpr
/// ovrs_prod_lwpr
/// stck_bsop_date
/// ovrs_nmix_prpr
/// ovrs_nmix_oprc
/// ovrs_nmix_hgpr
/// ovrs_nmix_lwpr
/// acml_vol
/// mod_yn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyChartpriceRequest {
    /// FID 조건 시장 분류 코드 (String, 필수)
    /// N: 해외지수, X 환율, I: 국채, S:금선물
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드 (String, 필수)
    /// 종목코드
    /// ※ 해외주식 마스터 코드 참조
    /// (포럼 > FAQ > 종목정보 다운로드(해외) > 해외지수)
    ///
    /// ※ 해당 API로 미국주식 조회 시, 다우30, 나스닥100, S&P500 종목만 조회 가능합니다. 더 많은 미국주식 종목 시세를 이용할 시에는, 해외주식기간별시세 API 사용 부탁드립니다.
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1 (String, 필수)
    /// 시작일자(YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2 (String, 필수)
    /// 종료일자(YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// FID 기간 분류 코드 (String, 필수)
    /// D:일, W:주, M:월, Y:년
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [해외주식조건검색] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식조건검색[v1_해외주식-015]
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireSearchRequest {
    /// 사용자권한정보 (String, 필수)
    /// "" (Null 값 설정)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 현재가선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_PRICECUR")]
    pub co_yn_pricecur: String,
    /// 현재가시작범위가 (String, 선택)
    /// 단위: 각국통화(JPY, USD, HKD, CNY, VND)
    #[serde(rename = "CO_ST_PRICECUR")]
    pub co_st_pricecur: String,
    /// 현재가끝범위가 (String, 선택)
    /// 단위: 각국통화(JPY, USD, HKD, CNY, VND)
    #[serde(rename = "CO_EN_PRICECUR")]
    pub co_en_pricecur: String,
    /// 등락율선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_RATE")]
    pub co_yn_rate: String,
    /// 등락율시작율 (String, 선택)
    /// %
    #[serde(rename = "CO_ST_RATE")]
    pub co_st_rate: String,
    /// 등락율끝율 (String, 선택)
    /// %
    #[serde(rename = "CO_EN_RATE")]
    pub co_en_rate: String,
    /// 시가총액선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_VALX")]
    pub co_yn_valx: String,
    /// 시가총액시작액 (String, 선택)
    /// 단위: 천
    #[serde(rename = "CO_ST_VALX")]
    pub co_st_valx: String,
    /// 시가총액끝액 (String, 선택)
    /// 단위: 천
    #[serde(rename = "CO_EN_VALX")]
    pub co_en_valx: String,
    /// 발행주식수선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_SHAR")]
    pub co_yn_shar: String,
    /// 발행주식시작수 (String, 선택)
    /// 단위: 천
    #[serde(rename = "CO_ST_SHAR")]
    pub co_st_shar: String,
    /// 발행주식끝수 (String, 선택)
    /// 단위: 천
    #[serde(rename = "CO_EN_SHAR")]
    pub co_en_shar: String,
    /// 거래량선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_VOLUME")]
    pub co_yn_volume: String,
    /// 거래량시작량 (String, 선택)
    /// 단위: 주
    #[serde(rename = "CO_ST_VOLUME")]
    pub co_st_volume: String,
    /// 거래량끝량 (String, 선택)
    /// 단위: 주
    #[serde(rename = "CO_EN_VOLUME")]
    pub co_en_volume: String,
    /// 거래대금선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_AMT")]
    pub co_yn_amt: String,
    /// 거래대금시작금 (String, 선택)
    /// 단위: 천
    #[serde(rename = "CO_ST_AMT")]
    pub co_st_amt: String,
    /// 거래대금끝금 (String, 선택)
    /// 단위: 천
    #[serde(rename = "CO_EN_AMT")]
    pub co_en_amt: String,
    /// EPS선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_EPS")]
    pub co_yn_eps: String,
    /// EPS시작 (String, 선택)
    #[serde(rename = "CO_ST_EPS")]
    pub co_st_eps: String,
    /// EPS끝 (String, 선택)
    #[serde(rename = "CO_EN_EPS")]
    pub co_en_eps: String,
    /// PER선택조건 (String, 선택)
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_PER")]
    pub co_yn_per: String,
    /// PER시작 (String, 선택)
    #[serde(rename = "CO_ST_PER")]
    pub co_st_per: String,
    /// PER끝 (String, 선택)
    #[serde(rename = "CO_EN_PER")]
    pub co_en_per: String,
    /// NEXT KEY BUFF (String, 선택)
    /// "" 공백 입력
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외결제일자조회] 요청 구조체
/// [해외주식] 기본시세
/// 해외결제일자조회[해외주식-017]
/// prdt_type_cd
/// tr_natn_cd
/// tr_natn_name
/// natn_eng_abrv_cd
/// tr_mket_cd
/// tr_mket_name
/// acpl_sttl_dt
/// dmst_sttl_dt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CountriesHolidayRequest {
    /// 기준일자 (String, 필수)
    /// 기준일자(YYYYMMDD)
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// 연속조회키 (String, 필수)
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// 연속조회검색조건 (String, 필수)
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
}

/// [해외주식 상품기본정보] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 상품기본정보[v1_해외주식-034]
/// std_pdno
/// prdt_eng_name
/// natn_cd
/// natn_name
/// tr_mket_cd
/// tr_mket_name
/// ovrs_excg_cd
/// ovrs_excg_name
/// tr_crcy_cd
/// ovrs_papr
/// crcy_name
/// ovrs_stck_dvsn_cd
/// prdt_clsf_cd
/// prdt_clsf_name
/// sll_unit_qty
/// buy_unit_qty
/// tr_unit_amt
/// lstg_stck_num
/// lstg_dt
/// ovrs_stck_tr_stop_dvsn_cd
/// lstg_abol_item_yn
/// ovrs_stck_prdt_grp_no
/// lstg_yn
/// tax_levy_yn
/// ovrs_stck_erlm_rosn_cd
/// ovrs_stck_hist_rght_dvsn_cd
/// chng_bf_pdno
/// prdt_type_cd_2
/// ovrs_item_name
/// sedol_no
/// blbg_tckr_text
/// ovrs_stck_etf_risk_drtp_cd
/// etp_chas_erng_rt_dbnb
/// istt_usge_isin_cd
/// mint_svc_yn
/// mint_svc_yn_chng_dt
/// prdt_name
/// lei_cd
/// ovrs_stck_stop_rson_cd
/// lstg_abol_dt
/// mini_stk_tr_stat_dvsn_cd
/// mint_frst_svc_erlm_dt
/// mint_dcpt_trad_psbl_yn
/// mint_fnum_trad_psbl_yn
/// mint_cblc_cvsn_ipsb_yn
/// ptp_item_yn
/// ptp_item_trfx_exmt_yn
/// ptp_item_trfx_exmt_strt_dt
/// ptp_item_trfx_exmt_end_dt
/// dtm_tr_psbl_yn
/// sdrf_stop_ecls_yn
/// sdrf_stop_ecls_erlm_dt
/// memo_text1
/// ovrs_now_pric1
/// last_rcvg_dtime
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchInfoV2Request {
    /// 상품유형코드 (String, 필수)
    /// 512 미국 나스닥 / 513 미국 뉴욕 / 529 미국 아멕스
    /// 515 일본
    /// 501 홍콩 / 543 홍콩CNY / 558 홍콩USD
    /// 507 베트남 하노이 / 508 베트남 호치민
    /// 551 중국 상해A / 552 중국 심천A
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 상품번호 (String, 필수)
    /// 예) AAPL (애플)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [해외주식 업종별시세] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 업종별시세[해외주식-048]
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndustryThemeRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 업종코드 (String, 필수)
    /// 업종코드별조회(HHDFS76370100) 를 통해 확인
    #[serde(rename = "ICOD")]
    pub icod: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 업종별코드조회] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 업종별코드조회[해외주식-049]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndustryPriceRequest {
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
}

/// [해외주식 복수종목 시세조회] 요청 구조체
/// [해외주식] 기본시세
/// 해외주식 복수종목 시세조회
/// t_xprc
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MultpriceRequest {
    /// 사용자권한정보 (String, 필수)
    /// 공백 입력 필수
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 종목요청개수 (String, 필수)
    /// 최대 10
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// 거래소코드 (String, 필수)
    /// HKS : 홍콩
    /// NYS : 뉴욕
    /// NAS : 나스닥
    /// AMS : 아멕스
    /// TSE : 도쿄
    /// SHS : 상해
    /// SZS : 심천
    /// SHI : 상해지수
    /// SZI : 심천지수
    /// HSX : 호치민
    /// HNX : 하노이
    /// BAY : 뉴욕(주간)
    /// BAQ : 나스닥(주간)
    /// BAA : 아멕스(주간)
    #[serde(rename = "EXCD_01 ~ 10")]
    pub excd_01_10: String,
    /// 종목코드 (String, 필수)
    /// API 문서 -> 종목정보파일 -> 마스터 파일 참조
    #[serde(rename = "SYMB_01 ~ 10")]
    pub symb_01_10: String,
}

/// [해외주식 가격급등락] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 가격급등락[해외주식-038]
/// n_base
/// n_diff
/// n_rate
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PriceFluctRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 급등/급락구분 (String, 필수)
    /// 0(급락), 1(급등)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// N분전콤보값 (String, 필수)
    /// N분전 : 0(1분전), 1(2분전), 2(3분전), 3(5분전), 4(10분전), 5(15분전), 6(20분전), 7(30분전), 8(60분전), 9(120분전)
    #[serde(rename = "MINX")]
    pub minx: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래량급증] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 거래량급증[해외주식-039]
/// n_tvol
/// n_diff
/// n_rate
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumeSurgeRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N분전콤보값 (String, 필수)
    /// N분전 : 0(1분전), 1(2분전), 2(3분전), 3(5분전), 4(10분전), 5(15분전), 6(20분전), 7(30분전), 8(60분전), 9(120분전)
    #[serde(rename = "MINX")]
    pub minx: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 매수체결강도상위] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 매수체결강도상위[해외주식-040]
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumePowerV2Request {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    /// N분전 : 0(1분전), 1(2분전), 2(3분전), 3(5분전), 4(10분전), 5(15분전), 6(20분전), 7(30분전), 8(60분전), 9(120분전)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 상승율/하락율] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 상승율/하락율[해외주식-041]
/// n_base
/// n_diff
/// n_rate
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UpdownRateV2Request {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 상승율/하락율 구분 (String, 필수)
    /// 0(하락율), 1(상승율)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// N일자값 (String, 필수)
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 신고/신저가] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 신고/신저가[해외주식-042]
/// n_base
/// n_diff
/// n_rate
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewHighlowRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 신고/신저 구분 (String, 필수)
    /// 신고(1) 신저(0)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// 일시돌파/돌파 구분 (String, 필수)
    /// 일시돌파(0) 돌파유지(1)
    #[serde(rename = "GUBN2")]
    pub gubn2: String,
    /// N일자값 (String, 필수)
    /// N일전 : 0(5일), 1(10일), 2(20일), 3(30일), 4(60일), 5(120일전), 6(52주), 7(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래량순위] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 거래량순위[해외주식-043]
/// a_tvol
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradeVolRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 현재가 필터범위 1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "PRC1")]
    pub prc1: String,
    /// 현재가 필터범위 2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "PRC2")]
    pub prc2: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래대금순위] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 거래대금순위[해외주식-044]
/// a_tamt
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradePbmnRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
    /// 현재가 필터범위 1 (String, 필수)
    /// 가격 ~
    #[serde(rename = "PRC1")]
    pub prc1: String,
    /// 현재가 필터범위 2 (String, 필수)
    /// ~ 가격
    #[serde(rename = "PRC2")]
    pub prc2: String,
}

/// [해외주식 거래증가율순위] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 거래증가율순위[해외주식-045]
/// n_tvol
/// n_rate
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradeGrowthRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래회전율순위] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 거래회전율순위[해외주식-046]
/// n_tvol
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradeTurnoverRequest {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값 (String, 필수)
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 시가총액순위] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 시가총액순위[해외주식-047]
/// e_ordyn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketCapV2Request {
    /// NEXT KEY BUFF (String, 필수)
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보 (String, 필수)
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드 (String, 필수)
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 거래량조건 (String, 필수)
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 기간별권리조회] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 기간별권리조회 [해외주식-052]
/// bass_dt
/// rght_type_cd
/// prdt_name
/// prdt_type_cd
/// std_pdno
/// acpl_bass_dt
/// sbsc_strt_dt
/// sbsc_end_dt
/// cash_alct_rt
/// stck_alct_rt
/// crcy_cd
/// crcy_cd2
/// crcy_cd3
/// crcy_cd4
/// alct_frcr_unpr
/// stkp_dvdn_frcr_amt2
/// stkp_dvdn_frcr_amt3
/// stkp_dvdn_frcr_amt4
/// dfnt_yn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PeriodRightsV2Request {
    /// 권리유형코드 (String, 필수)
    /// '%%(전체), 01(유상), 02(무상), 03(배당), 11(합병),
    /// 14(액면분할), 15(액면병합), 17(감자), 54(WR청구),
    /// 61(원리금상환), 71(WR소멸), 74(배당옵션), 75(특별배당), 76(ISINCODE변경), 77(실권주청약)'
    #[serde(rename = "RGHT_TYPE_CD")]
    pub rght_type_cd: String,
    /// 조회구분코드 (String, 필수)
    /// 02(현지기준일), 03(청약시작일), 04(청약종료일)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// 조회시작일자 (String, 필수)
    /// 일자 ~
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// ~ 일자
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 상품번호 (String, 필수)
    /// 공백
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// 공백
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 연속조회키50 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_NK50")]
    pub ctx_area_nk50: String,
    /// 연속조회검색조건50 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_FK50")]
    pub ctx_area_fk50: String,
}

/// [해외뉴스종합(제목)] 요청 구조체
/// [해외주식] 시세분석
/// 해외뉴스종합(제목) [해외주식-053]
/// info_gb
/// news_key
/// data_dt
/// data_tm
/// class_cd
/// class_name
/// source
/// nation_cd
/// exchange_cd
/// symb_name
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewsTitleV2Request {
    /// 뉴스구분 (String, 필수)
    /// 전체: 공백
    #[serde(rename = "INFO_GB")]
    pub info_gb: String,
    /// 중분류 (String, 필수)
    /// 전체: 공백
    #[serde(rename = "CLASS_CD")]
    pub class_cd: String,
    /// 국가코드 (String, 필수)
    /// 전체: 공백
    /// CN(중국), HK(홍콩), US(미국)
    #[serde(rename = "NATION_CD")]
    pub nation_cd: String,
    /// 거래소코드 (String, 필수)
    /// 전체: 공백
    #[serde(rename = "EXCHANGE_CD")]
    pub exchange_cd: String,
    /// 종목코드 (String, 필수)
    /// 전체: 공백
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 조회일자 (String, 필수)
    /// 전체: 공백
    /// 특정일자(YYYYMMDD) ex. 20240502
    #[serde(rename = "DATA_DT")]
    pub data_dt: String,
    /// 조회시간 (String, 필수)
    /// 전체: 공백
    /// 전체: 공백
    /// 특정시간(HHMMSS) ex. 093500
    #[serde(rename = "DATA_TM")]
    pub data_tm: String,
    /// 다음키 (String, 필수)
    /// 공백 입력
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [해외주식 권리종합] 요청 구조체
/// [해외주식] 시세분석
/// 해외주식 권리종합 [해외주식-050]
/// anno_dt
/// ca_title
/// div_lock_dt
/// pay_dt
/// record_dt
/// validity_dt
/// local_end_dt
/// lock_dt
/// delist_dt
/// redempt_dt
/// early_redempt_dt
/// effective_dt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct RightsByIceRequest {
    /// 국가코드 (String, 필수)
    /// CN:중국 HK:홍콩 US:미국 JP:일본 VN:베트남
    #[serde(rename = "NCOD")]
    pub ncod: String,
    /// 심볼 (String, 필수)
    /// 종목코드
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 일자 시작일 (String, 필수)
    /// 미입력 시, 오늘-3개월
    /// 기간지정 시, 종료일 입력(ex. 20240514)
    ///
    /// ※ 조회기간 기준일 입력시 참고
    /// - 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일
    #[serde(rename = "ST_YMD")]
    pub st_ymd: String,
    /// 일자 종료일 (String, 필수)
    /// 미입력 시, 오늘+3개월
    /// 기간지정 시, 종료일 입력(ex. 20240514)
    ///
    /// ※ 조회기간 기준일 입력시 참고
    /// - 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일
    #[serde(rename = "ED_YMD")]
    pub ed_ymd: String,
}

/// [당사 해외주식담보대출 가능 종목] 요청 구조체
/// [해외주식] 시세분석
/// 당사 해외주식담보대출 가능 종목 [해외주식-051]
/// ovrs_item_name
/// loan_rt
/// mgge_mntn_rt
/// mgge_ensu_rt
/// loan_exec_psbl_yn
/// stff_name
/// erlm_dt
/// tr_mket_name
/// crcy_cd
/// natn_kor_name
/// ovrs_excg_cd
/// loan_psbl_item_num
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ColableByCompanyRequest {
    /// 상품번호 (String, 필수)
    /// ex)AMD
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// 공백
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 조회시작일자 (String, 필수)
    /// 공백
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// 공백
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 조회구분 (String, 필수)
    /// 공백
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 국가코드 (String, 필수)
    /// 840(미국), 344(홍콩), 156(중국)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 조회순서구분 (String, 필수)
    /// 01(이름순), 02(코드순)
    #[serde(rename = "INQR_SQN_DVSN")]
    pub inqr_sqn_dvsn: String,
    /// 비율구분코드 (String, 필수)
    /// 공백
    #[serde(rename = "RT_DVSN_CD")]
    pub rt_dvsn_cd: String,
    /// 비율 (String, 필수)
    /// 공백
    #[serde(rename = "RT")]
    pub rt: String,
    /// 대출가능여부 (String, 필수)
    /// 공백
    #[serde(rename = "LOAN_PSBL_YN")]
    pub loan_psbl_yn: String,
    /// 연속조회검색조건100 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외속보(제목)] 요청 구조체
/// [해외주식] 시세분석
/// 해외속보(제목) [해외주식-055]
/// cntt_usiq_srno
/// news_ofer_entp_code
/// data_dt
/// data_tm
/// hts_pbnt_titl_cntt
/// news_lrdv_code
/// iscd10
/// kor_isnm1
/// kor_isnm2
/// kor_isnm3
/// kor_isnm4
/// kor_isnm5
/// kor_isnm6
/// kor_isnm7
/// kor_isnm8
/// kor_isnm9
/// kor_isnm10
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BrknewsTitleRequest {
    /// 뉴스제공업체코드 (String, 필수)
    /// 뉴스제공업체구분=>0:전체조회
    #[serde(rename = "FID_NEWS_OFER_ENTP_CODE")]
    pub fid_news_ofer_entp_code: String,
    /// 조건시장구분코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 입력종목코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 제목내용 (String, 필수)
    /// 공백
    #[serde(rename = "FID_TITL_CNTT")]
    pub fid_titl_cntt: String,
    /// 입력날짜1 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력시간1 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 순위정렬구분코드 (String, 필수)
    /// 공백
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력일련번호 (String, 필수)
    /// 공백
    #[serde(rename = "FID_INPUT_SRNO")]
    pub fid_input_srno: String,
    /// 조건화면분류코드 (String, 필수)
    /// 화면번호:11801
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
}

/// [해외주식 실시간호가] 요청 구조체
/// [해외주식] 실시간시세
/// 해외주식 실시간호가[실시간-021]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfsasp0Request {
    /// 거래ID (String, 필수)
    /// HDFSASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// R거래소명종목코드 (String, 필수)
    /// <미국 야간거래 - 무료시세>
    /// D+시장구분(3자리)+종목코드
    /// 예) DNASAAPL : D+NAS(나스닥)+AAPL(애플)
    /// [시장구분]
    /// NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    ///
    /// <미국 주간거래>
    /// R+시장구분(3자리)+종목코드
    /// 예) RBAQAAPL : R+BAQ(나스닥)+AAPL(애플)
    /// [시장구분]
    /// BAY : 뉴욕(주간), BAQ : 나스닥(주간). BAA : 아멕스(주간)
    ///
    /// <아시아국가 - 유료시세>
    /// ※ 유료시세 신청시에만 유료시세 수신가능
    /// "포럼 > FAQ > 해외주식 유료시세 신청방법" 참고
    /// R+시장구분(3자리)+종목코드
    /// 예) RHKS00003 : R+HKS(홍콩)+00003(홍콩중화가스)
    /// [시장구분]
    /// TSE : 도쿄, HKS : 홍콩,
    /// SHS : 상해, SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 지연호가(아시아)] 요청 구조체
/// [해외주식] 실시간시세
/// 해외주식 지연호가(아시아)[실시간-008]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfsasp1Request {
    /// 거래ID (String, 필수)
    /// HDFSASP1
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// D거래소명종목코드 (String, 필수)
    /// <아시아국가 - 무료시세>
    /// D+시장구분(3자리)+종목코드
    /// 예) DHKS00003 : D+HKS(홍콩)+00003(홍콩중화가스)
    /// [시장구분]
    /// TSE : 도쿄, HKS : 홍콩,
    /// SHS : 상해, SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 실시간지연체결가] 요청 구조체
/// [해외주식] 실시간시세
/// 해외주식 실시간지연체결가[실시간-007]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfscnt0Request {
    /// 거래ID (String, 필수)
    /// HDFSCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// D거래소명종목코드 (String, 필수)
    /// <미국 야간거래/아시아 주간거래 - 무료시세>
    /// D+시장구분(3자리)+종목코드
    /// 예) DNASAAPL : D+NAS(나스닥)+AAPL(애플)
    /// [시장구분]
    /// NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스 ,
    /// TSE : 도쿄, HKS : 홍콩,
    /// SHS : 상해, SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    ///
    /// <미국 야간거래/아시아 주간거래 - 유료시세>
    /// ※ 유료시세 신청시에만 유료시세 수신가능
    /// "포럼 > FAQ > 해외주식 유료시세 신청방법" 참고
    /// R+시장구분(3자리)+종목코드
    /// 예) RNASAAPL : R+NAS(나스닥)+AAPL(애플)
    /// [시장구분]
    /// NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스 ,
    /// TSE : 도쿄, HKS : 홍콩,
    /// SHS : 상해, SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    ///
    /// <미국 주간거래>
    /// R+시장구분(3자리)+종목코드
    /// 예) RBAQAAPL : R+BAQ(나스닥)+AAPL(애플)
    /// [시장구분]
    /// BAY : 뉴욕(주간), BAQ : 나스닥(주간). BAA : 아멕스(주간)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 실시간체결통보] 요청 구조체
/// [해외주식] 실시간시세
/// 해외주식 실시간체결통보[실시간-009]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Gscni0Request {
    /// 거래ID (String, 필수)
    /// [실전투자]
    /// H0GSCNI0 : 실시간 해외주식 체결통보
    ///
    /// [모의투자]
    /// H0GSCNI9 : 실시간 해외주식 체결통보
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID (String, 필수)
    /// HTSID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 주문] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 주문 [v1_해외선물-001]
/// ORD_DT
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderV3Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외선물FX상품번호 (String, 필수)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// 매도매수구분코드 (String, 필수)
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM청산미결제체결일자 (String, 선택)
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_USTL_CCLD_DT")]
    pub fm_lqd_ustl_ccld_dt: String,
    /// FM청산미결제체결번호 (String, 선택)
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_USTL_CCNO")]
    pub fm_lqd_ustl_ccno: String,
    /// 가격구분코드 (String, 필수)
    /// 1.지정, 2. 시장, 3. STOP, 4 S/L
    #[serde(rename = "PRIC_DVSN_CD")]
    pub pric_dvsn_cd: String,
    /// FMLIMIT주문가격 (String, 필수)
    /// 지정가인 경우 가격 입력
    /// * 시장가, STOP주문인 경우, 빈칸("") 입력
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    /// FMSTOP주문가격 (String, 필수)
    /// STOP 주문 가격 입력
    /// * 시장가, 지정가인 경우, 빈칸("") 입력
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    /// FM주문수량 (String, 필수)
    #[serde(rename = "FM_ORD_QTY")]
    pub fm_ord_qty: String,
    /// FM청산LIMIT주문가격 (String, 선택)
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: String,
    /// FM청산STOP주문가격 (String, 선택)
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: String,
    /// 체결조건코드 (String, 필수)
    /// 일반적으로 6 (EOD, 지정가)
    /// GTD인 경우 5, 시장가인 경우만 2
    #[serde(rename = "CCLD_CNDT_CD")]
    pub ccld_cndt_cd: String,
    /// 복합주문구분코드 (String, 필수)
    /// 0 (hedge청산만 이용)
    #[serde(rename = "CPLX_ORD_DVSN_CD")]
    pub cplx_ord_dvsn_cd: String,
    /// 행사예약주문여부 (String, 필수)
    /// N
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
    /// FM_HEDGE주문화면여부 (String, 필수)
    /// N
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
}

/// [해외선물옵션 정정취소주문] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 정정취소주문 [v1_해외선물-002, 003]
/// ORD_DT
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclV4Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 원주문일자 (String, 필수)
    /// 원 주문 시 출력되는 ORD_DT 값을 입력 (현지거래일)
    #[serde(rename = "ORGN_ORD_DT")]
    pub orgn_ord_dt: String,
    /// 원주문번호 (String, 필수)
    /// 정정/취소시 주문번호(ODNO) 8자리를 문자열처럼 "0"을 포함해서 전송 (원 주문 시 출력된 ODNO 값 활용)
    /// (ex. ORGN_ODNO : 00360686)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// FMLIMIT주문가격 (String, 선택)
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    /// FMSTOP주문가격 (String, 선택)
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    /// FM청산LIMIT주문가격 (String, 선택)
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: String,
    /// FM청산STOP주문가격 (String, 선택)
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: String,
    /// FM_HEDGE주문화면여부 (String, 필수)
    /// N
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    /// FM시장가전환여부 (String, 선택)
    /// OTFM3003U(해외선물옵션주문취소)만 사용
    ///
    /// ※ FM_MKPR_CVSN_YN 항목에 'Y'로 설정하여 취소주문을 접수할 경우, 주문 취소확인이 들어오면 원장에서 시장가주문을 하나 또 내줌
    #[serde(rename = "FM_MKPR_CVSN_YN")]
    pub fm_mkpr_cvsn_yn: String,
}

/// [해외선물옵션 당일주문내역조회] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 당일주문내역조회 [v1_해외선물-004]
/// acnt_prdt_cd
/// ord_dt
/// orgn_ord_dt
/// orgn_odno
/// ovrs_futr_fx_pdno
/// rcit_dvsn_cd
/// sll_buy_dvsn_cd
/// trad_stgy_dvsn_cd
/// bass_pric_type_cd
/// ord_stat_cd
/// fm_ord_qty
/// fm_ord_pric
/// fm_stop_ord_pric
/// rsvn_dvsn
/// fm_ccld_qty
/// fm_ccld_pric
/// fm_ord_rmn_qty
/// ord_grp_name
/// erlm_dtl_dtime
/// ccld_dtl_dtime
/// ord_stfno
/// new_lqd_dvsn_cd
/// fm_lqd_lmt_ord_pric
/// fm_lqd_stop_pric
/// ccld_cndt_cd
/// noti_vald_dt
/// acnt_type_cd
/// fuop_dvsn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcldRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 체결미체결구분 (String, 필수)
    /// 01:전체 / 02:체결 / 03:미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 매도매수구분코드 (String, 필수)
    /// %%:전체 / 01:매도 / 02:매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 선물옵션구분 (String, 필수)
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 미결제내역조회(잔고)] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]
/// acnt_prdt_cd
/// ovrs_futr_fx_pdno
/// prdt_type_cd
/// crcy_cd
/// sll_buy_dvsn_cd
/// fm_ustl_qty
/// fm_ccld_avg_pric
/// fm_now_pric
/// fm_evlu_pfls_amt
/// fm_opt_evlu_amt
/// fm_otp_evlu_pfls_amt
/// fuop_dvsn
/// ecis_rsvn_ord_yn
/// fm_lqd_psbl_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireUnpdRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 선물옵션구분 (String, 필수)
    /// 00: 전체 / 01:선물 / 02: 옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건100 (String, 필수)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외선물옵션 주문가능조회] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 주문가능조회 [v1_해외선물-006]
/// acnt_prdt_cd
/// ovrs_futr_fx_pdno
/// crcy_cd
/// sll_buy_dvsn_cd
/// fm_ustl_qty
/// fm_lqd_psbl_qty
/// fm_new_ord_psbl_qty
/// fm_tot_ord_psbl_qty
/// fm_mkpr_tot_ord_psbl_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsamountV2Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외선물FX상품번호 (String, 필수)
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// 매도매수구분코드 (String, 필수)
    /// 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM주문가격 (String, 필수)
    #[serde(rename = "FM_ORD_PRIC")]
    pub fm_ord_pric: String,
    /// 행사예약주문여부 (String, 필수)
    /// N
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
}

/// [해외선물옵션 기간계좌손익 일별] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 기간계좌손익 일별[해외선물-010]
/// acnt_prdt_cd
/// crcy_cd
/// fm_buy_qty
/// fm_sll_qty
/// fm_lqd_pfls_amt
/// fm_fee
/// fm_net_pfls_amt
/// fm_ustl_buy_qty
/// fm_ustl_sll_qty
/// fm_ustl_evlu_pfls_amt
/// fm_ustl_evlu_pfls_amt2
/// fm_ustl_evlu_pfls_icdc_amt
/// fm_ustl_agrm_amt
/// fm_opt_lqd_amt
/// acnt_prdt_cd
/// ovrs_futr_fx_pdno
/// crcy_cd
/// fm_buy_qty
/// fm_sll_qty
/// fm_lqd_pfls_amt
/// fm_fee
/// fm_net_pfls_amt
/// fm_ustl_buy_qty
/// fm_ustl_sll_qty
/// fm_ustl_evlu_pfls_amt
/// fm_ustl_evlu_pfls_amt2
/// fm_ustl_evlu_pfls_icdc_amt
/// fm_ccld_avg_pric
/// fm_ustl_agrm_amt
/// fm_opt_lqd_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodCcldRequest {
    /// 조회기간FROM일자 (String, 필수)
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// 조회기간TO일자 (String, 필수)
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드 (String, 필수)
    /// '%%% : 전체
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본'
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 전체환산여부 (String, 필수)
    /// N
    #[serde(rename = "WHOL_TRSL_YN")]
    pub whol_trsl_yn: String,
    /// 선물옵션구분 (String, 필수)
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 일별 체결내역] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 일별 체결내역[해외선물-011]
/// fm_tot_ccld_qty
/// fm_tot_futr_agrm_amt
/// fm_tot_opt_agrm_amt
/// fm_fee_smtl
/// ovrs_futr_fx_pdno
/// sll_buy_dvsn_cd
/// fm_ccld_qty
/// fm_ccld_amt
/// fm_futr_ccld_amt
/// fm_opt_ccld_amt
/// crcy_cd
/// fm_fee
/// fm_futr_pure_agrm_amt
/// fm_opt_pure_agrm_amt
/// ccld_dtl_dtime
/// ord_dt
/// ord_mdia_dvsn_name
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldV3Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작일자 (String, 필수)
    /// 시작일자(YYYYMMDD)
    #[serde(rename = "STRT_DT")]
    pub strt_dt: String,
    /// 종료일자 (String, 필수)
    /// 종료일자(YYYYMMDD)
    #[serde(rename = "END_DT")]
    pub end_dt: String,
    /// 선물옵션구분코드 (String, 필수)
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// FM상품군코드 (String, 필수)
    /// 공란(Default)
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// 통화코드 (String, 필수)
    /// %%% : 전체
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본
    /// VND: 베트남
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// FM종목합산여부 (String, 필수)
    /// "N"(Default)
    #[serde(rename = "FM_ITEM_FTNG_YN")]
    pub fm_item_ftng_yn: String,
    /// 매도매수구분코드 (String, 필수)
    /// %%: 전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 예수금현황] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 예수금현황[해외선물-012]
/// fm_nxdy_dncl_amt
/// fm_tot_asst_evlu_amt
/// acnt_prdt_cd
/// crcy_cd
/// resp_dt
/// fm_dnca_rmnd
/// fm_lqd_pfls_amt
/// fm_fee
/// fm_fuop_evlu_pfls_amt
/// fm_rcvb_amt
/// fm_brkg_mgn_amt
/// fm_mntn_mgn_amt
/// fm_add_mgn_amt
/// fm_risk_rt
/// fm_ord_psbl_amt
/// fm_drwg_psbl_amt
/// fm_echm_rqrm_amt
/// fm_drwg_prar_amt
/// fm_opt_tr_chgs
/// fm_opt_icld_asst_evlu_amt
/// fm_opt_evlu_amt
/// fm_crcy_sbst_amt
/// fm_crcy_sbst_use_amt
/// fm_crcy_sbst_stup_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDepositV3Request {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드 (String, 필수)
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본
    /// VND: 베트남
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 조회일자 (String, 필수)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
}

/// [해외선물옵션 일별 주문내역] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 일별 주문내역[해외선물-013]
/// acnt_prdt_cd
/// ord_dt
/// orgn_ord_dt
/// orgn_odno
/// ovrs_futr_fx_pdno
/// rvse_cncl_dvsn_cd
/// sll_buy_dvsn_cd
/// cplx_ord_dvsn_cd
/// pric_dvsn_cd
/// rcit_dvsn_cd
/// fm_ord_qty
/// fm_ord_pric
/// fm_stop_ord_pric
/// ecis_rsvn_ord_yn
/// fm_ccld_qty
/// fm_ccld_pric
/// fm_ord_rmn_qty
/// ord_grp_name
/// rcit_dtl_dtime
/// ccld_dtl_dtime
/// ordr_emp_no
/// rjct_rson_name
/// ccld_cndt_cd
/// trad_end_dt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyOrderRequest {
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
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
    /// 01:전체 / 02:체결 / 03:미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 매도매수구분코드 (String, 필수)
    /// %%전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 선물옵션구분 (String, 필수)
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 기간계좌거래내역] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 기간계좌거래내역[해외선물-014]
/// bass_dt
/// acnt_prdt_cd
/// fm_ldgr_inog_seq
/// acnt_tr_type_name
/// crcy_cd
/// tr_itm_name
/// fm_iofw_amt
/// fm_fee
/// fm_tax_amt
/// fm_sttl_amt
/// fm_bf_dncl_amt
/// fm_dncl_amt
/// fm_rcvb_occr_amt
/// fm_rcvb_pybk_amt
/// ovdu_int_pybk_amt
/// rmks_text
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodTransV2Request {
    /// 조회기간FROM일자 (String, 필수)
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// 조회기간TO일자 (String, 필수)
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// 종합계좌번호 (String, 필수)
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 계좌거래유형코드 (String, 필수)
    /// 1: 전체, 2:입출금 , 3: 결제
    #[serde(rename = "ACNT_TR_TYPE_CD")]
    pub acnt_tr_type_cd: String,
    /// 통화코드 (String, 필수)
    /// '%%% : 전체
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본
    /// VND: 베트남 '
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 연속조회검색조건100 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK100값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100 (String, 필수)
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 비밀번호체크여부 (String, 필수)
    /// 공란(Default)
    #[serde(rename = "PWD_CHK_YN")]
    pub pwd_chk_yn: String,
}

/// [해외선물옵션 증거금상세] 요청 구조체
/// [해외선물옵션] 주문/계좌
/// 해외선물옵션 증거금상세 [해외선물-032]
/// acnt_prdt_cd
/// crcy_cd
/// resp_dt
/// acnt_net_risk_mgna_aply_yn
/// fm_ord_psbl_amt
/// fm_add_mgn_amt
/// fm_brkg_mgn_amt
/// fm_excc_brkg_mgn_amt
/// fm_ustl_mgn_amt
/// fm_mntn_mgn_amt
/// fm_ord_mgn_amt
/// fm_futr_ord_mgn_amt
/// fm_opt_buy_ord_amt
/// fm_opt_sll_ord_mgn_amt
/// fm_opt_buy_ord_mgn_amt
/// fm_ecis_rsvn_mgn_amt
/// fm_span_brkg_mgn_amt
/// fm_span_pric_altr_mgn_amt
/// fm_span_term_sprd_mgn_amt
/// fm_span_buy_opt_min_mgn_amt
/// fm_span_opt_min_mgn_amt
/// fm_span_tot_risk_mgn_amt
/// fm_span_mntn_mgn_amt
/// fm_span_mntn_pric_altr_mgn_amt
/// fm_span_mntn_term_sprd_mgn_amt
/// fm_span_mntn_opt_pric_mgn_amt
/// fm_span_mntn_opt_min_mgn_amt
/// fm_span_mntn_tot_risk_mgn_amt
/// fm_eurx_brkg_mgn_amt
/// fm_eurx_pric_altr_mgn_amt
/// fm_eurx_term_sprd_mgn_amt
/// fm_eurx_opt_pric_mgn_amt
/// fm_eurx_buy_opt_min_mgn_amt
/// fm_eurx_tot_risk_mgn_amt
/// fm_eurx_mntn_mgn_amt
/// fm_eurx_mntn_pric_altr_mgn_amt
/// fm_eurx_mntn_term_sprd_mgn_amt
/// fm_eurx_mntn_opt_pric_mgn_amt
/// fm_eurx_mntn_tot_risk_mgn_amt
/// fm_gnrl_brkg_mgn_amt
/// fm_futr_ustl_mgn_amt
/// fm_sll_opt_ustl_mgn_amt
/// fm_buy_opt_ustl_mgn_amt
/// fm_sprd_ustl_mgn_amt
/// fm_avg_dsct_mgn_amt
/// fm_gnrl_mntn_mgn_amt
/// fm_futr_mntn_mgn_amt
/// fm_opt_mntn_mgn_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarginDetailRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드 (String, 필수)
    /// 'TKR(TOT_KRW), TUS(TOT_USD),
    /// USD(미국달러), HKD(홍콩달러),
    /// CNY(중국위안화), JPY )일본엔화), VND(베트남동)'
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 조회일자 (String, 필수)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
}

/// [해외선물종목현재가] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물종목현재가 [v1_해외선물-009]
/// proc_date
/// high_price
/// proc_time
/// open_price
/// trst_mgn
/// low_price
/// last_price
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// bid_qntt
/// bid_price
/// ask_qntt
/// ask_price
/// prev_price
/// exch_cd
/// crc_cd
/// trd_fr_date
/// expr_date
/// trd_to_date
/// remn_cnt
/// last_qntt
/// tot_ask_qntt
/// tot_bid_qntt
/// tick_size
/// open_date
/// open_time
/// close_date
/// close_time
/// sbsnsdate
/// sttl_price
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceV4Request {
    /// 종목코드 (String, 필수)
    /// ex) CNHU24
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수선물" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물종목상세] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물종목상세 [v1_해외선물-008]
/// exch_cd
/// tick_sz
/// disp_digit
/// trst_mgn
/// sttl_date
/// prev_price
/// crc_cd
/// clas_cd
/// tick_val
/// mrkt_open_date
/// mrkt_open_time
/// mrkt_close_date
/// mrkt_close_time
/// trd_fr_date
/// expr_date
/// trd_to_date
/// remn_cnt
/// stat_tp
/// ctrt_size
/// stl_tp
/// frst_noti_date
/// sprd_srs_cd1
/// sprd_srs_cd2
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct StockDetailRequest {
    /// 종목코드 (String, 필수)
    /// ex) CNHU24
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수선물" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물 호가] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 호가 [해외선물-031]
/// open_price
/// high_price
/// lowp_rice
/// last_price
/// prev_price
/// prev_diff_price
/// prev_diff_rate
/// quot_date
/// quot_time
/// bid_qntt
/// bid_num
/// bid_price
/// ask_qntt
/// ask_num
/// ask_price
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceV3Request {
    /// 종목명 (String, 필수)
    /// 종목코드
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물 분봉조회] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 분봉조회[해외선물-016]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeFuturechartpriceRequest {
    /// 종목코드 (String, 필수)
    /// ex) CNHU24
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수선물" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// ex) 20230823
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 120 (조회갯수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 5 (분간격)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 다음조회(QRY_TP를 P로 입력) 시, 이전 호출의 "output1 > index_key" 기입하여 조회
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(틱)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 체결추이(틱)[해외선물-019]
/// tret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TickCcnlRequest {
    /// 종목코드 (String, 필수)
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(주간)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 체결추이(주간)[해외선물-017]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct WeeklyCcnlRequest {
    /// 종목코드 (String, 필수)
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(일간)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 체결추이(일간)[해외선물-018]
/// tret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyCcnlRequest {
    /// 종목코드 (String, 필수)
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(월간)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 체결추이(월간)[해외선물-020]
/// tret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MonthlyCcnlRequest {
    /// 종목코드 (String, 필수)
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 상품기본정보] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 상품기본정보 [해외선물-023]
/// exch_cd
/// clas_cd
/// crc_cd
/// sttl_price
/// sttl_date
/// trst_mgn
/// disp_digit
/// tick_sz
/// tick_val
/// mrkt_open_date
/// mrkt_open_time
/// mrkt_close_date
/// mrkt_close_time
/// trd_fr_date
/// expr_date
/// trd_to_date
/// remn_cnt
/// stat_tp
/// ctrt_size
/// stl_tp
/// frst_noti_date
/// sub_exch_nm
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchContractDetailRequest {
    /// 요청개수 (String, 필수)
    /// 입력한 코드 개수
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 품목종류 (String, 필수)
    /// 최대 32개 까지 가능
    #[serde(rename = "SRS_CD_01")]
    pub srs_cd_01: String,
    /// 품목종류… (String, 필수)
    #[serde(rename = "SRS_CD_02…")]
    pub srs_cd_02: String,
    /// 품목종류 (String, 필수)
    #[serde(rename = "SRS_CD_32")]
    pub srs_cd_32: String,
}

/// [해외선물 미결제추이] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물 미결제추이 [해외선물-029]
/// row_cnt
/// prod_iscd
/// cftc_iscd
/// bsop_date
/// bidp_spec
/// askp_spec
/// spread_spec
/// bidp_hedge
/// askp_hedge
/// hts_otst_smtn
/// bidp_missing
/// askp_missing
/// bidp_spec_cust
/// askp_spec_cust
/// spread_spec_cust
/// bidp_hedge_cust
/// askp_hedge_cust
/// cust_smtn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorUnpdTrendRequest {
    /// 상품 (String, 필수)
    /// 금리 (GE, ZB, ZF,ZN,ZT), 금속(GC, PA, PL,SI, HG), 농산물(CC, CT,KC, OJ, SB, ZC,ZL, ZM, ZO, ZR, ZS, ZW), 에너지(CL, HO, NG, WBS), 지수(ES, NQ, TF, YM, VX), 축산물(GF, HE, LE), 통화(6A, 6B, 6C, 6E, 6J, 6N, 6S, DX)
    #[serde(rename = "PROD_ISCD")]
    pub prod_iscd: String,
    /// 일자 (String, 필수)
    /// 기준일(ex)20240513)
    #[serde(rename = "BSOP_DATE")]
    pub bsop_date: String,
    /// 구분 (String, 필수)
    /// 0(수량), 1(증감)
    #[serde(rename = "UPMU_GUBUN")]
    pub upmu_gubun: String,
    /// CTS_KEY (String, 필수)
    /// 공백
    #[serde(rename = "CTS_KEY")]
    pub cts_key: String,
}

/// [해외옵션종목현재가] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션종목현재가 [해외선물-035]
/// proc_date
/// proc_time
/// open_price
/// high_price
/// low_price
/// last_price
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// bid_qntt
/// bid_price
/// ask_qntt
/// ask_price
/// trst_mgn
/// exch_cd
/// crc_cd
/// trd_fr_date
/// expr_date
/// trd_to_date
/// remn_cnt
/// last_qntt
/// tot_ask_qntt
/// tot_bid_qntt
/// tick_size
/// open_date
/// open_time
/// close_date
/// close_time
/// sbsnsdate
/// sttl_price
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptPriceRequest {
    /// 종목명 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션종목상세] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션종목상세 [해외선물-034]
/// exch_cd
/// clas_cd
/// crc_cd
/// sttl_price
/// sttl_date
/// trst_mgn
/// disp_digit
/// tick_sz
/// tick_val
/// mrkt_open_date
/// mrkt_open_time
/// mrkt_close_date
/// mrkt_close_time
/// trd_fr_date
/// expr_date
/// trd_to_date
/// remn_cnt
/// stat_tp
/// ctrt_size
/// stl_tp
/// frst_noti_date
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptDetailRequest {
    /// 종목명 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션 호가] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 호가 [해외선물-033]
/// open_price
/// high_price
/// lowp_rice
/// last_price
/// sttl_price
/// prev_diff_price
/// prev_diff_rate
/// quot_date
/// quot_time
/// bid_qntt
/// bid_num
/// bid_price
/// ask_qntt
/// ask_num
/// ask_price
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptAskingPriceRequest {
    /// 종목명 (String, 필수)
    /// 예)OESM24 C5340
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션 분봉조회] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 분봉조회 [해외선물-040]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeOptchartpriceRequest {
    /// 종목코드 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// "" 공란 입력
    /// ※ 날짜 입력해도 처리 안됨
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 120 (최대 120)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 1: 1분봉, 5: 5분봉 ...
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 다음조회(QRY_TP를 P로 입력) 시, 이전 호출의 "output1 > index_key" 기입하여 조회
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(틱)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 체결추이(틱) [해외선물-038]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptTickCcnlRequest {
    /// 종목코드 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// "" 공란 입력
    /// ※ 날짜 입력해도 처리 안됨
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// 공백
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// 다음조회(QRY_TP를 P로 입력) 시, 이전 호출의 "output1 > index_key" 기입하여 조회
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(일간)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 체결추이(일간) [해외선물-037]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptDailyCcnlRequest {
    /// 종목코드 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 100 (최대 119)
    /// ※ QRY_CNT 입력값의 +1 개 데이터가 조회됩니다.
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// "" 공란 입력
    /// ※ 다음조회 불가
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(주간)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 체결추이(주간) [해외선물-036]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptWeeklyCcnlRequest {
    /// 종목코드 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 20 (최대 120)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(월간)] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 체결추이(월간) [해외선물-039]
/// ret_cnt
/// last_n_cnt
/// index_key
/// data_date
/// data_time
/// open_price
/// high_price
/// low_price
/// last_price
/// last_qntt
/// prev_diff_flag
/// prev_diff_price
/// prev_diff_rate
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptMonthlyCcnlRequest {
    /// 종목코드 (String, 필수)
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드 (String, 필수)
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분 (String, 필수)
    /// Q
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수 (String, 필수)
    /// 예) 20 (최대 120)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수 (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY (String, 필수)
    /// "" 공란 입력
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 상품기본정보] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외옵션 상품기본정보 [해외선물-041]
/// exch_cd
/// clas_cd
/// crc_cd
/// sttl_price
/// sttl_date
/// trst_mgn
/// disp_digit
/// tick_sz
/// tick_val
/// mrkt_open_date
/// mrkt_open_time
/// mrkt_close_date
/// mrkt_close_time
/// trd_fr_date
/// expr_date
/// trd_to_date
/// remn_cnt
/// stat_tp
/// ctrt_size
/// stl_tp
/// frst_noti_date
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchOptDetailRequest {
    /// 요청개수 (String, 필수)
    /// 입력한 코드 개수
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 종목코드1 (String, 필수)
    /// SRS_CD_01부터 차례로 입력(ex ) OESU24 C5500
    /// 최대 30개 까지 가능
    #[serde(rename = "SRS_CD_01")]
    pub srs_cd_01: String,
    /// 종목코드2 (String, 필수)
    #[serde(rename = "SRS_CD_02...")]
    pub srs_cd_02: String,
    /// 종목코드30 (String, 필수)
    #[serde(rename = "SRS_CD_30")]
    pub srs_cd_30: String,
}

/// [해외선물옵션 장운영시간] 요청 구조체
/// [해외선물옵션] 기본시세
/// 해외선물옵션 장운영시간 [해외선물-030]
/// fm_pdgr_cd
/// fm_pdgr_name
/// fm_excg_cd
/// fm_excg_name
/// fuop_dvsn_name
/// fm_clas_cd
/// fm_clas_name
/// am_mkmn_strt_tmd
/// am_mkmn_end_tmd
/// pm_mkmn_strt_tmd
/// pm_mkmn_end_tmd
/// mkmn_nxdy_strt_tmd
/// mkmn_nxdy_end_tmd
/// base_mket_strt_tmd
/// base_mket_end_tmd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketTimeV2Request {
    /// FM상품군코드 (String, 필수)
    /// 공백
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// FM클래스코드 (String, 필수)
    /// '공백(전체), 001(통화), 002(금리), 003(지수),
    /// 004(농산물),005(축산물),006(금속),007(에너지)'
    #[serde(rename = "FM_CLAS_CD")]
    pub fm_clas_cd: String,
    /// FM거래소코드 (String, 필수)
    /// 'CME(CME), EUREX(EUREX), HKEx(HKEx),
    /// ICE(ICE), SGX(SGX), OSE(OSE), ASX(ASX),
    /// CBOE(CBOE), MDEX(MDEX), NYSE(NYSE),
    /// BMF(BMF),FTX(FTX), HNX(HNX), ETC(기타)'
    #[serde(rename = "FM_EXCG_CD")]
    pub fm_excg_cd: String,
    /// 옵션여부 (String, 필수)
    /// %(전체), N(선물), Y(옵션)
    #[serde(rename = "OPT_YN")]
    pub opt_yn: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외선물옵션 실시간체결가] 요청 구조체
/// [해외선물옵션]실시간시세
/// 해외선물옵션 실시간체결가[실시간-017]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff020Request {
    /// 거래ID (String, 필수)
    /// HDFFF020
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    ///
    /// ※ CME, SGX 실시간시세 유료시세 신청 필수
    /// "포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)"
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간호가] 요청 구조체
/// [해외선물옵션]실시간시세
/// 해외선물옵션 실시간호가[실시간-018]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff010Request {
    /// 거래ID (String, 필수)
    /// HDFFF010
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드 (String, 필수)
    /// 종목코드
    ///
    /// ※ CME, SGX 실시간시세 유료시세 신청 필수
    /// "포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)"
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간주문내역통보] 요청 구조체
/// [해외선물옵션]실시간시세
/// 해외선물옵션 실시간주문내역통보[실시간-019]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff1C0Request {
    /// 거래ID (String, 필수)
    /// HDFFF1C0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID (String, 필수)
    /// HTSID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간체결내역통보] 요청 구조체
/// [해외선물옵션]실시간시세
/// 해외선물옵션 실시간체결내역통보[실시간-020]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff2C0Request {
    /// 거래ID (String, 필수)
    /// HDFFF2C0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID (String, 필수)
    /// HTSID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [장내채권 매수주문] 요청 구조체
/// [장내채권] 주문/계좌
/// 장내채권 매수주문 [국내주식-124]
/// krx_fwdg_ord_orgno
/// ord_tmd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BuyRequest {
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
    /// SAMT_MKET_PTCI_YN(소액시장참여여부) : N(일반시장) 입력 시 10단위 입력
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 채권주문단가 (String, 필수)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 소액시장참여여부 (String, 필수)
    /// N: 일반시장, Y: 소액시장
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// 채권소매시장여부 (String, 필수)
    /// Y, N
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// 유치자직원번호 (String, 필수)
    /// 공백
    #[serde(rename = "IDCR_STFNO")]
    pub idcr_stfno: String,
    /// 운용사지정주문번호 (String, 필수)
    /// 공백
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    /// Unique key(0)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호 (String, 필수)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [장내채권 매도주문] 요청 구조체
/// [장내채권] 주문/계좌
/// 장내채권 매도주문 [국내주식-123]
/// krx_fwdg_ord_orgno
/// ord_tmd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SellRequest {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문구분 (String, 필수)
    /// '01: 종목별 (매수일자, 매수순번 공백입력)
    /// 02: 일자별 (매수순번: 0 입력)
    /// 03: 체결가별 '
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량2 (String, 필수)
    /// SAMT_MKET_PTCI_YN(소액시장참여여부) : N(일반시장) 입력 시 10단위 입력
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 주문단가 (String, 필수)
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 분리과세여부 (String, 필수)
    /// N: 종합과세, Y:분리과세
    #[serde(rename = "SPRX_YN")]
    pub sprx_yn: String,
    /// 매수일자 (String, 필수)
    /// (잔고조회 참조)
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// 매수순번 (String, 필수)
    /// (잔고조회 참조)
    #[serde(rename = "BUY_SEQ")]
    pub buy_seq: String,
    /// 소액시장참여여부 (String, 필수)
    /// N: 일반시장, Y: 소액시장
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// 매도대행사반대매도여부 (String, 필수)
    /// N
    #[serde(rename = "SLL_AGCO_OPPS_SLL_YN")]
    pub sll_agco_opps_sll_yn: String,
    /// 채권소매시장여부 (String, 필수)
    /// N
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// 운용사지정주문번호 (String, 필수)
    /// 공백
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    /// Unique key(0)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호 (String, 필수)
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [장내채권 정정취소주문] 요청 구조체
/// [장내채권] 주문/계좌
/// 장내채권 정정취소주문 [국내주식-125]
/// krx_fwdg_ord_orgno
/// ord_tmd
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclV5Request {
    /// 종합계좌번호 (String, 필수)
    /// -
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// -
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호 (String, 필수)
    /// -
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호 (String, 필수)
    /// -
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문수량2 (String, 필수)
    /// 원주문이 일반시장 주문일 시 10단위 입력
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 채권주문단가 (String, 필수)
    /// -
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 잔량전부주문여부 (String, 필수)
    /// Y: 잔량전부(주문수량 입력안함),
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 정정취소구분코드 (String, 필수)
    /// 01: 정정, 02: 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 운용사지정주문번호 (String, 필수)
    /// 공백
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드 (String, 필수)
    /// Unique key(0)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호 (String, 필수)
    /// -
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [채권정정취소가능주문조회] 요청 구조체
/// [장내채권] 주문/계좌
/// 채권정정취소가능주문조회 [국내주식-126]
/// rvse_cncl_dvsn_name
/// ord_qty
/// bond_ord_unpr
/// ord_tmd
/// tot_ccld_qty
/// tot_ccld_amt
/// ord_psbl_qty
/// orgn_odno
/// sll_buy_dvsn_cd
/// ord_dvsn_cd
/// mgco_aptm_odno
/// samt_mket_ptci_yn
/// prdt_abrv_name
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblRvsecnclV2Request {
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

/// [장내채권 주문체결내역] 요청 구조체
/// [장내채권] 주문/계좌
/// 장내채권 주문체결내역 [국내주식-127]
/// tot_ord_qty
/// tot_ccld_qty_smtl
/// tot_bond_ccld_avg_unpr
/// tot_ccld_amt_smtl
/// ord_dt
/// orgn_odno
/// ord_dvsn_name
/// sll_buy_dvsn_cd_name
/// shtn_pdno
/// prdt_abrv_name
/// ord_qty
/// bond_ord_unpr
/// ord_tmd
/// tot_ccld_qty
/// bond_avg_unpr
/// tot_ccld_amt
/// loan_dt
/// buy_dt
/// samt_mket_ptci_yn_name
/// sprx_psbl_yn_ifom
/// ord_mdia_dvsn_name
/// sll_buy_dvsn_cd
/// nccs_qty
/// ord_gno_brno
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldV4Request {
    /// 종합계좌번호 (String, 필수)
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자 (String, 필수)
    /// 일자 ~ (1주일 이내)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// ~ 일자 (조회 당일)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 매도매수구분코드 (String, 필수)
    /// %(전체), 01(매도), 02(매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 정렬순서구분 (String, 필수)
    /// 01(주문순서), 02(주문역순)
    #[serde(rename = "SORT_SQN_DVSN")]
    pub sort_sqn_dvsn: String,
    /// 상품번호 (String, 필수)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 미체결여부 (String, 필수)
    /// N(전체), C(체결), Y(미체결)
    #[serde(rename = "NCCS_YN")]
    pub nccs_yn: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [장내채권 잔고조회] 요청 구조체
/// [장내채권] 주문/계좌
/// 장내채권 잔고조회 [국내주식-198]
/// prdt_name
/// buy_dt
/// buy_sqno
/// cblc_qty
/// agrx_qty
/// sprx_qty
/// buy_erng_rt
/// buy_unpr
/// buy_amt
/// ord_psbl_qty
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceV5Request {
    /// 종합계좌번호 (String, 필수)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드 (String, 필수)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회조건 (String, 필수)
    /// 00: 전체, 01: 상품번호단위
    #[serde(rename = "INQR_CNDT")]
    pub inqr_cndt: String,
    /// 상품번호 (String, 필수)
    /// 공백
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매수일자 (String, 필수)
    /// 공백
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// 연속조회검색조건200 (String, 필수)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200 (String, 필수)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [장내채권 매수가능조회] 요청 구조체
/// [장내채권] 주문/계좌
/// 장내채권 매수가능조회 [국내주식-199]
/// ord_psbl_cash
/// ord_psbl_sbst
/// ruse_psbl_amt
/// bond_ord_unpr2
/// buy_psbl_amt
/// buy_psbl_qty
/// cma_evlu_amt
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderV4Request {
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
    /// Y(소액시장) N (일반시장)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
}

/// [장내채권현재가(호가)] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권현재가(호가) [국내주식-132]
/// aspr_acpt_hour
/// bond_askp1
/// bond_askp2
/// bond_askp3
/// bond_askp4
/// bond_askp5
/// bond_bidp1
/// bond_bidp2
/// bond_bidp3
/// bond_bidp4
/// bond_bidp5
/// askp_rsqn1
/// askp_rsqn2
/// askp_rsqn3
/// askp_rsqn4
/// askp_rsqn5
/// bidp_rsqn1
/// bidp_rsqn2
/// bidp_rsqn3
/// bidp_rsqn4
/// bidp_rsqn5
/// total_askp_rsqn
/// total_bidp_rsqn
/// ntby_aspr_rsqn
/// seln_ernn_rate1
/// seln_ernn_rate2
/// seln_ernn_rate3
/// seln_ernn_rate4
/// seln_ernn_rate5
/// shnu_ernn_rate1
/// shnu_ernn_rate2
/// shnu_ernn_rate3
/// shnu_ernn_rate4
/// shnu_ernn_rate5
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceV4Request {
    /// 조건 시장 분류 코드 (String, 필수)
    /// B: 장내
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 채권종목코드
    /// ex. KR2088012A16
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(시세)] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권현재가(시세) [국내주식-200]
/// stnd_iscd
/// hts_kor_isnm
/// bond_prpr
/// prdy_vrss_sign
/// bond_prdy_vrss
/// prdy_ctrt
/// acml_vol
/// bond_prdy_clpr
/// bond_oprc
/// bond_hgpr
/// bond_lwpr
/// ernn_rate
/// oprc_ert
/// hgpr_ert
/// lwpr_ert
/// bond_mxpr
/// bond_llam
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceV5Request {
    /// 조건시장분류코드 (String, 필수)
    /// B (업종코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 채권종목코드(ex KR2033022D33)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(체결)] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권현재가(체결) [국내주식-201]
/// stck_cntg_hour
/// bond_prpr
/// bond_prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// cntg_vol
/// acml_vol
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlV5Request {
    /// 조건시장분류코드 (String, 필수)
    /// B (업종코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 채권종목코드(ex KR2033022D33)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(일별)] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권현재가(일별) [국내주식-202]
/// stck_bsop_date
/// bond_prpr
/// bond_prdy_vrss
/// prdy_vrss_sign
/// prdy_ctrt
/// acml_vol
/// bond_oprc
/// bond_hgpr
/// bond_lwpr
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyPriceV2Request {
    /// 조건시장분류코드 (String, 필수)
    /// B (업종코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드 (String, 필수)
    /// 채권종목코드(ex KR2033022D33)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권 기간별시세(일)] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권 기간별시세(일) [국내주식-159]
/// stck_bsop_date
/// bond_oprc
/// bond_hgpr
/// bond_lwpr
/// bond_prpr
/// acml_vol
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyItemchartpriceV2Request {
    /// 조건 시장 구분 코드 (String, 필수)
    /// Unique key(B)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드 (String, 필수)
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권 평균단가조회] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권 평균단가조회 [국내주식-158]
/// evlu_dt
/// prdt_type_cd
/// prdt_name
/// kis_unpr
/// kbp_unpr
/// nice_evlu_unpr
/// fnp_unpr
/// avg_evlu_unpr
/// kis_crdt_grad_text
/// kbp_crdt_grad_text
/// nice_crdt_grad_text
/// fnp_crdt_grad_text
/// chng_yn
/// kis_erng_rt
/// kbp_erng_rt
/// nice_evlu_erng_rt
/// fnp_erng_rt
/// avg_evlu_erng_rt
/// kis_rf_unpr
/// kbp_rf_unpr
/// nice_evlu_rf_unpr
/// avg_evlu_rf_unpr
/// evlu_dt
/// prdt_type_cd
/// prdt_name
/// kis_evlu_amt
/// kbp_evlu_amt
/// nice_evlu_amt
/// fnp_evlu_amt
/// avg_evlu_amt
/// chng_yn
/// evlu_dt
/// prdt_type_cd
/// prdt_name
/// kis_crcy_cd
/// kis_evlu_unit_pric
/// kis_evlu_pric
/// kbp_crcy_cd
/// kbp_evlu_unit_pric
/// kbp_evlu_pric
/// nice_crcy_cd
/// nice_evlu_unit_pric
/// nice_evlu_pric
/// avg_evlu_unit_pric
/// avg_evlu_pric
/// chng_yn
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AvgUnitRequest {
    /// 조회시작일자 (String, 필수)
    /// 일자 ~
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자 (String, 필수)
    /// ~ 일자
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 상품번호 (String, 필수)
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// Unique key(302)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 검증종류코드 (String, 필수)
    /// Unique key(00)
    #[serde(rename = "VRFC_KIND_CD")]
    pub vrfc_kind_cd: String,
    /// 연속조회키30 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_NK30")]
    pub ctx_area_nk30: String,
    /// 연속조회검색조건100 (String, 필수)
    /// 공백
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [장내채권 발행정보] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권 발행정보[국내주식-156]
/// prdt_type_cd
/// prdt_name
/// prdt_eng_name
/// ivst_heed_prdt_yn
/// exts_yn
/// bond_clsf_cd
/// bond_clsf_kor_name
/// int_mned_dvsn_cd
/// rvnu_shap_cd
/// issu_amt
/// lstg_rmnd
/// int_dfrm_mcnt
/// bond_int_dfrm_mthd_cd
/// splt_rdpt_rcnt
/// prca_dfmt_term_mcnt
/// int_anap_dvsn_cd
/// bond_rght_dvsn_cd
/// prdt_pclc_text
/// prdt_abrv_name
/// prdt_eng_abrv_name
/// sprx_psbl_yn
/// pbff_pplc_ofrg_mthd_cd
/// cmco_cd
/// issu_istt_cd
/// issu_istt_name
/// pnia_dfrm_agcy_istt_cd
/// dsct_ec_rt
/// srfc_inrt
/// expd_rdpt_rt
/// expd_asrc_erng_rt
/// bond_grte_istt_name
/// int_dfrm_day_type_cd
/// ksd_int_calc_unit_cd
/// int_wunt_uder_prcs_dvsn_cd
/// rvnu_dt
/// issu_dt
/// lstg_dt
/// expd_dt
/// rdpt_dt
/// sbst_pric
/// rgbf_int_dfrm_dt
/// nxtm_int_dfrm_dt
/// frst_int_dfrm_dt
/// ecis_pric
/// rght_stck_std_pdno
/// ecis_opng_dt
/// ecis_end_dt
/// bond_rvnu_mthd_cd
/// oprt_stfno
/// oprt_stff_name
/// rgbf_int_dfrm_wday
/// nxtm_int_dfrm_wday
/// kis_crdt_grad_text
/// kbp_crdt_grad_text
/// nice_crdt_grad_text
/// fnp_crdt_grad_text
/// dpsi_psbl_yn
/// pnia_int_calc_unpr
/// prcm_idx_bond_yn
/// expd_exts_srdp_rcnt
/// expd_exts_srdp_rt
/// loan_psbl_yn
/// grte_dvsn_cd
/// fnrr_rank_dvsn_cd
/// krx_lstg_abol_dvsn_cd
/// asst_rqdi_dvsn_cd
/// opcb_dvsn_cd
/// crfd_item_yn
/// crfd_item_rstc_cclc_dt
/// bond_nmpr_unit_pric
/// ivst_heed_bond_dvsn_name
/// add_erng_rt
/// add_erng_rt_aply_dt
/// bond_tr_stop_dvsn_cd
/// ivst_heed_bond_dvsn_cd
/// pclr_cndt_text
/// hbbd_yn
/// cdtl_cptl_scty_type_cd
/// elec_scty_yn
/// sq1_clop_ecis_opng_dt
/// frst_erlm_stfno
/// frst_erlm_dt
/// frst_erlm_tmd
/// tlg_rcvg_dtl_dtime
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IssueInfoRequest {
    /// 사용자권한정보 (String, 필수)
    /// 채권 종목번호(ex. KR6449111CB8)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 거래소코드 (String, 필수)
    /// Unique key(302)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [장내채권 기본조회] 요청 구조체
/// [장내채권] 기본시세
/// 장내채권 기본조회 [국내주식-129]
/// prdt_type_cd
/// ksd_bond_item_name
/// ksd_bond_item_eng_name
/// ksd_bond_lstg_type_cd
/// ksd_ofrg_dvsn_cd
/// ksd_bond_int_dfrm_dvsn_cd
/// issu_dt
/// rdpt_dt
/// rvnu_dt
/// iso_crcy_cd
/// mdwy_rdpt_dt
/// ksd_rcvg_bond_dsct_rt
/// ksd_rcvg_bond_srfc_inrt
/// bond_expd_rdpt_rt
/// ksd_prca_rdpt_mthd_cd
/// int_caltm_mcnt
/// ksd_int_calc_unit_cd
/// uval_cut_dvsn_cd
/// uval_cut_dcpt_dgit
/// ksd_dydv_caltm_aply_dvsn_cd
/// dydv_calc_dcnt
/// bond_expd_asrc_erng_rt
/// padf_plac_hdof_name
/// lstg_dt
/// lstg_abol_dt
/// ksd_bond_issu_mthd_cd
/// laps_indf_yn
/// ksd_lhdy_pnia_dfrm_mthd_cd
/// frst_int_dfrm_dt
/// ksd_prcm_lnkg_gvbd_yn
/// dpsi_end_dt
/// dpsi_strt_dt
/// dpsi_psbl_yn
/// atyp_rdpt_bond_erlm_yn
/// dshn_occr_yn
/// expd_exts_yn
/// pclr_ptcr_text
/// dpsi_psbl_excp_stat_cd
/// expd_exts_srdp_rcnt
/// expd_exts_srdp_rt
/// expd_rdpt_rt
/// expd_asrc_erng_rt
/// bond_int_dfrm_mthd_cd
/// int_dfrm_day_type_cd
/// prca_dfmt_term_mcnt
/// splt_rdpt_rcnt
/// rgbf_int_dfrm_dt
/// nxtm_int_dfrm_dt
/// sprx_psbl_yn
/// ictx_rt_dvsn_cd
/// bond_clsf_cd
/// bond_clsf_kor_name
/// int_mned_dvsn_cd
/// pnia_int_calc_unpr
/// frn_intr
/// aply_day_prcm_idx_lnkg_cefc
/// ksd_expd_dydv_calc_bass_cd
/// expd_dydv_calc_dcnt
/// ksd_cbbw_dvsn_cd
/// crfd_item_yn
/// pnia_bank_ofdy_dfrm_mthd_cd
/// qib_yn
/// qib_cclc_dt
/// csbd_yn
/// csbd_cclc_dt
/// ksd_opcb_yn
/// ksd_sodn_yn
/// ksd_rqdi_scty_yn
/// elec_scty_yn
/// rght_ecis_mbdy_dvsn_cd
/// int_rkng_mthd_dvsn_cd
/// ofrg_dvsn_cd
/// ksd_tot_issu_amt
/// next_indf_chk_ecls_yn
/// ksd_bond_intr_dvsn_cd
/// ksd_inrt_aply_dvsn_cd
/// krx_issu_istt_cd
/// ksd_indf_frqc_uder_calc_cd
/// ksd_indf_frqc_uder_calc_dcnt
/// tlg_rcvg_dtl_dtime
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchBondInfoRequest {
    /// 상품번호 (String, 필수)
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드 (String, 필수)
    /// Unique key(302)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [일반채권 실시간체결가] 요청 구조체
/// [장내채권] 실시간시세
/// 일반채권 실시간체결가 [실시간-052]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Bjcnt0Request {
    /// 거래ID (String, 필수)
    /// H0BJCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 채권 종목코드 (ex. KR103502GA34)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [일반채권 실시간호가] 요청 구조체
/// [장내채권] 실시간시세
/// 일반채권 실시간호가 [실시간-053]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Bjasp0Request {
    /// 거래ID (String, 필수)
    /// H0BJCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 채권 종목코드 (ex. KR103502GA34)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [채권지수 실시간체결가] 요청 구조체
/// [장내채권] 실시간시세
/// 채권지수 실시간체결가 [실시간-060]
/// true friend 한국투자 Open API
/// KIS Developers COPYRIGHTS
/// 2021-12-29 11:22:33
/// 0.0.0.0
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Bicnt0Request {
    /// 거래ID (String, 필수)
    /// H0BICNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값 (String, 필수)
    /// 채권 종목코드 (ex. KR103502GA34)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}
