use eframe::egui;
use egui::{Color32, Rect, Stroke, Sense};
use std::collections::HashSet;
use crate::models;
use crate::state::VisualSettings;

pub fn draw_bar_layout(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    rect: &Rect,
    settings: &mut VisualSettings,
    notes: &[models::note::Note],
    _total_ticks: u32,
    current_tick: u32,
    is_playing: bool,
    time_sig: &str,
    ppq: u32,
    _played_indices: &HashSet<usize>, // 不再使用，保留参数以兼容调用，但逻辑中忽略
) {
    let show_frame = settings.bar_view.visible;

    let mut beats_per_bar = 4;
    if let Some(slash) = time_sig.find('/') {
        if let Ok(num) = time_sig[..slash].parse::<u32>() {
            beats_per_bar = num;
        }
    }
    let ticks_per_beat = ppq;
    let ticks_per_bar = ticks_per_beat * beats_per_bar;
    if ticks_per_bar == 0 {
        return;
    }

    let current_bar = current_tick / ticks_per_bar;
    let bar_start = current_bar * ticks_per_bar;
    let bar_end = (current_bar + 1) * ticks_per_bar;

    let box_width = rect.width() * settings.bar_view.width_ratio;
    let box_height = rect.height() * settings.bar_view.height_ratio;
    let pos_x = rect.min.x + (rect.width() - box_width) * settings.bar_view.pos_x;
    let pos_y = rect.min.y + (rect.height() - box_height) * settings.bar_view.pos_y;
    let center = egui::pos2(pos_x + box_width / 2.0, pos_y + box_height / 2.0);
    let angle = settings.bar_view.rotation;

    let rotate_point = |p: egui::Pos2| -> egui::Pos2 {
        let dx = p.x - center.x;
        let dy = p.y - center.y;
        let cos = angle.cos();
        let sin = angle.sin();
        egui::pos2(center.x + dx * cos - dy * sin, center.y + dx * sin + dy * cos)
    };

    let corners = [
        rotate_point(egui::pos2(pos_x, pos_y)),
        rotate_point(egui::pos2(pos_x + box_width, pos_y)),
        rotate_point(egui::pos2(pos_x + box_width, pos_y + box_height)),
        rotate_point(egui::pos2(pos_x, pos_y + box_height)),
    ];
    let box_rect = Rect::from_points(&corners);

    if show_frame {
        let bg_color = Color32::from_rgb(
            settings.piano_bg_r,
            settings.piano_bg_g,
            settings.piano_bg_b,
        );
        painter.rect_filled(box_rect, 4.0, bg_color);
        painter.rect_stroke(box_rect, 4.0, Stroke::new(2.0_f32, Color32::from_rgb(160, 160, 160)));

        // ---------- 键盘（旋转+半透明，低音在下）----------
        let kbd_width = 30.0;
        let kbd_rect = Rect::from_min_max(
            egui::pos2(pos_x - kbd_width - 5.0, pos_y),
            egui::pos2(pos_x - 5.0, pos_y + box_height),
        );
        draw_keyboard(painter, kbd_rect, settings, rotate_point);
    }

    // 音高范围
    let pitch_min = settings.pitch_min + settings.bar_view.display_pitch_offset;
    let pitch_range = settings.bar_view.display_pitch_range;
    if pitch_range <= 0 { return; }

    let bright_color = Color32::from_rgb(settings.grid_bright_r, settings.grid_bright_g, settings.grid_bright_b);
    let dim_color = Color32::from_rgb(settings.grid_dim_r, settings.grid_dim_g, settings.grid_dim_b);

    // 水平音高线（低音在下，高音在上，反转 y）
    for i in 0..=pitch_range {
        let rel_y = 1.0 - (i as f32 / pitch_range as f32); // 反转：0 → 底部（低音），1 → 顶部（高音）
        let y = pos_y + rel_y * box_height;
        let p1 = rotate_point(egui::pos2(pos_x, y));
        let p2 = rotate_point(egui::pos2(pos_x + box_width, y));
        let actual_pitch = pitch_min + i;
        let color = if actual_pitch % 12 == 0 { bright_color } else { dim_color };
        painter.line_segment([p1, p2], Stroke::new(0.8_f32, color));
    }

    // 垂直节拍线
    let mut tick = bar_start;
    while tick <= bar_end {
        let rel_x = (tick - bar_start) as f32 / ticks_per_bar as f32;
        let x = pos_x + rel_x * box_width;
        let p1 = rotate_point(egui::pos2(x, pos_y));
        let p2 = rotate_point(egui::pos2(x, pos_y + box_height));
        let is_bar_line = (tick - bar_start) % ticks_per_bar == 0;
        let color = if is_bar_line { bright_color } else { dim_color };
        let stroke_width = if is_bar_line { 1.5_f32 } else { 0.8_f32 };
        painter.line_segment([p1, p2], Stroke::new(stroke_width, color));
        tick += ticks_per_beat;
        if tick > bar_end { break; }
    }

    // 音符
    let bar_notes: Vec<&models::note::Note> = notes
        .iter()
        .filter(|n| n.start_tick < bar_end && n.start_tick + n.duration > bar_start)
        .collect();

    for (_idx, &note) in bar_notes.iter().enumerate() {
        let note_pitch = note.pitch as i32;
        if note_pitch < pitch_min || note_pitch > pitch_min + pitch_range { continue; }

        let start_tick_rel = note.start_tick.max(bar_start) - bar_start;
        let end_tick_rel = (note.start_tick + note.duration).min(bar_end) - bar_start;
        let x_start = pos_x + (start_tick_rel as f32 / ticks_per_bar as f32) * box_width;
        let x_end = pos_x + (end_tick_rel as f32 / ticks_per_bar as f32) * box_width;
        if x_end <= x_start { continue; }

        // 反转音高：低音在下
        let rel_pitch = 1.0 - (note_pitch - pitch_min) as f32 / pitch_range as f32;
        let y_center = pos_y + rel_pitch * box_height;
        let note_height = box_height / pitch_range as f32 * 0.8;

        let is_active = is_playing && current_tick >= note.start_tick && current_tick <= note.start_tick + note.duration;
        let is_highlighted = settings.sustain_highlight && note.start_tick <= current_tick;
        let color = if is_active || is_highlighted {
            Color32::from_rgb(settings.active_note_r, settings.active_note_g, settings.active_note_b)
        } else {
            Color32::from_rgb(settings.note_r, settings.note_g, settings.note_b)
        };

        let rect_points = [
            rotate_point(egui::pos2(x_start, y_center - note_height / 2.0)),
            rotate_point(egui::pos2(x_end, y_center - note_height / 2.0)),
            rotate_point(egui::pos2(x_end, y_center + note_height / 2.0)),
            rotate_point(egui::pos2(x_start, y_center + note_height / 2.0)),
        ];
        painter.add(egui::Shape::convex_polygon(rect_points.to_vec(), color, Stroke::NONE));
    }

    // 光标（受 show_cursor 控制）
    if settings.show_cursor {
        let cursor_color = Color32::from_rgb(settings.cursor_r, settings.cursor_g, settings.cursor_b);
        let cursor_rel_x = (current_tick - bar_start) as f32 / ticks_per_bar as f32;
        let cursor_x = pos_x + cursor_rel_x * box_width;
        let p1 = rotate_point(egui::pos2(cursor_x, pos_y));
        let p2 = rotate_point(egui::pos2(cursor_x, pos_y + box_height));
        painter.line_segment([p1, p2], Stroke::new(2.0_f32, cursor_color));
    }

    // ----- 优化拖拽交互（像素级精确跟随，移出框仍继续） -----
    // 使用轴对齐包围盒作为交互区域（点击检测），但拖拽时按像素位移更新位置
    let response = ui.interact(box_rect, egui::Id::new("bar_view_move"), Sense::drag());
    if response.dragged() {
        let delta = response.drag_delta();
        // 当前左上角像素位置（未旋转前的原始位置）
        let current_px_x = rect.min.x + (rect.width() - box_width) * settings.bar_view.pos_x;
        let current_px_y = rect.min.y + (rect.height() - box_height) * settings.bar_view.pos_y;
        let new_px_x = current_px_x + delta.x;
        let new_px_y = current_px_y + delta.y;
        // 转回归一化并限制在 [0,1]
        let new_ratio_x = ((new_px_x - rect.min.x) / (rect.width() - box_width)).clamp(0.0, 1.0);
        let new_ratio_y = ((new_px_y - rect.min.y) / (rect.height() - box_height)).clamp(0.0, 1.0);
        settings.bar_view.pos_x = new_ratio_x;
        settings.bar_view.pos_y = new_ratio_y;
        ui.ctx().request_repaint();
    }
}

