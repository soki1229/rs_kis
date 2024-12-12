use ::http::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use crate::api::oauth_certification::TokenInfo;
use crate::environment;
use crate::api::{Config, http};
use crate::error::RestfulError as Error;
use log::debug;

pub async fn check_deposit(client: &Client, config: &Config, token_info: &TokenInfo, account_num: &String) -> Result<Response, Error> {
    let env = environment::get();

    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("authorization"), HeaderValue::from_str(&format!("Bearer {}", token_info.get_token())).unwrap());
    headers.insert(HeaderName::from_static("appkey"), HeaderValue::from_static(&env.app_key));
    headers.insert(HeaderName::from_static("appsecret"), HeaderValue::from_static(&env.app_secret));
    headers.insert(HeaderName::from_static("tr_id"), HeaderValue::from_static(if cfg!(feature = "vts_mock_disabled"){"TTTS3012R"}else{"VTTS3012R"}));
    headers.insert(HeaderName::from_static("tr_cont"), HeaderValue::from_static(""));

    debug!("account_num: {}", account_num);
    let (a_num, a_type) = account_num.split_once('-').unwrap();
    let query_data = vec![
        ("CANO", a_num),
        ("ACNT_PRDT_CD", a_type),
        ("OVRS_EXCG_CD", "NAS"),
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
    ).await?;

    Ok(response)
}