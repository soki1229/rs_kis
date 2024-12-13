pub mod request;
pub mod response;

use std::str::FromStr;

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
