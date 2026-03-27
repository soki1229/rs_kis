use tokio::sync::broadcast;

/// 알림 심각도. 스펙 Section 15 채널 정책:
/// - Critical: Telegram 즉시 + 로그 (trading-server 담당)
/// - Warn: 로그 + Telegram (trading-server 담당)
/// - Info: 로그만 (Telegram 미발송)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertSeverity {
    Critical,
    Warn,
    Info,
}

#[derive(Debug, Clone)]
pub struct AlertMessage {
    pub severity: AlertSeverity,
    pub message: String,
}

/// trading-bot이 발행하는 알림 채널.
/// trading-server가 구독하여 심각도에 따라 Telegram 발송.
/// trading-bot은 발송 성공 여부와 무관하게 채널에 넣고 계속 실행.
#[derive(Clone)]
pub struct AlertRouter {
    sender: broadcast::Sender<AlertMessage>,
}

impl AlertRouter {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AlertMessage> {
        self.sender.subscribe()
    }

    pub fn critical(&self, message: String) {
        self.send(AlertSeverity::Critical, message);
    }

    pub fn warn(&self, message: String) {
        self.send(AlertSeverity::Warn, message);
    }

    pub fn info(&self, message: String) {
        self.send(AlertSeverity::Info, message);
    }

    fn send(&self, severity: AlertSeverity, message: String) {
        // 구독자 없어도 에러 무시 (로그만)
        let _ = self.sender.send(AlertMessage { severity, message });
    }
}
