use super::SerialDevice;
use super::libs::SerialLineReader;
use super::types::BaudRate;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
pub mod wasm_impl {

    use super::*;
    use futures::channel::mpsc::{self, UnboundedReceiver};
    use js_sys::Reflect;
    use js_sys::{Array, Promise, Uint8Array};
    use wasm_bindgen::JsCast;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::console;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = navigator, js_name = serial)]
        pub static SERIAL: Serial;

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

    impl super::super::Serial {
        pub fn update_ports(&mut self, self_rc: Rc<RefCell<Self>>) {
            self.loading = true;

            wasm_bindgen_futures::spawn_local(async move {
                let mut devices = vec![];

                let _ = JsFuture::from(SERIAL.requestPort().unwrap()).await;
                let ports_val = JsFuture::from(SERIAL.get_ports().unwrap()).await.unwrap();
                let arr = Array::from(&ports_val);
                let ports_js: Vec<SerialPort> = arr
                    .iter()
                    .map(|a| a.unchecked_into::<SerialPort>())
                    .collect();

                for (i, js_port) in arr.iter().enumerate() {
                    if let Ok(port) = js_port.dyn_into::<SerialPort>() {
                        let info_val = port.getInfo();
                        let usb_vendor_id =
                            Reflect::get(&info_val, &JsValue::from_str("usbVendorId"))
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
                        console::log_1(&format!("Порт {}", name).into());
                    }
                }
                let mut s = self_rc.borrow_mut();

                console::log(&arr);
                console::log_1(&"Порты обновлены".into());
                s.ports = devices;
                s.__ports = ports_js;
                s.loading = false;
            });
        }

        pub fn open_port(
            &mut self,
            id: usize,
            baud_rate: BaudRate,
            self_rc: Rc<RefCell<Self>>,
        ) -> Option<UnboundedReceiver<String>> {
            let baud_rate = baud_rate.value();
            let (tx, mut rx) = mpsc::unbounded::<String>();

            if self.opened_port.is_some() {
                console::log_1(&"Порт уже открыт".into());
                return None;
            }

            wasm_bindgen_futures::spawn_local(async move {
                let mut self_clone = self_rc.borrow_mut();
                let port = self_clone.__ports.get(id).unwrap();
                let options = js_sys::Object::new();
                let _ = Reflect::set(
                    &options,
                    &JsValue::from_str("baudRate"),
                    &JsValue::from_f64(baud_rate as f64),
                );
                match port.open(&JsValue::from(options)) {
                    Ok(p) => {
                        JsFuture::from(p).await.unwrap();
                        console::log_1(&"Порт открыт".into());
                    }
                    Err(er) => {
                        console::log_1(&"Ошибка открытия порта".into());
                        return;
                    }
                };

                let reader = js_sys::Reflect::get(port, &JsValue::from_str("readable"))
                    .unwrap()
                    .unchecked_into::<web_sys::ReadableStream>();

                self_clone.opened_port = Some(self_clone.ports.get(id).unwrap().clone());
                self_clone.reader = Some(SerialLineReader::new(reader.get_reader(), tx));
                let reader_obj = reader.get_reader();
                // let read_promise = Reflect::get(&reader_obj, &JsValue::from_str("read"))
                //     .unwrap()
                //     .dyn_into::<js_sys::Function>()
                //     .unwrap()
                //     .call0(&reader_obj)
                //     .unwrap();
                // let result = JsFuture::from(js_sys::Promise::from(read_promise))
                //     .await
                //     .unwrap();
                // console::log_1(&result);
                console::log_1(&"Порт подключен".into());
            });

            return Some(rx);
        }

        /// Закрыть порт
        pub fn close_port(&mut self) {
            self.opened_port = None;
            // self.reader.unwrap().get_reader().
            self.reader = None;
            console::log_1(&"Порт закрыт".into());
        }

        // pub fn send_data(&self, data: &[u8]) {
        //     if let Some(port) = &self.opened_port {
        //         let writer = js_sys::Reflect::get(port, &JsValue::from_str("writable"))
        //             .unwrap()
        //             .unchecked_into::<web_sys::WritableStream>();

        //         let data_vec = data.to_vec();
        //         wasm_bindgen_futures::spawn_local(async move {
        //             let writer_obj = writer.get_writer().unwrap();
        //             let encoded = js_sys::Uint8Array::from(&data_vec[..]);
        //             writer_obj.write_with_chunk(&encoded).unwrap();
        //             writer_obj.release_lock();
        //         });
        //     } else {
        //         console::log_1(&"Нет открытого порта для отправки".into());
        //     }
        // }

        // Асинхронное чтение данных (через callback или кэш)
        // pub fn read_data(&mut self, self_rc: Rc<RefCell<Self>>) {
        //     if self.opened_port.is_none() || self.reader.is_none() {
        //         console::log_1(&"Нет открытого порта для чтения".into());
        //         return;
        //     }

        //     wasm_bindgen_futures::spawn_local(async move {
        //         let self_clone = self_rc.borrow();
        //         let reader_obj = self_clone.reader.as_ref().unwrap();
        //         loop {
        //             let result = read_line(&reader_obj).await.unwrap();
        //             console::log_1(&result.unwrap().into());
        //         }
        //     });
        // }
    }
}
