use dotenv::dotenv;
use std::collections::HashMap;

#[tokio::main]
async fn query(_app_key : &String, _app_secret : &String) {
    let url = "https://openapivts.koreainvestment.com:29443/oauth2/Approval";

    let mut body = HashMap::new();
    body.insert("grant_type", "client_credentials");
    body.insert("appkey", _app_key);
    body.insert("secretkey", _app_secret);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&body)
        .send()
        .await;

    let response_body = response.unwrap().text().await;
    println!("{:?}", response_body);
}

fn main() {
    dotenv().ok();

    let app_key = std::env::var("VTS_APP_KEY").unwrap();
    let app_secret = std::env::var("VTS_APP_SECRET").unwrap();

    query(&app_key, &app_secret);
}