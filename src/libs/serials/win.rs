use super::{Serial, SerialDevice, SerialEvent};
use crate::libs::serials::BaudRate;
use std::sync::Mutex;

#[cfg(not(target_arch = "wasm32"))]
pub mod desktop_impl {

    use super::*;
    use serialport::available_ports;

    impl Serial {
        pub fn update_ports(&mut self) -> SerialEvent {
            self.ports = available_ports()
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .map(|(i, p)| SerialDevice {
                    id: i,
                    name: p.port_name,
                })
                .collect();
            SerialEvent::Ports(Ok(self.ports.clone()))
        }

        pub fn open_port(&mut self, id: usize, baud_rate: BaudRate) -> SerialEvent {
            match serialport::new(name, baud_rate).open() {
                Ok(port) => {
                    unsafe { GLOBAL_PORT = Some(Mutex::new(port)) };
                    Ok(())
                }
                Err(e) => Err(format!("Не удалось открыть порт {}: {}", name, e)),
            }
        }

        pub fn close_port(&mut self) -> SerialEvent {
            unsafe { GLOBAL_PORT = None };
        }

        pub fn send_data(&self, data: &[u8]) -> SerialEvent {
            unsafe {
                if let Some(ref m) = GLOBAL_PORT {
                    let mut port = m.lock().unwrap();
                    port.write_all(data).map_err(|e| e.to_string())
                } else {
                    Err("Порт не открыт".to_string())
                }
            }
        }

        pub fn read_data(&mut self) -> SerialEvent {
            unsafe {
                if let Some(ref m) = GLOBAL_PORT {
                    let mut port = m.lock().unwrap();
                    let mut buf = vec![0u8; 1024];
                    match port.read(&mut buf) {
                        Ok(size) => Ok(buf[..size].to_vec()),
                        Err(e) => Err(e.to_string()),
                    }
                } else {
                    Err("Порт не открыт".to_string())
                }
            }
        }
    }
}
