use super::super::libs::button_image::button_image_14;
use super::types::BaudRate;
use crate::libs::{svg_img::SvgImage, types::Timer};
use egui::Color32;
use serialport::{SerialPortBuilder, available_ports};
use std::time::{Duration, Instant};

pub struct Panel {
    ports: Vec<serialport::SerialPortInfo>,
    selected_port: Option<String>,
    baud_rate: BaudRate,
    opened_port: Option<SerialPortBuilder>,

    _timer: Timer,
}

impl Default for Panel {
    fn default() -> Self {
        let ports = available_ports().unwrap();
        Self {
            ports,
            selected_port: None,
            opened_port: None,
            baud_rate: BaudRate::Baud115200,
            _timer: Timer {
                last_update: Instant::now(),
                interval: Duration::from_secs(30),
            },
        }
    }
}

impl Panel {
    fn update(&mut self) {
        let now_ports = available_ports().unwrap();
        if let Some(selected_port) = &self.selected_port {
            let selected_port = self.ports.iter().find(|a| a.port_name == *selected_port);
        }
        self.ports = now_ports;
    }

    fn connect(&mut self) {
        let Some(selected_port) = &self.selected_port else {
            return;
        };
        let Some(port) = self.ports.iter().find(|a| a.port_name == *selected_port) else {
            return;
        };
        let opened_port = serialport::new(selected_port, self.baud_rate.clone() as u32);
    }

    pub fn get_opened_port(&self) -> &Option<SerialPortBuilder> {
        &self.opened_port
    }

    pub fn run(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self._timer.is_pass_iterval() {
            self.update();
        }
        let width = ui.available_width();

        ui.vertical_centered_justified(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("port_select")
                    .width(width - 40.0)
                    .selected_text(
                        self.selected_port
                            .clone()
                            .unwrap_or_else(|| "Выберите порт".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for port in &self.ports {
                            ui.selectable_value(
                                &mut self.selected_port,
                                Some(port.port_name.clone()),
                                &port.port_name,
                            );
                        }
                    });

                ui.add_space(4.0);
                if button_image_14(ui, SvgImage::RELOAD, Color32::WHITE).clicked() {
                    self.update()
                };
            });
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("baud_rate_select")
                    .width((width - 16.0) / 2.0)
                    .selected_text(self.baud_rate.value().to_string())
                    .show_ui(ui, |ui| {
                        for baud_rate in BaudRate::all() {
                            ui.selectable_value(
                                &mut self.baud_rate,
                                *baud_rate,
                                baud_rate.value().to_string(),
                            );
                        }
                    });
                ui.add_space(4.0);

                ui.scope(|ui| {
                    ui.set_min_size(egui::vec2((width - 16.0) / 2.0, 18.0));

                    if ui
                        .add_enabled(
                            self.selected_port.is_some(),
                            egui::Button::new("🔗 Подключиться"),
                        )
                        .clicked()
                    {
                        let Some(selected_port) = &self.selected_port else {
                            return;
                        };
                        println!(
                            "{:?}",
                            self.ports
                                .iter()
                                .find(|a| a.port_name == *selected_port)
                                .unwrap()
                        );
                        println!("Connect to port: {}", self.selected_port.as_ref().unwrap());
                    };
                });
            });
        });
    }
}
