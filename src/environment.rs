use std::env;
use std::sync::Once;
use log::{warn, error};

const SERVICE_TYPE_KIS: &str = "KIS";
const SERVICE_TYPE_VTS: &str = "VTS";

const ENV_FILE_KIS: &str = ".env";
const ENV_FILE_VTS: &str = ".env.test";

// Once instance for initializing gloabl variable
static mut INSTANCE: Option<Config> = None;
static CONFIG: Once = Once::new();

pub struct Config {
    pub app_key: String,
    pub app_secret: String,
    pub domain_socket: String,
    pub domain_restful: String,
}

pub fn init() {
    // Ensure the initialize will be performed only once
    CONFIG.call_once(|| {

        // The "vts_mock_disabled" feature determines whether to use KIS (production) or VTS (test) configuration
        let (service_type, envfile) = if cfg!(feature = "vts_mock_disabled") {
            (SERVICE_TYPE_KIS, ENV_FILE_KIS)
        } else {
            (SERVICE_TYPE_VTS, ENV_FILE_VTS)
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
                app_key: env::var("APP_KEY").expect("APP_KEY must be set"),
                app_secret: env::var("APP_SECRET").expect("APP_SECRET must be set"),
                domain_socket: env::var("WS_URL").expect("WS_URL must be set"),
                domain_restful: env::var("REST_URL").expect("REST_URL must be set"),
            });
        }
    });
}

pub fn get() -> &'static Config {
    unsafe {
        INSTANCE.as_ref().expect("Config not initialized")
    }
}