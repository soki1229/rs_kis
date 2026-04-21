#![allow(clippy::doc_lazy_continuation)]
use serde::{Deserialize, Serialize};

/// [접근토큰발급(P)[인증-001]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOauth2TokenPRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [접근토큰폐기(P)[인증-002]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOauth2RevokePRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}

/// [실시간 (웹소켓) 접속키 발급[실시간-000]] 요청 구조체

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct AuthOauth2ApprovalRequest {
    /// headerMap (String, 선택)
    #[serde(rename = "headerMap")]
    pub header_map: String,
    /// methodList (String, 선택)
    #[serde(rename = "methodList")]
    pub method_list: String,
    /// contentTypeList (String, 선택)
    #[serde(rename = "contentTypeList")]
    pub content_type_list: String,
    /// pathList (String, 선택)
    #[serde(rename = "pathList")]
    pub path_list: String,
    /// queryMap (String, 선택)
    #[serde(rename = "queryMap")]
    pub query_map: String,
    /// formMap (String, 선택)
    #[serde(rename = "formMap")]
    pub form_map: String,
    /// jsonBody (String, 선택)
    #[serde(rename = "jsonBody")]
    pub json_body: String,
    /// jsonResponse (String, 선택)
    #[serde(rename = "jsonResponse")]
    pub json_response: String,
    /// address (String, 선택)
    #[serde(rename = "address")]
    pub address: String,
}
