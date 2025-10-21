use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SettingsChart {
    pub display: Vec<bool>,
}
