use crate::{
    extractor::types::{Action, Event},
    libs::mpsc,
};

pub enum ExtractorType {
    Serial,
    Api,
    File,
    TcpUdp,

    None,
}

pub struct Extractor {
    pub is_running: bool,
    pub extractor_type: ExtractorType,

    __interval: i32,

    txs: Vec<mpsc::Sender<Event>>,
    rxs: Vec<mpsc::Receiver<Action>>,
}

impl Extractor {
    // pub fn new() -> Self {

    // }

    pub fn spawn_loop(mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            loop {
                let actions = self
                    .rxs
                    .iter_mut()
                    .map(|rx| rx.try_recv())
                    .collect::<Vec<_>>();

                for action in actions {
                    if action.is_none() {
                        continue;
                    }

                    let result = match action.unwrap() {
                        SerialAction::UpdatePorts => self.update_ports(),
                        SerialAction::OpenPort((port, baud_rate)) => {
                            self.send_event(SerialEvent::Loading(Ok(true)));
                            let res = self.open_port(port.id, baud_rate);
                            self.send_event(SerialEvent::Loading(Ok(false)));
                            res
                        }
                        SerialAction::ClosePort => self.close_port(),
                        SerialAction::SendData(data) => self.send_data(data.as_bytes()),
                    };
                    self.send_event(result);
                }

                if self.is_opened() {
                    let result = self.read_data();
                    self.send_event(result);
                }

                sleep_ms(10);
            }
        });

        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                use crate::libs::sleep::sleep_ms;

                let actions = self
                    .rxs
                    .iter_mut()
                    .map(|rx| rx.try_recv())
                    .collect::<Vec<_>>();

                self.send_event(Event::Loading(Ok(true))).await;
                for action in actions {
                    if let Some(action) = action {
                        let result = match action {
                            Action::Update => self.update_ports().await,
                            Action::Open(extractor_type) => {
                                self.open_port(port.id, baud_rate).await
                            }
                            Action::ClosePort => self.close_port().await,
                            Action::SendData(data) => self.send_data(data.as_bytes()).await,
                            Action::SetInterval(interval) => self.__interval = interval,
                        };
                        self.send_event(result).await;
                    }
                }
                self.send_event(Event::Loading(Ok(false))).await;

                if self.is_running {
                    let result = self.read_data().await;
                    self.send_event(result).await;
                }

                sleep_ms(self.__interval).await;
            }
        });
    }

    pub fn subscribe(&mut self) -> (mpsc::Receiver<Event>, mpsc::Sender<Action>) {
        let (tx_event, rx_event) = mpsc::channel::<Event>();
        let (tx_action, rx_action) = mpsc::channel::<Action>();
        self.txs.push(tx_event);
        self.rxs.push(rx_action);
        (rx_event, tx_action)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn send_event(&mut self, event: Event) {
        use futures::future::join_all;

        let futures = self
            .txs
            .iter_mut()
            .map(|tx| tx.send(event.clone()))
            .collect::<Vec<_>>();
        let _ = join_all(futures).await;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn send_event(&mut self, event: Event) {
        let _ = self
            .txs
            .iter_mut()
            .map(|tx| tx.send(event.clone()))
            .collect::<Vec<_>>();
    }
}
