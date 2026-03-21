use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub header: Header,
    pub body: Body,
}

#[derive(Serialize)]
pub struct Header {
    pub approval_key: String,
    pub custtype: String,
    pub tr_type: String,
    #[serde(rename = "content-type")]
    pub content_type: String,
}

#[derive(Serialize)]
pub struct Body {
    pub input: Input,
}

#[derive(Serialize)]
pub struct Input {
    pub tr_id: String,
    pub tr_key: String,
}