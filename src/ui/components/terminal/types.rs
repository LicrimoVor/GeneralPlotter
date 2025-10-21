use egui::{Label, RichText};

#[derive(Clone)]
pub struct TerminalLabel {
    pub text: RichText,
    pub selectable: bool,
}

impl TerminalLabel {
    pub fn to_label(&self) -> Label {
        Label::new(self.text.clone()).selectable(self.selectable)
    }
}
