use crate::logic::{Logic, SensorData};
use crate::ui::{ConfigLogic, UserInterface};
use eframe::App;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
pub struct AppState {
    ui: UserInterface,

    config: ConfigLogic,
    sensor_data: SensorData,
}

impl AppState {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        let (config_tx, config_rx) = mpsc::channel::<ConfigLogic>();
        let (sensor_tx, sensor_rx) = mpsc::channel::<SensorData>();
        let mut config = ConfigLogic::new(storage);
        let mut sensor_data = SensorData::new(storage);

        let mut logic = Logic::new(config_rx, sensor_tx);
        let mut ui = UserInterface::new(sensor_rx, config_tx);

        thread::spawn(move || {
            loop {
                logic.run();
            }
        });
        Self {
            ui,
            config,
            sensor_data,
        }
    }
}

impl App for AppState {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, crate::consts::KEY_CONFIG, &self.config);
        eframe::set_value(storage, crate::consts::KEY_DATA, &self.sensor_data);
        // eframe::Storage::set_value(storage, crate::consts::KEY_SETTINGS, &self.);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui.run(ctx, _frame);
    }
}
