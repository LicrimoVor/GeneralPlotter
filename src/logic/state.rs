use super::config::ConfigLogic;
use crate::libs::types::{LinierFunc, Value};
use crate::logic::SensorData;
use chrono::Utc;
use std::sync::{Arc, Mutex};

pub struct Logic {
    // settings: Arc<Mutex<Settings>>,
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,

    // extractor: Option<dyn Any>,
    // logger: Option<dyn Any>,
    // serializer: Option<dyn Any>,
    __t_start: f64,
}

impl Logic {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        // settings: Arc<Mutex<Settings>>,
    ) -> Self {
        Self {
            config,
            sensor_data,
            // settings,
            __t_start: Utc::now().timestamp_millis() as f64 / 1000.0,
        }
    }
    fn update(&mut self) {
        // while let Ok(config) = self.state.config.lock() {}
        // let sensor_data = self.state.sensor_data.lock().unwrap();
    }

    pub fn init(&mut self) {
        self.sensor_data.lock().unwrap().clear();
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
        if self.config.lock().unwrap().cols.len() < parsed.len() {
            let mut config = self.config.lock().unwrap();
            config.cols = parsed.clone();
            config.linier_funcs = (0..parsed.len())
                .map(|_| LinierFunc::default())
                .collect::<Vec<LinierFunc>>();
        }
        self.sensor_data
            .lock()
            .unwrap()
            .add_data(serial, parsed, t_win, t_serial);

        // println!("run logic");
    }
}
