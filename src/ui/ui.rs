use super::panel::Panel;
use super::settings::SettingsModal;
use super::types::ConfigLogic;
use crate::{core::settings::Settings, libs::types::Theme, logic::SensorData};
use egui::Vec2;
use std::sync::{Arc, Mutex};

pub struct UserInterface {
    // data
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
    settings: Arc<Mutex<Settings>>,

    // ui
    settings_modal: SettingsModal,
    panel: Panel,
}

impl UserInterface {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
    ) -> Self {
        Self {
            config,
            sensor_data,
            settings_modal: SettingsModal::new(settings.clone()),
            settings,
            panel: Panel::default(),
        }
    }

    fn update(&mut self) {
        // while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
    }

    pub fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update();
        // ctx.set_debug_on_hover(true);

        {
            let settings = self.settings.lock().unwrap();
            match settings.theme {
                Theme::LIGTH => super::styles::apply_light_theme(ctx),
                Theme::DARK => super::styles::apply_dark_theme(ctx),
                Theme::CUSTOM => super::styles::apply_light_theme(ctx),
            }
        }

        let Vec2 {
            x: width,
            y: heigth,
        } = ctx.content_rect().size();

        if width > 720.0 {
            egui::SidePanel::left("left")
                .min_width(200.0)
                .max_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    self.panel.show(ctx, ui);
                    self.settings_modal.show(ctx, ui);
                });
        }
        if width > 1170.0 {
            egui::SidePanel::right("rigth")
                .min_width(200.0)
                .max_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Левая панель");
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.panel.run(ctx, ui);
        });
    }
}
