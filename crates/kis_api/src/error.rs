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
    #[error("HTTP {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Auth Error: {0}")]
    Auth(String),
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    #[error("Stream lagged by {0} messages")]
    Lagged(u64),
    #[error("WebSocket stream closed")]
    StreamClosed,
    #[error("이 API는 모의투자(VTS) 환경에서 지원되지 않습니다.")]
    NotSupportedInVts,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Option<T>,
}

impl<T> ApiResponse<T> {
    pub async fn from_response(resp: reqwest::Response) -> Result<Self, KisError>
    where
        T: for<'de> serde::Deserialize<'de> + Default,
    {
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();
            return Err(KisError::Http { status, message });
        }

        let body_text = resp.text().await?;
        serde_json::from_str(&body_text).map_err(|e| {
            KisError::Auth(format!(
                "Failed to parse API response: {}. Body: {}",
                e, body_text
            ))
        })
    }

    pub fn into_result(self) -> Result<T, KisError>
    where
        T: Default + serde::de::DeserializeOwned,
    {
        if self.rt_cd != "0" && self.rt_cd != "00" {
            return Err(KisError::Api {
                rt_cd: self.rt_cd,
                msg_cd: self.msg_cd,
                message: self.msg1,
            });
        }
        match self.output {
            Some(out) => Ok(out),
            None => {
                // rt_cd가 0인데 output이 없는 경우 (예: 주문 취소 성공 등)
                // T의 기본값을 반환하거나 처리 로직이 필요함.
                // 여기서는 안전하게 Deserialize를 시도하거나 Default를 반환.
                Ok(T::default())
            }
        }
    }
}
