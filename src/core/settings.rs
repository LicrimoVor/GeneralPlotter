use crate::libs::types::Theme;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Settings {
    pub theme: Theme,
}

impl Settings {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, super::consts::KEY_SETTINGS).unwrap_or_default()
        } else {
            Settings::default()
        }
    }
}
