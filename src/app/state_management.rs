use eframe::egui::Context;

use crate::app::MidiApp;
use crate::models;

pub fn update_playback(
    app: &mut MidiApp,
    ctx: &Context,
) {
    let (mut state, mut engine) = {
        let state_guard = app.state.lock().unwrap();
        let engine_guard = app.engine.lock().unwrap();
        (state_guard, engine_guard)
    };

    if state.file_loaded && state.is_playing {
        engine.update(&mut state);
        ctx.request_repaint();

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

    drop(engine);
    drop(state);

    // 仅在音频启用时处理回放
    if let Some(audio) = &mut app.audio_engine {
        let mut state = app.state.lock().unwrap();
        if state.settings.enable_audio {
            audio.process_playback(&mut state);
        }
        drop(state);
    }
}

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