mod extractor;
mod logger;
mod serializer;
mod state;
mod traits;
mod types;

pub mod config;
pub use state::Logic;
pub use types::SensorData;

use crate::libs::{
    mpsc,
    serials::{SerialAction, SerialEvent},
};

pub fn run_logic(
    logic: &mut Logic,
    serial_rx: &mut mpsc::Receiver<SerialEvent>,
    serial_tx: &mut mpsc::Sender<SerialAction>,
) {
    let event = serial_rx.try_recv();
    if event.is_none() {
        return;
    }

    let event = event.unwrap();
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
