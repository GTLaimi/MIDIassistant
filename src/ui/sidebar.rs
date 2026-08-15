use eframe::egui;
use crate::state::{AppState, LayoutOrientation};
use rfd::FileDialog;

pub fn draw_sidebar(
    ctx: &egui::Context,
    state: &mut AppState,
    requested_load_file: &mut Option<String>,
) {
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .min_width(120.0)
        .max_width(400.0)
        .show(ctx, |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, egui::Color32::from_rgb(15, 15, 15));

            ui.horizontal(|ui| {
                if ui.button("加载 MIDI 文件").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("MIDI", &["mid", "midi"]).pick_file() {
                        *requested_load_file = Some(path.display().to_string());
                    }
                }
            });
            ui.add_space(5.0);

            if ui.button("设置").clicked() {
                state.show_settings = true;
                ctx.request_repaint();
            }
            ui.add_space(10.0);

            // ---------- 布局切换 ----------
            ui.separator();
            ui.label("卷帘窗布局");
            ui.horizontal(|ui| {
                let current = state.settings.layout_orientation;
                if ui.selectable_label(current == LayoutOrientation::Horizontal, "水平").clicked() {
                    state.settings.layout_orientation = LayoutOrientation::Horizontal;
                    ctx.request_repaint();
                }
                if ui.selectable_label(current == LayoutOrientation::Vertical, "垂直(瀑布流)").clicked() {
                    state.settings.layout_orientation = LayoutOrientation::Vertical;
                    ctx.request_repaint();
                }
                if ui.selectable_label(current == LayoutOrientation::Bar, "小节").clicked() {
                    state.settings.layout_orientation = LayoutOrientation::Bar;
                    ctx.request_repaint();
                }
            });
            ui.add_space(10.0);

            // ---------- 音符持续高亮 ----------
            ui.separator();
            ui.checkbox(&mut state.settings.sustain_highlight, "音符持续高亮");
            ui.add_space(10.0);

            // ---------- 全屏按钮 ----------
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button(if state.fullscreen { "退出全屏" } else { "全屏" }).clicked() {
                    state.fullscreen = !state.fullscreen;
                    ctx.request_repaint();
                }
                ui.label("(F11)");
            });
            ui.add_space(10.0);

            if let Some(err) = &state.load_error {
                ui.colored_label(egui::Color32::from_rgb(255, 80, 80), err);
            } else {
                ui.label("空格键: 播放/暂停");
                ui.label("R键: 重置");
            }

            ui.separator();
            ui.label(format!("文件: {}", state.file_name));
            ui.label(format!("音符数: {}", state.notes.len()));
            ui.label(format!("拍号: {}", state.time_sig));
            ui.label(format!("BPM: {:.1}", state.bpm));
            ui.label(format!("状态: {}", if state.is_playing { "播放中" } else { "暂停" }));

            if state.file_loaded {
                let mut active_note_names = Vec::new();
                for n in &state.notes {
                    if state.current_tick >= n.start_tick && state.current_tick <= n.start_tick + n.duration {
                        active_note_names.push(n.pitch_name.clone());
                    }
                }
                if active_note_names.is_empty() {
                    ui.label("当前音符: 无");
                } else {
                    ui.label(format!("当前音符: {}", active_note_names.join(", ")));
                }
                if let Some(chord) = &state.current_chord {
                    ui.label(format!("和弦: {}", chord));
                } else {
                    ui.label("和弦: (无)");
                }
                ui.separator();
            }

            if state.file_loaded {
                if ui.button("音符列表").clicked() {
                    state.show_note_list = true;
                    ctx.request_repaint();
                }
            }
        });
}