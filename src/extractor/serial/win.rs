#[cfg(not(target_arch = "wasm32"))]
pub mod desktop_impl {
    use super::*;

    pub use serialport::SerialPort;
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
            let port = &self.ports[id];
            match serialport::new(port.name.clone(), baud_rate.value()).open() {
                Ok(port) => {
                    self.__opened_port = Some(port);
                    SerialEvent::Opened(Ok(true))
                }
                Err(_) => SerialEvent::Opened(Err("Не удалось открыть порт".to_string())),
            }
        }

        pub fn close_port(&mut self) -> SerialEvent {
            self.__opened_port = None;
            SerialEvent::Opened(Ok(false))
        }

        pub fn send_data(&mut self, data: &[u8]) -> SerialEvent {
            if let Some(port) = &mut self.__opened_port {
                match port.write_all(data) {
                    Ok(_) => SerialEvent::Sended(Ok(true)),
                    Err(e) => SerialEvent::Sended(Err(e.to_string())),
                }
            } else {
                SerialEvent::Sended(Err("Порт не открыт".to_string()))
            }
        }

        pub fn read_data(&mut self) -> SerialEvent {
            let mut buf = vec![0u8; 1024];
            if let Some(port) = &mut self.__opened_port {
                match port.read(&mut buf) {
                    Ok(size) => Ok(buf[..size].to_vec()),
                    Err(e) => Err(e.to_string()),
                }
            } else {
                Err("Порт не открыт".to_string())
            };

            if let Ok(text) = String::from_utf8(buf) {
                self.buffer.push_str(&text);
            }

            let mut lines = Vec::new();
            while let Some(pos) = self.buffer.find('\n') {
                let line = self.buffer[..pos]
                    .trim_end_matches(&['\r', '\n'][..])
                    .to_string();
                lines.push(line);
                self.buffer = self.buffer[pos + 1..].to_string();
            }

            SerialEvent::Data(Ok(lines))
        }

        pub fn send_event(&mut self, event: SerialEvent) {
            let _ = self
                .txs
                .iter_mut()
                .map(|tx| tx.send(event.clone()))
                .collect::<Vec<_>>();
        }
    }
}
