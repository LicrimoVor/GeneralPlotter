#[cfg(target_arch = "wasm32")]
pub fn print(text: &str) {
    web_sys::console::log_1(&text.into());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print(text: &str) {
    println!("{}", text);
}
