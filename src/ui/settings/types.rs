use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
