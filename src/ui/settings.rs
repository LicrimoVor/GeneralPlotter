use super::components::central_panel::{
    chart::settings::SettingsChart, terminal::settings::SettingsTerminal,
};
use crate::{core::consts::KEY_SETTINGS, libs::types::Theme};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
    pub delimiter: char,
    pub time_step_ms: i32,
    pub is_time_serial: bool,
    pub time_serial_col: usize,

    pub terminal: SettingsTerminal,
    pub chart: SettingsChart,

    pub is_updated: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::DARK,
            delimiter: ';',
            time_step_ms: 50,
            is_time_serial: false,
            time_serial_col: 0,

            terminal: SettingsTerminal::default(),
            chart: SettingsChart::default(),

            is_updated: false,
        }
    }
}

impl Settings {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, KEY_SETTINGS).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn clear(&mut self) {
        // self.chart.display.clear();
    }
}
