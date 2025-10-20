use crate::libs::serials::Serial;
use crate::ui::{
    components::{config_port::ConfigPort, settings_modal::SettingsModal},
    settings::Settings,
};
use std::sync::{Arc, Mutex};

pub struct LeftPanel {
    settings: Arc<Mutex<Settings>>,

    // ui
    settings_modal: SettingsModal,
    config_port: ConfigPort,
}

impl LeftPanel {
    pub fn new(settings: Arc<Mutex<Settings>>, serial: &mut Serial) -> Self {
        let (serial_rx, serial_tx) = serial.subscribe();

        Self {
            settings: settings.clone(),

            settings_modal: SettingsModal::new(settings),
            config_port: ConfigPort::new(serial_rx, serial_tx),
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
        self.settings_modal.show(ctx, ui);
    }
}
