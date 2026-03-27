use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::error::BotError;
use crate::types::LlmVerdict;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmResponse {
    pub verdict: LlmVerdict,
    pub bull: String,
    pub bear: String,
    pub neutral: String,
    pub block_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LlmInput {
    pub symbol: String,
    pub price: f64,
    pub change_pct: f64,
    pub volume_ratio: f64,
    pub ma_state: String,
    pub setup_score: i32,
    pub news_headlines: Vec<String>,
}

pub fn parse_llm_response(json: &str) -> Result<LlmResponse, BotError> {
    serde_json::from_str(json).map_err(BotError::Json)
}

pub struct LlmEngine {
    client: Client,
    api_key: String,
    model: String,
    timeout: Duration,
}

impl LlmEngine {
    pub fn new(api_key: String, model: String, timeout_secs: u64) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub async fn evaluate(&self, input: &LlmInput) -> Result<LlmResponse, BotError> {
        let news_text = input.news_headlines
            .iter()
            .enumerate()
            .map(|(i, h)| format!("  {}. {}", i + 1, h))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "종목: {} | 현재가: {:.2} | 등락률: {:.2}%\n\
             거래량 비율: {:.1}x | MA 상태: {} | Setup Score: {}/100\n\
             최근 뉴스 (24h):\n{}\n\n\
             세 관점에서 각각 판단하라:\n\
             - Bull: 매수 근거 (1~2문장)\n\
             - Bear: 위험 요인 (1~2문장)\n\
             - Neutral: 중립 평가 (1문장)\n\n\
             JSON으로만 응답하라. 형식:\n\
             {{\"verdict\":\"ENTER\"|\"WATCH\"|\"BLOCK\",\"bull\":\"...\",\"bear\":\"...\",\"neutral\":\"...\",\"block_reason\":null|\"...\"}}\n\
             BLOCK인 경우에만 block_reason을 채워라.",
            input.symbol, input.price, input.change_pct,
            input.volume_ratio, input.ma_state, input.setup_score,
            news_text
        );

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 300,
            "messages": [{"role": "user", "content": prompt}]
        });

        let resp = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .timeout(self.timeout)
            .json(&body)
            .send()
            .await
            .map_err(BotError::Http)?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(BotError::Llm(format!("Anthropic API error {}: {}", status, text)));
        }

        let raw: serde_json::Value = resp.json().await.map_err(BotError::Http)?;
        let content = raw["content"][0]["text"]
            .as_str()
            .ok_or_else(|| BotError::Llm("no content in response".to_string()))?;

        parse_llm_response(content)
    }
}
