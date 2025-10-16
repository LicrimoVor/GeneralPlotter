use super::{Receiver, Sender};
pub use std::sync::mpsc;

impl<T> Sender<T> {
    pub fn send(&mut self, value: T) -> Result<(), mpsc::TrySendError<T>> {
        Ok(self.tx.send(value)?)
    }

    pub fn try_send(&mut self, value: T) -> Result<(), mpsc::TrySendError<T>> {
        Ok(self.tx.send(value)?)
    }
}

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        if let Ok(value) = self.rx.recv() {
            return Some(value);
        } else {
            return None;
        }
    }

    pub fn try_recv(&mut self) -> Option<T> {
        if let Ok(value) = self.rx.try_recv() {
            return Some(value);
        } else {
            return None;
        }
    }
}

pub fn channel_impl<T>() -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    mpsc::channel::<T>()
}
