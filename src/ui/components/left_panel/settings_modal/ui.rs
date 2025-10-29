use super::{tab_chart::TabChart, tab_terminal::TabTerminal};
use crate::{
    libs::{svg_img::SvgImage, types::Theme},
    ui::{UiData, libs::button_image::button_image_18, settings::Settings},
};
use egui::{DragValue, Id, Modal, TextEdit, Vec2};
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
enum SettingsTab {
    General,
    Terminal,
    Chart,
}

pub struct SettingsModal {
    settings: Arc<Mutex<Settings>>,
    ui_data: Arc<Mutex<UiData>>,

    //ui
    active_tab: SettingsTab,
    terminal: TabTerminal,
    chart: TabChart,

    _is_open: bool,
    _delimiter: String,
}

impl SettingsModal {
    pub fn new(settings: Arc<Mutex<Settings>>, ui_data: Arc<Mutex<UiData>>) -> Self {
        Self {
            settings: settings.clone(),
            ui_data: ui_data.clone(),

            active_tab: SettingsTab::General,
            terminal: TabTerminal::new(settings.clone(), ui_data.clone()),
            chart: TabChart::new(settings.clone(), ui_data.clone()),

            _is_open: false,
            _delimiter: settings.lock().unwrap().delimiter.to_string(),
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
                ui.selectable_value(&mut self.active_tab, SettingsTab::Chart, "График");
            });

            ui.separator();

            match self.active_tab {
                SettingsTab::General => {
                    let mut settings = self.settings.lock().unwrap();
                    let mut ui_data = self.ui_data.lock().unwrap();
                    ui.heading("Основные настройки");
                    ui.horizontal(|ui| {
                        ui.label("Тема интерфейса:");
                        ui.radio_value(&mut settings.theme, Theme::LIGTH, "Светлая");
                        ui.radio_value(&mut settings.theme, Theme::DARK, "Тёмная");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Разделитель:");
                        ui.add(
                            TextEdit::singleline(&mut self._delimiter)
                                .char_limit(1)
                                .desired_width(12.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Столбец времени serial:");
                        ui.add(
                            DragValue::new(&mut settings.time_serial_col)
                                .speed(1)
                                .max_decimals(0)
                                .min_decimals(0)
                                .range(0..=20),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Время ОС или serial:");
                        ui.checkbox(&mut settings.is_time_serial, "");
                    });
                    if !self._delimiter.is_empty() {
                        settings.delimiter = self._delimiter.chars().next().unwrap();
                        ui_data.is_reboot = true;
                    }
                }
                SettingsTab::Terminal => {
                    self.terminal.show(ctx, ui);
                }
                SettingsTab::Chart => {
                    self.chart.show(ctx, ui);
                }
            }
        });

        if modal.should_close() {
            self._is_open = false;
        }
    }
}
