use super::SerialDevice;
use super::types::BaudRate;

#[cfg(target_arch = "wasm32")]
pub mod wasm_impl {
    use super::*;
    use crate::libs::{print::print, serials::SerialEvent};
    use futures::future::join_all;
    use js_sys::{Array, Promise, Reflect, Uint8Array};
    use wasm_bindgen::{JsCast, prelude::*};
    use wasm_bindgen_futures::JsFuture;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = ReadableStreamDefaultReader)]
        pub type ReadableStreamDefaultReader;

        #[wasm_bindgen(method)]
        pub fn read(this: &ReadableStreamDefaultReader) -> js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn releaseLock(this: &ReadableStreamDefaultReader);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = Serial)]
        pub type Serial;

        #[wasm_bindgen(method, catch, js_name = getPorts)]
        pub fn get_ports(this: &Serial) -> Result<Promise, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub fn requestPort(this: &Serial) -> Result<Promise, JsValue>;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = SerialPort)]
        pub type SerialPort;

        #[wasm_bindgen(method, catch)]
        pub fn open(this: &SerialPort, options: &JsValue) -> Result<Promise, JsValue>;

        #[wasm_bindgen(method)]
        pub fn close(this: &SerialPort) -> Promise;

        #[wasm_bindgen(method, getter)]
        pub fn writable(this: &SerialPort) -> JsValue;

        #[wasm_bindgen(method, getter)]
        pub fn readable(this: &SerialPort) -> JsValue;

        #[wasm_bindgen(method)]
        pub fn getInfo(this: &SerialPort) -> JsValue;
    }

    pub fn get_navigator_serial() -> Option<Serial> {
        let navigator = web_sys::window()?.navigator();
        let serial_value = Reflect::get(&navigator, &JsValue::from_str("serial")).ok()?;
        serial_value.dyn_into::<Serial>().ok()
    }

    impl super::super::Serial {
        pub async fn update_ports(&mut self) -> SerialEvent {
            let mut devices = vec![];
            let serial = get_navigator_serial();
            if serial.is_none() {
                return SerialEvent::Ports(Err("Не получилось запросить serial".to_string()));
            }
            let serial = serial.unwrap();
            let _ = JsFuture::from(serial.requestPort().unwrap()).await;
            let ports_val = JsFuture::from(serial.get_ports().unwrap()).await.unwrap();
            let arr = Array::from(&ports_val);
            let ports_js: Vec<SerialPort> = arr
                .iter()
                .map(|a| a.unchecked_into::<SerialPort>())
                .collect();

            for (i, js_port) in arr.iter().enumerate() {
                if let Ok(port) = js_port.dyn_into::<SerialPort>() {
                    let info_val = port.getInfo();
                    let usb_vendor_id = Reflect::get(&info_val, &JsValue::from_str("usbVendorId"))
                        .ok()
                        .and_then(|v| v.as_f64().map(|n| n as u32));

                    let usb_product_id =
                        Reflect::get(&info_val, &JsValue::from_str("usbProductId"))
                            .ok()
                            .and_then(|v| v.as_f64().map(|n| n as u32));

                    let name: String = format!(
                        "{} - {}:{}",
                        i,
                        usb_vendor_id.unwrap_or(0),
                        usb_product_id.unwrap_or(0)
                    )
                    .chars()
                    .take(15)
                    .collect();

                    devices.push(SerialDevice {
                        name: name.clone(),
                        id: i,
                    });
                    print(format!("Порт {}", name).as_str());
                }
            }

            print("Порты обновлены");
            self.ports = devices;
            self.__ports = ports_js;
            SerialEvent::Ports(Ok(self.ports.clone()))
        }

        pub async fn open_port(&mut self, id: usize, baud_rate: BaudRate) -> SerialEvent {
            let baud_rate = baud_rate.value();
            if self.is_opened() {
                print("Порт уже открыт");
                return SerialEvent::Opened(Err("Порт уже открыт".to_string()));
            }

            let port = self.__ports.get(id).unwrap();
            let options = js_sys::Object::new();
            let _ = Reflect::set(
                &options,
                &JsValue::from_str("baudRate"),
                &JsValue::from_f64(baud_rate as f64),
            );
            match port.open(&JsValue::from(options)) {
                Ok(p) => match JsFuture::from(p).await {
                    Ok(_) => {}
                    Err(_) => {
                        print("Ошибка открытия порта");
                        return SerialEvent::Opened(Err("Ошибка открытия порта".to_string()));
                    }
                },
                Err(_) => {
                    print("Ошибка открытия порта");
                    return SerialEvent::Opened(Err("Ошибка открытия порта".to_string()));
                }
            };

            let reader = js_sys::Reflect::get(port, &JsValue::from_str("readable"))
                .unwrap()
                .unchecked_into::<web_sys::ReadableStream>();

            self.opened_port = Some(self.ports.get(id).unwrap().clone());
            self.__reader = Some(
                reader
                    .get_reader()
                    .dyn_into::<ReadableStreamDefaultReader>()
                    .unwrap(),
            );
            print("Порт подключен");
            SerialEvent::Opened(Ok(true))
        }

        pub async fn close_port(&mut self) -> SerialEvent {
            print("Закрываю порт");
            if !self.is_opened() {
                print("Нет открытого порта для чтения");
                return SerialEvent::Opened(Ok(false));
            }
            let port = self
                .__ports
                .get(self.opened_port.as_ref().unwrap().id)
                .unwrap();
            self.__reader.as_ref().unwrap().releaseLock();
            JsFuture::from(port.close()).await.unwrap();

            self.opened_port = None;
            self.__reader = None;
            print("Порт закрыт");
            SerialEvent::Opened(Ok(false))
        }

        pub async fn send_data(&self, data: &[u8]) -> SerialEvent {
            if !self.is_opened() {
                print("Нет открытого порта для чтения");
                return SerialEvent::Sended(Err("Нет открытого порта для чтения".to_string()));
            }
            if let Some(port) = &self.opened_port {
                let port = self.__ports.get(port.id).unwrap();
                let writer = js_sys::Reflect::get(port, &JsValue::from_str("writable"))
                    .unwrap()
                    .unchecked_into::<web_sys::WritableStream>();

                let data_vec = data.to_vec();
                let writer_obj = writer.get_writer().unwrap();
                let encoded = js_sys::Uint8Array::from(&data_vec[..]);
                JsFuture::from(writer_obj.write_with_chunk(&encoded))
                    .await
                    .unwrap();
                writer_obj.release_lock();
            }
            SerialEvent::Sended(Ok(true))
        }

        pub async fn read_data(&mut self) -> SerialEvent {
            if !self.is_opened() {
                print("Нет открытого порта для чтения");
                return SerialEvent::Data(Err("Нет открытого порта для чтения".to_string()));
            }

            let reader_obj = self.__reader.as_ref().unwrap();
            let result = match JsFuture::from(Promise::from(reader_obj.read())).await {
                Ok(result) => result,
                Err(_) => {
                    return SerialEvent::Data(Err("Ошибка чтения порта".to_string()));
                }
            };
            let done = match Reflect::get(&result, &JsValue::from_str("done")) {
                Ok(v) => v.as_bool().unwrap_or(false),
                Err(_) => {
                    return SerialEvent::Data(Err("Ошибка чтения порта".to_string()));
                }
            };

            if done {
                if self.buffer.is_empty() {
                    return SerialEvent::Data(Ok(vec![]));
                } else {
                    let leftover = std::mem::take(&mut self.buffer);
                    return SerialEvent::Data(Ok(vec![leftover]));
                }
            }

            let value = match Reflect::get(&result, &JsValue::from_str("value")) {
                Ok(v) => v,
                Err(_) => {
                    return SerialEvent::Data(
                        Err("Ошибка поля value при чтении порта".to_string()),
                    );
                }
            };
            if value.is_undefined() {
                return SerialEvent::Data(Ok(vec![]));
            }

            let array = Uint8Array::new(&value);
            let mut chunk = vec![0u8; array.length() as usize];
            array.copy_to(&mut chunk[..]);

            if let Ok(text) = String::from_utf8(chunk) {
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

        pub async fn send_event(&mut self, event: SerialEvent) {
            let futures = self
                .txs
                .iter_mut()
                .map(|tx| tx.send(event.clone()))
                .collect::<Vec<_>>();
            let _ = join_all(futures).await;
        }
    }
}
