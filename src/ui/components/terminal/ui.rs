use crate::core::settings::Settings;
use egui::{Color32, Frame, Margin, RichText, ScrollArea, TextEdit};
use std::sync::{Arc, Mutex};

pub struct Terminal {
    _is_open: bool,
    messages: Vec<(String, Color32)>,
    input: String,

    settings: Arc<Mutex<Settings>>,
}

impl Terminal {
    pub fn new(settings: Arc<Mutex<Settings>>) -> Self {
        Self {
            _is_open: false,
            settings,

            messages: vec![
                ("Тест".to_string(), Color32::WHITE),
                ("Тест".to_string(), Color32::RED),
                ("Тест".to_string(), Color32::CYAN),
                ("Тест".to_string(), Color32::WHITE),
                ("Тест".to_string(), Color32::RED),
                ("Тест".to_string(), Color32::CYAN),
                ("Тест".to_string(), Color32::WHITE),
                ("Тест".to_string(), Color32::RED),
                ("Тест".to_string(), Color32::CYAN),
                ("Тест".to_string(), Color32::WHITE),
                ("Тест".to_string(), Color32::RED),
                ("Тест".to_string(), Color32::CYAN),
                ("Тест".to_string(), Color32::WHITE),
                ("Тест".to_string(), Color32::RED),
                ("Тест".to_string(), Color32::CYAN),
                ("Тест".to_string(), Color32::WHITE),
                ("Тест".to_string(), Color32::RED),
                ("Тест".to_string(), Color32::CYAN),
            ],
            input: String::new(),
        }
    }
}
impl Terminal {
    pub fn update(&mut self) {}

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let height = ui.available_height();
        Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .stroke(ui.visuals().window_stroke)
            .inner_margin(Margin::symmetric(8, 8))
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .max_height(height - 55.0)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            ui.add_space(12.0);
                            ui.vertical(|ui| {
                                for (msg, color) in &self.messages {
                                    ui.label(RichText::new(msg).color(*color));
                                }
                            });
                        });
                    });

                ui.separator();

                // Нижняя часть — поле ввода
                let mut submitted = None;
                let resp = TextEdit::singleline(&mut self.input)
                    .hint_text("Введите команду...")
                    .desired_width(f32::INFINITY)
                    .show(ui);

                // Если нажали Enter → отправляем
                if resp.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    submitted = Some(self.input.clone());
                    self.input.clear();
                }
            });
    }
}
