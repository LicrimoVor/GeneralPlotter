use super::{super::types::ExtractorTrait, Serial as ExtractorSerial, types::BaudRate};
use crate::{
    extractor::{
        serial::{SerialDevice, libs::update_ports},
        types::Event,
    },
    libs::print::print,
};
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

pub fn get_navigator_serial() -> Option<Serial> {
    let navigator = web_sys::window()?.navigator();
    let serial_value = Reflect::get(&navigator, &JsValue::from_str("serial")).ok()?;
    serial_value.dyn_into::<Serial>().ok()
}

impl ExtractorTrait for ExtractorSerial {
    async fn open(&mut self) -> Event {
        let baud_rate = self.baud_rate.value();
        if let Ok(ports) = update_ports(false).await {
            self.__ports = ports;
        } else {
            return Event::Opened(Err("Ошибка обновления портов".to_string()));
        };
        if self.is_opened() {
            return Event::Opened(Err("Порт уже открыт".to_string()));
        }
        let Some(selected_port) = &self.selected_port else {
            return Event::Opened(Err("Порт не выбран".to_string()));
        };

        let port = self.__ports.get(selected_port.id).unwrap();
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
                    return Event::Opened(Err("Ошибка открытия порта".to_string()));
                }
            },
            Err(_) => {
                print("Ошибка открытия порта");
                return Event::Opened(Err("Ошибка открытия порта".to_string()));
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
        Event::Opened(Ok(()))
    }

    async fn close(&mut self) -> Event {
        if !self.is_opened() {
            return Event::Opened(Err("Нет открытого порта для закрытия".to_string()));
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
        Event::Opened(Ok(()))
    }

    async fn send_data(&self, data: &[u8]) -> Event {
        if !self.is_opened() {
            return Event::Sended(Err("Нет открытого порта для чтения".to_string()));
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
        Event::Sended(Ok(()))
    }

    async fn read(&mut self) -> Event {
        if !self.is_opened() {
            return Event::Data(Err("Нет открытого порта для чтения".to_string()));
        }

        let reader_obj = self.__reader.as_ref().unwrap();
        let result = match JsFuture::from(Promise::from(reader_obj.read())).await {
            Ok(result) => result,
            Err(_) => {
                return Event::Data(Err("Ошибка чтения порта".to_string()));
            }
        };
        let done = match Reflect::get(&result, &JsValue::from_str("done")) {
            Ok(v) => v.as_bool().unwrap_or(false),
            Err(_) => {
                return Event::Data(Err("Ошибка чтения порта".to_string()));
            }
        };

        if done {
            if self.buffer.is_empty() {
                return Event::Data(Ok(vec![]));
            } else {
                let leftover = std::mem::take(&mut self.buffer);
                return Event::Data(Ok(vec![leftover]));
            }
        }

        let value = match Reflect::get(&result, &JsValue::from_str("value")) {
            Ok(v) => v,
            Err(_) => {
                return Event::Data(Err("Ошибка поля value при чтении порта".to_string()));
            }
        };
        if value.is_undefined() {
            return Event::Data(Ok(vec![]));
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

        Event::Data(Ok(lines))
    }
}
