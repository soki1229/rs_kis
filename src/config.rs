// use dotenv::dotenv;
use dotenv::from_filename;
use std::env;
use std::sync::Once;

// 전역 변수 초기화를 위한 Once 객체
static mut INSTANCE: Option<Config> = None;
static INIT: Once = Once::new();

pub struct Config {
    pub app_key:    String,
    pub app_secret: String,
    pub ws_url:     String,
    pub rest_url:   String,
}

pub fn init() {
    // 한번만 초기화
    INIT.call_once(|| {
        // dotenv().ok();
        from_filename(".env.test").ok();
        
        // 안전한 블록에서 INSTANCE 초기화
        unsafe {
            INSTANCE = Some(Config {
                app_key:    env::var("APP_KEY").expect("APP_KEY must be set"),
                app_secret: env::var("APP_SECRET").expect("APP_SECRET must be set"),
                ws_url:     env::var("WS_URL").expect("WS_URL must be set"),
                rest_url:   env::var("REST_URL").expect("REST_URL must be set"),
            });
        }
    });
}

pub fn get() -> &'static Config {
    unsafe {
        INSTANCE.as_ref().expect("Config not initialized")
    }
}