use kis_bot::error::BotError;

#[test]
fn bot_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BotError>();
}

#[test]
fn bot_error_display() {
    let e = BotError::Config("missing key".to_string());
    assert!(e.to_string().contains("missing key"));
}
