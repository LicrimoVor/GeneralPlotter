#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

mod app;
mod libs;
mod logic;
mod ui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Plotter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app::AppState::default()))
        }),
    )
    .expect("failed to start egui");
}
