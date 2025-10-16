mod types;
mod wasm;
mod win;

use futures::{SinkExt, channel::mpsc, future::join_all};
use serde::{Deserialize, Serialize};
pub use types::BaudRate;
#[cfg(target_arch = "wasm32")]
use wasm::wasm_impl;
#[cfg(not(target_arch = "wasm32"))]
use win::desktop_impl;

use crate::libs::sleep::sleep_ms;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SerialDevice {
    pub id: usize,
    pub name: String,
}

pub enum SerialAction {
    UpdatePorts,
    OpenPort((SerialDevice, BaudRate)),
    ClosePort,
    SendData(String),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SerialEvent {
    Loading(Result<bool, String>),
    Ports(Result<Vec<SerialDevice>, String>),
    Data(Result<Vec<String>, String>),
    Opened(Result<bool, String>),
    Sended(Result<bool, String>),
}

pub struct Serial {
    pub ports: Vec<SerialDevice>,
    pub opened_port: Option<SerialDevice>,
    pub loading: bool,

    buffer: String,
    txs: Vec<mpsc::Sender<SerialEvent>>,
    rxs: Vec<mpsc::Receiver<SerialAction>>,

    #[cfg(target_arch = "wasm32")]
    __ports: Vec<wasm_impl::SerialPort>,
    #[cfg(target_arch = "wasm32")]
    __reader: Option<wasm_impl::ReadableStreamDefaultReader>,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            ports: vec![],
            loading: false,
            opened_port: None,
            buffer: "".to_string(),

            txs: vec![],
            rxs: vec![],

            #[cfg(target_arch = "wasm32")]
            __reader: None,
            #[cfg(target_arch = "wasm32")]
            __ports: vec![],
        }
    }

    pub fn subscribe(&mut self) -> (mpsc::Receiver<SerialEvent>, mpsc::Sender<SerialAction>) {
        let (tx_event, rx_event) = mpsc::channel::<SerialEvent>(100);
        let (tx_action, rx_action) = mpsc::channel::<SerialAction>(100);
        self.txs.push(tx_event);
        self.rxs.push(rx_action);
        (rx_event, tx_action)
    }

    pub fn spawn_loop(mut self) {
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                let actions = self
                    .rxs
                    .iter_mut()
                    .map(|rx| rx.try_next().ok().flatten())
                    .collect::<Vec<_>>();

                for action in actions {
                    if action.is_none() {
                        continue;
                    }

                    self.send_event(SerialEvent::Loading(Ok(true))).await;
                    let result = match action.unwrap() {
                        SerialAction::UpdatePorts => self.update_ports().await,
                        SerialAction::OpenPort((port, baud_rate)) => {
                            self.open_port(port.id, baud_rate).await
                        }
                        SerialAction::ClosePort => self.close_port().await,
                        SerialAction::SendData(data) => self.send_data(data.as_bytes()).await,
                    };
                    self.send_event(result).await;
                    self.send_event(SerialEvent::Loading(Ok(false))).await;
                }

                if self.is_opened() {
                    let result = self.read_data().await;
                    self.send_event(result).await;
                }

                sleep_ms(10).await;
            }
        });
    }

    fn is_opened(&self) -> bool {
        self.opened_port.is_some() && self.__reader.is_some()
    }

    async fn send_event(&mut self, event: SerialEvent) {
        let futures = self
            .txs
            .iter_mut()
            .map(|tx| tx.send(event.clone()))
            .collect::<Vec<_>>();
        let results = join_all(futures).await;
        for res in results {
            // if let Err(e) = res {
            //     console::log_1(&e);
            // }
        }
    }
}
