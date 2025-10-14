use super::SerialDevice;

#[cfg(not(target_arch = "wasm32"))]
pub mod desktop_impl {
    use super::*;
    use serialport::{SerialPort, SerialPortInfo, available_ports};
    use std::sync::Mutex;

    static mut GLOBAL_PORT: Option<Mutex<Box<dyn SerialPort>>> = None;

    pub fn get_ports_impl() -> Vec<SerialDevice> {
        available_ports()
            .unwrap_or_default()
            .into_iter()
            .map(|p: SerialPortInfo| SerialDevice { name: p.port_name })
            .collect()
    }

    pub fn open_port_impl(name: &str, baud_rate: u32) -> Result<(), String> {
        match serialport::new(name, baud_rate).open() {
            Ok(port) => {
                unsafe { GLOBAL_PORT = Some(Mutex::new(port)) };
                Ok(())
            }
            Err(e) => Err(format!("Не удалось открыть порт {}: {}", name, e)),
        }
    }

    pub fn close_port_impl() {
        unsafe { GLOBAL_PORT = None };
    }

    pub fn send_data_impl(data: &[u8]) -> Result<(), String> {
        unsafe {
            if let Some(ref m) = GLOBAL_PORT {
                let mut port = m.lock().unwrap();
                port.write_all(data).map_err(|e| e.to_string())
            } else {
                Err("Порт не открыт".to_string())
            }
        }
    }

    pub fn read_data_impl() -> Result<Vec<u8>, String> {
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
