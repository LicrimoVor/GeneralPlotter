use super::components::terminal::settings::SettingsTerminal;
use crate::{core::consts::KEY_SETTINGS, libs::types::Theme};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Settings {
    pub theme: Theme,
    pub delimeter: char,
    pub time_step_ms: i32,

    pub terminal: SettingsTerminal,

    pub _is_updated: bool,
}

impl Settings {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, KEY_SETTINGS).unwrap_or_default()
        } else {
            Self {
                delimeter: ';',
                time_step_ms: 50,
                _is_updated: false,
                ..Settings::default()
            }
        }
    }
}
