use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Text(String),
    Number(i32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    LIGTH,
    DARK,
    CUSTOM,
}

impl Default for Theme {
    fn default() -> Self {
        Self::DARK
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
