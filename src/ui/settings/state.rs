use super::Theme;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsUI {
    theme: Theme,
}

impl Default for SettingsUI {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}
