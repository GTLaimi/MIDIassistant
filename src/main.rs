#![deny(warnings)]

mod midi_parser;
mod models;
mod playback;
mod state;

use std::sync::{Arc, Mutex};

use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Key, Rect, Stroke, UiBuilder};
use rfd::FileDialog;

use playback::PlaybackEngine;
use state::AppState;

struct MidiApp {
    state: Arc<Mutex<AppState>>,
    engine: Arc<Mutex<PlaybackEngine>>,
}

impl MidiApp {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AppState::new())),
            engine: Arc::new(Mutex::new(PlaybackEngine::new(120.0, 480))),
        }
    }

    fn load_file(&self, file_path: String) {
        let mut state = self.state.lock().unwrap();
        *state = AppState::new();
        drop(state);

        match midi_parser::parse_midi(&file_path) {
            Ok((notes, total_ticks, bpm, ppq, time_sig)) => {
                let mut state = self.state.lock().unwrap();
                state.notes = notes;
                state.total_ticks = total_ticks;
                state.bpm = bpm;
                state.ppq = ppq;
                state.time_sig = time_sig;
                state.file_loaded = true;
                state.file_name = file_path;
                state.load_error = None;
                drop(state);

                let mut engine = self.engine.lock().unwrap();
                *engine = PlaybackEngine::new(bpm, ppq);
                engine.reset();
            }
            Err(e) => {
                let mut state = self.state.lock().unwrap();
                state.load_error = Some(format!("解析失败: {}", e));
            }
        }
    }
}

