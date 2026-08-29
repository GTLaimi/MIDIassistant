use eframe::egui::Context;

use crate::app::MidiApp;
use crate::models;
use crate::chord_detector; // 引入和弦检测模块

/// 更新播放状态（推进 tick，更新和弦，处理持续高亮）
pub fn update_playback(
    app: &mut MidiApp,
    ctx: &Context,
) {
    let (mut state, mut engine) = {
        let state_guard = app.state.lock().unwrap();
        let engine_guard = app.engine.lock().unwrap();
        (state_guard, engine_guard)
    };

    // 1. 推进播放 tick（仅当正在播放）
    if state.file_loaded && state.is_playing {
        engine.update(&mut state);
        ctx.request_repaint();

        // 2. 持续高亮（记录已演奏过的音符）
        if state.settings.sustain_highlight {
            let mut to_insert = Vec::new();
            for (idx, note) in state.notes.iter().enumerate() {
                if note.start_tick <= state.current_tick && state.current_tick <= note.start_tick + note.duration {
                    to_insert.push(idx);
                }
            }
            for idx in to_insert {
                state.played_note_indices.insert(idx);
            }
        }
    }

    // 3. 更新和弦（无论是否播放，只要文件已加载，就基于当前 current_tick 计算）
    if state.file_loaded {
        // 收集当前激活音符的音高
        let active_pitches: Vec<u8> = state.notes
            .iter()
            .filter(|n| state.current_tick >= n.start_tick && state.current_tick <= n.start_tick + n.duration)
            .map(|n| n.pitch)
            .collect();
        state.current_chord = chord_detector::detect_chord(&active_pitches);
    }

    drop(engine);
    drop(state);

    // 4. 音频引擎回放处理（仅当启用音频）
    if let Some(audio) = &mut app.audio_engine {
        let mut state = app.state.lock().unwrap();
        if state.settings.enable_audio {
            audio.process_playback(&mut state);
        }
        drop(state);
    }
}

/// 更新渲染进度（用于导出等）
pub fn update_rendering(
    app: &mut MidiApp,
    ctx: &Context,
) {
    let mut state = app.state.lock().unwrap();

    if state.is_rendering {
        if let Some(audio) = &mut app.audio_engine {
            let (done, progress) = audio.render_next_chunk();
            state.render_progress = progress;
            if done {
                state.is_rendering = false;
                state.render_success = true;
            }
            ctx.request_repaint();
        }
    }
}

/// 收集状态快照（用于渲染线程安全读取）
pub fn collect_state_snapshot(
    app: &MidiApp,
) -> (
    bool,
    String,
    Option<String>,
    Vec<models::note::Note>,
    u32,
    u32,
    bool,
    crate::state::DisplayMode,
    String,
    f32,
    u32,
    bool,
) {
    let state = app.state.lock().unwrap();
    (
        state.file_loaded,
        state.file_name.clone(),
        state.load_error.clone(),
        state.notes.clone(),
        state.total_ticks,
        state.current_tick,
        state.is_playing,
        state.display_mode,
        state.time_sig.clone(),
        state.bpm,
        state.ppq,
        state.fullscreen,
    )
}