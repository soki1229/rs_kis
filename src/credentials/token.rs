use crate::error::KisClientError;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};

pub trait TokenProvider {
    fn access_token(&self) -> &str;
    fn expiration_date(&self) -> &str;
    fn token_type(&self) -> &str;
    fn longevity(&self) -> &i64;
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    access_token: String,
    access_token_token_expired: String,
    token_type: String,
    expires_in: i64,
}

impl TokenProvider for AccessToken {
    fn access_token(&self) -> &str {
        &self.access_token
    }

    fn expiration_date(&self) -> &str {
        &self.access_token_token_expired
    }

    fn token_type(&self) -> &str {
        &self.token_type
    }

    fn longevity(&self) -> &i64 {
        &self.expires_in
    }
}

impl AccessToken {
    pub fn new() -> Self {
        AccessToken {
            access_token: String::new(),
            access_token_token_expired: current_time().format("%Y-%m-%d %H:%M:%S").to_string(),
            token_type: String::new(),
            expires_in: 0,
        }
    }

    pub fn get_expires_in(&self) -> String {
        let longevity = Duration::seconds(self.expires_in);
        format!(
            "{:02}:{:02}:{:02}",
            longevity.num_hours(),
            longevity.num_minutes() % 60,
            longevity.num_seconds() % 60
        )
    }

    pub fn is_expired(&self) -> Result<bool, KisClientError> {
        if self.expires_in == 0 {
            return Ok(true);
        }

        let naive_dt =
            NaiveDateTime::parse_from_str(&self.access_token_token_expired, "%Y-%m-%d %H:%M:%S")?;
        let expiry_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);

        Ok(current_time() >= expiry_time)
    }

    pub fn update(&mut self) -> Result<(), KisClientError> {
        let longevity = Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(
            &self.access_token_token_expired,
            "%Y-%m-%d %H:%M:%S",
        )?) - current_time();

        self.expires_in = longevity.num_seconds();
        info!("access_token expires in {}", self.get_expires_in());
        Ok(())
    }
}

pub fn current_time() -> DateTime<Utc> {
    Utc::now() + Duration::hours(9)
}
