#![allow(clippy::doc_lazy_continuation)]
use serde::{Deserialize, Serialize};

/// [접근토큰발급(P)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TokenPRequest {
    /// 권한부여 Type
    /// client_credentials
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// 앱키
    /// 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 앱시크릿키
    /// 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
}

/// [접근토큰폐기(P)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct RevokePRequest {
    /// 고객 앱Key
    /// 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 고객 앱Secret
    /// 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appsecret")]
    pub appsecret: String,
    /// 접근토큰
    /// OAuth 토큰이 필요한 API 경우 발급한 Access token
    /// 일반고객(Access token 유효기간 1일, OAuth 2.0의 Client Credentials Grant 절차를 준용)
    /// 법인(Access token 유효기간 3개월, Refresh token 유효기간 1년, OAuth 2.0의 Authorization Code Grant 절차를 준용)
    #[serde(rename = "token")]
    pub token: String,
}

/// [Hashkey] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct HashkeyRequest {
    /// 요청값
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ApprovalRequest {
    /// 권한부여타입
    /// "client_credentials"
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    /// 앱키
    /// 한국투자증권 홈페이지에서 발급받은 appkey (절대 노출되지 않도록 주의해주세요.)
    #[serde(rename = "appkey")]
    pub appkey: String,
    /// 시크릿키
    /// 한국투자증권 홈페이지에서 발급받은 appsecret (절대 노출되지 않도록 주의해주세요.)
    /// * 주의 : appsecret와 secretkey는 동일하오니 착오없으시기 바랍니다. (용어가 다른점 양해 부탁드립니다.)
    #[serde(rename = "secretkey")]
    pub secretkey: String,
}

/// [주식주문(현금)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderCashRequest {
    /// 종합계좌번호
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 상품유형코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// 종목코드(6자리) , ETN의 경우 7자리 입력
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형 (매도주문 시)
    /// 01@일반매도
    /// 02@임의매매
    /// 05@대차매도
    /// → 미입력시 01 일반매도로 진행
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 주문구분
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
    /// 주문수량
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    /// 주문단가
    /// 시장가 등 주문시, "0"으로 입력
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 조건가격
    /// 스탑지정가호가 주문 (ORD_DVSN이 22) 사용 시에만 필수
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
    /// 거래소ID구분코드
    /// 한국거래소 : KRX
    /// 대체거래소 (넥스트레이드) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// → 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
}

/// [주식주문(신용)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderCreditRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// 종목코드(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형
    /// 공란 입력
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 신용유형
    /// [매도] 22 : 유통대주신규, 24 : 자기대주신규, 25 : 자기융자상환, 27 : 유통융자상환
    /// [매수] 21 : 자기융자신규, 23 : 유통융자신규 , 26 : 유통대주상환, 28 : 자기대주상환
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// 대출일자
    /// [신용매수]
    /// 신규 대출로, 오늘날짜(yyyyMMdd)) 입력
    ///
    /// [신용매도]
    /// 매도할 종목의 대출일자(yyyyMMdd)) 입력
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 주문구분
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
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    /// 1주당 가격
    /// * 장전 시간외, 장후 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 예약주문여부
    /// 정규 증권시장이 열리지 않는 시간 (15:10분 ~ 익일 7:30분) 에 주문을 미리 설정 하여 다음 영업일 또는 설정한 기간 동안 아침 동시 호가에 주문하는 것
    /// Y : 예약주문
    /// N : 신용주문
    #[serde(rename = "RSVN_ORD_YN")]
    pub rsvn_ord_yn: String,
    /// 비상주문여부
    #[serde(rename = "EMGC_ORD_YN")]
    pub emgc_ord_yn: String,
    /// 프로그램매매구분
    #[serde(rename = "PGTR_DVSN")]
    pub pgtr_dvsn: String,
    /// 운용사지정주문번호
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 대량거래협상상세번호
    #[serde(rename = "LQTY_TR_NGTN_DTL_NO")]
    pub lqty_tr_ngtn_dtl_no: String,
    /// 대량거래협정번호
    #[serde(rename = "LQTY_TR_AGMT_NO")]
    pub lqty_tr_agmt_no: String,
    /// 대량거래협상자Id
    #[serde(rename = "LQTY_TR_NGTN_ID")]
    pub lqty_tr_ngtn_id: String,
    /// LP주문여부
    #[serde(rename = "LP_ORD_YN")]
    pub lp_ord_yn: String,
    /// 매체주문번호
    #[serde(rename = "MDIA_ODNO")]
    pub mdia_odno: String,
    /// 주문서버구분코드
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 프로그램호가신고구분코드
    #[serde(rename = "PGM_NMPR_STMT_DVSN_CD")]
    pub pgm_nmpr_stmt_dvsn_cd: String,
    /// 반대매매선정사유코드
    #[serde(rename = "CVRG_SLCT_RSON_CD")]
    pub cvrg_slct_rson_cd: String,
    /// 반대매매순번
    #[serde(rename = "CVRG_SEQ")]
    pub cvrg_seq: String,
    /// 거래소ID구분코드
    /// 한국거래소 : KRX
    /// 대체거래소 (넥스트레이드) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// → 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
    /// 조건가격
    /// 스탑지정가호가에서 사용
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
}

/// [주식주문(정정취소)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclRequest {
    /// 종합계좌번호
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 상품유형코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 한국거래소전송주문조직번호
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    /// 원주문번호
    /// 원주문번호
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문구분
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
    /// 정정취소구분코드
    /// 01@정정
    /// 02@취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    /// 주문단가
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 잔량전부주문여부
    /// 'Y@전량
    /// N@일부'
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 조건가격
    /// 스탑지정가호가에서 사용
    #[serde(rename = "CNDT_PRIC")]
    pub cndt_pric: String,
    /// 거래소ID구분코드
    /// 한국거래소 : KRX
    /// 대체거래소 (넥스트레이드) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// → 미입력시 KRX로 진행되며, 모의투자는 KRX만 가능
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
}

/// [주식정정취소가능주문조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblRvsecnclRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 연속조회검색조건100
    /// '공란 : 최초 조회시는
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// '공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 조회구분1
    /// '0 주문
    /// 1 종목'
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 조회구분2
    /// '0 전체
    /// 1 매도
    /// 2 매수'
    #[serde(rename = "INQR_DVSN_2")]
    pub inqr_dvsn_2: String,
}

/// [주식일별주문체결조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자
    /// YYYYMMDD
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// YYYYMMDD
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 매도매수구분코드
    /// 00 : 전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 상품번호
    /// 종목번호(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문채번지점번호
    /// 주문시 한국투자증권 시스템에서 지정된 영업점코드
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호
    /// 주문시 한국투자증권 시스템에서 채번된 주문번호
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 체결구분
    /// '00 전체
    /// 01 체결
    /// 02 미체결'
    #[serde(rename = "CCLD_DVSN")]
    pub ccld_dvsn: String,
    /// 조회구분
    /// '00 역순
    /// 01 정순'
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 조회구분1
    /// '없음: 전체
    /// 1: ELW
    /// 2: 프리보드'
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 조회구분3
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
    /// 거래소ID구분코드
    /// 한국거래소 : KRX
    /// 대체거래소 (NXT) : NXT
    /// SOR (Smart Order Routing) : SOR
    /// ALL : 전체
    /// ※ 모의투자는 KRX만 제공
    #[serde(rename = "EXCG_ID_DVSN_CD")]
    pub excg_id_dvsn_cd: String,
    /// 연속조회검색조건100
    /// '공란 : 최초 조회시는
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// '공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)'
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [주식잔고조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시간외단일가, 거래소여부
    /// N : 기본값,
    /// Y : 시간외단일가,
    /// X : NXT 정규장 (프리마켓, 메인, 애프터마켓)
    /// ※ NXT 선택 시 : NXT 거래종목만 시세 등 정보가 NXT 기준으로 변동됩니다. KRX 종목들은 그대로 유지
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// 오프라인여부
    /// 공란(Default)
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// 조회구분
    /// 01 : 대출일별
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 단가구분
    /// 01 : 기본값
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// 펀드결제분포함여부
    /// N : 포함하지 않음
    /// Y : 포함
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// 융자금액자동상환여부
    /// N : 기본값
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// 처리구분
    /// 00 : 전일매매포함
    /// 01 : 전일매매미포함
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// 연속조회검색조건100
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [매수가능조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// 종목번호(6자리)
    /// * PDNO, ORD_UNPR 공란 입력 시, 매수수량 없이 매수금액만 조회됨
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문단가
    /// 1주당 가격
    /// * 시장가(ORD_DVSN:01)로 조회 시, 공란으로 입력
    /// * PDNO, ORD_UNPR 공란 입력 시, 매수수량 없이 매수금액만 조회됨
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 주문구분
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
    /// CMA평가금액포함여부
    /// Y : 포함
    /// N : 포함하지 않음
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 해외포함여부
    /// Y : 포함
    /// N : 포함하지 않음
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
}

/// [매도가능수량조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblSellRequest {
    /// 종합계좌번호
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목번호
    /// 보유종목 코드 ex)000660
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [신용매수가능조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCreditPsamountRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// 종목코드(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문단가
    /// 1주당 가격
    /// * 장전 시간외, 장후 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 주문구분
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
    /// 신용유형
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
    /// CMA평가금액포함여부
    /// Y/N
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 해외포함여부
    /// Y/N
    #[serde(rename = "OVRS_ICLD_YN")]
    pub ovrs_icld_yn: String,
}

/// [주식예약주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드(6자리)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량
    /// 주문주식수
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    /// 1주당 가격
    /// * 장전 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 매도매수구분코드
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문구분코드
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 05 : 장전 시간외
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 주문대상잔고구분코드
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
    /// 대출일자
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 예약주문종료일자
    /// (YYYYMMDD) 현재 일자보다 이후로 설정해야 함
    /// * RSVN_ORD_END_DT(예약주문종료일자)를 안 넣으면 다음날 주문처리되고 예약주문은 종료됨
    /// * RSVN_ORD_END_DT(예약주문종료일자)는 익영업일부터 달력일 기준으로 공휴일 포함하여 최대 30일이 되는 일자까지 입력 가능
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 대여일자
    #[serde(rename = "LDNG_DT")]
    pub ldng_dt: String,
}

/// [주식예약주문정정취소] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvRvsecnclRequest {
    /// 종합계좌번호
    /// [정정/취소] 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// [정정/취소] 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종목코드(6자리)
    /// [정정]
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량
    /// [정정] 주문주식수
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문단가
    /// [정정] 1주당 가격
    /// * 장전 시간외, 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력 권고
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 매도매수구분코드
    /// [정정]
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문구분코드
    /// [정정]
    /// 00 : 지정가
    /// 01 : 시장가
    /// 02 : 조건부지정가
    /// 05 : 장전 시간외
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 주문대상잔고구분코드
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
    /// 대출일자
    /// [정정]
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    /// 예약주문종료일자
    /// [정정]
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 연락전화번호
    /// [정정]
    #[serde(rename = "CTAL_TLNO")]
    pub ctal_tlno: String,
    /// 예약주문순번
    /// [정정/취소]
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// 예약주문조직번호
    /// [정정/취소]
    #[serde(rename = "RSVN_ORD_ORGNO")]
    pub rsvn_ord_orgno: String,
    /// 예약주문주문일자
    /// [정정/취소]
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
}

/// [주식예약주문조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvCcnlRequest {
    /// 예약주문시작일자
    #[serde(rename = "RSVN_ORD_ORD_DT")]
    pub rsvn_ord_ord_dt: String,
    /// 예약주문종료일자
    #[serde(rename = "RSVN_ORD_END_DT")]
    pub rsvn_ord_end_dt: String,
    /// 예약주문순번
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    /// 단말매체종류코드
    /// "00" 입력
    #[serde(rename = "TMNL_MDIA_KIND_CD")]
    pub tmnl_mdia_kind_cd: String,
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 처리구분코드
    /// 0: 전체
    /// 1: 처리내역
    /// 2: 미처리내역
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
    /// 취소여부
    /// "Y" 유효한 주문만 조회
    #[serde(rename = "CNCL_YN")]
    pub cncl_yn: String,
    /// 상품번호
    /// 종목코드(6자리) (공백 입력 시 전체 조회)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 연속조회검색조건200
    /// 다음 페이지 조회시 사용
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 다음 페이지 조회시 사용
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [퇴직연금 체결기준잔고] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePresentBalanceRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 사용자구분코드
    /// 00
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// 연속조회검색조건100
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 처리구분코드
    /// 00 : 보유 주식 전체 조회
    /// 01 : 보유 주식 중 0주 주식 숨김
    #[serde(rename = "PRCS_DVSN_CD")]
    pub prcs_dvsn_cd: String,
}

/// [퇴직연금 미체결내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldNextRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 사용자구분코드
    /// %%
    #[serde(rename = "USER_DVSN_CD")]
    pub user_dvsn_cd: String,
    /// 매도매수구분코드
    /// 00 : 전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분
    /// %% : 전체 / 01 : 체결 / 02 : 미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 조회구분3
    /// 00 : 전체
    #[serde(rename = "INQR_DVSN_3")]
    pub inqr_dvsn_3: String,
    /// 연속조회검색조건100
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [퇴직연금 매수가능조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderNextRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 적립금구분코드
    /// 00
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// CMA평가금액포함여부
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 주문구분
    /// 00 : 지정가 / 01 : 시장가
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 주문단가
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
}

/// [퇴직연금 예수금조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDepositRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 적립금구분코드
    /// 00
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
}

/// [퇴직연금 잔고조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceNextRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 29
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 적립금구분코드
    /// 00
    #[serde(rename = "ACCA_DVSN_CD")]
    pub acca_dvsn_cd: String,
    /// 조회구분
    /// 00 : 전체
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 연속조회검색조건100
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [주식잔고조회_실현손익] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceRlzPlRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시간외단일가여부
    /// 'N : 기본값
    /// Y : 시간외단일가'
    #[serde(rename = "AFHR_FLPR_YN")]
    pub afhr_flpr_yn: String,
    /// 오프라인여부
    /// 공란
    #[serde(rename = "OFL_YN")]
    pub ofl_yn: String,
    /// 조회구분
    /// 00 : 전체
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 단가구분
    /// 01 : 기본값
    #[serde(rename = "UNPR_DVSN")]
    pub unpr_dvsn: String,
    /// 펀드결제포함여부
    /// N : 포함하지 않음
    /// Y : 포함
    #[serde(rename = "FUND_STTL_ICLD_YN")]
    pub fund_sttl_icld_yn: String,
    /// 융자금액자동상환여부
    /// N : 기본값
    #[serde(rename = "FNCG_AMT_AUTO_RDPT_YN")]
    pub fncg_amt_auto_rdpt_yn: String,
    /// PRCS_DVSN
    /// 00 : 전일매매포함
    /// 01 : 전일매매미포함
    #[serde(rename = "PRCS_DVSN")]
    pub prcs_dvsn: String,
    /// 비용포함여부
    #[serde(rename = "COST_ICLD_YN")]
    pub cost_icld_yn: String,
    /// 연속조회검색조건100
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100 값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [투자계좌자산현황조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAccountBalanceRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회구분1
    /// 공백입력
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 기준가이전일자적용여부
    /// 공백입력
    #[serde(rename = "BSPR_BF_DT_APLY_YN")]
    pub bspr_bf_dt_aply_yn: String,
}

/// [기간별손익일별합산조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodProfitRequest {
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 조회시작일자
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 상품번호
    /// ""공란입력 시, 전체
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 연속조회키100
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 조회종료일자
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 정렬구분
    /// 00: 최근 순, 01: 과거 순, 02: 최근 순
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// 조회구분
    /// 00 입력
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 잔고구분
    /// 00: 전체
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// 연속조회검색조건100
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [기간별매매손익현황조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodTradeProfitRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 정렬구분
    /// 00: 최근 순, 01: 과거 순, 02: 최근 순
    #[serde(rename = "SORT_DVSN")]
    pub sort_dvsn: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// ""공란입력 시, 전체
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 조회시작일자
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 연속조회키100
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 잔고구분
    /// 00: 전체
    #[serde(rename = "CBLC_DVSN")]
    pub cblc_dvsn: String,
    /// 연속조회검색조건100
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [주식통합증거금 현황] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntgrMarginRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// CMA평가금액포함여부
    /// N 입력
    #[serde(rename = "CMA_EVLU_AMT_ICLD_YN")]
    pub cma_evlu_amt_icld_yn: String,
    /// 원화외화구분코드
    /// 01(외화기준),02(원화기준)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 선도환계약외화구분코드
    /// 01(외화기준),02(원화기준)
    #[serde(rename = "FWEX_CTRT_FRCR_DVSN_CD")]
    pub fwex_ctrt_frcr_dvsn_cd: String,
}

/// [기간별계좌권리현황조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PeriodRightsRequest {
    /// 조회구분
    /// 03 입력
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 고객실명확인번호25
    /// 공란
    #[serde(rename = "CUST_RNCNO25")]
    pub cust_rncno25: String,
    /// 홈넷ID
    /// 공란
    #[serde(rename = "HMID")]
    pub hmid: String,
    /// 종합계좌번호
    /// 계좌번호 8자리 입력 (ex.12345678)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 상품계좌번호 2자리 입력(ex. 01 or 22)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자
    /// 조회시작일자(YYYYMMDD)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// 조회종료일자(YYYYMMDD)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 권리유형코드
    /// 공란
    #[serde(rename = "RGHT_TYPE_CD")]
    pub rght_type_cd: String,
    /// 상품번호
    /// 공란
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// 공란
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 연속조회키100
    /// 다음조회시 입력
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 연속조회검색조건100
    /// 다음조회시 입력
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [주식현재가 시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자) // ETN은 종목코드 6자리 앞에 Q 입력 필수
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 시세2] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePrice2Request {
    /// FID 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 000660
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 일자별] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyPriceRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기간 분류 코드
    /// 'D : (일)최근 30거래일
    /// W : (주)최근 30주
    /// M : (월)최근 30개월'
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 수정주가 원주가 가격
    /// '0 : 수정주가미반영
    /// 1 : 수정주가반영
    /// * 수정주가는 액면분할/액면병합 등 권리 발생 시 과거 시세를 현재 주가에 맞게 보정한 가격'
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
}

/// [주식현재가 호가/예상체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceExpCcnRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 투자자] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireInvestorRequest {
    /// 조건 시장 분류 코드
    /// J : KRX, NX : NXT, UN : 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 회원사] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireMemberRequest {
    /// FID 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내주식기간별시세(일/주/월/년)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyItemchartpriceRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜 1
    /// 조회 시작일자
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 날짜 2
    /// 조회 종료일자 (최대 100개)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 기간분류코드
    /// D:일봉 W:주봉, M:월봉, Y:년봉
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 수정주가 원주가 가격 여부
    /// 0:수정주가 1:원주가
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
}

/// [주식당일분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeItemchartpriceRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1
    /// 입력시간
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 과거 데이터 포함 여부
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// 기타 구분 코드
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [주식일별분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeDailychartpriceRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1
    /// 입력 시간(ex 13시 130000)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 날짜1
    /// 입력 날짜(20241023)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 과거 데이터 포함 여부
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// 허봉 포함 여부
    /// 공백 필수 입력
    #[serde(rename = "FID_FAKE_TICK_INCU_YN")]
    pub fid_fake_tick_incu_yn: String,
}

