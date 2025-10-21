use super::types::TerminalLabel;
use crate::{
    libs::{message::Message, mpsc, print, serials::SerialAction},
    ui::{UiData, settings::Settings},
};
use egui::{Color32, Frame, Label, Margin, RichText, ScrollArea, TextEdit, TextStyle};
use egui_extras::{Column, TableBuilder};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct Terminal {
    //state
    settings: Arc<Mutex<Settings>>,
    ui_data: Arc<Mutex<UiData>>,

    serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,

    // ui
    _is_open: bool,
    input: String,
    // label_id, label_time, msg_text
    labels: Vec<(Option<TerminalLabel>, Option<TerminalLabel>, TerminalLabel)>,
    rows: Vec<(
        Option<TerminalLabel>,
        Option<TerminalLabel>,
        Vec<TerminalLabel>,
    )>,
    last_len: usize,
}

impl Terminal {
    pub fn new(
        settings: Arc<Mutex<Settings>>,
        ui_data: Arc<Mutex<UiData>>,
        serial_tx: Rc<RefCell<mpsc::Sender<SerialAction>>>,
    ) -> Self {
        Self {
            settings,
            ui_data,

            serial_tx,

            _is_open: false,
            input: String::new(),
            labels: Vec::new(),
            rows: Vec::new(),
            last_len: 0,
        }
    }
}

impl Terminal {
    pub fn update(&mut self) {
        let settings = self.settings.lock().unwrap();
        let ui_data = self.ui_data.lock().unwrap();
        let messages = &ui_data.messages;

        if messages.len() != self.last_len || settings.is_updated || ui_data.is_reboot {
            let iter: Box<dyn Iterator<Item = (usize, &Message)>> = if ui_data.is_reboot {
                self.labels.clear();
                self.rows.clear();
                Box::new(messages.iter().enumerate())
            } else {
                Box::new(messages.iter().enumerate().skip(self.last_len))
            };

            for (i, msg) in iter {
                if !settings.terminal.get_is_show(&msg.r#type) {
                    continue;
                }
                let color = settings.terminal.get_color(&msg.r#type);
                let label_id = if settings.terminal.show_id {
                    Some(TerminalLabel {
                        text: RichText::new(format!("[{:05}] ", i + 1)).color(color),
                        selectable: settings.terminal.id_selectable,
                    })
                } else {
                    None
                };
                let label_time = if settings.terminal.show_id {
                    Some(TerminalLabel {
                        text: RichText::new(format!("<{}> ", msg.get_created())).color(color),
                        selectable: settings.terminal.time_selectable,
                    })
                } else {
                    None
                };
                let labels = msg
                    .text
                    .split(settings.delimeter)
                    .map(|text| TerminalLabel {
                        text: RichText::new(text.to_string()).color(color),
                        selectable: true,
                    })
                    .collect::<Vec<TerminalLabel>>();

                self.labels.push((
                    label_id.clone(),
                    label_time.clone(),
                    TerminalLabel {
                        text: RichText::new(msg.text.clone()).color(color),
                        selectable: true,
                    },
                ));
                self.rows.push((label_id, label_time, labels));
            }

            self.last_len = messages.len();
        }
    }

    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        let height = ui.available_height();
        let row_height = ui.text_style_height(&TextStyle::Body);
        let settings_terminal = self.settings.lock().unwrap().terminal;
        let count_msg = settings_terminal.count_msg as usize;
        let len_msg = self.labels.len();
        let count_max: usize = if count_msg == 0 {
            len_msg
        } else {
            count_msg.min(len_msg)
        };

        if !settings_terminal.mode_table {
            Frame::group(ui.style())
                .fill(ui.visuals().extreme_bg_color)
                .stroke(ui.visuals().window_stroke)
                .inner_margin(Margin::symmetric(8, 8))
                .show(ui, |ui| {
                    ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .stick_to_bottom(true)
                        .max_height(height - 55.0)
                        .show_rows(ui, row_height, count_max, |ui, row_range| {
                            for i in row_range {
                                let index = len_msg - count_max + i;
                                if index >= len_msg {
                                    print::print("index >= len_msg");
                                    break;
                                }
                                let (label_id, label_time, msg) = self.labels.get(index).unwrap();
                                ui.horizontal(|ui| {
                                    if label_id.is_some() {
                                        ui.add(
                                            Label::new(label_id.as_ref().unwrap().text.clone())
                                                .selectable(label_id.as_ref().unwrap().selectable),
                                        );
                                    }
                                    if label_time.is_some() {
                                        ui.add(
                                            Label::new(label_time.as_ref().unwrap().text.clone())
                                                .selectable(
                                                    label_time.as_ref().unwrap().selectable,
                                                ),
                                        );
                                    }
                                    ui.add(Label::new(msg.text.clone()).selectable(msg.selectable));
                                });
                            }
                        });
                });
        } else {
            if len_msg == 0 {
            } else {
                ui.add_space(12.0);
                let mut table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .auto_shrink([false, false])
                    .min_scrolled_height(0.0)
                    .max_scroll_height(f32::INFINITY);

                let mut labels: Vec<String> = vec![];
                let last = self.rows.last().unwrap();
                if last.0.is_some() {
                    labels.push("ID".to_string());
                    table = table.column(Column::auto());
                }
                if last.1.is_some() {
                    labels.push("Время".to_string());
                    table = table.column(Column::auto());
                }
                for i in 0..last.2.len() {
                    labels.push(format!("Колонка {}", i + 1));
                    table = table.column(Column::remainder());
                }
                table
                    .header(20.0, |mut header| {
                        for label in labels.iter() {
                            header.col(|ui| {
                                ui.label(RichText::new(label).strong());
                            });
                        }
                    })
                    .body(|mut body| {
                        for (row, color) in rows.iter().zip(colors) {
                            body.row(18.0, |mut table_row| {
                                for (i, cell) in row.iter().enumerate() {
                                    table_row.col(|ui| {
                                        if (i == 0
                                            && (settings.terminal.show_id
                                                || settings.terminal.show_time))
                                            || (i == 1 && settings.terminal.show_time)
                                        {
                                            let selectable =
                                                if (i == 0) && settings.terminal.show_id {
                                                    settings.terminal.id_selectable
                                                } else {
                                                    settings.terminal.time_selectable
                                                };
                                            ui.add(
                                                Label::new(RichText::new(cell).color(color))
                                                    .selectable(selectable),
                                            );
                                        } else {
                                            ui.label(RichText::new(cell.clone()).color(color));
                                        }
                                    });
                                }
                            });
                        }
                    });
            }
        }

        ui.separator();

        let resp = TextEdit::singleline(&mut self.input)
            .hint_text("Введите команду...")
            .desired_width(f32::INFINITY)
            .show(ui);
        if resp.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let _ = self
                .serial_tx
                .borrow_mut()
                .send(SerialAction::SendData(self.input.clone()));
            self.input.clear();
        };
    }
}
