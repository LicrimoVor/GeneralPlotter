use std::time::{Duration, Instant};

use serialport::{SerialPort, available_ports};

use crate::libs::types::Timer;

pub struct Panel {
    ports: Vec<serialport::SerialPortInfo>,
    selected_port: Option<String>,

    _timer: Timer,
}

impl Default for Panel {
    fn default() -> Self {
        let ports = available_ports().unwrap();
        Self {
            ports,
            selected_port: None,
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

    pub fn run(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self._timer.is_pass_iterval() {
            self.update();
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let selected_port = if self.selected_port.is_some() {
                    self.selected_port.as_ref().unwrap()
                } else {
                    "Выберите порт"
                };
                egui::ComboBox::from_id_salt("port_select")
                    .selected_text(selected_port)
                    .show_ui(ui, |ui| {
                        for (i, port) in self.ports.iter().enumerate() {
                            ui.selectable_value(
                                &mut self.selected_port,
                                Some(port.port_name.clone()),
                                &port.port_name,
                            );
                        }
                    });
                if ui
                    .add_enabled(
                        self.selected_port.is_some(),
                        egui::Button::new("🔗 Подключиться"),
                    )
                    .clicked()
                {
                    self.update()
                };
            });

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
    }
}
