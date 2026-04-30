use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TokenRequest {
    pub grant_type: String,
    pub appkey: String,
    pub appsecret: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub access_token_token_expired: String,
    pub token_type: String,
    pub expires_in: u32,
}
