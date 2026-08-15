use eframe::egui::Context;

use crate::app::MidiApp;
use crate::ui::{
    draw_settings_window, draw_theme_window, draw_note_list_window,
    draw_sidebar, draw_piano_roll,
};
use crate::models;

pub fn render_ui(
    app: &mut MidiApp,
    ctx: &Context,
    requested_load_file: &mut Option<String>,
    notes: &[models::note::Note],
    total_ticks: u32,
    current_tick: u32,
    is_playing: bool,
    time_sig: &str,
    ppq: u32,
    fullscreen: bool,
    bpm: f32,
    active_note_names: &[String],
    bar_beat_str: &str,
    time_str: &str,
) {
    if !fullscreen {
        let mut state = app.state.lock().unwrap();
        draw_sidebar(ctx,&mut state, requested_load_file);
    }

    // 提取所有需要的数据，避免在调用 draw_piano_roll 时同时持有可变借用
        let (settings, notes_vec, total_ticks_val, current_tick_val, is_playing_val, time_sig_val, ppq_val,
            mut fullscreen_val, played_indices, file_name, bpm_val, current_time_str, current_chord, track_name, author, played_count, total_count) = {
        let state = app.state.lock().unwrap();
        // 计算派生数据
        let tick_rate = (state.bpm / 60.0) * state.ppq as f32;
        let secs = state.current_tick as f32 / tick_rate;
        let minutes = (secs / 60.0) as u32;
        let seconds = secs % 60.0;
        let millis = ((seconds - seconds.floor()) * 1000.0) as u32;
        let current_time_str = format!("{:02}:{:02}.{:03}", minutes, seconds as u32, millis);
        let played_notes_count = state.notes.iter().filter(|n| n.start_tick <= state.current_tick).count();

        // 提取所有需要的数据
        (
            state.settings.clone(),           // VisualSettings
            state.notes.clone(),              // Vec<Note>
            state.total_ticks,
            state.current_tick,
            state.is_playing,
            state.time_sig.clone(),
            state.ppq,
            state.fullscreen,                  // bool，稍后写回
            state.played_note_indices.clone(),
            state.file_name.clone(),
            state.bpm,
            current_time_str,
            state.current_chord.clone(),
            state.settings.track_name.clone(),
            state.settings.author.clone(),
            played_notes_count,
            state.notes.len(),
        )
    };

    // 将 settings 设为可变，以便传递可变引用给 draw_piano_roll
    let mut settings = settings;

    // 调用 draw_piano_roll，传递所有数据
    draw_piano_roll(
        ctx,
        &mut settings,          // 需要可变引用，但可以在调用后丢弃
        &notes_vec,
        total_ticks_val,
        current_tick_val,
        is_playing_val,
        &time_sig_val,
        ppq_val,
        &mut fullscreen_val,    // 可能被修改
        &played_indices,
        &file_name,
        bpm_val,
        active_note_names,
        bar_beat_str,
        &current_time_str,      // 注意这里需要 &str
        &current_chord,
        &track_name,
        &author,
        played_count,
        total_count,
    );

    // 如果 fullscreen 被修改，写回 state
    if fullscreen_val != fullscreen {
        let mut state = app.state.lock().unwrap();
        state.fullscreen = fullscreen_val;
        // 同时更新 settings（因为 draw_piano_roll 可能修改了 settings）
        state.settings = settings;
    } else {
        // 即使 fullscreen 未变，也可能修改了 settings
        let mut state = app.state.lock().unwrap();
        state.settings = settings;
    }

    if !fullscreen {
        crate::app::transport::draw_transport(
            ctx,
            app,
            notes,
            total_ticks,
            current_tick,
            is_playing,
            time_sig,
            bpm,
            ppq,
            time_str,
            bar_beat_str,
        );
    }

    {
        let mut state = app.state.lock().unwrap();

        if state.show_settings {
            let mut open = true;
            draw_settings_window(ctx, &mut state, &mut open);
            if !open {
                state.show_settings = false;
            }
        }

        if state.show_theme_window {
            let mut open = true;
            draw_theme_window(ctx, &mut state, &mut open);
            if !open {
                state.show_theme_window = false;
            }
        }

        if state.show_note_list {
            let mut open = true;
            draw_note_list_window(ctx, &mut state, &mut open);
            if !open {
                state.show_note_list = false;
            }
        }
    }
}

pub fn compute_info_data(
    notes: &[models::note::Note],
    current_tick: u32,
    bpm: f32,
    ppq: u32,
    time_sig: &str,
) -> (Vec<String>, String, String) {
    let active_note_names: Vec<String> = notes
        .iter()
        .filter(|n| current_tick >= n.start_tick && current_tick <= n.start_tick + n.duration)
        .map(|n| n.pitch_name.clone())
        .collect();

    let tick_rate = (bpm / 60.0) * ppq as f32;
    let secs = current_tick as f32 / tick_rate;
    let minutes = (secs / 60.0) as u32;
    let seconds = secs % 60.0;
    let millis = ((seconds - seconds.floor()) * 1000.0) as u32;
    let time_str = format!("{:02}:{:02}.{:03}", minutes, seconds as u32, millis);

    let mut beats_per_bar = 4;
    if let Some(slash) = time_sig.find('/') {
        if let Ok(num) = time_sig[..slash].parse::<u32>() {
            beats_per_bar = num;
        }
    }
    let total_beats = current_tick as f32 / ppq as f32;
    let bar = (total_beats / beats_per_bar as f32) as u32;
    let beat = (total_beats % beats_per_bar as f32) as u32;
    let tick_in_beat = current_tick % ppq;
    let bar_beat_str = format!("{}:{}:{:03}", bar + 1, beat + 1, tick_in_beat);

    (active_note_names, time_str, bar_beat_str)
}