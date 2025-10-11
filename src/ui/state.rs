use egui::Vec2;

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

        let Vec2 {
            x: width,
            y: heigth,
        } = ctx.screen_rect().size();

        if width > 720.0 {
            egui::SidePanel::left("left")
                .min_width(170.0)
                .max_width(170.0)
                .resizable(false)
                .show(ctx, |ui| {
                    let mut style = ui.style_mut().clone();
                    style.spacing.interact_size = egui::vec2(0.0, 18.0);
                    style.spacing.button_padding = egui::vec2(4.0, 4.0);
                    ui.set_style(style);

                    ui.add_space(8.0);
                    self.panel.run(ctx, ui);
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.panel.run(ctx, ui);
        });
        if width > 1170.0 {
            egui::SidePanel::right("rigth")
                .min_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Левая панель");
                });
        }
    }
}
