use eframe::egui;
use egui::{Color32, FontId, Rect, Stroke, Sense};
use crate::state::{InfoField, InfoOverlaySettings, VisualSettings, ALL_INFO_FIELDS};

pub fn draw_info_overlay(
    painter: &egui::Painter,
    ui: &mut egui::Ui,
    rect: &Rect,
    settings: &mut InfoOverlaySettings,
    visual_settings: &VisualSettings,
    track_name: &str,
    author: &str,
    note_count_played: usize,
    note_count_total: usize,
    time_sig: &str,
    bpm: f32,
    current_time_str: &str,
    progress: f32,
    active_note_names: &[String],
    bar_beat_str: &str,
    chord: &Option<String>,
) {
    if settings.enabled_fields.is_empty() {
        return;
    }

    while settings.group_positions.len() < settings.enabled_fields.len() {
        let default_y = 0.02 + settings.group_positions.len() as f32 * 0.1;
        settings.group_positions.push((0.02, default_y));
    }

    let groups: Vec<(InfoField, String, f32)> = settings.enabled_fields
        .iter()
        .enumerate()
        .filter_map(|(_group_idx, field)| {
            let text = match field {
                InfoField::TrackName => Some(format!("曲目: {}", track_name)),
                InfoField::Author => Some(format!("作者: {}", author)),
                InfoField::NoteCount => Some(format!("已播: {} / 总: {}", note_count_played, note_count_total)),
                InfoField::TimeSig => Some(format!("拍号: {}", time_sig)),
                InfoField::Bpm => Some(format!("BPM: {:.1}", bpm)),
                InfoField::CurrentTime => Some(format!("时间: {}", current_time_str)),
                InfoField::ProgressPercent => Some(format!("进度: {:.1}%", progress)),
                InfoField::ActiveNotes => {
                    if active_note_names.is_empty() {
                        Some("当前音符: 无".to_string())
                    } else {
                        Some(format!("当前音符: {}", active_note_names.join(", ")))
                    }
                }
                InfoField::BarBeat => Some(format!("小节: {}", bar_beat_str)),
                InfoField::Chord => {
                    if let Some(c) = chord {
                        Some(format!("和弦: {}", c))
                    } else {
                        Some("和弦: (无)".to_string())
                    }
                }
            };
            text.map(|t| {
                let idx = ALL_INFO_FIELDS.iter().position(|f| *f == *field).unwrap_or(0);
                let scale = if idx < settings.field_scales.len() {
                    settings.field_scales[idx]
                } else {
                    1.0
                };
                (*field, t, scale)
            })
        })
        .collect::<Vec<_>>();

    if groups.is_empty() {
        return;
    }

    let base_font_size = 14.0;
    let base_padding = 8.0;
    let base_line_height = 20.0;

    for (idx, (_field, text, scale)) in groups.iter().enumerate() {
        let font_size = base_font_size * scale;
        let padding = base_padding * scale;
        let line_height = base_line_height * scale;

        let galley = painter.layout(
            text.to_string(),
            FontId::proportional(font_size),
            Color32::WHITE,
            f32::INFINITY,
        );
        let text_width = galley.size().x;
        let box_width = text_width + padding * 2.0;
        let box_height = line_height + padding * 2.0;

        let (pos_x_ratio, pos_y_ratio) = settings.group_positions[idx];
        let pos_x = rect.min.x + (rect.width() - box_width) * pos_x_ratio;
        let pos_y = rect.min.y + (rect.height() - box_height) * pos_y_ratio;

        // 无旋转，直接使用矩形
        let box_rect = Rect::from_min_max(
            egui::pos2(pos_x, pos_y),
            egui::pos2(pos_x + box_width, pos_y + box_height),
        );

        // 背景
        let bg_color = Color32::from_rgb(
            visual_settings.piano_bg_r,
            visual_settings.piano_bg_g,
            visual_settings.piano_bg_b,
        );
        let bg_color = bg_color.linear_multiply(settings.background_opacity);
        painter.rect_filled(box_rect, 4.0, bg_color);

        let text_color = Color32::from_rgb(
            settings.text_color_r,
            settings.text_color_g,
            settings.text_color_b,
        );

        let text_pos = egui::pos2(pos_x + padding, pos_y + padding + line_height / 2.0);
        painter.text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            text.clone(),
            FontId::proportional(font_size),
            text_color,
        );

        // 边框（如果启用）
        if settings.show_border {
            painter.rect_stroke(box_rect, 4.0, Stroke::new(1.5_f32, Color32::from_rgb(100, 200, 255)));
        }

        // 拖拽交互（无旋转）
        if let Some(mouse) = ui.ctx().pointer_latest_pos() {
            if box_rect.contains(mouse) {
                let response = ui.interact(box_rect, egui::Id::new(format!("info_group_{}", idx)), Sense::drag());
                if response.dragged() {
                    let delta = response.drag_delta();
                    let new_x = settings.group_positions[idx].0 + delta.x / rect.width();
                    let new_y = settings.group_positions[idx].1 + delta.y / rect.height();
                    settings.group_positions[idx] = (new_x, new_y);
                    ui.ctx().request_repaint();
                }
            }
        }
    }
}