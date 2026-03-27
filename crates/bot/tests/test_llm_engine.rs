use kis_bot::signal::llm_engine::{LlmResponse, parse_llm_response};
use kis_bot::types::LlmVerdict;

#[test]
fn parse_enter_verdict() {
    let json = r#"{"verdict":"Enter","bull":"strong momentum","bear":"overbought","neutral":"ok","block_reason":null}"#;
    let resp: LlmResponse = parse_llm_response(json).unwrap();
    assert_eq!(resp.verdict, LlmVerdict::Enter);
    assert!(resp.block_reason.is_none());
}

#[test]
fn parse_block_verdict_with_reason() {
    let json = r#"{"verdict":"Block","bull":"","bear":"FDA rejection news","neutral":"","block_reason":"negative catalyst"}"#;
    let resp: LlmResponse = parse_llm_response(json).unwrap();
    assert_eq!(resp.verdict, LlmVerdict::Block);
    assert_eq!(resp.block_reason.as_deref(), Some("negative catalyst"));
}

#[test]
fn parse_watch_verdict() {
    let json = r#"{"verdict":"Watch","bull":"","bear":"","neutral":"mixed","block_reason":null}"#;
    let resp: LlmResponse = parse_llm_response(json).unwrap();
    assert_eq!(resp.verdict, LlmVerdict::Watch);
}

#[test]
fn invalid_json_returns_error() {
    let result = parse_llm_response("not json");
    assert!(result.is_err());
}
