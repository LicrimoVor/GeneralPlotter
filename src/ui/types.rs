use crate::libs::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct UiData {
    pub messages: Vec<Message>,
}

impl UiData {
    pub fn new(storage: Option<&dyn eframe::Storage>) -> Self {
        if let Some(storage) = storage {
            eframe::get_value(storage, crate::core::consts::KEY_DATA).unwrap_or_default()
        } else {
            UiData::default()
        }
    }

    pub fn update(&mut self, message: String) {
        self.messages.push(Message::info(message));
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
