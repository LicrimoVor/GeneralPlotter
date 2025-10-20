use egui::{Color32, Image};

use crate::libs::svg_img::SvgImage;

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Ok,
    Error,
    Default,
    IsLoading,
}

pub fn status_img(status: &Status, ui: &mut egui::Ui) -> Image<'static> {
    match status {
        Status::Ok => SvgImage::CircleCheck.get_image().tint(Color32::GREEN),
        Status::Default => SvgImage::NONE
            .get_image()
            .tint(ui.style().visuals.strong_text_color()),
        Status::Error => SvgImage::CircleX.get_image().tint(Color32::RED),
        Status::IsLoading => SvgImage::CircleLoader
            .get_image()
            .tint(ui.style().visuals.strong_text_color()),
    }
}
