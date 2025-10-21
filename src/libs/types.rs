use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Text(String),
    Number(f64),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinierFunc {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for LinierFunc {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            beta: 0.0,
        }
    }
}

impl LinierFunc {
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self { alpha, beta }
    }

    pub fn value(&self, x: f64) -> f64 {
        self.alpha * x + self.beta
    }
}
