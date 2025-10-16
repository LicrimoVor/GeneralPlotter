use super::super::libs::button_image::button_image_18;
use crate::libs::serials::{BaudRate, SerialAction, SerialDevice, SerialEvent};
use crate::{
    libs::{svg_img::SvgImage, timer::Timer},
    ui::libs::status::{Status, status_img},
};
use egui::{Vec2, Widget};
use futures::channel::mpsc;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Panel {
    ports: Vec<SerialDevice>,
    selected_port: Option<SerialDevice>,
    baud_rate: BaudRate,
    status: Status,
    is_opened: bool,

    serial_rx: Rc<RefCell<mpsc::Receiver<SerialEvent>>>,
    serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,

    _angle_loader: f32,
    _timer: Timer,
}

impl Panel {
    pub fn new(
        serial_rx: Rc<RefCell<mpsc::Receiver<SerialEvent>>>,
        serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,
    ) -> Self {
        let mut panel = Self {
            ports: vec![],
            selected_port: None,
            status: Status::Default,
            baud_rate: BaudRate::Baud9600,
            is_opened: false,

            serial_rx,
            serial_tx,

            _angle_loader: 0.0,
            _timer: Timer::default(),
        };
        panel.update_ports();
        panel
    }
}

impl Panel {
    fn update_ports(&mut self) {
        let _ = self
            .serial_tx
            .borrow_mut()
            .try_send(SerialAction::UpdatePorts);
    }

    fn update(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.update_ports();
    }

    fn serial_read(&mut self) {
        let event = self.serial_rx.borrow_mut().try_next().ok().flatten();
        if event.is_none() {
            return;
        }
        let event = event.unwrap();
        match event {
            SerialEvent::Opened(result) => match result {
                Ok(true) => self.status = Status::Ok,
                Ok(false) => self.status = Status::Default,
                Err(_) => self.status = Status::Error,
                _ => {}
            },
            SerialEvent::Loading(is_loading) => match is_loading {
                Ok(true) => self.status = Status::isLoading,
                Err(_) => self.status = Status::Error,
                _ => {}
            },
            SerialEvent::Ports(result) => {
                if let Ok(ports) = result {
                    self.ports = ports;
                }
            }
            _ => {}
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self._timer.is_pass_iterval() {
            self.update();
        }
        self.serial_read();

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
                        if self.ports.len() == 0 {
                            ui.label("Нет портов");
                        }
                        for port in &self.ports {
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

                    if self.status == Status::Ok {
                        if button_image_18(ui, SvgImage::DISCONNECT, None).clicked() {
                            let _ = self
                                .serial_tx
                                .borrow_mut()
                                .try_send(SerialAction::ClosePort);
                        };
                    } else {
                        if button_image_18(ui, SvgImage::CONNECT, None).clicked() {
                            let _ = self
                                .serial_tx
                                .borrow_mut()
                                .try_send(SerialAction::OpenPort((
                                    self.selected_port.as_ref().unwrap().clone(),
                                    self.baud_rate,
                                )));
                        };
                    }
                });

                ui.add_space(2.0);
                if self.status == Status::isLoading {
                    self._angle_loader += 0.03;
                    status_img(&self.status, ui)
                        .rotate(self._angle_loader, Vec2::splat(0.5))
                        .ui(ui);
                } else {
                    status_img(&self.status, ui).ui(ui);
                }
                // ui.add(status_img(&self.status).ui(ui));
            });
        });
    }
}
