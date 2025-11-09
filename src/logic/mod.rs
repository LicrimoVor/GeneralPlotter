pub mod config;
mod main;
mod serializer;
mod types;
pub use main::Logic;
use std::sync::{Arc, Mutex};
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

    {
        let mut config = config.lock().unwrap();
        while let Some(action) = config.actions.pop() {
            match action {
                config::Action::Reload => {
                    logic.reload(config.clone());
                }
            }
        }
    }
}
