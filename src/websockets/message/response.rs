use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    #[serde(default)]
    pub header: Header,
    #[serde(default)]
    pub body: Body,
}

#[derive(Default, Deserialize)]
pub struct Header {
    #[serde(default)]
    pub tr_id: String,
    #[serde(default)]
    pub tr_key: String,
    #[serde(default)]
    pub encrypt: String,
    #[serde(default)]
    pub datetime: String,
}

#[derive(Default, Deserialize)]
pub struct Body {
    #[serde(default)]
    pub rt_cd: String,
    #[serde(default)]
    pub msg_cd: String,
    #[serde(default)]
    pub msg1: String,
    #[serde(default)]
    pub output: Output,
}

#[derive(Default, Deserialize)]
pub struct Output {
    #[serde(default)]
    pub iv: String,
    #[serde(default)]
    pub key: String,
}

impl TryFrom<&str> for Message {
    type Error = serde_json::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}