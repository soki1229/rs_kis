use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    access_token: String,
    access_token_token_expired: String,
    token_type: String,
    expires_in: i64,
}

impl TokenInfo {
    pub fn new() -> Self {
        TokenInfo {
            access_token: String::new(),
            access_token_token_expired: current_time().format("%Y-%m-%d %H:%M:%S").to_string(),
            token_type: String::new(),
            expires_in: 0,
        }
    }

    pub fn get_token(&self) -> &String {
        &self.access_token
    }

    pub fn get_expires_in(&self) -> String {
        let hours = self.expires_in / 3600;
        let minutes = (self.expires_in % 3600) / 60;
        let seconds = self.expires_in % 60;

        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }

    pub fn is_expired(&self) -> bool {
        // Check if the token has already expired based on `expires_in`
        if self.expires_in == 0 {
            return true;
        }

        // Attempt to parse the expiration time
        let naive_dt: NaiveDateTime = match NaiveDateTime::parse_from_str(
            &self.access_token_token_expired,
            "%Y-%m-%d %H:%M:%S",
        ) {
            Ok(dt) => dt,
            Err(_) => {
                error!(
                    "Failed to parse access_token_token_expired: {}",
                    self.access_token_token_expired
                );
                return true; // Assume expired if parsing fails
            }
        };

        // Convert NaiveDateTime to DateTime<Utc>
        let expiry_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);

        // let remains = expiry_time - current_time();
        // self.expires_in = remains.num_seconds();

        // Compare the current time with the expiration time
        current_time() >= expiry_time
    }

    pub fn update(&mut self) {
        // Attempt to parse the expiration time
        let naive_dt =
            NaiveDateTime::parse_from_str(&self.access_token_token_expired, "%Y-%m-%d %H:%M:%S")
                .expect("Failed to parse access_token_token_expired");

        // Convert NaiveDateTime to DateTime<Utc>
        let expiry_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);
        let remains = expiry_time - current_time();

        self.expires_in = remains.num_seconds();
        info!("access_token expires in {}", &self.get_expires_in());
    }
}

pub fn current_time() -> DateTime<Utc> {
    Utc::now() + Duration::hours(9)
}
