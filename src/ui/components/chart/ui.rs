use crate::logic::SensorData;
use crate::ui::settings::Settings;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::{Arc, Mutex};

pub struct Chart {
    all_points: Vec<(Vec<[f64; 2]>, Vec<[f64; 2]>)>,
    sensor_data: Arc<Mutex<SensorData>>,
    settings: Arc<Mutex<Settings>>,
}

impl Chart {
    pub fn new(settings: Arc<Mutex<Settings>>, sensor_data: Arc<Mutex<SensorData>>) -> Self {
        Self {
            all_points: vec![],
            sensor_data,
            settings,
        }
    }
}

impl Chart {
    pub fn update(&mut self) {
        self.all_points = self.sensor_data.lock().unwrap().all_points.clone();
    }

    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        let is_time_serial = self.settings.lock().unwrap().is_time_serial;

        let lines: Vec<Line> = self
            .all_points
            .iter()
            .enumerate()
            .map(|(i, points)| {
                let line = if !is_time_serial {
                    points.0.clone()
                } else {
                    points.1.clone()
                };
                Line::new(format!("Line {}", i + 1), PlotPoints::from(line))
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
