use super::{Receiver, Sender};
pub use futures::channel::mpsc;
use futures::{SinkExt, StreamExt, sink::Send};

impl<T> Sender<T> {
    pub fn send(&mut self, value: T) -> Send<'_, mpsc::Sender<T>, T> {
        self.tx.send(value)
    }

    pub fn try_send(&mut self, value: T) -> Result<(), mpsc::TrySendError<T>> {
        self.tx.try_send(value)
    }
}

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.rx.next().await
    }

    pub fn try_recv(&mut self) -> Option<T> {
        self.rx.try_next().ok().flatten()
    }
}

pub fn channel_impl<T>() -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    mpsc::channel::<T>(100)
}
