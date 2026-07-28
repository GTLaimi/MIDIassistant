#![deny(warnings)]

mod midi_parser;
mod models;
mod playback;
mod state;
mod theme_manager;
mod ui;
mod audio_engine;

use std::sync::{Arc, Mutex};
use std::path::Path;

use eframe::egui;
use eframe::egui::{Color32, Key, Rect};

use playback::PlaybackEngine;
use state::{AppState, DisplayMode};
use audio_engine::AudioEngine;

use ui::{draw_settings_window, draw_theme_window, draw_note_list_window, draw_sidebar, draw_piano_roll};

struct MidiApp {
    state: Arc<Mutex<AppState>>,
    engine: Arc<Mutex<PlaybackEngine>>,
    audio_engine: Option<AudioEngine>,
}

impl MidiApp {
    fn new() -> Self {
        let audio_engine = if Path::new("piano.sf2").exists() {
            match AudioEngine::new("piano.sf2") {
                Ok(engine) => {
                    println!("✅ 音频引擎加载成功");
                    Some(engine)
                }
                Err(e) => {
                    eprintln!("❌ 加载音频引擎失败: {}", e);
                    None
                }
            }
        } else {
            eprintln!("⚠️ 未找到 piano.sf2");
            None
        };

        Self {
            state: Arc::new(Mutex::new(AppState::new())),
            engine: Arc::new(Mutex::new(PlaybackEngine::new(120.0, 480))),
            audio_engine,
        }
    }

    fn load_file(&mut self, file_path: String) {
        let mut state = self.state.lock().unwrap();
        state.notes.clear();
        state.total_ticks = 0;
        state.current_tick = 0;
        state.last_processed_tick = 0;
        state.is_playing = false;
        state.file_loaded = false;
        state.file_name.clear();
        state.load_error = None;
        state.bpm = 120.0;
        state.ppq = 480;
        state.time_sig = "4/4".to_string();
        state.is_rendering = false;
        state.render_progress = 0.0;
        state.render_success = false;
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
                drop(engine);

                if let Some(audio) = &mut self.audio_engine {
                    let state = self.state.lock().unwrap();
                    if state.settings.enable_audio {
                        let notes = state.notes.clone();
                        let total_ticks = state.total_ticks;
                        let bpm = state.bpm;
                        let ppq = state.ppq;
                        drop(state);

                        if let Err(e) = audio.start_pre_render(&notes, total_ticks, bpm, ppq) {
                            eprintln!("预渲染启动失败: {}", e);
                        } else {
                            let mut state = self.state.lock().unwrap();
                            state.is_rendering = true;
                            state.render_progress = 0.0;
                            state.render_success = false;
                        }
                    } else {
                        let mut state = self.state.lock().unwrap();
                        state.is_rendering = false;
                        state.render_progress = 1.0;
                        state.render_success = true;
                    }
                }
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

        // 提取所有需要的数据（使用局部变量，避免借用冲突）
        let (_file_loaded, _file_name, _load_error, notes, total_ticks, current_tick, is_playing, display_mode, time_sig, bpm, ppq) = {
            let mut state = self.state.lock().unwrap();
            let mut engine = self.engine.lock().unwrap();

            if ctx.input(|i| i.key_pressed(Key::Space)) && state.file_loaded {
                state.is_playing = !state.is_playing;
                if state.is_playing { engine.reset(); }
            }
            if ctx.input(|i| i.key_pressed(Key::R)) && state.file_loaded {
                state.is_playing = false;
                state.current_tick = 0;
                state.last_processed_tick = 0;
                engine.reset();
            }

            if state.file_loaded && state.is_playing {
                engine.update(&mut state);
                ctx.request_repaint();
            }

            if state.is_rendering {
                if let Some(audio) = &mut self.audio_engine {
                    let (done, progress) = audio.render_next_chunk();
                    state.render_progress = progress;
                    if done {
                        state.is_rendering = false;
                        state.render_success = true;
                    }
                    ctx.request_repaint();
                }
            }

            if let Some(audio) = &mut self.audio_engine {
                audio.process_playback(&mut state);
            }

            let file_loaded = state.file_loaded;
            let file_name = state.file_name.clone();
            let load_error = state.load_error.clone();
            let notes = state.notes.clone();
            let total_ticks = state.total_ticks;
            let current_tick = state.current_tick;
            let is_playing = state.is_playing;
            let display_mode = state.display_mode;
            let time_sig = state.time_sig.clone();
            let bpm = state.bpm;
            let ppq = state.ppq;

            drop(engine);
            drop(state);

            (file_loaded, file_name, load_error, notes, total_ticks, current_tick, is_playing, display_mode, time_sig, bpm, ppq)
        };

        // 边栏
        {
            let mut state = self.state.lock().unwrap();
            draw_sidebar(ctx, &mut state, &mut requested_load_file);
        }

        // 卷帘窗（传递 &mut settings，其他数据用局部变量）
        {
            let mut state = self.state.lock().unwrap();
            draw_piano_roll(
                ctx,
                &mut state.settings,
                &notes,
                total_ticks,
                current_tick,
                is_playing,
                &time_sig,
                ppq,
            );
        }

        // 底部走带
        egui::TopBottomPanel::bottom("transport")
            .min_height(60.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(2.0);

                    ui.horizontal(|ui| {
                        if ui.button(if is_playing { "暂停" } else { "播放" }).clicked() {
                            let mut state = self.state.lock().unwrap();
                            state.is_playing = !state.is_playing;
                            if state.is_playing {
                                let mut engine = self.engine.lock().unwrap();
                                engine.reset();
                            }
                        }
                        if ui.button("重置 (R)").clicked() {
                            let mut state = self.state.lock().unwrap();
                            state.is_playing = false;
                            state.current_tick = 0;
                            state.last_processed_tick = 0;
                            let mut engine = self.engine.lock().unwrap();
                            engine.reset();
                        }

                        ui.add_space(10.0);
                        ui.label(format!("| {} | {} BPM", time_sig, bpm));

                        ui.add_space(20.0);
                        let tick_rate = (bpm / 60.0) * ppq as f32;
                        let secs = current_tick as f32 / tick_rate;
                        let minutes = (secs / 60.0) as u32;
                        let seconds = secs % 60.0;
                        let millis = ((seconds - seconds.floor()) * 1000.0) as u32;
                        let time_str = format!("{:02}:{:02}.{:03}", minutes, seconds as u32, millis);
                        let percent = if total_ticks > 0 { (current_tick as f32 / total_ticks as f32) * 100.0 } else { 0.0 };
                        let played_notes = notes.iter().filter(|n| n.start_tick <= current_tick).count();
                        let note_percent = if notes.len() > 0 { (played_notes as f32 / notes.len() as f32) * 100.0 } else { 0.0 };
                        let beats_per_bar = 4;
                        let total_beats = current_tick as f32 / ppq as f32;
                        let bar = (total_beats / beats_per_bar as f32) as u32;
                        let beat = (total_beats % beats_per_bar as f32) as u32;
                        let tick_in_beat = current_tick % ppq;

                        let text = match display_mode {
                            DisplayMode::Time => format!("时长: {}", time_str),
                            DisplayMode::TimePercent => format!("进度: {:.1}%", percent),
                            DisplayMode::NoteCount => format!("音符: {}/{}", played_notes, notes.len()),
                            DisplayMode::NotePercent => format!("音符%: {:.1}%", note_percent),
                            DisplayMode::BarBeat => format!("小节: {}:{}:{:03}", bar + 1, beat + 1, tick_in_beat),
                        };

                        let label = egui::Label::new(egui::RichText::new(text).size(14.0)).sense(egui::Sense::click());
                        let response = ui.add(label);
                        if response.clicked() {
                            let mut state = self.state.lock().unwrap();
                            let new_mode = match state.display_mode {
                                DisplayMode::Time => DisplayMode::TimePercent,
                                DisplayMode::TimePercent => DisplayMode::NoteCount,
                                DisplayMode::NoteCount => DisplayMode::NotePercent,
                                DisplayMode::NotePercent => DisplayMode::BarBeat,
                                DisplayMode::BarBeat => DisplayMode::Time,
                            };
                            state.display_mode = new_mode;
                        }

                        ui.add_space(20.0);
                        let (resp, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), 6.0), egui::Sense::drag());
                        let slider_rect = resp.rect;
                        painter.rect_filled(slider_rect, 2.0, Color32::from_rgb(50, 50, 50));
                        if total_ticks > 0 {
                            let progress = current_tick as f32 / total_ticks as f32;
                            let cursor_x = slider_rect.min.x + progress * slider_rect.width();
                            let fill_rect = Rect::from_min_max(egui::pos2(slider_rect.min.x, slider_rect.min.y), egui::pos2(cursor_x, slider_rect.max.y));
                            painter.rect_filled(fill_rect, 2.0, Color32::from_rgb(100, 180, 255));
                            painter.circle_filled(egui::pos2(cursor_x, slider_rect.center().y), 8.0, Color32::WHITE);

                            if resp.drag_started() || resp.dragged() {
                                if let Some(mouse_pos) = ctx.pointer_latest_pos() {
                                    let rel_x = mouse_pos.x - slider_rect.min.x;
                                    let clamped_x = rel_x.clamp(0.0, slider_rect.width());
                                    let new_progress = clamped_x / slider_rect.width();
                                    let mut state = self.state.lock().unwrap();
                                    state.current_tick = (new_progress * total_ticks as f32) as u32;
                                    state.is_playing = false;
                                }
                            }
                        }
                    });

