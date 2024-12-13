use crate::error::KisClientError as Error;
use log::{debug, error};
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde_json::Value;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct Config {
    domain: String,
    default_timeout: Duration,
    max_retries: u32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            domain: String::from("https://openapi.koreainvestment.com:9443"),
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

pub async fn execute_api_call(
    client: &Client,
    config: &Config,
    path: &str,
    method: Method,
    headers: Option<HeaderMap>,
    body_data: Option<Value>,
    query_data: Option<Vec<(&str, &str)>>,
) -> Result<reqwest::Response, Error> {
    let url = Url::parse(&config.domain)?.join(path)?;

    for attempt in 1..=config.max_retries {
        let mut request = client
            .request(method.clone(), url.clone())
            .timeout(config.default_timeout);

        if let Some(header_content) = headers.clone() {
            request = request.headers(header_content);
        }
        if let Some(body_content) = body_data.clone() {
            request = request.json(&body_content);
        }
        if let Some(query_content) = query_data.clone() {
            request = request.query(&query_content);
        }

        debug!("Sending request to {} (attempt {})", url, attempt);
        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response);
                } else {
                    let status = response.status();
                    let body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unable to read response body".to_string());
                    error!("HTTP error: {}, body: {}", status, body);
                    return Err(Error::HttpError { status, body });
                }
            }
            Err(e) if attempt <= config.max_retries => {
                error!("Request failed (attempt {}): {}", attempt, e);
                tokio::time::sleep(Duration::from_secs(2u64.pow(attempt))).await;
            }
            Err(e) => return Err(Error::RequestError(e)),
        }
    }

    Err(Error::MaxRetriesExceeded)
}
