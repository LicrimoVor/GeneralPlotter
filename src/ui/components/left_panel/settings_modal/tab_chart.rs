use egui::DragValue;

use crate::ui::{UiData, settings::Settings};
use std::sync::{Arc, Mutex};

pub struct TabChart {
    settings: Arc<Mutex<Settings>>,
    ui_data: Arc<Mutex<UiData>>,
}

impl TabChart {
    pub fn new(settings: Arc<Mutex<Settings>>, ui_data: Arc<Mutex<UiData>>) -> Self {
        Self { settings, ui_data }
    }

    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Настройки графика");
        let settings = &mut self.settings.lock().unwrap().chart;

        ui.add(
            DragValue::new(&mut settings.count_points)
                .min_decimals(0)
                .max_decimals(0)
                .speed(1)
                .range(0..=3000),
        );
    }
}
