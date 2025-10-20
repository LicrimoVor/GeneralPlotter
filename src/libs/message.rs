use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static ID: AtomicU64 = AtomicU64::new(0);
#[derive(Serialize, Deserialize, Clone)]
pub enum MessageType {
    Info,
    Error,
    Fetch,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: u64,
    created: i64,
    create_paresed: String,
    pub text: String,
    pub r#type: MessageType,
}

impl Message {
    fn new(text: String, r#type: MessageType) -> Self {
        let id = ID.fetch_add(1, Ordering::Relaxed);
        let created = chrono::Utc::now().timestamp_millis();

        let hour = created / 1000 / 60 / 60 % 24;
        let minute = (created / 1000 / 60) % 60;
        let second = (created / 1000) % 60;
        let millis = created % 1000;

        Message {
            id,
            created,
            create_paresed: format!("{:02}:{:02}:{:02}.{:04}", hour, minute, second, millis),
            text,
            r#type,
        }
    }

    pub fn info(text: String) -> Self {
        Message::new(text, MessageType::Info)
    }

    pub fn error(text: String) -> Self {
        Message::new(text, MessageType::Error)
    }

    pub fn fetch(text: String) -> Self {
        Message::new(text, MessageType::Fetch)
    }

    pub fn get_created(&self) -> String {
        self.create_paresed.clone()
    }
}
