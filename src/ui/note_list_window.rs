// src/ui/note_list_window.rs
use eframe::egui;
use egui::Align2;
use rfd::FileDialog;
use crate::state::AppState;

pub fn draw_note_list_window(ctx: &egui::Context, state: &mut AppState, open: &mut bool) {
    egui::Window::new("音符列表预览")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .default_size(egui::vec2(550.0, 400.0))
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.style_mut().spacing.scroll.bar_width = 6.0;

            let mut text_content = String::new();
            text_content.push_str("序号, 开始时间(Tick), 音高名称(编号), 时长(Tick), 力度\n");
            for (i, note) in state.notes.iter().enumerate() {
                text_content.push_str(&format!(
                    "{:4}, {:10}, {:6} ({:3}), {:10}, {:3}\n",
                    i + 1,
                    note.start_tick,
                    note.pitch_name,
                    note.pitch,
                    note.duration,
                    note.velocity
                ));
            }

            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 40.0)
                .show(ui, |ui| {
                    ui.monospace(&text_content);
                });

            ui.separator();
            if ui.button("导出文本").clicked() {
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