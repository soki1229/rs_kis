use std::time::Duration;

pub trait ConfigurationProvider {
    fn http_endpoint(&self) -> &str;
    fn socket_endpoint(&self) -> &str;
    fn time_out(&self) -> Duration;
    fn max_retries(&self) -> u32;
}

impl ConfigurationProvider for Configurations {
    fn http_endpoint(&self) -> &str {
        &self.presets.http_endpoint
    }

    fn socket_endpoint(&self) -> &str {
        &self.presets.socket_endpoint
    }

    fn time_out(&self) -> Duration {
        self.default_timeout
    }

    fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

#[derive(Clone, Debug, Default)]
pub struct Configurations {
    presets: Presets,
    default_timeout: Duration,
    max_retries: u32,
}

impl Configurations {
    pub fn new() -> Self {
        Self {
            presets: Presets::new(),
            default_timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }

    pub fn mockup(&mut self) {
        self.presets = Presets::mock();
    }
}

#[derive(Clone, Debug, Default)]
struct Presets {
    http_endpoint: &'static str,
    socket_endpoint: &'static str,
}

impl Presets {
    fn new() -> Self {
        Self {
            http_endpoint: "https://openapi.koreainvestment.com:9443",
            socket_endpoint: "ws://ops.koreainvestment.com:21000",
        }
    }

    fn mock() -> Self {
        Self {
            http_endpoint: "https://openapi.koreainvestment.com:29443",
            socket_endpoint: "ws://ops.koreainvestment.com:31000",
        }
    }
}
