use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as WsError;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("WebSocket connection error: {0}")]
    ConnectionError(#[from] WsError),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Error sending message: {0}")]
    SendError(String),
    #[error("RestfulError: {0}")]
    RestfulError(#[from] RestfulError),
}

#[derive(Error, Debug)]
pub enum RestfulError {
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("HTTP error: {status}, body: {body}")]
    HttpError { status: reqwest::StatusCode, body: String },
    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
