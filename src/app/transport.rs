use eframe::egui;
use eframe::egui::{Color32, Rect};

use crate::app::MidiApp;
use crate::app::handlers::{handle_play_button, handle_reset_button};
use crate::state::DisplayMode;
use crate::models;

pub fn draw_transport(
    ctx: &egui::Context,
    app: &mut MidiApp,
    notes: &[models::note::Note],
    total_ticks: u32,
    current_tick: u32,
    is_playing: bool,
    time_sig: &str,
    bpm: f32,
    _ppq: u32,
    time_str: &str,
    bar_beat_str: &str,
) {
    egui::TopBottomPanel::bottom("transport")
        .min_height(60.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    if ui.button(if is_playing { "暂停" } else { "播放" }).clicked() {
                        let mut state = app.state.lock().unwrap();
                        let mut engine = app.engine.lock().unwrap();
                        handle_play_button(&mut state, &mut engine);
                    }

                    if ui.button("重置 (R)").clicked() {
                        let mut state = app.state.lock().unwrap();
                        let mut engine = app.engine.lock().unwrap();
                        handle_reset_button(&mut state, &mut engine);
                    }

                    ui.add_space(10.0);
                    ui.label(format!("| {} | {} BPM", time_sig, bpm));

                    ui.add_space(20.0);
                    let display_text = {
                        let percent = if total_ticks > 0 {
                            (current_tick as f32 / total_ticks as f32) * 100.0
                        } else {
                            0.0
                        };
                        let played_notes = notes.iter().filter(|n| n.start_tick <= current_tick).count();
                        let note_percent = if notes.len() > 0 {
                            (played_notes as f32 / notes.len() as f32) * 100.0
                        } else {
                            0.0
                        };
                        match app.state.lock().unwrap().display_mode {
                            DisplayMode::Time => format!("时长: {}", time_str),
                            DisplayMode::TimePercent => format!("进度: {:.1}%", percent),
                            DisplayMode::NoteCount => format!("音符: {}/{}", played_notes, notes.len()),
                            DisplayMode::NotePercent => format!("音符%: {:.1}%", note_percent),
                            DisplayMode::BarBeat => format!("小节: {}", bar_beat_str),
                        }
                    };

                    let label = egui::Label::new(egui::RichText::new(display_text).size(14.0))
                        .sense(egui::Sense::click());
                    let response = ui.add(label);
                    if response.clicked() {
                        let mut state = app.state.lock().unwrap();
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
                    draw_progress_slider(ctx, ui, app, total_ticks, current_tick);
                });

                ui.add_space(2.0);
                draw_render_progress(ui, app);
            });
        });
}

fn draw_progress_slider(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    app: &mut MidiApp,
    total_ticks: u32,
    current_tick: u32,
) {
    let (resp, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), 6.0), egui::Sense::drag());
    let slider_rect = resp.rect;
    painter.rect_filled(slider_rect, 2.0, Color32::from_rgb(50, 50, 50));

    if total_ticks > 0 {
        let progress = current_tick as f32 / total_ticks as f32;
        let cursor_x = slider_rect.min.x + progress * slider_rect.width();
        let fill_rect = Rect::from_min_max(
            egui::pos2(slider_rect.min.x, slider_rect.min.y),
            egui::pos2(cursor_x, slider_rect.max.y),
        );
        painter.rect_filled(fill_rect, 2.0, Color32::from_rgb(100, 180, 255));
        painter.circle_filled(egui::pos2(cursor_x, slider_rect.center().y), 8.0, Color32::WHITE);

        if resp.drag_started() || resp.dragged() {
            if let Some(mouse_pos) = ctx.pointer_latest_pos() {
                let rel_x = mouse_pos.x - slider_rect.min.x;
                let clamped_x = rel_x.clamp(0.0, slider_rect.width());
                let new_progress = clamped_x / slider_rect.width();
                let mut state = app.state.lock().unwrap();
                state.current_tick = (new_progress * total_ticks as f32) as u32;
                state.is_playing = false;
            }
        }
    }
}

fn draw_render_progress(ui: &mut egui::Ui, app: &mut MidiApp) {
    ui.horizontal(|ui| {
        let state = app.state.lock().unwrap();
        if state.is_rendering {
            ui.label("渲染中:");
            let (resp, painter) = ui.allocate_painter(
                egui::vec2(ui.available_width(), 10.0),
                egui::Sense::hover(),
            );
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
}