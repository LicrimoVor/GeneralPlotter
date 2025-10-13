use crate::libs::svg_img::SvgImage;
use egui::{Button, Color32, Response, Vec2};

pub fn button_image<'a>(
    ui: &mut egui::Ui,
    svg: SvgImage,
    color: Option<Color32>,
    size: Vec2,
) -> Response {
    ui.add(Button::image(
        svg.get_image()
            .tint(color.unwrap_or(ui.style().visuals.strong_text_color()))
            .max_size(size),
    ))
}

pub fn button_image_18<'a>(ui: &mut egui::Ui, svg: SvgImage, color: Option<Color32>) -> Response {
    button_image(ui, svg, color, Vec2 { x: 18.0, y: 18.0 })
}
