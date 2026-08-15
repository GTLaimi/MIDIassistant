use std::sync::{Arc, Mutex};
use std::path::Path;

use crate::audio_engine::AudioEngine;
use crate::playback::PlaybackEngine;
use crate::state::AppState;

pub mod handlers;
pub mod render;
pub mod state_management;
pub mod transport;

pub struct MidiApp {
    pub state: Arc<Mutex<AppState>>,
    pub engine: Arc<Mutex<PlaybackEngine>>,
    pub audio_engine: Option<AudioEngine>,
}

impl MidiApp {
    pub fn new() -> Self {
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

    pub fn load_file(&mut self, file_path: String) {
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
        state.played_note_indices.clear();
        drop(state);

        match crate::midi_parser::parse_midi(&file_path) {
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