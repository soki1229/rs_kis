use crate::configurations::Configurations;
use crate::core::http;
use crate::credentials::{CredentialProvider, Credentials};
use ::http::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};

use crate::error::KisClientError as Error;

pub async fn current_transaction_price(
    client: &Client,
    config: &Configurations,
    credential: &Credentials,
    symbol: &str,
) -> Result<Response, Error> {
    let token = credential.token().to_string();
    let app_key = credential.app_key().to_string();
    let app_secret = credential.app_secret().to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    headers.insert(
        HeaderName::from_static("appkey"),
        HeaderValue::from_str(&app_key).unwrap(),
    );
    headers.insert(
        HeaderName::from_static("appsecret"),
        HeaderValue::from_str(&app_secret).unwrap(),
    );
    headers.insert(
        HeaderName::from_static("custtype"),
        HeaderValue::from_static("P"),
    );
    headers.insert(
        HeaderName::from_static("tr_id"),
        HeaderValue::from_static("HHDFS76200200"),
    );

    let query_data = vec![("AUTH", ""), ("EXCD", "NAS"), ("SYMB", symbol)];

    let response = http::execute_api_call(
        client,
        config,
        "/uapi/overseas-price/v1/quotations/price-detail",
        Method::GET,
        Some(headers),
        None,
        Some(query_data),
    )
    .await?;

    Ok(response)
}