                    ui.add_space(2.0);

                    // 预渲染进度条
                    ui.horizontal(|ui| {
                        let state = self.state.lock().unwrap();
                        if state.is_rendering {
                            ui.label("渲染中:");
                            let (resp, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), 10.0), egui::Sense::hover());
                            let rect = resp.rect;
                            painter.rect_filled(rect, 2.0, Color32::from_rgb(50, 50, 50));
                            let progress = state.render_progress;
                            let fill_rect = Rect::from_min_max(
                                egui::pos2(rect.min.x, rect.min.y),
                                egui::pos2(rect.min.x + progress * rect.width(), rect.max.y),
                            );
                            painter.rect_filled(fill_rect, 2.0, Color32::from_rgb(200, 50, 50));
                            ui.add_space(10.0);
                            ui.label(format!("{:.1}%", progress * 100.0));
                        } else if state.render_success {
                            ui.label("✅ 渲染完毕");
                        } else {
                            ui.label("");
                        }
                        drop(state);
                    });
                });
            });

        // 窗口
        {
            let mut state = self.state.lock().unwrap();

            if state.show_settings {
                let mut open = true;
                draw_settings_window(ctx, &mut state, &mut open);
                if !open { state.show_settings = false; }
            }

            if state.show_theme_window {
                let mut open = true;
                draw_theme_window(ctx, &mut state, &mut open);
                if !open { state.show_theme_window = false; }
            }

            if state.show_note_list {
                let mut open = true;
                draw_note_list_window(ctx, &mut state, &mut open);
                if !open { state.show_note_list = false; }
            }
        }

        if let Some(path) = requested_load_file {
            self.load_file(path);
            ctx.request_repaint();
            return;
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("MIDIassistant"),
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