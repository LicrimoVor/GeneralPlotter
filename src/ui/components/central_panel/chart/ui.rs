use crate::logic::SensorData;
use crate::ui::settings::Settings;
use egui_plot::{Corner, Legend, Line, Plot, PlotPoint, PlotPoints};
use std::sync::{Arc, Mutex};

pub struct Chart {
    all_points: Vec<(Vec<PlotPoint>, Vec<PlotPoint>)>,
    sensor_data: Arc<Mutex<SensorData>>,
    settings: Arc<Mutex<Settings>>,

    is_init: bool,
}

impl Chart {
    pub fn new(settings: Arc<Mutex<Settings>>, sensor_data: Arc<Mutex<SensorData>>) -> Self {
        Self {
            all_points: vec![],
            sensor_data,
            settings,

            is_init: true,
        }
    }
}

impl Chart {
    pub fn update(&mut self) {
        let sensor_data = self.sensor_data.lock().unwrap();
        if sensor_data.is_reload || self.is_init {
            self.is_init = false;
            self.all_points.clear();
            for points in sensor_data.all_points.iter() {
                self.all_points.push((
                    points
                        .0
                        .iter()
                        .map(|p| PlotPoint::new(p[0], p[1]))
                        .collect(),
                    points
                        .1
                        .iter()
                        .map(|p| PlotPoint::new(p[0], p[1]))
                        .collect(),
                ));
            }
        } else if sensor_data.is_updated {
            for _ in self.all_points.len()..sensor_data.all_points.len() {
                self.all_points.push((vec![], vec![]));
            }

            for (i, points) in sensor_data.all_points.iter().enumerate() {
                for j in self.all_points[i].0.len()..points.0.len() {
                    self.all_points[i]
                        .0
                        .push(PlotPoint::new(points.0[j][0], points.0[j][1]));
                    self.all_points[i]
                        .1
                        .push(PlotPoint::new(points.1[j][0], points.1[j][1]));
                }
            }
        }
    }

    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        let settings = self.settings.lock().unwrap();
        let is_time_serial = settings.is_time_serial;
        let display = settings.chart.display.clone();
        let count_points = settings.chart.count_points;

        Plot::new("serial_plot")
            // .default_x_bounds(0.0, 100.0)
            // .view_aspect(2.0)
            .legend(Legend::default().title("Легенда").position(Corner::LeftTop))
            .show(ui, |plot_ui| {
                for ((i, points), flag) in self.all_points.iter().enumerate().zip(display) {
                    if !flag {
                        continue;
                    }
                    let line = if !is_time_serial {
                        &points.0
                    } else {
                        &points.1
                    };
                    let visible = if line.len() > count_points && count_points > 0 {
                        &line[line.len() - count_points..]
                    } else {
                        line
                    };
                    plot_ui.line(Line::new(
                        format!("Линия {}", i + 1),
                        PlotPoints::Borrowed(visible),
                    ));
                }
            });
    }
}
