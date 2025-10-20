use crate::{
    libs::{message::MessageType, mpsc, serials::SerialAction},
    ui::{UiData, settings::Settings},
};
use egui::{Color32, Frame, Label, Margin, RichText, ScrollArea, TextEdit};
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
    ui_cache: Vec<(
        Option<egui::widgets::Label>,
        Option<egui::widgets::Label>,
        egui::widgets::Label,
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
            ui_cache: Vec::new(),
            last_len: 0,
        }
    }
}

impl Terminal {
    pub fn update(&mut self) {
        let settings = self.settings.lock().unwrap();
        let ui_data = self.ui_data.lock().unwrap();
        let messages = &ui_data.messages;

        if messages.len() != self.last_len || settings._is_updated {
            for (i, msg) in messages.iter().enumerate().skip(self.last_len) {
                let color = settings.terminal.get_color(&msg.r#type);
                let label_id = if settings.terminal.show_id {
                    Some(
                        Label::new(RichText::new(format!("[{:05}] ", i + 1)).color(color))
                            .selectable(settings.terminal.id_selectable)
                            .selectable(settings.terminal.id_selectable),
                    )
                } else {
                    None
                };
                let label_time = if settings.terminal.show_id {
                    Some(
                        Label::new(RichText::new(format!("<{}> ", msg.get_created())).color(color))
                            .selectable(settings.terminal.time_selectable),
                    )
                } else {
                    None
                };
                self.ui_cache.push((
                    label_id,
                    label_time,
                    Label::new(RichText::new(msg.text.clone()).color(color)),
                ));
            }
            self.last_len = messages.len();
        }
    }

    pub fn show(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
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
                        if !self.settings.lock().unwrap().terminal.mode_table {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                                ui.add_space(12.0);
                                ui.vertical(|ui| {
                                    for (label_id, label_time, msg) in self.ui_cache.iter() {
                                        if self.settings.lock().unwrap().terminal.show_separator {
                                            ui.separator();
                                        }

                                        ui.horizontal(|ui| {
                                            let color = settings.terminal.get_color(&msg.r#type);
                                            if settings.terminal.show_id {
                                                ui.add(
                                                    Label::new(
                                                        RichText::new(format!("[{:05}]", i + 1,))
                                                            .color(color),
                                                    )
                                                    .selectable(settings.terminal.id_selectable),
                                                );
                                            }
                                            if settings.terminal.show_time {
                                                ui.add(
                                                    Label::new(
                                                        RichText::new(format!(
                                                            "<{}>",
                                                            msg.get_created()
                                                        ))
                                                        .color(color),
                                                    )
                                                    .selectable(settings.terminal.time_selectable),
                                                );
                                            }
                                            ui.label(msg.text.clone());
                                        });
                                    }
                                });
                            });
                        } else {
                            let mut rows: Vec<Vec<String>> = vec![];
                            let mut colors: Vec<Color32> = vec![];
                            let mut labels: Vec<String> = vec![];
                            if settings.terminal.show_id {
                                labels.push("ID".to_string());
                            }
                            if settings.terminal.show_time {
                                labels.push("Время".to_string());
                            }
                            for (i, msg) in self.ui_data.lock().unwrap().messages.iter().enumerate()
                            {
                                if !settings.terminal.get_is_show(&msg.r#type) {
                                    continue;
                                }

                                let mut cols: Vec<String> = msg
                                    .text
                                    .split(settings.delimeter)
                                    .map(|s| s.trim().to_string())
                                    .collect();
                                if settings.terminal.show_time {
                                    cols.insert(0, format!("<{}>", msg.get_created()));
                                }
                                if settings.terminal.show_id {
                                    cols.insert(0, format!("[{:05}]", i));
                                }
                                rows.push(cols);
                                colors.push(settings.terminal.get_color(&msg.r#type));
                            }

                            if rows.is_empty() {
                                ui.label("Нет данных для отображения в табличном режиме");
                            } else {
                                let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(1);
                                ui.add_space(12.0);
                                let mut table = TableBuilder::new(ui)
                                    .striped(true)
                                    .resizable(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .auto_shrink([false, false])
                                    .min_scrolled_height(0.0)
                                    .max_scroll_height(f32::INFINITY);

                                for i in 0..num_cols {
                                    if (i == 0
                                        && (settings.terminal.show_id
                                            || settings.terminal.show_time))
                                        || (i == 1 && settings.terminal.show_time)
                                    {
                                        table = table.column(Column::auto());
                                    } else {
                                        labels.push(format!("Колонка {}", i + 1));
                                        table = table.column(Column::remainder());
                                    }
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
                                                            || (i == 1
                                                                && settings.terminal.show_time)
                                                        {
                                                            let selectable = if (i == 0)
                                                                && settings.terminal.show_id
                                                            {
                                                                settings.terminal.id_selectable
                                                            } else {
                                                                settings.terminal.time_selectable
                                                            };
                                                            ui.add(
                                                                Label::new(
                                                                    RichText::new(cell)
                                                                        .color(color),
                                                                )
                                                                .selectable(selectable),
                                                            );
                                                        } else {
                                                            ui.label(
                                                                RichText::new(cell.clone())
                                                                    .color(color),
                                                            );
                                                        }
                                                    });
                                                }
                                            });
                                        }
                                    });
                            }
                        }
                    });

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
                }
            });
    }
}
