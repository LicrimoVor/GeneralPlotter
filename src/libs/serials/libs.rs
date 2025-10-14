use futures::SinkExt;
use futures::channel::mpsc;
use js_sys::{Promise, Reflect, Uint8Array};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};

pub struct SerialLineReader {
    reader_obj: js_sys::Object,
    buffer: Rc<RefCell<String>>,
    sender: mpsc::UnboundedSender<String>,
}

impl SerialLineReader {
    pub fn new(reader_obj: js_sys::Object, sender: mpsc::UnboundedSender<String>) -> Self {
        Self {
            reader_obj,
            buffer: Rc::new(RefCell::new(String::new())),
            sender,
        }
    }

    pub fn start_listening(self: Rc<Self>) {
        let reader_obj = self.reader_obj.clone();
        let buffer = self.buffer.clone();
        let mut sender = self.sender.clone();

        spawn_local(async move {
            loop {
                // Вызов reader.read()
                let read_fn = match Reflect::get(&reader_obj, &JsValue::from_str("read")) {
                    Ok(f) => f,
                    Err(e) => {
                        web_sys::console::error_1(&e);
                        break;
                    }
                };

                let read_promise = match read_fn.dyn_into::<js_sys::Function>() {
                    Ok(f) => f.call0(&reader_obj).unwrap(),
                    Err(_) => continue,
                };

                let result = match JsFuture::from(Promise::from(read_promise)).await {
                    Ok(r) => r,
                    Err(e) => {
                        web_sys::console::error_1(&e);
                        break;
                    }
                };

                let done = Reflect::get(&result, &JsValue::from_str("done"))
                    .ok()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if done {
                    break;
                }

                let value = Reflect::get(&result, &JsValue::from_str("value")).unwrap();
                if value.is_undefined() {
                    continue;
                }

                let array = Uint8Array::new(&value);
                let mut chunk = vec![0u8; array.length() as usize];
                array.copy_to(&mut chunk[..]);

                // Преобразуем в текст и добавляем в общий буфер
                if let Ok(text) = String::from_utf8(chunk) {
                    let mut buf = buffer.borrow_mut();
                    buf.push_str(&text);

                    // Разделяем на строки
                    while let Some(pos) = buf.find('\n') {
                        let line = buf[..pos].trim_end_matches(&['\r', '\n'][..]).to_string();

                        if sender.send(line).await.is_err() {
                            break;
                        }

                        // Оставляем остаток в буфере
                        let remainder = buf[pos + 1..].to_string();
                        *buf = remainder;
                    }
                }
            }
        });
    }
}
