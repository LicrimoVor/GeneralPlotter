use eframe::App;
use std::{sync::mpsc, thread};

use crate::logic::{Logic, ProxyData};
use crate::ui::{ConfigLogic, UserInterface};

pub struct AppState {
    ui: UserInterface,
}

impl Default for AppState {
    fn default() -> Self {
        let (config_tx, config_rx) = mpsc::channel::<ConfigLogic>();
        let (proxy_tx, proxy_rx) = mpsc::channel::<ProxyData>();

        let mut logic = Logic::new(config_rx, proxy_tx);
        let ui = UserInterface::new(proxy_rx, config_tx);

        thread::spawn(move || {
            loop {
                logic.run();
            }
        });
        Self { ui }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui.run(ctx, _frame);
    }
}
