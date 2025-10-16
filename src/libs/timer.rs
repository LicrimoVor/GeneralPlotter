#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

pub struct Timer {
    pub last_update: Instant,
    pub interval: Duration,
}

impl Timer {
    pub fn default() -> Self {
        Self {
            last_update: Instant::now(),
            interval: Duration::from_secs(5),
        }
    }

    pub fn new(iterval_ms: u64) -> Self {
        Self {
            last_update: Instant::now(),
            interval: Duration::from_millis(iterval_ms),
        }
    }

    pub fn is_pass_iterval(&mut self) -> bool {
        if self.last_update.elapsed() >= self.interval {
            self.last_update = Instant::now();
            return true;
        }
        false
    }
}
