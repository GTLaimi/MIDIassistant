use eframe::egui;
use egui::Align2;
use crate::state::{AppState, SettingsTab, FollowMode, InfoField, ALL_INFO_FIELDS};
use crate::theme_manager;
use crate::font_loader;

pub fn draw_settings_window(ctx: &egui::Context, state: &mut AppState, open: &mut bool) {
    // 获取可用字体列表
    let font_names = font_loader::get_available_font_names();

    egui::Window::new("设置")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.label("v0.2.0 — 交互增强");
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
                    ui.collapsing("通用设置", |ui| {
                        ui.add(egui::Slider::new(&mut state.settings.pitch_min, 0..=96).text("基准音高 (Pitch Min)"));
                        ui.add(egui::Slider::new(&mut state.settings.pitch_range, 12..=96).text("显示半音数"));
                        ui.add(egui::Slider::new(&mut state.settings.time_zoom, 0.02..=0.9).text("水平缩放"));
                        ui.separator();

                        ui.label("播放头跟随模式");
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut state.settings.follow_mode, FollowMode::Edge, "边缘");
                            ui.radio_value(&mut state.settings.follow_mode, FollowMode::Center, "居中");
                            ui.radio_value(&mut state.settings.follow_mode, FollowMode::Right, "偏右");
                        });
                        if state.settings.follow_mode != FollowMode::Edge {
                            ui.add(egui::Slider::new(&mut state.settings.manual_offset_ratio, 0.0..=1.0).text("手动偏移"));
                        } else {
                            ui.label("边缘模式下视图跟随播放头，不显示指针。");
                        }
                        ui.separator();

                        ui.checkbox(&mut state.settings.show_cursor, "显示指针");
                        ui.checkbox(&mut state.settings.enable_audio, "开启音频并预渲染");
                        if !state.settings.enable_audio {
                            ui.label("（关闭后，加载 MIDI 时将不会生成音频）");
                        }
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("重置钢琴键盘为默认大小").clicked() {
                                state.settings.horizontal_keyboard_width = 50.0;
                                state.settings.vertical_keyboard_height = 80.0;
                                state.settings.vertical_black_key_offset = 0.5;
                                state.settings.vertical_black_key_width_scale = 0.9;
                                ctx.request_repaint();
                            }
                            ui.label("（水平50px，垂直80px）");
                        });
                    });

                    ui.collapsing("小节视图设置", |ui| {
                        ui.checkbox(&mut state.settings.bar_view.visible, "显示方框边框（仍绘制内容）");
                        ui.add(egui::Slider::new(&mut state.settings.bar_view.width_ratio, 0.3..=1.0).text("宽度比例"));
                        ui.add(egui::Slider::new(&mut state.settings.bar_view.height_ratio, 0.3..=1.0).text("高度比例"));
                        let mut degrees = state.settings.bar_view.rotation.to_degrees();
                        ui.add(egui::Slider::new(&mut degrees, -180.0..=180.0).text("旋转角度 (°)"));
                        state.settings.bar_view.rotation = degrees.to_radians();
                        ui.add(egui::Slider::new(&mut state.settings.bar_view.display_pitch_range, 12..=72).text("音高范围 (半音数)"));
                        ui.label("（可在卷帘窗中拖拽移动位置）");
                    });

                    ui.collapsing("曲目信息", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("曲目名");
                            ui.add(egui::TextEdit::singleline(&mut state.settings.track_name).hint_text("请输入曲目名"));
                        });
                        ui.horizontal(|ui| {
                            ui.label("作者");
                            ui.add(egui::TextEdit::singleline(&mut state.settings.author).hint_text("请输入作者"));
                        });
                    });

                    ui.collapsing("信息覆盖设置", |ui| {
                        ui.label("信息文本颜色");
                        ui.horizontal(|ui| {
                            ui.label("R");
                            ui.add(egui::Slider::new(&mut state.settings.info_overlay.text_color_r, 0..=255));
                        });
                        ui.horizontal(|ui| {
                            ui.label("G");
                            ui.add(egui::Slider::new(&mut state.settings.info_overlay.text_color_g, 0..=255));
                        });
                        ui.horizontal(|ui| {
                            ui.label("B");
                            ui.add(egui::Slider::new(&mut state.settings.info_overlay.text_color_b, 0..=255));
                        });
                        ui.separator();

                        ui.checkbox(&mut state.settings.info_overlay.show_border, "显示边框");
                        ui.add(egui::Slider::new(&mut state.settings.info_overlay.background_opacity, 0.0..=1.0).text("背景透明度"));
                        ui.separator();

                        ui.label("显示字段、缩放与字体：");
                        // 确保 field_font_names 和 field_scales 长度足够
                        while state.settings.info_overlay.field_font_names.len() < ALL_INFO_FIELDS.len() {
                            state.settings.info_overlay.field_font_names.push("proportional".to_string());
                        }
                        while state.settings.info_overlay.field_scales.len() < ALL_INFO_FIELDS.len() {
                            state.settings.info_overlay.field_scales.push(1.0);
                        }

                        egui::Grid::new("info_fields_grid")
                            .num_columns(3)
                            .spacing([20.0, 6.0])
                            .show(ui, |ui| {
                                for (idx, &field) in ALL_INFO_FIELDS.iter().enumerate() {
                                    let mut checked = state.settings.info_overlay.enabled_fields.contains(&field);
                                    let mut scale = state.settings.info_overlay.field_scales[idx];
                                    let mut font_name = state.settings.info_overlay.field_font_names[idx].clone();

                                    let label = match field {
                                        InfoField::TrackName => "曲目名",
                                        InfoField::Author => "作者",
                                        InfoField::NoteCount => "音符计数",
                                        InfoField::TimeSig => "拍号",
                                        InfoField::Bpm => "BPM",
                                        InfoField::CurrentTime => "当前时间",
                                        InfoField::ProgressPercent => "进度",
                                        InfoField::ActiveNotes => "当前音符",
                                        InfoField::BarBeat => "小节/拍",
                                        InfoField::Chord => "和弦",
                                    };

                                    if ui.checkbox(&mut checked, label).changed() {
                                        if checked {
                                            if !state.settings.info_overlay.enabled_fields.contains(&field) {
                                                state.settings.info_overlay.enabled_fields.push(field);
                                            }
                                        } else {
                                            state.settings.info_overlay.enabled_fields.retain(|f| *f != field);
                                        }
                                    }

                                    ui.add(egui::Slider::new(&mut scale, 0.5..=2.0).text("缩放").smallest_positive(0.1));
                                    state.settings.info_overlay.field_scales[idx] = scale;

                                    // 字体下拉选择框
                                    egui::ComboBox::from_id_salt(format!("font_{}", idx))
                                        .selected_text(&font_name)
                                        .show_ui(ui, |ui| {
                                            for f in &font_names {
                                                if ui.selectable_value(&mut font_name, f.clone(), f).changed() {
                                                    state.settings.info_overlay.field_font_names[idx] = font_name.clone();
                                                }
                                            }
                                        });
                                    ui.end_row();
                                }
                            });
                        ui.label("（可在卷帘窗中拖拽移动位置）");
                    });
                }
                SettingsTab::About => {
                    ui.label("hesychiamids 是一个轻量级、高精度的 MIDI 播放与可视化工具。");
                    ui.label("控制：空格键 播放/暂停，R 键 重置。");
                    ui.label("版本：v0.2.0");
                    ui.label("许可证：MIT");
                }
            }
        });
}