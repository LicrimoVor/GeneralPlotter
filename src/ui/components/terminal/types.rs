use egui::RichText;

#[derive(Clone)]
pub struct TerminalLabel {
    pub text: RichText,
    pub selectable: bool,
}
