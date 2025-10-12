use egui::{
    Color32, CornerRadius, Shadow, Stroke, Visuals,
    style::{Selection, WidgetVisuals},
};

pub fn gravity_light_visuals() -> Visuals {
    let mut visuals = Visuals::default();
    visuals.dark_mode = false;

    let brand_base = Color32::from_rgb(255, 190, 92);
    let brand_hover = Color32::from_rgb(255, 210, 141);
    let brand_strong = Color32::from_rgb(255, 197, 108);

    let text_color = Color32::from_rgb(50, 40, 30);
    let text_contrast = Color32::from_rgb(20, 15, 10);

    let bg_main = Color32::from_rgb(255, 245, 231);
    let bg_panel = Color32::from_rgb(255, 249, 239);
    let bg_hover = Color32::from_rgb(255, 242, 222);
    let bg_active = Color32::from_rgb(255, 236, 206);
    let border_color = Color32::from_rgb(233, 174, 86);

    visuals.override_text_color = Some(text_color);
    visuals.extreme_bg_color = bg_main;
    visuals.panel_fill = bg_panel;
    visuals.window_fill = bg_panel;
    visuals.faint_bg_color = Color32::from_rgb(255, 242, 222);
    visuals.code_bg_color = Color32::from_rgb(255, 249, 239);
    visuals.hyperlink_color = brand_base;

    visuals.selection = Selection {
        bg_fill: brand_base,
        stroke: Stroke::new(1.0, brand_hover),
    };

    visuals.window_stroke = Stroke::new(1.0, border_color);
    visuals.window_shadow = Shadow {
        offset: [2, 2],
        blur: 8,
        spread: 0,
        color: Color32::from_rgba_unmultiplied(0, 0, 0, 25),
    };
    visuals.window_corner_radius = CornerRadius::from(8.0);

    visuals.widgets = egui::style::Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: bg_panel,
            weak_bg_fill: bg_panel,
            bg_stroke: Stroke::new(1.0, border_color),
            corner_radius: CornerRadius::from(6.0),
            fg_stroke: Stroke::new(1.0, text_color),
            expansion: 0.0,
        },
        inactive: WidgetVisuals {
            bg_fill: Color32::from_rgb(255, 249, 239), // фон кнопок по умолчанию
            weak_bg_fill: Color32::from_rgb(255, 249, 239),
            bg_stroke: Stroke::new(1.0, border_color),
            corner_radius: CornerRadius::from(6.0),
            fg_stroke: Stroke::new(1.2, text_color),
            expansion: 0.0,
        },
        hovered: WidgetVisuals {
            bg_fill: bg_hover, // Hover — лёгкий акцент
            weak_bg_fill: bg_hover,
            bg_stroke: Stroke::new(1.2, brand_strong),
            corner_radius: CornerRadius::from(6.0),
            fg_stroke: Stroke::new(1.4, text_contrast),
            expansion: 0.0,
        },
        active: WidgetVisuals {
            bg_fill: brand_base, // Active — фирменный цвет
            weak_bg_fill: brand_base,
            bg_stroke: Stroke::new(1.4, brand_hover),
            corner_radius: CornerRadius::from(6.0),
            fg_stroke: Stroke::new(1.4, text_contrast),
            expansion: 0.0,
        },
        open: WidgetVisuals {
            bg_fill: bg_active, // открытые элементы (например combo)
            weak_bg_fill: bg_active,
            bg_stroke: Stroke::new(1.0, brand_strong),
            corner_radius: CornerRadius::from(6.0),
            fg_stroke: Stroke::new(1.2, text_color),
            expansion: 0.0,
        },
    };

    visuals.text_cursor = egui::style::TextCursorStyle {
        stroke: Stroke::new(2.0, brand_base),
        preview: true,
        ..Default::default()
    };

    visuals
}
