use egui::{self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Style, Vec2};
mod dark;
mod ligth;

fn base_style_from(ctx: &egui::Context) -> egui::Style {
    let mut style = (*ctx.style()).clone();

    // spacing
    style.spacing.item_spacing = Vec2::new(8.0, 6.0); // межстрочный / между элементами
    style.spacing.button_padding = Vec2::new(8.0, 6.0);

    // немного более мягкие углы
    style.visuals.window_corner_radius = CornerRadius::from(6.0);
    style.visuals.widgets.inactive.corner_radius = CornerRadius::from(6.0);
    style.visuals.widgets.hovered.corner_radius = CornerRadius::from(6.0);
    style.visuals.widgets.active.corner_radius = CornerRadius::from(6.0);

    // немного увеличим заголовки
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(20.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(14.0)),
        (egui::TextStyle::Monospace, egui::FontId::monospace(13.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(14.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(11.0)),
    ]
    .into();
    style
}

/// Подключить (опционально) встроенные шрифты: Roboto-like fallback
fn apply_default_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("../../../assets/OpenSans.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    ctx.set_fonts(fonts);
}

/// Тёмная тема — современная, мягкие акценты
pub fn apply_dark_theme(ctx: &egui::Context) {
    apply_default_fonts(ctx);

    let mut style = base_style_from(ctx);
    style.visuals = self::dark::gravity_dark_visuals();
    ctx.set_style(Style {
        visuals: self::dark::gravity_dark_visuals(),
        ..Default::default()
    });

    ctx.set_style(style);
}

/// Светлая тема — чистая, с мягким контрастом
pub fn apply_light_theme(ctx: &egui::Context) {
    apply_default_fonts(ctx);

    let mut style = base_style_from(ctx);
    style.visuals = self::ligth::gravity_light_visuals();
    ctx.set_style(Style {
        visuals: self::ligth::gravity_light_visuals(),
        ..Default::default()
    });

    ctx.set_style(style);
}
