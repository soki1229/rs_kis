#![allow(clippy::doc_lazy_continuation)]
use crate::client::KisClient;
use crate::error::KisError;
use crate::models::*;

#[allow(dead_code)]
pub struct Common(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Quotations(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Ranking(pub(crate) KisClient);

#[allow(dead_code)]
pub struct Trading(pub(crate) KisClient);

impl crate::endpoints::Overseas {
    pub fn common(&self) -> Common {
        Common(self.0.clone())
    }
    pub fn quotations(&self) -> Quotations {
        Quotations(self.0.clone())
    }
    pub fn ranking(&self) -> Ranking {
        Ranking(self.0.clone())
    }
    pub fn trading(&self) -> Trading {
        Trading(self.0.clone())
    }
}

#[allow(non_snake_case)]
impl Common {
    /// 접근토큰발급(P)
    ///
    /// - TR_ID: Real= / VTS=
    /// - Endpoint: /oauth2/tokenP
    ///
    /// OAuth인증
    /// 접근토큰발급(P)[인증-001]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 본인 계좌에 필요한 인증 절차로, 인증을 통해 접근 토큰을 부여받아 오픈API 활용이 가능합니다.
    ///
    /// 1. 접근토큰(access_token)의 유효기간은 24시간 이며(1일 1회발급 원칙)
    /// 갱신발급주기는 6시간 입니다.(6시간 이내는 기존 발급키로 응답)
    ///
    /// 2. 접근토큰발급(/oauth2/tokenP) 시 접근토큰값(access_token)과 함께 수신되는
    /// 접근토큰 유효기간(acess_token_token_expired)을 이용해 접근토큰을 관리하실 수 있습니다.
    ///
    ///
    /// [참고]
    ///
    /// '23.4.28 이후 지나치게 잦은 토큰 발급 요청건을 제어 하기 위해 신규 접근토큰발급 이후 일정시간 이내에 재호출 시에는 직전 토큰값을 리턴하게 되었습니다. 일정시간 이후 접근토큰발급 API 호출 시에는 신규 토큰값을 리턴합니다.
    /// 접근토큰발급 API 호출 및 코드 작성하실 때 해당 사항을 참고하시길 바랍니다.
    ///
    /// ※ 참고 : 포럼 > 공지사항 >  [수정] [중요] 접근 토큰 발급 변경 안내
    pub async fn tokenP(&self, req: TokenPRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/tokenP", tr_id, req).await
    }

    /// 접근토큰폐기(P)
    ///
    /// - TR_ID: Real= / VTS=
    /// - Endpoint: /oauth2/revokeP
    ///
    /// OAuth인증
    /// 접근토큰폐기(P)[인증-002]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 부여받은 접큰토큰을 더 이상 활용하지 않을 때 사용합니다.
    pub async fn revokeP(&self, req: RevokePRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/revokeP", tr_id, req).await
    }

    /// Hashkey
    ///
    /// - TR_ID: Real= / VTS=
    /// - Endpoint: /uapi/hashkey
    ///
    /// OAuth인증
    /// Hashkey
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해쉬키(Hashkey)는 보안을 위한 요소로 사용자가 보낸 요청 값을 중간에 탈취하여 변조하지 못하도록 하는데 사용됩니다.
    /// 해쉬키를 사용하면 POST로 보내는 요청(주로 주문/정정/취소 API 해당)의 body 값을 사전에 암호화시킬 수 있습니다.
    /// 해쉬키는 비필수값으로 사용하지 않아도 POST API 호출은 가능합니다.
    pub async fn hashkey(&self, req: HashkeyRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/uapi/hashkey", tr_id, req).await
    }

    /// 실시간 (웹소켓) 접속키 발급
    ///
    /// - TR_ID: Real= / VTS=
    /// - Endpoint: /oauth2/Approval
    ///
    /// OAuth인증
    /// 실시간 (웹소켓) 접속키 발급[실시간-000]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 실시간 (웹소켓) 접속키 발급받으실 수 있는 API 입니다.
    /// 웹소켓 이용 시 해당 키를 appkey와 appsecret 대신 헤더에 넣어 API를 호출합니다.
    ///
    /// 접속키의 유효기간은 24시간이지만, 접속키는 세션 연결 시 초기 1회만 사용하기 때문에 접속키 인증 후에는 세션종료되지 않는 이상 접속키 신규 발급받지 않으셔도 365일 내내 웹소켓 데이터 수신하실 수 있습니다.
    pub async fn Approval(&self, req: ApprovalRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/Approval", tr_id, req).await
    }

    /// 국내주식 실시간체결가 (KRX)
    ///
    /// - TR_ID: Real=H0STCNT0 / VTS=H0STCNT0
    /// - Endpoint: /tryitout/H0STCNT0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간체결가 (KRX) [실시간-003]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    ///
    /// ※ 데이터가 많은 경우 여러 건을 페이징 처리해서 데이터를 보내는 점 참고 부탁드립니다.
    /// ex) 0|H0STCNT0|004|... 인 경우 004가 데이터 개수를 의미하여, 뒤에 체결데이터가 4건 들어옴
    /// → 0|H0STCNT0|004|005930^123929...(체결데이터1)...^005930^123929...(체결데이터2)...^005930^123929...(체결데이터3)...^005930^123929...(체결데이터4)...
    pub async fn H0STCNT0(&self, req: H0Stcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STCNT0",
            crate::client::KisEnv::Vts => "H0STCNT0",
        };
        self.0.post("/tryitout/H0STCNT0", tr_id, req).await
    }

    /// 국내주식 실시간호가 (KRX)
    ///
    /// - TR_ID: Real=H0STASP0 / VTS=H0STASP0
    /// - Endpoint: /tryitout/H0STASP0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간호가 (KRX) [실시간-004]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id
    /// - 데이터 건수 : (ex. 001 데이터 건수를 참조하여 활용)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STASP0(&self, req: H0Stasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STASP0",
            crate::client::KisEnv::Vts => "H0STASP0",
        };
        self.0.post("/tryitout/H0STASP0", tr_id, req).await
    }

    /// 국내주식 실시간체결통보
    ///
    /// - TR_ID: Real=H0STCNI0 / VTS=H0STCNI9
    /// - Endpoint: /tryitout/H0STCNI0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간체결통보 [실시간-005]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 실시간 체결통보 수신 시에 (1) 주문·정정·취소·거부 접수 통보 와 (2) 체결 통보 가 모두 수신됩니다.
    /// (14번째 값(CNTG_YN;체결여부)가 2이면 체결통보, 1이면 주문·정정·취소·거부 접수 통보입니다.)
    ///
    /// ※ 모의투자는 H0STCNI9 로 변경하여 사용합니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id
    /// - 데이터 건수 : (ex. 001 데이터 건수를 참조하여 활용)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    ///
    /// 체결 통보 응답 결과는 암호화되어 출력됩니다. AES256 KEY IV를 활용해 복호화하여 활용하세요. 자세한 예제는 [도구>wikidocs]에 준비되어 있습니다.
    pub async fn H0STCNI0(&self, req: H0Stcni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STCNI0",
            crate::client::KisEnv::Vts => "H0STCNI9",
        };
        self.0.post("/tryitout/H0STCNI0", tr_id, req).await
    }

    /// 국내주식 실시간예상체결 (KRX)
    ///
    /// - TR_ID: Real=H0STANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STANC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간예상체결 (KRX) [실시간-041]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 실시간예상체결 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STANC0(&self, req: H0Stanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STANC0", tr_id, req).await
    }

    /// 국내주식 실시간회원사 (KRX)
    ///
    /// - TR_ID: Real=H0STMBC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STMBC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간회원사 (KRX) [실시간-047]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STMBC0(&self, req: H0Stmbc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STMBC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STMBC0", tr_id, req).await
    }

    /// 국내주식 실시간프로그램매매 (KRX)
    ///
    /// - TR_ID: Real=H0STPGM0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STPGM0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간프로그램매매 (KRX) [실시간-048]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STPGM0(&self, req: H0Stpgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STPGM0", tr_id, req).await
    }

    /// 국내주식 장운영정보 (KRX)
    ///
    /// - TR_ID: Real=H0STMKO0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STMKO0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 장운영정보 (KRX) [실시간-049]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 장운영정보 연결 시, 연결종목의 VI 발동 시와 VI 해제 시에 데이터 수신됩니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STMKO0(&self, req: H0Stmko0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STMKO0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STMKO0", tr_id, req).await
    }

    /// 국내주식 시간외 실시간호가 (KRX)
    ///
    /// - TR_ID: Real=H0STOAA0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STOAA0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 시간외 실시간호가 (KRX) [실시간-025]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외 실시간호가 API입니다.
    /// 국내주식 시간외 단일가(16:00~18:00) 시간대에 실시간호가 데이터 확인 가능합니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STOAA0(&self, req: H0Stoaa0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STOAA0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STOAA0", tr_id, req).await
    }

    /// 국내주식 시간외 실시간체결가 (KRX)
    ///
    /// - TR_ID: Real=H0STOUP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STOUP0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 시간외 실시간체결가 (KRX) [실시간-042]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외 실시간체결가 API입니다.
    /// 국내주식 시간외 단일가(16:00~18:00) 시간대에 실시간체결가 데이터 확인 가능합니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STOUP0(&self, req: H0Stoup0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STOUP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STOUP0", tr_id, req).await
    }

    /// 국내주식 시간외 실시간예상체결 (KRX)
    ///
    /// - TR_ID: Real=H0STOAC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STOAC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 시간외 실시간예상체결 (KRX) [실시간-024]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내주식 시간외 실시간예상체결 API입니다.
    /// 국내주식 시간외 단일가(16:00~18:00) 시간대에 실시간예상체결 데이터 확인 가능합니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0STOAC0(&self, req: H0Stoac0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STOAC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STOAC0", tr_id, req).await
    }

    /// 국내지수 실시간체결
    ///
    /// - TR_ID: Real=H0UPCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UPCNT0
    ///
    /// [국내주식] 실시간시세
    /// 국내지수 실시간체결 [실시간-026]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0UPCNT0(&self, req: H0Upcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UPCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UPCNT0", tr_id, req).await
    }

    /// 국내지수 실시간예상체결
    ///
    /// - TR_ID: Real=H0UPANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UPANC0
    ///
    /// [국내주식] 실시간시세
    /// 국내지수 실시간예상체결 [실시간-027]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0UPANC0(&self, req: H0Upanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UPANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UPANC0", tr_id, req).await
    }

    /// 국내지수 실시간프로그램매매
    ///
    /// - TR_ID: Real=H0UPPGM0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UPPGM0
    ///
    /// [국내주식] 실시간시세
    /// 국내지수 실시간프로그램매매 [실시간-028]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0UPPGM0(&self, req: H0Uppgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UPPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UPPGM0", tr_id, req).await
    }

    /// ELW 실시간호가
    ///
    /// - TR_ID: Real=H0EWASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EWASP0
    ///
    /// [국내주식] 실시간시세
    /// ELW 실시간호가 [실시간-062]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ELW 실시간호가 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0EWASP0(&self, req: H0Ewasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EWASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EWASP0", tr_id, req).await
    }

    /// ELW 실시간체결가
    ///
    /// - TR_ID: Real=H0EWCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EWCNT0
    ///
    /// [국내주식] 실시간시세
    /// ELW 실시간체결가 [실시간-061]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ELW 실시간체결가 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0EWCNT0(&self, req: H0Ewcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EWCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EWCNT0", tr_id, req).await
    }

    /// ELW 실시간예상체결
    ///
    /// - TR_ID: Real=H0EWANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EWANC0
    ///
    /// [국내주식] 실시간시세
    /// ELW 실시간예상체결 [실시간-063]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ELW 실시간예상체결 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0EWANC0(&self, req: H0Ewanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EWANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EWANC0", tr_id, req).await
    }

    /// 국내ETF NAV추이
    ///
    /// - TR_ID: Real=H0STNAV0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0STNAV0
    ///
    /// [국내주식] 실시간시세
    /// 국내ETF NAV추이 [실시간-051]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0STNAV0(&self, req: H0Stnav0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STNAV0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STNAV0", tr_id, req).await
    }

    /// 국내주식 실시간체결가 (통합)
    ///
    /// - TR_ID: Real=H0UNCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UNCNT0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간체결가 (통합)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0UNCNT0(&self, req: H0Uncnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNCNT0", tr_id, req).await
    }

    /// 국내주식 실시간호가 (통합)
    ///
    /// - TR_ID: Real=H0UNASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UNASP0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간호가 (통합)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0UNASP0(&self, req: H0Unasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNASP0", tr_id, req).await
    }

    /// 국내주식 실시간예상체결 (통합)
    ///
    /// - TR_ID: Real=H0UNANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UNANC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간예상체결 (통합)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0UNANC0(&self, req: H0Unanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNANC0", tr_id, req).await
    }

    /// 국내주식 실시간회원사 (통합)
    ///
    /// - TR_ID: Real=H0UNMBC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UNMBC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간회원사 (통합)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0UNMBC0(&self, req: H0Unmbc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNMBC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNMBC0", tr_id, req).await
    }

    /// 국내주식 실시간프로그램매매 (통합)
    ///
    /// - TR_ID: Real=H0UNPGM0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UNPGM0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간프로그램매매 (통합)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0UNPGM0(&self, req: H0Unpgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNPGM0", tr_id, req).await
    }

    /// 국내주식 장운영정보 (통합)
    ///
    /// - TR_ID: Real=H0UNMKO0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0UNMKO0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 장운영정보 (통합)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0UNMKO0(&self, req: H0Unmko0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNMKO0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNMKO0", tr_id, req).await
    }

    /// 국내주식 실시간체결가 (NXT)
    ///
    /// - TR_ID: Real=H0NXCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0NXCNT0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간체결가 (NXT)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0NXCNT0(&self, req: H0Nxcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXCNT0", tr_id, req).await
    }

    /// 국내주식 실시간호가 (NXT)
    ///
    /// - TR_ID: Real=H0NXASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0NXASP0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간호가 (NXT)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0NXASP0(&self, req: H0Nxasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXASP0", tr_id, req).await
    }

    /// 국내주식 실시간예상체결 (NXT)
    ///
    /// - TR_ID: Real=H0NXANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0NXANC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간예상체결 (NXT)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0NXANC0(&self, req: H0Nxanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXANC0", tr_id, req).await
    }

    /// 국내주식 실시간회원사 (NXT)
    ///
    /// - TR_ID: Real=H0NXMBC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0NXMBC0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간회원사 (NXT)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0NXMBC0(&self, req: H0Nxmbc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXMBC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXMBC0", tr_id, req).await
    }

    /// 국내주식 실시간프로그램매매 (NXT)
    ///
    /// - TR_ID: Real=H0NXPGM0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0NXPGM0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 실시간프로그램매매 (NXT)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0NXPGM0(&self, req: H0Nxpgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXPGM0", tr_id, req).await
    }

    /// 국내주식 장운영정보 (NXT)
    ///
    /// - TR_ID: Real=H0NXMKO0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0NXMKO0
    ///
    /// [국내주식] 실시간시세
    /// 국내주식 장운영정보 (NXT)
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0NXMKO0(&self, req: H0Nxmko0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXMKO0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXMKO0", tr_id, req).await
    }

    /// 지수선물 실시간호가
    ///
    /// - TR_ID: Real=H0IFASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0IFASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// 지수선물 실시간호가[실시간-011]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ 선물옵션 호가 데이터는 0.2초 필터링 옵션이 있습니다.
    /// 필터링 사유는 순간적으로 데이터가 폭증할 경우 서버 뿐만아니라 클라이언트 환경에도 부하를 줄 수 있어 적용된 사항인 점 양해 부탁드립니다.
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0IFASP0(&self, req: H0Ifasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IFASP0", tr_id, req).await
    }

    /// 지수선물 실시간체결가
    ///
    /// - TR_ID: Real=H0IFCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0IFCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// 지수선물 실시간체결가[실시간-010]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0IFCNT0(&self, req: H0Ifcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IFCNT0", tr_id, req).await
    }

    /// 지수옵션 실시간호가
    ///
    /// - TR_ID: Real=H0IOASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0IOASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// 지수옵션 실시간호가[실시간-015]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0IOASP0(&self, req: H0Ioasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IOASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IOASP0", tr_id, req).await
    }

    /// 지수옵션 실시간체결가
    ///
    /// - TR_ID: Real=H0IOCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0IOCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// 지수옵션 실시간체결가[실시간-014]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0IOCNT0(&self, req: H0Iocnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IOCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IOCNT0", tr_id, req).await
    }

    /// 선물옵션 실시간체결통보
    ///
    /// - TR_ID: Real=H0IFCNI0 / VTS=H0IFCNI9
    /// - Endpoint: /tryitout/H0IFCNI0
    ///
    /// [국내선물옵션] 실시간시세
    /// 선물옵션 실시간체결통보[실시간-012]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0IFCNI0(&self, req: H0Ifcni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IFCNI0",
            crate::client::KisEnv::Vts => "H0IFCNI9",
        };
        self.0.post("/tryitout/H0IFCNI0", tr_id, req).await
    }

    /// 상품선물 실시간호가
    ///
    /// - TR_ID: Real=H0CFASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0CFASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// 상품선물 실시간호가[실시간-023]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ 선물옵션 호가 데이터는 0.2초 필터링 옵션이 있습니다.
    /// 필터링 사유는 순간적으로 데이터가 폭증할 경우 서버 뿐만아니라 클라이언트 환경에도 부하를 줄 수 있어 적용된 사항인 점 양해 부탁드립니다.
    pub async fn H0CFASP0(&self, req: H0Cfasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0CFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0CFASP0", tr_id, req).await
    }

    /// 상품선물 실시간체결가
    ///
    /// - TR_ID: Real=H0CFCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0CFCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// 상품선물 실시간체결가[실시간-022]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0CFCNT0(&self, req: H0Cfcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0CFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0CFCNT0", tr_id, req).await
    }

    /// 주식선물 실시간호가
    ///
    /// - TR_ID: Real=H0ZFASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0ZFASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// 주식선물 실시간호가 [실시간-030]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ 선물옵션 호가 데이터는 0.2초 필터링 옵션이 있습니다.
    /// 필터링 사유는 순간적으로 데이터가 폭증할 경우 서버 뿐만아니라 클라이언트 환경에도 부하를 줄 수 있어 적용된 사항인 점 양해 부탁드립니다.
    pub async fn H0ZFASP0(&self, req: H0Zfasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZFASP0", tr_id, req).await
    }

    /// 주식선물 실시간체결가
    ///
    /// - TR_ID: Real=H0ZFCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0ZFCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// 주식선물 실시간체결가 [실시간-029]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0ZFCNT0(&self, req: H0Zfcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZFCNT0", tr_id, req).await
    }

    /// 주식선물 실시간예상체결
    ///
    /// - TR_ID: Real=H0ZFANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0ZFANC0
    ///
    /// [국내선물옵션] 실시간시세
    /// 주식선물 실시간예상체결 [실시간-031]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0ZFANC0(&self, req: H0Zfanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZFANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZFANC0", tr_id, req).await
    }

    /// 주식옵션 실시간호가
    ///
    /// - TR_ID: Real=H0ZOASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0ZOASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// 주식옵션 실시간호가 [실시간-045]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0ZOASP0(&self, req: H0Zoasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZOASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZOASP0", tr_id, req).await
    }

    /// 주식옵션 실시간체결가
    ///
    /// - TR_ID: Real=H0ZOCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0ZOCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// 주식옵션 실시간체결가 [실시간-044]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn H0ZOCNT0(&self, req: H0Zocnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZOCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZOCNT0", tr_id, req).await
    }

    /// 주식옵션 실시간예상체결
    ///
    /// - TR_ID: Real=H0ZOANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0ZOANC0
    ///
    /// [국내선물옵션] 실시간시세
    /// 주식옵션 실시간예상체결 [실시간-046]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0ZOANC0(&self, req: H0Zoanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZOANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZOANC0", tr_id, req).await
    }

    /// KRX야간옵션 실시간호가
    ///
    /// - TR_ID: Real=H0EUASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EUASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간옵션 실시간호가 [실시간-033]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0EUASP0(&self, req: H0Euasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EUASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUASP0", tr_id, req).await
    }

    /// KRX야간옵션 실시간체결가
    ///
    /// - TR_ID: Real=H0EUCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EUCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간옵션 실시간체결가 [실시간-032]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0EUCNT0(&self, req: H0Eucnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EUCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUCNT0", tr_id, req).await
    }

    /// KRX야간옵션실시간예상체결
    ///
    /// - TR_ID: Real=H0EUANC0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EUANC0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간옵션실시간예상체결 [실시간-034]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0EUANC0(&self, req: H0Euanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EUANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUANC0", tr_id, req).await
    }

    /// KRX야간옵션실시간체결통보
    ///
    /// - TR_ID: Real=H0MFCNI0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0EUCNI0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간옵션실시간체결통보 [실시간-067]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0EUCNI0(&self, req: H0Eucni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFCNI0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUCNI0", tr_id, req).await
    }

    /// KRX야간선물 실시간호가
    ///
    /// - TR_ID: Real=H0MFASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0MFASP0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간선물 실시간호가 [실시간-065]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ 선물옵션 호가 데이터는 0.2초 필터링 옵션이 있습니다.
    /// 필터링 사유는 순간적으로 데이터가 폭증할 경우 서버 뿐만아니라 클라이언트 환경에도 부하를 줄 수 있어 적용된 사항인 점 양해 부탁드립니다.
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0MFASP0(&self, req: H0Mfasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0MFASP0", tr_id, req).await
    }

    /// KRX야간선물 실시간종목체결
    ///
    /// - TR_ID: Real=H0MFCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0MFCNT0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간선물 실시간종목체결 [실시간-064]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0MFCNT0(&self, req: H0Mfcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0MFCNT0", tr_id, req).await
    }

    /// KRX야간선물 실시간체결통보
    ///
    /// - TR_ID: Real=H0MFCNI0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0MFCNI0
    ///
    /// [국내선물옵션] 실시간시세
    /// KRX야간선물 실시간체결통보 [실시간-066]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn H0MFCNI0(&self, req: H0Mfcni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFCNI0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0MFCNI0", tr_id, req).await
    }

    /// 해외주식 실시간호가
    ///
    /// - TR_ID: Real=HDFSASP0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFSASP0
    ///
    /// [해외주식] 실시간시세
    /// 해외주식 실시간호가[실시간-021]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 실시간호가 API를 이용하여 미국 실시간 10호가(매수/매도) 시세가 무료로 제공됩니다. (미국은 유료시세 제공 X)
    ///
    /// 아시아 국가의 경우, HTS(efriend Plus) [7781] 시세신청(실시간) 화면에서 유료 서비스 신청 시,
    /// "해외주식 실시간호가 HDFSASP0" 을 이용하여 아시아국가 유료시세(실시간호가)를 받아보실 수 있습니다. (24.11.29 반영)
    /// (아시아 국가 무료시세는 "해외주식 지연호가(아시아) HDFSASP1" 를 이용하시기 바랍니다.)
    ///
    /// ※ 미국 : 실시간 무료, 매수/매도 각 10호가 (0분지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// ※ 아시아(홍콩, 베트남, 중국, 일본) : 실시간 유료 (단, 중국은 HTS[7781]에서 실시간시세 무료로 신청 후 이용 가능)
    ///
    /// 해당 API로 미국주간거래(10:00~16:00) 시세 조회도 가능합니다.
    /// ※ 미국주간거래 실시간 조회 시, 맨 앞자리(R), tr_key 중 시장구분 값을 다음과 같이 입력 → 나스닥: BAQ, 뉴욕: BAY, 아멕스: BAA
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// ​[미국주식시세 이용시 유의사항]
    ///
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    ///
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    ///
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가
    /// 있을 수 있으며 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    ///
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 유료 실시간 시세 서비스와 다를 수 있으며,
    /// 종목별 과거 데이터(거래량, 시가, 종가, 고가, 차트 데이터 등)는 장 종료 후(오후 12시경) 유료 실시간 시세 서비스 데이터와 동일하게 업데이트됩니다.
    /// (출처: 한국투자증권 외화증권 거래설명서 - https://securities.koreainvestment.com/main/customer/guide/Guide.jsp?&cmd=TF04ag010002¤tPage=1&num=64)
    pub async fn HDFSASP0(&self, req: Hdfsasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFSASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFSASP0", tr_id, req).await
    }

    /// 해외주식 지연호가(아시아)
    ///
    /// - TR_ID: Real=HDFSASP1 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFSASP1
    ///
    /// [해외주식] 실시간시세
    /// 해외주식 지연호가(아시아)[실시간-008]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 지연호가(아시아)의 경우 아시아 무료시세(지연호가)가 제공됩니다.
    ///
    /// HTS(efriend Plus) [7781] 시세신청(실시간) 화면에서 유료 서비스 신청 시,
    /// "해외주식 실시간호가 HDFSASP0" 을 이용하여 아시아국가 유료시세(실시간호가)를 받아보실 수 있습니다. (24.11.29 반영)
    ///
    /// ※ 지연시세 지연시간 : 홍콩, 베트남, 중국, 일본 - 15분지연
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn HDFSASP1(&self, req: Hdfsasp1Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFSASP1",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFSASP1", tr_id, req).await
    }

    /// 해외주식 실시간지연체결가
    ///
    /// - TR_ID: Real=HDFSCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFSCNT0
    ///
    /// [해외주식] 실시간시세
    /// 해외주식 실시간지연체결가[실시간-007]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 실시간지연체결가의 경우 기본적으로 무료시세(지연체결가)가 제공되며,
    /// 아시아 국가의 경우 HTS(efriend Plus) [7781] 시세신청(실시간) 화면에서 유료 서비스 신청 시 API로도 유료시세(실시간체결가)를 받아보실 수 있습니다. (24.11.29 반영)
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분지연) / 홍콩, 베트남, 중국, 일본 - 15분지연 (중국은 실시간시세 신청 시 무료실시간시세 제공)
    /// 미국의 경우 0분지연시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// 해당 API로 미국주간거래(10:00~16:00) 시세 조회도 가능합니다.
    /// ※ 미국주간거래 실시간 조회 시, 맨 앞자리(R), tr_key 중 시장구분 값을 다음과 같이 입력 → 나스닥: BAQ, 뉴욕: BAY, 아멕스: BAA
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn HDFSCNT0(&self, req: Hdfscnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFSCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFSCNT0", tr_id, req).await
    }

    /// 해외주식 실시간체결통보
    ///
    /// - TR_ID: Real=H0GSCNI0 / VTS=H0GSCNI9
    /// - Endpoint: /tryitout/H0GSCNI0
    ///
    /// [해외주식] 실시간시세
    /// 해외주식 실시간체결통보[실시간-009]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    pub async fn H0GSCNI0(&self, req: H0Gscni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0GSCNI0",
            crate::client::KisEnv::Vts => "H0GSCNI9",
        };
        self.0.post("/tryitout/H0GSCNI0", tr_id, req).await
    }

    /// 해외선물옵션 실시간체결가
    ///
    /// - TR_ID: Real=HDFFF020 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFFF020
    ///
    /// [해외선물옵션]실시간시세
    /// 해외선물옵션 실시간체결가[실시간-017]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ CME, SGX 실시간시세 유료시세 신청 필수 (포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소))
    /// - CME, SGX 거래소 실시간시세는 유료시세 신청 후 이용하시는 모든 계좌에 대해서 접근토큰발급 API 호출하셔야 하며,
    /// 접근토큰발급 이후 2시간 이내로 신청정보가 동기화되어 유료시세 수신이 가능해집니다.
    /// - CME, SGX 거래소 종목은 유료시세 신청되어 있지 않으면 실시간시세 종목등록이 불가하며,
    /// 등록 시도 시 "SUBSCRIBE ERROR : mci send failed" 에러가 발생합니다.
    ///
    ///
    /// (중요) 해외선물옵션시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드 - 해외선물옵션 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn HDFFF020(&self, req: Hdfff020Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF020",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF020", tr_id, req).await
    }

    /// 해외선물옵션 실시간호가
    ///
    /// - TR_ID: Real=HDFFF010 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFFF010
    ///
    /// [해외선물옵션]실시간시세
    /// 해외선물옵션 실시간호가[실시간-018]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ CME, SGX 실시간시세 유료시세 신청 필수 (포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소))
    /// - CME, SGX 거래소 실시간시세는 유료시세 신청 후 이용하시는 모든 계좌에 대해서 접근토큰발급 API 호출하셔야 하며,
    /// 접근토큰발급 이후 2시간 이내로 신청정보가 동기화되어 유료시세 수신이 가능해집니다.
    /// - CME, SGX 거래소 종목은 유료시세 신청되어 있지 않으면 실시간시세 종목등록이 불가하며,
    /// 등록 시도 시 "SUBSCRIBE ERROR : mci send failed" 에러가 발생합니다.
    ///
    /// (중요) 해외선물옵션시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드 - 해외선물옵션 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    ///
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn HDFFF010(&self, req: Hdfff010Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF010",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF010", tr_id, req).await
    }

    /// 해외선물옵션 실시간주문내역통보
    ///
    /// - TR_ID: Real=HDFFF1C0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFFF1C0
    ///
    /// [해외선물옵션]실시간시세
    /// 해외선물옵션 실시간주문내역통보[실시간-019]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// [참고자료]
    ///
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn HDFFF1C0(&self, req: Hdfff1C0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF1C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF1C0", tr_id, req).await
    }

    /// 해외선물옵션 실시간체결내역통보
    ///
    /// - TR_ID: Real=HDFFF2C0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/HDFFF2C0
    ///
    /// [해외선물옵션]실시간시세
    /// 해외선물옵션 실시간체결내역통보[실시간-020]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    pub async fn HDFFF2C0(&self, req: Hdfff2C0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF2C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF2C0", tr_id, req).await
    }

    /// 일반채권 실시간체결가
    ///
    /// - TR_ID: Real=H0BJCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0BJCNT0
    ///
    /// [장내채권] 실시간시세
    /// 일반채권 실시간체결가 [실시간-052]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 일반채권 실시간체결가 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 채권 종목코드 마스터파일은 "포럼 >  FAQ > 종목정보 다운로드(국내) > 장내채권 - 채권코드" 참고 부탁드립니다.
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0BJCNT0(&self, req: H0Bjcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0BJCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0BJCNT0", tr_id, req).await
    }

    /// 일반채권 실시간호가
    ///
    /// - TR_ID: Real=H0BJCNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0BJASP0
    ///
    /// [장내채권] 실시간시세
    /// 일반채권 실시간호가 [실시간-053]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 일반채권 실시간호가 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 채권 종목코드 마스터파일은 "포럼 >  FAQ > 종목정보 다운로드(국내) > 장내채권 - 채권코드" 참고 부탁드립니다.
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0BJASP0(&self, req: H0Bjasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0BJCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0BJASP0", tr_id, req).await
    }

    /// 채권지수 실시간체결가
    ///
    /// - TR_ID: Real=H0BICNT0 / VTS=모의투자 미지원
    /// - Endpoint: /tryitout/H0BICNT0
    ///
    /// [장내채권] 실시간시세
    /// 채권지수 실시간체결가 [실시간-060]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 채권지수 실시간체결가 API입니다.
    ///
    /// [참고자료]
    /// 실시간시세(웹소켓) 파이썬 샘플코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/blob/main/websocket/python/ws_domestic_overseas_all.py
    ///
    /// 실시간시세(웹소켓) API 사용방법에 대한 자세한 설명은 한국투자증권 Wikidocs 참고 부탁드립니다.
    /// https://wikidocs.net/book/7847 (국내주식 업데이트 완료, 추후 해외주식·국내선물옵션 업데이트 예정)
    ///
    /// 채권 종목코드 마스터파일은 "포럼 > FAQ > 종목정보 다운로드(국내) > 장내채권 - 채권코드" 참고 부탁드립니다.
    ///
    /// [호출 데이터]
    /// 헤더와 바디 값을 합쳐 JSON 형태로 전송합니다.
    ///
    /// [응답 데이터]
    /// 1. 정상 등록 여부 (JSON)
    /// - JSON["body"]["msg1"] - 정상 응답 시, SUBSCRIBE SUCCESS
    /// - JSON["body"]["output"]["iv"] - 실시간 결과 복호화에 필요한 AES256 IV (Initialize Vector)
    /// - JSON["body"]["output"]["key"] - 실시간 결과 복호화에 필요한 AES256 Key
    ///
    /// 2. 실시간 결과 응답 ( | 로 구분되는 값)
    /// ex) 0|H0STCNT0|004|005930^123929^73100^5^...
    /// - 암호화 유무 : 0 암호화 되지 않은 데이터 / 1 암호화된 데이터
    /// - TR_ID : 등록한 tr_id (ex. H0STCNT0)
    /// - 데이터 건수 : (ex. 001 인 경우 데이터 건수 1건, 004인 경우 데이터 건수 4건)
    /// - 응답 데이터 : 아래 response 데이터 참조 ( ^로 구분됨)
    pub async fn H0BICNT0(&self, req: H0Bicnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0BICNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0BICNT0", tr_id, req).await
    }
}

