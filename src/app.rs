use crate::core::settings::Settings;
use crate::logic::{Logic, SensorData};
use crate::ui::{ConfigLogic, UserInterface};
use eframe::App;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
use wasm_bindgen_futures::spawn_local;

pub struct AppState {
    ui: UserInterface,
    // logic_handle: thread::JoinHandle<()>,
    settings: Arc<Mutex<Settings>>,
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
}

impl AppState {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        let config = Arc::new(Mutex::new(ConfigLogic::new(storage)));
        let sensor_data = Arc::new(Mutex::new(SensorData::new(storage)));
        let settings = Arc::new(Mutex::new(Settings::new(storage)));

        let mut logic = Logic::new(config.clone(), sensor_data.clone());
        let mut ui = UserInterface::new(config.clone(), sensor_data.clone(), settings.clone());

        // let logic_handle = thread::spawn(move || {
        //     loop {
        //         logic.run();
        //         std::thread::sleep(std::time::Duration::from_millis(50));
        //     }
        // });

        #[cfg(not(target_arch = "wasm32"))]
        {
            std::thread::spawn(move || {
                loop {
                    logic.run();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });
        }

        // #[cfg(target_arch = "wasm32")]
        // {
        //     spawn_local(async move {
        //         loop {
        //             logic.run();
        //         }
        //     });
        // }
        Self {
            ui,
            // logic_handle,
            config,
            sensor_data,
            settings,
        }
    }
}

impl App for AppState {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let config_guard = self.config.lock().unwrap();
        let sensor_data_guard = self.sensor_data.lock().unwrap();
        eframe::set_value(storage, crate::core::consts::KEY_CONFIG, &*config_guard);
        eframe::set_value(storage, crate::core::consts::KEY_DATA, &*sensor_data_guard);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui.run(ctx, _frame);
    }
}
