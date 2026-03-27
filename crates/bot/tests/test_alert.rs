use kis_bot::monitoring::alert::{AlertRouter, AlertSeverity};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn critical_event_received_by_subscriber() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.critical("Kill switch triggered".to_string());

    let event = timeout(Duration::from_millis(100), rx.recv())
        .await
        .expect("should receive")
        .expect("channel should not close");

    assert_eq!(event.severity, AlertSeverity::Critical);
    assert!(event.message.contains("Kill switch"));
}

#[tokio::test]
async fn warn_event_received() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.warn("MDD reached -10%".to_string());

    let event = rx.recv().await.unwrap();
    assert_eq!(event.severity, AlertSeverity::Warn);
}

#[tokio::test]
async fn info_event_received() {
    let router = AlertRouter::new(64);
    let mut rx = router.subscribe();

    router.info("LLM underperforming".to_string());

    let event = rx.recv().await.unwrap();
    assert_eq!(event.severity, AlertSeverity::Info);
}

#[tokio::test]
async fn multiple_subscribers_all_receive() {
    let router = AlertRouter::new(64);
    let mut rx1 = router.subscribe();
    let mut rx2 = router.subscribe();

    router.critical("test".to_string());

    let e1 = rx1.recv().await.unwrap();
    let e2 = rx2.recv().await.unwrap();
    assert_eq!(e1.severity, AlertSeverity::Critical);
    assert_eq!(e2.severity, AlertSeverity::Critical);
}
