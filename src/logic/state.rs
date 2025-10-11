use crate::libs::types::{LinierFunc, Value};
use crate::ui::ConfigLogic;
use egui_plot::PlotPoint;
use std::sync::mpsc;

struct LogicState {
    config_rx: mpsc::Receiver<ConfigLogic>,
    proxy_data_tx: mpsc::Sender<ProxyData>,

    all_serial_data: Vec<Vec<Vec<String>>>,
    all_parsed_data: Vec<Vec<Vec<Value>>>,
    all_times_windows: Vec<Vec<Vec<i32>>>,
    all_times_serial: Vec<Vec<Vec<i32>>>,

    __proxy_data: ProxyData,
}

pub struct ProxyData {
    serial_datas: Vec<Vec<String>>,
    parsed_datas: Vec<Vec<Value>>,
    times_windows: Vec<Vec<i32>>,
    times_serial: Vec<Vec<i32>>,

    all_points: Vec<Vec<Vec<PlotPoint>>>,
}

impl Default for ProxyData {
    fn default() -> Self {
        Self {
            serial_datas: vec![],
            parsed_datas: vec![],
            times_windows: vec![],
            times_serial: vec![],

            all_points: vec![],
        }
    }
}

pub struct Logic {
    state: LogicState,
    // extractor: Option<dyn Any>,
    // logger: Option<dyn Any>,
    // serializer: Option<dyn Any>,
    funcs: Vec<LinierFunc>,
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
                all_serial_data: vec![],
                all_parsed_data: vec![],
                all_times_windows: vec![],
                all_times_serial: vec![],
                __proxy_data: ProxyData::default(),
            },
            funcs: vec![],
        }
    }
    fn update(&mut self) {
        while let Ok(config) = self.state.config_rx.try_recv() {}
    }

    pub fn run(&mut self) {
        self.update();
    }
}
