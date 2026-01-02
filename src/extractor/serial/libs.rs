use super::{SerialDevice, wasm::get_navigator_serial};
use crate::libs::print::print;
use js_sys::{Array, Promise, Reflect};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

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

pub async fn update_ports(request: bool) -> Result<Vec<(SerialPort, SerialDevice)>, String> {
    let mut devices = vec![];
    let serial = get_navigator_serial();
    if serial.is_none() {
        return Err("Не получилось запросить serial".to_string());
    }
    let serial = serial.unwrap();
    if request {
        let _ = JsFuture::from(serial.requestPort().unwrap()).await;
    }

    let ports_val = JsFuture::from(serial.get_ports().unwrap()).await.unwrap();
    let arr = Array::from(&ports_val);

    for (i, js_port) in arr.iter().enumerate() {
        if let Ok(port) = js_port.dyn_into::<SerialPort>() {
            let info_val = port.getInfo();
            let usb_vendor_id = Reflect::get(&info_val, &JsValue::from_str("usbVendorId"))
                .ok()
                .and_then(|v| v.as_f64().map(|n| n as u32));

            let usb_product_id = Reflect::get(&info_val, &JsValue::from_str("usbProductId"))
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

            devices.push((
                port,
                SerialDevice {
                    name: name.clone(),
                    id: i,
                },
            ));
            print(format!("Порт {}", name).as_str());
        }
    }

    print("Порты обновлены");
    Ok(devices)
}
