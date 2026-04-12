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
    /// 2021-12-29 11:22:33
    pub async fn tokenP(&self, req: TokenPRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/tokenP", tr_id, req).await
    }

    /// 접근토큰폐기(P)
    /// 2021-12-29 11:22:33
    pub async fn revokeP(&self, req: RevokePRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/revokeP", tr_id, req).await
    }

    /// Hashkey
    /// 2021-12-29 11:22:33
    pub async fn hashkey(&self, req: HashkeyRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/uapi/hashkey", tr_id, req).await
    }

    /// 실시간 (웹소켓) 접속키 발급
    /// 실시간 (웹소켓) 접속키 발급[실시간-000]
    pub async fn Approval(&self, req: ApprovalRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "",
            crate::client::KisEnv::Vts => "",
        };
        self.0.post("/oauth2/Approval", tr_id, req).await
    }

    /// 국내주식 실시간체결가 (KRX)
    /// 국내주식 실시간체결가 (KRX) [실시간-003]
    pub async fn H0STCNT0(&self, req: H0Stcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STCNT0",
            crate::client::KisEnv::Vts => "H0STCNT0",
        };
        self.0.post("/tryitout/H0STCNT0", tr_id, req).await
    }

    /// 국내주식 실시간호가 (KRX)
    /// 국내주식 실시간호가 (KRX) [실시간-004]
    pub async fn H0STASP0(&self, req: H0Stasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STASP0",
            crate::client::KisEnv::Vts => "H0STASP0",
        };
        self.0.post("/tryitout/H0STASP0", tr_id, req).await
    }

    /// 국내주식 실시간체결통보
    /// 국내주식 실시간체결통보 [실시간-005]
    pub async fn H0STCNI0(&self, req: H0Stcni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STCNI0",
            crate::client::KisEnv::Vts => "H0STCNI9",
        };
        self.0.post("/tryitout/H0STCNI0", tr_id, req).await
    }

    /// 국내주식 실시간예상체결 (KRX)
    /// 국내주식 실시간예상체결 (KRX) [실시간-041]
    pub async fn H0STANC0(&self, req: H0Stanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STANC0", tr_id, req).await
    }

    /// 국내주식 실시간회원사 (KRX)
    /// 국내주식 실시간회원사 (KRX) [실시간-047]
    pub async fn H0STMBC0(&self, req: H0Stmbc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STMBC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STMBC0", tr_id, req).await
    }

    /// 국내주식 실시간프로그램매매 (KRX)
    /// 국내주식 실시간프로그램매매 (KRX) [실시간-048]
    pub async fn H0STPGM0(&self, req: H0Stpgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STPGM0", tr_id, req).await
    }

    /// 국내주식 장운영정보 (KRX)
    /// 국내주식 장운영정보 (KRX) [실시간-049]
    pub async fn H0STMKO0(&self, req: H0Stmko0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STMKO0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STMKO0", tr_id, req).await
    }

    /// 국내주식 시간외 실시간호가 (KRX)
    /// 국내주식 시간외 실시간호가 (KRX) [실시간-025]
    pub async fn H0STOAA0(&self, req: H0Stoaa0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STOAA0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STOAA0", tr_id, req).await
    }

    /// 국내주식 시간외 실시간체결가 (KRX)
    /// 국내주식 시간외 실시간체결가 (KRX) [실시간-042]
    pub async fn H0STOUP0(&self, req: H0Stoup0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STOUP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STOUP0", tr_id, req).await
    }

    /// 국내주식 시간외 실시간예상체결 (KRX)
    /// 국내주식 시간외 실시간예상체결 (KRX) [실시간-024]
    pub async fn H0STOAC0(&self, req: H0Stoac0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STOAC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STOAC0", tr_id, req).await
    }

    /// 국내지수 실시간체결
    /// 국내지수 실시간체결 [실시간-026]
    pub async fn H0UPCNT0(&self, req: H0Upcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UPCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UPCNT0", tr_id, req).await
    }

    /// 국내지수 실시간예상체결
    /// 국내지수 실시간예상체결 [실시간-027]
    pub async fn H0UPANC0(&self, req: H0Upanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UPANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UPANC0", tr_id, req).await
    }

    /// 국내지수 실시간프로그램매매
    /// 국내지수 실시간프로그램매매 [실시간-028]
    pub async fn H0UPPGM0(&self, req: H0Uppgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UPPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UPPGM0", tr_id, req).await
    }

    /// ELW 실시간호가
    /// 2021-12-29 11:22:33
    pub async fn H0EWASP0(&self, req: H0Ewasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EWASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EWASP0", tr_id, req).await
    }

    /// ELW 실시간체결가
    /// ELW 실시간체결가 [실시간-061]
    pub async fn H0EWCNT0(&self, req: H0Ewcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EWCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EWCNT0", tr_id, req).await
    }

    /// ELW 실시간예상체결
    /// ELW 실시간예상체결 [실시간-063]
    pub async fn H0EWANC0(&self, req: H0Ewanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EWANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EWANC0", tr_id, req).await
    }

    /// 국내ETF NAV추이
    /// 국내ETF NAV추이 [실시간-051]
    pub async fn H0STNAV0(&self, req: H0Stnav0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0STNAV0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0STNAV0", tr_id, req).await
    }

    /// 국내주식 실시간체결가 (통합)
    /// 2021-12-29 11:22:33
    pub async fn H0UNCNT0(&self, req: H0Uncnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNCNT0", tr_id, req).await
    }

    /// 국내주식 실시간호가 (통합)
    /// 2021-12-29 11:22:33
    pub async fn H0UNASP0(&self, req: H0Unasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNASP0", tr_id, req).await
    }

    /// 국내주식 실시간예상체결 (통합)
    /// 2021-12-29 11:22:33
    pub async fn H0UNANC0(&self, req: H0Unanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNANC0", tr_id, req).await
    }

    /// 국내주식 실시간회원사 (통합)
    /// 2021-12-29 11:22:33
    pub async fn H0UNMBC0(&self, req: H0Unmbc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNMBC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNMBC0", tr_id, req).await
    }

    /// 국내주식 실시간프로그램매매 (통합)
    /// 2021-12-29 11:22:33
    pub async fn H0UNPGM0(&self, req: H0Unpgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNPGM0", tr_id, req).await
    }

    /// 국내주식 장운영정보 (통합)
    /// 2021-12-29 11:22:33
    pub async fn H0UNMKO0(&self, req: H0Unmko0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0UNMKO0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0UNMKO0", tr_id, req).await
    }

    /// 국내주식 실시간체결가 (NXT)
    /// 2021-12-29 11:22:33
    pub async fn H0NXCNT0(&self, req: H0Nxcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXCNT0", tr_id, req).await
    }

    /// 국내주식 실시간호가 (NXT)
    /// 2021-12-29 11:22:33
    pub async fn H0NXASP0(&self, req: H0Nxasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXASP0", tr_id, req).await
    }

    /// 국내주식 실시간예상체결 (NXT)
    /// 2021-12-29 11:22:33
    pub async fn H0NXANC0(&self, req: H0Nxanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXANC0", tr_id, req).await
    }

    /// 국내주식 실시간회원사 (NXT)
    /// 2021-12-29 11:22:33
    pub async fn H0NXMBC0(&self, req: H0Nxmbc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXMBC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXMBC0", tr_id, req).await
    }

    /// 국내주식 실시간프로그램매매 (NXT)
    /// 국내주식 실시간프로그램매매 (NXT)
    pub async fn H0NXPGM0(&self, req: H0Nxpgm0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXPGM0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXPGM0", tr_id, req).await
    }

    /// 국내주식 장운영정보 (NXT)
    /// 2021-12-29 11:22:33
    pub async fn H0NXMKO0(&self, req: H0Nxmko0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0NXMKO0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0NXMKO0", tr_id, req).await
    }

    /// 지수선물 실시간호가
    /// 2021-12-29 11:22:33
    pub async fn H0IFASP0(&self, req: H0Ifasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IFASP0", tr_id, req).await
    }

    /// 지수선물 실시간체결가
    /// 지수선물 실시간체결가[실시간-010]
    pub async fn H0IFCNT0(&self, req: H0Ifcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IFCNT0", tr_id, req).await
    }

    /// 지수옵션 실시간호가
    /// 2021-12-29 11:22:33
    pub async fn H0IOASP0(&self, req: H0Ioasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IOASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IOASP0", tr_id, req).await
    }

    /// 지수옵션 실시간체결가
    /// 지수옵션 실시간체결가[실시간-014]
    pub async fn H0IOCNT0(&self, req: H0Iocnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IOCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0IOCNT0", tr_id, req).await
    }

    /// 선물옵션 실시간체결통보
    /// 선물옵션 실시간체결통보[실시간-012]
    pub async fn H0IFCNI0(&self, req: H0Ifcni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0IFCNI0",
            crate::client::KisEnv::Vts => "H0IFCNI9",
        };
        self.0.post("/tryitout/H0IFCNI0", tr_id, req).await
    }

    /// 상품선물 실시간호가
    /// 2021-12-29 11:22:33
    pub async fn H0CFASP0(&self, req: H0Cfasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0CFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0CFASP0", tr_id, req).await
    }

    /// 상품선물 실시간체결가
    /// 상품선물 실시간체결가[실시간-022]
    pub async fn H0CFCNT0(&self, req: H0Cfcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0CFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0CFCNT0", tr_id, req).await
    }

    /// 주식선물 실시간호가
    /// 주식선물 실시간호가 [실시간-030]
    pub async fn H0ZFASP0(&self, req: H0Zfasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZFASP0", tr_id, req).await
    }

    /// 주식선물 실시간체결가
    /// 주식선물 실시간체결가 [실시간-029]
    pub async fn H0ZFCNT0(&self, req: H0Zfcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZFCNT0", tr_id, req).await
    }

    /// 주식선물 실시간예상체결
    /// 주식선물 실시간예상체결 [실시간-031]
    pub async fn H0ZFANC0(&self, req: H0Zfanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZFANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZFANC0", tr_id, req).await
    }

    /// 주식옵션 실시간호가
    /// 주식옵션 실시간호가 [실시간-045]
    pub async fn H0ZOASP0(&self, req: H0Zoasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZOASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZOASP0", tr_id, req).await
    }

    /// 주식옵션 실시간체결가
    /// 주식옵션 실시간체결가 [실시간-044]
    pub async fn H0ZOCNT0(&self, req: H0Zocnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZOCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZOCNT0", tr_id, req).await
    }

    /// 주식옵션 실시간예상체결
    /// 주식옵션 실시간예상체결 [실시간-046]
    pub async fn H0ZOANC0(&self, req: H0Zoanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0ZOANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0ZOANC0", tr_id, req).await
    }

    /// KRX야간옵션 실시간호가
    /// KRX야간옵션 실시간호가 [실시간-033]
    pub async fn H0EUASP0(&self, req: H0Euasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EUASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUASP0", tr_id, req).await
    }

    /// KRX야간옵션 실시간체결가
    /// KRX야간옵션 실시간체결가 [실시간-032]
    pub async fn H0EUCNT0(&self, req: H0Eucnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EUCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUCNT0", tr_id, req).await
    }

    /// KRX야간옵션실시간예상체결
    /// KRX야간옵션실시간예상체결 [실시간-034]
    pub async fn H0EUANC0(&self, req: H0Euanc0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0EUANC0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUANC0", tr_id, req).await
    }

    /// KRX야간옵션실시간체결통보
    /// KRX야간옵션실시간체결통보 [실시간-067]
    pub async fn H0EUCNI0(&self, req: H0Eucni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFCNI0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0EUCNI0", tr_id, req).await
    }

    /// KRX야간선물 실시간호가
    /// KRX야간선물 실시간호가 [실시간-065]
    pub async fn H0MFASP0(&self, req: H0Mfasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0MFASP0", tr_id, req).await
    }

    /// KRX야간선물 실시간종목체결
    /// KRX야간선물 실시간종목체결 [실시간-064]
    pub async fn H0MFCNT0(&self, req: H0Mfcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0MFCNT0", tr_id, req).await
    }

    /// KRX야간선물 실시간체결통보
    /// KRX야간선물 실시간체결통보 [실시간-066]
    pub async fn H0MFCNI0(&self, req: H0Mfcni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0MFCNI0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0MFCNI0", tr_id, req).await
    }

    /// 해외주식 실시간호가
    /// 2021-12-29 11:22:33
    pub async fn HDFSASP0(&self, req: Hdfsasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFSASP0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFSASP0", tr_id, req).await
    }

    /// 해외주식 지연호가(아시아)
    /// 해외주식 지연호가(아시아)[실시간-008]
    pub async fn HDFSASP1(&self, req: Hdfsasp1Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFSASP1",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFSASP1", tr_id, req).await
    }

    /// 해외주식 실시간지연체결가
    /// 해외주식 실시간지연체결가[실시간-007]
    pub async fn HDFSCNT0(&self, req: Hdfscnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFSCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFSCNT0", tr_id, req).await
    }

    /// 해외주식 실시간체결통보
    /// 해외주식 실시간체결통보[실시간-009]
    pub async fn H0GSCNI0(&self, req: H0Gscni0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0GSCNI0",
            crate::client::KisEnv::Vts => "H0GSCNI9",
        };
        self.0.post("/tryitout/H0GSCNI0", tr_id, req).await
    }

    /// 해외선물옵션 실시간체결가
    /// 해외선물옵션 실시간체결가[실시간-017]
    pub async fn HDFFF020(&self, req: Hdfff020Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF020",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF020", tr_id, req).await
    }

    /// 해외선물옵션 실시간호가
    /// 해외선물옵션 실시간호가[실시간-018]
    pub async fn HDFFF010(&self, req: Hdfff010Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF010",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF010", tr_id, req).await
    }

    /// 해외선물옵션 실시간주문내역통보
    /// 해외선물옵션 실시간주문내역통보[실시간-019]
    pub async fn HDFFF1C0(&self, req: Hdfff1C0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF1C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF1C0", tr_id, req).await
    }

    /// 해외선물옵션 실시간체결내역통보
    /// 해외선물옵션 실시간체결내역통보[실시간-020]
    pub async fn HDFFF2C0(&self, req: Hdfff2C0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HDFFF2C0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/HDFFF2C0", tr_id, req).await
    }

    /// 일반채권 실시간체결가
    /// 일반채권 실시간체결가 [실시간-052]
    pub async fn H0BJCNT0(&self, req: H0Bjcnt0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0BJCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0BJCNT0", tr_id, req).await
    }

    /// 일반채권 실시간호가
    /// 일반채권 실시간호가 [실시간-053]
    pub async fn H0BJASP0(&self, req: H0Bjasp0Request) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "H0BJCNT0",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0.post("/tryitout/H0BJASP0", tr_id, req).await
    }

    /// 채권지수 실시간체결가
    /// 채권지수 실시간체결가 [실시간-060]
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
    /// ETF/ETN 현재가[v1_국내주식-068]
    pub async fn inquire_price(
        &self,
        req: InquirePriceNextRequest,
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
    /// ETF 구성종목시세[국내주식-073]
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
    /// NAV 비교추이(종목)[v1_국내주식-069]
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
    /// NAV 비교추이(일)[v1_국내주식-071]
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
    /// NAV 비교추이(분)[v1_국내주식-070]
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
    /// ELW 신규상장종목 [국내주식-181]
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
    /// ELW 기초자산별 종목시세 [국내주식-186]
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
    /// 2021-12-29 11:22:33
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
    /// ELW 기초자산 목록조회 [국내주식-185]
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
    /// ELW 비교대상종목조회 [국내주식-183]
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
    /// ELW LP매매추이 [국내주식-182]
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
    /// ELW 투자지표추이(체결) [국내주식-172]
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
    /// ELW 투자지표추이(분별) [국내주식-174]
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
    /// ELW 투자지표추이(일별) [국내주식-173]
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
    /// ELW 변동성 추이(틱) [국내주식-180]
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
    /// ELW 변동성추이(체결) [국내주식-177]
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
    /// ELW 변동성 추이(일별) [국내주식-178]
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
    /// ELW 민감도 추이(체결) [국내주식-175]
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
    /// ELW 변동성 추이(분별) [국내주식-179]
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
    /// ELW 민감도 추이(일별) [국내주식-176]
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
    /// ELW 만기예정/만기종목 [국내주식-184]
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
    /// 2021-12-29 11:22:33
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
    /// 선물옵션 시세[v1_국내선물-006]
    pub async fn inquire_price_next(
        &self,
        req: InquirePriceNextNextRequest,
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
    /// 선물옵션 시세호가[v1_국내선물-007]
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
    /// 선물옵션기간별시세(일/주/월/년)[v1_국내선물-008]
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
    /// 선물옵션 분봉조회[v1_국내선물-012]
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
    /// 국내옵션전광판_옵션월물리스트[국내선물-020]
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
    /// 국내선물 기초자산 시세[국내선물-021]
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
    /// 국내옵션전광판_콜풋[국내선물-022]
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
    /// 국내옵션전광판_선물[국내선물-023]
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
    /// 선물옵션 일중예상체결추이[국내선물-018]
    pub async fn exp_price_trend(
        &self,
        req: ExpPriceTrendNextRequest,
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
    /// 해외주식 현재가상세[v1_해외주식-029]
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
    /// 해외주식 현재가 호가 [해외주식-033]
    pub async fn inquire_asking_price_next(
        &self,
        req: InquireAskingPriceNextRequest,
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
    /// 해외주식 현재체결가[v1_해외주식-009]
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
    /// 2021-12-29 11:22:33
    pub async fn inquire_ccnl(
        &self,
        req: InquireCcnlNextNextNextRequest,
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
    /// 해외주식분봉조회[v1_해외주식-030]
    pub async fn inquire_time_itemchartprice(
        &self,
        req: InquireTimeItemchartpriceNextRequest,
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
    /// 해외지수분봉조회[v1_해외주식-031]
    pub async fn inquire_time_indexchartprice(
        &self,
        req: InquireTimeIndexchartpriceNextRequest,
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
    /// 해외주식 기간별시세[v1_해외주식-010]
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
    /// 해외주식 종목/지수/환율기간별시세(일/주/월/년)[v1_해외주식-012]
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
    /// 해외주식조건검색[v1_해외주식-015]
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
    /// 2021-12-29 11:22:33
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
    /// 해외주식 상품기본정보[v1_해외주식-034]
    pub async fn search_info(
        &self,
        req: SearchInfoNextRequest,
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
    /// 해외주식 업종별시세[해외주식-048]
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
    /// 해외주식 업종별코드조회[해외주식-049]
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
    /// 2021-12-29 11:22:33
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
    /// 해외주식 기간별권리조회 [해외주식-052]
    pub async fn period_rights(
        &self,
        req: PeriodRightsNextRequest,
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
    /// 해외뉴스종합(제목) [해외주식-053]
    pub async fn news_title(
        &self,
        req: NewsTitleNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "HHPSTH60100C1",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .get("/uapi/overseas-price/v1/quotations/news-title", tr_id, req)
            .await
    }

    /// 해외주식 권리종합
    /// 해외주식 권리종합 [해외주식-050]
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
    /// 당사 해외주식담보대출 가능 종목 [해외주식-051]
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
    /// 2021-12-29 11:22:33
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
    /// 해외선물종목현재가 [v1_해외선물-009]
    pub async fn inquire_price_next_next(
        &self,
        req: InquirePriceNextNextNextRequest,
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
    /// 해외선물종목상세 [v1_해외선물-008]
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
    /// 2021-12-29 11:22:33
    pub async fn inquire_asking_price_next_next(
        &self,
        req: InquireAskingPriceNextNextRequest,
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
    /// 2021-12-29 11:22:33
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
    /// 해외선물 체결추이(틱)[해외선물-019]
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
    /// 해외선물 체결추이(주간)[해외선물-017]
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
    /// 해외선물 체결추이(일간)[해외선물-018]
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
    /// 해외선물 체결추이(월간)[해외선물-020]
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
    /// 해외선물 상품기본정보 [해외선물-023]
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
    /// 해외선물 미결제추이 [해외선물-029]
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
    /// 해외옵션종목현재가 [해외선물-035]
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
    /// 2021-12-29 11:22:33
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
    /// 2021-12-29 11:22:33
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
    /// 해외옵션 분봉조회 [해외선물-040]
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
    /// 해외옵션 체결추이(틱) [해외선물-038]
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
    /// 해외옵션 체결추이(일간) [해외선물-037]
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
    /// 해외옵션 체결추이(주간) [해외선물-036]
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
    /// 해외옵션 체결추이(월간) [해외선물-039]
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
    /// 해외옵션 상품기본정보 [해외선물-041]
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
    /// 해외선물옵션 장운영시간 [해외선물-030]
    pub async fn market_time(
        &self,
        req: MarketTimeNextRequest,
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
    /// 장내채권현재가(호가) [국내주식-132]
    pub async fn inquire_asking_price_next_next_next(
        &self,
        req: InquireAskingPriceNextNextNextRequest,
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
    /// 장내채권현재가(시세) [국내주식-200]
    pub async fn inquire_price_next_next_next(
        &self,
        req: InquirePriceNextNextNextNextRequest,
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
    /// 장내채권현재가(체결) [국내주식-201]
    pub async fn inquire_ccnl_next(
        &self,
        req: InquireCcnlNextNextNextNextRequest,
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
    /// 장내채권현재가(일별) [국내주식-202]
    pub async fn inquire_daily_price(
        &self,
        req: InquireDailyPriceNextRequest,
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
    /// 장내채권 기간별시세(일) [국내주식-159]
    pub async fn inquire_daily_itemchartprice(
        &self,
        req: InquireDailyItemchartpriceNextRequest,
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
    /// 장내채권 평균단가조회 [국내주식-158]
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
    /// 2021-12-29 11:22:33
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
    /// 장내채권 기본조회 [국내주식-129]
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
    /// ELW 민감도 순위[국내주식-170]
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
    /// ELW 당일급변종목[국내주식-171]
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
    /// 2021-12-29 11:22:33
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
    /// 2021-12-29 11:22:33
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
    /// 2021-12-29 11:22:33
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
    /// 해외주식 가격급등락[해외주식-038]
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
    /// 해외주식 거래량급증[해외주식-039]
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
    /// 해외주식 매수체결강도상위[해외주식-040]
    pub async fn volume_power(
        &self,
        req: VolumePowerNextRequest,
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
    /// 해외주식 상승율/하락율[해외주식-041]
    pub async fn updown_rate_next(
        &self,
        req: UpdownRateNextRequest,
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
    /// 해외주식 신고/신저가[해외주식-042]
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
    /// 해외주식 거래량순위[해외주식-043]
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
    /// 해외주식 거래대금순위[해외주식-044]
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
    /// 해외주식 거래증가율순위[해외주식-045]
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
    /// 해외주식 거래회전율순위[해외주식-046]
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
    /// 해외주식 시가총액순위[해외주식-047]
    pub async fn market_cap(
        &self,
        req: MarketCapNextRequest,
    ) -> Result<serde_json::Value, KisError> {
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
    /// 선물옵션 주문[v1_국내선물-001]
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
    /// 선물옵션 정정취소주문[v1_국내선물-002]
    pub async fn order_rvsecncl(
        &self,
        req: OrderRvsecnclNextRequest,
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
    /// 선물옵션 주문체결내역조회[v1_국내선물-003]
    pub async fn inquire_ccnl(
        &self,
        req: InquireCcnlNextRequest,
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
    /// 선물옵션 잔고현황[v1_국내선물-004]
    pub async fn inquire_balance(
        &self,
        req: InquireBalanceNextNextRequest,
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
    /// 선물옵션 주문가능[v1_국내선물-005]
    pub async fn inquire_psbl_order(
        &self,
        req: InquirePsblOrderNextNextRequest,
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
    /// (야간)선물옵션 주문체결 내역조회 [국내선물-009]
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
    /// (야간)선물옵션 잔고현황 [국내선물-010]
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
    /// (야간)선물옵션 주문가능 조회 [국내선물-011]
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
    /// (야간)선물옵션 증거금 상세 [국내선물-024]
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
    /// 선물옵션 잔고정산손익내역[v1_국내선물-013]
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
    /// 선물옵션 총자산현황[v1_국내선물-014]
    pub async fn inquire_deposit(
        &self,
        req: InquireDepositNextRequest,
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
    /// 선물옵션 잔고평가손익내역[v1_국내선물-015]
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
    /// 선물옵션 기준일체결내역[v1_국내선물-016]
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
    /// 선물옵션기간약정수수료일별[v1_국내선물-017]
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
    /// 해외주식 주문[v1_해외주식-001]
    pub async fn order_next(&self, req: OrderNextRequest) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT1002U",
            crate::client::KisEnv::Vts => "VTTT1002U",
        };
        self.0
            .post("/uapi/overseas-stock/v1/trading/order", tr_id, req)
            .await
    }

    /// 해외주식 정정취소주문
    /// 해외주식 정정취소주문[v1_해외주식-003]
    pub async fn order_rvsecncl_next(
        &self,
        req: OrderRvsecnclNextNextRequest,
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
    /// 해외주식 예약주문접수[v1_해외주식-002]
    pub async fn order_resv(
        &self,
        req: OrderResvNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "TTTT3014U",
            crate::client::KisEnv::Vts => "VTTT3014U",
        };
        self.0
            .post("/uapi/overseas-stock/v1/trading/order-resv", tr_id, req)
            .await
    }

    /// 해외주식 예약주문접수취소
    /// 해외주식 예약주문접수취소[v1_해외주식-004]
    pub async fn order_resv_ccnl(
        &self,
        req: OrderResvCcnlNextRequest,
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
    /// 해외주식 매수가능금액조회[v1_해외주식-014]
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
    /// 해외주식 미체결내역[v1_해외주식-005]
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
    /// 해외주식 잔고[v1_해외주식-006]
    pub async fn inquire_balance_next(
        &self,
        req: InquireBalanceNextNextNextRequest,
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
    /// 해외주식 주문체결내역[v1_해외주식-007]
    pub async fn inquire_ccnl_next(
        &self,
        req: InquireCcnlNextNextRequest,
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
    /// 해외주식 체결기준현재잔고[v1_해외주식-008]
    pub async fn inquire_present_balance(
        &self,
        req: InquirePresentBalanceNextRequest,
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
    /// 해외주식 예약주문조회[v1_해외주식-013]
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
    /// 해외주식 결제기준잔고 [해외주식-064]
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
    /// 해외주식 일별거래내역 [해외주식-063]
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
    /// 해외주식 기간손익[v1_해외주식-032]
    pub async fn inquire_period_profit(
        &self,
        req: InquirePeriodProfitNextRequest,
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
    /// 해외증거금 통화별조회 [해외주식-035]
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
    /// 해외주식 미국주간주문[v1_해외주식-026]
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
    /// 해외주식 미국주간정정취소[v1_해외주식-027]
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
    /// 해외주식 지정가주문번호조회 [해외주식-071]
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
    /// 해외주식 지정가체결내역조회 [해외주식-070]
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
    /// 해외선물옵션 주문 [v1_해외선물-001]
    pub async fn order_next_next(
        &self,
        req: OrderNextNextRequest,
    ) -> Result<serde_json::Value, KisError> {
        let tr_id = match self.0.env() {
            crate::client::KisEnv::Real => "OTFM3001U",
            crate::client::KisEnv::Vts => "모의투자 미지원",
        };
        self.0
            .post("/uapi/overseas-futureoption/v1/trading/order", tr_id, req)
            .await
    }

    /// 해외선물옵션 정정취소주문
    /// 해외선물옵션 정정취소주문 [v1_해외선물-002, 003]
    pub async fn order_rvsecncl_next_next(
        &self,
        req: OrderRvsecnclNextNextNextRequest,
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
    /// 해외선물옵션 당일주문내역조회 [v1_해외선물-004]
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
    /// 해외선물옵션 미결제내역조회(잔고) [v1_해외선물-005]
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
    /// 해외선물옵션 주문가능조회 [v1_해외선물-006]
    pub async fn inquire_psamount_next(
        &self,
        req: InquirePsamountNextRequest,
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
    /// 해외선물옵션 기간계좌손익 일별[해외선물-010]
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
    /// 해외선물옵션 일별 체결내역[해외선물-011]
    pub async fn inquire_daily_ccld(
        &self,
        req: InquireDailyCcldNextNextRequest,
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
    /// 해외선물옵션 예수금현황[해외선물-012]
    pub async fn inquire_deposit_next(
        &self,
        req: InquireDepositNextNextRequest,
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
    /// 해외선물옵션 일별 주문내역[해외선물-013]
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
    /// 해외선물옵션 기간계좌거래내역[해외선물-014]
    pub async fn inquire_period_trans_next(
        &self,
        req: InquirePeriodTransNextRequest,
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
    /// 해외선물옵션 증거금상세 [해외선물-032]
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
    /// 장내채권 매수주문 [국내주식-124]
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
    /// 장내채권 매도주문 [국내주식-123]
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
    /// 장내채권 정정취소주문 [국내주식-125]
    pub async fn order_rvsecncl_next_next_next(
        &self,
        req: OrderRvsecnclNextNextNextNextRequest,
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
    /// 채권정정취소가능주문조회 [국내주식-126]
    pub async fn inquire_psbl_rvsecncl(
        &self,
        req: InquirePsblRvsecnclNextRequest,
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
    /// 장내채권 주문체결내역 [국내주식-127]
    pub async fn inquire_daily_ccld_next(
        &self,
        req: InquireDailyCcldNextNextNextRequest,
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
    /// 장내채권 잔고조회 [국내주식-198]
    pub async fn inquire_balance_next_next(
        &self,
        req: InquireBalanceNextNextNextNextRequest,
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
    /// 장내채권 매수가능조회 [국내주식-199]
    pub async fn inquire_psbl_order_next(
        &self,
        req: InquirePsblOrderNextNextNextRequest,
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
