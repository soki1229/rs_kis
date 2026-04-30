use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KisError {
    #[error("KIS API 에러: {rt_cd}/{msg_cd} - {message}")]
    Api {
        rt_cd: String,
        msg_cd: String,
        message: String,
    },
    #[error("인증 에러: {0}")]
    Auth(String),
    #[error("네트워크 에러: {0}")]
    Network(#[from] reqwest::Error),
    #[error("데이터 파싱 에러: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("WebSocket 에러: {0}")]
    WebSocket(String),
    #[error("Stream lagged by {0} messages")]
    Lagged(u64),
    #[error("WebSocket stream closed")]
    StreamClosed,
    #[error("이 API는 모의투자(VTS) 환경에서 지원되지 않습니다.")]
    NotSupportedInVts,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponseHeader {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
}

impl ApiResponseHeader {
    pub fn is_success(&self) -> bool {
        self.rt_cd == "0" || self.rt_cd == "7" // 7 is also success in some KIS APIs
    }

    pub fn to_error(&self) -> KisError {
        KisError::Api {
            rt_cd: self.rt_cd.clone(),
            msg_cd: self.msg_cd.clone(),
            message: self.msg1.clone(),
        }
    }
}
