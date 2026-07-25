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
                state.is_playing = false;
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
        let mut requested_load_file: Option<String> = None;
        let mut requested_reset = false;

        let mut state = self.state.lock().unwrap();
        let mut engine = self.engine.lock().unwrap();

        if ctx.input(|i| i.key_pressed(Key::Space)) && state.file_loaded {
            state.is_playing = !state.is_playing;
            if state.is_playing {
                engine.reset();
            }
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

        let file_loaded = state.file_loaded;
        let file_name = state.file_name.clone();
        let load_error = state.load_error.clone();
        let notes_len = state.notes.len();
        let bpm = state.bpm;
        let time_sig = state.time_sig.clone();
        let is_playing = state.is_playing;
        let total_ticks = state.total_ticks;
        let current_tick = state.current_tick;
        let notes = state.notes.clone();
        drop(state);

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(rect);
            painter.rect_filled(rect, 0.0, Color32::from_rgb(15, 15, 15));

            let sidebar_w = 230.0;
            let kbd_x = rect.min.x + sidebar_w;

            let title_rect = Rect::from_min_max(
                egui::pos2(rect.min.x, rect.min.y),
                egui::pos2(kbd_x, rect.min.y + 100.0),
            );

            ui.allocate_new_ui(UiBuilder::new().max_rect(title_rect), |ui| {
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

            let info_rect = Rect::from_min_max(
                egui::pos2(rect.min.x, rect.min.y + 110.0),
                egui::pos2(kbd_x, rect.min.y + 250.0),
            );
            ui.allocate_new_ui(UiBuilder::new().max_rect(info_rect), |ui| {
                ui.label(format!("文件: {}", file_name));
                ui.label(format!("总音符: {}", notes_len));
                ui.label(format!("BPM: {:.1}", bpm));
                ui.label(format!("拍号: {}", time_sig));
                ui.label(format!("状态: {}", if is_playing { "播放中" } else { "暂停" }));
            });

            if !file_loaded || notes.is_empty() {
                if load_error.is_none() {
                    painter.text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        "点击左上角按钮加载 MIDI 文件",
                        // 使用 proportional 以使用加载的中文字体
                        FontId::proportional(16.0),
                        Color32::GRAY,
                    );
                }
                return;
            }

            let kbd_w = 60.0;
            let kbd_h = rect.height();
            let pnl_x = kbd_x + kbd_w;
            let pnl_w = rect.width() - sidebar_w - kbd_w;

            let pitch_min = 36;
            let pitch_range = 48;
            let pitch_h = kbd_h / pitch_range as f32;

            painter.rect_filled(
                Rect::from_min_max(egui::pos2(pnl_x, rect.min.y), egui::pos2(rect.max.x, rect.max.y)),
                0.0,
                Color32::from_rgb(22, 22, 22),
            );

            for i in 0..=pitch_range {
                let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                let grid_col = if actual_pitch % 12 == 0 {
                    Color32::from_rgb(110, 110, 110)
                } else {
                    Color32::from_rgb(50, 50, 50)
                };
                painter.line_segment(
                    [egui::pos2(pnl_x, y), egui::pos2(rect.max.x, y)],
                    Stroke::new(1.0_f32, grid_col),
                );
            }

            let total_ticks_f = total_ticks as f32;
            for note in &notes {
                let note_pitch = note.pitch as i32;
                if note_pitch < pitch_min || note_pitch > pitch_min + pitch_range {
                    continue;
                }

                let x_start = pnl_x + (note.start_tick as f32 / total_ticks_f) * pnl_w;
                let x_end = pnl_x + ((note.start_tick + note.duration) as f32 / total_ticks_f) * pnl_w;
                if x_end < pnl_x || x_start > pnl_x + pnl_w {
                    continue;
                }

                let rel_pitch = note_pitch - pitch_min;
                let y_center = rect.min.y + ((pitch_range - rel_pitch) as f32 * pitch_h);

                let nw = (x_end - x_start).max(1.5);
                let nh = pitch_h * 0.8;
                let is_active = is_playing && current_tick >= note.start_tick && current_tick <= note.start_tick + note.duration;
                let note_col = if is_active {
                    Color32::from_rgb(50, 200, 255)
                } else {
                    Color32::from_rgb(100, 210, 100)
                };

                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x_start, y_center - nh / 2.0_f32), egui::vec2(nw, nh)),
                    2.0_f32,
                    note_col,
                );
            }

            let play_progress = if total_ticks > 0 {
                current_tick as f32 / total_ticks as f32
            } else {
                0.0
            };
            let cursor_x = pnl_x + play_progress * pnl_w;
            painter.line_segment(
                [egui::pos2(cursor_x, rect.min.y), egui::pos2(cursor_x, rect.max.y)],
                Stroke::new(2.0_f32, Color32::from_rgb(255, 255, 0)),
            );

            painter.rect_filled(
                Rect::from_min_max(egui::pos2(kbd_x, rect.min.y), egui::pos2(kbd_x + kbd_w, rect.max.y)),
                0.0,
                Color32::WHITE,
            );
            let black_w = kbd_w * 0.6;
            for i in 0..=pitch_range {
                let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                painter.line_segment(
                    [egui::pos2(kbd_x, y), egui::pos2(kbd_x + kbd_w, y)],
                    Stroke::new(1.0_f32, Color32::from_rgb(160, 160, 160)),
                );
                let mod12 = actual_pitch % 12;
                if mod12 == 1 || mod12 == 3 || mod12 == 6 || mod12 == 8 || mod12 == 10 {
                    let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                    let black_h = pitch_h * 0.6;
                    painter.rect_filled(
                        Rect::from_min_max(
                            egui::pos2(kbd_x + kbd_w - black_w, y_center - black_h / 2.0_f32),
                            egui::pos2(kbd_x + kbd_w, y_center + black_h / 2.0_f32),
                        ),
                        0.0,
                        Color32::from_rgb(25, 25, 25),
                    );
                }
            }
        });

        if let Some(path) = requested_load_file {
            drop(engine);
            self.load_file(path);
            ctx.request_repaint();
            return;
        }

        if requested_reset {
            let mut state = self.state.lock().unwrap();
            state.is_playing = false;
            state.current_tick = 0;
            engine.reset();
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("MIDIassistant - Rust v0.1.1"),
        ..Default::default()
    };

    // 关键修复：在初始化 eframe 时加载支持中文的字体
    eframe::run_native("MIDIassistant", options, Box::new(|cc| {
        // 配置字体系统
        let mut fonts = egui::FontDefinitions::default();
        // 安装默认支持中文的字体（微软雅黑），如果系统里有的话
        fonts.font_data.insert(
            "msyh".to_owned(),
            egui::FontData::from_static(include_bytes!("c:/Windows/Fonts/msyh.ttc")), // Windows 路径
        );
        // 将中文字体设为最高优先级，并作为后备字体
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "msyh".to_owned());
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .push("msyh".to_owned());
        
        cc.egui_ctx.set_fonts(fonts);

        Ok(Box::new(MidiApp::new()))
    }))
}