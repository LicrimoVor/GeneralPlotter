use crate::libs::{serials::Serial, sleep::sleep_ms};
use crate::logic::{Logic, SensorData, config::ConfigLogic, run_logic};
use crate::ui::UiData;
use crate::ui::{UserInterface, settings::Settings};
use eframe::App;
use std::sync::{Arc, Mutex};

pub struct AppState {
    ui: UserInterface,
    settings: Arc<Mutex<Settings>>,
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
    ui_data: Arc<Mutex<UiData>>,
}

impl AppState {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        let config = Arc::new(Mutex::new(ConfigLogic::new(storage)));
        let sensor_data = Arc::new(Mutex::new(SensorData::new(storage)));
        let settings = Arc::new(Mutex::new(Settings::new(storage)));
        let ui_data = Arc::new(Mutex::new(UiData::new(storage)));

        let mut serial = Serial::new();
        let (mut serial_rx, mut serial_tx) = serial.subscribe();
        let mut logic = Logic::new(config.clone(), sensor_data.clone());
        let ui = UserInterface::new(
            config.clone(),
            sensor_data.clone(),
            settings.clone(),
            ui_data.clone(),
            &mut serial,
        );

        #[cfg(not(target_arch = "wasm32"))]
        {
            let settings_clone = settings.clone();
            std::thread::spawn(move || {
                loop {
                    sleep_ms(settings_clone.lock().unwrap().time_step_ms);
                    run_logic(&mut logic, &mut serial_rx, &mut serial_tx);
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                loop {
                    sleep_ms(25).await;
                    run_logic(&mut logic, &mut serial_rx, &mut serial_tx)
                }
            });
        }
        serial.spawn_loop();
        Self {
            ui,
            config,
            sensor_data,
            settings,
            ui_data,
        }
    }
}

impl App for AppState {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let config_guard = self.config.lock().unwrap();
        let sensor_data_guard = self.sensor_data.lock().unwrap();
        let settings_guard = self.settings.lock().unwrap();
        let ui_data_guard = self.ui_data.lock().unwrap();
        eframe::set_value(storage, crate::core::consts::KEY_CONFIG, &*config_guard);
        eframe::set_value(storage, crate::core::consts::KEY_DATA, &*sensor_data_guard);
        eframe::set_value(storage, crate::core::consts::KEY_SETTINGS, &*settings_guard);
        eframe::set_value(storage, crate::core::consts::KEY_UI_DATA, &*ui_data_guard);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
        self.ui.run(ctx, _frame);
    }
}
