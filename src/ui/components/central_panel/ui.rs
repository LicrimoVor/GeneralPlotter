use super::super::{chart::Chart, terminal::Terminal};
use crate::libs::mpsc;
use crate::libs::serials::SerialAction;
use crate::logic::config::ConfigLogic;
use crate::ui::UiData;
use crate::ui::libs::button_image::button_image_18;
use crate::ui::settings::Settings;
use crate::{libs::svg_img::SvgImage, logic::SensorData};
use egui::Id;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct CentralPanel {
    // data
    sensor_data: Arc<Mutex<SensorData>>,
    settings: Arc<Mutex<Settings>>,
    ui_data: Arc<Mutex<UiData>>,

    // ui
    chart: Chart,
    terminal: Terminal,

    _collapsed_chart: bool,
    _is_min_height: bool,
}

impl CentralPanel {
    pub fn new(
        sensor_data: Arc<Mutex<SensorData>>,
        settings: Arc<Mutex<Settings>>,
        ui_data: Arc<Mutex<UiData>>,
        serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,
    ) -> Self {
        Self {
            sensor_data: sensor_data.clone(),
            settings: settings.clone(),
            ui_data: ui_data.clone(),
            chart: Chart::new(settings.clone(), sensor_data.clone()),
            terminal: Terminal::new(settings.clone(), ui_data.clone(), serial_tx),

            _collapsed_chart: false,
            _is_min_height: false,
        }
    }

    pub fn update(&mut self) {
        // while let Ok(proxy_data) = self.state.proxy_data_rx.try_recv() {}
        self.chart.update();
        self.terminal.update();
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let heigth_panel = ui.available_height();
        let min_height = if self._is_min_height {
            heigth_panel / 2.0
        } else {
            0.0
        };

        if !self._collapsed_chart {
            let chart = egui::TopBottomPanel::top("top_panel")
                .resizable(true)
                .min_height(min_height)
                .max_height(heigth_panel - 256.0)
                .default_height(heigth_panel / 2.0)
                .show_inside(ui, |ui| {
                    self.chart.show(ctx, ui);
                });
            self._is_min_height = false;
            if chart.response.rect.height() < 80.0 {
                self._collapsed_chart = true;
            }
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.terminal.show(ctx, ui);
        });

        egui::Area::new(Id::new("chart_up"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-225.0, 20.0))
            .show(ctx, |ui| {
                let img = if self._collapsed_chart {
                    SvgImage::DOWN
                } else {
                    SvgImage::UP
                };
                if button_image_18(ui, img, None).clicked() {
                    self._collapsed_chart = !self._collapsed_chart;
                    self._is_min_height = true;
                }
            });
    }
}
