use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Response {
    #[serde(default)]
    pub header: Header,
    #[serde(default)]
    pub body:   Body,
}

#[derive(Default, Deserialize)]
pub struct Header {
    #[serde(default)]
    pub tr_id:      String,
    #[serde(default)]
    pub datetime:   String,
}

#[derive(Default, Deserialize)]
pub struct Body {
    #[serde(default)]
    pub rt_cd:  String,
    #[serde(default)]
    pub msg_cd: String,
    #[serde(default)]
    pub msg1:   String,
    #[serde(default)]
    pub output: Output,
}

#[derive(Default, Deserialize)]
pub struct Output {
    #[serde(default)]
    pub iv:     String,
    #[serde(default)]
    pub key:    String,
}

impl TryFrom<&str> for Response {
    type Error = serde_json::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}

#[derive(Debug)]
pub enum TransactionId {
    // Session Liveness
    PINGPONG,
    // 실시가 체결가(해외)
    HDFSCNT0(String),
    // 실시간 호가(미국)
    HDFSASP0(String),
    // 실시간 호가(아시아)
    HDFSASP1(String),
    // 실시간 체결 통보(해외)
    H0GSCNI0(String),
    // 미지정
    UnKnown,
}

impl FromStr for TransactionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "PINGPONG" => TransactionId::PINGPONG,
            "HDFSCNT0" => TransactionId::HDFSCNT0("실시가 체결가(해외)".to_string()),
            "HDFSASP0" => TransactionId::HDFSASP0("실시간 호가(미국)".to_string()),
            "HDFSASP1" => TransactionId::HDFSASP1("실시간 호가(아시아)".to_string()),
            "H0GSCNI0" => TransactionId::H0GSCNI0("실시간 체결 통보(해외)".to_string()),
            _ => TransactionId::UnKnown,
        })
    }
}