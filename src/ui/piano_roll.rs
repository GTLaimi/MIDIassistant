// src/ui/piano_roll.rs
use eframe::egui;
use egui::{Color32, FontId, Rect, Stroke};
use crate::models;
use crate::state::{VisualSettings, LayoutOrientation};

pub fn draw_piano_roll(
    ctx: &egui::Context,
    settings: &mut VisualSettings,
    notes: &[models::note::Note],
    total_ticks: u32,
    current_tick: u32,
    is_playing: bool,
    time_sig: &str,
    ppq: u32,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter().with_clip_rect(rect);

        let piano_bg = Color32::from_rgb(settings.piano_bg_r, settings.piano_bg_g, settings.piano_bg_b);
        painter.rect_filled(rect, 0.0, piano_bg);

        let layout = settings.layout_orientation;

        if layout == LayoutOrientation::Horizontal {
            draw_horizontal_layout(
                ui,
                &painter,
                &rect,
                settings,
                notes,
                total_ticks,
                current_tick,
                is_playing,
                time_sig,
                ppq,
            );
        } else {
            draw_vertical_layout(
                ui,
                &painter,
                &rect,
                settings,
                notes,
                total_ticks,
                current_tick,
                is_playing,
                time_sig,
                ppq,
            );
        }
    });
}

