use std::time::{Duration, Instant};

pub enum Value {
    Text(String),
    Number(i32),
}

pub struct Timer {
    pub last_update: Instant,
    pub interval: Duration,
}

impl Timer {
    pub fn is_pass_iterval(&mut self) -> bool {
        if self.last_update.elapsed() >= self.interval {
            self.last_update = Instant::now();
            return true;
        }
        false
    }
}
