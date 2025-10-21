mod main;
mod serializer;
mod types;

pub mod config;
use std::sync::{Arc, Mutex};

pub use main::Logic;
pub use types::SensorData;

use crate::{
    libs::{
        mpsc,
        serials::{SerialAction, SerialEvent},
    },
    logic::config::ConfigLogic,
};

pub fn run_logic(
    logic: &mut Logic,
    config: &Arc<Mutex<ConfigLogic>>,
    serial_rx: &mut mpsc::Receiver<SerialEvent>,
    serial_tx: &mut mpsc::Sender<SerialAction>,
) {
    let event = serial_rx.try_recv();
    if let Some(event) = event {
        match event {
            SerialEvent::Data(result) => {
                if let Ok(data) = result {
                    for val in data {
                        logic.run(val.clone());
                        // print(val.as_str());
                    }
                }
            }
            SerialEvent::Opened(result) => match result {
                Ok(true) => logic.init(),
                _ => {}
            },
            _ => {}
        }
    }

    if config.lock().unwrap().is_reload {
        logic.reload();
    }
}
