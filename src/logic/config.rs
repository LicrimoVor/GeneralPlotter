use crate::libs::types::{LinierFunc, Value};

pub enum Action {
    Reload,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ConfigLogic {
    pub cols: Vec<Value>,

    #[serde(skip)]
    pub actions: Vec<Action>,

    pub linier_funcs: Vec<Option<LinierFunc>>,
    pub delimiter: char,

    pub is_updated: bool,
}

impl Default for ConfigLogic {
    fn default() -> Self {
        Self {
            cols: Vec::new(),
            actions: vec![],

            linier_funcs: Vec::new(),
            delimiter: ';',

            is_updated: false,
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
