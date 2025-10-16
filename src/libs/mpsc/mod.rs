#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(not(target_arch = "wasm32"))]
mod win;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
pub use win::*;

pub struct Sender<T> {
    tx: mpsc::Sender<T>,
}

pub struct Receiver<T> {
    rx: mpsc::Receiver<T>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = channel_impl::<T>();

    return (Sender { tx }, Receiver { rx });
}
