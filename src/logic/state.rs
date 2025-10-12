use crate::libs::types::{LinierFunc, Value};
use crate::logic::SensorData;
use crate::ui::ConfigLogic;
use egui_plot::PlotPoint;
use std::sync::mpsc;

struct LogicState {
    config_rx: mpsc::Receiver<ConfigLogic>,
    sensor_data_tx: mpsc::Sender<SensorData>,

    all_serial_data: Vec<Vec<Vec<String>>>,
    all_parsed_data: Vec<Vec<Vec<Value>>>,
    all_times_windows: Vec<Vec<Vec<i32>>>,
    all_times_serial: Vec<Vec<Vec<i32>>>,
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
        proxy_data_tx: mpsc::Sender<SensorData>,
    ) -> Self {
        Self {
            state: LogicState {
                config_rx: config_rx,
                sensor_data_tx: proxy_data_tx,
                all_serial_data: vec![],
                all_parsed_data: vec![],
                all_times_windows: vec![],
                all_times_serial: vec![],
            },
            funcs: vec![],
        }
    }
    fn update(&mut self) {
        while let Ok(config) = self.state.config_rx.try_recv() {}
    }

    pub fn run(&mut self) {
        self.update();

        println!("run logic");
    }
}
