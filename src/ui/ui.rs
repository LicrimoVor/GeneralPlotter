use super::components::{central_panel::CentralPanel, left_panel::LeftPanel};
use super::types::ConfigLogic;
use crate::libs::timer::Timer;
use crate::{
    core::settings::Settings,
    libs::{
        mpsc,
        serials::{Serial, SerialAction, SerialEvent},
        types::Theme,
    },
    logic::SensorData,
};
use egui::Vec2;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct UserInterface {
    // data
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
    settings: Arc<Mutex<Settings>>,

    // serial
    serial_rx: Rc<RefCell<mpsc::Receiver<SerialEvent>>>,
    serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,

    // ui
    left_panel: LeftPanel,
    central_panel: CentralPanel,

    _timer: Timer,
}

impl UserInterface {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
        serial: &mut Serial,
    ) -> Self {
        let (serial_rx, serial_tx) = serial.subscribe();
        let serial_rx = Rc::new(RefCell::new(serial_rx));
        let serial_tx = Rc::new(RefCell::new(serial_tx));

        Self {
            config: config.clone(),
            sensor_data: sensor_data.clone(),
            settings: settings.clone(),

            serial_rx: serial_rx.clone(),
            serial_tx: serial_tx.clone(),

            central_panel: CentralPanel::new(
                config.clone(),
                sensor_data.clone(),
                settings.clone(),
                serial_tx.clone(),
            ),
            left_panel: LeftPanel::new(settings.clone(), serial_rx.clone(), serial_tx.clone()),

            _timer: Timer::new(50),
        }
    }

    fn update(&mut self) {
        // while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
        self.central_panel.update();
        self.left_panel.update();
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

        let Vec2 {
            x: width,
            y: heigth,
        } = ctx.content_rect().size();

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
                    ui.label("Правая панель");
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            self.central_panel.show(ctx, ui);
        });
    }
}
