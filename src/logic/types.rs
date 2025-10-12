use crate::libs::types::Value;
use egui_plot::PlotPoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct SensorData {
    serial_datas: Vec<Vec<String>>,
    parsed_datas: Vec<Vec<Value>>,
    times_windows: Vec<Vec<i32>>,
    times_serial: Vec<Vec<i32>>,

    #[serde(skip)]
    all_points: Vec<Vec<Vec<PlotPoint>>>,
}

impl SensorData {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::consts::KEY_DATA).unwrap_or_default()
        } else {
            SensorData::default()
        }
    }
}