#[allow(non_snake_case)]
impl Quotations {
    /// ETF/ETN 현재가
    ///
    /// - TR_ID: Real=FHPST02400000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/inquire-price
    ///
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
    ///
    /// # Example (Scraped)
    /// ETF/ETN 현재가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0240] ETF/ETN 현재가 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn inquire_price(
        &self,
        req: InquirePriceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02400000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/etfetn/v1/quotations/inquire-price", tr_id, req)
            .await
    }

    /// ETF 구성종목시세
    ///
    /// - TR_ID: Real=FHKST121600C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/inquire-component-stock-price
    ///
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
    ///
    /// # Example (Scraped)
    /// ETF 구성종목시세 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0245] ETF/ETN 구성종목시세 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn inquire_component_stock_price(
        &self,
        req: InquireComponentStockPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST121600C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/inquire-component-stock-price",
                tr_id,
                req,
            )
            .await
    }

    /// NAV 비교추이(종목)
    ///
    /// - TR_ID: Real=FHPST02440000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/nav-comparison-trend
    ///
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
    ///
    /// # Example (Scraped)
    /// NAV 비교추이(종목) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0244] ETF/ETN 비교추이(NAV/IIV) 좌측 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn nav_comparison_trend(
        &self,
        req: NavComparisonTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02440000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/nav-comparison-trend",
                tr_id,
                req,
            )
            .await
    }

    /// NAV 비교추이(일)
    ///
    /// - TR_ID: Real=FHPST02440200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/nav-comparison-daily-trend
    ///
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
    ///
    /// # Example (Scraped)
    /// NAV 비교추이(일) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0244] ETF/ETN 비교추이(NAV/IIV) 좌측 화면 "일별" 비교추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능합니다.
    pub async fn nav_comparison_daily_trend(
        &self,
        req: NavComparisonDailyTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02440200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/nav-comparison-daily-trend",
                tr_id,
                req,
            )
            .await
    }

    /// NAV 비교추이(분)
    ///
    /// - TR_ID: Real=FHPST02440100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/etfetn/v1/quotations/nav-comparison-time-trend
    ///
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
    ///
    /// # Example (Scraped)
    /// NAV 비교추이(분) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0244] ETF/ETN 비교추이(NAV/IIV) 좌측 화면 "분별" 비교추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 실전계좌의 경우, 한 번의 호출에 최근 30건까지 확인 가능합니다.
    pub async fn nav_comparison_time_trend(
        &self,
        req: NavComparisonTimeTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPST02440100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/etfetn/v1/quotations/nav-comparison-time-trend",
                tr_id,
                req,
            )
            .await
    }

    /// ELW 신규상장종목
    ///
    /// - TR_ID: Real=FHKEW154800C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/newly-listed
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 신규상장종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0297] ELW 신규상장종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn newly_listed(
        &self,
        req: NewlyListedRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW154800C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/newly-listed", tr_id, req)
            .await
    }

    /// ELW 기초자산별 종목시세
    ///
    /// - TR_ID: Real=FHKEW154101C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/udrl-asset-price
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 기초자산별 종목시세  API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0288] ELW 기초자산별 ELW 시세 화면의 "우측 기초자산별 종목 리스트" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn udrl_asset_price(
        &self,
        req: UdrlAssetPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW154101C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/udrl-asset-price", tr_id, req)
            .await
    }

    /// ELW 종목검색
    ///
    /// - TR_ID: Real=FHKEW15100000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/cond-search
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 종목검색 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0291] ELW 종목검색 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 한 번의 호출에 최대 100건까지 확인 가능합니다.
    pub async fn cond_search(&self, req: CondSearchRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW15100000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/cond-search", tr_id, req)
            .await
    }

    /// ELW 기초자산 목록조회
    ///
    /// - TR_ID: Real=FHKEW154100C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/udrl-asset-list
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 기초자산 목록조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0288] ELW 기초자산별 ELW 시세 화면 의 "왼쪽 기초자산 목록" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn udrl_asset_list(
        &self,
        req: UdrlAssetListRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW154100C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/udrl-asset-list", tr_id, req)
            .await
    }

    /// ELW 비교대상종목조회
    ///
    /// - TR_ID: Real=FHKEW151701C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/compare-stocks
    ///
    /// [국내주식] ELW 시세
    /// ELW 비교대상종목조회 [국내주식-183]
    /// elw_shrn_iscd
    /// elw_kor_isnm
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ELW 비교대상종목조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0288] ELW 기초자산별 ELW 시세의 좌측 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn compare_stocks(
        &self,
        req: CompareStocksRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW151701C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/compare-stocks", tr_id, req)
            .await
    }

    /// ELW LP매매추이
    ///
    /// - TR_ID: Real=FHPEW03760000 / VTS=
    /// - Endpoint: /uapi/elw/v1/quotations/lp-trade-trend
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW LP매매추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0376] ELW LP매매추이 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn lp_trade_trend(
        &self,
        req: LpTradeTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW03760000",
            crate::client::KisEnv::Vts => "",
        };
        self.0
            .get("/uapi/elw/v1/quotations/lp-trade-trend", tr_id, req)
            .await
    }

    /// ELW 투자지표추이(체결)
    ///
    /// - TR_ID: Real=FHPEW02740100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/indicator-trend-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 투자지표추이(체결) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0274] ELW 투자지표추이 화면에서 "시간별 비교추이" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn indicator_trend_ccnl(
        &self,
        req: IndicatorTrendCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02740100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/indicator-trend-ccnl", tr_id, req)
            .await
    }

    /// ELW 투자지표추이(분별)
    ///
    /// - TR_ID: Real=FHPEW02740300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/indicator-trend-minute
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 투자지표추이(분별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0274] ELW 투자지표추이 화면 데이터의 "분별 비교추이" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn indicator_trend_minute(
        &self,
        req: IndicatorTrendMinuteRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02740300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/indicator-trend-minute", tr_id, req)
            .await
    }

    /// ELW 투자지표추이(일별)
    ///
    /// - TR_ID: Real=FHPEW02740200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/indicator-trend-daily
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 투자지표추이(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0274] ELW 투자지표추이 화면에서 "일자별 비교추이" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn indicator_trend_daily(
        &self,
        req: IndicatorTrendDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02740200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/indicator-trend-daily", tr_id, req)
            .await
    }

    /// ELW 변동성 추이(틱)
    ///
    /// - TR_ID: Real=FHPEW02840400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-tick
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 변동성 추이(틱) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0284] ELW 변동성 추이 화면의 "틱 차트" 변동성 추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volatility_trend_tick(
        &self,
        req: VolatilityTrendTickRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02840400",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/volatility-trend-tick", tr_id, req)
            .await
    }

    /// ELW 변동성추이(체결)
    ///
    /// - TR_ID: Real=FHPEW02840100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 변동성 추이(체결) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0284] ELW 변동성 추이 화면의 "시간별" 변동성 추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volatility_trend_ccnl(
        &self,
        req: VolatilityTrendCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02840100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/volatility-trend-ccnl", tr_id, req)
            .await
    }

    /// ELW 변동성 추이(일별)
    ///
    /// - TR_ID: Real=FHPEW02840200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-daily
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 변동성 추이(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0284] ELW 변동성 추이 화면의 "일별" 변동성 추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volatility_trend_daily(
        &self,
        req: VolatilityTrendDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02840200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/volatility-trend-daily", tr_id, req)
            .await
    }

    /// ELW 민감도 추이(체결)
    ///
    /// - TR_ID: Real=FHPEW02830100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/sensitivity-trend-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 민감도 추이(체결) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0283] ELW 민감도 추이 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn sensitivity_trend_ccnl(
        &self,
        req: SensitivityTrendCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02830100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/sensitivity-trend-ccnl", tr_id, req)
            .await
    }

    /// ELW 변동성 추이(분별)
    ///
    /// - TR_ID: Real=FHPEW02840300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/volatility-trend-minute
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 변동성 추이(분별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0284] ELW 변동성 추이 화면의 "분별" 변동성 추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volatility_trend_minute(
        &self,
        req: VolatilityTrendMinuteRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02840300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/volatility-trend-minute",
                tr_id,
                req,
            )
            .await
    }

    /// ELW 민감도 추이(일별)
    ///
    /// - TR_ID: Real=FHPEW02830200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/sensitivity-trend-daily
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 민감도 추이(일별) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0283] ELW 민감도 추이 화면의 "일자별" 민감도추이 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn sensitivity_trend_daily(
        &self,
        req: SensitivityTrendDailyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02830200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/elw/v1/quotations/sensitivity-trend-daily",
                tr_id,
                req,
            )
            .await
    }

    /// ELW 만기예정/만기종목
    ///
    /// - TR_ID: Real=FHKEW154700C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/quotations/expiration-stocks
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 만기예정/만기종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0290] ELW 만기예정/만기종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 최근 100건까지 데이터 조회 가능합니다.
    pub async fn expiration_stocks(
        &self,
        req: ExpirationStocksRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKEW154700C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/quotations/expiration-stocks", tr_id, req)
            .await
    }

    /// 선물옵션 증거금률
    ///
    /// - TR_ID: Real=TTTO6032R / VTS=미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/margin-rate
    ///
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
    ///
    /// # Example (Scraped)
    /// ※ 승수, 계약당 선물 증거금은 최근월물 기준으로 표기되며, 월물에 따라 상이할 수 있습니다.
    /// ※ 계약당 선물 증거금은 선물 1계약 기준 신규 주문증거금이며 스프레드 증거금은 조회되지 않습니다.
    /// ※ 2023.05.24일부터 조회 가능하며, 익영업일 기준 증거금은 17:00~18:00시에 조회됩니다.
    /// ※ 데이터는 하루에 한 번 고정된 이후 데이터 변동이 없으므로  조회가 제한되는 점 이용에 참고 부탁드립니다.
    pub async fn margin_rate(&self, req: MarginRateRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTO6032R",
            crate::client::KisEnv::Vts => "미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/margin-rate",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 시세
    ///
    /// - TR_ID: Real=FHMIF10000000 / VTS=FHMIF10000000
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 시세 API입니다.
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn inquire_price_v2(
        &self,
        req: InquirePriceV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHMIF10000000",
            crate::client::KisEnv::Vts => "FHMIF10000000",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-price",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 시세호가
    ///
    /// - TR_ID: Real=FHMIF10010000 / VTS=FHMIF10010000
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-asking-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 시세호가 API입니다.
    pub async fn inquire_asking_price(
        &self,
        req: InquireAskingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHMIF10010000",
            crate::client::KisEnv::Vts => "FHMIF10010000",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-asking-price",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션기간별시세(일/주/월/년)
    ///
    /// - TR_ID: Real=FHKIF03020100 / VTS=FHKIF03020100
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice
    ///
    /// [국내선물옵션] 기본시세
    /// 선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// (지수)선물옵션 기간별시세 데이터(일/주/월/년) 조회 (최대 100건 조회)
    /// 실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    /// 모의계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    pub async fn inquire_daily_fuopchartprice(
        &self,
        req: InquireDailyFuopchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKIF03020100",
            crate::client::KisEnv::Vts => "FHKIF03020100",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 분봉조회
    ///
    /// - TR_ID: Real=FHKIF03020200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 분봉조회 API입니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 102건까지 확인 가능하며,
    /// FID_INPUT_DATE_1(입력날짜), FID_INPUT_HOUR_1(입력시간)을 이용하여 다음조회 가능합니다.
    pub async fn inquire_time_fuopchartprice(
        &self,
        req: InquireTimeFuopchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKIF03020200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 국내옵션전광판_옵션월물리스트
    ///
    /// - TR_ID: Real=FHPIO056104C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-option-list
    ///
    /// [국내선물옵션] 기본시세
    /// 국내옵션전광판_옵션월물리스트[국내선물-020]
    /// mtrt_yymm_code
    /// mtrt_yymm
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 국내업종 국내옵션전광판_옵션월물리스트 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0503] 선물옵션 종합시세(Ⅰ) 화면의 "월물리스트 목록 확인" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn display_board_option_list(
        &self,
        req: DisplayBoardOptionListRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPIO056104C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-option-list",
                tr_id,
                req,
            )
            .await
    }

    /// 국내선물 기초자산 시세
    ///
    /// - TR_ID: Real=FHPIF05030000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-top
    ///
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
    ///
    /// # Example (Scraped)
    /// 국내선물 기초자산 시세 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0503] 선물옵션 종합시세(Ⅰ) 화면의 "상단 바" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn display_board_top(
        &self,
        req: DisplayBoardTopRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPIF05030000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-top",
                tr_id,
                req,
            )
            .await
    }

    /// 국내옵션전광판_콜풋
    ///
    /// - TR_ID: Real=FHPIF05030100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-callput
    ///
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
    ///
    /// # Example (Scraped)
    /// 국내옵션전광판_콜풋 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0503] 선물옵션 종합시세(Ⅰ) 화면의 "중앙" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ output1, output2 각각 100건까지만 확인이 가능합니다. (FY25년도 서비스 개선 예정)
    /// ※ 조회시간이 긴 API인 점 참고 부탁드리며, 잦은 호출을 삼가해주시기 바랍니다. (1초당 최대 1건 권장)
    pub async fn display_board_callput(
        &self,
        req: DisplayBoardCallputRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPIF05030100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-callput",
                tr_id,
                req,
            )
            .await
    }

    /// 국내옵션전광판_선물
    ///
    /// - TR_ID: Real=FHPIF05030200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/display-board-futures
    ///
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
    ///
    /// # Example (Scraped)
    /// 국내옵션전광판_선물 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0503] 선물옵션 종합시세(Ⅰ) 화면의 "하단" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn display_board_futures(
        &self,
        req: DisplayBoardFuturesRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPIF05030200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/display-board-futures",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 일중예상체결추이
    ///
    /// - TR_ID: Real=FHPIF05110100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/quotations/exp-price-trend
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 일중예상체결추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0548] 선물옵션 예상체결추이 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn exp_price_trend(
        &self,
        req: ExpPriceTrendV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPIF05110100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/quotations/exp-price-trend",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 현재가상세
    ///
    /// - TR_ID: Real=HHDFS76200200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/price-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 현재가상세 API입니다.
    ///
    /// 해당 API를 활용하여 해외주식 종목의 매매단위(vnit), 호가단위(e_hogau), PER, PBR, EPS, BPS 등의 데이터를 확인하실 수 있습니다.
    ///
    /// 해외주식 시세는 무료시세(지연시세)만이 제공되며, API로는 유료시세(실시간시세)를 받아보실 수 없습니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// 이용에 유의 부탁드립니다.
    pub async fn price_detail(
        &self,
        req: PriceDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76200200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/price-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 현재가 호가
    ///
    /// - TR_ID: Real=HHDFS76200100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-asking-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 현재가 호가 API입니다.
    /// 미국 거래소는 10호가, 그 외 국가 거래소는 1호가만 제공됩니다.
    /// 한국투자 HTS(eFriend Plus) > [7620] 해외주식 현재가 화면에서 "왼쪽 호가 창" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 해외주식 시세는 무료시세(지연시세)만이 제공되며, API로는 유료시세(실시간시세)를 받아보실 수 없습니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// 이용에 유의 부탁드립니다.
    pub async fn inquire_asking_price_v2(
        &self,
        req: InquireAskingPriceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76200100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-asking-price",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 현재체결가
    ///
    /// - TR_ID: Real=HHDFS00000300 / VTS=HHDFS00000300
    /// - Endpoint: /uapi/overseas-price/v1/quotations/price
    ///
    /// [해외주식] 기본시세
    /// 해외주식 현재체결가[v1_해외주식-009]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식종목의 현재체결가를 확인하는 API 입니다.
    ///
    /// 해외주식 시세는 무료시세(지연시세)만이 제공되며, API로는 유료시세(실시간시세)를 받아보실 수 없습니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// 이용에 유의 부탁드립니다.
    pub async fn price(&self, req: PriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS00000300",
            crate::client::KisEnv::Vts => "HHDFS00000300",
        };
        self.0
            .get("/uapi/overseas-price/v1/quotations/price", tr_id, req)
            .await
    }

    /// 해외주식 체결추이
    ///
    /// - TR_ID: Real=HHDFS76200300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-ccnl
    ///
    /// [해외주식] 기본시세
    /// 해외주식 체결추이[해외주식-037]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 체결추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7625] 해외주식 체결추이 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn inquire_ccnl(
        &self,
        req: InquireCcnlV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76200300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식분봉조회
    ///
    /// - TR_ID: Real=HHDFS76950200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-time-itemchartprice
    ///
    /// [해외주식] 기본시세
    /// 해외주식분봉조회[v1_해외주식-030]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식분봉조회 API입니다. 실전계좌의 경우, 한 번의 호출에 최근 120건까지 확인 가능합니다.
    /// NEXT 및 KEYB 값을 사용하여 데이터를 계속해서 다음 조회할 수 있으며, 최대 다음조회 가능 기간은 약 1개월입니다.
    ///
    /// ※ 해외주식분봉조회 조회 방법
    /// params
    /// . 초기 조회:
    /// - PINC: "1" 입력
    /// - NEXT: 처음 조회 시, "" 공백 입력
    /// - KEYB: 처음 조회 시, "" 공백 입력
    /// . 다음 조회:
    /// - PINC: "1" 입력
    /// - NEXT: "1" 입력
    /// - KEYB: 이전 조회 결과의 마지막 분봉 데이터를 이용하여, 1분 전 혹은 n분 전의 시간을 입력 (형식: YYYYMMDDHHMMSS, ex. 20241014140100)
    ///
    /// * 따라서 분봉데이터를 기간별로 수집하고자 하실 경우 NEXT, KEYB 값을 이용하시면서 다음조회하시면 됩니다.
    /// * 한국투자 Github에서 해외주식 분봉 다음조회 파이썬 샘플코드 참고하실 수 있습니다. (아래 링크 참고)
    /// https://github.com/koreainvestment/open-trading-api/blob/main/rest/get_ovsstk_chart_price.py
    ///
    /// ※ 해외주식 분봉은 정규장만 과거조회 가능합니다.
    /// 미국주식 주간거래( EXCD: BAY, BAQ, BAA )의 경우 본 API로 최대 1일치 분봉만 조회가 가능합니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분지연) / 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분지연시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// 해외주식 시세는 무료시세(지연시세)만이 제공되며, API로는 유료시세(실시간시세)를 받아보실 수 없습니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// 이용에 유의 부탁드립니다.
    pub async fn inquire_time_itemchartprice(
        &self,
        req: InquireTimeItemchartpriceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76950200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-time-itemchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 해외지수분봉조회
    ///
    /// - TR_ID: Real=FHKST03030200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-time-indexchartprice
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외지수분봉조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0303] 해외지수 종합차트 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 102건까지 확인 가능합니다.
    pub async fn inquire_time_indexchartprice(
        &self,
        req: InquireTimeIndexchartpriceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03030200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-time-indexchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 기간별시세
    ///
    /// - TR_ID: Real=HHDFS76240000 / VTS=HHDFS76240000
    /// - Endpoint: /uapi/overseas-price/v1/quotations/dailyprice
    ///
    /// [해외주식] 기본시세
    /// 해외주식 기간별시세[v1_해외주식-010]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식의 기간별시세를 확인하는 API 입니다.
    /// 실전계좌/모의계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능합니다.
    ///
    /// 해외주식 시세는 무료시세(지연체결가)만이 제공되며, API로는 유료시세(실시간체결가)를 받아보실 수 없습니다.
    ///
    /// 해외주식 시세는 무료시세(지연시세)만이 제공되며, API로는 유료시세(실시간시세)를 받아보실 수 없습니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// 이용에 유의 부탁드립니다.
    pub async fn dailyprice(&self, req: DailypriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76240000",
            crate::client::KisEnv::Vts => "HHDFS76240000",
        };
        self.0
            .get("/uapi/overseas-price/v1/quotations/dailyprice", tr_id, req)
            .await
    }

    /// 해외주식 종목/지수/환율기간별시세(일/주/월/년)
    ///
    /// - TR_ID: Real=FHKST03030100 / VTS=FHKST03030100
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-daily-chartprice
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 종목/지수/환율기간별시세(일/주/월/년) API입니다.
    ///
    /// 해외지수 당일 시세의 경우 지연시세 or 종가시세가 제공됩니다.
    ///
    /// ※ 해당 API로 미국주식 조회 시, 다우30, 나스닥100, S&P500 종목만 조회 가능합니다.
    /// 더 많은 미국주식 종목 시세를 이용할 시에는, 해외주식기간별시세 API 사용 부탁드립니다.
    pub async fn inquire_daily_chartprice(
        &self,
        req: InquireDailyChartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST03030100",
            crate::client::KisEnv::Vts => "FHKST03030100",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-daily-chartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식조건검색
    ///
    /// - TR_ID: Real=HHDFS76410000 / VTS=HHDFS76410000
    /// - Endpoint: /uapi/overseas-price/v1/quotations/inquire-search
    ///
    /// [해외주식] 기본시세
    /// 해외주식조건검색[v1_해외주식-015]
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 조건검색 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7641] 해외주식 조건검색 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 현재 조건검색 결과값은 최대 100개까지 조회 가능합니다. 다음 조회(100개 이후의 값) 기능에 대해서는 개선검토 중에 있습니다.
    ///
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분지연) / 홍콩, 베트남, 중국, 일본 - 15분지연 (중국은 실시간시세 신청 시 무료실시간시세 제공)
    /// 미국의 경우 0분지연시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// ※ 2024년 12월 13일(금) 오후 5시부터 HTS(efriend Plus) [7781] 시세신청(실시간) 화면에서 유료 서비스 신청 후 접근토큰 발급하면 최대 2시간 이후 실시간 유료 시세 수신 가능
    ///
    /// ※ 그날 거래량이나 시세 형성이 안된 종목은 해외주식 기간별시세(HHDFS76240000)에서는 조회되지만
    /// 해외주식 조건검색(HHDFS76410000)에서 조회되지 않습니다. (EX. NAS AATC)
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세 서비스가 기본 제공되며, 유료 실시간 시세 서비스는 HTS ‘[7781] 시세신청 (실시간)’과 MTS(모바일) ‘고객서비스 > 거래 서비스신청 > 해외주식 > 해외 실시간시세 신청’ 에서 신청 가능합니다.
    /// ※ 무료(매수/매도 각 1호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : 미국 전체 거래소들의 통합 주문체결 및 최우선 호가
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로
    /// 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료∙유료 모두 미국에 상장된 종목(뉴욕, 나스닥, 아멕스 등)의 시세를 제공하며, 동일한 시스템을 사용하여 주문∙체결됩니다.
    /// 단, 무료∙유료의 기반 데이터 차이로 호가 및 체결 데이터는 차이가 발생할 수 있고, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 유료 실시간 시세 서비스와 다를 수 있으며,
    /// 종목별 과거 데이터(거래량, 시가, 종가, 고가, 차트 데이터 등)는 장 종료 후(오후 12시경) 유료 실시간 시세 서비스 데이터와 동일하게 업데이트됩니다.
    /// ■ 유료 실시간 시세 서비스는 신청 시 1~12개월까지 기간 선택 후 해당 요금을 일괄 납부하며,
    /// 해지 시 해지한 달의 말일까지 시세 제공 후 남은 기간 해당 금액이 환급되니 유의하시기 바랍니다.
    /// (출처: 한국투자증권 외화증권 거래설명서 - https://www.truefriend.com/main/customer/guide/Guide.jsp?&cmd=TF04ag010002¤tPage=1&num=64)
    /// ■ output2에 조회되는 순위(rank)는 거래량을 기준으로 내림차순 정렬 시 나오는 순위인 점 참고 부탁드립니다.
    pub async fn inquire_search(
        &self,
        req: InquireSearchRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76410000",
            crate::client::KisEnv::Vts => "HHDFS76410000",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/inquire-search",
                tr_id,
                req,
            )
            .await
    }

    /// 해외결제일자조회
    ///
    /// - TR_ID: Real=CTOS5011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/quotations/countries-holiday
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외결제일자조회 API입니다.
    pub async fn countries_holiday(
        &self,
        req: CountriesHolidayRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTOS5011R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/quotations/countries-holiday",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 상품기본정보
    ///
    /// - TR_ID: Real=CTPF1702R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/search-info
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 상품기본정보 API입니다.
    /// 시세제공기관(연합)에서 제공하는 해외주식 상품기본정보 데이터를 확인하실 수 있습니다.
    ///
    /// ※ 해당자료는 시세제공기관(연합)의 자료를 제공하고 있으며, 오류와 지연이 발생할 수 있습니다.
    /// ※ 위 정보에 의한 투자판단의 최종책임은 정보이용자에게 있으며, 당사와 시세제공기관(연합)는 어떠한 법적인 책임도 지지 않사오니 투자에 참고로만 이용하시기 바랍니다.
    pub async fn search_info(
        &self,
        req: SearchInfoV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF1702R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-price/v1/quotations/search-info", tr_id, req)
            .await
    }

    /// 해외주식 업종별시세
    ///
    /// - TR_ID: Real=HHDFS76370000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/industry-theme
    ///
    /// [해외주식] 기본시세
    /// 해외주식 업종별시세[해외주식-048]
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 업종별시세 API입니다.
    pub async fn industry_theme(
        &self,
        req: IndustryThemeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76370000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/industry-theme",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 업종별코드조회
    ///
    /// - TR_ID: Real=HHDFS76370100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/industry-price
    ///
    /// [해외주식] 기본시세
    /// 해외주식 업종별코드조회[해외주식-049]
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 업종별코드조회 API입니다.
    pub async fn industry_price(
        &self,
        req: IndustryPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76370100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/industry-price",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 복수종목 시세조회
    ///
    /// - TR_ID: Real=HHDFS76220000 / VTS=미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/multprice
    ///
    /// [해외주식] 기본시세
    /// 해외주식 복수종목 시세조회
    /// t_xprc
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// ※ 지연시세 지연시간 : 미국 - 실시간무료(0분 지연, 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보)
    /// 홍콩, 베트남, 중국, 일본 - 15분지연
    /// 미국의 경우 0분 지연 시세로 제공되나, 장중 당일 시가는 상이할 수 있으며, 익일 정정 표시됩니다.
    ///
    /// [미국주식시세 이용시 유의사항]
    /// ■ 무료 실시간 시세(나스닥 토탈뷰)를 별도 신청없이 제공하고 있으며, 유료 시세 서비스를 신청하시더라도 OpenAPI의 경우 무료 시세로만 제공하고있습니다.
    /// ※ 무료(매수/매도 각 10호가) : 나스닥 마켓센터에서 거래되는 호가 및 호가 잔량 정보
    /// ※ 유료(매수/매도 각 1호가) : OpenAPI 서비스 미제공
    /// ■ 무료 실시간 시세 서비스는 유료 실시간 시세 서비스 대비 평균 50% 수준에 해당하는 정보이므로 현재가/호가/순간체결량/차트 등에서 일시적·부분적 차이가 있을 수 있습니다.
    /// ■ 무료 실시간 시세 서비스의 시가, 저가, 고가, 종가는 타 매체의 유료 실시간 시세 서비스와 다를 수 있으며, 이로 인해 발생하는 손실에 대해서 당사가 책임지지 않습니다.
    /// 이용에 유의 부탁드립니다.
    pub async fn multprice(&self, req: MultpriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76220000",
            crate::client::KisEnv::Vts => "미지원",
        };
        self.0
            .get("/uapi/overseas-price/v1/quotations/multprice", tr_id, req)
            .await
    }

    /// 해외주식 기간별권리조회
    ///
    /// - TR_ID: Real=CTRGT011R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/period-rights
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 기간별권리조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7520] 기간별해외증권권리조회 화면을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 확정여부가 '예정'으로 표시되는 경우는 권리정보가 변경될 수 있으니 참고자료로만 활용하시기 바랍니다.
    pub async fn period_rights(
        &self,
        req: PeriodRightsV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRGT011R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/period-rights",
                tr_id,
                req,
            )
            .await
    }

    /// 해외뉴스종합(제목)
    ///
    /// - TR_ID: Real=HHPSTH60100C1 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/news-title
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외뉴스종합(제목) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7702] 해외뉴스종합 화면의 "우측 상단 뉴스목록" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn news_title(&self, req: NewsTitleV2Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHPSTH60100C1",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-price/v1/quotations/news-title", tr_id, req)
            .await
    }

    /// 해외주식 권리종합
    ///
    /// - TR_ID: Real=HHDFS78330900 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/rights-by-ice
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 권리종합 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7833] 해외주식 권리(ICE제공) 화면의 "전체" 탭 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 조회기간 기준일 입력시 참고 - 상환: 상환일자, 조기상환: 조기상환일자, 티커변경: 적용일, 그 외: 발표일
    pub async fn rights_by_ice(
        &self,
        req: RightsByIceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS78330900",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/rights-by-ice",
                tr_id,
                req,
            )
            .await
    }

    /// 당사 해외주식담보대출 가능 종목
    ///
    /// - TR_ID: Real=CTLN4050R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/colable-by-company
    ///
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
    ///
    /// # Example (Scraped)
    /// 당사 해외주식담보대출 가능 종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0497] 당사 해외주식담보대출 가능 종목 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 한 번의 호출에 20건까지 조회가 가능하며 다음조회가 불가하기에, PDNO에 데이터 확인하고자 하는 종목코드를 입력하여 단건조회용으로 사용하시기 바랍니다.
    pub async fn colable_by_company(
        &self,
        req: ColableByCompanyRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTLN4050R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/colable-by-company",
                tr_id,
                req,
            )
            .await
    }

    /// 해외속보(제목)
    ///
    /// - TR_ID: Real=FHKST01011801 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-price/v1/quotations/brknews-title
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외속보(제목) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7704] 해외속보 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 최대 100건까지 조회 가능합니다.
    pub async fn brknews_title(
        &self,
        req: BrknewsTitleRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKST01011801",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-price/v1/quotations/brknews-title",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물종목현재가
    ///
    /// - TR_ID: Real=HHDFC55010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-price
    ///
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
    ///
    /// # Example (Scraped)
    /// (중요) 해외선물시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드(해외) - 해외지수선물 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    ///
    /// [참고자료]
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    ///
    /// ※ 모의투자는 실전투자계좌를 활용하여 조회 부탁드립니다.
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn inquire_price_v3(
        &self,
        req: InquirePriceV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55010000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-price",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물종목상세
    ///
    /// - TR_ID: Real=HHDFC55010100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/stock-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// (중요) 해외선물시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드(해외) - 해외지수선물 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석.
    ///
    /// ※ 모의투자는 실전투자계좌를 활용하여 조회 부탁드립니다.
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn stock_detail(
        &self,
        req: StockDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55010100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/stock-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 호가
    ///
    /// - TR_ID: Real=HHDFC86000000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-asking-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물 호가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [8602] 해외선물옵션 종합주문(Ⅰ) 화면에서 "왼쪽 호가 창" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// (중요) 해외선물옵션시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드 - 해외선물옵션 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    ///
    /// [참고자료]
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn inquire_asking_price_v3(
        &self,
        req: InquireAskingPriceV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC86000000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-asking-price",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 분봉조회
    ///
    /// - TR_ID: Real=HHDFC55020400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물분봉조회 API입니다. ★ 반드시 아래 호출방법을 확인하시고 호출 사용하시기 바랍니다.
    /// 한국투자 HTS(eFriend Plus) > [5502] 해외선물옵션 체결추이 화면에서 "분" 선택 시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    ///
    /// ※ 해외선물분봉조회 조회 방법
    /// params
    /// . START_DATE_TIME: 공란 입력 ("")
    /// . CLOSE_DATE_TIME: 조회일자 입력 ("20231214")
    /// . QRY_CNT: 120 입력 시, 가장 최근 분봉 120건 조회( 한번에 최대 120건 조회 가능)
    /// 240 입력 시, 240 이전 분봉 ~ 120 이전 분봉 조회
    /// 360 입력 시, 360 이전 분봉 ~ 240 이전 분봉 조회
    /// . QRY_TP: 처음조회시, 공백 입력
    /// 다음조회시, P 입력
    /// . INDEX_KEY: 처음조회시, 공백 입력
    /// 다음조회시, 이전 조회 응답의 output2 > index_key 값 입력
    ///
    /// * 따라서 분봉데이터를 기간별로 수집하고자 하실 경우 QRY_TP, INDEX_KEY 값을 이용하시면서 다음조회하시면 됩니다.
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn inquire_time_futurechartprice(
        &self,
        req: InquireTimeFuturechartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55020400",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(틱)
    ///
    /// - TR_ID: Real=HHDFC55020200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/tick-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 체결추이(틱) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [5502] 해외선물옵션 체결추이 화면에서 "Tick" 선택 시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// (중요) 해외선물시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드(해외) - 해외지수선물 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn tick_ccnl(&self, req: TickCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55020200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/tick-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(주간)
    ///
    /// - TR_ID: Real=HHDFC55020000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/weekly-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 체결추이(주간) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [5502] 해외선물옵션 체결추이 화면에서 "주간" 선택 시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// (중요) 해외선물시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드(해외) - 해외지수선물 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn weekly_ccnl(&self, req: WeeklyCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55020000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/weekly-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(일간)
    ///
    /// - TR_ID: Real=HHDFC55020100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/daily-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 체결추이(일간) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [5502] 해외선물옵션 체결추이 화면에서 "일간" 선택 시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// (중요) 해외선물시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드(해외) - 해외지수선물 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn daily_ccnl(&self, req: DailyCcnlRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55020100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/daily-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 체결추이(월간)
    ///
    /// - TR_ID: Real=HHDFC55020300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/monthly-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 체결추이(월간) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [5502] 해외선물옵션 체결추이 화면에서 "월간" 선택 시 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// (중요) 해외선물시세 출력값을 해석하실 때 ffcode.mst(해외선물종목마스터 파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - ffcode.mst(해외선물종목마스터 파일) 다운로드 방법 2가지
    /// 1) 한국투자증권 Github의 파이썬 샘플코드를 사용하여 mst 파일 다운로드 및 excel 파일로 정제
    /// https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/overseas_future_code.py
    ///
    /// 2) 혹은 포럼 - FAQ - 종목정보 다운로드(해외) - 해외지수선물 클릭하셔서 ffcode.mst(해외선물종목마스터 파일)을 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외선물정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, ffcode.mst(해외선물종목마스터 파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) ffcode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 6A 계산소수점 -4 → 시세 6882.5 수신 시 0.68825 로 해석
    /// 품목코드 GC 계산소수점 -1 → 시세 19225 수신 시 1922.5 로 해석
    ///
    /// ※ CME, SGX 거래소 API시세는 유료시세로 HTS/MTS에서 유료가입 후 익일부터 시세 이용 가능합니다.
    /// 포럼 > FAQ > 해외선물옵션 API 유료시세 신청방법(CME, SGX 거래소)
    pub async fn monthly_ccnl(
        &self,
        req: MonthlyCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55020300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/monthly-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 상품기본정보
    ///
    /// - TR_ID: Real=HHDFC55200000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/search-contract-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 상품기본정보 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0054] 해외선물옵션 상품기본정보 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// QRY_CNT에 SRS_CD 요청 개수 입력, SRS_CD_01 ~SRS_CD_32 까지 최대 32건의 상품코드 추가 입력하여 해외선물옵션 상품기본정보 확인이 가능합니다.
    pub async fn search_contract_detail(
        &self,
        req: SearchContractDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFC55200000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/search-contract-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물 미결제추이
    ///
    /// - TR_ID: Real=HHDDB95030000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/investor-unpd-trend
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물 미결제추이 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [1959] 해외선물 미결제추이의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn investor_unpd_trend(
        &self,
        req: InvestorUnpdTrendRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDDB95030000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/investor-unpd-trend",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션종목현재가
    ///
    /// - TR_ID: Real=HHDFO55010000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션종목현재가 API입니다.
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn opt_price(&self, req: OptPriceRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55010000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-price",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션종목상세
    ///
    /// - TR_ID: Real=HHDFO55010100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션종목상세 API입니다.
    ///
    /// (주의) sstl_price 자리에 정산가 X 전일종가 O 가 수신되는 점 유의 부탁드립니다.
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn opt_detail(&self, req: OptDetailRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55010100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 호가
    ///
    /// - TR_ID: Real=HHDFO86000000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-asking-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 호가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [5501] 해외선물옵션 현재가 화면 의 "왼쪽 상단 현재가" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn opt_asking_price(
        &self,
        req: OptAskingPriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO86000000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-asking-price",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 분봉조회
    ///
    /// - TR_ID: Real=HHDFO55020400 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/inquire-time-optchartprice
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 분봉조회 API입니다.
    /// 한 번의 호출에 120건까지 확인 가능하며, QRY_TP, INDEX_KEY 를 이용하여 다음조회 가능합니다.
    ///
    /// ※ 다음조회 방법
    /// (처음조회) "QRY_TP":"Q", "QRY_CNT":"120", "INDEX_KEY":""
    /// (다음조회) "QRY_TP":"P", "QRY_CNT":"120", "INDEX_KEY":"20240902         5"  ◀ 이전 호출의 "output1 > index_key" 기입
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn inquire_time_optchartprice(
        &self,
        req: InquireTimeOptchartpriceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55020400",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/inquire-time-optchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(틱)
    ///
    /// - TR_ID: Real=HHDFO55020200 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 체결추이(틱) API입니다.
    /// 한 번의 호출에 40건까지 확인 가능하며, QRY_TP, INDEX_KEY 를 이용하여 다음조회 가능합니다.
    ///
    /// ※ 다음조회 방법
    /// (처음조회) "QRY_TP":"Q", "QRY_CNT":"40", "INDEX_KEY":""
    /// (다음조회) "QRY_TP":"P", "QRY_CNT":"40", "INDEX_KEY":"20240906       221"  ◀ 이전 호출의 "output1 > index_key" 기입
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn opt_tick_ccnl(
        &self,
        req: OptTickCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55020200",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(일간)
    ///
    /// - TR_ID: Real=HHDFO55020100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-daily-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 체결추이(일간) API입니다.
    /// 최근 120건까지 데이터 확인이 가능합니다. ("QRY_CNT: 119 입력", START_DATE_TIME, CLOSE_DATE_TIME은 공란)
    ///
    /// ※ 호출 시 유의사항
    /// : START_DATE_TIME, CLOSE_DATE_TIME은 공란 입력, QRY_CNT는 확인 데이터 개수의 -1 개 입력
    /// ex) "START_DATE_TIME":"","CLOSE_DATE_TIME":"","QRY_CNT":"119" → 최근 120건 데이터 조회
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn opt_daily_ccnl(
        &self,
        req: OptDailyCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55020100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-daily-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(주간)
    ///
    /// - TR_ID: Real=HHDFO55020000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-weekly-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 체결추이(주간) API입니다.
    /// 최근 120건까지 데이터 확인이 가능합니다. (START_DATE_TIME, CLOSE_DATE_TIME은 공란 입력)
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn opt_weekly_ccnl(
        &self,
        req: OptWeeklyCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55020000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-weekly-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 체결추이(월간)
    ///
    /// - TR_ID: Real=HHDFO55020300 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/opt-monthly-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 체결추이(월간) API입니다.
    /// 최근 120건까지 데이터 확인이 가능합니다. (START_DATE_TIME, CLOSE_DATE_TIME은 공란 입력)
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn opt_monthly_ccnl(
        &self,
        req: OptMonthlyCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55020300",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/opt-monthly-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외옵션 상품기본정보
    ///
    /// - TR_ID: Real=HHDFO55200000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/search-opt-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외옵션 상품기본정보 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0054] 관심종목 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// (중요) 해외옵션시세 출력값을 해석하실 때 focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)에 있는 sCalcDesz(계산 소수점) 값을 활용하셔야 정확한 값을 받아오실 수 있습니다.
    ///
    /// - focode.mst(해외지수옵션 종목마스터파일), (해외주식옵션 종목마스터파일) 다운로드 방법
    /// 1) focode.mst(해외지수옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외지수옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외옵션정보.h)를 참고하여 해석
    /// 2) fostkcode.mst(해외주식옵션 종목마스터파일)
    /// : 포럼 > FAQ > 종목정보 다운로드(해외) - 해외주식옵션 클릭하여 다운로드 후
    /// Github의 헤더정보(https://github.com/koreainvestment/open-trading-api/blob/main/stocks_info/해외주식옵션정보.h)를 참고하여 해석
    ///
    /// - 소수점 계산 시, focode.mst(해외지수옵션 종목마스터파일), fostkcode.mst(해외주식옵션 종목마스터파일)의 sCalcDesz(계산 소수점) 값 참고
    /// EX) focode.mst 파일의 sCalcDesz(계산 소수점) 값
    /// 품목코드 OES 계산소수점 -2 → 시세 7525 수신 시 75.25 로 해석
    /// 품목코드 O6E 계산소수점 -4 → 시세 54.0 수신 시 0.0054 로 해석
    pub async fn search_opt_detail(
        &self,
        req: SearchOptDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFO55200000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/search-opt-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 장운영시간
    ///
    /// - TR_ID: Real=OTFM2229R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/quotations/market-time
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물 장운영시간 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [6773] 해외선물 장운영시간 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn market_time(
        &self,
        req: MarketTimeV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM2229R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/quotations/market-time",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권현재가(호가)
    ///
    /// - TR_ID: Real=FHKBJ773401C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-asking-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권현재가(호가) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 "우측 호가창" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn inquire_asking_price_v4(
        &self,
        req: InquireAskingPriceV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKBJ773401C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-asking-price",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권현재가(시세)
    ///
    /// - TR_ID: Real=FHKBJ773400C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권현재가(시세) API입니다.
    /// 장내채권의 기본시세(시가,고가,저가,종가)를 확인할 수 있습니다.
    pub async fn inquire_price_v4(
        &self,
        req: InquirePriceV5Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKBJ773400C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-price",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권현재가(체결)
    ///
    /// - TR_ID: Real=FHKBJ773403C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권현재가(체결) API입니다
    /// 장내채권의 체결데이터를 확인할 수 있습니다.
    pub async fn inquire_ccnl_v2(
        &self,
        req: InquireCcnlV5Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKBJ773403C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-bond/v1/quotations/inquire-ccnl", tr_id, req)
            .await
    }

    /// 장내채권현재가(일별)
    ///
    /// - TR_ID: Real=FHKBJ773404C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-daily-price
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권현재가(일별) API입니다.
    /// 장내채권의 일별 시세데이터를 최근 100건까지 확인할 수 있습니다.
    pub async fn inquire_daily_price(
        &self,
        req: InquireDailyPriceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKBJ773404C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-daily-price",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권 기간별시세(일)
    ///
    /// - TR_ID: Real=FHKBJ773701C0 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/inquire-daily-itemchartprice
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 기간별시세(일) API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0979] 장내채권종합주문 화면 가운데 "일별" 클릭 시 시세 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// 최근 30건까지 데이터 확인이 가능합니다.
    pub async fn inquire_daily_itemchartprice(
        &self,
        req: InquireDailyItemchartpriceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHKBJ773701C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/inquire-daily-itemchartprice",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권 평균단가조회
    ///
    /// - TR_ID: Real=CTPF2005R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/avg-unit
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 평균단가조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7216] 채권 발행정보 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn avg_unit(&self, req: AvgUnitRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF2005R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-bond/v1/quotations/avg-unit", tr_id, req)
            .await
    }

    /// 장내채권 발행정보
    ///
    /// - TR_ID: Real=CTPF1101R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/issue-info
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 발행정보 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7216] 채권 발행정보 화면의 상단 채권정보 데이터를 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn issue_info(&self, req: IssueInfoRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF1101R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-bond/v1/quotations/issue-info", tr_id, req)
            .await
    }

    /// 장내채권 기본조회
    ///
    /// - TR_ID: Real=CTPF1114R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/quotations/search-bond-info
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 기본조회 API입니다.
    /// 장내채권의 상품정보를 확인 가능합니다.
    pub async fn search_bond_info(
        &self,
        req: SearchBondInfoRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTPF1114R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/quotations/search-bond-info",
                tr_id,
                req,
            )
            .await
    }
}

#[allow(non_snake_case)]
impl Ranking {
    /// ELW 민감도 순위
    ///
    /// - TR_ID: Real=FHPEW02850000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/sensitivity
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 민감도 순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0285] ELW 민감도 순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn sensitivity(
        &self,
        req: SensitivityRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02850000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/ranking/sensitivity", tr_id, req)
            .await
    }

    /// ELW 당일급변종목
    ///
    /// - TR_ID: Real=FHPEW02870000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/quick-change
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 당일급변종목 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0287] ELW 당일급변종목 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn quick_change(
        &self,
        req: QuickChangeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02870000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/ranking/quick-change", tr_id, req)
            .await
    }

    /// ELW 지표순위
    ///
    /// - TR_ID: Real=FHPEW02790000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/indicator
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 지표순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0279] ELW 지표순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn indicator(&self, req: IndicatorRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02790000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/ranking/indicator", tr_id, req)
            .await
    }

    /// ELW 상승률순위
    ///
    /// - TR_ID: Real=FHPEW02770000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/updown-rate
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 상승률순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0277] ELW 상승률순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn updown_rate(&self, req: UpdownRateRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02770000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/ranking/updown-rate", tr_id, req)
            .await
    }

    /// ELW 거래량순위
    ///
    /// - TR_ID: Real=FHPEW02780000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/elw/v1/ranking/volume-rank
    ///
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
    ///
    /// # Example (Scraped)
    /// ELW 거래량순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0278] ELW 거래량순위 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volume_rank(&self, req: VolumeRankRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "FHPEW02780000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/elw/v1/ranking/volume-rank", tr_id, req)
            .await
    }

    /// 해외주식 가격급등락
    ///
    /// - TR_ID: Real=HHDFS76260000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/price-fluct
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 가격급등락 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7626] 가격급등락 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn price_fluct(&self, req: PriceFluctRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76260000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/price-fluct", tr_id, req)
            .await
    }

    /// 해외주식 거래량급증
    ///
    /// - TR_ID: Real=HHDFS76270000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/volume-surge
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 거래량급증 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7627] 거래대금순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volume_surge(
        &self,
        req: VolumeSurgeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76270000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/volume-surge", tr_id, req)
            .await
    }

    /// 해외주식 매수체결강도상위
    ///
    /// - TR_ID: Real=HHDFS76280000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/volume-power
    ///
    /// [해외주식] 시세분석
    /// 해외주식 매수체결강도상위[해외주식-040]
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 매수체결강도상위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7628] 매수체결강도상위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn volume_power(
        &self,
        req: VolumePowerV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76280000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/volume-power", tr_id, req)
            .await
    }

    /// 해외주식 상승율/하락율
    ///
    /// - TR_ID: Real=HHDFS76290000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/updown-rate
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 상승율/하락율 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7629] 상승율/하락율 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn updown_rate_v2(
        &self,
        req: UpdownRateV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76290000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/updown-rate", tr_id, req)
            .await
    }

    /// 해외주식 신고/신저가
    ///
    /// - TR_ID: Real=HHDFS76300000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/new-highlow
    ///
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
    ///
    /// # Example (Scraped)
    /// "해외주식 신고/신저가 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7630] 신고/신저가 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다."
    pub async fn new_highlow(&self, req: NewHighlowRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76300000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/new-highlow", tr_id, req)
            .await
    }

    /// 해외주식 거래량순위
    ///
    /// - TR_ID: Real=HHDFS76310010 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-vol
    ///
    /// [해외주식] 시세분석
    /// 해외주식 거래량순위[해외주식-043]
    /// a_tvol
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 거래량순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7631] 거래대금순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn trade_vol(&self, req: TradeVolRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76310010",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/trade-vol", tr_id, req)
            .await
    }

    /// 해외주식 거래대금순위
    ///
    /// - TR_ID: Real=HHDFS76320010 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-pbmn
    ///
    /// [해외주식] 시세분석
    /// 해외주식 거래대금순위[해외주식-044]
    /// a_tamt
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 거래대금순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7632] 거래대금순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn trade_pbmn(&self, req: TradePbmnRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76320010",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/trade-pbmn", tr_id, req)
            .await
    }

    /// 해외주식 거래증가율순위
    ///
    /// - TR_ID: Real=HHDFS76330000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-growth
    ///
    /// [해외주식] 시세분석
    /// 해외주식 거래증가율순위[해외주식-045]
    /// n_tvol
    /// n_rate
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 거래증가율순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7633] 거래증가율순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn trade_growth(
        &self,
        req: TradeGrowthRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76330000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/trade-growth", tr_id, req)
            .await
    }

    /// 해외주식 거래회전율순위
    ///
    /// - TR_ID: Real=HHDFS76340000 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/trade-turnover
    ///
    /// [해외주식] 시세분석
    /// 해외주식 거래회전율순위[해외주식-046]
    /// n_tvol
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 거래회전율순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7634] 거래회전율순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn trade_turnover(
        &self,
        req: TradeTurnoverRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76340000",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/trade-turnover", tr_id, req)
            .await
    }

    /// 해외주식 시가총액순위
    ///
    /// - TR_ID: Real=HHDFS76350100 / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/ranking/market-cap
    ///
    /// [해외주식] 시세분석
    /// 해외주식 시가총액순위[해외주식-047]
    /// e_ordyn
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 시가총액순위 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7635] 시가총액순위 화면 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn market_cap(&self, req: MarketCapV2Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHDFS76350100",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/ranking/market-cap", tr_id, req)
            .await
    }
}

