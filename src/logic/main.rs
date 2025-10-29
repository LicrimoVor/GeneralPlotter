use super::SensorData;
use super::config::ConfigLogic;
use super::serializer::Serializer;
use crate::{
    libs::{
        print,
        types::{LinierFunc, Value},
    },
    ui::settings::Settings,
};
use chrono::Utc;
use std::sync::{Arc, Mutex};

pub struct Logic {
    settings: Arc<Mutex<Settings>>,
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,

    serializer: Serializer,
    __t_start: f64,
}

impl Logic {
    pub fn new(
        config: Arc<Mutex<ConfigLogic>>,
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
    ) -> Self {
        Self {
            config: config.clone(),
            sensor_data: sensor_data.clone(),
            settings: settings.clone(),

            serializer: Serializer::new(config.clone(), settings.clone()),
            __t_start: Utc::now().timestamp_millis() as f64 / 1000.0,
        }
    }

    pub fn init(&mut self) {
        self.sensor_data.lock().unwrap().clear();
        self.__t_start = Utc::now().timestamp_millis() as f64 / 1000.0;
    }

    pub fn run(&mut self, data: String) {
        let (serial, original) = self.serializer.run(data);
        let t_win = Utc::now().timestamp_millis() as f64 / 1000.0 - self.__t_start;
        let t_serial = {
            let time_serial_col = self.settings.lock().unwrap().time_serial_col;
            let mut t_serial = None;
            if original.len() > time_serial_col {
                match original[time_serial_col] {
                    Value::Number(n) => t_serial = Some(n),
                    _ => (),
                }
            }
            t_serial
        };

        let parsed = {
            let mut config = self.config.lock().unwrap();
            if config.cols.len() < original.len() {
                config.cols = original.clone();
                let mut i: usize = 0;
                config.linier_funcs = original
                    .iter()
                    .enumerate()
                    .map(|(k, v)| match v {
                        Value::Number(_) => {
                            let mut linier = None;
                            for j in i..=k {
                                i += 1;
                                if j >= config.linier_funcs.len() {
                                    break;
                                }

                                let linier_f = &config.linier_funcs[j];
                                if linier_f.is_none() {
                                    continue;
                                } else {
                                    linier = linier_f.clone();
                                    break;
                                }
                            }
                            if linier.is_none() {
                                Some(LinierFunc::default())
                            } else {
                                linier
                            }
                        }
                        _ => None,
                    })
                    .collect::<Vec<Option<LinierFunc>>>();
            }

            SensorData::apply_linier(&original, &config.linier_funcs)
        };

        self.sensor_data
            .lock()
            .unwrap()
            .add_data(serial, original, parsed, t_win, t_serial);
    }

    pub fn reload(&mut self) {
        let config = self.config.lock().unwrap();
        self.sensor_data
            .lock()
            .unwrap()
            .reload(&config.linier_funcs);
        self.settings.lock().unwrap().is_updated = true;
    }
}
