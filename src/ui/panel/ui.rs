use super::super::libs::button_image::button_image_18;
use super::types::BaudRate;
use crate::{
    libs::{svg_img::SvgImage, timer::Timer},
    ui::{
        libs::status::{Status, status_img},
        panel::utils::get_ports,
    },
};
use egui::Widget;
use serialport::SerialPortBuilder;
use web_time::{Duration, Instant};

pub struct Panel {
    ports: Vec<serialport::SerialPortInfo>,
    selected_port: Option<String>,
    baud_rate: BaudRate,
    opened_port: Option<SerialPortBuilder>,
    status: Status,

    _timer: Timer,
}

impl Default for Panel {
    fn default() -> Self {
        let ports = get_ports();

        Self {
            ports,
            selected_port: None,
            opened_port: None,
            baud_rate: BaudRate::Baud115200,
            status: Status::Default,
            _timer: Timer {
                last_update: Instant::now(),
                interval: Duration::from_secs(5),
            },
        }
    }
}

impl Panel {
    fn update(&mut self) {
        let now_ports = get_ports();

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

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self._timer.is_pass_iterval() {
            self.update();
        }
        let width = ui.available_width();

        ui.vertical_centered_justified(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("port_select")
                    .width(width - 32.0)
                    .selected_text(
                        self.selected_port
                            .clone()
                            .unwrap_or_else(|| "Выберите порт".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        if self.ports.len() == 0 {
                            ui.label("Нет портов");
                        }
                        for port in &self.ports {
                            ui.selectable_value(
                                &mut self.selected_port,
                                Some(port.port_name.clone()),
                                &port.port_name,
                            );
                        }
                    });

                if button_image_18(ui, SvgImage::RELOAD, None).clicked() {
                    self.update()
                };
            });
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("baud_rate_select")
                    .width(width - 64.0)
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

                ui.scope(|ui| {
                    if self.selected_port.is_none() {
                        ui.disable();
                    }

                    if button_image_18(ui, SvgImage::CONNECT, None).clicked() {
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
                ui.add_space(2.0);
                status_img(&self.status, ui).ui(ui);
                // ui.add(status_img(&self.status).ui(ui));
            });
        });
    }
}
