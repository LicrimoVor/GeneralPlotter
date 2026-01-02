mod libs;
mod types;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod win;

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm as wasm_impl;

#[cfg(not(target_arch = "wasm32"))]
use win::desktop_impl::SerialPort;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SerialDevice {
    pub id: usize,
    pub name: String,
}

pub struct Serial {
    pub ports: Vec<SerialDevice>,
    pub opened_port: Option<SerialDevice>,
    pub loading: bool,

    buffer: String,
    pub selected_port: Option<SerialDevice>,
    pub baud_rate: types::BaudRate,

    #[cfg(not(target_arch = "wasm32"))]
    __opened_port: Option<Box<dyn SerialPort>>,

    #[cfg(target_arch = "wasm32")]
    __ports: Vec<(libs::SerialPort, SerialDevice)>,
    #[cfg(target_arch = "wasm32")]
    __reader: Option<wasm_impl::ReadableStreamDefaultReader>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            ports: vec![],
            loading: false,
            opened_port: None,
            selected_port: None,
            baud_rate: types::BaudRate::Baud115200,
            buffer: "".to_string(),

            #[cfg(not(target_arch = "wasm32"))]
            __opened_port: None,

            #[cfg(target_arch = "wasm32")]
            __reader: None,
            #[cfg(target_arch = "wasm32")]
            __ports: vec![],
        }
    }

    fn is_opened(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            return self.opened_port.is_some() && self.__opened_port.is_some();
        }

        #[cfg(target_arch = "wasm32")]
        {
            return self.opened_port.is_some() && self.__reader.is_some();
        }
    }
}
