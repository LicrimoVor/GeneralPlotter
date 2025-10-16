use crate::core::settings::Settings;
use crate::libs::types::{LinierFunc, Value};
use crate::logic::SensorData;
use crate::ui::ConfigLogic;
use chrono::Utc;
use std::sync::{Arc, Mutex};

struct LogicState {
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
}

pub struct Logic {
    settings: Arc<Mutex<Settings>>,
    state: LogicState,
    // extractor: Option<dyn Any>,
    // logger: Option<dyn Any>,
    // serializer: Option<dyn Any>,
    funcs: Vec<LinierFunc>,

    __t_start: f64,
}

impl Logic {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
    ) -> Self {
        Self {
            state: LogicState {
                config,
                sensor_data,
            },
            funcs: vec![],
            settings,

            __t_start: Utc::now().timestamp_millis() as f64 / 1000.0,
        }
    }
    fn update(&mut self) {
        // while let Ok(config) = self.state.config.lock() {}
        // let sensor_data = self.state.sensor_data.lock().unwrap();
    }

    pub fn init(&mut self) {
        self.state.sensor_data.lock().unwrap().clear();
        self.__t_start = Utc::now().timestamp_millis() as f64 / 1000.0;
    }

    pub fn run(&mut self, data: String) {
        self.update();

        let t_win = Utc::now().timestamp_millis() as f64 / 1000.0 - self.__t_start;
        let t_serial = None;

        let serial = data
            .split(';')
            .map(|val| val.to_string())
            .collect::<Vec<String>>();
        let parsed = serial
            .iter()
            .map(|val| match val.parse::<f64>() {
                Ok(numb) => Value::Number(numb),
                _ => Value::Text(val.clone()),
            })
            .collect::<Vec<Value>>();

        self.state
            .sensor_data
            .lock()
            .unwrap()
            .add_data(serial, parsed, t_win, t_serial);

        // println!("run logic");
    }
}
