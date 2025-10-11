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

pub struct LinierFunc {
    pub alpha: f32,
    pub beta: f32,
}

impl LinierFunc {
    pub fn new(alpha: f32, beta: f32) -> Self {
        Self { alpha, beta }
    }

    pub fn value(&self, x: f32) -> f32 {
        self.alpha * x + self.beta
    }
}
