pub struct Description {}

impl Description {
    pub fn new() -> Self {
        Self {}
    }
}

impl Description {
    pub fn update(&mut self) {}
    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        ui.collapsing("Инструкция", |ui| {
            egui::Grid::new("instructions")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    // ui.style_mut().f
                    let egui::InputOptions {
                        zoom_modifier,
                        horizontal_scroll_modifier,
                        vertical_scroll_modifier,
                        ..
                    } = egui::InputOptions::default();
                    let mut style = (**ui.style()).clone();
                    style.text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(12.0, egui::FontFamily::Proportional),
                    );
                    ui.set_style(style);

                    ui.label("Перемещение");
                    ui.label("Left-drag");
                    ui.end_row();

                    ui.label("Гор. перемещение");
                    ui.label(format!(
                        "{} + Scroll",
                        ui.ctx().format_modifiers(horizontal_scroll_modifier)
                    ));
                    ui.end_row();

                    ui.label("Приближение");
                    ui.label(format!(
                        "{} + Scroll",
                        ui.ctx().format_modifiers(zoom_modifier)
                    ));
                    ui.end_row();

                    ui.label("Гор. приближение");
                    ui.label(format!(
                        "{} + Scroll",
                        ui.ctx()
                            .format_modifiers(zoom_modifier | horizontal_scroll_modifier)
                    ));
                    ui.end_row();

                    ui.label("Вер. приближение");
                    ui.label(format!(
                        "{} + Scroll",
                        ui.ctx()
                            .format_modifiers(zoom_modifier | vertical_scroll_modifier)
                    ));
                    ui.end_row();

                    ui.label("Выделение");
                    ui.label("Right-drag");
                    ui.end_row();

                    ui.label("Сбос");
                    ui.label("Double-click");
                    ui.end_row();
                });
        });
    }
}
