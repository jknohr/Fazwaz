use std::sync::Arc;
use tokio::time::{Duration, Instant};
use dashmap::DashMap;

pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
    window: Duration,
    max_requests: usize,
}

impl RateLimiter {
    pub fn new(window_secs: u64, max_requests: usize) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            window: Duration::from_secs(window_secs),
            max_requests,
        }
    }

    pub fn check_rate_limit(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut entry = self.requests.entry(key.to_string()).or_default();
        
        // Remove old timestamps
        entry.retain(|ts| now.duration_since(*ts) <= self.window);
        
        if entry.len() >= self.max_requests {
            false
        } else {
            entry.push(now);
            true
        }
    }
} 