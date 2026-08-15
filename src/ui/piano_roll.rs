use eframe::egui;
use egui::{Color32, FontId, Rect};
use std::collections::HashSet;
use crate::models;
use crate::state::{VisualSettings, LayoutOrientation};

use super::horizontal::draw_horizontal_layout;
use super::vertical::draw_vertical_layout;
use super::bar::draw_bar_layout;
use super::info_overlay::draw_info_overlay;

pub fn draw_piano_roll(
    ctx: &egui::Context,
    settings: &mut VisualSettings,
    notes: &[models::note::Note],
    total_ticks: u32,
    current_tick: u32,
    is_playing: bool,
    time_sig: &str,
    ppq: u32,
    fullscreen: &mut bool,
    played_indices: &HashSet<usize>,
    _file_name: &str,
    bpm: f32,
    active_note_names: &[String],
    bar_beat_str: &str,
    current_time_str: &str,
    chord: &Option<String>,
    track_name: &str,
    author: &str,
    note_count_played: usize,
    note_count_total: usize,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
        let painter = ui.painter().with_clip_rect(rect);

        let piano_bg = Color32::from_rgb(settings.piano_bg_r, settings.piano_bg_g, settings.piano_bg_b);
        painter.rect_filled(rect, 0.0, piano_bg);

        let layout = settings.layout_orientation;

        match layout {
            LayoutOrientation::Horizontal => {
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
                    played_indices,
                );
            }
            LayoutOrientation::Vertical => {
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
                    played_indices,
                );
            }
            LayoutOrientation::Bar => {
                draw_bar_layout(
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
                    played_indices,
                );
            }
        }

        let progress = if total_ticks > 0 { (current_tick as f32 / total_ticks as f32) * 100.0 } else { 0.0 };
        
        let mut info_overlay = settings.info_overlay.clone();
        draw_info_overlay(
            &painter,
            ui,
            &rect,
            &mut info_overlay,
            settings,
            track_name,
            author,
            note_count_played,
            note_count_total,
            time_sig,
            bpm,
            current_time_str,
            progress,
            active_note_names,
            bar_beat_str,
            chord,
        );
        settings.info_overlay = info_overlay;

        if *fullscreen {
            let button_size = 30.0;
            let button_rect = Rect::from_min_size(
                egui::pos2(rect.max.x - button_size - 10.0, rect.min.y + 10.0),
                egui::vec2(button_size, button_size),
            );
            let response = ui.interact(button_rect, egui::Id::new("fullscreen_exit"), egui::Sense::click());
            painter.rect_filled(button_rect, 4.0, Color32::from_rgba_premultiplied(200, 50, 50, 200));
            painter.text(
                button_rect.center(),
                egui::Align2::CENTER_CENTER,
                "✕",
                FontId::proportional(20.0),
                Color32::WHITE,
            );
            if response.clicked() {
                *fullscreen = false;
                ctx.request_repaint();
            }
        }
    });
}