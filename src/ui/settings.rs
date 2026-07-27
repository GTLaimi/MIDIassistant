use eframe::egui;
use egui::Align2;
use crate::state::{AppState, SettingsTab};
use crate::theme_manager;

pub fn draw_settings_window(ctx: &egui::Context, state: &mut AppState, open: &mut bool) {
    egui::Window::new("设置")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.label("v0.1.4 — 全面自定义");
            ui.separator();

            ui.horizontal(|ui| {
                ui.selectable_value(&mut state.settings_tab, SettingsTab::Colors, "颜色");
                ui.selectable_value(&mut state.settings_tab, SettingsTab::Layout, "布局");
                ui.selectable_value(&mut state.settings_tab, SettingsTab::About, "关于");
            });
            ui.separator();

            match state.settings_tab {
                SettingsTab::Colors => {
                    ui.columns(3, |cols| {
                        cols[0].label("背景与键盘");
                        cols[0].separator();
                        cols[0].label("卷帘窗背景");
                        cols[0].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.piano_bg_r, 0..=255)); });
                        cols[0].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.piano_bg_g, 0..=255)); });
                        cols[0].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.piano_bg_b, 0..=255)); });
                        cols[0].separator();
                        cols[0].label("白键色");
                        cols[0].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.white_key_r, 0..=255)); });
                        cols[0].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.white_key_g, 0..=255)); });
                        cols[0].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.white_key_b, 0..=255)); });
                        cols[0].separator();
                        cols[0].label("黑键色");
                        cols[0].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.black_key_r, 0..=255)); });
                        cols[0].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.black_key_g, 0..=255)); });
                        cols[0].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.black_key_b, 0..=255)); });

                        cols[1].label("音符与指针");
                        cols[1].separator();
                        cols[1].label("常规音符色");
                        cols[1].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.note_r, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.note_g, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.note_b, 0..=255)); });
                        cols[1].separator();
                        cols[1].label("高亮音符色");
                        cols[1].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.active_note_r, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.active_note_g, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.active_note_b, 0..=255)); });
                        cols[1].separator();
                        cols[1].label("指针颜色");
                        cols[1].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.cursor_r, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.cursor_g, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.cursor_b, 0..=255)); });
                        cols[1].separator();
                        cols[1].label("音名色");
                        cols[1].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.text_r, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.text_g, 0..=255)); });
                        cols[1].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.text_b, 0..=255)); });

                        cols[2].label("网格颜色");
                        cols[2].separator();
                        cols[2].label("网格亮线 (八度/小节)");
                        cols[2].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.grid_bright_r, 0..=255)); });
                        cols[2].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.grid_bright_g, 0..=255)); });
                        cols[2].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.grid_bright_b, 0..=255)); });
                        cols[2].separator();
                        cols[2].label("网格暗线 (半音/拍)");
                        cols[2].horizontal(|ui| { ui.label("R"); ui.add(egui::Slider::new(&mut state.settings.grid_dim_r, 0..=255)); });
                        cols[2].horizontal(|ui| { ui.label("G"); ui.add(egui::Slider::new(&mut state.settings.grid_dim_g, 0..=255)); });
                        cols[2].horizontal(|ui| { ui.label("B"); ui.add(egui::Slider::new(&mut state.settings.grid_dim_b, 0..=255)); });
                    });

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("应用预设主题 / 参数选项").clicked() {
                            state.show_theme_window = true;
                            ctx.request_repaint();
                        }
                        if ui.button("保存当前参数为主题文件").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name("my_theme.toml")
                                .add_filter("Theme file", &["toml"])
                                .save_file()
                            {
                                let theme_name = path.file_stem().unwrap().to_str().unwrap();
                                if let Err(e) = theme_manager::save_custom_theme(theme_name, &state.settings) {
                                    eprintln!("保存主题失败: {}", e);
                                }
                            }
                        }
                    });
                }
                SettingsTab::Layout => {
                    ui.label("垂直滚动 (基准音高)");
                    ui.add(egui::Slider::new(&mut state.settings.pitch_min, 0..=96).text("Pitch Min"));
                    ui.label("垂直缩放 (显示半音数)");
                    ui.add(egui::Slider::new(&mut state.settings.pitch_range, 12..=96).text("半音数"));
                    ui.separator();
                    ui.label("水平缩放 (0.05 ~ 0.9)");
                    ui.add(egui::Slider::new(&mut state.settings.time_zoom, 0.05..=0.9).text("zoom"));
                    ui.separator();
                    ui.checkbox(&mut state.settings.show_cursor, "显示指针");
                    ui.separator();
                    ui.checkbox(&mut state.settings.follow_playhead, "跟随播放头自动移动");
                }
                SettingsTab::About => {
                    ui.label("介绍：MIDIassistant 是一个轻量级、高精度的 MIDI 播放与可视化工具。");
                    ui.label("帮助：按 空格键 播放/暂停，按 R 键 重置。");
                    ui.label("版本：v0.1.4");
                    ui.label("版权：MIT License");
                }
            }
        });
}