use crate::libs::types::{LinierFunc, Value};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigLogic {
    pub linier_funcs: Vec<Option<LinierFunc>>,
    pub cols: Vec<Value>,
    pub delimiter: char,

    pub is_updated: bool,
    pub is_reload: bool,
}

impl Default for ConfigLogic {
    fn default() -> Self {
        Self {
            linier_funcs: Vec::new(),
            cols: Vec::new(),
            delimiter: ';',

            is_updated: false,
            is_reload: false,
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

    pub fn clear(&mut self) {
        // self.linier_funcs.clear();
        self.cols.clear();
    }
}
