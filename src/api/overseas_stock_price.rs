use ::http::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use crate::environment;
use crate::api::{Config, http};
use crate::error::RestfulError as Error;

pub async fn current_transaction_price(client: &Client, config: &Config, access_token: &str) -> Result<Response, Error> {
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
        ("SYMB", "TSLA"),
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
    // let response_data = response.text().await?;

    // // let approval_response: Response = response.json().await?;
    
    // if response_data.is_empty() {
    //     error!("Received empty approval key");
    //     Err(Error::MissingApprovalKey)
    // } else {
    //     info!("response_data: [{}]", response_data);
    //     Ok(response_data)
    // }
}