use crate::libs::svg_img::SvgImage;
use egui::{Color32, CornerRadius, ImageButton, Response, Vec2};

pub fn button_image<'a>(
    ui: &mut egui::Ui,
    svg: SvgImage,
    color: Color32,
    size: Vec2,
    radius: u8,
) -> Response {
    ui.add(
        ImageButton::new(svg.get_image().tint(color).max_size(size))
            .corner_radius(CornerRadius::from(radius)),
    )
}

pub fn button_image_14<'a>(ui: &mut egui::Ui, svg: SvgImage, color: Color32) -> Response {
    button_image(ui, svg, color, Vec2 { x: 14.0, y: 14.0 }, 4)
}
