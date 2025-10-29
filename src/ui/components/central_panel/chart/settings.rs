use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsChart {
    pub display: Vec<bool>,
    pub count_points: usize,
}

impl Default for SettingsChart {
    fn default() -> Self {
        Self {
            display: Vec::new(),
            count_points: 1000,
        }
    }
}
