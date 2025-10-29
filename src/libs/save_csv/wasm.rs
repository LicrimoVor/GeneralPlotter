use wasm_bindgen::prelude::*;
use web_sys::{Blob, Url};

#[wasm_bindgen]
pub fn save_csv(filename: &str, content: &str) -> Result<(), JsValue> {
    // 1. Создаём Blob из данных
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(content));
    let blob = Blob::new_with_str_sequence(&parts)?;

    // 2. Создаём URL для Blob
    let url = Url::create_object_url_with_blob(&blob)?;

    // 3. Создаём <a> элемент и “симулируем” клик
    let document = web_sys::window().unwrap().document().unwrap();
    let a = document
        .create_element("a")?
        .dyn_into::<web_sys::HtmlElement>()?;

    a.set_attribute("href", &url)?;
    a.set_attribute("download", filename)?;
    a.set_attribute("style", "display:none")?;
    document.body().unwrap().append_child(&a)?;
    a.click();

    // 4. Чистим за собой
    Url::revoke_object_url(&url)?;
    a.remove();

    Ok(())
}
