use super::config::ConfigLogic;
use crate::{libs::types::Value, ui::settings::Settings};
use std::sync::{Arc, Mutex};

pub struct Serializer {
    config: Arc<Mutex<ConfigLogic>>,
    settings: Arc<Mutex<Settings>>,
}

impl Serializer {
    pub fn new(config: Arc<Mutex<ConfigLogic>>, settings: Arc<Mutex<Settings>>) -> Self {
        Self { config, settings }
    }

    pub fn run(&mut self, data: String) -> (Vec<String>, Vec<Value>) {
        let delimiter = self.settings.lock().unwrap().delimiter;

        let serial = data
            .split(delimiter)
            .map(|val| val.to_string())
            .collect::<Vec<String>>();

        let parsed = serial
            .iter()
            .map(
                |val| match val.replace(" ", ".").replace(",", ".").parse::<f64>() {
                    Ok(numb) => Value::Number(numb),
                    _ => Value::Text(val.clone()),
                },
            )
            .collect::<Vec<Value>>();

        (serial, parsed)
    }
}
