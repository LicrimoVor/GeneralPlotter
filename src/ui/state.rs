use super::panel::Panel;
use super::types::ConfigLogic;
use crate::libs::types::Value;
use crate::logic::SensorData;
use egui::Vec2;
use egui_plot::PlotPoint;
use std::sync::mpsc;

struct UserInterfaceState {
    serial_datas: Vec<Vec<String>>,
    parsed_datas: Vec<Vec<Value>>,
    times_windows: Vec<Vec<i32>>,
    times_serial: Vec<Vec<i32>>,
    all_points: Vec<Vec<PlotPoint>>,
}

pub struct UserInterface {
    sensor_rx: mpsc::Receiver<SensorData>,
    config_tx: mpsc::Sender<ConfigLogic>,

    // state: UserInterfaceState,
    panel: Panel,
}

impl UserInterface {
    pub fn new(
        sensor_rx: mpsc::Receiver<SensorData>,
        config_tx: mpsc::Sender<ConfigLogic>,
    ) -> Self {
        Self {
            sensor_rx: sensor_rx,
            config_tx: config_tx,
            panel: Panel::default(),
        }
    }

    fn update(&mut self) {
        // while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
    }

    pub fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update();

        super::styles::apply_dark_theme(ctx);
        // super::styles::apply_light_theme(ctx);

        let Vec2 {
            x: width,
            y: heigth,
        } = ctx.content_rect().size();

        if width > 720.0 {
            egui::SidePanel::left("left")
                .min_width(170.0)
                .max_width(170.0)
                .resizable(false)
                .show(ctx, |ui| {
                    let mut style = ui.style_mut().clone();
                    style.spacing.interact_size = egui::vec2(0.0, 18.0);
                    style.spacing.button_padding = egui::vec2(4.0, 4.0);
                    ui.set_style(style);

                    ui.add_space(8.0);
                    self.panel.run(ctx, ui);
                });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // self.panel.run(ctx, ui);
        });
        if width > 1170.0 {
            egui::SidePanel::right("rigth")
                .min_width(200.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Левая панель");
                });
        }
    }
}
