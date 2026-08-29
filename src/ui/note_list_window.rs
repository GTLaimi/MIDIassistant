// src/ui/note_list_window.rs
use eframe::egui;
use egui::{Align2, Grid, ScrollArea};
use rfd::FileDialog;
use crate::state::AppState;

pub fn draw_note_list_window(ctx: &egui::Context, state: &mut AppState, open: &mut bool) {
    egui::Window::new("音符列表预览")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .default_size(egui::vec2(350.0, 500.0))
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.style_mut().spacing.scroll.bar_width = 8.0;

            ScrollArea::vertical()
                .max_height(ui.available_height() - 50.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Grid::new("note_list_grid")
                        .num_columns(6)
                        .spacing([12.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("序号");
                            ui.label("开始时间");
                            ui.label("音名");
                            ui.label("编号");
                            ui.label("时长");
                            ui.label("力度");
                            ui.end_row();

                            for (i, note) in state.notes.iter().enumerate() {
                                ui.label(format!("{}", i + 1));
                                ui.label(format!("{}", note.start_tick));
                                ui.label(note.pitch_name.clone());
                                ui.label(format!("{}", note.pitch));
                                ui.label(format!("{}", note.duration));
                                ui.label(format!("{}", note.velocity));
                                ui.end_row();
                            }
                        });
                });

            ui.separator();
            if ui.button("导出文本").clicked() {
                // 使用英文表头，固定宽度对齐
                let mut text_content = String::new();
                text_content.push_str(&format!(
                    "{:<6} {:<12} {:<8} {:<6} {:<8} {:<6}\n",
                    "No.", "Start", "Note", "Pitch", "Duration", "Vel."
                ));
                for (i, note) in state.notes.iter().enumerate() {
                    text_content.push_str(&format!(
                        "{:<6} {:<12} {:<8} {:<6} {:<8} {:<6}\n",
                        i + 1,
                        note.start_tick,
                        note.pitch_name,
                        note.pitch,
                        note.duration,
                        note.velocity
                    ));
                }

                if let Some(path) = FileDialog::new()
                    .add_filter("文本文件", &["txt"])
                    .save_file()
                {
                    if let Err(e) = std::fs::write(path, &text_content) {
                        eprintln!("导出文本失败: {}", e);
                    }
                }
            }
        });
}