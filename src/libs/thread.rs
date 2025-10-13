use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

pub fn start_thread(step: impl FnOnce() + 'static) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            loop {
                step();
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        spawn_local(async move {
            loop {
                step();
            }
        });
    }
}
