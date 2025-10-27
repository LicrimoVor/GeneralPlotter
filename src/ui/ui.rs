use super::components::{
    central_panel::CentralPanel, left_panel::LeftPanel, right_panel::RightPanel,
};
use super::settings::Settings;
use crate::libs::timer::Timer;
use crate::logic::config::ConfigLogic;
use crate::{
    libs::{
        mpsc,
        serials::{Serial, SerialAction, SerialEvent},
        types::Theme,
    },
    logic::SensorData,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct UserInterface {
    // data
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
    ui_data: Arc<Mutex<super::UiData>>,
    settings: Arc<Mutex<Settings>>,

    // serial
    serial_rx: mpsc::Receiver<SerialEvent>,
    serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,

    // ui
    left_panel: LeftPanel,
    central_panel: CentralPanel,
    right_panel: RightPanel,

    _timer: Timer,
}

impl UserInterface {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
        ui_data: Arc<Mutex<super::UiData>>,
        serial: &mut Serial,
    ) -> Self {
        let (serial_rx, serial_tx) = serial.subscribe();
        let serial_tx = Rc::new(RefCell::new(serial_tx));

        Self {
            config: config.clone(),
            sensor_data: sensor_data.clone(),
            settings: settings.clone(),
            ui_data: ui_data.clone(),

            serial_rx: serial_rx,
            serial_tx: serial_tx.clone(),

            central_panel: CentralPanel::new(
                sensor_data.clone(),
                settings.clone(),
                ui_data.clone(),
                serial_tx.clone(),
            ),
            left_panel: LeftPanel::new(settings.clone(), ui_data.clone(), serial),
            right_panel: RightPanel::new(settings.clone(), config.clone()),

            _timer: Timer::new(50),
        }
    }

    fn update(&mut self) {
        // while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
        self.central_panel.update();
        self.left_panel.update();
        self.right_panel.update();

        self.settings.lock().unwrap().is_updated = false;
        self.ui_data.lock().unwrap().is_reboot = false;

        let event = self.serial_rx.try_recv();
        if event.is_none() {
            return;
        }

        let event = event.unwrap();
        match event {
            SerialEvent::Opened(result) => match result {
                Ok(true) => {
                    self.sensor_data.lock().unwrap().clear();
                    self.config.lock().unwrap().clear();
                    self.ui_data.lock().unwrap().clear();
                }
                _ => {}
            },
            SerialEvent::Data(result) => match result {
                Ok(data) => {
                    let mut ui_data = self.ui_data.lock().unwrap();
                    for val in data {
                        ui_data.update(val);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        if self._timer.is_pass_iterval() {
            self.update();
        }
        // ctx.set_debug_on_hover(true);

        {
            let settings = self.settings.lock().unwrap();
            match settings.theme {
                Theme::LIGTH => super::styles::apply_light_theme(ctx),
                Theme::DARK => super::styles::apply_dark_theme(ctx),
                Theme::CUSTOM => super::styles::apply_light_theme(ctx),
            }
        }

        let width = ctx.content_rect().size().x;

        if width > 720.0 {
            egui::SidePanel::left("left")
                .min_width(200.0)
                .max_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    self.left_panel.show(ctx, ui);
                });
        }
        if width > 1170.0 {
            egui::SidePanel::right("rigth")
                .min_width(200.0)
                .max_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    self.right_panel.show(ctx, ui);
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel.show(ctx, ui);
        });
    }
}
