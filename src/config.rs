use dotenv::dotenv;
use std::env;
use std::sync::Once;

pub struct Config {
    pub app_key: String,
    pub app_secret: String,
}

// 전역 변수 초기화를 위한 Once 객체
static mut INSTANCE: Option<Config> = None;
static INIT: Once = Once::new();

pub fn init() {
    // 한번만 초기화
    INIT.call_once(|| {
        dotenv().ok(); // .env 파일 로드

        // 안전한 블록에서 INSTANCE 초기화
        unsafe {
            INSTANCE = Some(Config {
                app_key: env::var("VTS_APP_KEY").expect("APP_KEY must be set"),
                app_secret: env::var("VTS_APP_SECRET").expect("APP_SECRET must be set"),
            });
        }
    });
}

pub fn get() -> &'static Config {
    unsafe {
        INSTANCE.as_ref().expect("Config not initialized")
    }
}