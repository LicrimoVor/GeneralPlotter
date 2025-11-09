use super::{config_port::ConfigPort, description::Description, settings_modal::SettingsModal};
use crate::libs::serials::Serial;
use crate::ui::UiData;
use crate::ui::settings::Settings;
use std::sync::{Arc, Mutex};

pub struct LeftPanel {
    // ui
    settings_modal: SettingsModal,
    config_port: ConfigPort,
    description: Description,
}

impl LeftPanel {
    pub fn new(
        settings: Arc<Mutex<Settings>>,
        ui_data: Arc<Mutex<UiData>>,
        serial: &mut Serial,
    ) -> Self {
        let _ = ui_data;
        let (serial_rx, serial_tx) = serial.subscribe();

        Self {
            settings_modal: SettingsModal::new(settings, ui_data),
            config_port: ConfigPort::new(serial_rx, serial_tx),
            description: Description::new(),
        }
    }
}

impl LeftPanel {
    pub fn update(&mut self) {
        self.config_port.update();
        self.settings_modal.update();
    }
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.config_port.show(ctx, ui);
        self.description.show(ctx, ui);
        self.settings_modal.show(ctx, ui);
    }
}
