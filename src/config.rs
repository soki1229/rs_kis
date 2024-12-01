use std::env;
use std::sync::Once;
use log::{warn, error};

const SERVICE_TYPE_KIS: &str = "KIS";
const SERVICE_TYPE_VTS: &str = "VTS";

// Once instance for initializing gloabl variable
static mut INSTANCE: Option<Config> = None;
static INIT: Once = Once::new();

pub struct Config {
    pub app_key:    String,
    pub app_secret: String,
    pub ws_url:     String,
    pub rest_url:   String,
}

pub fn init() {
    // Ensure the initialize will be performed only once
    INIT.call_once(|| {

        // The "vts_mock_disabled" feature determines whether to use KIS (production) or VTS (test) configuration
        let (service_type, envfile) = if cfg!(feature = "vts_mock_disabled") {
            (SERVICE_TYPE_KIS, ".env")
        } else {
            (SERVICE_TYPE_VTS, ".env.test")
        };

        warn!("!!! CAUTION !!!");
        warn!(" - Configuration set: [{}]", service_type);
        
        match dotenv::from_filename(envfile) {
            Ok(_) => warn!(" - Environment variables loaded successfully from ({})", envfile),
            Err(e) => error!(" - Failed to load environment variables: {}", e),
        }
        warn!("-----------------------------------------------------------------");

        // Initialize INSTANCE in safe block
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