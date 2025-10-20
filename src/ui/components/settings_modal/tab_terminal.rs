use crate::ui::settings::Settings;
use std::sync::{Arc, Mutex};

pub struct TabTerminal {
    settings: Arc<Mutex<Settings>>,
}

impl TabTerminal {
    pub fn new(settings: Arc<Mutex<Settings>>) -> Self {
        Self { settings }
    }

    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Настройки терминала");
        let settings = &mut self.settings.lock().unwrap().terminal;

        ui.checkbox(&mut settings.show_time, "Показывать время");
        ui.checkbox(&mut settings.show_id, "Показывать ID сообщения");

        ui.checkbox(&mut settings.time_selectable, "Выделение времени");
        ui.checkbox(&mut settings.id_selectable, "Выделение ID сообщения");

        ui.checkbox(&mut settings.show_separator, "Показывать разделитель");

        ui.separator();
        ui.label("Режимы:");
        ui.horizontal(|ui| {
            ui.checkbox(&mut settings.mode_table, "Таблица");
        });

        ui.separator();
        ui.label("Типы сообщений:");

        ui.horizontal(|ui| {
            ui.checkbox(&mut settings.show_msg_info, "Info");
            ui.color_edit_button_srgb(&mut settings.msg_info_color);
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut settings.show_msg_error, "Error");
            ui.color_edit_button_srgb(&mut settings.msg_error_color);
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut settings.show_msg_fetch, "Fetch");
            ui.color_edit_button_srgb(&mut settings.msg_fetch_color);
        });
    }
}
