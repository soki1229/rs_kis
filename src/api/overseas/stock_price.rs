use ::http::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use crate::environment;
use crate::api::{Config, http};
use crate::error::KisClientError as Error;

pub async fn current_transaction_price(client: &Client, config: &Config, access_token: &str, symbol: &str) -> Result<Response, Error> {
    let env = environment::get();
    
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("authorization"), HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap());
    headers.insert(HeaderName::from_static("appkey"), HeaderValue::from_static(&env.app_key));
    headers.insert(HeaderName::from_static("appsecret"), HeaderValue::from_static(&env.app_secret));
    headers.insert(HeaderName::from_static("custtype"), HeaderValue::from_static("P"));
    headers.insert(HeaderName::from_static("tr_id"), HeaderValue::from_static("HHDFS76200200"));

    let query_data = vec![
        ("AUTH", ""),
        ("EXCD", "NAS"),
        ("SYMB", symbol),
    ];

    let response = http::execute_api_call(
        client,
        config,
        "/uapi/overseas-price/v1/quotations/price-detail",
        Method::GET,
        Some(headers),
        None,
        Some(query_data),
    ).await?;

    Ok(response)
}