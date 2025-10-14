use std::cell::RefCell;
use std::rc::Rc;

use super::super::libs::button_image::button_image_18;
use crate::libs::serials::{BaudRate, Serial, SerialDevice};
use crate::{
    libs::{svg_img::SvgImage, timer::Timer},
    ui::libs::status::{Status, status_img},
};
use egui::Widget;
#[cfg(target_arch = "wasm32")]
use futures::channel::mpsc;
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

pub struct Panel {
    serial: Rc<RefCell<Serial>>,
    selected_port: Option<SerialDevice>,
    baud_rate: BaudRate,
    status: Status,

    _timer: Timer,
    #[cfg(target_arch = "wasm32")]
    reader: Option<mpsc::UnboundedReceiver<String>>,
}

impl Default for Panel {
    fn default() -> Self {
        let serial = Rc::new(RefCell::new(Serial::new()));
        serial.borrow_mut().update_ports(serial.clone());

        Self {
            serial: serial.clone(),
            selected_port: None,
            status: Status::Default,
            baud_rate: BaudRate::Baud9600,
            _timer: Timer {
                last_update: Instant::now(),
                interval: Duration::from_secs(5),
            },

            #[cfg(target_arch = "wasm32")]
            reader: None,
        }
    }
}

impl Panel {
    fn update_ports(&mut self) {
        self.serial.borrow_mut().update_ports(self.serial.clone());
    }

    fn update(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.update_ports();
    }

    fn connect(&mut self) {
        let Some(selected_port) = &self.selected_port else {
            return;
        };
        let ports = &self.serial.borrow().ports;
        let Some(port) = ports.iter().find(|a| a.name == *selected_port.name) else {
            return;
        };
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
                            .map_or("Выберите порт".to_string(), |a| a.name),
                    )
                    .show_ui(ui, |ui| {
                        if self.serial.borrow().ports.len() == 0 {
                            ui.label("Нет портов");
                        }
                        for port in &self.serial.borrow().ports {
                            ui.selectable_value(
                                &mut self.selected_port,
                                Some(port.clone()),
                                &port.name,
                            );
                        }
                    });

                #[cfg(not(target_arch = "wasm32"))]
                let img = SvgImage::RELOAD;
                #[cfg(target_arch = "wasm32")]
                let img = SvgImage::PLUS;
                if button_image_18(ui, img, None).clicked() {
                    self.update_ports()
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
                        self.reader = self.serial.clone().borrow_mut().open_port(
                            self.selected_port.as_ref().unwrap().id,
                            self.baud_rate.clone(),
                            self.serial.clone(),
                        );
                    };
                });

                if self.reader.is_some() {
                    while let Some(line) = self
                        .reader
                        .as_mut()
                        .unwrap()
                        .try_next()
                        .inspect_err(|a| {
                            web_sys::console::log_1(&a.to_string().into());
                        })
                        .unwrap()
                    {
                        web_sys::console::log_1(&format!("Получено: {}", line).into());
                    }
                }
                ui.add_space(2.0);
                status_img(&self.status, ui).ui(ui);
                // ui.add(status_img(&self.status).ui(ui));
            });
        });
    }
}
