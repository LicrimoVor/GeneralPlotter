use egui::{Image, ImageSource};

pub enum SvgImage {
    RELOAD,
    PEACE,
    CONNECT,
    CIRCLE_CHECK,
    CIRCLE_X,
    CIRCLE,
    SETTINGS,
}

impl SvgImage {
    pub fn get_source(self) -> ImageSource<'static> {
        match self {
            SvgImage::RELOAD => egui::include_image!("../../assets/reload.svg"),
            SvgImage::PEACE => egui::include_image!("../../assets/peace.svg"),
            SvgImage::CONNECT => egui::include_image!("../../assets/connect.svg"),
            SvgImage::CIRCLE => egui::include_image!("../../assets/circle-dashed.svg"),
            SvgImage::CIRCLE_CHECK => egui::include_image!("../../assets/circle-dashed-check.svg"),
            SvgImage::CIRCLE_X => egui::include_image!("../../assets/circle-dashed-x.svg"),
            SvgImage::SETTINGS => egui::include_image!("../../assets/settings.svg"),
        }
    }

    pub fn get_image(self) -> Image<'static> {
        egui::Image::new(self.get_source()).fit_to_original_size(1.0)
    }
}
