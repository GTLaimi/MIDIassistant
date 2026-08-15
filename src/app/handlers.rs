use eframe::egui::{Context, Key};
use crate::state::AppState;
use crate::playback::PlaybackEngine;

pub fn handle_input(
    ctx: &Context,
    state: &mut AppState,
    engine: &mut PlaybackEngine,
) {
    if ctx.input(|i| i.key_pressed(Key::F11)) {
        state.fullscreen = !state.fullscreen;
        ctx.request_repaint();
    }

    if state.is_playing && state.settings.manual_offset_ratio != 0.0 {
        state.settings.manual_offset_ratio = 0.0;
        ctx.request_repaint();
    }

    if ctx.input(|i| i.key_pressed(Key::Space)) && state.file_loaded {
        state.is_playing = !state.is_playing;
        if state.is_playing {
            engine.reset();
            state.settings.manual_offset_ratio = 0.0;
        }
    }

    if ctx.input(|i| i.key_pressed(Key::R)) && state.file_loaded {
        state.is_playing = false;
        state.current_tick = 0;
        state.last_processed_tick = 0;
        engine.reset();
        state.settings.manual_offset_ratio = 0.0;
        state.played_note_indices.clear();
    }
}

pub fn handle_play_button(
    state: &mut AppState,
    engine: &mut PlaybackEngine,
) {
    state.is_playing = !state.is_playing;
    if state.is_playing {
        engine.reset();
        state.settings.manual_offset_ratio = 0.0;
    }
}

pub fn handle_reset_button(
    state: &mut AppState,
    engine: &mut PlaybackEngine,
) {
    state.is_playing = false;
    state.current_tick = 0;
    state.last_processed_tick = 0;
    engine.reset();
    state.settings.manual_offset_ratio = 0.0;
    state.played_note_indices.clear();
}