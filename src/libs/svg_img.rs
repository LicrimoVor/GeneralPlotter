use egui::{Image, ImageSource};

pub enum SvgImage {
    RELOAD,
    PEACE,
}

impl SvgImage {
    pub fn get_source(self) -> ImageSource<'static> {
        match self {
            SvgImage::RELOAD => egui::include_image!("../../assets/reload.svg"),
            SvgImage::PEACE => egui::include_image!("../../assets/peace.svg"),
        }
    }

    pub fn get_image(self) -> Image<'static> {
        egui::Image::new(self.get_source()).fit_to_original_size(1.0)
    }
}