#[allow(non_snake_case)]
impl Trading {
    /// 선물옵션 주문
    ///
    /// - TR_ID: Real=TTTO1101U / VTS=VTTO1101U
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/order
    ///
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
    ///
    /// # Example (Scraped)
    /// ​선물옵션 주문 API입니다.
    /// * 선물옵션 운영시간 외 API 호출 시 애러가 발생하오니 운영시간을 확인해주세요.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn order(&self, req: OrderRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTO1101U",
            crate::client::KisEnv::Vts => "VTTO1101U",
        };
        self.0
            .post("/uapi/domestic-futureoption/v1/trading/order", tr_id, req)
            .await
    }

    /// 선물옵션 정정취소주문
    ///
    /// - TR_ID: Real=TTTO1103U / VTS=VTTO1103U
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/order-rvsecncl
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 주문 건에 대하여 정정 및 취소하는 API입니다. 단, 이미 체결된 건은 정정 및 취소가 불가합니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    pub async fn order_rvsecncl(
        &self,
        req: OrderRvsecnclV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTO1103U",
            crate::client::KisEnv::Vts => "VTTO1103U",
        };
        self.0
            .post(
                "/uapi/domestic-futureoption/v1/trading/order-rvsecncl",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 주문체결내역조회
    ///
    /// - TR_ID: Real=TTTO5201R / VTS=VTTO5201R
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 주문체결내역조회 API입니다. 한 번의 호출에 최대 100건​까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    pub async fn inquire_ccnl(
        &self,
        req: InquireCcnlV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTO5201R",
            crate::client::KisEnv::Vts => "VTTO5201R",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 잔고현황
    ///
    /// - TR_ID: Real=CTFO6118R / VTS=VTFO6118R
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 잔고현황 API입니다. 한 번의 호출에 최대 20건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    pub async fn inquire_balance(
        &self,
        req: InquireBalanceV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTFO6118R",
            crate::client::KisEnv::Vts => "VTFO6118R",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 주문가능
    ///
    /// - TR_ID: Real=TTTO5105R / VTS=VTTO5105R
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-psbl-order
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 주문가능 API입니다. 주문가능 내역과 수량을 확인하실 수 있습니다.
    pub async fn inquire_psbl_order(
        &self,
        req: InquirePsblOrderV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTO5105R",
            crate::client::KisEnv::Vts => "VTTO5105R",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-psbl-order",
                tr_id,
                req,
            )
            .await
    }

    /// (야간)선물옵션 주문체결 내역조회
    ///
    /// - TR_ID: Real=JTCE5005R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// (야간)선물옵션 주문체결 내역조회 API입니다.
    ///
    /// 1. 야간 시장이 종료(06:00)된 이후 약 06:10경 야간시장의 주문체결내역이 주간으로 이관됩니다.
    /// > 주간 API를 사용한다면 야간 장 중 주문체결내역을 실시간으로 조회할 수 없습니다.
    /// > 주문체결내역의 이관이 완료되는 시점부터 주간 테이블에서 야간의 주문체결내역을 조회할 수 있습니다.
    ///
    /// 2. KRX야간시장의 경우 주문일자는 (T+1)일 입니다.
    /// > 금요일의 경우 주문일자는 주말 및 공휴일을 제외하고 익 영업일인 월요일로 설정됩니다.
    /// > 위 내용은 당사의 기준이 아닌 KRX 거래소의 기준으로 전 회원사 동일한 기준으로 주문체결이 이루어지고 있습니다.
    pub async fn inquire_ngt_ccnl(
        &self,
        req: InquireNgtCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "JTCE5005R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// (야간)선물옵션 잔고현황
    ///
    /// - TR_ID: Real=JTCE6001R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ngt-balance
    ///
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
    ///
    /// # Example (Scraped)
    /// (야간)선물옵션 잔고현황 API입니다.
    pub async fn inquire_ngt_balance(
        &self,
        req: InquireNgtBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "JTCE6001R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ngt-balance",
                tr_id,
                req,
            )
            .await
    }

    /// (야간)선물옵션 주문가능 조회
    ///
    /// - TR_ID: Real=JTCE1004R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order
    ///
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
    ///
    /// # Example (Scraped)
    /// (야간)선물옵션 주문가능 조회 API입니다.
    pub async fn inquire_psbl_ngt_order(
        &self,
        req: InquirePsblNgtOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "JTCE1004R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order",
                tr_id,
                req,
            )
            .await
    }

    /// (야간)선물옵션 증거금 상세
    ///
    /// - TR_ID: Real=JTCE6003R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/ngt-margin-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// (야간)선물옵션 증거금상세 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [2537] 야간선물옵션 증거금상세 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn ngt_margin_detail(
        &self,
        req: NgtMarginDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "JTCE6003R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/ngt-margin-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 잔고정산손익내역
    ///
    /// - TR_ID: Real=CTFO6117R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 잔고정산손익내역 API입니다.
    pub async fn inquire_balance_settlement_pl(
        &self,
        req: InquireBalanceSettlementPlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTFO6117R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 총자산현황
    ///
    /// - TR_ID: Real=CTRP6550R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-deposit
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 총자산현황 API 입니다.
    pub async fn inquire_deposit(
        &self,
        req: InquireDepositV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRP6550R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-deposit",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 잔고평가손익내역
    ///
    /// - TR_ID: Real=CTFO6159R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 잔고평가손익내역 API입니다.
    pub async fn inquire_balance_valuation_pl(
        &self,
        req: InquireBalanceValuationPlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTFO6159R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션 기준일체결내역
    ///
    /// - TR_ID: Real=CTFO5139R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션 기준일체결내역 API입니다.
    pub async fn inquire_ccnl_bstime(
        &self,
        req: InquireCcnlBstimeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTFO5139R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime",
                tr_id,
                req,
            )
            .await
    }

    /// 선물옵션기간약정수수료일별
    ///
    /// - TR_ID: Real=CTFO6119R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee
    ///
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
    ///
    /// # Example (Scraped)
    /// 선물옵션기간약정수수료일별 API입니다.
    pub async fn inquire_daily_amount_fee(
        &self,
        req: InquireDailyAmountFeeRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTFO6119R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 주문
    ///
    /// - TR_ID: Real=TTTT1002U / VTS=VTTT1002U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order
    ///
    /// [해외주식] 주문/계좌
    /// 해외주식 주문[v1_해외주식-001]
    /// KRX_FWDG_ORD_ORGNO
    /// ORD_TMD
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 주문 API입니다.
    ///
    /// * 모의투자의 경우, 모든 해외 종목 매매가 지원되지 않습니다. 일부 종목만 매매 가능한 점 유의 부탁드립니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// * 해외 거래소 운영시간 외 API 호출 시 에러가 발생하오니 운영시간을 확인해주세요. (미국주식 주간주문은 "해외주식 미국주간주문"을 이용)
    /// * 해외 거래소 운영시간(한국시간 기준)
    /// 1) 미국 : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
    /// * 프리마켓(18:00 ~ 23:30, Summer Time : 17:00 ~ 22:30), 애프터마켓(06:00 ~ 07:00, Summer Time : 05:00 ~ 07:00) 시간대에도 주문 가능
    /// 2) 일본 : (오전) 09:00 ~ 11:30, (오후) 12:30 ~ 15:00
    /// 3) 상해 : 10:30 ~ 16:00
    /// 4) 홍콩 : (오전) 10:30 ~ 13:00, (오후) 14:00 ~ 17:00
    ///
    /// * 기존에는 내부통제 요건에 따라 상장주식수의 1%를 초과하는 주문은 접수할 수 없었으나, 2025.08.14 시행 이후부터는 접수가 가능합니다. 단, 타 매체(HTS 등)는 안내 팝업 확인 후 주문이 가능하지만, Open API는 별도의 안내 화면 없이 주문이 바로 접수되므로 유의하시기 바랍니다.
    ///
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn order_v2(&self, req: OrderV2Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT1002U",
            crate::client::KisEnv::Vts => "VTTT1002U",
        };
        self.0
            .post("/uapi/overseas-stock/v1/trading/order", tr_id, req)
            .await
    }

    /// 해외주식 정정취소주문
    ///
    /// - TR_ID: Real=TTTT1004U / VTS=VTTT1004U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-rvsecncl
    ///
    /// [해외주식] 주문/계좌
    /// 해외주식 정정취소주문[v1_해외주식-003]
    /// KRX_FWDG_ORD_ORGNO
    /// ORD_TMD
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 접수된 해외주식 주문을 정정하거나 취소하기 위한 API입니다.
    /// (해외주식주문 시 Return 받은 ODNO를 참고하여 API를 호출하세요.)
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// * 해외 거래소 운영시간 외 API 호출 시 에러가 발생하오니 운영시간을 확인해주세요.
    /// * 해외 거래소 운영시간(한국시간 기준)
    /// 1) 미국 : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
    /// * 프리마켓(18:00 ~ 23:30, Summer Time : 17:00 ~ 22:30), 애프터마켓(06:00 ~ 07:00, Summer Time : 05:00 ~ 07:00) 시간대에도 주문 가능
    /// 2) 일본 : (오전) 09:00 ~ 11:30, (오후) 12:30 ~ 15:00
    /// 3) 상해 : 10:30 ~ 16:00
    /// 4) 홍콩 : (오전) 10:30 ~ 13:00, (오후) 14:00 ~ 17:00
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    pub async fn order_rvsecncl_v2(
        &self,
        req: OrderRvsecnclV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT1004U",
            crate::client::KisEnv::Vts => "VTTT1004U",
        };
        self.0
            .post("/uapi/overseas-stock/v1/trading/order-rvsecncl", tr_id, req)
            .await
    }

    /// 해외주식 예약주문접수
    ///
    /// - TR_ID: Real=TTTT3014U / VTS=VTTT3014U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv
    ///
    /// [해외주식] 주문/계좌
    /// 해외주식 예약주문접수[v1_해외주식-002]
    /// RSVN_ORD_RCIT_DT
    /// OVRS_RSVN_ODNO
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 미국거래소 운영시간 외 미국주식을 예약 매매하기 위한 API입니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// * 아래 각 국가의 시장별 예약주문 접수 가능 시간을 확인하시길 바랍니다.
    ///
    /// 미국 예약주문 접수시간
    /// 1) 10:00 ~ 23:20 / 10:00 ~ 22:20 (서머타임 시)
    /// 2) 주문제한 : 16:30 ~ 16:45 경까지 (사유 : 시스템 정산작업시간)
    /// 3) 23:30 정규장으로 주문 전송 (서머타임 시 22:30 정규장 주문 전송)
    /// 4) 미국 거래소 운영시간(한국시간 기준) : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
    ///
    /// 홍콩 예약주문 접수시간
    /// 1) 09:00 ~ 10:20 접수, 10:30 주문전송
    /// 2) 10:40 ~ 13:50 접수, 14:00 주문전송
    ///
    /// 중국 예약주문 접수시간
    /// 1) 09:00 ~ 10:20 접수, 10:30 주문전송
    /// 2) 10:40 ~ 13:50 접수, 14:00 주문전송
    ///
    /// 일본 예약주문 접수시간
    /// 1) 09:10 ~ 12:20 까지 접수, 12:30 주문전송
    ///
    /// 베트남 예약주문 접수시간
    /// 1) 09:00 ~ 11:00 까지 접수, 11:15 주문전송
    /// 2) 11:20 ~ 14:50 까지 접수, 15:00 주문전송
    ///
    /// * 예약주문 유의사항
    /// 1) 예약주문 유효기간 : 당일
    /// - 미국장 마감 후, 미체결주문은 자동취소
    /// - 미국휴장 시, 익 영업일로 이전
    /// (미국예약주문화면에서 취소 가능)
    /// 2) 증거금 및 잔고보유 : 체크 안함
    /// 3) 주문전송 불가사유
    /// - 매수증거금 부족: 수수료 포함 매수금액부족, 환전, 시세이용료 출금, 인출에 의한 증거금 부족
    /// - 기타 매수증거금 부족, 매도가능수량 부족, 주권변경 등 권리발생으로 인한 주문불가사유 발생
    /// 4) 지정가주문만 가능
    /// * 단 미국 예약매도주문(TTTT3016U)의 경우, MOO(장개시시장가)로 주문 접수 가능
    pub async fn order_resv(&self, req: OrderResvV2Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT3014U",
            crate::client::KisEnv::Vts => "VTTT3014U",
        };
        self.0
            .post("/uapi/overseas-stock/v1/trading/order-resv", tr_id, req)
            .await
    }

    /// 해외주식 예약주문접수취소
    ///
    /// - TR_ID: Real=TTTT3017U / VTS=VTTT3017U
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-ccnl
    ///
    /// [해외주식] 주문/계좌
    /// 해외주식 예약주문접수취소[v1_해외주식-004]
    /// OVRS_RSVN_ODNO
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 접수된 미국주식 예약주문을 취소하기 위한 API입니다.
    /// (해외주식 예약주문접수 시 Return 받은 ODNO를 참고하여 API를 호출하세요.)
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    pub async fn order_resv_ccnl(
        &self,
        req: OrderResvCcnlV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT3017U",
            crate::client::KisEnv::Vts => "VTTT3017U",
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/order-resv-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 매수가능금액조회
    ///
    /// - TR_ID: Real=TTTS3007R / VTS=VTTS3007R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-psamount
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 매수가능금액조회 API입니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    pub async fn inquire_psamount(
        &self,
        req: InquirePsamountRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3007R",
            crate::client::KisEnv::Vts => "VTTS3007R",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-psamount",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 미체결내역
    ///
    /// - TR_ID: Real=TTTS3018R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-nccs
    ///
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
    ///
    /// # Example (Scraped)
    /// 접수된 해외주식 주문 중 체결되지 않은 미체결 내역을 조회하는 API입니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 40건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    ///
    /// ※ 해외주식 미체결내역 API 모의투자에서는 사용이 불가합니다.
    /// 모의투자로 해외주식 미체결내역 확인시에는 해외주식 주문체결내역[v1_해외주식-007] API 조회하셔서 nccs_qty(미체결수량)으로 해외주식 미체결수량을 조회하실 수 있습니다.
    ///
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// * 해외 거래소 운영시간(한국시간 기준)
    /// 1) 미국 : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
    /// * 프리마켓(18:00 ~ 23:30, Summer Time : 17:00 ~ 22:30), 애프터마켓(06:00 ~ 07:00, Summer Time : 05:00 ~ 07:00)
    /// 2) 일본 : (오전) 09:00 ~ 11:30, (오후) 12:30 ~ 15:00
    /// 3) 상해 : 10:30 ~ 16:00
    /// 4) 홍콩 : (오전) 10:30 ~ 13:00, (오후) 14:00 ~ 17:00
    pub async fn inquire_nccs(
        &self,
        req: InquireNccsRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3018R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/trading/inquire-nccs", tr_id, req)
            .await
    }

    /// 해외주식 잔고
    ///
    /// - TR_ID: Real=TTTS3012R / VTS=VTTS3012R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-balance
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 잔고를 조회하는 API 입니다.
    /// 한국투자 HTS(eFriend Plus) > [7600] 해외주식 종합주문 화면의 좌측 하단 '실시간잔고' 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    /// 다만 미국주간거래 가능종목에 대해서는 frcr_evlu_pfls_amt(외화평가손익금액), evlu_pfls_rt(평가손익율), ovrs_stck_evlu_amt(해외주식평가금액), now_pric2(현재가격2) 값이 HTS와는 상이하게 표출될 수 있습니다.
    /// (주간시간 시간대에 HTS는 주간시세로 노출, API로는 야간시세로 노출)
    ///
    /// 실전계좌의 경우, 한 번의 호출에 최대 100건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// * 미니스탁 잔고는 해당 API로 확인이 불가합니다.
    pub async fn inquire_balance_v2(
        &self,
        req: InquireBalanceV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3012R",
            crate::client::KisEnv::Vts => "VTTS3012R",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 주문체결내역
    ///
    /// - TR_ID: Real=TTTS3035R / VTS=VTTS3035R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 일정 기간의 해외주식 주문 체결 내역을 확인하는 API입니다.
    /// 실전계좌의 경우, 한 번의 호출에 최대 20건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    /// 모의계좌의 경우, 한 번의 호출에 최대 15건까지 확인 가능하며, 이후의 값은 연속조회를 통해 확인하실 수 있습니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    ///
    /// * 해외 거래소 운영시간(한국시간 기준)
    /// 1) 미국 : 23:30 ~ 06:00 (썸머타임 적용 시 22:30 ~ 05:00)
    /// * 프리마켓(18:00 ~ 23:30, Summer Time : 17:00 ~ 22:30), 애프터마켓(06:00 ~ 07:00, Summer Time : 05:00 ~ 07:00)
    /// 2) 일본 : (오전) 09:00 ~ 11:30, (오후) 12:30 ~ 15:00
    /// 3) 상해 : 10:30 ~ 16:00
    /// 4) 홍콩 : (오전) 10:30 ~ 13:00, (오후) 14:00 ~ 17:00
    pub async fn inquire_ccnl_v2(
        &self,
        req: InquireCcnlV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3035R",
            crate::client::KisEnv::Vts => "VTTS3035R",
        };
        self.0
            .get("/uapi/overseas-stock/v1/trading/inquire-ccnl", tr_id, req)
            .await
    }

    /// 해외주식 체결기준현재잔고
    ///
    /// - TR_ID: Real=CTRP6504R / VTS=VTRP6504R
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-present-balance
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 잔고를 체결 기준으로 확인하는 API 입니다.
    ///
    /// HTS(eFriend Plus) [0839] 해외 체결기준잔고 화면을 API로 구현한 사항으로 화면을 함께 보시면 기능 이해가 쉽습니다.
    ///
    /// (※모의계좌의 경우 output3(외화평가총액 등 확인 가능)만 정상 출력됩니다.
    /// 잔고 확인을 원하실 경우에는 해외주식 잔고[v1_해외주식-006] API 사용을 부탁드립니다.)
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// 해외주식 체결기준현재잔고 유의사항
    /// 1. 해외증권 체결기준 잔고현황을 조회하는 화면입니다.
    /// 2. 온라인국가는 수수료(국내/해외)가 반영된 최종 정산금액으로 잔고가 변동되며, 결제작업 지연등으로 인해 조회시간은 차이가 발생할 수 있습니다.
    /// - 아시아 온라인국가 : 매매일 익일    08:40 ~ 08:45분 경
    /// - 미국 온라인국가   : 당일 장 종료후 08:40 ~ 08:45분 경
    /// ※ 단, 애프터연장 참여 신청계좌는 10:30 ~ 10:35분 경(Summer Time : 09:30 ~ 09:35분 경)에 최종 정산금액으로 변동됩니다.
    /// 3. 미국 현재가 항목은 주간시세 및 애프터시세는 반영하지 않으며, 정규장 마감 후에는 종가로 조회됩니다.
    /// 4. 온라인국가를 제외한 국가의 현재가는 실시간 시세가 아니므로 주문화면의 잔고 평가금액 등과 차이가 발생할 수 있습니다.
    /// 5. 해외주식 담보대출 매도상환 체결내역은 해당 잔고화면에 반영되지 않습니다.
    /// 결제가 완료된 이후 외화잔고에 포함되어 반영되오니 참고하여 주시기 바랍니다.
    /// 6. 외화평가금액은 당일 최초고시환율이 적용된 금액으로 실제 환전금액과는 차이가 있습니다.
    /// 7. 미국은 메인 시스템이 아닌 별도 시스템을 통해 거래되므로, 18시 10~15분 이후 발생하는 미국 매매내역은 해당 화면에 실시간으로 반영되지 않으니 하단 내용을 참고하여 안내하여 주시기 바랍니다.
    /// [외화잔고 및 해외 유가증권 현황 조회]
    /// - 일반/통합증거금 계좌 : 미국장 종료 + 30분 후 부터 조회 가능
    /// 단, 통합증거금 계좌에 한해 주문금액은 외화잔고 항목에 실시간 반영되며, 해외 유가증권 현황은 반영되지
    /// 않아 해외 유가증권 평가금액이 과다 또는 과소 평가될 수 있습니다.
    /// - 애프터연장 신청계좌  : 실시간 반영
    /// 단, 시스템정산작업시간(23:40~00:10) 및 거래량이 많은 경우 메인시스템에 반영되는 시간으로 인해 차이가
    /// 발생할 수 있습니다.
    /// ※ 배치작업시간에 따라 시간은 변동될 수 있습니다.
    pub async fn inquire_present_balance(
        &self,
        req: InquirePresentBalanceV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRP6504R",
            crate::client::KisEnv::Vts => "VTRP6504R",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-present-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 예약주문조회
    ///
    /// - TR_ID: Real=TTTT3039R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/order-resv-list
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 예약주문 조회 API입니다.
    /// ※ 모의투자는 사용 불가합니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    pub async fn order_resv_list(
        &self,
        req: OrderResvListRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT3039R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/order-resv-list",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 결제기준잔고
    ///
    /// - TR_ID: Real=CTRP6010R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 결제기준잔고 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0829] 해외 결제기준잔고 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 적용환율은 당일 매매기준이며, 현재가의 경우 지연된 시세로 평가되므로 실제매도금액과 상이할 수 있습니다.
    /// ※ 주문가능수량 : 보유수량 - 미결제 매도수량
    /// ※ 매입금액 계산 시 결제일의 최초고시환율을 적용하므로, 금일 최초고시환율을 적용하는 체결기준 잔고와는 상이합니다.
    /// ※ 해외증권 투자 및 업무문의 안내: 한국투자증권 해외투자지원부 02)3276-5300
    pub async fn inquire_paymt_stdr_balance(
        &self,
        req: InquirePaymtStdrBalanceRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTRP6010R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 일별거래내역
    ///
    /// - TR_ID: Real=CTOS4001R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-period-trans
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 일별거래내역 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0828] 해외증권 일별거래내역 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ 체결가격, 매매금액, 정산금액, 수수료 원화금액은 국내 결제일까지는 예상환율로 적용되고, 국내 결제일 익일부터 확정환율로 적용됨으로 금액이 변경될 수 있습니다.
    /// ※ 해외증권 투자 및 업무문의 안내: 한국투자증권 해외투자지원부 02)3276-5300
    pub async fn inquire_period_trans(
        &self,
        req: InquirePeriodTransRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTOS4001R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-period-trans",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 기간손익
    ///
    /// - TR_ID: Real=TTTS3039R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-period-profit
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 기간손익 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7717] 해외 기간손익 화면의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// [해외 기간손익 유의 사항]
    /// ■ 단순 매체결내역을 기초로 만든 화면으로 매도체결시점의 체결기준 매입단가와 비교하여 손익이 계산됩니다.
    /// 결제일의 환율과 금액을 기준으로 산출하는 해외주식 양도소득세 계산방식과는 상이하오니, 참고용으로만 활용하여 주시기 바랍니다.
    /// ■ 기간손익은 매매일 익일부터 조회가능합니다.
    /// ﻿﻿■ 매입금액/매도금액 원화 환산 시 매도일의 환율이 적용되어있습니다.
    /// ﻿﻿■ 손익금액의 비용은 "매도비용" 만 포함되어있습니다. 단, 동일 종목의 매수/매도가 동시에 있는 경우에는 해당일 발생한 매수비용도 함께 계산됩니다.
    /// ﻿﻿■ 담보상환내역은 기간손익화면에 표시되지 많으니 참고하여 주시기 바랍니다.
    pub async fn inquire_period_profit(
        &self,
        req: InquirePeriodProfitV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS3039R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-period-profit",
                tr_id,
                req,
            )
            .await
    }

    /// 해외증거금 통화별조회
    ///
    /// - TR_ID: Real=TTTC2101R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/foreign-margin
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외증거금 통화별조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [7718] 해외주식 증거금상세 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn foreign_margin(
        &self,
        req: ForeignMarginRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC2101R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/trading/foreign-margin", tr_id, req)
            .await
    }

    /// 해외주식 미국주간주문
    ///
    /// - TR_ID: Real=TTTS6036U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order
    ///
    /// [해외주식] 주문/계좌
    /// 해외주식 미국주간주문[v1_해외주식-026]
    /// KRX_FWDG_ORD_ORGNO
    /// ORD_TMD
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 미국주간주문 API입니다.
    ///
    /// * 미국주식 주간거래 시 아래 참고 부탁드립니다.
    /// . 포럼 > FAQ > 미국주식 주간거래 시 어떤 API를 사용해야 하나요?
    ///
    /// * 미국주간거래의 경우, 모든 미국 종목 매매가 지원되지 않습니다. 일부 종목만 매매 가능한 점 유의 부탁드립니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// * 미국주간거래시간 외 API 호출 시 에러가 발생하오니 운영시간을 확인해주세요.
    /// . 주간거래(장전거래)(한국시간 기준) : 10:00 ~ 18:00 (Summer Time 동일)
    ///
    /// * 한국투자증권 해외주식 시장별 매매안내(매매수수료, 거래시간 안내, 결제일 정보, 환전안내)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca050000.jsp
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn daytime_order(
        &self,
        req: DaytimeOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6036U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/overseas-stock/v1/trading/daytime-order", tr_id, req)
            .await
    }

    /// 해외주식 미국주간정정취소
    ///
    /// - TR_ID: Real=TTTS6038U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/daytime-order-rvsecncl
    ///
    /// [해외주식] 주문/계좌
    /// 해외주식 미국주간정정취소[v1_해외주식-027]
    /// KRX_FWDG_ORD_ORGNO
    /// ORD_TMD
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외주식 미국주간정정취소 API입니다.
    ///
    /// * 미국주식 주간거래 시 아래 참고 부탁드립니다.
    /// . 포럼 > FAQ > 미국주식 주간거래 시 어떤 API를 사용해야 하나요?
    ///
    /// * 미국주간거래의 경우, 모든 미국 종목 매매가 지원되지 않습니다. 일부 종목만 매매 가능한 점 유의 부탁드립니다.
    ///
    /// * 해외주식 서비스 신청 후 이용 가능합니다. (아래 링크 3번 해외증권 거래신청 참고)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca010001.jsp
    ///
    /// * 미국주간거래시간 외 API 호출 시 에러가 발생하오니 운영시간을 확인해주세요.
    /// . 주간거래(장전거래)(한국시간 기준) : 10:00 ~ 18:00 (Summer Time 동일)
    ///
    /// * 한국투자증권 해외주식 시장별 매매안내(매매수수료, 거래시간 안내, 결제일 정보, 환전안내)
    /// https://securities.koreainvestment.com/main/bond/research/_static/TF03ca050000.jsp
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn daytime_order_rvsecncl(
        &self,
        req: DaytimeOrderRvsecnclRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6038U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post(
                "/uapi/overseas-stock/v1/trading/daytime-order-rvsecncl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외주식 지정가주문번호조회
    ///
    /// - TR_ID: Real=TTTS6058R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/algo-ordno
    ///
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
    ///
    /// # Example (Scraped)
    /// TWAP, VWAP 주문에 대한 주문번호를 조회하는 API
    pub async fn algo_ordno(&self, req: AlgoOrdnoRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6058R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-stock/v1/trading/algo-ordno", tr_id, req)
            .await
    }

    /// 해외주식 지정가체결내역조회
    ///
    /// - TR_ID: Real=TTTS6059R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-stock/v1/trading/inquire-algo-ccnl
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외주식 TWAP, VWAP 주문에 대한 체결내역 조회 API로 지정가 주문번호조회 API를 수행 후 조회해야합니다
    pub async fn inquire_algo_ccnl(
        &self,
        req: InquireAlgoCcnlRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTS6059R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-stock/v1/trading/inquire-algo-ccnl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 주문
    ///
    /// - TR_ID: Real=OTFM3001U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/order
    ///
    /// [해외선물옵션] 주문/계좌
    /// 해외선물옵션 주문 [v1_해외선물-001]
    /// ORD_DT
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 주문 API 입니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    ///
    /// ※ 종목코드 마스터파일 파이썬 정제코드는 한국투자증권 Github 참고 부탁드립니다.
    /// https://github.com/koreainvestment/open-trading-api/tree/main/stocks_info
    pub async fn order_v3(&self, req: OrderV3Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3001U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/overseas-futureoption/v1/trading/order", tr_id, req)
            .await
    }

    /// 해외선물옵션 정정취소주문
    ///
    /// - TR_ID: Real=OTFM3002U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/order-rvsecncl
    ///
    /// [해외선물옵션] 주문/계좌
    /// 해외선물옵션 정정취소주문 [v1_해외선물-002, 003]
    /// ORD_DT
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 정정취소주문 API 입니다.
    ///
    /// ※ POST API의 경우 BODY값의 key값들을 대문자로 작성하셔야 합니다.
    /// (EX. "CANO" : "12345678", "ACNT_PRDT_CD": "01",...)
    pub async fn order_rvsecncl_v3(
        &self,
        req: OrderRvsecnclV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3002U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post(
                "/uapi/overseas-futureoption/v1/trading/order-rvsecncl",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 당일주문내역조회
    ///
    /// - TR_ID: Real=OTFM3116R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-ccld
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 당일주문내역조회 API입니다.
    pub async fn inquire_ccld(
        &self,
        req: InquireCcldRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3116R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-ccld",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 미결제내역조회(잔고)
    ///
    /// - TR_ID: Real=OTFM1412R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-unpd
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 미결제내역조회(잔고) API입니다.
    pub async fn inquire_unpd(
        &self,
        req: InquireUnpdRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM1412R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-unpd",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 주문가능조회
    ///
    /// - TR_ID: Real=OTFM3304R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-psamount
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 주문가능조회 API입니다.
    pub async fn inquire_psamount_v2(
        &self,
        req: InquirePsamountV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3304R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-psamount",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 기간계좌손익 일별
    ///
    /// - TR_ID: Real=OTFM3118R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-period-ccld
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 기간계좌손익 일별 API입니다.
    pub async fn inquire_period_ccld(
        &self,
        req: InquirePeriodCcldRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3118R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-period-ccld",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 일별 체결내역
    ///
    /// - TR_ID: Real=OTFM3122R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-daily-ccld
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 일별 체결내역 API입니다.
    /// 거래소 체결 내역에 따라 , output1에 동일한 주문번호의 데이터들이 수신될 수 있습니다.
    pub async fn inquire_daily_ccld(
        &self,
        req: InquireDailyCcldV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3122R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-daily-ccld",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 예수금현황
    ///
    /// - TR_ID: Real=OTFM1411R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-deposit
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 예수금현황 API입니다.
    pub async fn inquire_deposit_v2(
        &self,
        req: InquireDepositV3Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM1411R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-deposit",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 일별 주문내역
    ///
    /// - TR_ID: Real=OTFM3120R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-daily-order
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 일별 주문내역 API입니다.
    pub async fn inquire_daily_order(
        &self,
        req: InquireDailyOrderRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3120R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-daily-order",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 기간계좌거래내역
    ///
    /// - TR_ID: Real=OTFM3114R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/inquire-period-trans
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 기간계좌거래내역 API입니다.
    pub async fn inquire_period_trans_v2(
        &self,
        req: InquirePeriodTransV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3114R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/inquire-period-trans",
                tr_id,
                req,
            )
            .await
    }

    /// 해외선물옵션 증거금상세
    ///
    /// - TR_ID: Real=OTFM3115R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/overseas-futureoption/v1/trading/margin-detail
    ///
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
    ///
    /// # Example (Scraped)
    /// 해외선물옵션 증거금상세 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [2711] 해외선물옵션 증거금상세 화면 의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// [증거금 상세설명]
    /// - SPAN, EUREX 증거금
    /// 1. 가격변동증거금 : 보유하고 있는 미결제를 Product Class 별로 구간[SPAN-16구간, EUREX-29구간)손익 합계액 산출하며 최대손실구간의 금액을 해당 Class의 증거금으로 산정
    /// 2. 스프레드증거금 : 보유하고 있는 미결제를 Product Class 별로 스프레드 산정하며 스프레드 증거금 적용
    /// ** 스프레드 산정방법 : SPAN은 선물+옵션의 Delta Spread로 계산, EUREX는 선물의 Spread만 산정 보유중인 옵선가치를 평가하며 청산가치가 양수(고객미 수취할 금액이 있는 경우)에 해당하는 금액을 증거금에서 할인
    /// 3. 옵션가격증거금 : 보유중인 옵션가치를 평가하여 청산가치가 양수(고객이 수취할 금액이 있는 경우)에 해당하는 금액을 증거금에서 할인
    /// **계산식 : MAXID, 온선평가대금 Class별 합계액) ** 산출된 값을 음수처리함 옵션 미결제약정에 대해 최소로 징구하는 증거금
    /// 4. 옵선최소증거금 증거금 : 옵션 미결제약정에 대해 최소로 징구하는 증거금
    /// ﻿** SPAN : 매도옵선회소증거금(행사가별로 상미)과 매수옵선최소증거금(계약당 1Tick에 해당하는 금액)
    /// ** EUREX : 매수옵선최소증거금(계약당 1Tick에 해당하는 금액)(EUREX는 매도옵션최소증거금이 가격변동증거금에 포함되어 있음)
    /// 5. 일방해소증거금 : (기본개념)보유중인 포트폴리오 중에서 머느 일방향을 전량 청산했을 경우 잔존하는 미결제 약정의 최대손실가능액을 사전에 징구함
    /// 가격상승포지션과 가격하락포지션에 대해 최불리증거금을 각각 산정하며 큰 금액을 증거금으로 장구
    /// ﻿* 가격장승포지션 : 선물매수포지션, 풋옵션매도포지션
    /// ﻿* 가격하락포지션 : 선물매도포지션, 콜옵션매도포지선
    ///
    /// - 일반 증거금
    /// 1. 선물미결제증거금 : 선물미결제약정에 대해 계약당증거금율 적용
    /// 2. 매도옵션미결제증거금 : 매도옵션미결제약정에 대해 옵선계약당 증거금을 적용
    /// ** 옵션계약당증거금 : 각 종목별 최불리증거금액으로 해외 거래소에서 계산하며 제공되는 데이터임
    /// 3. 매수옵션미결제증거금 : 매수옵션최소증거금으로 1Tick에 해당하는 금액을 적용
    ///
    /// - 주문 증거금
    /// 1. 선물 주문증거금 : 선물 미체결주문에 대해 계약당 증거금을 적용(신규주문에 한해 징수)
    /// 2. 매도옵션 주문증거금 : 옵션매도 미체결주문에 대해 계약당증거금을 적용(신규주문에 한해 징수)
    /// 3. 매수옵션 주문증거금 : 옵션매수 미체결주문에 대해 최소증거금(Tick Value와 10 중에서 큰 금액)과 만기행사예약한 미체결주문에 대한 행사예약증거금을 징수
    /// 4. 매수옵션 주문대금 : 옵션매수 미체결주문의 매수대금(주문가격을 기준으로 대금 산정, 시장가주문시 현재가 +50틱으로 매수대금 산정)
    /// 5. 매수옵선행사예약증거금 : 옵선매수 미결제약정 중에서 행사예약한 수량에 대해 기초자산선물의 계약당 증거금을 징수
    pub async fn margin_detail(
        &self,
        req: MarginDetailRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3115R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/overseas-futureoption/v1/trading/margin-detail",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권 매수주문
    ///
    /// - TR_ID: Real=TTTC0952U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/buy
    ///
    /// [장내채권] 주문/계좌
    /// 장내채권 매수주문 [국내주식-124]
    /// krx_fwdg_ord_orgno
    /// ord_tmd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 장내채권 매수주문 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 '채권매수' 탭의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn buy(&self, req: BuyRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0952U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/domestic-bond/v1/trading/buy", tr_id, req)
            .await
    }

    /// 장내채권 매도주문
    ///
    /// - TR_ID: Real=TTTC0958U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/sell
    ///
    /// [장내채권] 주문/계좌
    /// 장내채권 매도주문 [국내주식-123]
    /// krx_fwdg_ord_orgno
    /// ord_tmd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 장내채권 매도주문 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 '채권매도' 탭의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn sell(&self, req: SellRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0958U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/domestic-bond/v1/trading/sell", tr_id, req)
            .await
    }

    /// 장내채권 정정취소주문
    ///
    /// - TR_ID: Real=TTTC0953U / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/order-rvsecncl
    ///
    /// [장내채권] 주문/계좌
    /// 장내채권 정정취소주문 [국내주식-125]
    /// krx_fwdg_ord_orgno
    /// ord_tmd
    /// true friend 한국투자 Open API
    /// KIS Developers COPYRIGHTS
    /// 2021-12-29 11:22:33
    /// 0.0.0.0
    ///
    /// # Example (Scraped)
    /// 장내채권 정정취소주문 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 '채권정정/취소' 탭의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn order_rvsecncl_v4(
        &self,
        req: OrderRvsecnclV5Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC0953U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/domestic-bond/v1/trading/order-rvsecncl", tr_id, req)
            .await
    }

    /// 채권정정취소가능주문조회
    ///
    /// - TR_ID: Real=CTSC8035R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-psbl-rvsecncl
    ///
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
    ///
    /// # Example (Scraped)
    /// 채권정정취소가능주문조회 API입니다.
    /// 정정취소가능한 채권주문 목록을 확인할 수 있습니다.
    pub async fn inquire_psbl_rvsecncl(
        &self,
        req: InquirePsblRvsecnclV2Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC8035R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-psbl-rvsecncl",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권 주문체결내역
    ///
    /// - TR_ID: Real=CTSC8013R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-daily-ccld
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 주문체결내역 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 '채권주문체결' 탭의 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn inquire_daily_ccld_v2(
        &self,
        req: InquireDailyCcldV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC8013R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-daily-ccld",
                tr_id,
                req,
            )
            .await
    }

    /// 장내채권 잔고조회
    ///
    /// - TR_ID: Real=CTSC8407R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-balance
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 잔고조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0979] 장내채권종합주문 화면의 "왼쪽 하단 잔고" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    pub async fn inquire_balance_v3(
        &self,
        req: InquireBalanceV5Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "CTSC8407R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/domestic-bond/v1/trading/inquire-balance", tr_id, req)
            .await
    }

    /// 장내채권 매수가능조회
    ///
    /// - TR_ID: Real=TTTC8910R / VTS=모의투자 미지원
    /// - Endpoint: /uapi/domestic-bond/v1/trading/inquire-psbl-order
    ///
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
    ///
    /// # Example (Scraped)
    /// 장내채권 매수가능조회 API입니다.
    /// 한국투자 HTS(eFriend Plus) > [0978] 장내채권주문 화면의 "왼쪽 하단 증거금 사용가능 내역 / 주문가능금액 및 수량" 기능을 API로 개발한 사항으로, 해당 화면을 참고하시면 기능을 이해하기 쉽습니다.
    ///
    /// ※ (중요) 채권의 경우 주식과 달리, 매수가능수량(buy_psbl_qty) = 매수가능금액(buy_psbl_amt) / 채권주문단가2(bond_ord_unpr2) * 10 인 점 유의하시기 바랍니다.
    pub async fn inquire_psbl_order_v2(
        &self,
        req: InquirePsblOrderV4Request,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTC8910R",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get(
                "/uapi/domestic-bond/v1/trading/inquire-psbl-order",
                tr_id,
                req,
            )
            .await
    }
}
