use crate::{
    libs::{svg_img::SvgImage, types::Theme},
    ui::{
        components::settings_modal::tab_terminal::TabTerminal, libs::button_image::button_image_18,
        settings::Settings,
    },
};
use egui::{Id, Modal, Vec2};
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
enum SettingsTab {
    General,
    Terminal,
}

pub struct SettingsModal {
    _is_open: bool,
    settings: Arc<Mutex<Settings>>,
    active_tab: SettingsTab,

    //ui
    terminal: TabTerminal,
}

impl SettingsModal {
    pub fn new(settings: Arc<Mutex<Settings>>) -> Self {
        Self {
            _is_open: false,
            settings: settings.clone(),

            active_tab: SettingsTab::General,
            terminal: TabTerminal::new(settings.clone()),
        }
    }
}

impl SettingsModal {
    pub fn update(&mut self) {}

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if button_image_18(ui, SvgImage::SETTINGS, None).clicked() {
            self._is_open = true;
        }

        if !self._is_open {
            return;
        }

        let modal = Modal::new(Id::new("settings_modal")).show(ctx, |ui| {
            ui.set_min_size(Vec2::new(600.0, 500.0));
            ui.set_max_size(Vec2::new(600.0, 500.0));

            ui.horizontal(|ui| {
                // Вкладки
                ui.selectable_value(&mut self.active_tab, SettingsTab::General, "Основные");
                ui.selectable_value(&mut self.active_tab, SettingsTab::Terminal, "Терминал");
            });

            ui.separator();

            match self.active_tab {
                SettingsTab::General => {
                    let mut settings = self.settings.lock().unwrap();
                    ui.heading("Основные настройки");
                    ui.label("Тема интерфейса:");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut settings.theme, Theme::LIGTH, "Светлая");
                        ui.radio_value(&mut settings.theme, Theme::DARK, "Тёмная");
                    });
                }

                SettingsTab::Terminal => {
                    self.terminal.show(ctx, ui);
                }
            }
        });

        if modal.should_close() {
            self._is_open = false;
        }
    }
}
