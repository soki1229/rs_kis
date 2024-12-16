use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KisClientError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Error sending message: {0}")]
    SendError(String),
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("HTTP error: {status}, body: {body}")]
    HttpError { status: StatusCode, body: String },
    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse date: {0}")]
    DateParseError(#[from] chrono::ParseError),
}

impl From<String> for KisClientError {
    fn from(msg: String) -> Self {
        Self::SendError(msg)
    }
}
