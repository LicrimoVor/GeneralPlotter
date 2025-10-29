use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SettingsPort {
    pub show_all: bool,
    pub is_auto_connect: bool,
}
