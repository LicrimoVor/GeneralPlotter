use std::sync::mpsc;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigLogic {}

impl Default for ConfigLogic {
    fn default() -> Self {
        Self {}
    }
}

impl ConfigLogic {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::consts::KEY_CONFIG).unwrap_or_default()
        } else {
            ConfigLogic::default()
        }
    }
}

pub struct SensorData {
    pub sensor_rx: mpsc::Receiver<SensorData>,
    pub config_tx: mpsc::Sender<ConfigLogic>,
}