/// [주식현재가 당일시간대별체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeItemconclusionRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 시간1
    /// 입력시간
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [주식현재가 시간외일자별주가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyOvertimepriceRequest {
    /// FID 조건 시장 분류 코드
    /// J : 주식, ETF, ETN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [주식현재가 시간외시간별체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeOvertimeconclusionRequest {
    /// 조건 시장 분류 코드
    /// J : 주식, ETF, ETN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간 구분 코드
    /// 1 : 시간외 (Default)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
}

/// [국내주식 시간외현재가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireOvertimePriceRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내주식 시간외호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireOvertimeAskingPriceRequest {
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 장마감 예상체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpClosingPriceRequest {
    /// 순위 정렬 구분 코드
    /// 0:전체, 1:상한가마감예상, 2:하한가마감예상, 3:직전대비상승률상위 ,4:직전대비하락률상위
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(11173)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 소속 구분 코드
    /// 0:전체, 1:종가범위연장
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ETF/ETN 현재가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceNextRequest {
    /// FID 입력 종목코드
    /// 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// FID 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [ETF 구성종목시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireComponentStockPriceRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드
    /// Unique key( 11216 )
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
}

/// [NAV 비교추이(종목)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NavComparisonTrendRequest {
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [NAV 비교추이(일)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NavComparisonDailyTrendRequest {
    /// FID 조건 시장 분류 코드
    /// J 입력
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목코드 (6자리)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1
    /// 조회 시작일자 (ex. 20240101)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2
    /// 조회 종료일자 (ex. 20240220)
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
}

/// [NAV 비교추이(분)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NavComparisonTimeTrendRequest {
    /// FID 시간 구분 코드
    /// 1분 :60, 3분: 180 … 120분:7200
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// FID 조건 시장 분류 코드
    /// E - 고정값
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [ELW 현재가 시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireElwPriceRequest {
    /// 조건 시장 분류 코드
    /// W
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목번호 (6자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 신규상장종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewlyListedRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Unique key(11548)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 분류구분코드
    /// 전체(02), 콜(00), 풋(01)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 기초자산입력종목코드
    /// 'ex) 000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 입력종목코드2
    /// '00003(한국투자증권), 00017(KB증권),
    /// 00005(미래에셋증권)'
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 입력날짜1
    /// 날짜 (ex) 20240402)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 결재방법
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNC_CLS_CODE")]
    pub fid_blnc_cls_code: String,
}

/// [ELW 민감도 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SensitivityRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Unique key(20285)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 입력종목코드
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 콜풋구분코드
    /// 0(전체), 1(콜), 2(풋)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 순위정렬구분코드
    /// '0(이론가), 1(델타), 2(감마), 3(로), 4(베가) , 5(로)
    /// , 6(내재변동성), 7(90일변동성)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 잔존일수(이상)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 조회기준일
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 결재방법
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 기초자산별 종목시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UdrlAssetPriceRequest {
    /// 조건시장분류코드
    /// 시장구분(W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Uniquekey(11541)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 시장구분코드
    /// 전체(A),콜(C),풋(P)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 입력종목코드
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기초자산입력종목코드
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 거래량수
    /// 전일거래량(정수량미만)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상제외구분코드
    /// 거래불가종목제외(0:미체크,1:체크)
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력가격1
    /// 가격~원이상
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력가격2
    /// 가격~월이하
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 입력거래량1
    /// 거래량~계약이상
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 입력거래량2
    /// 거래량~계약이하
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 입력잔존일수1
    /// 잔존일(~일이상)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 입력잔존일수2
    /// 잔존일(~일이하)
    #[serde(rename = "FID_INPUT_RMNN_DYNU_2")]
    pub fid_input_rmnn_dynu_2: String,
    /// 옵션
    /// 옵션상태(0:없음,1:ATM,2:ITM,3:OTM)
    #[serde(rename = "FID_OPTION")]
    pub fid_option: String,
    /// 입력옵션1
    #[serde(rename = "FID_INPUT_OPTION_1")]
    pub fid_input_option_1: String,
    /// 입력옵션2
    #[serde(rename = "FID_INPUT_OPTION_2")]
    pub fid_input_option_2: String,
}

/// [ELW 종목검색] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CondSearchRequest {
    /// 조건시장분류코드
    /// ELW(W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// 화면번호(11510)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 순위정렬구분코드
    /// '정렬1정렬안함(0)종목코드(1)현재가(2)대비율(3)거래량(4)행사가격(5)
    /// 전환비율(6)상장일(7)만기일(8)잔존일수(9)레버리지(10)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력수1
    /// 정렬1기준 - 상위(1)하위(2)
    #[serde(rename = "FID_INPUT_CNT_1")]
    pub fid_input_cnt_1: String,
    /// 순위정렬구분코드2
    /// 정렬2
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_2")]
    pub fid_rank_sort_cls_code_2: String,
    /// 입력수2
    /// 정렬2기준 - 상위(1)하위(2)
    #[serde(rename = "FID_INPUT_CNT_2")]
    pub fid_input_cnt_2: String,
    /// 순위정렬구분코드3
    /// 정렬3
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_3")]
    pub fid_rank_sort_cls_code_3: String,
    /// 입력수3
    /// 정렬3기준 - 상위(1)하위(2)
    #[serde(rename = "FID_INPUT_CNT_3")]
    pub fid_input_cnt_3: String,
    /// 대상구분코드
    /// 0:발행회사종목코드,1:기초자산종목코드,2:FID시장구분코드,3:FID입력날짜1(상장일),
    /// 4:FID입력날짜2(만기일),5:LP회원사종목코드,6:행사가기초자산비교>=(1) <=(2),
    /// 7:잔존일 이상 이하, 8:현재가, 9:전일대비율, 10:거래량, 11:최종거래일, 12:레버리지
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 입력종목코드
    /// 발행사종목코드전체(00000)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 기초자산입력종목코드
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 시장구분코드
    /// 권리유형전체(A)콜(CO)풋(PO)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 입력날짜1
    /// 상장일전체(0)금일(1)7일이하(2)8~30일(3)31~90일(4)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2
    /// 만기일전체(0)1개월(1)1~2(2)2~3(3)3~6(4)6~9(5)9~12(6)12이상(7)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 입력종목코드2
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 기타구분코드
    /// 행사가전체(0)>=(1)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 입력잔존일수1
    /// 잔존일이상
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 입력잔존일수2
    /// 잔존일이하
    #[serde(rename = "FID_INPUT_RMNN_DYNU_2")]
    pub fid_input_rmnn_dynu_2: String,
    /// 현재가수1
    /// 현재가이상
    #[serde(rename = "FID_PRPR_CNT1")]
    pub fid_prpr_cnt1: String,
    /// 현재가수2
    /// 현재가이하
    #[serde(rename = "FID_PRPR_CNT2")]
    pub fid_prpr_cnt2: String,
    /// 등락비율1
    /// 전일대비율이상
    #[serde(rename = "FID_RSFL_RATE1")]
    pub fid_rsfl_rate1: String,
    /// 등락비율2
    /// 전일대비율이하
    #[serde(rename = "FID_RSFL_RATE2")]
    pub fid_rsfl_rate2: String,
    /// 거래량1
    /// 거래량이상
    #[serde(rename = "FID_VOL1")]
    pub fid_vol1: String,
    /// 거래량2
    /// 거래량이하
    #[serde(rename = "FID_VOL2")]
    pub fid_vol2: String,
    /// 적용범위가격1
    /// 최종거래일from
    #[serde(rename = "FID_APLY_RANG_PRC_1")]
    pub fid_aply_rang_prc_1: String,
    /// 적용범위가격2
    /// 최종거래일to
    #[serde(rename = "FID_APLY_RANG_PRC_2")]
    pub fid_aply_rang_prc_2: String,
    /// 레버리지값1
    #[serde(rename = "FID_LVRG_VAL1")]
    pub fid_lvrg_val1: String,
    /// 레버리지값2
    #[serde(rename = "FID_LVRG_VAL2")]
    pub fid_lvrg_val2: String,
    /// 거래량3
    /// LP종료일from
    #[serde(rename = "FID_VOL3")]
    pub fid_vol3: String,
    /// 거래량4
    /// LP종료일to
    #[serde(rename = "FID_VOL4")]
    pub fid_vol4: String,
    /// 내재변동성1
    /// 내재변동성이상
    #[serde(rename = "FID_INTS_VLTL1")]
    pub fid_ints_vltl1: String,
    /// 내재변동성2
    /// 내재변동성이하
    #[serde(rename = "FID_INTS_VLTL2")]
    pub fid_ints_vltl2: String,
    /// 프리미엄값1
    /// 프리미엄이상
    #[serde(rename = "FID_PRMM_VAL1")]
    pub fid_prmm_val1: String,
    /// 프리미엄값2
    /// 프리미엄이하
    #[serde(rename = "FID_PRMM_VAL2")]
    pub fid_prmm_val2: String,
    /// 기어링1
    /// 기어링이상
    #[serde(rename = "FID_GEAR1")]
    pub fid_gear1: String,
    /// 기어링2
    /// 기어링이하
    #[serde(rename = "FID_GEAR2")]
    pub fid_gear2: String,
    /// 손익분기비율1
    /// 손익분기이상
    #[serde(rename = "FID_PRLS_QRYR_RATE1")]
    pub fid_prls_qryr_rate1: String,
    /// 손익분기비율2
    /// 손익분기이하
    #[serde(rename = "FID_PRLS_QRYR_RATE2")]
    pub fid_prls_qryr_rate2: String,
    /// 델타1
    /// 델타이상
    #[serde(rename = "FID_DELTA1")]
    pub fid_delta1: String,
    /// 델타2
    /// 델타이하
    #[serde(rename = "FID_DELTA2")]
    pub fid_delta2: String,
    /// 행사가1
    #[serde(rename = "FID_ACPR1")]
    pub fid_acpr1: String,
    /// 행사가2
    #[serde(rename = "FID_ACPR2")]
    pub fid_acpr2: String,
    /// 주식전환비율1
    /// 전환비율이상
    #[serde(rename = "FID_STCK_CNVR_RATE1")]
    pub fid_stck_cnvr_rate1: String,
    /// 주식전환비율2
    /// 전환비율이하
    #[serde(rename = "FID_STCK_CNVR_RATE2")]
    pub fid_stck_cnvr_rate2: String,
    /// 분류구분코드
    /// 0:전체,1:일반,2:조기종료
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 패리티1
    /// 패리티이상
    #[serde(rename = "FID_PRIT1")]
    pub fid_prit1: String,
    /// 패리티2
    /// 패리티이하
    #[serde(rename = "FID_PRIT2")]
    pub fid_prit2: String,
    /// 자본지지점1
    /// 배리어이상
    #[serde(rename = "FID_CFP1")]
    pub fid_cfp1: String,
    /// 자본지지점2
    /// 배리어이하
    #[serde(rename = "FID_CFP2")]
    pub fid_cfp2: String,
    /// 지수가격1
    /// LP보유비율이상
    #[serde(rename = "FID_INPUT_NMIX_PRICE_1")]
    pub fid_input_nmix_price_1: String,
    /// 지수가격2
    /// LP보유비율이하
    #[serde(rename = "FID_INPUT_NMIX_PRICE_2")]
    pub fid_input_nmix_price_2: String,
    /// E기어링값1
    /// 접근도이상
    #[serde(rename = "FID_EGEA_VAL1")]
    pub fid_egea_val1: String,
    /// E기어링값2
    /// 접근도이하
    #[serde(rename = "FID_EGEA_VAL2")]
    pub fid_egea_val2: String,
    /// 배당수익율
    /// 손익분기점이상
    #[serde(rename = "FID_INPUT_DVDN_ERT")]
    pub fid_input_dvdn_ert: String,
    /// 역사적변동성
    /// 손익분기점이하
    #[serde(rename = "FID_INPUT_HIST_VLTL")]
    pub fid_input_hist_vltl: String,
    /// 세타1
    /// MONEYNESS이상
    #[serde(rename = "FID_THETA1")]
    pub fid_theta1: String,
    /// 세타2
    /// MONEYNESS이하
    #[serde(rename = "FID_THETA2")]
    pub fid_theta2: String,
}

/// [ELW 당일급변종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct QuickChangeRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Unique key(20287)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장구분코드
    /// Unique key(A)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 가격(이상)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 시간구분코드
    /// 1(분), 2(일)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 입력 일 또는 분
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 기준시간(분 선택 시)
    #[serde(rename = "FID_INPUT_HOUR_2")]
    pub fid_input_hour_2: String,
    /// 순위정렬구분코드
    /// '1(가격급등), 2(가격급락), 3(거래량급증)
    /// , 4(매수잔량급증), 5(매도잔량급증)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 결재방법
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 기초자산 목록조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UdrlAssetListRequest {
    /// 조건화면분류코드
    /// 11541(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 순위정렬구분코드
    /// 0(종목명순), 1(콜발행종목순), 2(풋발행종목순), 3(전일대비 상승율순), 4(전일대비 하락율순), 5(현재가 크기순), 6(종목코드순)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력종목코드
    /// 00000(전체), 00003(한국투자증권), 00017(KB증권), 00005(미래에셋)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 비교대상종목조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompareStocksRequest {
    /// 조건화면분류코드
    /// 11517(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드
    /// 종목코드(ex)005930(삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW LP매매추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct LpTradeTrendRequest {
    /// 조건시장분류코드
    /// 시장구분(W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 입력종목코드(ex 52K577(미래 K577KOSDAQ150콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 투자지표추이(체결)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorTrendCcnlRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 투자지표추이(분별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorTrendMinuteRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간구분코드
    /// '60(1분), 180(3분), 300(5분), 600(10분), 1800(30분), 3600(60분), 7200(60분)
    /// '
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거데이터 포함 여부
    /// N(과거데이터포함X),Y(과거데이터포함O)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [ELW 투자지표추이(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorTrendDailyRequest {
    /// 시장분류코드
    /// W
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종콕코드
    /// ex. 57K281
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(틱)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendTickRequest {
    /// 조건시장분류코드
    /// W(Unique key)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성추이(체결)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendCcnlRequest {
    /// 조건시장분류코드
    /// W(Unique key)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendDailyRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 민감도 추이(체결)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SensitivityTrendCcnlRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 변동성 추이(분별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolatilityTrendMinuteRequest {
    /// 조건시장분류코드
    /// W(Unique key)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex) 58J297(KBJ297삼성전자콜)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간구분코드
    /// '60(1분), 180(3분), 300(5분), 600(10분), 1800(30분), 3600(60분)
    /// '
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거데이터 포함 여부
    /// N(과거데이터포함X),Y(과거데이터포함O)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [ELW 민감도 추이(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SensitivityTrendDailyRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// ex)(58J438(KBJ438삼성전자풋)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [ELW 만기예정/만기종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpirationStocksRequest {
    /// 조건시장분류코드
    /// W 입력
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// 11547 입력
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력날짜1
    /// 입력날짜 ~ (ex) 20240402)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2
    /// ~입력날짜 (ex) 20240408)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 분류구분코드
    /// 0(콜),1(풋),2(전체)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 기타구분코드
    /// 공백 입력
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 기초자산입력종목코드
    /// 000000(전체), 2001(KOSPI 200), 기초자산코드(종목코드 ex. 삼성전자-005930)
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행회사코드
    /// 00000(전체), 00003(한국투자증권), 00017(KB증권), 00005(미래에셋증권)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 결제방법
    /// 0(전체),1(일반),2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 입력옵션1
    /// 공백 입력
    #[serde(rename = "FID_INPUT_OPTION_1")]
    pub fid_input_option_1: String,
}

/// [ELW 지표순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndicatorRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Unique key(20279)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 콜풋구분코드
    /// 0(전체), 1(콜), 2(풋)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 순위정렬구분코드
    /// 0(전환비율), 1(레버리지), 2(행사가 ), 3(내재가치), 4(시간가치)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 결재방법
    /// 0(전체), 1(일반), 2(조기종료)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [ELW 상승률순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UpdownRateRequest {
    /// 사용자권한정보
    /// 시장구분코드 (W)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 거래소코드
    /// Unique key(20277)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 상승율/하락율 구분
    /// '000000(전체), 2001(코스피200)
    /// , 3003(코스닥150), 005930(삼성전자) '
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// N일자값
    /// '00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 거래량조건
    /// '0(전체), 1(1개월이하), 2(1개월~2개월),
    /// 3(2개월~3개월), 4(3개월~6개월),
    /// 5(6개월~9개월),6(9개월~12개월), 7(12개월이상)'
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// NEXT KEY BUFF
    /// 0(전체), 1(콜), 2(풋)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 사용자권한정보
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 거래소코드
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 상승율/하락율 구분
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// N일자값
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 거래량조건
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// NEXT KEY BUFF
    /// '0(상승율), 1(하락율), 2(시가대비상승율)
    /// , 3(시가대비하락율), 4(변동율)'
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 사용자권한정보
    /// 0(전체)
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 거래소코드
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [ELW 거래량순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumeRankRequest {
    /// 조건시장분류코드
    /// W
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// 20278
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 기초자산입력종목코드
    /// 000000
    #[serde(rename = "FID_UNAS_INPUT_ISCD")]
    pub fid_unas_input_iscd: String,
    /// 발행사
    /// 00000(전체), 00003(한국투자증권)
    /// , 00017(KB증권), 00005(미래에셋주식회사)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력잔존일수
    #[serde(rename = "FID_INPUT_RMNN_DYNU_1")]
    pub fid_input_rmnn_dynu_1: String,
    /// 콜풋구분코드
    /// 0(전체), 1(콜), 2(풋)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 가격(이상)
    /// 거래가격1(이상)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 가격(이하)
    /// 거래가격1(이하)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량(이상)
    /// 거래량1(이상)
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
    /// 거래량(이하)
    /// 거래량1(이하)
    #[serde(rename = "FID_INPUT_VOL_2")]
    pub fid_input_vol_2: String,
    /// 조회기준일
    /// 입력날짜(기준가 조회기준)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 순위정렬구분코드
    /// 0: 거래량순 1: 평균거래증가율 2: 평균거래회전율 3:거래금액순 4: 순매수잔량순 5: 순매도잔량순
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 소속구분코드
    /// 0: 전체
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// LP발행사
    /// 0000
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 만기일-최종거래일조회
    /// 공백
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [국내업종 현재지수] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexPriceRequest {
    /// FID 조건 시장 분류 코드
    /// 업종(U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 코스피(0001), 코스닥(1001), 코스피200(2001)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내업종 일자별지수] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexDailyPriceRequest {
    /// FID 기간 분류 코드
    /// 일/주/월 구분코드 ( D:일별 , W:주별, M:월별 )
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// FID 조건 시장 분류 코드
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 코스피(0001), 코스닥(1001), 코스피200(2001)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1
    /// 입력 날짜(ex. 20240223)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내업종 시간별지수(초)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexTickpriceRequest {
    /// 입력 종목코드
    /// 0001:거래소, 1001:코스닥, 2001:코스피200, 3003:KSQ150
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장 분류 코드
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내업종 시간별지수(분)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexTimepriceRequest {
    /// ?입력 시간1
    /// 초단위, 60(1분), 300(5분), 600(10분)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 종목코드
    /// 0001:거래소, 1001:코스닥, 2001:코스피200, 3003:KSQ150
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [업종 분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeIndexchartpriceRequest {
    /// FID 조건 시장 분류 코드
    /// U
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 기타 구분 코드
    /// 0: 기본 1:장마감,시간외 제외
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// FID 입력 종목코드
    /// 0001 : 종합
    /// 0002 : 대형주
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 시간1
    /// 30, 60 -> 1분, 600-> 10분, 3600 -> 1시간
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// FID 과거 데이터 포함 여부
    /// Y (과거) / N (당일)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [국내주식업종기간별시세(일/주/월/년)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyIndexchartpriceRequest {
    /// 조건 시장 분류 코드
    /// 업종 : U
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 업종 상세코드
    /// '0001 : 종합
    /// 0002 : 대형주
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)'
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회 시작일자
    /// 조회 시작일자 (ex. 20220501)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 조회 종료일자
    /// 조회 종료일자 (ex. 20220530)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// ' 기간분류코드'
    /// ' D:일봉 W:주봉, M:월봉, Y:년봉'
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [국내업종 구분별전체시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireIndexCategoryPriceRequest {
    /// FID 조건 시장 분류 코드
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 코스피(0001), 코스닥(1001), 코스피200(2001)
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 조건 화면 분류 코드
    /// Unique key( 20214 )
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 시장 구분 코드
    /// 시장구분코드(K:거래소, Q:코스닥, K2:코스피200)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// FID 소속 구분 코드
    /// 시장구분코드에 따라 아래와 같이 입력
    /// 시장구분코드(K:거래소) 0:전업종, 1:기타구분, 2:자본금구분 3:상업별구분
    /// 시장구분코드(Q:코스닥) 0:전업종, 1:기타구분, 2:벤처구분 3:일반구분
    /// 시장구분코드(K2:코스닥) 0:전업종
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
}

/// [국내주식 예상체결지수 추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpIndexTrendRequest {
    /// 장운영 구분 코드
    /// 1: 장시작전, 2: 장마감
    #[serde(rename = "FID_MKOP_CLS_CODE")]
    pub fid_mkop_cls_code: String,
    /// 입력 시간1
    /// 10(10초), 30(30초), 60(1분), 600(10분)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:코스피, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 예상체결 전체지수] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpTotalIndexRequest {
    /// 시장 구분 코드
    /// 0:전체 K:거래소 Q:코스닥
    #[serde(rename = "fid_mrkt_cls_code")]
    pub fid_mrkt_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (업종 U)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(11175)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 장운영 구분 코드
    /// 1:장시작전, 2:장마감
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
}

/// [변동성완화장치(VI) 현황] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireViStatusRequest {
    /// FID 분류 구분 코드
    /// 0:전체 1:상승 2:하락
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// FID 조건 화면 분류 코드
    /// 20139
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 시장 구분 코드
    /// 0:전체 K:거래소 Q:코스닥
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// FID 입력 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 순위 정렬 구분 코드
    /// 0:전체1:정적2:동적3:정적&동적
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// FID 입력 날짜1
    /// 영업일
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 대상 구분 코드
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID 대상 제외 구분 코드
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [금리 종합(국내채권/금리)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompInterestRequest {
    /// 조건시장분류코드
    /// Unique key(I)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Unique key(20702)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 분류구분코드
    /// 1: 해외금리지표
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 분류구분코드
    /// 공백 : 전체
    #[serde(rename = "FID_DIV_CLS_CODE1")]
    pub fid_div_cls_code1: String,
}

/// [종합 시황/공시(제목)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewsTitleRequest {
    /// 뉴스 제공 업체 코드
    /// 공백 필수 입력
    #[serde(rename = "FID_NEWS_OFER_ENTP_CODE")]
    pub fid_news_ofer_entp_code: String,
    /// 조건 시장 구분 코드
    /// 공백 필수 입력
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 입력 종목코드
    /// 공백: 전체, 종목코드 : 해당코드가 등록된 뉴스
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 제목 내용
    /// 공백 필수 입력
    #[serde(rename = "FID_TITL_CNTT")]
    pub fid_titl_cntt: String,
    /// 입력 날짜
    /// 공백: 현재기준, 조회일자(ex 00YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 시간
    /// 공백: 현재기준, 조회시간(ex 0000HHMMSS)
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 순위 정렬 구분 코드
    /// 공백 필수 입력
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 일련번호
    /// 공백 필수 입력
    #[serde(rename = "FID_INPUT_SRNO")]
    pub fid_input_srno: String,
}

/// [국내휴장일조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ChkHolidayRequest {
    /// 기준일자
    /// 기준일자(YYYYMMDD)
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 연속조회키
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// 연속조회검색조건
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
}

/// [상품기본조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchInfoRequest {
    /// 상품번호
    /// '주식(하이닉스) : 000660 (코드 : 300)
    /// 선물(101S12) : KR4101SC0009 (코드 : 301)
    /// 미국(AAPL) : AAPL (코드 : 512)'
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchStockInfoRequest {
    /// 상품유형코드
    /// 300: 주식, ETF, ETN, ELW
    /// 301 : 선물옵션
    /// 302 : 채권
    /// 306 : ELS'
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 상품번호
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [국내주식 대차대조표] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BalanceSheetRequest {
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 손익계산서] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IncomeStatementRequest {
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    ///
    /// ※ 분기데이터는 연단위 누적합산
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 재무비율] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FinancialRatioRequest {
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 수익성비율] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProfitRatioRequest {
    /// 입력 종목코드
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 기타주요비율] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OtherMajorRatiosRequest {
    /// 입력 종목코드
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 안정성비율] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct StabilityRatioRequest {
    /// 입력 종목코드
    /// 000660 : 종목코드
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// J
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 성장성비율] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct GrowthRatioRequest {
    /// 입력 종목코드
    /// ex : 000660
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 년, 1: 분기
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [국내주식 당사 신용가능종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CreditByCompanyRequest {
    /// 순위 정렬 구분 코드
    /// 0:코드순, 1:이름순
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 선택 여부
    /// 0:신용주문가능, 1: 신용주문불가
    #[serde(rename = "fid_slct_yn")]
    pub fid_slct_yn: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 조건 화면 분류 코드
    /// Unique key(20477)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
}

/// [예탁원정보(배당일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DividendRequest {
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회구분
    /// 0:배당전체, 1:결산배당, 2:중간배당
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 고배당여부
    /// 공백
    #[serde(rename = "HIGH_GB")]
    pub high_gb: String,
}

/// [예탁원정보(주식매수청구일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PurreqRequest {
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(합병/분할일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MergerSplitRequest {
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(액면교체일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct RevSplitRequest {
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 시장구분
    /// 0:전체, 1:코스피, 2:코스닥
    #[serde(rename = "MARKET_GB")]
    pub market_gb: String,
}

/// [예탁원정보(자본감소일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CapDcrsRequest {
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(상장정보일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ListInfoRequest {
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(공모주청약일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PubOfferRequest {
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
}

/// [예탁원정보(실권주일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ForfeitRequest {
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(의무예치일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MandDepositRequest {
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [예탁원정보(유상증자일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PaidinCapinRequest {
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회구분
    /// 1(청약일별), 2(기준일별)
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백(전체), 특정종목 조회시(종목코드)
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(무상증자일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BonusIssueRequest {
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [예탁원정보(주주총회일정)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SharehldMeetRequest {
    /// CTS
    /// 공백
    #[serde(rename = "CTS")]
    pub cts: String,
    /// 조회일자From
    /// 일자 ~
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 조회일자To
    /// ~ 일자
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 종목코드
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [국내주식 종목추정실적] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct EstimatePerformRequest {
    /// 종목코드
    /// ex) 265520
    #[serde(rename = "SHT_CD")]
    pub sht_cd: String,
}

/// [당사 대주가능 종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct LendableByCompanyRequest {
    /// 거래소구분코드
    /// 00(전체), 02(거래소), 03(코스닥)
    #[serde(rename = "EXCG_DVSN_CD")]
    pub excg_dvsn_cd: String,
    /// 상품번호
    /// 공백 : 전체조회, 종목코드 입력 시 해당종목만 조회
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 당사대주가능여부
    /// Y
    #[serde(rename = "THCO_STLN_PSBL_YN")]
    pub thco_stln_psbl_yn: String,
    /// 조회구분1
    /// 0 : 전체조회, 1: 종목코드순 정렬
    #[serde(rename = "INQR_DVSN_1")]
    pub inqr_dvsn_1: String,
    /// 연속조회검색조건200
    /// 미입력 (다음조회 불가)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키100
    /// 미입력 (다음조회 불가)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [국내주식 종목투자의견] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestOpinionRequest {
    /// 조건시장분류코드
    /// J(시장 구분 코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// 16633(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드
    /// 종목코드(ex) 005930(삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력날짜1
    /// 이후 ~(ex) 0020231113)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2
    /// ~ 이전(ex) 0020240513)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [국내주식 증권사별 투자의견] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestOpbysecRequest {
    /// 조건시장분류코드
    /// J(시장 구분 코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// 16634(Primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드
    /// 회원사코드 (kis developers 포탈 사이트 포럼-> FAQ -> 종목정보 다운로드(국내) 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류구분코드
    /// 전체(0) 매수(1) 중립(2) 매도(3)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력날짜1
    /// 이후 ~
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2
    /// ~ 이전
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [종목조건검색 목록조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PsearchTitleRequest {
    /// 사용자 HTS ID
    #[serde(rename = "user_id")]
    pub user_id: String,
}

/// [종목조건검색조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PsearchResultRequest {
    /// 사용자 HTS ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 사용자조건 키값
    /// 종목조건검색 목록조회 API의 output인 'seq'을 이용
    /// (0 부터 시작)
    #[serde(rename = "seq")]
    pub seq: String,
}

/// [관심종목 그룹조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntstockGrouplistRequest {
    /// 관심종목구분코드
    /// Unique key(1)
    #[serde(rename = "TYPE")]
    pub r#type: String,
    /// FID 기타 구분 코드
    /// Unique key(00)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
    /// 사용자 ID
    /// HTS_ID 입력
    #[serde(rename = "USER_ID")]
    pub user_id: String,
}

/// [관심종목(멀티종목) 시세조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntstockMultpriceRequest {
    /// 조건 시장 분류 코드1
    /// 그룹별종목조회 결과 fid_mrkt_cls_code(시장구분) 1 입력
    /// J: KRX, NX: NXT, UN: 통합
    /// ex) J
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_1")]
    pub fid_cond_mrkt_div_code_1: String,
    /// 입력 종목코드1
    /// 그룹별종목조회 결과 jong_code(종목코드) 1 입력
    /// ex) 005930
    #[serde(rename = "FID_INPUT_ISCD_1")]
    pub fid_input_iscd_1: String,
    /// 조건 시장 분류 코드2
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_2")]
    pub fid_cond_mrkt_div_code_2: String,
    /// 입력 종목코드2
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 조건 시장 분류 코드3
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_3")]
    pub fid_cond_mrkt_div_code_3: String,
    /// 입력 종목코드3
    #[serde(rename = "FID_INPUT_ISCD_3")]
    pub fid_input_iscd_3: String,
    /// 조건 시장 분류 코드4
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_4")]
    pub fid_cond_mrkt_div_code_4: String,
    /// 입력 종목코드4
    #[serde(rename = "FID_INPUT_ISCD_4")]
    pub fid_input_iscd_4: String,
    /// 조건 시장 분류 코드5
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_5")]
    pub fid_cond_mrkt_div_code_5: String,
    /// 입력 종목코드5
    #[serde(rename = "FID_INPUT_ISCD_5")]
    pub fid_input_iscd_5: String,
    /// 조건 시장 분류 코드6
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_6")]
    pub fid_cond_mrkt_div_code_6: String,
    /// 입력 종목코드6
    #[serde(rename = "FID_INPUT_ISCD_6")]
    pub fid_input_iscd_6: String,
    /// 조건 시장 분류 코드7
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_7")]
    pub fid_cond_mrkt_div_code_7: String,
    /// 입력 종목코드7
    #[serde(rename = "FID_INPUT_ISCD_7")]
    pub fid_input_iscd_7: String,
    /// 조건 시장 분류 코드8
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_8")]
    pub fid_cond_mrkt_div_code_8: String,
    /// 입력 종목코드8
    #[serde(rename = "FID_INPUT_ISCD_8")]
    pub fid_input_iscd_8: String,
    /// 조건 시장 분류 코드9
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_9")]
    pub fid_cond_mrkt_div_code_9: String,
    /// 입력 종목코드9
    #[serde(rename = "FID_INPUT_ISCD_9")]
    pub fid_input_iscd_9: String,
    /// 조건 시장 분류 코드10
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_10")]
    pub fid_cond_mrkt_div_code_10: String,
    /// 입력 종목코드10
    #[serde(rename = "FID_INPUT_ISCD_10")]
    pub fid_input_iscd_10: String,
    /// 조건 시장 분류 코드11
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_11")]
    pub fid_cond_mrkt_div_code_11: String,
    /// 입력 종목코드11
    #[serde(rename = "FID_INPUT_ISCD_11")]
    pub fid_input_iscd_11: String,
    /// 조건 시장 분류 코드12
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_12")]
    pub fid_cond_mrkt_div_code_12: String,
    /// 입력 종목코드12
    #[serde(rename = "FID_INPUT_ISCD_12")]
    pub fid_input_iscd_12: String,
    /// 조건 시장 분류 코드13
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_13")]
    pub fid_cond_mrkt_div_code_13: String,
    /// 입력 종목코드13
    #[serde(rename = "FID_INPUT_ISCD_13")]
    pub fid_input_iscd_13: String,
    /// 조건 시장 분류 코드14
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_14")]
    pub fid_cond_mrkt_div_code_14: String,
    /// 입력 종목코드14
    #[serde(rename = "FID_INPUT_ISCD_14")]
    pub fid_input_iscd_14: String,
    /// 조건 시장 분류 코드15
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_15")]
    pub fid_cond_mrkt_div_code_15: String,
    /// 입력 종목코드15
    #[serde(rename = "FID_INPUT_ISCD_15")]
    pub fid_input_iscd_15: String,
    /// 조건 시장 분류 코드16
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_16")]
    pub fid_cond_mrkt_div_code_16: String,
    /// 입력 종목코드16
    #[serde(rename = "FID_INPUT_ISCD_16")]
    pub fid_input_iscd_16: String,
    /// 조건 시장 분류 코드17
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_17")]
    pub fid_cond_mrkt_div_code_17: String,
    /// 입력 종목코드17
    #[serde(rename = "FID_INPUT_ISCD_17")]
    pub fid_input_iscd_17: String,
    /// 조건 시장 분류 코드18
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_18")]
    pub fid_cond_mrkt_div_code_18: String,
    /// 입력 종목코드18
    #[serde(rename = "FID_INPUT_ISCD_18")]
    pub fid_input_iscd_18: String,
    /// 조건 시장 분류 코드19
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_19")]
    pub fid_cond_mrkt_div_code_19: String,
    /// 입력 종목코드19
    #[serde(rename = "FID_INPUT_ISCD_19")]
    pub fid_input_iscd_19: String,
    /// 조건 시장 분류 코드20
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_20")]
    pub fid_cond_mrkt_div_code_20: String,
    /// 입력 종목코드20
    #[serde(rename = "FID_INPUT_ISCD_20")]
    pub fid_input_iscd_20: String,
    /// 조건 시장 분류 코드21
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_21")]
    pub fid_cond_mrkt_div_code_21: String,
    /// 입력 종목코드21
    #[serde(rename = "FID_INPUT_ISCD_21")]
    pub fid_input_iscd_21: String,
    /// 조건 시장 분류 코드22
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_22")]
    pub fid_cond_mrkt_div_code_22: String,
    /// 입력 종목코드22
    #[serde(rename = "FID_INPUT_ISCD_22")]
    pub fid_input_iscd_22: String,
    /// 조건 시장 분류 코드23
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_23")]
    pub fid_cond_mrkt_div_code_23: String,
    /// 입력 종목코드23
    #[serde(rename = "FID_INPUT_ISCD_23")]
    pub fid_input_iscd_23: String,
    /// 조건 시장 분류 코드24
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_24")]
    pub fid_cond_mrkt_div_code_24: String,
    /// 입력 종목코드24
    #[serde(rename = "FID_INPUT_ISCD_24")]
    pub fid_input_iscd_24: String,
    /// 조건 시장 분류 코드25
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_25")]
    pub fid_cond_mrkt_div_code_25: String,
    /// 입력 종목코드25
    #[serde(rename = "FID_INPUT_ISCD_25")]
    pub fid_input_iscd_25: String,
    /// 조건 시장 분류 코드26
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_26")]
    pub fid_cond_mrkt_div_code_26: String,
    /// 입력 종목코드26
    #[serde(rename = "FID_INPUT_ISCD_26")]
    pub fid_input_iscd_26: String,
    /// 조건 시장 분류 코드27
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_27")]
    pub fid_cond_mrkt_div_code_27: String,
    /// 입력 종목코드27
    #[serde(rename = "FID_INPUT_ISCD_27")]
    pub fid_input_iscd_27: String,
    /// 조건 시장 분류 코드28
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_28")]
    pub fid_cond_mrkt_div_code_28: String,
    /// 입력 종목코드28
    #[serde(rename = "FID_INPUT_ISCD_28")]
    pub fid_input_iscd_28: String,
    /// 조건 시장 분류 코드29
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_29")]
    pub fid_cond_mrkt_div_code_29: String,
    /// 입력 종목코드29
    #[serde(rename = "FID_INPUT_ISCD_29")]
    pub fid_input_iscd_29: String,
    /// 조건 시장 분류 코드30
    #[serde(rename = "FID_COND_MRKT_DIV_CODE_30")]
    pub fid_cond_mrkt_div_code_30: String,
    /// 입력 종목코드30
    #[serde(rename = "FID_INPUT_ISCD_30")]
    pub fid_input_iscd_30: String,
}

/// [관심종목 그룹별 종목조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IntstockStocklistByGroupRequest {
    /// 관심종목구분코드
    /// Unique key(1)
    #[serde(rename = "TYPE")]
    pub r#type: String,
    /// 사용자 ID
    /// HTS_ID 입력
    #[serde(rename = "USER_ID")]
    pub user_id: String,
    /// 데이터 순위
    /// 공백
    #[serde(rename = "DATA_RANK")]
    pub data_rank: String,
    /// 관심 그룹 코드
    /// 관심그룹 조회 결과의 그룹 값 입력
    #[serde(rename = "INTER_GRP_CODE")]
    pub inter_grp_code: String,
    /// 관심 그룹 명
    /// 공백
    #[serde(rename = "INTER_GRP_NAME")]
    pub inter_grp_name: String,
    /// HTS 한글 종목명
    /// 공백
    #[serde(rename = "HTS_KOR_ISNM")]
    pub hts_kor_isnm: String,
    /// 체결 구분 코드
    /// 공백
    #[serde(rename = "CNTG_CLS_CODE")]
    pub cntg_cls_code: String,
    /// 기타 구분 코드
    /// Unique key(4)
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [국내기관_외국인 매매종목가집계] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ForeignInstitutionTotalRequest {
    /// 시장 분류 코드
    /// V(Default)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// 16449(Default)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:코스피, 1001:코스닥
    /// ...
    /// 포탈 (FAQ : 종목정보 다운로드(국내) - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 수량정열, 1: 금액정열
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드
    /// 0: 순매수상위, 1: 순매도상위
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 기타 구분 정렬
    /// 0:전체 1:외국인 2:기관계 3:기타
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [외국계 매매종목 가집계] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FrgnmemTradeEstimateRequest {
    /// 조건시장분류코드
    /// 시장구분코드 (J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Uniquekey (16441)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드
    /// 0000(전체), 1001(코스피), 2001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위정렬구분코드
    /// 0(금액순), 1(수량순)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 순위정렬구분코드2
    /// 0(매수순), 1(매도순)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE_2")]
    pub fid_rank_sort_cls_code_2: String,
}

/// [종목별 투자자매매동향(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorTradeByStockDailyRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목번호 (6자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1
    /// 입력 날짜(20250812) (해당일 조회는 장 종료 후 정상 조회 가능)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 수정주가 원주가 가격
    /// 공란 입력
    #[serde(rename = "FID_ORG_ADJ_PRC")]
    pub fid_org_adj_prc: String,
    /// 기타 구분 코드
    /// "1" 입력
    #[serde(rename = "FID_ETC_CLS_CODE")]
    pub fid_etc_cls_code: String,
}

/// [시장별 투자자매매동향(시세)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireInvestorTimeByMarketRequest {
    /// 시장구분
    /// 코스피: KSP, 코스닥:KSQ,
    /// 선물,콜옵션,풋옵션 : K2I, 주식선물:999,
    /// ETF: ETF, ELW:ELW, ETN: ETN,
    /// 미니: MKI, 위클리월 : WKM, 위클리목: WKI
    /// 코스닥150: KQI
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 업종구분
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireInvestorDailyByMarketRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (업종 U)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 코스피, 코스닥 : 업종분류코드 (종목정보파일 - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1
    /// ex. 20240517
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력 종목코드
    /// 코스피(KSP), 코스닥(KSQ)
    #[serde(rename = "FID_INPUT_ISCD_1")]
    pub fid_input_iscd_1: String,
    /// 입력 날짜2
    /// 입력 날짜1과 동일날짜 입력
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 하위 분류코드
    /// 코스피, 코스닥 : 업종분류코드 (종목정보파일 - 업종코드 참조)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
}

/// [종목별 외국계 순매수추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FrgnmemPchsTrendRequest {
    /// 조건시장분류코드
    /// 종목코드(ex) 005930(삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드
    /// 외국계 전체(99999)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 시장구분코드
    /// J (KRX만 지원)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [회원사 실시간 매매동향(틱)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FrgnmemTradeTrendRequest {
    /// 화면분류코드
    /// 20432(primary key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// FID 조건 시장 분류 코드
    /// J 고정 입력
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드
    /// ex. 005930(삼성전자)
    ///
    /// ※ FID_INPUT_ISCD(종목코드) 혹은 FID_MRKT_CLS_CODE(시장구분코드) 둘 중 하나만 입력
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 회원사코드
    /// ex. 99999(전체)
    ///
    /// ※ 회원사코드 (kis developers 포탈 사이트 포럼-> FAQ -> 종목정보 다운로드(국내) 참조)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 시장구분코드
    /// A(전체),K(코스피), Q(코스닥), K2(코스피200), W(ELW)
    ///
    /// ※ FID_INPUT_ISCD(종목코드) 혹은 FID_MRKT_CLS_CODE(시장구분코드) 둘 중 하나만 입력
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 거래량
    /// 거래량 ~
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
}

/// [주식현재가 회원사 종목매매동향] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireMemberDailyRequest {
    /// 조건시장분류코드
    /// J: KRX, NX: NXT, UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 주식종목코드입력
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 회원사코드
    /// 회원사코드 (kis developers 포탈 사이트 포럼-> FAQ -> 종목정보 다운로드(국내) > 회원사 참조)
    #[serde(rename = "FID_INPUT_ISCD_2")]
    pub fid_input_iscd_2: String,
    /// 입력날짜1
    /// 날짜 ~
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력날짜2
    /// ~ 날짜
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 구간구분코드
    /// 공백
    #[serde(rename = "FID_SCTN_CLS_CODE")]
    pub fid_sctn_cls_code: String,
}

/// [종목별 프로그램매매추이(체결)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProgramTradeByStockRequest {
    /// 조건 시장 분류 코드
    /// KRX : J , NXT : NX, 통합 : UN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [종목별 프로그램매매추이(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProgramTradeByStockDailyRequest {
    /// 조건 시장 분류 코드
    /// KRX : J , NXT : NX, 통합 : UN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1
    /// 기준일 (ex 0020240308), 미입력시 당일부터 조회
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [종목별 외인기관 추정가집계] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorTrendEstimateRequest {
    /// 종목코드
    /// 종목코드
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
}

/// [종목별일별매수매도체결량] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyTradeVolumeRequest {
    /// FID 조건 시장 분류 코드
    /// J: KRX, NX: NXT, UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 005930
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1
    /// from
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2
    /// to
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// FID 기간 분류 코드
    /// D
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [프로그램매매 종합현황(시간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompProgramTradeTodayRequest {
    /// 시장 분류 코드
    /// KRX : J , NXT : NX, 통합 : UN
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드
    /// K:코스피, Q:코스닥
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 구간 구분 코드
    /// 공백 입력
    #[serde(rename = "FID_SCTN_CLS_CODE")]
    pub fid_sctn_cls_code: String,
    /// 입력 종목코드
    /// 공백 입력
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시장 분류코드1
    /// 공백 입력
    #[serde(rename = "FID_COND_MRKT_DIV_CODE1")]
    pub fid_cond_mrkt_div_code1: String,
    /// 입력 시간1
    /// 공백 입력
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [프로그램매매 종합현황(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CompProgramTradeDailyRequest {
    /// 시장 분류 코드
    /// J : KRX, NX : NXT, UN : 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드
    /// K:코스피, Q:코스닥
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 검색시작일
    /// 공백 입력, 입력 시 ~ 입력일자까지 조회됨
    /// * 8개월 이상 과거 조회 불가
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 검색종료일
    /// 공백 입력
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
}

/// [프로그램매매 투자자매매동향(당일)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorProgramTradeTodayRequest {
    /// 거래소 구분 코드
    /// J : KRX, NX : NXT, UN : 통합
    #[serde(rename = "EXCH_DIV_CLS_CODE")]
    pub exch_div_cls_code: String,
    /// 시장 구분 코드
    /// 1:코스피, 4:코스닥
    #[serde(rename = "MRKT_DIV_CLS_CODE")]
    pub mrkt_div_cls_code: String,
}

/// [국내주식 신용잔고 일별추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyCreditBalanceRequest {
    /// 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 화면 분류 코드
    /// Unique key(20476)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 종목코드
    /// 종목코드 (ex 005930)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 결제일자
    /// 결제일자 (ex 20240313)
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 예상체결가 추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpPriceTrendRequest {
    /// 장운영 구분 코드
    /// 0:전체, 4:체결량 0 제외
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드(ex. 005930)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
}

/// [국내주식 공매도 일별추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyShortSaleRequest {
    /// 입력 날짜2
    /// ~ 누적
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 입력 날짜1
    /// 공백시 전체 (기간 ~)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 시간외예상체결등락률] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OvertimeExpTransFluctRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J: 주식)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(11186)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000(전체), 0001(코스피), 1001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드
    /// 0(상승률), 1(상승폭), 2(보합), 3(하락률), 4(하락폭)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드
    /// '0(전체), 1(관리종목), 2(투자주의), 3(투자경고),
    /// 4(투자위험예고), 5(투자위험), 6(보통주), 7(우선주)'
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 가격 ~
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 공백
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 입력 거래량
    /// 거래량 ~
    #[serde(rename = "FID_INPUT_VOL_1")]
    pub fid_input_vol_1: String,
}

/// [국내주식 체결금액별 매매비중] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradprtByamtRequest {
    /// 조건시장분류코드
    /// J: KRX, NX: NXT, UN: 통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// Uniquekey(11119)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력종목코드
    /// 종목코드(ex)(005930 (삼성전자))
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [국내 증시자금 종합] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MktfundsRequest {
    /// 입력날짜1
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [종목별 일별 대차거래추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyLoanTransRequest {
    /// 조회구분
    /// 1(코스피), 2(코스닥), 3(종목)
    #[serde(rename = "MRKT_DIV_CLS_CODE")]
    pub mrkt_div_cls_code: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "MKSC_SHRN_ISCD")]
    pub mksc_shrn_iscd: String,
    /// 조회시작일시
    /// 조회기간 ~
    #[serde(rename = "START_DATE")]
    pub start_date: String,
    /// 조회종료일시
    /// ~ 조회기간
    #[serde(rename = "END_DATE")]
    pub end_date: String,
    /// 이전조회KEY
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [국내주식 상하한가 포착] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CaptureUplowpriceRequest {
    /// 조건시장분류코드
    /// 시장구분(J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건화면분류코드
    /// 11300(Unique key)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 상하한가 구분코드
    /// 0(상한가),1(하한가)
    #[serde(rename = "FID_PRC_CLS_CODE")]
    pub fid_prc_cls_code: String,
    /// 분류구분코드
    /// '0(상하한가종목),6(8%상하한가 근접), 5(10%상하한가 근접), 1(15%상하한가 근접),2(20%상하한가 근접),
    /// 3(25%상하한가 근접)'
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력종목코드
    /// 전체(0000), 코스피(0001),코스닥(1001)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 대상구분코드
    /// 공백 입력
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상제외구분코드
    /// 공백 입력
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력가격1
    /// 공백 입력
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력가격2
    /// 공백 입력
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량수
    /// 공백 입력
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
}

/// [국내주식 매물대/거래비중] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PbarTratioRequest {
    /// 조건시장분류코드
    /// J:KRX, NX:NXT, UN:통합
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 주식단축종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건화면분류코드
    /// Uniquekey(20113)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력시간1
    /// 공백
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [거래량순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumeRankNextRequest {
    /// 조건 시장 분류 코드
    /// J:KRX, NX:NXT
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// 20171
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000(전체) 기타(업종코드)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0(전체) 1(보통주) 2(우선주)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 소속 구분 코드
    /// 0 : 평균거래량 1:거래증가율 2:평균거래회전율 3:거래금액순 4:평균거래금액회전율
    #[serde(rename = "FID_BLNG_CLS_CODE")]
    pub fid_blng_cls_code: String,
    /// 대상 구분 코드
    /// 1 or 0 9자리 (차례대로 증거금 30% 40% 50% 60% 100% 신용보증금 30% 40% 50% 60%)
    /// ex) "111111111"
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 1 or 0 10자리 (차례대로 투자위험/경고/주의 관리종목 정리매매 불성실공시 우선주 거래정지 ETF ETN 신용주문불가 SPAC)
    /// ex) "0000000000"
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1
    /// 가격 ~
    /// ex) "0"
    ///
    /// 전체 가격 대상 조회 시 FID_INPUT_PRICE_1, FID_INPUT_PRICE_2 모두 ""(공란) 입력
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// ~ 가격
    /// ex) "1000000"
    ///
    /// 전체 가격 대상 조회 시 FID_INPUT_PRICE_1, FID_INPUT_PRICE_2 모두 ""(공란) 입력
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 거래량 ~
    /// ex) "100000"
    ///
    /// 전체 거래량 대상 조회 시 FID_VOL_CNT ""(공란) 입력
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 입력 날짜1
    /// ""(공란) 입력
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
}

/// [국내주식 등락률 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FluctuationRequest {
    /// 등락 비율2
    /// 공백 입력 시 전체 (~ 비율
    #[serde(rename = "fid_rsfl_rate2")]
    pub fid_rsfl_rate2: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20170 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000(전체) 코스피(0001), 코스닥(1001), 코스피200(2001)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드
    /// 0:상승율순 1:하락율순 2:시가대비상승율 3:시가대비하락율 4:변동율
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 수1
    /// 0:전체 , 누적일수 입력
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// 가격 구분 코드
    /// 'fid_rank_sort_cls_code :0 상승율 순일때 (0:저가대비, 1:종가대비)
    /// fid_rank_sort_cls_code :1 하락율 순일때 (0:고가대비, 1:종가대비)
    /// fid_rank_sort_cls_code : 기타 (0:전체)'
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// 입력 가격1
    /// 공백 입력 시 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 공백 입력 시 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 공백 입력 시 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 분류 구분 코드
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 등락 비율1
    /// 공백 입력 시 전체 (비율 ~)
    #[serde(rename = "fid_rsfl_rate1")]
    pub fid_rsfl_rate1: String,
}

/// [국내주식 호가잔량 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct QuoteBalanceRequest {
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20172 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000(전체) 코스피(0001), 코스닥(1001), 코스피200(2001)
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드
    /// 0: 순매수잔량순, 1:순매도잔량순, 2:매수비율순, 3:매도비율순
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 대상 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 수익자산지표 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ProfitAssetIndexRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 대상 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20173 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 입력 옵션1
    /// 회계연도 (2023)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2
    /// 0: 1/4분기 , 1: 반기, 2: 3/4분기, 3: 결산
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드
    /// 0:매출이익 1:영업이익 2:경상이익 3:당기순이익 4:자산총계 5:부채총계 6:자본총계
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드
    /// 0:전체
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시가총액 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketCapRequest {
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20174 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드
    /// 0: 전체, 1:보통주, 2:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
}

/// [국내주식 재무비율 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct FinanceRatioRequest {
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20175 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 입력 옵션1
    /// 회계년도 입력 (ex 2023)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2
    /// 0: 1/4분기 , 1: 반기, 2: 3/4분기, 3: 결산
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드
    /// 7: 수익성 분석, 11 : 안정성 분석, 15: 성장성 분석, 20: 활동성 분석
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드
    /// 0
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시간외잔량 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AfterHourBalanceRequest {
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20176 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 순위 정렬 구분 코드
    /// 1: 장전 시간외, 2: 장후 시간외, 3:매도잔량, 4:매수잔량
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 우선주/괴리율 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PreferDisparateRatioRequest {
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20177 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드
    /// 0: 전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
}

/// [국내주식 이격도 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisparityRequest {
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20178 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드
    /// 0: 전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보톧주, 7:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드
    /// 0: 이격도상위순, 1:이격도하위순
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 시간 구분 코드
    /// 5:이격도5, 10:이격도10, 20:이격도20, 60:이격도60, 120:이격도120
    #[serde(rename = "fid_hour_cls_code")]
    pub fid_hour_cls_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
}

/// [국내주식 시장가치 순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketValueRequest {
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20179 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보톧주, 7:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 입력 옵션1
    /// 회계연도 입력 (ex 2023)
    #[serde(rename = "fid_input_option_1")]
    pub fid_input_option_1: String,
    /// 입력 옵션2
    /// 0: 1/4분기 , 1: 반기, 2: 3/4분기, 3: 결산
    #[serde(rename = "fid_input_option_2")]
    pub fid_input_option_2: String,
    /// 순위 정렬 구분 코드
    /// '가치분석(23:PER, 24:PBR, 25:PCR, 26:PSR, 27: EPS, 28:EVA,
    /// 29: EBITDA, 30: EV/EBITDA, 31:EBITDA/금융비율'
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 소속 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 체결강도 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumePowerRequest {
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key( 20168 )
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0: 전체, 1: 보통주 2: 우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
}

/// [국내주식 관심종목등록 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TopInterestStockRequest {
    /// 입력 필수값2
    /// 000000 : 필수입력값
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20180)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 업종 코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0 : 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "fid_input_price_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 분류 구분 코드
    /// 0: 전체 1: 관리종목 2: 투자주의 3: 투자경고 4: 투자위험예고 5: 투자위험 6: 보통주 7: 우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 입력값
    /// 순위검색 입력값(1: 1위부터, 10:10위부터)
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
}

/// [국내주식 예상체결 상승/하락상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpTransUpdownRequest {
    /// 순위 정렬 구분 코드
    /// 0:상승률1:상승폭2:보합3:하락율4:하락폭5:체결량6:거래대금
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20182)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 0:전체 1:보통주 2:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 적용 범위 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
    /// 거래대금
    /// 입력값 없을때 전체 (거래대금 ~) 천원단위
    #[serde(rename = "fid_pbmn")]
    pub fid_pbmn: String,
    /// 소속 구분 코드
    /// 0: 전체
    #[serde(rename = "fid_blng_cls_code")]
    pub fid_blng_cls_code: String,
    /// 장운영 구분 코드
    /// 0:장전예상1:장마감예상
    #[serde(rename = "fid_mkop_cls_code")]
    pub fid_mkop_cls_code: String,
}

/// [국내주식 당사매매종목 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradedByCompanyRequest {
    /// 대상 제외 구분 코드
    /// 0: 전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20186)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드
    /// 0:전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보통주, 7:우선주
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 순위 정렬 구분 코드
    /// 0:매도상위,1:매수상위
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 날짜1
    /// 기간~
    #[serde(rename = "fid_input_date_1")]
    pub fid_input_date_1: String,
    /// 입력 날짜2
    /// ~기간
    #[serde(rename = "fid_input_date_2")]
    pub fid_input_date_2: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드
    /// 0: 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 적용 범위 거래량
    /// 0: 전체, 100: 100주 이상
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: String,
    /// 적용 범위 가격2
    /// ~ 가격
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// 적용 범위 가격1
    /// 가격 ~
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
}

/// [국내주식 신고/신저근접종목 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NearNewHighlowRequest {
    /// 적용 범위 거래량
    /// 0: 전체, 100: 100주 이상
    #[serde(rename = "fid_aply_rang_vol")]
    pub fid_aply_rang_vol: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20187)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 분류 구분 코드
    /// 0:전체, 1:관리종목, 2:투자주의, 3:투자경고
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 수1
    /// 괴리율 최소
    #[serde(rename = "fid_input_cnt_1")]
    pub fid_input_cnt_1: String,
    /// 입력 수2
    /// 괴리율 최대
    #[serde(rename = "fid_input_cnt_2")]
    pub fid_input_cnt_2: String,
    /// 가격 구분 코드
    /// 0:신고근접, 1:신저근접
    #[serde(rename = "fid_prc_cls_code")]
    pub fid_prc_cls_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 대상 구분 코드
    /// 0: 전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 0:전체, 1:관리종목, 2:투자주의, 3:투자경고, 4:투자위험예고, 5:투자위험, 6:보통주, 7:우선주
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 적용 범위 가격1
    /// 가격 ~
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 적용 범위 가격2
    /// ~ 가격
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
}

/// [국내주식 배당률 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DividendRateRequest {
    /// CTS_AREA
    /// 공백
    #[serde(rename = "CTS_AREA")]
    pub cts_area: String,
    /// KOSPI
    /// 0:전체, 1:코스피, 2: 코스피200, 3: 코스닥,
    #[serde(rename = "GB1")]
    pub gb1: String,
    /// 업종구분
    /// '코스피(0001:종합, 0002:대형주.…0027:제조업 ),
    /// 코스닥(1001:종합, …. 1041:IT부품
    /// 코스피200 (2001:KOSPI200, 2007:KOSPI100, 2008:KOSPI50)'
    #[serde(rename = "UPJONG")]
    pub upjong: String,
    /// 종목선택
    /// 0:전체, 6:보통주, 7:우선주
    #[serde(rename = "GB2")]
    pub gb2: String,
    /// 배당구분
    /// 1:주식배당, 2: 현금배당
    #[serde(rename = "GB3")]
    pub gb3: String,
    /// 기준일From
    #[serde(rename = "F_DT")]
    pub f_dt: String,
    /// 기준일To
    #[serde(rename = "T_DT")]
    pub t_dt: String,
    /// 결산/중간배당
    /// 0:전체, 1:결산배당, 2:중간배당
    #[serde(rename = "GB4")]
    pub gb4: String,
}

/// [국내주식 대량체결건수 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BulkTransNumRequest {
    /// 적용 범위 가격2
    /// ~ 가격
    #[serde(rename = "fid_aply_rang_prc_2")]
    pub fid_aply_rang_prc_2: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J:KRX, NX:NXT)
    #[serde(rename = "fid_cond_mrkt_div_code")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(11909)
    #[serde(rename = "fid_cond_scr_div_code")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200, 4001: KRX100
    #[serde(rename = "fid_input_iscd")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드
    /// 0:매수상위, 1:매도상위
    #[serde(rename = "fid_rank_sort_cls_code")]
    pub fid_rank_sort_cls_code: String,
    /// 분류 구분 코드
    /// 0:전체
    #[serde(rename = "fid_div_cls_code")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 건별금액 ~
    #[serde(rename = "fid_input_price_1")]
    pub fid_input_price_1: String,
    /// 적용 범위 가격1
    /// 가격 ~
    #[serde(rename = "fid_aply_rang_prc_1")]
    pub fid_aply_rang_prc_1: String,
    /// 입력 종목코드2
    /// 공백:전체종목, 개별종목 조회시 종목코드 (000660)
    #[serde(rename = "fid_input_iscd_2")]
    pub fid_input_iscd_2: String,
    /// 대상 제외 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_exls_cls_code")]
    pub fid_trgt_exls_cls_code: String,
    /// 대상 구분 코드
    /// 0:전체
    #[serde(rename = "fid_trgt_cls_code")]
    pub fid_trgt_cls_code: String,
    /// 거래량 수
    /// 거래량 ~
    #[serde(rename = "fid_vol_cnt")]
    pub fid_vol_cnt: String,
}

/// [국내주식 신용잔고 상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CreditBalanceRequest {
    /// 조건 화면 분류 코드
    /// Unique key(11701)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:거래소, 1001:코스닥, 2001:코스피200,
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 증가율기간
    /// 2~999
    #[serde(rename = "FID_OPTION")]
    pub fid_option: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 순위 정렬 구분 코드
    /// '(융자)0:잔고비율 상위, 1: 잔고수량 상위, 2: 잔고금액 상위, 3: 잔고비율 증가상위, 4: 잔고비율 감소상위
    /// (대주)5:잔고비율 상위, 6: 잔고수량 상위, 7: 잔고금액 상위, 8: 잔고비율 증가상위, 9: 잔고비율 감소상위 '
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
}

/// [국내주식 공매도 상위종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ShortSaleRequest {
    /// FID 적용 범위 거래량
    /// 공백
    #[serde(rename = "FID_APLY_RANG_VOL")]
    pub fid_aply_rang_vol: String,
    /// 조건 시장 분류 코드
    /// 시장구분코드 (주식 J)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20482)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000:전체, 0001:코스피, 1001:코스닥, 2001:코스피200, 4001: KRX100, 3003: 코스닥150
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회구분 (일/월)
    /// 조회구분 (일/월) D: 일, M:월
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
    /// 조회가간(일수
    /// '조회가간(일수):
    /// 조회구분(D) 0:1일, 1:2일, 2:3일, 3:4일, 4:1주일, 9:2주일, 14:3주일,
    /// 조회구분(M) 1:1개월, 2:2개월, 3:3개월'
    #[serde(rename = "FID_INPUT_CNT_1")]
    pub fid_input_cnt_1: String,
    /// 대상 제외 구분 코드
    /// 공백
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
    /// FID 대상 구분 코드
    /// 공백
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// FID 적용 범위 가격1
    /// 가격 ~
    #[serde(rename = "FID_APLY_RANG_PRC_1")]
    pub fid_aply_rang_prc_1: String,
    /// FID 적용 범위 가격2
    /// ~ 가격
    #[serde(rename = "FID_APLY_RANG_PRC_2")]
    pub fid_aply_rang_prc_2: String,
}

/// [국내주식 시간외등락율순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OvertimeFluctuationRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J: 주식)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 시장 구분 코드
    /// 공백 입력
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20234)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000(전체), 0001(코스피), 1001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 분류 구분 코드
    /// 1(상한가), 2(상승률), 3(보합),4(하한가),5(하락률)
    #[serde(rename = "FID_DIV_CLS_CODE")]
    pub fid_div_cls_code: String,
    /// 입력 가격1
    /// 입력값 없을때 전체 (가격 ~)
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// 입력값 없을때 전체 (~ 가격)
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 입력값 없을때 전체 (거래량 ~)
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드
    /// 공백 입력
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 공백 입력
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 시간외거래량순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OvertimeVolumeRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (J: 주식)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20235)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 입력 종목코드
    /// 0000(전체), 0001(코스피), 1001(코스닥)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 순위 정렬 구분 코드
    /// 0(매수잔량), 1(매도잔량), 2(거래량)
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력 가격1
    /// 가격 ~
    #[serde(rename = "FID_INPUT_PRICE_1")]
    pub fid_input_price_1: String,
    /// 입력 가격2
    /// ~ 가격
    #[serde(rename = "FID_INPUT_PRICE_2")]
    pub fid_input_price_2: String,
    /// 거래량 수
    /// 거래량 ~
    #[serde(rename = "FID_VOL_CNT")]
    pub fid_vol_cnt: String,
    /// 대상 구분 코드
    /// 공백
    #[serde(rename = "FID_TRGT_CLS_CODE")]
    pub fid_trgt_cls_code: String,
    /// 대상 제외 구분 코드
    /// 공백
    #[serde(rename = "FID_TRGT_EXLS_CLS_CODE")]
    pub fid_trgt_exls_cls_code: String,
}

/// [국내주식 실시간체결가 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stcnt0Request {
    /// 거래ID
    /// [실전/모의투자]
    /// H0STCNT0 : 실시간 주식 체결가
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stasp0Request {
    /// 거래ID
    /// [실전/모의투자]
    /// H0STASP0 : 주식호가
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목번호 (6자리)
    /// ETN의 경우, Q로 시작 (EX. Q500001)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결통보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stcni0Request {
    /// 거래ID
    /// '[실전/모의투자]
    /// H0STCNI0 : 국내주식 실시간체결통보
    /// H0STCNI9 : 모의투자 실시간 체결통보
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// HTS ID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stanc0Request {
    /// 거래ID
    /// H0STANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stmbc0Request {
    /// 거래ID
    /// H0STMBC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stpgm0Request {
    /// 거래ID
    /// H0STPGM0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stmko0Request {
    /// 거래ID
    /// H0STMKO0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간호가 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stoaa0Request {
    /// 거래ID
    /// H0STOAA0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간체결가 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stoup0Request {
    /// 거래ID
    /// H0STOUP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 시간외 실시간예상체결 (KRX)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stoac0Request {
    /// 거래ID
    /// H0STOAC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Upcnt0Request {
    /// 거래ID
    /// H0UPCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 업종구분코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간예상체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Upanc0Request {
    /// 거래ID
    /// H0UPANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 업종구분코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내지수 실시간프로그램매매] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Uppgm0Request {
    /// 거래ID
    /// H0UPPGM0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 업종구분코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ewasp0Request {
    /// 거래ID
    /// H0EWASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// ELW 종목코드(ex. 57LA24)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ewcnt0Request {
    /// 거래ID
    /// H0EWCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// ELW 종목코드(ex. 57LA24)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [ELW 실시간예상체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ewanc0Request {
    /// 거래ID
    /// H0EWANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// ELW 종목코드(ex. 57LA24)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내ETF NAV추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Stnav0Request {
    /// 거래ID
    /// H0STNAV0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex. 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결가 (통합)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Uncnt0Request {
    /// 거래ID
    /// H0UNCNT0 : 실시간 주식 체결가 통합
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (통합)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unasp0Request {
    /// 거래ID
    /// H0UNASP0 : 실시간 주식 체결가 통합
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (통합)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unanc0Request {
    /// 거래ID
    /// [실전투자]
    /// H0UNANC0 : 국내주식 실시간예상체결 (통합)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (통합)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unmbc0Request {
    /// 거래ID
    /// H0UNMBC0 : 국내주식 주식종목회원사 (통합)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (통합)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unpgm0Request {
    /// 거래ID
    /// H0UNPGM0 : 실시간 주식종목프로그램매매 통합
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (통합)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Unmko0Request {
    /// 거래ID
    /// H0UNMKO0 : 국내주식 장운영정보 (통합)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간체결가 (NXT)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxcnt0Request {
    /// 거래ID
    /// H0NXCNT0 : 주식종목체결 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간호가 (NXT)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxasp0Request {
    /// 거래ID
    /// H0NXASP0 : 실시간 주식 호가 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간예상체결 (NXT)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxanc0Request {
    /// 거래ID
    /// H0NXANC0 : 국내주식 실시간예상체결 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간회원사 (NXT)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxmbc0Request {
    /// 거래ID
    /// H0NXMBC0 : 국내주식 주식종목회원사 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 실시간프로그램매매 (NXT)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxpgm0Request {
    /// 거래ID
    /// H0NXPGM0 : 실시간 주식프로그램매매 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [국내주식 장운영정보 (NXT)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Nxmko0Request {
    /// 거래ID
    /// H0NXMKO0 : 국내주식 장운영정보 (NXT)
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 종목코드 (ex 005930 삼성전자)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [선물옵션 주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRequest {
    /// 주문처리구분코드
    /// 02 : 주문전송
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 매도매수구분코드
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 단축상품번호
    /// 종목번호
    /// 선물 6자리 (예: A01603)
    /// 옵션 9자리 (예: B01603955)
    #[serde(rename = "SHTN_PDNO")]
    pub shtn_pdno: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 주문가격1
    /// 시장가나 최유리 지정가인 경우 0으로 입력
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드
    /// ※ ORD_DVSN_CD(주문구분코드)를 입력한 경우 ""(공란)으로 입력해도 됨
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// 한국거래소호가조건코드
    /// ※ ORD_DVSN_CD(주문구분코드)를 입력한 경우 ""(공란)으로 입력해도 됨
    /// 0 : 없음
    /// 3 : IOC
    /// 4 : FOK
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// 연락전화번호
    /// 고객의 연락 가능한 전화번호
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 선물옵션종목구분코드
    /// 공란(Default)
    #[serde(rename = "FUOP_ITEM_DVSN_CD")]
    pub fuop_item_dvsn_cd: String,
    /// 주문구분코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclNextRequest {
    /// 주문처리구분코드
    /// 02 : 주문전송
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 정정취소구분코드
    /// 01 : 정정
    /// 02 : 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 원주문번호
    /// 정정 혹은 취소할 주문의 번호
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문수량
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
    /// 주문가격1
    /// 시장가나 최유리의 경우 0으로 입력 (취소 시에도 0 입력)
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 호가유형코드
    /// 01 : 지정가
    /// 02 : 시장가
    /// 03 : 조건부
    /// 04 : 최유리
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    /// 한국거래소호가조건코드
    /// 취소시 0으로 입력
    /// 정정시
    /// 0 : 없음
    /// 3 : IOC
    /// 4 : FOK
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    /// 잔여수량여부
    /// Y : 전량
    /// N : 일부
    #[serde(rename = "RMN_QTY_YN")]
    pub rmn_qty_yn: String,
    /// 선물옵션종목구분코드
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
    /// 주문구분코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작주문일자
    /// 주문내역 조회 시작 일자, YYYYMMDD
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// 종료주문일자
    /// 주문내역 조회 마지막 일자, YYYYMMDD
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// 매도매수구분코드
    /// 00 : 전체
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분
    /// 00 : 전체
    /// 01 : 체결
    /// 02 : 미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 정렬순서
    /// AS : 정순
    /// DS : 역순
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 시작주문번호
    /// 조회 시작 번호 입력
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// 상품번호
    /// 공란 시, 전체 조회
    /// 선물 6자리 (예: 101S03)
    /// 옵션 9자리 (예: 201S03370)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 시장ID코드
    /// 공란(Default)
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 잔고현황] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금 구분
    /// 01 : 개시
    /// 02 : 유지
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드
    /// 1 : 정산 (정산가격으로 잔고 조회)
    /// 2 : 본정산 (매입가격으로 잔고 조회)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 주문가능] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// 선물옵션종목코드
    /// 선물 6자리 (예: 101S03)
    /// 옵션 9자리 (예: 201S03370)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문가격1
    /// 주문가격
    /// ※ 주문가격 '0'일 경우
    /// - 옵션매수 : 현재가
    /// - 그 이외 : 기준가
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 주문구분코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireNgtCcnlRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작주문일자
    #[serde(rename = "STRT_ORD_DT")]
    pub strt_ord_dt: String,
    /// 종료주문일자
    /// 조회하려는 마지막 일자 다음일자로 조회
    /// (ex. 20221011 까지의 내역을 조회하고자 할 경우,
    /// 20221012로 종료주문일자 설정)
    #[serde(rename = "END_ORD_DT")]
    pub end_ord_dt: String,
    /// 매도매수구분코드
    /// 공란 : default (00: 전체 ,01 : 매도, 02 : 매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 체결미체결구분
    /// 00 : 전체
    /// 01 : 체결
    /// 02 : 미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 정렬순서
    /// 공란 : default (DS : 정순, 그외 : 역순)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 시작주문번호
    /// 공란 : default
    #[serde(rename = "STRT_ODNO")]
    pub strt_odno: String,
    /// 상품번호
    /// 공란 : default
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 시장ID코드
    /// 공란 : default
    #[serde(rename = "MKET_ID_CD")]
    pub mket_id_cd: String,
    /// 선물옵션구분코드
    /// 공란 : 전체, 01 : 선물, 02 : 옵션
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// 화면구분
    /// 02(Default)
    #[serde(rename = "SCRN_DVSN")]
    pub scrn_dvsn: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [(야간)선물옵션 잔고현황] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireNgtBalanceRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 계좌비밀번호
    /// 공란("")으로 조회
    #[serde(rename = "ACNT_PWD")]
    pub acnt_pwd: String,
    /// 증거금구분
    /// 01 : 개시, 02 : 유지
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드
    /// 1 : 정산 (정산가격으로 잔고 조회)
    /// 2 : 본정산 (매입가격으로 잔고 조회)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [(야간)선물옵션 주문가능 조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblNgtOrderRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// 301 : 선물옵션
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 매도매수구분코드
    /// 01 : 매도 , 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 주문가격1
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    /// 주문구분코드
    /// '01 : 지정가 02 : 시장가
    /// 03 : 조건부 04 : 최유리,
    /// 10 : 지정가(IOC) 11 : 지정가(FOK)
    /// 12 : 시장가(IOC) 13 : 시장가(FOK)
    /// 14 : 최유리(IOC) 15 : 최유리(FOK)'
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

/// [(야간)선물옵션 증거금 상세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NgtMarginDetailRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금 구분코드
    /// 위탁(01), 유지(02)
    #[serde(rename = "MGNA_DVSN_CD")]
    pub mgna_dvsn_cd: String,
}

/// [선물옵션 잔고정산손익내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceSettlementPlRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회일자
    /// 조회일자(YYYYMMDD)
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
    /// 연속조회검색조건200
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 총자산현황] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDepositNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
}

/// [선물옵션 잔고평가손익내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceValuationPlRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 증거금구분
    /// 01 : 개시, 02 : 유지
    #[serde(rename = "MGNA_DVSN")]
    pub mgna_dvsn: String,
    /// 정산상태코드
    /// 1 : 정산 (정산가격으로 잔고 조회)
    /// 2 : 본정산 (매입가격으로 잔고 조회)
    #[serde(rename = "EXCC_STAT_CD")]
    pub excc_stat_cd: String,
    /// 연속조회검색조건200
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 기준일체결내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlBstimeRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자
    /// 주문일자(YYYYMMDD)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 선물옵션거래시작시각
    /// 선물옵션거래시작시간(HHMMSS)
    #[serde(rename = "FUOP_TR_STRT_TMD")]
    pub fuop_tr_strt_tmd: String,
    /// 선물옵션거래종료시각
    /// 선물옵션거래종료시간(HHMMSS)
    #[serde(rename = "FUOP_TR_END_TMD")]
    pub fuop_tr_end_tmd: String,
    /// 연속조회검색조건200
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션기간약정수수료일별] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyAmountFeeRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일
    /// 조회시작일(YYYYMMDD)
    #[serde(rename = "INQR_STRT_DAY")]
    pub inqr_strt_day: String,
    /// 조회종료일
    /// 조회종료일(YYYYMMDD)
    #[serde(rename = "INQR_END_DAY")]
    pub inqr_end_day: String,
    /// 연속조회검색조건200
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 증거금률] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarginRateRequest {
    /// 기준일자
    /// 날짜 입력) ex) 20260313
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 기초자산ID
    /// 공백 입력
    #[serde(rename = "BAST_ID")]
    pub bast_id: String,
    /// 연속조회키200
    /// 다음 조회 시 필요, 입력 후 header tr_cont : N 설정 필수
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [선물옵션 시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceNextNextRequest {
    /// FID 조건 시장 분류 코드
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목코드 (예: 101S03)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [선물옵션 시세호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceRequest {
    /// FID 조건 시장 분류 코드
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목코드 (예: 101S03)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [선물옵션기간별시세(일/주/월/년)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyFuopchartpriceRequest {
    /// FID 조건 시장 분류 코드
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션,
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드
    /// 종목번호 (지수선물:6자리, 지수옵션 9자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조회 시작일자
    /// 조회 시작일자 (ex. 20220401)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 조회 종료일자
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
    /// 기간분류코드
    /// D:일봉 W:주봉, M:월봉, Y:년봉
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [선물옵션 분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeFuopchartpriceRequest {
    /// FID 조건 시장 분류 코드
    /// F: 지수선물, O:지수옵션
    /// JF: 주식선물, JO:주식옵션,
    /// CF: 상품선물(금), 금리선물(국채), 통화선물(달러)
    /// CM: 야간선물, EU: 야간옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목번호 (지수선물:6자리, 지수옵션 9자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 시간 구분 코드
    /// FID 시간 구분 코드(30: 30초, 60: 1분, 3600: 1시간)
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// FID 과거 데이터 포함 여부
    /// Y(과거) / N (당일)
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
    /// FID 허봉 포함 여부
    /// N으로 입력
    #[serde(rename = "FID_FAKE_TICK_INCU_YN")]
    pub fid_fake_tick_incu_yn: String,
    /// FID 입력 날짜1
    /// 입력 날짜 기준으로 이전 기간 조회(YYYYMMDD)
    /// ex) 20230908 입력 시, 2023년 9월 8일부터 일자 역순으로 조회
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 시간1
    /// 입력 시간 기준으로 이전 시간 조회(HHMMSS)
    /// ex) 093000 입력 시, 오전 9시 30분부터 역순으로 분봉 조회
    ///
    /// * CM(야간선물), EU(야간옵션)인 경우, 자정 이후 시간은 +24시간으로 입력
    /// ex) 253000 입력 시, 새벽 1시 30분부터 역순으로 분봉 조회
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
}

/// [국내옵션전광판_옵션월물리스트] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardOptionListRequest {
    /// 조건 화면 분류 코드
    /// Unique key(509)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 분류 코드
    /// 공백
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 시장 구분 코드
    /// 공백
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [국내선물 기초자산 시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardTopRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (F: 선물)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 선물최근월물 ex)(101V06)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드
    /// 공백
    #[serde(rename = "FID_COND_MRKT_DIV_CODE1")]
    pub fid_cond_mrkt_div_code1: String,
    /// 조건 화면 분류 코드
    /// 공백
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 만기 수
    /// 공백
    #[serde(rename = "FID_MTRT_CNT")]
    pub fid_mtrt_cnt: String,
    /// 조건 시장 구분 코드
    /// 공백
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [국내옵션전광판_콜풋] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardCallputRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (O: 옵션)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20503)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 시장 구분 코드
    /// 시장구분코드 (CO: 콜옵션)
    #[serde(rename = "FID_MRKT_CLS_CODE")]
    pub fid_mrkt_cls_code: String,
    /// 만기 수
    /// - FID_COND_MRKT_CLS_CODE : 공백(KOSPI200), MKI(미니KOSPI200), KQI(KOSDAQ150) 인 경우
    /// : 만기년월(YYYYMM) 입력 (ex. 202407)
    ///
    /// - FID_COND_MRKT_CLS_CODE : WKM(KOSPI200위클리(월)), WKI(KOSPI200위클리(목)) 인 경우
    /// : 만기년월주차(YYMMWW) 입력
    /// (ex. 2024년도 7월 3주차인 경우, 240703 입력)
    #[serde(rename = "FID_MTRT_CNT")]
    pub fid_mtrt_cnt: String,
    /// 조건 시장 구분 코드
    /// 공백: KOSPI200
    /// MKI: 미니KOSPI200
    /// WKM: KOSPI200위클리(월)
    /// WKI: KOSPI200위클리(목)
    /// KQI: KOSDAQ150
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 시장 구분 코드
    /// 시장구분코드 (PO: 풋옵션)
    #[serde(rename = "FID_MRKT_CLS_CODE1")]
    pub fid_mrkt_cls_code1: String,
}

/// [국내옵션전광판_선물] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DisplayBoardFuturesRequest {
    /// 조건 시장 분류 코드
    /// 시장구분코드 (F: 선물)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 조건 화면 분류 코드
    /// Unique key(20503)
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
    /// 조건 시장 구분 코드
    /// 공백: KOSPI200
    /// MKI: 미니KOSPI200
    /// WKM: KOSPI200위클리(월)
    /// WKI: KOSPI200위클리(목)
    /// KQI: KOSDAQ150
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
}

/// [선물옵션 일중예상체결추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ExpPriceTrendNextRequest {
    /// 입력 종목코드
    /// 종목번호 (지수선물:6자리, 지수옵션 9자리)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 조건 시장 분류 코드
    /// F : 지수선물, O : 지수옵션
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
}

/// [지수선물 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ifasp0Request {
    /// 거래ID
    /// H0IFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드
    /// 예:101S12
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수선물 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ifcnt0Request {
    /// 거래ID
    /// H0IFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드
    /// 예:101S12
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수옵션 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ioasp0Request {
    /// 거래ID
    /// H0IOASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드
    /// 예:201S11305
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [지수옵션 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Iocnt0Request {
    /// 거래ID
    /// H0IOCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드
    /// 예:201S11305
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [선물옵션 실시간체결통보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Ifcni0Request {
    /// 거래ID
    /// [실전투자]
    /// H0IFCNI0 : 실시간 선물옵션 체결통보
    ///
    /// [모의투자]
    /// H0IFCNI9 : 실시간 선물옵션 체결통보
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 코드
    /// 예:101S12
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [상품선물 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Cfasp0Request {
    /// 거래ID
    /// H0CFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [상품선물 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Cfcnt0Request {
    /// 거래ID
    /// H0CFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zfasp0Request {
    /// 거래ID
    /// H0ZFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zfcnt0Request {
    /// 거래ID
    /// H0ZFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식선물 실시간예상체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zfanc0Request {
    /// 거래ID
    /// H0ZFANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 주식선물 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zoasp0Request {
    /// 거래ID
    /// H0ZOASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zocnt0Request {
    /// 거래ID
    /// H0ZOCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [주식옵션 실시간예상체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Zoanc0Request {
    /// 거래ID
    /// H0ZOANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 주식옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Euasp0Request {
    /// 거래ID
    /// H0EUASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 야간옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Eucnt0Request {
    /// 거래ID
    /// H0EUCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 야간옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션실시간예상체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Euanc0Request {
    /// 거래ID
    /// H0EUANC0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 야간옵션 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간옵션실시간체결통보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Eucni0Request {
    /// 거래ID
    /// H0MFCNI0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// HTS ID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Mfasp0Request {
    /// 거래ID
    /// H0MFASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 야간선물 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간종목체결] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Mfcnt0Request {
    /// 거래ID
    /// H0MFCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 야간선물 종목코드
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [KRX야간선물 실시간체결통보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Mfcni0Request {
    /// 거래ID
    /// H0MFCNI0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// HTS ID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외주식 주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
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
    /// 상품번호
    /// 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량
    /// 주문수량 (해외거래소 별 최소 주문수량 및 주문단위 확인 필요)
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가
    /// 1주당 가격
    /// * 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 판매유형
    /// 제거 : 매수
    /// 00 : 매도
    #[serde(rename = "SLL_TYPE")]
    pub sll_type: String,
    /// 주문서버구분코드
    /// "0"(Default)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분
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
    /// 시작시간
    /// ※ TWAP, VWAP 주문유형이고 알고리즘주문시간구분코드가 00일때 사용
    /// ※ YYMMDD 형태로 입력
    /// ※ 시간 입력 시 정규장 종료 5분전까지 입력 가능
    #[serde(rename = "START_TIME")]
    pub start_time: String,
    /// 종료시간
    /// ※ TWAP, VWAP 주문유형이고 알고리즘주문시간구분코드가 00일때 사용
    /// ※ YYMMDD 형태로 입력
    /// ※ 시간 입력 시 정규장 종료 5분전까지 입력 가능
    #[serde(rename = "END_TIME")]
    pub end_time: String,
    /// 알고리즘주문시간구분코드
    /// 00 : 분할주문 시간 직접입력 , 02 : 정규장 종료시까지
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD")]
    pub algo_ord_tmd_dvsn_cd: String,
}

/// [해외주식 정정취소주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
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
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호
    /// 정정 또는 취소할 원주문번호
    /// (해외주식_주문 API ouput ODNO
    /// or 해외주식 미체결내역 API output ODNO 참고)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 정정취소구분코드
    /// 01 : 정정
    /// 02 : 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가
    /// 취소주문 시, "0" 입력
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 운용사지정주문번호
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드
    /// "0"(Default)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

/// [해외주식 예약주문접수] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 매도매수구분코드
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 정정취소구분코드
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    /// 00 : "매도/매수 주문"시 필수 항목
    /// 02 : 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    /// 515 : 일본
    /// 501 : 홍콩 / 543 : 홍콩CNY / 558 : 홍콩USD
    /// 507 : 베트남 하노이거래소 / 508 : 베트남 호치민거래소
    /// 551 : 중국 상해A / 552 : 중국 심천A
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 해외거래소코드
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
    /// FT주문수량
    #[serde(rename = "FT_ORD_QTY")]
    pub ft_ord_qty: String,
    /// FT주문단가3
    #[serde(rename = "FT_ORD_UNPR3")]
    pub ft_ord_unpr3: String,
    /// 주문서버구분코드
    /// "0"(Default)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 예약주문접수일자
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    #[serde(rename = "RSVN_ORD_RCIT_DT")]
    pub rsvn_ord_rcit_dt: String,
    /// 주문구분
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
    /// 해외예약주문번호
    /// tr_id가 TTTS3013U(중국/홍콩/일본/베트남 예약 주문)인 경우만 사용
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
    /// 알고리즘주문시간구분코드
    /// ※ TWAP, VWAP 주문에서만 사용. 예약주문은 시간입력 불가하여 02로 값 고정
    /// ※ 정규장 종료 10분전까지 가능
    #[serde(rename = "ALGO_ORD_TMD_DVSN_CD")]
    pub algo_ord_tmd_dvsn_cd: String,
}

/// [해외주식 예약주문접수취소] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvCcnlNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외주문접수일자
    #[serde(rename = "RSYN_ORD_RCIT_DT")]
    pub rsyn_ord_rcit_dt: String,
    /// 해외예약주문번호
    /// 해외주식_예약주문접수 API Output ODNO(주문번호) 참고
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
}

/// [해외주식 매수가능금액조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsamountRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
    /// NASD : 나스닥 / NYSE : 뉴욕 / AMEX : 아멕스
    /// SEHK : 홍콩 / SHAA : 중국상해 / SZAA : 중국심천
    /// TKSE : 일본 / HASE : 하노이거래소 / VNSE : 호치민거래소
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 해외주문단가
    /// 해외주문단가 (23.8) 정수부분 23자리, 소수부분 8자리
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 종목코드
    /// 종목코드
    #[serde(rename = "ITEM_CD")]
    pub item_cd: String,
}

/// [해외주식 미체결내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireNccsRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
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
    /// 정렬순서
    /// DS : 정순
    /// 그외 : 역순
    ///
    /// [header tr_id: TTTS3018R]
    /// ""(공란)
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 잔고] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceNextNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
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
    /// 거래통화코드
    /// USD : 미국달러
    /// HKD : 홍콩달러
    /// CNY : 중국위안화
    /// JPY : 일본엔화
    /// VND : 베트남동
    #[serde(rename = "TR_CRCY_CD")]
    pub tr_crcy_cd: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 주문체결내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// 전종목일 경우 "%" 입력
    /// ※ 모의투자계좌의 경우 ""(전체 조회)만 가능
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문시작일자
    /// YYYYMMDD 형식 (현지시각 기준)
    #[serde(rename = "ORD_STRT_DT")]
    pub ord_strt_dt: String,
    /// 주문종료일자
    /// YYYYMMDD 형식 (현지시각 기준)
    #[serde(rename = "ORD_END_DT")]
    pub ord_end_dt: String,
    /// 매도매수구분
    /// 00 : 전체
    /// 01 : 매도
    /// 02 : 매수
    /// ※ 모의투자계좌의 경우 "00"(전체 조회)만 가능
    #[serde(rename = "SLL_BUY_DVSN")]
    pub sll_buy_dvsn: String,
    /// 체결미체결구분
    /// 00 : 전체
    /// 01 : 체결
    /// 02 : 미체결
    /// ※ 모의투자계좌의 경우 "00"(전체 조회)만 가능
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 해외거래소코드
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
    /// 정렬순서
    /// DS : 정순
    /// AS : 역순
    /// ※ 모의투자계좌의 경우 정렬순서 사용불가(Default : DS(정순))
    #[serde(rename = "SORT_SQN")]
    pub sort_sqn: String,
    /// 주문일자
    /// "" (Null 값 설정)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문채번지점번호
    /// "" (Null 값 설정)
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호
    /// "" (Null 값 설정)
    /// ※ 주문번호로 검색 불가능합니다. 반드시 ""(Null 값 설정) 바랍니다.
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 체결기준현재잔고] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePresentBalanceNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 원화외화구분코드
    /// 01 : 원화
    /// 02 : 외화
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 국가코드
    /// 000 전체
    /// 840 미국
    /// 344 홍콩
    /// 156 중국
    /// 392 일본
    /// 704 베트남
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 거래시장코드
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
    /// 조회구분코드
    /// 00 : 전체
    /// 01 : 일반해외주식
    /// 02 : 미니스탁
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
}

/// [해외주식 예약주문조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderResvListRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자
    /// 조회시작일자(YYYYMMDD)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// 조회종료일자(YYYYMMDD)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 조회구분코드
    /// 00 : 전체
    /// 01 : 일반해외주식
    /// 02 : 미니스탁
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// 상품유형코드
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
    /// 해외거래소코드
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
    /// 연속조회검색조건200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK200값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외주식 결제기준잔고] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePaymtStdrBalanceRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 기준일자
    #[serde(rename = "BASS_DT")]
    pub bass_dt: String,
    /// 원화외화구분코드
    /// 01(원화기준),02(외화기준)
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 조회구분코드
    /// 00(전체), 01(일반), 02(미니스탁)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
}

/// [해외주식 일별거래내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodTransRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 등록시작일자
    /// 입력날짜 ~ (ex) 20240420)
    #[serde(rename = "ERLM_STRT_DT")]
    pub erlm_strt_dt: String,
    /// 등록종료일자
    /// ~입력날짜 (ex) 20240520)
    #[serde(rename = "ERLM_END_DT")]
    pub erlm_end_dt: String,
    /// 해외거래소코드
    /// 공백
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호
    /// 공백 (전체조회), 개별종목 조회는 상품번호입력
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도매수구분코드
    /// 00(전체), 01(매도), 02(매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 대출구분코드
    /// 공백
    #[serde(rename = "LOAN_DVSN_CD")]
    pub loan_dvsn_cd: String,
    /// 연속조회검색조건100
    /// 공백
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// 공백
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외주식 기간손익] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodProfitNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
    /// 공란 : 전체,
    /// NASD : 미국, SEHK : 홍콩,
    /// SHAA : 중국, TKSE : 일본, HASE : 베트남
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 국가코드
    /// 공란(Default)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 통화코드
    /// 공란 : 전체
    /// USD : 미국달러, HKD : 홍콩달러,
    /// CNY : 중국위안화, JPY : 일본엔화, VND : 베트남동
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 상품번호
    /// 공란 : 전체
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 조회시작일자
    /// YYYYMMDD
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// YYYYMMDD
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 원화외화구분코드
    /// 01 : 외화, 02 : 원화
    #[serde(rename = "WCRC_FRCR_DVSN_CD")]
    pub wcrc_frcr_dvsn_cd: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외증거금 통화별조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ForeignMarginRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
}

/// [해외주식 미국주간주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DaytimeOrderRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
    /// NASD:나스닥 / NYSE:뉴욕 / AMEX:아멕스
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호
    /// 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량
    /// 해외거래소 별 최소 주문수량 및 주문단위 확인 필요
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가
    /// 소수점 포함, 1주당 가격
    /// * 시장가의 경우 1주당 가격을 공란으로 비우지 않음 "0"으로 입력
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호
    /// " "
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호
    /// " "
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드
    /// "0"
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 주문구분
    /// [미국 매수/매도 주문]
    /// 00 : 지정가
    /// * 주간거래는 지정가만 가능
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
}

/// [해외주식 미국주간정정취소] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DaytimeOrderRvsecnclRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외거래소코드
    /// NASD:나스닥 / NYSE:뉴욕 / AMEX:아멕스
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    /// 상품번호
    /// 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호
    /// '정정 또는 취소할 원주문번호(매매 TR의 주문번호)
    /// - 해외주식 주문체결내역api (/uapi/overseas-stock/v1/trading/inquire-nccs)에서 odno(주문번호) 참조'
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 정정취소구분코드
    /// '01 : 정정
    /// 02 : 취소'
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 주문수량
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 해외주문단가
    /// 소수점 포함, 1주당 가격
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    /// 연락전화번호
    /// " "
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    /// 운용사지정주문번호
    /// " "
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드
    /// "0"
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

/// [해외주식 지정가주문번호조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AlgoOrdnoRequest {
    /// 거래일자
    /// YYYYMMDD
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// 계좌번호
    /// 종합계좌번호 (8자리)
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌상품코드 (2자리) : 주식계좌는 01
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 지정가체결내역조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAlgoCcnlRequest {
    /// 계좌번호
    /// 종합계좌번호 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 상품코드 2자리 (주식계좌 : 01)
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자
    /// 주문일자 (YYYYMMDD)
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문채번지점번호
    /// TTS6058R 조회 시 해당 주문번호(odno)의 ord_gno_brno 입력
    #[serde(rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    /// 주문번호
    /// 지정가주문번호 (TTTS6058R)에서 조회된 주문번호 입력
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 집계포함여부
    #[serde(rename = "TTLZ_ICLD_YN")]
    pub ttlz_icld_yn: String,
    /// 연속조회키200
    /// 연속조회 시 사용
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회조건200
    /// 연속조회 시 사용
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외주식 현재가상세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PriceDetailRequest {
    /// 사용자권한정보
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소명
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
    /// 종목코드
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 현재가 호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceNextRequest {
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
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
    /// 종목코드
    /// 종목코드 예)TSLA
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 현재체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PriceRequest {
    /// 사용자권한정보
    /// "" (Null 값 설정)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
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
    /// 종목코드
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식 체결추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlNextNextNextRequest {
    /// 거래소명
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 당일전일구분
    /// 0:전일, 1:당일
    #[serde(rename = "TDAY")]
    pub tday: String,
    /// 종목코드
    /// 해외종목코드
    #[serde(rename = "SYMB")]
    pub symb: String,
}

/// [해외주식분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeItemchartpriceNextRequest {
    /// 사용자권한정보
    /// "" 공백으로 입력
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
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
    /// 종목코드
    /// 종목코드(ex. TSLA)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 분갭
    /// 분단위(1: 1분봉, 2: 2분봉, ...)
    #[serde(rename = "NMIN")]
    pub nmin: String,
    /// 전일포함여부
    /// 0:당일 1:전일포함
    /// ※ 다음조회 시 반드시 "1"로 입력
    #[serde(rename = "PINC")]
    pub pinc: String,
    /// 다음여부
    /// 처음조회 시, "" 공백 입력
    /// 다음조회 시, "1" 입력
    #[serde(rename = "NEXT")]
    pub next: String,
    /// 요청갯수
    /// 레코드요청갯수 (최대 120)
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// 미체결채움구분
    /// "" 공백으로 입력
    #[serde(rename = "FILL")]
    pub fill: String,
    /// NEXT KEY BUFF
    /// 처음 조회 시, "" 공백 입력
    /// 다음 조회 시, 이전 조회 결과의 마지막 분봉 데이터를 이용하여, 1분 전 혹은 n분 전의 시간을 입력
    /// (형식: YYYYMMDDHHMMSS, ex. 20241014140100)
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외지수분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeIndexchartpriceNextRequest {
    /// 조건 시장 분류 코드
    /// N 해외지수
    /// X 환율
    /// KX 원화환율
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목번호(ex. TSLA)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 시간 구분 코드
    /// 0: 정규장, 1: 시간외
    #[serde(rename = "FID_HOUR_CLS_CODE")]
    pub fid_hour_cls_code: String,
    /// 과거 데이터 포함 여부
    /// Y/N
    #[serde(rename = "FID_PW_DATA_INCU_YN")]
    pub fid_pw_data_incu_yn: String,
}

/// [해외주식 기간별시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailypriceRequest {
    /// 사용자권한정보
    /// "" (Null 값 설정)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
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
    /// 종목코드
    /// 종목코드 (ex. TSLA)
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 일/주/월구분
    /// 0 : 일
    /// 1 : 주
    /// 2 : 월
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// 조회기준일자
    /// 조회기준일자(YYYYMMDD)
    /// ※ 공란 설정 시, 기준일 오늘 날짜로 설정
    #[serde(rename = "BYMD")]
    pub bymd: String,
    /// 수정주가반영여부
    /// 0 : 미반영
    /// 1 : 반영
    #[serde(rename = "MODP")]
    pub modp: String,
    /// NEXT KEY BUFF
    /// 응답시 다음값이 있으면 값이 셋팅되어 있으므로 다음 조회시 응답값 그대로 셋팅
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외주식 종목/지수/환율기간별시세(일/주/월/년)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyChartpriceRequest {
    /// FID 조건 시장 분류 코드
    /// N: 해외지수, X 환율, I: 국채, S:금선물
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// FID 입력 종목코드
    /// 종목코드
    /// ※ 해외주식 마스터 코드 참조
    /// (포럼 > FAQ > 종목정보 다운로드(해외) > 해외지수)
    ///
    /// ※ 해당 API로 미국주식 조회 시, 다우30, 나스닥100, S&P500 종목만 조회 가능합니다. 더 많은 미국주식 종목 시세를 이용할 시에는, 해외주식기간별시세 API 사용 부탁드립니다.
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// FID 입력 날짜1
    /// 시작일자(YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// FID 입력 날짜2
    /// 종료일자(YYYYMMDD)
    #[serde(rename = "FID_INPUT_DATE_2")]
    pub fid_input_date_2: String,
    /// FID 기간 분류 코드
    /// D:일, W:주, M:월, Y:년
    #[serde(rename = "FID_PERIOD_DIV_CODE")]
    pub fid_period_div_code: String,
}

/// [해외주식조건검색] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireSearchRequest {
    /// 사용자권한정보
    /// "" (Null 값 설정)
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 현재가선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_PRICECUR")]
    pub co_yn_pricecur: String,
    /// 현재가시작범위가
    /// 단위: 각국통화(JPY, USD, HKD, CNY, VND)
    #[serde(rename = "CO_ST_PRICECUR")]
    pub co_st_pricecur: String,
    /// 현재가끝범위가
    /// 단위: 각국통화(JPY, USD, HKD, CNY, VND)
    #[serde(rename = "CO_EN_PRICECUR")]
    pub co_en_pricecur: String,
    /// 등락율선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_RATE")]
    pub co_yn_rate: String,
    /// 등락율시작율
    /// %
    #[serde(rename = "CO_ST_RATE")]
    pub co_st_rate: String,
    /// 등락율끝율
    /// %
    #[serde(rename = "CO_EN_RATE")]
    pub co_en_rate: String,
    /// 시가총액선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_VALX")]
    pub co_yn_valx: String,
    /// 시가총액시작액
    /// 단위: 천
    #[serde(rename = "CO_ST_VALX")]
    pub co_st_valx: String,
    /// 시가총액끝액
    /// 단위: 천
    #[serde(rename = "CO_EN_VALX")]
    pub co_en_valx: String,
    /// 발행주식수선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_SHAR")]
    pub co_yn_shar: String,
    /// 발행주식시작수
    /// 단위: 천
    #[serde(rename = "CO_ST_SHAR")]
    pub co_st_shar: String,
    /// 발행주식끝수
    /// 단위: 천
    #[serde(rename = "CO_EN_SHAR")]
    pub co_en_shar: String,
    /// 거래량선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_VOLUME")]
    pub co_yn_volume: String,
    /// 거래량시작량
    /// 단위: 주
    #[serde(rename = "CO_ST_VOLUME")]
    pub co_st_volume: String,
    /// 거래량끝량
    /// 단위: 주
    #[serde(rename = "CO_EN_VOLUME")]
    pub co_en_volume: String,
    /// 거래대금선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_AMT")]
    pub co_yn_amt: String,
    /// 거래대금시작금
    /// 단위: 천
    #[serde(rename = "CO_ST_AMT")]
    pub co_st_amt: String,
    /// 거래대금끝금
    /// 단위: 천
    #[serde(rename = "CO_EN_AMT")]
    pub co_en_amt: String,
    /// EPS선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_EPS")]
    pub co_yn_eps: String,
    /// EPS시작
    #[serde(rename = "CO_ST_EPS")]
    pub co_st_eps: String,
    /// EPS끝
    #[serde(rename = "CO_EN_EPS")]
    pub co_en_eps: String,
    /// PER선택조건
    /// 해당조건 사용시(1), 미사용시 필수항목아님
    #[serde(rename = "CO_YN_PER")]
    pub co_yn_per: String,
    /// PER시작
    #[serde(rename = "CO_ST_PER")]
    pub co_st_per: String,
    /// PER끝
    #[serde(rename = "CO_EN_PER")]
    pub co_en_per: String,
    /// NEXT KEY BUFF
    /// "" 공백 입력
    #[serde(rename = "KEYB")]
    pub keyb: String,
}

/// [해외결제일자조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct CountriesHolidayRequest {
    /// 기준일자
    /// 기준일자(YYYYMMDD)
    #[serde(rename = "TRAD_DT")]
    pub trad_dt: String,
    /// 연속조회키
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_NK")]
    pub ctx_area_nk: String,
    /// 연속조회검색조건
    /// 공백으로 입력
    #[serde(rename = "CTX_AREA_FK")]
    pub ctx_area_fk: String,
}

/// [해외주식 상품기본정보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchInfoNextRequest {
    /// 상품유형코드
    /// 512 미국 나스닥 / 513 미국 뉴욕 / 529 미국 아멕스
    /// 515 일본
    /// 501 홍콩 / 543 홍콩CNY / 558 홍콩USD
    /// 507 베트남 하노이 / 508 베트남 호치민
    /// 551 중국 상해A / 552 중국 심천A
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 상품번호
    /// 예) AAPL (애플)
    #[serde(rename = "PDNO")]
    pub pdno: String,
}

/// [해외주식 업종별시세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndustryThemeRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 업종코드
    /// 업종코드별조회(HHDFS76370100) 를 통해 확인
    #[serde(rename = "ICOD")]
    pub icod: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 업종별코드조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IndustryPriceRequest {
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
}

/// [해외주식 복수종목 시세조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MultpriceRequest {
    /// 사용자권한정보
    /// 공백 입력 필수
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 종목요청개수
    /// 최대 10
    #[serde(rename = "NREC")]
    pub nrec: String,
    /// 거래소코드
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
    /// 종목코드
    /// API 문서 -> 종목정보파일 -> 마스터 파일 참조
    #[serde(rename = "SYMB_01 ~ 10")]
    pub symb_01_10: String,
}

/// [해외주식 가격급등락] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PriceFluctRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 급등/급락구분
    /// 0(급락), 1(급등)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// N분전콤보값
    /// N분전 : 0(1분전), 1(2분전), 2(3분전), 3(5분전), 4(10분전), 5(15분전), 6(20분전), 7(30분전), 8(60분전), 9(120분전)
    #[serde(rename = "MINX")]
    pub minx: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래량급증] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumeSurgeRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N분전콤보값
    /// N분전 : 0(1분전), 1(2분전), 2(3분전), 3(5분전), 4(10분전), 5(15분전), 6(20분전), 7(30분전), 8(60분전), 9(120분전)
    #[serde(rename = "MINX")]
    pub minx: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 매수체결강도상위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct VolumePowerNextRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값
    /// N분전 : 0(1분전), 1(2분전), 2(3분전), 3(5분전), 4(10분전), 5(15분전), 6(20분전), 7(30분전), 8(60분전), 9(120분전)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 상승율/하락율] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct UpdownRateNextRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 상승율/하락율 구분
    /// 0(하락율), 1(상승율)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// N일자값
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 신고/신저가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewHighlowRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 신고/신저 구분
    /// 신고(1) 신저(0)
    #[serde(rename = "GUBN")]
    pub gubn: String,
    /// 일시돌파/돌파 구분
    /// 일시돌파(0) 돌파유지(1)
    #[serde(rename = "GUBN2")]
    pub gubn2: String,
    /// N일자값
    /// N일전 : 0(5일), 1(10일), 2(20일), 3(30일), 4(60일), 5(120일전), 6(52주), 7(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래량순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradeVolRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 현재가 필터범위 1
    /// 가격 ~
    #[serde(rename = "PRC1")]
    pub prc1: String,
    /// 현재가 필터범위 2
    /// ~ 가격
    #[serde(rename = "PRC2")]
    pub prc2: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래대금순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradePbmnRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
    /// 현재가 필터범위 1
    /// 가격 ~
    #[serde(rename = "PRC1")]
    pub prc1: String,
    /// 현재가 필터범위 2
    /// ~ 가격
    #[serde(rename = "PRC2")]
    pub prc2: String,
}

/// [해외주식 거래증가율순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradeGrowthRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 거래회전율순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TradeTurnoverRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// N일자값
    /// N일전 : 0(당일), 1(2일), 2(3일), 3(5일), 4(10일), 5(20일전), 6(30일), 7(60일), 8(120일), 9(1년)
    #[serde(rename = "NDAY")]
    pub nday: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 시가총액순위] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketCapNextRequest {
    /// NEXT KEY BUFF
    /// 공백
    #[serde(rename = "KEYB")]
    pub keyb: String,
    /// 사용자권한정보
    /// 공백
    #[serde(rename = "AUTH")]
    pub auth: String,
    /// 거래소코드
    /// 'NYS : 뉴욕, NAS : 나스닥, AMS : 아멕스
    /// HKS : 홍콩, SHS : 상해 , SZS : 심천
    /// HSX : 호치민, HNX : 하노이
    /// TSE : 도쿄 '
    #[serde(rename = "EXCD")]
    pub excd: String,
    /// 거래량조건
    /// 0(전체), 1(1백주이상), 2(1천주이상), 3(1만주이상), 4(10만주이상), 5(100만주이상), 6(1000만주이상)
    #[serde(rename = "VOL_RANG")]
    pub vol_rang: String,
}

/// [해외주식 기간별권리조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct PeriodRightsNextRequest {
    /// 권리유형코드
    /// '%%(전체), 01(유상), 02(무상), 03(배당), 11(합병),
    /// 14(액면분할), 15(액면병합), 17(감자), 54(WR청구),
    /// 61(원리금상환), 71(WR소멸), 74(배당옵션), 75(특별배당), 76(ISINCODE변경), 77(실권주청약)'
    #[serde(rename = "RGHT_TYPE_CD")]
    pub rght_type_cd: String,
    /// 조회구분코드
    /// 02(현지기준일), 03(청약시작일), 04(청약종료일)
    #[serde(rename = "INQR_DVSN_CD")]
    pub inqr_dvsn_cd: String,
    /// 조회시작일자
    /// 일자 ~
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// ~ 일자
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 상품번호
    /// 공백
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// 공백
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 연속조회키50
    /// 공백
    #[serde(rename = "CTX_AREA_NK50")]
    pub ctx_area_nk50: String,
    /// 연속조회검색조건50
    /// 공백
    #[serde(rename = "CTX_AREA_FK50")]
    pub ctx_area_fk50: String,
}

/// [해외뉴스종합(제목)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct NewsTitleNextRequest {
    /// 뉴스구분
    /// 전체: 공백
    #[serde(rename = "INFO_GB")]
    pub info_gb: String,
    /// 중분류
    /// 전체: 공백
    #[serde(rename = "CLASS_CD")]
    pub class_cd: String,
    /// 국가코드
    /// 전체: 공백
    /// CN(중국), HK(홍콩), US(미국)
    #[serde(rename = "NATION_CD")]
    pub nation_cd: String,
    /// 거래소코드
    /// 전체: 공백
    #[serde(rename = "EXCHANGE_CD")]
    pub exchange_cd: String,
    /// 종목코드
    /// 전체: 공백
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 조회일자
    /// 전체: 공백
    /// 특정일자(YYYYMMDD) ex. 20240502
    #[serde(rename = "DATA_DT")]
    pub data_dt: String,
    /// 조회시간
    /// 전체: 공백
    /// 전체: 공백
    /// 특정시간(HHMMSS) ex. 093500
    #[serde(rename = "DATA_TM")]
    pub data_tm: String,
    /// 다음키
    /// 공백 입력
    #[serde(rename = "CTS")]
    pub cts: String,
}

/// [해외주식 권리종합] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct RightsByIceRequest {
    /// 국가코드
    /// CN:중국 HK:홍콩 US:미국 JP:일본 VN:베트남
    #[serde(rename = "NCOD")]
    pub ncod: String,
    /// 심볼
    /// 종목코드
    #[serde(rename = "SYMB")]
    pub symb: String,
    /// 일자 시작일
    /// 미입력 시, 오늘-3개월
    /// 기간지정 시, 종료일 입력(ex. 20240514)
    ///
    /// ※ 조회기간 기준일 입력시 참고
    /// - 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일
    #[serde(rename = "ST_YMD")]
    pub st_ymd: String,
    /// 일자 종료일
    /// 미입력 시, 오늘+3개월
    /// 기간지정 시, 종료일 입력(ex. 20240514)
    ///
    /// ※ 조회기간 기준일 입력시 참고
    /// - 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일
    #[serde(rename = "ED_YMD")]
    pub ed_ymd: String,
}

/// [당사 해외주식담보대출 가능 종목] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct ColableByCompanyRequest {
    /// 상품번호
    /// ex)AMD
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// 공백
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 조회시작일자
    /// 공백
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// 공백
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 조회구분
    /// 공백
    #[serde(rename = "INQR_DVSN")]
    pub inqr_dvsn: String,
    /// 국가코드
    /// 840(미국), 344(홍콩), 156(중국)
    #[serde(rename = "NATN_CD")]
    pub natn_cd: String,
    /// 조회순서구분
    /// 01(이름순), 02(코드순)
    #[serde(rename = "INQR_SQN_DVSN")]
    pub inqr_sqn_dvsn: String,
    /// 비율구분코드
    /// 공백
    #[serde(rename = "RT_DVSN_CD")]
    pub rt_dvsn_cd: String,
    /// 비율
    /// 공백
    #[serde(rename = "RT")]
    pub rt: String,
    /// 대출가능여부
    /// 공백
    #[serde(rename = "LOAN_PSBL_YN")]
    pub loan_psbl_yn: String,
    /// 연속조회검색조건100
    /// 공백
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// 공백
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외속보(제목)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BrknewsTitleRequest {
    /// 뉴스제공업체코드
    /// 뉴스제공업체구분=>0:전체조회
    #[serde(rename = "FID_NEWS_OFER_ENTP_CODE")]
    pub fid_news_ofer_entp_code: String,
    /// 조건시장구분코드
    /// 공백
    #[serde(rename = "FID_COND_MRKT_CLS_CODE")]
    pub fid_cond_mrkt_cls_code: String,
    /// 입력종목코드
    /// 공백
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
    /// 제목내용
    /// 공백
    #[serde(rename = "FID_TITL_CNTT")]
    pub fid_titl_cntt: String,
    /// 입력날짜1
    /// 공백
    #[serde(rename = "FID_INPUT_DATE_1")]
    pub fid_input_date_1: String,
    /// 입력시간1
    /// 공백
    #[serde(rename = "FID_INPUT_HOUR_1")]
    pub fid_input_hour_1: String,
    /// 순위정렬구분코드
    /// 공백
    #[serde(rename = "FID_RANK_SORT_CLS_CODE")]
    pub fid_rank_sort_cls_code: String,
    /// 입력일련번호
    /// 공백
    #[serde(rename = "FID_INPUT_SRNO")]
    pub fid_input_srno: String,
    /// 조건화면분류코드
    /// 화면번호:11801
    #[serde(rename = "FID_COND_SCR_DIV_CODE")]
    pub fid_cond_scr_div_code: String,
}

/// [해외주식 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfsasp0Request {
    /// 거래ID
    /// HDFSASP0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// R거래소명종목코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfsasp1Request {
    /// 거래ID
    /// HDFSASP1
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// D거래소명종목코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfscnt0Request {
    /// 거래ID
    /// HDFSCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// D거래소명종목코드
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Gscni0Request {
    /// 거래ID
    /// [실전투자]
    /// H0GSCNI0 : 실시간 해외주식 체결통보
    ///
    /// [모의투자]
    /// H0GSCNI9 : 실시간 해외주식 체결통보
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID
    /// HTSID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외선물FX상품번호
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// 매도매수구분코드
    /// 01 : 매도
    /// 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM청산미결제체결일자
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_USTL_CCLD_DT")]
    pub fm_lqd_ustl_ccld_dt: String,
    /// FM청산미결제체결번호
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_USTL_CCNO")]
    pub fm_lqd_ustl_ccno: String,
    /// 가격구분코드
    /// 1.지정, 2. 시장, 3. STOP, 4 S/L
    #[serde(rename = "PRIC_DVSN_CD")]
    pub pric_dvsn_cd: String,
    /// FMLIMIT주문가격
    /// 지정가인 경우 가격 입력
    /// * 시장가, STOP주문인 경우, 빈칸("") 입력
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    /// FMSTOP주문가격
    /// STOP 주문 가격 입력
    /// * 시장가, 지정가인 경우, 빈칸("") 입력
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    /// FM주문수량
    #[serde(rename = "FM_ORD_QTY")]
    pub fm_ord_qty: String,
    /// FM청산LIMIT주문가격
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: String,
    /// FM청산STOP주문가격
    /// 빈칸 (hedge청산만 이용)
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: String,
    /// 체결조건코드
    /// 일반적으로 6 (EOD, 지정가)
    /// GTD인 경우 5, 시장가인 경우만 2
    #[serde(rename = "CCLD_CNDT_CD")]
    pub ccld_cndt_cd: String,
    /// 복합주문구분코드
    /// 0 (hedge청산만 이용)
    #[serde(rename = "CPLX_ORD_DVSN_CD")]
    pub cplx_ord_dvsn_cd: String,
    /// 행사예약주문여부
    /// N
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
    /// FM_HEDGE주문화면여부
    /// N
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
}

/// [해외선물옵션 정정취소주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclNextNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 원주문일자
    /// 원 주문 시 출력되는 ORD_DT 값을 입력 (현지거래일)
    #[serde(rename = "ORGN_ORD_DT")]
    pub orgn_ord_dt: String,
    /// 원주문번호
    /// 정정/취소시 주문번호(ODNO) 8자리를 문자열처럼 "0"을 포함해서 전송 (원 주문 시 출력된 ODNO 값 활용)
    /// (ex. ORGN_ODNO : 00360686)
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// FMLIMIT주문가격
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    /// FMSTOP주문가격
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    /// FM청산LIMIT주문가격
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: String,
    /// FM청산STOP주문가격
    /// OTFM3002U(해외선물옵션주문정정)만 사용
    #[serde(rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: String,
    /// FM_HEDGE주문화면여부
    /// N
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    /// FM시장가전환여부
    /// OTFM3003U(해외선물옵션주문취소)만 사용
    ///
    /// ※ FM_MKPR_CVSN_YN 항목에 'Y'로 설정하여 취소주문을 접수할 경우, 주문 취소확인이 들어오면 원장에서 시장가주문을 하나 또 내줌
    #[serde(rename = "FM_MKPR_CVSN_YN")]
    pub fm_mkpr_cvsn_yn: String,
}

/// [해외선물옵션 당일주문내역조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcldRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 체결미체결구분
    /// 01:전체 / 02:체결 / 03:미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 매도매수구분코드
    /// %%:전체 / 01:매도 / 02:매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 선물옵션구분
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 미결제내역조회(잔고)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireUnpdRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 선물옵션구분
    /// 00: 전체 / 01:선물 / 02: 옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건100
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
}

/// [해외선물옵션 주문가능조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsamountNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 해외선물FX상품번호
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    /// 매도매수구분코드
    /// 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// FM주문가격
    #[serde(rename = "FM_ORD_PRIC")]
    pub fm_ord_pric: String,
    /// 행사예약주문여부
    /// N
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
}

/// [해외선물옵션 기간계좌손익 일별] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodCcldRequest {
    /// 조회기간FROM일자
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// 조회기간TO일자
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드
    /// '%%% : 전체
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본'
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 전체환산여부
    /// N
    #[serde(rename = "WHOL_TRSL_YN")]
    pub whol_trsl_yn: String,
    /// 선물옵션구분
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 일별 체결내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작일자
    /// 시작일자(YYYYMMDD)
    #[serde(rename = "STRT_DT")]
    pub strt_dt: String,
    /// 종료일자
    /// 종료일자(YYYYMMDD)
    #[serde(rename = "END_DT")]
    pub end_dt: String,
    /// 선물옵션구분코드
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN_CD")]
    pub fuop_dvsn_cd: String,
    /// FM상품군코드
    /// 공란(Default)
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// 통화코드
    /// %%% : 전체
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본
    /// VND: 베트남
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// FM종목합산여부
    /// "N"(Default)
    #[serde(rename = "FM_ITEM_FTNG_YN")]
    pub fm_item_ftng_yn: String,
    /// 매도매수구분코드
    /// %%: 전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 예수금현황] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDepositNextNextRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본
    /// VND: 베트남
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 조회일자
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
}

/// [해외선물옵션 일별 주문내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyOrderRequest {
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 시작일자
    #[serde(rename = "STRT_DT")]
    pub strt_dt: String,
    /// 종료일자
    #[serde(rename = "END_DT")]
    pub end_dt: String,
    /// FM상품군코드
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// 체결미체결구분
    /// 01:전체 / 02:체결 / 03:미체결
    #[serde(rename = "CCLD_NCCS_DVSN")]
    pub ccld_nccs_dvsn: String,
    /// 매도매수구분코드
    /// %%전체 / 01 : 매도 / 02 : 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 선물옵션구분
    /// 00:전체 / 01:선물 / 02:옵션
    #[serde(rename = "FUOP_DVSN")]
    pub fuop_dvsn: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [해외선물옵션 기간계좌거래내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePeriodTransNextRequest {
    /// 조회기간FROM일자
    #[serde(rename = "INQR_TERM_FROM_DT")]
    pub inqr_term_from_dt: String,
    /// 조회기간TO일자
    #[serde(rename = "INQR_TERM_TO_DT")]
    pub inqr_term_to_dt: String,
    /// 종합계좌번호
    /// 계좌번호 체계(8-2)의 앞 8자리
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌번호 체계(8-2)의 뒤 2자리
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 계좌거래유형코드
    /// 1: 전체, 2:입출금 , 3: 결제
    #[serde(rename = "ACNT_TR_TYPE_CD")]
    pub acnt_tr_type_cd: String,
    /// 통화코드
    /// '%%% : 전체
    /// TUS: TOT_USD / TKR: TOT_KRW
    /// KRW: 한국 / USD: 미국
    /// EUR: EUR / HKD: 홍콩
    /// CNY: 중국 / JPY: 일본
    /// VND: 베트남 '
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 연속조회검색조건100
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_FK100값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
    /// 연속조회키100
    /// 공란 : 최초 조회시
    /// 이전 조회 Output CTX_AREA_NK100값 : 다음페이지 조회시(2번째부터)
    #[serde(rename = "CTX_AREA_NK100")]
    pub ctx_area_nk100: String,
    /// 비밀번호체크여부
    /// 공란(Default)
    #[serde(rename = "PWD_CHK_YN")]
    pub pwd_chk_yn: String,
}

/// [해외선물옵션 증거금상세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarginDetailRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 통화코드
    /// 'TKR(TOT_KRW), TUS(TOT_USD),
    /// USD(미국달러), HKD(홍콩달러),
    /// CNY(중국위안화), JPY )일본엔화), VND(베트남동)'
    #[serde(rename = "CRCY_CD")]
    pub crcy_cd: String,
    /// 조회일자
    #[serde(rename = "INQR_DT")]
    pub inqr_dt: String,
}

/// [해외선물종목현재가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceNextNextNextRequest {
    /// 종목코드
    /// ex) CNHU24
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수선물" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물종목상세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct StockDetailRequest {
    /// 종목코드
    /// ex) CNHU24
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수선물" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물 호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceNextNextRequest {
    /// 종목명
    /// 종목코드
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외선물 분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeFuturechartpriceRequest {
    /// 종목코드
    /// ex) CNHU24
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수선물" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// ex) 20230823
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 120 (조회갯수)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 5 (분간격)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 다음조회(QRY_TP를 P로 입력) 시, 이전 호출의 "output1 > index_key" 기입하여 조회
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(틱)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct TickCcnlRequest {
    /// 종목코드
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(주간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct WeeklyCcnlRequest {
    /// 종목코드
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(일간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct DailyCcnlRequest {
    /// 종목코드
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 체결추이(월간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MonthlyCcnlRequest {
    /// 종목코드
    /// 예) 6AM24
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 예) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// 공백
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// 예) 20240402
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 공백 (분만 사용)
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 공백
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외선물 상품기본정보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchContractDetailRequest {
    /// 요청개수
    /// 입력한 코드 개수
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 품목종류
    /// 최대 32개 까지 가능
    #[serde(rename = "SRS_CD_01")]
    pub srs_cd_01: String,
    /// 품목종류…
    #[serde(rename = "SRS_CD_02…")]
    pub srs_cd_02: String,
    /// 품목종류
    #[serde(rename = "SRS_CD_32")]
    pub srs_cd_32: String,
}

/// [해외선물 미결제추이] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InvestorUnpdTrendRequest {
    /// 상품
    /// 금리 (GE, ZB, ZF,ZN,ZT), 금속(GC, PA, PL,SI, HG), 농산물(CC, CT,KC, OJ, SB, ZC,ZL, ZM, ZO, ZR, ZS, ZW), 에너지(CL, HO, NG, WBS), 지수(ES, NQ, TF, YM, VX), 축산물(GF, HE, LE), 통화(6A, 6B, 6C, 6E, 6J, 6N, 6S, DX)
    #[serde(rename = "PROD_ISCD")]
    pub prod_iscd: String,
    /// 일자
    /// 기준일(ex)20240513)
    #[serde(rename = "BSOP_DATE")]
    pub bsop_date: String,
    /// 구분
    /// 0(수량), 1(증감)
    #[serde(rename = "UPMU_GUBUN")]
    pub upmu_gubun: String,
    /// CTS_KEY
    /// 공백
    #[serde(rename = "CTS_KEY")]
    pub cts_key: String,
}

/// [해외옵션종목현재가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptPriceRequest {
    /// 종목명
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션종목상세] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptDetailRequest {
    /// 종목명
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션 호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptAskingPriceRequest {
    /// 종목명
    /// 예)OESM24 C5340
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
}

/// [해외옵션 분봉조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireTimeOptchartpriceRequest {
    /// 종목코드
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// "" 공란 입력
    /// ※ 날짜 입력해도 처리 안됨
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 120 (최대 120)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 1: 1분봉, 5: 5분봉 ...
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 다음조회(QRY_TP를 P로 입력) 시, 이전 호출의 "output1 > index_key" 기입하여 조회
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(틱)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptTickCcnlRequest {
    /// 종목코드
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// "" 공란 입력
    /// ※ 날짜 입력해도 처리 안됨
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q : 최초조회시 , P : 다음키(INDEX_KEY) 입력하여 조회시
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 30 (최대 40)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// 공백
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// 다음조회(QRY_TP를 P로 입력) 시, 이전 호출의 "output1 > index_key" 기입하여 조회
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(일간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptDailyCcnlRequest {
    /// 종목코드
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// "" 공란 입력
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 100 (최대 119)
    /// ※ QRY_CNT 입력값의 +1 개 데이터가 조회됩니다.
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// "" 공란 입력
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// "" 공란 입력
    /// ※ 다음조회 불가
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(주간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptWeeklyCcnlRequest {
    /// 종목코드
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// "" 공란 입력
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 20 (최대 120)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// "" 공란 입력
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// "" 공란 입력
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 체결추이(월간)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OptMonthlyCcnlRequest {
    /// 종목코드
    /// ex) OESU24 C5500
    /// ※ 종목코드 "포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션/해외주식옵션" 참고
    #[serde(rename = "SRS_CD")]
    pub srs_cd: String,
    /// 거래소코드
    /// 종목코드에 맞는 거래소 코드 ex) CME
    #[serde(rename = "EXCH_CD")]
    pub exch_cd: String,
    /// 조회시작일시
    /// "" 공란 입력
    #[serde(rename = "START_DATE_TIME")]
    pub start_date_time: String,
    /// 조회종료일시
    /// "" 공란 입력
    #[serde(rename = "CLOSE_DATE_TIME")]
    pub close_date_time: String,
    /// 조회구분
    /// Q
    #[serde(rename = "QRY_TP")]
    pub qry_tp: String,
    /// 요청개수
    /// 예) 20 (최대 120)
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 묶음개수
    /// "" 공란 입력
    #[serde(rename = "QRY_GAP")]
    pub qry_gap: String,
    /// 이전조회KEY
    /// "" 공란 입력
    #[serde(rename = "INDEX_KEY")]
    pub index_key: String,
}

/// [해외옵션 상품기본정보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchOptDetailRequest {
    /// 요청개수
    /// 입력한 코드 개수
    #[serde(rename = "QRY_CNT")]
    pub qry_cnt: String,
    /// 종목코드1
    /// SRS_CD_01부터 차례로 입력(ex ) OESU24 C5500
    /// 최대 30개 까지 가능
    #[serde(rename = "SRS_CD_01")]
    pub srs_cd_01: String,
    /// 종목코드2
    #[serde(rename = "SRS_CD_02...")]
    pub srs_cd_02: String,
    /// 종목코드30
    #[serde(rename = "SRS_CD_30")]
    pub srs_cd_30: String,
}

/// [해외선물옵션 장운영시간] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct MarketTimeNextRequest {
    /// FM상품군코드
    /// 공백
    #[serde(rename = "FM_PDGR_CD")]
    pub fm_pdgr_cd: String,
    /// FM클래스코드
    /// '공백(전체), 001(통화), 002(금리), 003(지수),
    /// 004(농산물),005(축산물),006(금속),007(에너지)'
    #[serde(rename = "FM_CLAS_CD")]
    pub fm_clas_cd: String,
    /// FM거래소코드
    /// 'CME(CME), EUREX(EUREX), HKEx(HKEx),
    /// ICE(ICE), SGX(SGX), OSE(OSE), ASX(ASX),
    /// CBOE(CBOE), MDEX(MDEX), NYSE(NYSE),
    /// BMF(BMF),FTX(FTX), HNX(HNX), ETC(기타)'
    #[serde(rename = "FM_EXCG_CD")]
    pub fm_excg_cd: String,
    /// 옵션여부
    /// %(전체), N(선물), Y(옵션)
    #[serde(rename = "OPT_YN")]
    pub opt_yn: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [해외선물옵션 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff020Request {
    /// 거래ID
    /// HDFFF020
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    ///
    /// ※ CME, SGX 실시간시세 유료시세 신청 필수
    /// "포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)"
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff010Request {
    /// 거래ID
    /// HDFFF010
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 종목코드
    /// 종목코드
    ///
    /// ※ CME, SGX 실시간시세 유료시세 신청 필수
    /// "포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)"
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간주문내역통보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff1C0Request {
    /// 거래ID
    /// HDFFF1C0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID
    /// HTSID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [해외선물옵션 실시간체결내역통보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Hdfff2C0Request {
    /// 거래ID
    /// HDFFF2C0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// HTSID
    /// HTSID
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [장내채권 매수주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct BuyRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량2
    /// SAMT_MKET_PTCI_YN(소액시장참여여부) : N(일반시장) 입력 시 10단위 입력
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 채권주문단가
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 소액시장참여여부
    /// N: 일반시장, Y: 소액시장
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// 채권소매시장여부
    /// Y, N
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// 유치자직원번호
    /// 공백
    #[serde(rename = "IDCR_STFNO")]
    pub idcr_stfno: String,
    /// 운용사지정주문번호
    /// 공백
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드
    /// Unique key(0)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [장내채권 매도주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SellRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문구분
    /// '01: 종목별 (매수일자, 매수순번 공백입력)
    /// 02: 일자별 (매수순번: 0 입력)
    /// 03: 체결가별 '
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 주문수량2
    /// SAMT_MKET_PTCI_YN(소액시장참여여부) : N(일반시장) 입력 시 10단위 입력
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 주문단가
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 분리과세여부
    /// N: 종합과세, Y:분리과세
    #[serde(rename = "SPRX_YN")]
    pub sprx_yn: String,
    /// 매수일자
    /// (잔고조회 참조)
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// 매수순번
    /// (잔고조회 참조)
    #[serde(rename = "BUY_SEQ")]
    pub buy_seq: String,
    /// 소액시장참여여부
    /// N: 일반시장, Y: 소액시장
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    /// 매도대행사반대매도여부
    /// N
    #[serde(rename = "SLL_AGCO_OPPS_SLL_YN")]
    pub sll_agco_opps_sll_yn: String,
    /// 채권소매시장여부
    /// N
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    /// 운용사지정주문번호
    /// 공백
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드
    /// Unique key(0)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [장내채권 정정취소주문] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct OrderRvsecnclNextNextNextNextRequest {
    /// 종합계좌번호
    /// -
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// -
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    /// -
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 원주문번호
    /// -
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    /// 주문수량2
    /// 원주문이 일반시장 주문일 시 10단위 입력
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    /// 채권주문단가
    /// -
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 잔량전부주문여부
    /// Y: 잔량전부(주문수량 입력안함),
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 정정취소구분코드
    /// 01: 정정, 02: 취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    /// 운용사지정주문번호
    /// 공백
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    /// 주문서버구분코드
    /// Unique key(0)
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    /// 연락전화번호
    /// -
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

/// [채권정정취소가능주문조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblRvsecnclNextRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 주문일자
    #[serde(rename = "ORD_DT")]
    pub ord_dt: String,
    /// 주문번호
    #[serde(rename = "ODNO")]
    pub odno: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [장내채권 주문체결내역] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyCcldNextNextNextRequest {
    /// 종합계좌번호
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회시작일자
    /// 일자 ~ (1주일 이내)
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// ~ 일자 (조회 당일)
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 매도매수구분코드
    /// %(전체), 01(매도), 02(매수)
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 정렬순서구분
    /// 01(주문순서), 02(주문역순)
    #[serde(rename = "SORT_SQN_DVSN")]
    pub sort_sqn_dvsn: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 미체결여부
    /// N(전체), C(체결), Y(미체결)
    #[serde(rename = "NCCS_YN")]
    pub nccs_yn: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
}

/// [장내채권 잔고조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireBalanceNextNextNextNextRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 조회조건
    /// 00: 전체, 01: 상품번호단위
    #[serde(rename = "INQR_CNDT")]
    pub inqr_cndt: String,
    /// 상품번호
    /// 공백
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매수일자
    /// 공백
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    /// 연속조회검색조건200
    #[serde(rename = "CTX_AREA_FK200")]
    pub ctx_area_fk200: String,
    /// 연속조회키200
    #[serde(rename = "CTX_AREA_NK200")]
    pub ctx_area_nk200: String,
}

/// [장내채권 매수가능조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePsblOrderNextNextNextRequest {
    /// 종합계좌번호
    #[serde(rename = "CANO")]
    pub cano: String,
    /// 계좌상품코드
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 채권주문단가
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    /// 소액시장참여여부
    /// Y(소액시장) N (일반시장)
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
}

/// [장내채권현재가(호가)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireAskingPriceNextNextNextRequest {
    /// 조건 시장 분류 코드
    /// B: 장내
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 채권종목코드
    /// ex. KR2088012A16
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(시세)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquirePriceNextNextNextNextRequest {
    /// 조건시장분류코드
    /// B (업종코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 채권종목코드(ex KR2033022D33)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(체결)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireCcnlNextNextNextNextRequest {
    /// 조건시장분류코드
    /// B (업종코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 채권종목코드(ex KR2033022D33)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권현재가(일별)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyPriceNextRequest {
    /// 조건시장분류코드
    /// B (업종코드)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력종목코드
    /// 채권종목코드(ex KR2033022D33)
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권 기간별시세(일)] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct InquireDailyItemchartpriceNextRequest {
    /// 조건 시장 구분 코드
    /// Unique key(B)
    #[serde(rename = "FID_COND_MRKT_DIV_CODE")]
    pub fid_cond_mrkt_div_code: String,
    /// 입력 종목코드
    /// 종목코드
    #[serde(rename = "FID_INPUT_ISCD")]
    pub fid_input_iscd: String,
}

/// [장내채권 평균단가조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AvgUnitRequest {
    /// 조회시작일자
    /// 일자 ~
    #[serde(rename = "INQR_STRT_DT")]
    pub inqr_strt_dt: String,
    /// 조회종료일자
    /// ~ 일자
    #[serde(rename = "INQR_END_DT")]
    pub inqr_end_dt: String,
    /// 상품번호
    /// 공백: 전체, 특정종목 조회시 : 종목코드
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// Unique key(302)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    /// 검증종류코드
    /// Unique key(00)
    #[serde(rename = "VRFC_KIND_CD")]
    pub vrfc_kind_cd: String,
    /// 연속조회키30
    /// 공백
    #[serde(rename = "CTX_AREA_NK30")]
    pub ctx_area_nk30: String,
    /// 연속조회검색조건100
    /// 공백
    #[serde(rename = "CTX_AREA_FK100")]
    pub ctx_area_fk100: String,
}

/// [장내채권 발행정보] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct IssueInfoRequest {
    /// 사용자권한정보
    /// 채권 종목번호(ex. KR6449111CB8)
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 거래소코드
    /// Unique key(302)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [장내채권 기본조회] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct SearchBondInfoRequest {
    /// 상품번호
    /// 상품번호
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 상품유형코드
    /// Unique key(302)
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
}

/// [일반채권 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Bjcnt0Request {
    /// 거래ID
    /// H0BJCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 채권 종목코드 (ex. KR103502GA34)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [일반채권 실시간호가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Bjasp0Request {
    /// 거래ID
    /// H0BJCNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 채권 종목코드 (ex. KR103502GA34)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}

/// [채권지수 실시간체결가] 요청 구조체
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct H0Bicnt0Request {
    /// 거래ID
    /// H0BICNT0
    #[serde(rename = "tr_id")]
    pub tr_id: String,
    /// 구분값
    /// 채권 종목코드 (ex. KR103502GA34)
    #[serde(rename = "tr_key")]
    pub tr_key: String,
}
