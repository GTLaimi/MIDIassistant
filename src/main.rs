#![deny(warnings)]

mod midi_parser;
mod models;
mod playback;
mod state;

use std::sync::Arc;
use std::sync::Mutex;

use eframe::egui;
use egui::Color32;
use eframe::egui::Key;

use playback::PlaybackEngine;
use state::AppState;

struct MidiApp {
    state: Arc<Mutex<AppState>>,
    engine: Arc<Mutex<PlaybackEngine>>,
}

impl MidiApp {
    fn new() -> Self {
        let state = Arc::new(Mutex::new(AppState::new()));
        let engine = Arc::new(Mutex::new(PlaybackEngine::new(120.0, 480)));

        let file_path = "test.mid";
        if let Ok((notes, total, bpm, ppq, ts)) = midi_parser::parse_midi(file_path) {
            let mut s = state.lock().unwrap();
            s.notes = notes;
            s.total_ticks = total;
            s.bpm = bpm;
            s.ppq = ppq;
            s.time_sig = ts;
            s.file_loaded = true;
            s.file_name = file_path.to_string();
            s.is_playing = true; 

            drop(s);
            let mut e = engine.lock().unwrap();
            *e = PlaybackEngine::new(bpm, ppq);
            e.reset();
        }
        Self { state, engine }
    }
}

impl eframe::App for MidiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            let painter = ui.painter().with_clip_rect(rect);
            painter.rect_filled(rect, 0.0, Color32::from_rgb(15, 15, 15));

            if !state.file_loaded || state.notes.is_empty() {
                return;
            }

            let sidebar_w = 230.0;
            let kbd_w = 60.0;
            let kbd_x = rect.min.x + sidebar_w;
            let kbd_h = rect.height();
            let pnl_x = kbd_x + kbd_w;
            let pnl_w = rect.width() - sidebar_w - kbd_w;

            let pitch_min = 36;
            let pitch_range = 48;
            let pitch_h = kbd_h / pitch_range as f32;

            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(pnl_x, rect.min.y), egui::pos2(rect.max.x, rect.max.y)),
                0.0,
                Color32::from_rgb(22, 22, 22),
            );

            // 【修正】绘制网格 (现在低音在底部，高音在顶部)
            for i in 0..=pitch_range {
                let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                let grid_col = if actual_pitch % 12 == 0 { Color32::from_rgb(110, 110, 110) } else { Color32::from_rgb(50, 50, 50) };
                painter.line_segment([egui::pos2(pnl_x, y), egui::pos2(rect.max.x, y)], egui::Stroke::new(1.0_f32, grid_col));
            }

            let total_ticks_f = state.total_ticks as f32;
            for note in &state.notes {
                let note_pitch = note.pitch as i32;
                if note_pitch < pitch_min || note_pitch > pitch_min + pitch_range { continue; }

                let x_start = pnl_x + (note.start_tick as f32 / total_ticks_f) * pnl_w;
                let x_end = pnl_x + ((note.start_tick + note.duration) as f32 / total_ticks_f) * pnl_w;
                let rel_pitch = note_pitch - pitch_min;
                
                // 【修正核心】从下往上计算 Y 轴，纠正上下颠倒
                let y_center = rect.min.y + ((pitch_range - rel_pitch) as f32 * pitch_h);

                let nw = (x_end - x_start).max(1.5);
                let nh = pitch_h * 0.8;

                let is_active = state.is_playing && state.current_tick >= note.start_tick && state.current_tick <= note.start_tick + note.duration;
                let note_col = if is_active { Color32::from_rgb(50, 200, 255) } else { Color32::from_rgb(100, 210, 100) };

                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x_start, y_center - nh / 2.0_f32), egui::vec2(nw, nh)),
                    2.0_f32,
                    note_col,
                );
            }

            let play_progress = if state.total_ticks > 0 { state.current_tick as f32 / state.total_ticks as f32 } else { 0.0 };
            let cursor_x = pnl_x + play_progress * pnl_w;
            painter.line_segment([egui::pos2(cursor_x, rect.min.y), egui::pos2(cursor_x, rect.max.y)], egui::Stroke::new(2.0_f32, Color32::from_rgb(255, 255, 0)));

            // 【修正】钢琴键盘映射
            painter.rect_filled(egui::Rect::from_min_max(egui::pos2(kbd_x, rect.min.y), egui::pos2(kbd_x + kbd_w, rect.max.y)), 0.0, Color32::WHITE);
            let black_w = kbd_w * 0.6;
            for i in 0..=pitch_range {
                let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                let actual_pitch = pitch_min + i;
                painter.line_segment([egui::pos2(kbd_x, y), egui::pos2(kbd_x + kbd_w, y)], egui::Stroke::new(1.0_f32, Color32::from_rgb(160, 160, 160)));
                let mod12 = actual_pitch % 12;
                if mod12 == 1 || mod12 == 3 || mod12 == 6 || mod12 == 8 || mod12 == 10 {
                    let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
                    let black_h = pitch_h * 0.6;
                    painter.rect_filled(egui::Rect::from_min_max(egui::pos2(kbd_x + kbd_w - black_w, y_center - black_h / 2.0), egui::pos2(kbd_x + kbd_w, y_center + black_h / 2.0)), 0.0, Color32::from_rgb(25, 25, 25));
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]).with_min_inner_size([800.0, 600.0]).with_title("MIDIassistant - Rust"),
        ..Default::default()
    };
    eframe::run_native("MIDIassistant - Rust", options, Box::new(|_cc| Ok(Box::new(MidiApp::new()))))
}