/// 绘制键盘（完全照搬横式，应用旋转，半透明，黑键与白键同宽，低音在下）
fn draw_keyboard<F>(painter: &egui::Painter, rect: Rect, settings: &VisualSettings, rotate_point: F)
where
    F: Fn(egui::Pos2) -> egui::Pos2,
{
    // ... 与之前完全相同，未改动 ...
    let white_key_col = Color32::from_rgb(settings.white_key_r, settings.white_key_g, settings.white_key_b);
    let black_key_col = Color32::from_rgb(settings.black_key_r, settings.black_key_g, settings.black_key_b);
    let text_col = Color32::from_rgb(settings.text_r, settings.text_g, settings.text_b);

    let pitch_min = settings.pitch_min;
    let pitch_range = settings.pitch_range;
    let key_height = rect.height() / pitch_range as f32;
    let opacity = 0.35;

    // 白键
    for i in 0..=pitch_range {
        let rel_y = 1.0 - (i as f32 / pitch_range as f32);
        let y = rect.min.y + rel_y * rect.height();
        let actual_pitch = pitch_min + i;
        let mod12 = actual_pitch % 12;
        let is_white = mod12 == 0 || mod12 == 2 || mod12 == 4 || mod12 == 5 || mod12 == 7 || mod12 == 9 || mod12 == 11;
        if is_white {
            let key_rect = Rect::from_min_max(
                egui::pos2(rect.min.x, y),
                egui::pos2(rect.max.x, y + key_height),
            );
            let corners = [
                rotate_point(key_rect.min),
                rotate_point(egui::pos2(key_rect.max.x, key_rect.min.y)),
                rotate_point(key_rect.max),
                rotate_point(egui::pos2(key_rect.min.x, key_rect.max.y)),
            ];
            let rotated_rect = Rect::from_points(&corners);
            let color = white_key_col.linear_multiply(opacity);
            painter.rect_filled(rotated_rect, 0.0, color);
            painter.rect_stroke(rotated_rect, 0.0, Stroke::new(0.5_f32, Color32::from_rgb(180, 180, 180).linear_multiply(opacity)));
        }
    }

    // 黑键
    for i in 0..=pitch_range {
        let rel_y = 1.0 - (i as f32 / pitch_range as f32);
        let y = rect.min.y + rel_y * rect.height();
        let actual_pitch = pitch_min + i;
        let mod12 = actual_pitch % 12;
        let is_black = mod12 == 1 || mod12 == 3 || mod12 == 6 || mod12 == 8 || mod12 == 10;
        if is_black {
            let key_rect = Rect::from_min_max(
                egui::pos2(rect.min.x, y),
                egui::pos2(rect.max.x, y + key_height),
            );
            let corners = [
                rotate_point(key_rect.min),
                rotate_point(egui::pos2(key_rect.max.x, key_rect.min.y)),
                rotate_point(key_rect.max),
                rotate_point(egui::pos2(key_rect.min.x, key_rect.max.y)),
            ];
            let rotated_rect = Rect::from_points(&corners);
            let color = black_key_col.linear_multiply(opacity);
            painter.rect_filled(rotated_rect, 0.0, color);
            painter.rect_stroke(rotated_rect, 0.0, Stroke::new(0.5_f32, Color32::BLACK.linear_multiply(opacity)));
        }
    }

    // 音名（C 处）
    let font_size = (key_height * 0.25).clamp(6.0, 14.0);
    for i in 0..=pitch_range {
        let actual_pitch = pitch_min + i;
        if actual_pitch % 12 == 0 && font_size >= 6.0 {
            let rel_y = 1.0 - (i as f32 / pitch_range as f32);
            let y = rect.min.y + rel_y * rect.height() + key_height / 2.0;
            let display_pitch = actual_pitch + 12;
            let note_name = models::note::Note::new(display_pitch as u8, 0, 0, 0).pitch_name;
            let text_pos = rotate_point(egui::pos2(rect.min.x + 5.0, y));
            painter.text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                note_name,
                egui::FontId::proportional(font_size),
                text_col.linear_multiply(opacity),
            );
        }
    }
}