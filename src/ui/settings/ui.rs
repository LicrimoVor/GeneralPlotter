use crate::{
    core::settings::Settings,
    libs::{svg_img::SvgImage, types::Theme},
    ui::libs::button_image::button_image_18,
};
use egui::{Color32, Id, Modal};
use std::sync::{Arc, Mutex};

pub struct SettingsModal {
    _is_open: bool,

    settings: Arc<Mutex<Settings>>,
}

impl SettingsModal {
    pub fn new(settings: Arc<Mutex<Settings>>) -> Self {
        Self {
            _is_open: false,
            settings,
        }
    }
}
impl SettingsModal {
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let Self { _is_open, settings } = self;

        if button_image_18(ui, SvgImage::SETTINGS, Color32::WHITE).clicked() {
            *_is_open = true;
        }

        if !*_is_open {
            return;
        }

        let modal = Modal::new(Id::new("Settings")).show(ctx, |ui| {
            let mut settings = settings.lock().unwrap();

            ui.vertical(|ui| {
                ui.label("Theme");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut settings.theme, Theme::LIGTH, "Light");
                    ui.radio_value(&mut settings.theme, Theme::DARK, "Dark");
                });
            });
        });
        if modal.should_close() {
            *_is_open = false;
        }
    }
}
