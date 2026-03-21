use thiserror::Error;

#[derive(Error, Debug)]
pub enum KisError {
    #[error("auth error: {0}")]
    Auth(String),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("KIS API error {code}: {message}")]
    Api { code: String, message: String },

    #[error("websocket error: {0}")]
    WebSocket(String),

    #[error("event stream lagged: {0} events missed")]
    Lagged(u64),

    #[error("event stream closed")]
    StreamClosed,

    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl KisError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Network(_) | Self::WebSocket(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retryable_errors() {
        assert!(KisError::WebSocket("timeout".into()).is_retryable());
        assert!(!KisError::Auth("invalid key".into()).is_retryable());
        assert!(!KisError::StreamClosed.is_retryable());
        assert!(!KisError::Lagged(5).is_retryable());
    }

    #[test]
    fn error_display_messages() {
        assert_eq!(
            KisError::Lagged(42).to_string(),
            "event stream lagged: 42 events missed"
        );
        assert_eq!(KisError::StreamClosed.to_string(), "event stream closed");
        assert_eq!(
            KisError::Api { code: "EGW00123".into(), message: "Unauthorized".into() }.to_string(),
            "KIS API error EGW00123: Unauthorized"
        );
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn kis_error_is_send_sync() {
        assert_send_sync::<KisError>();
    }
}
