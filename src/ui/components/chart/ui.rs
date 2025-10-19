use crate::libs::timer::Timer;
use crate::logic::SensorData;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::{Arc, Mutex};

pub struct Chart {
    points: Vec<Vec<[f64; 2]>>,
    sensor_data: Arc<Mutex<SensorData>>,
}

impl Chart {
    pub fn new(sensor_data: Arc<Mutex<SensorData>>) -> Self {
        Self {
            points: vec![],
            sensor_data,
        }
    }
}

impl Chart {
    pub fn update(&mut self) {
        self.points = self.sensor_data.lock().unwrap().all_points.clone();
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let lines: Vec<Line> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, points)| {
                Line::new(format!("Line {}", i + 1), PlotPoints::from(points.clone()))
            })
            .collect();

        Plot::new("serial_plot")
            // .default_x_bounds(0.0, 100.0)
            // .view_aspect(2.0)
            .show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line);
                }
            });
    }
}
