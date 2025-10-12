#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigLogic {
    pub lol: String,
}

impl Default for ConfigLogic {
    fn default() -> Self {
        Self {
            lol: "123123".to_string(),
        }
    }
}

impl ConfigLogic {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::core::consts::KEY_CONFIG).unwrap_or_default()
        } else {
            ConfigLogic::default()
        }
    }
}
