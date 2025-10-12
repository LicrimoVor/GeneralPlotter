use crate::libs::types::{LinierFunc, Value};
use crate::logic::SensorData;
use crate::ui::ConfigLogic;
use egui_plot::PlotPoint;
use std::sync::{Arc, Mutex, mpsc};

struct LogicState {
    config: Arc<Mutex<ConfigLogic>>,
    sensor_data: Arc<Mutex<SensorData>>,
}

pub struct Logic {
    state: LogicState,
    // extractor: Option<dyn Any>,
    // logger: Option<dyn Any>,
    // serializer: Option<dyn Any>,
    funcs: Vec<LinierFunc>,
}

impl Logic {
    pub fn new(config: Arc<Mutex<ConfigLogic>>, sensor_data: Arc<Mutex<SensorData>>) -> Self {
        Self {
            state: LogicState {
                config,
                sensor_data,
            },
            funcs: vec![],
        }
    }
    fn update(&mut self) {
        // while let Ok(config) = self.state.config.lock() {}
        // let sensor_data = self.state.sensor_data.lock().unwrap();
    }

    pub fn run(&mut self) {
        self.update();

        // println!("run logic");
    }
}
