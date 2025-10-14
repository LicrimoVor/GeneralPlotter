mod libs;
mod types;
mod wasm;
mod win;

use futures::channel::mpsc;
use serde::{Deserialize, Serialize};
pub use types::BaudRate;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(not(target_arch = "wasm32"))]
use win::desktop_impl::*;

#[cfg(target_arch = "wasm32")]
use wasm::wasm_impl::*;

#[cfg(target_arch = "wasm32")]
use crate::libs::serials::wasm::wasm_impl;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SerialDevice {
    pub id: usize,
    pub name: String,
}

pub struct Serial {
    pub ports: Vec<SerialDevice>,
    pub opened_port: Option<SerialDevice>,

    pub baud_rate: BaudRate,
    pub loading: bool,

    #[cfg(target_arch = "wasm32")]
    pub __ports: Vec<wasm_impl::SerialPort>,

    #[cfg(target_arch = "wasm32")]
    reader: Option<libs::SerialLineReader>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            ports: vec![],
            baud_rate: BaudRate::Baud9600,
            loading: false,
            opened_port: None,

            #[cfg(target_arch = "wasm32")]
            reader: None,
            #[cfg(target_arch = "wasm32")]
            __ports: vec![],
        }
    }

    pub fn is_opened(&self) -> bool {
        self.opened_port.is_some() && self.reader.is_some()
    }
}
