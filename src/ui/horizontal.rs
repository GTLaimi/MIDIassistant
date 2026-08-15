use eframe::egui;
use egui::{Color32, FontId, Rect, Stroke};
use std::collections::HashSet;
use crate::models;
use crate::state::VisualSettings;

pub fn draw_horizontal_layout(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    rect: &Rect,
    settings: &mut VisualSettings,
    notes: &[models::note::Note],
    total_ticks: u32,
    current_tick: u32,
    is_playing: bool,
    time_sig: &str,
    ppq: u32,
    played_indices: &HashSet<usize>,
) {
    let max_kbd_w = rect.width() * 0.4_f32;
    let mut kbd_w = settings.horizontal_keyboard_width.clamp(0.0_f32, max_kbd_w);
    if kbd_w < 5.0_f32 {
        kbd_w = 0.0_f32;
    }

    let kbd_h = rect.height();
    let pnl_x = rect.min.x + kbd_w;
    let pnl_w = rect.width() - kbd_w;

    let pitch_min = settings.pitch_min as i32;
    let pitch_range = settings.pitch_range as i32;
    let pitch_h = kbd_h / pitch_range as f32;

    let visible_ticks = total_ticks as f32 * settings.time_zoom;

    let view_offset_ticks = match settings.follow_mode {
        crate::state::FollowMode::Edge => current_tick as f32,
        crate::state::FollowMode::Center => {
            let base = current_tick as f32 - visible_ticks * 0.5;
            base + settings.manual_offset_ratio * visible_ticks
        }
        crate::state::FollowMode::Right => {
            let base = current_tick as f32 - visible_ticks * 0.75;
            base + settings.manual_offset_ratio * visible_ticks
        }
    };
    let view_offset_ticks = view_offset_ticks.clamp(0.0, (total_ticks as f32 - visible_ticks).max(0.0));

    let bright_color = Color32::from_rgb(settings.grid_bright_r, settings.grid_bright_g, settings.grid_bright_b);
    let dim_color = Color32::from_rgb(settings.grid_dim_r, settings.grid_dim_g, settings.grid_dim_b);

    let mut beats_per_bar = 4;
    if let Some(slash) = time_sig.find('/') {
        if let Ok(num) = time_sig[..slash].parse::<u32>() {
            beats_per_bar = num;
        }
    }
    let ticks_per_beat = ppq;
    let ticks_per_bar = ticks_per_beat * beats_per_bar;

    let mut tick = 0;
    while tick <= total_ticks {
        let x = pnl_x + ((tick as f32 - view_offset_ticks) / visible_ticks) * pnl_w;
        if x >= pnl_x && x <= pnl_x + pnl_w {
            let color = if tick % ticks_per_bar == 0 { bright_color } else { dim_color };
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.min.y + kbd_h)],
                Stroke::new(1.0_f32, color),
            );
        }
        if ticks_per_beat == 0 { break; }
        tick += ticks_per_beat;
    }

    for i in 0..=pitch_range {
        let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
        let actual_pitch = pitch_min + i;
        let color = if actual_pitch % 12 == 0 { bright_color } else { dim_color };
        painter.line_segment(
            [egui::pos2(pnl_x, y), egui::pos2(rect.max.x, y)],
            Stroke::new(1.0_f32, color),
        );
    }

    for (idx, note) in notes.iter().enumerate() {
        let note_pitch = note.pitch as i32;
        if note_pitch < pitch_min || note_pitch > pitch_min + pitch_range { continue; }

        let x_start = pnl_x + ((note.start_tick as f32 - view_offset_ticks) / visible_ticks) * pnl_w;
        let x_end = pnl_x + (((note.start_tick + note.duration) as f32 - view_offset_ticks) / visible_ticks) * pnl_w;
        if x_end < pnl_x || x_start > pnl_x + pnl_w { continue; }

        let rel_pitch = note_pitch - pitch_min;
        let y_center = rect.min.y + ((pitch_range - rel_pitch) as f32 * pitch_h);

        let nw = (x_end - x_start).max(1.5_f32);
        let nh = pitch_h * 0.8_f32;
        let is_active = is_playing && current_tick >= note.start_tick && current_tick <= note.start_tick + note.duration;
        let is_highlighted = settings.sustain_highlight && played_indices.contains(&idx);
        let note_col = if is_active || is_highlighted {
            Color32::from_rgb(settings.active_note_r, settings.active_note_g, settings.active_note_b)
        } else {
            Color32::from_rgb(settings.note_r, settings.note_g, settings.note_b)
        };

        painter.rect_filled(
            Rect::from_min_size(egui::pos2(x_start, y_center - nh / 2.0_f32), egui::vec2(nw, nh)),
            2.0_f32,
            note_col,
        );
    }

    let show_cursor = settings.show_cursor && settings.follow_mode != crate::state::FollowMode::Edge;
    if show_cursor && total_ticks > 0 {
        let cursor_color = Color32::from_rgb(settings.cursor_r, settings.cursor_g, settings.cursor_b);
        let cursor_x = pnl_x + ((current_tick as f32 - view_offset_ticks) / visible_ticks) * pnl_w;
        painter.line_segment(
            [egui::pos2(cursor_x, rect.min.y), egui::pos2(cursor_x, rect.min.y + kbd_h)],
            Stroke::new(2.0_f32, cursor_color),
        );
    }

    if kbd_w > 0.0_f32 {
        let white_key_col = Color32::from_rgb(settings.white_key_r, settings.white_key_g, settings.white_key_b);
        let black_key_col = Color32::from_rgb(settings.black_key_r, settings.black_key_g, settings.black_key_b);
        let text_col = Color32::from_rgb(settings.text_r, settings.text_g, settings.text_b);

        let black_w = kbd_w;
        let black_h = pitch_h * 0.88_f32 + 1.0_f32;

        for i in 0..=pitch_range {
            let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
            let actual_pitch = pitch_min + i;
            let mod12 = actual_pitch % 12;
            if mod12 == 0 || mod12 == 2 || mod12 == 4 || mod12 == 5 || mod12 == 7 || mod12 == 9 || mod12 == 11 {
                let white_rect = Rect::from_min_max(
                    egui::pos2(rect.min.x, y_center - pitch_h / 2.0_f32),
                    egui::pos2(rect.min.x + kbd_w, y_center + pitch_h / 2.0_f32),
                );
                painter.rect_filled(white_rect, 0.0, white_key_col);
                painter.rect_stroke(white_rect, 0.0, Stroke::new(0.5_f32, Color32::from_rgb(180, 180, 180)));
            }
        }

        for i in 0..=pitch_range {
            let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
            let actual_pitch = pitch_min + i;
            let mod12 = actual_pitch % 12;
            if mod12 == 1 || mod12 == 3 || mod12 == 6 || mod12 == 8 || mod12 == 10 {
                let black_rect = Rect::from_min_max(
                    egui::pos2(rect.min.x + kbd_w - black_w, y_center - black_h / 2.0_f32),
                    egui::pos2(rect.min.x + kbd_w, y_center + black_h / 2.0_f32),
                );
                painter.rect_filled(black_rect, 0.0, black_key_col);
                painter.rect_stroke(black_rect, 0.0, Stroke::new(0.5_f32, Color32::BLACK));
            }
        }

        let font_size = (pitch_h * 0.25_f32).clamp(6.0_f32, 18.0_f32);
        for i in 0..=pitch_range {
            let y_center = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
            let actual_pitch = pitch_min + i;
            if actual_pitch % 12 == 0 && font_size >= 6.0_f32 {
                let display_pitch = actual_pitch + 12;
                let note_name = models::note::Note::new(display_pitch as u8, 0, 0, 0).pitch_name;
                painter.text(
                    egui::pos2(rect.min.x + 5.0_f32, y_center),
                    egui::Align2::LEFT_CENTER,
                    note_name,
                    FontId::proportional(font_size),
                    text_col,
                );
            }
        }
    }

    if kbd_w > 0.0_f32 {
        let sep_x = rect.min.x + kbd_w;
        let sep_rect = Rect::from_min_max(
            egui::pos2(sep_x - 3.0_f32, rect.min.y),
            egui::pos2(sep_x + 3.0_f32, rect.max.y),
        );
        let sep_response = ui.allocate_rect(sep_rect, egui::Sense::drag());
        painter.rect_filled(sep_rect, 0.0, Color32::from_rgba_premultiplied(200, 200, 200, 120));

        if sep_response.dragged() {
            if let Some(pos) = ui.ctx().pointer_latest_pos() {
                let new_width = pos.x - rect.min.x;
                settings.horizontal_keyboard_width = new_width.clamp(0.0_f32, max_kbd_w);
                ui.ctx().request_repaint();
            }
        }
    }
}