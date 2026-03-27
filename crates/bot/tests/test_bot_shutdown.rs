use kis_bot::bot::BotHandle;
use tokio_util::sync::CancellationToken;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn cancellation_token_stops_runner() {
    let token = CancellationToken::new();
    let handle = BotHandle::new(token.clone());

    token.cancel();
    let result = timeout(Duration::from_secs(1), handle.wait()).await;
    assert!(result.is_ok(), "shutdown should complete within 1 second");
}
