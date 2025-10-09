use crate::libs::types::Value;
use crate::ui::ConfigLogic;
use std::sync::mpsc;

struct LogicState {
    config_rx: mpsc::Receiver<ConfigLogic>,
    proxy_data_tx: mpsc::Sender<ProxyData>,
    __proxy_data: ProxyData,
}

pub struct ProxyData {
    serial_datas: Vec<Vec<String>>,
    parsed_datas: Vec<Vec<Value>>,
    times_windows: Vec<Vec<i32>>,
    times_serial: Vec<Vec<i32>>,
}

impl Default for ProxyData {
    fn default() -> Self {
        Self {
            serial_datas: vec![],
            parsed_datas: vec![],
            times_windows: vec![],
            times_serial: vec![],
        }
    }
}

pub struct Logic {
    state: LogicState,
    // extractor: Option<dyn Any>,
    // logger: Option<dyn Any>,
    // serializer: Option<dyn Any>,
}

impl Logic {
    pub fn new(
        config_rx: mpsc::Receiver<ConfigLogic>,
        proxy_data_tx: mpsc::Sender<ProxyData>,
    ) -> Self {
        Self {
            state: LogicState {
                config_rx: config_rx,
                proxy_data_tx: proxy_data_tx,
                __proxy_data: ProxyData::default(),
            },
        }
    }
    fn update(&mut self) {
        while let Ok(config) = self.state.config_rx.try_recv() {}
    }

    pub fn run(&mut self) {
        self.update();
    }
}
