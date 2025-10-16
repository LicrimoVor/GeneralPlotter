use egui::{Color32, Image};

use crate::libs::svg_img::SvgImage;

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Ok,
    Error,
    Default,
    isLoading,
}

pub fn status_img(status: &Status, ui: &mut egui::Ui) -> Image<'static> {
    match status {
        Status::Ok => SvgImage::CIRCLE_CHECK.get_image().tint(Color32::GREEN),
        Status::Default => SvgImage::NONE
            .get_image()
            .tint(ui.style().visuals.strong_text_color()),
        Status::Error => SvgImage::CIRCLE_X.get_image().tint(Color32::RED),
        Status::isLoading => SvgImage::CIRCLE_LOADER
            .get_image()
            .tint(ui.style().visuals.strong_text_color()),
    }
}