impl eframe::App for MidiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut requested_load_file = None;
        let mut requested_reset = false;

        let mut state = self.state.lock().unwrap();
        let mut engine = self.engine.lock().unwrap();

        if ctx.input(|i| i.key_pressed(Key::Space)) && state.file_loaded {
            state.is_playing = !state.is_playing;
            if state.is_playing { engine.reset(); }
        }
        if ctx.input(|i| i.key_pressed(Key::R)) && state.file_loaded {
            state.is_playing = false;
            state.current_tick = 0;
            engine.reset();
        }

        if state.file_loaded && state.is_playing {
            engine.update(&mut state);
            ctx.request_repaint();
        }

        // 预提取绘图数据
        let file_loaded = state.file_loaded;
        let file_name = state.file_name.clone();
        let load_error = state.load_error.clone();
        let notes = state.notes.clone();
        let total_ticks = state.total_ticks;
        let current_tick = state.current_tick;
        let is_playing = state.is_playing;

        drop(engine); // 释放引擎锁，让 UI 渲染顺畅

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(rect);

            // =====================================================
            // 【核心修复】直接获取可修改的 settings 引用，而非克隆体！
            // =====================================================
            let settings = &mut state.settings;

            let bg_color = Color32::from_rgb(
                (settings.bg_r * 255.0) as u8,
                (settings.bg_g * 255.0) as u8,
                (settings.bg_b * 255.0) as u8,
            );
            painter.rect_filled(rect, 0.0, bg_color);

            let sidebar_w = 230.0;
            let kbd_x = rect.min.x + sidebar_w;

            // 1. 基础操作区
            ui.allocate_new_ui(UiBuilder::new().max_rect(
                Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y), egui::pos2(kbd_x, rect.min.y + 60.0))
            ), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("打开 MIDI 文件").clicked() {
                        if let Some(path) = FileDialog::new().add_filter("MIDI", &["mid", "midi"]).pick_file() {
                            requested_load_file = Some(path.display().to_string());
                        }
                    }
                    if file_loaded && ui.button("重置 (R)").clicked() {
                        requested_reset = true;
                    }
                });
                if let Some(err) = &load_error {
                    ui.colored_label(Color32::from_rgb(255, 80, 80), err);
                } else {
                    ui.label("空格键: 播放/暂停");
                    ui.label("R键: 重置");
                }
            });

            // 2. 折叠式设置面板
            ui.allocate_new_ui(UiBuilder::new().max_rect(
                Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 70.0), egui::pos2(kbd_x, rect.min.y + 310.0))
            ), |ui| {
                ui.collapsing("视觉设置 / Settings", |ui| {
                    // 颜色
                    ui.label("背景 RGB:");
                    ui.horizontal(|ui| {
                        ui.label("R"); ui.add(egui::DragValue::new(&mut settings.bg_r).speed(0.005).range(0.0..=1.0));
                        ui.label("G"); ui.add(egui::DragValue::new(&mut settings.bg_g).speed(0.005).range(0.0..=1.0));
                        ui.label("B"); ui.add(egui::DragValue::new(&mut settings.bg_b).speed(0.005).range(0.0..=1.0));
                        let preview_rect = Rect::from_min_max(ui.min_rect().max - egui::vec2(80.0, 0.0), ui.min_rect().max - egui::vec2(60.0, 0.0));
                        ui.painter().rect_filled(preview_rect, 0.0, bg_color);
                    });

                    ui.separator();
                    // 键盘与音符
                    ui.horizontal(|ui| {
                        ui.label("白键宽:"); ui.add(egui::DragValue::new(&mut settings.kbd_width).speed(1.0).range(20.0..=150.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("黑键宽:"); ui.add(egui::DragValue::new(&mut settings.black_key_width).speed(1.0).range(5.0..=50.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("黑键高:"); ui.add(egui::DragValue::new(&mut settings.black_key_height_ratio).speed(0.01).range(0.3..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("音符高度:"); ui.add(egui::DragValue::new(&mut settings.note_height_ratio).speed(0.01).range(0.1..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("音名微调Y:"); ui.add(egui::DragValue::new(&mut settings.label_y_offset).speed(0.5).range(-200.0..=200.0));
                    });

                    ui.separator();
                    // 垂直滚动滑条
                    ui.label("垂直滚动音域:");
                    ui.add(egui::Slider::new(&mut settings.pitch_min, 0..=72).text("基准音高"));
                });
            });

            // 3. 底部文件信息区
            ui.allocate_new_ui(UiBuilder::new().max_rect(
                Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 320.0), egui::pos2(kbd_x, rect.min.y + 430.0))
            ), |ui| {
                ui.separator();
                ui.label(format!("文件: {}", file_name));
                ui.label(format!("音符数: {}", notes.len()));
            });

            // =====================================================
            // 卷帘窗绘图
            // =====================================================
            if !file_loaded || notes.is_empty() {
                if load_error.is_none() {
                    painter.text(rect.center(), Align2::CENTER_CENTER, "点击左上角加载 MIDI 文件", FontId::proportional(16.0), Color32::GRAY);
                }
                return;
            }

            let kbd_w = settings.kbd_width;
            let kbd_h = rect.height();
            let pnl_x = kbd_x + kbd_w;
            let pnl_w = rect.width() - sidebar_w - kbd_w;

            let pitch_min = settings.pitch_min as i32;
            let pitch_range = 48;
            let pitch_h = kbd_h / pitch_range as f32;

            painter.rect_filled(Rect::from_min_max(egui::pos2(pnl_x, rect.min.y), egui::pos2(rect.max.x, rect.max.y)), 0.0, Color32::from_rgb(22, 22, 22));

            for i in 0..=pitch_range {
                let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                let grid_col = if actual_pitch % 12 == 0 { Color32::from_rgb(110, 110, 110) } else { Color32::from_rgb(50, 50, 50) };
                painter.line_segment([egui::pos2(pnl_x, y), egui::pos2(rect.max.x, y)], Stroke::new(1.0_f32, grid_col));
            }

            let total_ticks_f = total_ticks as f32;
            for note in &notes {
                let note_pitch = note.pitch as i32;
                if note_pitch < pitch_min || note_pitch > pitch_min + pitch_range { continue; }

                let x_start = pnl_x + (note.start_tick as f32 / total_ticks_f) * pnl_w;
                let x_end = pnl_x + ((note.start_tick + note.duration) as f32 / total_ticks_f) * pnl_w;
                if x_end < pnl_x || x_start > pnl_x + pnl_w { continue; }

                let rel_pitch = note_pitch - pitch_min;
                let y_center = rect.min.y + ((pitch_range - rel_pitch) as f32 * pitch_h);

                let nw = (x_end - x_start).max(1.5);
                let nh = pitch_h * settings.note_height_ratio;
                let is_active = is_playing && current_tick >= note.start_tick && current_tick <= note.start_tick + note.duration;
                let note_col = if is_active { Color32::from_rgb(50, 200, 255) } else { Color32::from_rgb(100, 210, 100) };

                painter.rect_filled(egui::Rect::from_min_size(egui::pos2(x_start, y_center - nh / 2.0_f32), egui::vec2(nw, nh)), 2.0_f32, note_col);
            }

            let play_progress = if total_ticks > 0 { current_tick as f32 / total_ticks as f32 } else { 0.0 };
            let cursor_x = pnl_x + play_progress * pnl_w;
            painter.line_segment([egui::pos2(cursor_x, rect.min.y), egui::pos2(cursor_x, rect.max.y)], Stroke::new(2.0_f32, Color32::from_rgb(255, 255, 0)));

            let black_w = settings.black_key_width;
            let black_h = pitch_h * settings.black_key_height_ratio;

            for i in 0..=pitch_range {
                let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                let mod12 = actual_pitch % 12;
                if mod12 == 0 || mod12 == 2 || mod12 == 4 || mod12 == 5 || mod12 == 7 || mod12 == 9 || mod12 == 11 {
                    let white_rect = Rect::from_min_max(egui::pos2(kbd_x, y_center - pitch_h / 2.0_f32), egui::pos2(kbd_x + kbd_w, y_center + pitch_h / 2.0_f32));
                    painter.rect_filled(white_rect, 0.0, Color32::WHITE);
                    painter.rect_stroke(white_rect, 0.0, Stroke::new(0.5_f32, Color32::from_rgb(180, 180, 180)));
                }
            }

            for i in 0..=pitch_range {
                let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                let mod12 = actual_pitch % 12;
                if mod12 == 1 || mod12 == 3 || mod12 == 6 || mod12 == 8 || mod12 == 10 {
                    let black_rect = Rect::from_min_max(egui::pos2(kbd_x + kbd_w - black_w, y_center - black_h / 2.0_f32), egui::pos2(kbd_x + kbd_w, y_center + black_h / 2.0_f32));
                    painter.rect_filled(black_rect, 0.0, Color32::from_rgb(20, 20, 20));
                    painter.rect_stroke(black_rect, 0.0, Stroke::new(0.5_f32, Color32::BLACK));
                }
            }

            for i in 0..=pitch_range {
                let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                if actual_pitch % 12 == 0 {
                    let note_name = models::note::Note::new(actual_pitch as u8, 0, 0, 0).pitch_name;
                    let text_pos_y = y_center - pitch_h / 2.0_f32 + settings.label_y_offset;
                    painter.text(egui::pos2(kbd_x + 5.0, text_pos_y), egui::Align2::LEFT_TOP, note_name, FontId::proportional(11.0), Color32::BLACK);
                }
            }
        });

        // 【核心修复 2】退出 UI 绘制后，必须立刻释放 state 锁，避免后面的 load_file 出现死锁
        drop(state);

        if let Some(path) = requested_load_file {
            self.load_file(path);
            ctx.request_repaint();
            return;
        }

        if requested_reset {
            let mut state = self.state.lock().unwrap();
            state.is_playing = false;
            state.current_tick = 0;
            let mut engine = self.engine.lock().unwrap();
            engine.reset();
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]).with_min_inner_size([800.0, 600.0]).with_title("MIDIassistant - Rust v0.1.1"),
        ..Default::default()
    };

    eframe::run_native("MIDIassistant", options, Box::new(|cc| {
        let mut fonts = egui::FontDefinitions::default();
        if cfg!(target_os = "windows") {
            if let Ok(data) = std::fs::read("c:/Windows/Fonts/msyh.ttc") {
                fonts.font_data.insert("msyh".to_owned(), egui::FontData::from_owned(data).into());
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "msyh".to_owned());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("msyh".to_owned());
            }
        }
        cc.egui_ctx.set_fonts(fonts);
        Ok(Box::new(MidiApp::new()))
    }))
}