use crate::logic::ProxyData;
use std::sync::mpsc;

struct UserInterfaceState {
    proxy_data_rx: mpsc::Receiver<ProxyData>,
    config_tx: mpsc::Sender<ConfigLogic>,
    __config: ConfigLogic,
}

pub struct ConfigLogic {}

impl Default for ConfigLogic {
    fn default() -> Self {
        Self {}
    }
}

pub struct UserInterface {
    state: UserInterfaceState,
}

impl UserInterface {
    pub fn new(
        proxy_data_rx: mpsc::Receiver<ProxyData>,
        config_tx: mpsc::Sender<ConfigLogic>,
    ) -> Self {
        Self {
            state: UserInterfaceState {
                proxy_data_rx: proxy_data_rx,
                config_tx: config_tx,
                __config: ConfigLogic::default(),
            },
        }
    }

    fn update(&mut self) {
        while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
    }

    pub fn run(&mut self) {
        self.update();
    }
}
