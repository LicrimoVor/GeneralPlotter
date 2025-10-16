#[cfg(target_arch = "wasm32")]
pub async fn sleep_ms(ms: i32) {
    use js_sys::Promise;
    use wasm_bindgen_futures::JsFuture;

    let promise = Promise::new(&mut |resolve, _| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    });
    let _ = JsFuture::from(promise).await;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn sleep_ms(ms: i32) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}
