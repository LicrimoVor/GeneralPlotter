use egui::{Image, ImageSource};

pub enum SvgImage {
    RELOAD,
    PEACE,
    CONNECT,
    DISCONNECT,
    CircleCheck,
    CircleX,
    CIRCLE,
    CircleLoader,
    SETTINGS,
    PLUS,
    UP,
    DOWN,
    NONE,
}

impl SvgImage {
    pub fn get_source(self) -> ImageSource<'static> {
        match self {
            SvgImage::RELOAD => egui::include_image!("../../assets/reload.svg"),
            SvgImage::PEACE => egui::include_image!("../../assets/peace.svg"),
            SvgImage::CONNECT => egui::include_image!("../../assets/connect.svg"),
            SvgImage::DISCONNECT => egui::include_image!("../../assets/disconnect.svg"),
            SvgImage::CIRCLE => egui::include_image!("../../assets/circle-dashed.svg"),
            SvgImage::CircleLoader => egui::include_image!("../../assets/circle-loader.svg"),
            SvgImage::CircleCheck => egui::include_image!("../../assets/circle-dashed-check.svg"),
            SvgImage::CircleX => egui::include_image!("../../assets/circle-dashed-x.svg"),
            SvgImage::SETTINGS => egui::include_image!("../../assets/settings.svg"),
            SvgImage::PLUS => egui::include_image!("../../assets/plus.svg"),
            SvgImage::UP => egui::include_image!("../../assets/up.svg"),
            SvgImage::DOWN => egui::include_image!("../../assets/down.svg"),
            SvgImage::NONE => egui::include_image!("../../assets/none.svg"),
        }
    }

    pub fn get_image(self) -> Image<'static> {
        egui::Image::new(self.get_source())
    }
}
