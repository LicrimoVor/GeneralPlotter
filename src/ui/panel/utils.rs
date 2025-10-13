use serialport::{SerialPortInfo, SerialPortType, available_ports};

#[cfg(not(target_arch = "wasm32"))]
pub fn get_ports() -> Vec<SerialPortInfo> {
    serialport::available_ports()
        .expect("Не удалось получить список портов")
        .into_iter()
        .filter(|port| matches!(port.port_type, SerialPortType::UsbPort(_)))
        .collect()
}

#[cfg(target_arch = "wasm32")]
pub fn get_ports() -> Vec<SerialPortInfo> {
    vec![]
}
