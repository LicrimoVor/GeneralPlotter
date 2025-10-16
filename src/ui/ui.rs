use super::panel::Panel;
use super::settings::SettingsModal;
use super::types::ConfigLogic;
use crate::{
    core::settings::Settings,
    libs::{
        serials::{Serial, SerialAction, SerialEvent},
        svg_img::SvgImage,
        types::Theme,
    },
    logic::SensorData,
};
use egui::Vec2;
use futures::channel::mpsc;
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
    settings_modal: SettingsModal,
    panel: Panel,
}

impl UserInterface {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
        serial: &mut Serial,
    ) -> Self {
        let (mut serial_rx, mut serial_tx) = serial.subscribe();
        let serial_rx = Rc::new(RefCell::new(serial_rx));
        let serial_tx = Rc::new(RefCell::new(serial_tx));

        Self {
            config,
            sensor_data,
            settings_modal: SettingsModal::new(settings.clone()),
            settings,

            serial_rx: serial_rx.clone(),
            serial_tx: serial_tx.clone(),

            panel: Panel::new(serial_rx, serial_tx),
        }
    }

    fn update(&mut self) {
        // while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
    }

    pub fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        self.update();
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
                    self.panel.show(ctx, ui);
                    self.settings_modal.show(ctx, ui);
                });
        }
        if width > 1170.0 {
            egui::SidePanel::right("rigth")
                .min_width(200.0)
                .max_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Левая панель");
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.panel.run(ctx, ui);
            ui.add(SvgImage::PEACE.get_image());

            ui.add(SvgImage::PEACE.get_image());
        });
    }
}
