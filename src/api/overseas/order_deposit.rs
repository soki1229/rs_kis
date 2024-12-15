use crate::configurations::Configurations;
use crate::core::http;
use crate::credentials::{CredentialProvider, Credentials};
use crate::error::KisClientError as Error;
use ::http::header::{HeaderMap, HeaderName, HeaderValue};
use log::debug;
use log::info;
use reqwest::{Client, Method, Response};

pub async fn check_deposit(
    client: &Client,
    config: &Configurations,
    credential: &Credentials,
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
        HeaderName::from_static("tr_id"),
        HeaderValue::from_static("TTTS3012R"),
        // HeaderValue::from_static("VTTS3012R"),
    );
    headers.insert(
        HeaderName::from_static("tr_cont"),
        HeaderValue::from_static(""),
    );

    let (a_num, a_type) = credential.account_num().split_once('-').unwrap();
    let query_data = vec![
        ("CANO", a_num),
        ("ACNT_PRDT_CD", a_type),
        ("OVRS_EXCG_CD", "NASD"),
        ("TR_CRCY_CD", "USD"),
        ("CTX_AREA_FK200", ""),
        ("CTX_AREA_NK200", ""),
    ];

    let response = http::execute_api_call(
        client,
        config,
        "/uapi/overseas-stock/v1/trading/inquire-balance",
        Method::GET,
        Some(headers),
        None,
        Some(query_data),
    )
    .await?;

    Ok(response)
}