// ============================================================
// 水平布局（键盘在左，时间轴水平）
// ============================================================
fn draw_horizontal_layout(
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
) {
    let max_kbd_w = rect.width() * 0.4_f32;
    let mut kbd_w = settings.horizontal_keyboard_width.clamp(0.0_f32, max_kbd_w);
    // 宽度小于 5px 时强制为 0，消除白线
    if kbd_w < 5.0_f32 {
        kbd_w = 0.0_f32;
    }

    let kbd_h = rect.height();
    let pnl_x = rect.min.x + kbd_w;
    let pnl_w = rect.width() - kbd_w;

    let pitch_min = settings.pitch_min as i32;
    let pitch_range = settings.pitch_range as i32;
    let pitch_h = kbd_h / pitch_range as f32;

    let mut time_offset: f32 = 0.0;
    if settings.follow_playhead && total_ticks > 0 {
        time_offset = current_tick as f32 / total_ticks as f32;
    }

    let visible_ticks = total_ticks as f32 * settings.time_zoom;
    let view_offset_ticks = time_offset * total_ticks as f32;

    let bright_color = Color32::from_rgb(settings.grid_bright_r, settings.grid_bright_g, settings.grid_bright_b);
    let dim_color = Color32::from_rgb(settings.grid_dim_r, settings.grid_dim_g, settings.grid_dim_b);

    // 网格线（垂直）
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

    // 网格线（水平）
    for i in 0..=pitch_range {
        let y = rect.min.y + ((pitch_range - i) as f32 * pitch_h);
        let actual_pitch = pitch_min + i;
        let color = if actual_pitch % 12 == 0 { bright_color } else { dim_color };
        painter.line_segment(
            [egui::pos2(pnl_x, y), egui::pos2(rect.max.x, y)],
            Stroke::new(1.0_f32, color),
        );
    }

    // 音符
    for note in notes {
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
        let note_col = if is_active {
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

    // 光标
    if settings.show_cursor && total_ticks > 0 {
        let cursor_color = Color32::from_rgb(settings.cursor_r, settings.cursor_g, settings.cursor_b);
        let cursor_x = pnl_x + ((current_tick as f32 - view_offset_ticks) / visible_ticks) * pnl_w;
        painter.line_segment(
            [egui::pos2(cursor_x, rect.min.y), egui::pos2(cursor_x, rect.min.y + kbd_h)],
            Stroke::new(2.0_f32, cursor_color),
        );
    }

    // ---------- 键盘（仅在宽度 > 0 时绘制） ----------
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

        // 音名
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

    // ---------- 分隔条（仅当键盘可见时显示） ----------
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

// ============================================================
// 垂直布局（键盘在底，瀑布流）
// ============================================================
fn draw_vertical_layout(
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
) {
    let bottom_padding = 10.0_f32;
    let max_kbd_h = rect.height() * 0.5_f32;
    let kbd_h = settings.vertical_keyboard_height.clamp(0.0_f32, max_kbd_h);

    let kbd_w = rect.width();
    let pnl_y = rect.min.y;
    let pnl_h = rect.height() - kbd_h - bottom_padding;

    let pitch_min = settings.pitch_min as i32;
    let pitch_range = settings.pitch_range as i32;
    let pitch_w = kbd_w / pitch_range as f32;

    let mut time_offset: f32 = 0.0;
    if settings.follow_playhead && total_ticks > 0 {
        time_offset = current_tick as f32 / total_ticks as f32;
    }

    let visible_ticks = total_ticks as f32 * settings.time_zoom;
    let view_offset_ticks = time_offset * total_ticks as f32;

    let bright_color = Color32::from_rgb(settings.grid_bright_r, settings.grid_bright_g, settings.grid_bright_b);
    let dim_color = Color32::from_rgb(settings.grid_dim_r, settings.grid_dim_g, settings.grid_dim_b);

    // 网格线（水平）
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
        let y = rect.bottom() - kbd_h - bottom_padding - ((tick as f32 - view_offset_ticks) / visible_ticks) * pnl_h;
        if y >= pnl_y && y <= pnl_y + pnl_h {
            let color = if tick % ticks_per_bar == 0 { bright_color } else { dim_color };
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                Stroke::new(1.0_f32, color),
            );
        }
        if ticks_per_beat == 0 { break; }
        tick += ticks_per_beat;
    }

    // 网格线（垂直）
    for i in 0..=pitch_range {
        let x = rect.min.x + (i as f32 / pitch_range as f32) * kbd_w;
        let actual_pitch = pitch_min + i;
        let color = if actual_pitch % 12 == 0 { bright_color } else { dim_color };
        painter.line_segment(
            [egui::pos2(x, pnl_y), egui::pos2(x, pnl_y + pnl_h)],
            Stroke::new(1.0_f32, color),
        );
    }

    // 音符
    for note in notes {
        let note_pitch = note.pitch as i32;
        if note_pitch < pitch_min || note_pitch > pitch_min + pitch_range { continue; }

        let rel_pitch = note_pitch - pitch_min;
        let x_start = rect.min.x + (rel_pitch as f32 / pitch_range as f32) * kbd_w;
        let x_end = rect.min.x + ((rel_pitch + 1) as f32 / pitch_range as f32) * kbd_w;

        let y_start = rect.bottom() - kbd_h - bottom_padding - ((note.start_tick as f32 - view_offset_ticks) / visible_ticks) * pnl_h;
        let y_end = rect.bottom() - kbd_h - bottom_padding - (((note.start_tick + note.duration) as f32 - view_offset_ticks) / visible_ticks) * pnl_h;

        if y_start < pnl_y - 10.0_f32 || y_end > pnl_y + pnl_h + 10.0_f32 { continue; }

        let nw = (x_end - x_start).max(1.5_f32);
        let is_active = is_playing && current_tick >= note.start_tick && current_tick <= note.start_tick + note.duration;
        let note_col = if is_active {
            Color32::from_rgb(settings.active_note_r, settings.active_note_g, settings.active_note_b)
        } else {
            Color32::from_rgb(settings.note_r, settings.note_g, settings.note_b)
        };

        painter.rect_filled(
            Rect::from_min_max(
                egui::pos2(x_start, y_end),
                egui::pos2(x_start + nw, y_start),
            ),
            2.0_f32,
            note_col,
        );
    }

    // 光标
    if settings.show_cursor && total_ticks > 0 {
        let cursor_color = Color32::from_rgb(settings.cursor_r, settings.cursor_g, settings.cursor_b);
        let cursor_y = rect.bottom() - kbd_h - bottom_padding - ((current_tick as f32 - view_offset_ticks) / visible_ticks) * pnl_h;
        painter.line_segment(
            [egui::pos2(rect.min.x, cursor_y), egui::pos2(rect.max.x, cursor_y)],
            Stroke::new(2.0_f32, cursor_color),
        );
    }

    // ---------- 键盘 ----------
    let white_key_col = Color32::from_rgb(settings.white_key_r, settings.white_key_g, settings.white_key_b);
    let black_key_col = Color32::from_rgb(settings.black_key_r, settings.black_key_g, settings.black_key_b);
    let text_col = Color32::from_rgb(settings.text_r, settings.text_g, settings.text_b);

    let keyboard_rect = Rect::from_min_max(
        egui::pos2(rect.min.x, rect.bottom() - kbd_h - bottom_padding),
        egui::pos2(rect.max.x, rect.bottom() - bottom_padding),
    );

    // 白键
    for i in 0..=pitch_range {
        let actual_pitch = pitch_min + i;
        let mod12 = actual_pitch % 12;
        if mod12 == 0 || mod12 == 2 || mod12 == 4 || mod12 == 5 || mod12 == 7 || mod12 == 9 || mod12 == 11 {
            let x = rect.min.x + (i as f32 / pitch_range as f32) * kbd_w;
            let white_rect = Rect::from_min_max(
                egui::pos2(x, keyboard_rect.min.y),
                egui::pos2(x + pitch_w, keyboard_rect.max.y),
            );
            painter.rect_filled(white_rect, 0.0, white_key_col);
            painter.rect_stroke(white_rect, 0.0, Stroke::new(0.5_f32, Color32::from_rgb(180, 180, 180)));
        }
    }

    // 黑键（高度与白键一致，应用偏移和缩放）
    let black_w = pitch_w * settings.vertical_black_key_width_scale;
    let _black_h = kbd_h;  // 与白键一样高
    for i in 0..=pitch_range {
        let actual_pitch = pitch_min + i;
        let mod12 = actual_pitch % 12;
        if mod12 == 1 || mod12 == 3 || mod12 == 6 || mod12 == 8 || mod12 == 10 {
            let x = rect.min.x + (i as f32 / pitch_range as f32) * kbd_w
                - black_w / 2.0_f32
                + settings.vertical_black_key_offset * pitch_w;
            let black_rect = Rect::from_min_max(
                egui::pos2(x, keyboard_rect.min.y),
                egui::pos2(x + black_w, keyboard_rect.max.y),
            );
            painter.rect_filled(black_rect, 0.0, black_key_col);
            painter.rect_stroke(black_rect, 0.0, Stroke::new(0.5_f32, Color32::BLACK));
        }
    }

    // 音名
    let font_size = (pitch_w * 0.2_f32).clamp(6.0_f32, 14.0_f32);
    for i in 0..=pitch_range {
        let actual_pitch = pitch_min + i;
        if actual_pitch % 12 == 0 && font_size >= 6.0_f32 {
            let display_pitch = actual_pitch + 12;
            let note_name = models::note::Note::new(display_pitch as u8, 0, 0, 0).pitch_name;
            let x = rect.min.x + ((i as f32 + 0.5_f32) / pitch_range as f32) * kbd_w;
            painter.text(
                egui::pos2(x, keyboard_rect.min.y + 5.0_f32),
                egui::Align2::CENTER_TOP,
                note_name,
                FontId::proportional(font_size),
                text_col,
            );
        }
    }

    // ---------- 分隔条（可拖动） ----------
    let sep_y = rect.bottom() - kbd_h - bottom_padding;
    let sep_rect = Rect::from_min_max(
        egui::pos2(rect.min.x, sep_y - 3.0_f32),
        egui::pos2(rect.max.x, sep_y + 3.0_f32),
    );
    let sep_response = ui.allocate_rect(sep_rect, egui::Sense::drag());
    painter.rect_filled(sep_rect, 0.0, Color32::from_rgba_premultiplied(200, 200, 200, 120));

    if sep_response.dragged() {
        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            let new_height = rect.bottom() - bottom_padding - pos.y;
            settings.vertical_keyboard_height = new_height.clamp(0.0_f32, max_kbd_h);
            ui.ctx().request_repaint();
        }
    }
}