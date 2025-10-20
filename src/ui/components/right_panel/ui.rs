use egui::DragValue;

use crate::ui::settings::Settings;
use crate::{libs::types::Value, logic::config::ConfigLogic};
use std::sync::{Arc, Mutex};

pub struct RightPanel {
    settings: Arc<Mutex<Settings>>,
    config: Arc<Mutex<ConfigLogic>>,
}

impl RightPanel {
    pub fn new(settings: Arc<Mutex<Settings>>, config: Arc<Mutex<ConfigLogic>>) -> Self {
        Self { settings, config }
    }
}

impl RightPanel {
    pub fn update(&mut self) {}
    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("Линейные преобразования");

            let mut config = self.config.lock().unwrap();
            for i in 0..config.cols.len() {
                match config.cols[i] {
                    Value::Number(_) => {
                        ui.horizontal_centered(|ui| {
                            ui.label(format!("{}:", i));
                            ui.label("a=");
                            ui.add(DragValue::new(&mut config.linier_funcs[i].alpha).speed(0.1));
                            ui.label("b=");
                            ui.add(DragValue::new(&mut config.linier_funcs[i].beta).speed(0.1));
                        });
                    }
                    Value::Text(_) => {}
                }
            }
        });
    }
}
