use crate::libs::svg_img::SvgImage;
use crate::ui::settings::Settings;
use crate::{logic::config::ConfigLogic, ui::libs::button_image::button_image_18};
use egui::DragValue;
use std::sync::{Arc, Mutex};

pub struct RightPanel {
    settings: Arc<Mutex<Settings>>,
    config: Arc<Mutex<ConfigLogic>>,

    is_linier_mode: bool,
    is_reload: bool,
}

impl RightPanel {
    pub fn new(settings: Arc<Mutex<Settings>>, config: Arc<Mutex<ConfigLogic>>) -> Self {
        Self {
            settings,
            config,

            is_linier_mode: false,
            is_reload: false,
        }
    }
}

impl RightPanel {
    pub fn update(&mut self) {}
    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        let mut config = self.config.lock().unwrap();
        let mut settings = self.settings.lock().unwrap();

        if self.is_reload {
            config.is_reload = true;
            self.is_reload = false;
        }

        ui.horizontal(|ui| {
            ui.label("Линейные\nпреобразования");
            ui.add_space(24.0);
            if button_image_18(ui, SvgImage::RELOAD, None).clicked() {
                config.is_reload = true;
            }
            ui.checkbox(&mut self.is_linier_mode, "");
        });

        let mut count = 0;
        for (i, linier) in config.linier_funcs.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.set_height(32.0);
                ui.label(format!("{}:", i));

                match linier {
                    Some(linier_) => {
                        if ui
                            .add(
                                DragValue::new(&mut linier_.alpha)
                                    .speed(0.1)
                                    .range(-100.0..=100.0)
                                    .custom_formatter(|v, _| format!("{:05.2}", v))
                                    .prefix("a="),
                            )
                            .changed()
                        {
                            if self.is_linier_mode {
                                self.is_reload = true;
                            }
                        };
                        ui.label(";");
                        if ui
                            .add(
                                DragValue::new(&mut linier_.beta)
                                    .speed(0.1)
                                    .range(-100.0..=100.0)
                                    .custom_formatter(|v, _| format!("{:05.2}", v))
                                    .prefix("b="),
                            )
                            .changed()
                        {
                            if self.is_linier_mode {
                                self.is_reload = true;
                            }
                        };

                        ui.checkbox(&mut settings.chart.display[count], "");
                        count += 1;
                    }
                    None => {
                        ui.label("Тип параметра - строка");
                    }
                }
            });
        }
    }
}
