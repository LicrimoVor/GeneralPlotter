use crate::libs::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct SensorData {
    pub serial_datas: Vec<Vec<String>>,
    pub parsed_datas: Vec<Vec<Value>>,
    pub times_windows: Vec<Vec<i32>>,
    pub times_serial: Vec<Vec<i32>>,

    #[serde(skip)]
    pub all_points: Vec<Vec<Vec<[f64; 2]>>>,
}

impl SensorData {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::core::consts::KEY_DATA).unwrap_or_default()
        } else {
            SensorData::default()
        }
    }
}
