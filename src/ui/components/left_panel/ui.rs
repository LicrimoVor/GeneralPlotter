use crate::core::settings::Settings;
use crate::libs::mpsc;
use crate::libs::serials::{SerialAction, SerialEvent};
use crate::ui::components::config_port::ConfigPort;
use crate::ui::components::settings::SettingsModal;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct LeftPanel {
    serial_rx: Rc<RefCell<mpsc::Receiver<SerialEvent>>>,
    serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,
    settings: Arc<Mutex<Settings>>,

    // ui
    settings_modal: SettingsModal,
    config_port: ConfigPort,
}

impl LeftPanel {
    pub fn new(
        settings: Arc<Mutex<Settings>>,
        serial_rx: Rc<RefCell<mpsc::Receiver<SerialEvent>>>,
        serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,
    ) -> Self {
        Self {
            serial_rx: serial_rx.clone(),
            serial_tx: serial_tx.clone(),
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
