use tokio_util::sync::CancellationToken;

/// BotRunner의 외부 제어 핸들.
pub struct BotHandle {
    token: CancellationToken,
    _task: tokio::task::JoinHandle<()>,
}

impl BotHandle {
    pub fn new(token: CancellationToken) -> Self {
        let t = token.clone();
        let task = tokio::spawn(async move {
            t.cancelled().await;
            tracing::info!("Bot shutdown signal received");
        });
        Self { token, _task: task }
    }

    pub async fn wait(self) {
        let _ = self._task.await;
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }
}
