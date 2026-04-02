use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

/// Simple rate limiter based on minimum inter-request interval.
///
/// KIS allows ~20 req/sec for REST. We default to 15 req/sec
/// (67 ms interval) to stay safely below the server limit.
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<Instant>>,
    min_interval: Duration,
}

impl RateLimiter {
    /// Create a rate limiter that allows at most `req_per_sec` requests per second.
    pub fn new(req_per_sec: u32) -> Self {
        let min_interval = Duration::from_micros(1_000_000 / u64::from(req_per_sec));
        Self {
            // Start in the past so the first request goes through immediately.
            inner: Arc::new(Mutex::new(Instant::now() - min_interval)),
            min_interval,
        }
    }

    /// Wait until the next request is allowed, then return.
    pub async fn acquire(&self) {
        let mut last = self.inner.lock().await;
        let elapsed = last.elapsed();
        if elapsed < self.min_interval {
            tokio::time::sleep(self.min_interval - elapsed).await;
        }
        *last = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn first_request_is_immediate() {
        let rl = RateLimiter::new(15);
        let start = Instant::now();
        rl.acquire().await;
        assert!(start.elapsed() < Duration::from_millis(10));
    }

    #[tokio::test]
    async fn back_to_back_requests_are_throttled() {
        let rl = RateLimiter::new(15);
        rl.acquire().await;
        let start = Instant::now();
        rl.acquire().await;
        // At 15 req/s, interval ≈ 67ms. Allow some slack.
        assert!(start.elapsed() >= Duration::from_millis(50));
    }
}
