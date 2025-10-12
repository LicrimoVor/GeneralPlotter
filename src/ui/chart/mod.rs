use egui_plot::{Line, Plot, PlotPoints};

pub struct Chart {
    points: Vec<Vec<[f64; 2]>>,
}

impl Default for Chart {
    fn default() -> Self {
        Self { points: vec![] }
    }
}

impl Chart {
    fn update(&mut self) {}

    pub fn run(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let lines: Vec<Line> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, points)| {
                Line::new(format!("Line {}", i + 1), PlotPoints::from(points.clone()))
            })
            .collect();

        Plot::new("serial_plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line);
                }
            });
    }
}
