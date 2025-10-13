use egui::{Color32, Image};

use crate::libs::svg_img::SvgImage;

pub enum Status {
    Ok,
    Error,
    Default,
}

pub fn status_img(status: &Status, ui: &mut egui::Ui) -> Image<'static> {
    match status {
        Status::Ok => SvgImage::CIRCLE_CHECK.get_image().tint(Color32::GREEN),
        Status::Default => SvgImage::CIRCLE
            .get_image()
            .tint(ui.style().visuals.strong_text_color()),
        Status::Error => SvgImage::CIRCLE_X.get_image().tint(Color32::RED),
    }
}
