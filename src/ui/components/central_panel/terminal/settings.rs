use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::libs::message::MessageType;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsTerminal {
    pub show_time: bool,
    pub show_id: bool,
    pub id_selectable: bool,
    pub time_selectable: bool,

    pub show_msg_info: bool,
    pub msg_info_color: [u8; 3],
    pub show_msg_error: bool,
    pub msg_error_color: [u8; 3],
    pub show_msg_fetch: bool,
    pub msg_fetch_color: [u8; 3],

    pub mode_table: bool,
    pub show_separator: bool,
    pub count_msg: u32,
}

impl Default for SettingsTerminal {
    fn default() -> Self {
        Self {
            show_time: true,
            show_id: true,
            id_selectable: false,
            time_selectable: false,

            show_msg_info: true,
            msg_info_color: [255, 190, 92],

            show_msg_error: true,
            msg_error_color: [255, 0, 0],

            show_msg_fetch: true,
            msg_fetch_color: [0, 255, 0],

            mode_table: false,
            show_separator: false,
            count_msg: 500,
        }
    }
}

impl SettingsTerminal {
    pub fn get_color(&self, msg_type: &MessageType) -> Color32 {
        match msg_type {
            MessageType::Info => Color32::from_rgb(
                self.msg_info_color[0],
                self.msg_info_color[1],
                self.msg_info_color[2],
            ),
            MessageType::Error => Color32::from_rgb(
                self.msg_error_color[0],
                self.msg_error_color[1],
                self.msg_error_color[2],
            ),
            MessageType::Fetch => Color32::from_rgb(
                self.msg_fetch_color[0],
                self.msg_fetch_color[1],
                self.msg_fetch_color[2],
            ),
        }
    }

    pub fn get_is_show(&self, msg_type: &MessageType) -> bool {
        match msg_type {
            MessageType::Info => self.show_msg_info,
            MessageType::Error => self.show_msg_error,
            MessageType::Fetch => self.show_msg_fetch,
        }
    }
}
