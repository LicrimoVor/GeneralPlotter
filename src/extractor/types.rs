use super::extractor::ExtractorType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum EventError {
    TimeOut,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Event {
    Loading(Result<bool, EventError>),
    Data(Result<Vec<String>, EventError>),
    Updated(Result<(), EventError>),
    Opened(Result<(), EventError>),
    Sended(Result<(), EventError>),
}

pub enum Action {
    Update,
    Open(ExtractorType),
    ClosePort,
    SendData(String),

    SetInterval(i32),
}

#[cfg(target_arch = "wasm32")]
pub(super) trait ExtractorTrait {
    async fn open(&mut self) -> Event;
    async fn close(&mut self) -> Event;
    async fn send_data(&self, data: &[u8]) -> Event;
    async fn read(&mut self) -> Event;
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) trait ExtractorTrait {
    fn update(&mut self) -> Event;
    fn open(&mut self) -> Event;
    fn close(&mut self) -> Event;
    fn send_data(&mut self) -> Event;
}
