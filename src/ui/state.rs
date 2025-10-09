use super::panel::Panel;
use crate::logic::ProxyData;
use std::sync::mpsc;

struct UserInterfaceState {
    proxy_data_rx: mpsc::Receiver<ProxyData>,
    config_tx: mpsc::Sender<ConfigLogic>,
    __config: ConfigLogic,
}

pub struct ConfigLogic {}

impl Default for ConfigLogic {
    fn default() -> Self {
        Self {}
    }
}

pub struct UserInterface {
    state: UserInterfaceState,
    panel: Panel,
}

impl UserInterface {
    pub fn new(
        proxy_data_rx: mpsc::Receiver<ProxyData>,
        config_tx: mpsc::Sender<ConfigLogic>,
    ) -> Self {
        Self {
            state: UserInterfaceState {
                proxy_data_rx: proxy_data_rx,
                config_tx: config_tx,
                __config: ConfigLogic::default(),
            },
            panel: Panel::default(),
        }
    }

    fn update(&mut self) {
        while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
    }

    pub fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update();
        egui::SidePanel::left("left")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Левая панель");
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.panel.run(ctx, ui);
            ui.centered_and_justified(|ui| {
                ui.label("Полное центрирование");
            });
        });
    }
}
