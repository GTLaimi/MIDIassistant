use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::models::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DisplayMode {
    Time,
    TimePercent,
    NoteCount,
    NotePercent,
    BarBeat,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Time
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LayoutOrientation {
    Horizontal,
    Vertical,
    Bar,
}

impl Default for LayoutOrientation {
    fn default() -> Self {
        LayoutOrientation::Horizontal
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingsTab {
    Colors,
    Layout,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FollowMode {
    Edge,
    Center,
    Right,
}

impl Default for FollowMode {
    fn default() -> Self {
        FollowMode::Edge
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InfoField {
    TrackName,
    Author,
    NoteCount,
    TimeSig,
    Bpm,
    CurrentTime,
    ProgressPercent,
    ActiveNotes,
    BarBeat,
    Chord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InfoOverlaySettings {
    pub pos_x: f32,
    pub pos_y: f32,
    pub background_opacity: f32,
    pub enabled_fields: Vec<InfoField>,
    pub group_positions: Vec<(f32, f32)>,
    pub text_color_r: u8,
    pub text_color_g: u8,
    pub text_color_b: u8,
    pub show_border: bool,
    pub field_scales: Vec<f32>,
    pub field_font_names: Vec<String>,  // 新增
}

impl Default for InfoOverlaySettings {
    fn default() -> Self {
        let default_fonts = vec!["proportional".to_string(); ALL_INFO_FIELDS.len()];
        Self {
            pos_x: 0.02,
            pos_y: 0.02,
            background_opacity: 0.6,
            enabled_fields: vec![
                InfoField::TrackName,
                InfoField::Author,
                InfoField::NoteCount,
                InfoField::TimeSig,
                InfoField::Bpm,
                InfoField::CurrentTime,
            ],
            group_positions: vec![],
            text_color_r: 255,
            text_color_g: 255,
            text_color_b: 255,
            show_border: false,
            field_scales: vec![],
            field_font_names: default_fonts,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BarViewSettings {
    pub pos_x: f32,
    pub pos_y: f32,
    pub rotation: f32,
    pub width_ratio: f32,
    pub height_ratio: f32,
    pub display_pitch_offset: i32,
    pub display_pitch_range: i32,
    pub visible: bool,
}

impl Default for BarViewSettings {
    fn default() -> Self {
        Self {
            pos_x: 0.1,
            pos_y: 0.1,
            rotation: 0.0,
            width_ratio: 0.8,
            height_ratio: 0.7,
            display_pitch_offset: 0,
            display_pitch_range: 48,
            visible: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VisualSettings {
    pub piano_bg_r: u8, pub piano_bg_g: u8, pub piano_bg_b: u8,
    pub white_key_r: u8, pub white_key_g: u8, pub white_key_b: u8,
    pub black_key_r: u8, pub black_key_g: u8, pub black_key_b: u8,
    pub note_r: u8, pub note_g: u8, pub note_b: u8,
    pub active_note_r: u8, pub active_note_g: u8, pub active_note_b: u8,
    pub text_r: u8, pub text_g: u8, pub text_b: u8,
    pub cursor_r: u8, pub cursor_g: u8, pub cursor_b: u8,
    pub grid_bright_r: u8, pub grid_bright_g: u8, pub grid_bright_b: u8,
    pub grid_dim_r: u8, pub grid_dim_g: u8, pub grid_dim_b: u8,
    pub pitch_min: i32,
    pub pitch_range: i32,
    pub time_zoom: f32,
    pub follow_mode: FollowMode,
    pub manual_offset_ratio: f32,
    pub show_cursor: bool,
    pub enable_audio: bool,
    pub layout_orientation: LayoutOrientation,
    pub horizontal_keyboard_width: f32,
    pub vertical_keyboard_height: f32,
    pub vertical_black_key_offset: f32,
    pub vertical_black_key_width_scale: f32,
    pub sustain_highlight: bool,
    pub track_name: String,
    pub author: String,
    pub info_overlay: InfoOverlaySettings,
    pub bar_view: BarViewSettings,
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            piano_bg_r: 22, piano_bg_g: 22, piano_bg_b: 22,
            white_key_r: 240, white_key_g: 240, white_key_b: 240,
            black_key_r: 20, black_key_g: 20, black_key_b: 20,
            note_r: 100, note_g: 210, note_b: 100,
            active_note_r: 50, active_note_g: 200, active_note_b: 255,
            text_r: 0, text_g: 0, text_b: 0,
            cursor_r: 255, cursor_g: 255, cursor_b: 0,
            grid_bright_r: 110, grid_bright_g: 110, grid_bright_b: 110,
            grid_dim_r: 50, grid_dim_g: 50, grid_dim_b: 50,
            pitch_min: 40,
            pitch_range: 48,
            time_zoom: 0.5,
            follow_mode: FollowMode::default(),
            manual_offset_ratio: 0.0,
            show_cursor: true,
            enable_audio: true,
            layout_orientation: LayoutOrientation::Horizontal,
            horizontal_keyboard_width: 50.0,
            vertical_keyboard_height: 80.0,
            vertical_black_key_offset: 0.5,
            vertical_black_key_width_scale: 1.0,
            sustain_highlight: false,
            track_name: String::new(),
            author: String::new(),
            info_overlay: InfoOverlaySettings::default(),
            bar_view: BarViewSettings::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub notes: Vec<Note>,
    pub total_ticks: u32,
    pub current_tick: u32,
    pub is_playing: bool,
    pub file_loaded: bool,
    pub file_name: String,
    pub bpm: f32,
    pub ppq: u32,
    pub time_sig: String,
    pub load_error: Option<String>,
    pub settings: VisualSettings,
    pub show_settings: bool,
    pub display_mode: DisplayMode,
    pub settings_tab: SettingsTab,
    pub show_note_list: bool,
    pub active_theme_name: String,
    pub plugin_overrides_theme: bool,
    pub show_theme_window: bool,
    pub last_processed_tick: u32,
    pub is_rendering: bool,
    pub render_progress: f32,
    pub render_success: bool,
    pub fullscreen: bool,
    pub played_note_indices: HashSet<usize>,
    pub current_chord: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            total_ticks: 0,
            current_tick: 0,
            is_playing: false,
            file_loaded: false,
            file_name: String::new(),
            bpm: 120.0,
            ppq: 480,
            time_sig: "4/4".to_string(),
            load_error: None,
            settings: VisualSettings::default(),
            show_settings: false,
            display_mode: DisplayMode::Time,
            settings_tab: SettingsTab::Colors,
            show_note_list: false,
            active_theme_name: "default".to_string(),
            plugin_overrides_theme: false,
            show_theme_window: false,
            last_processed_tick: 0,
            is_rendering: false,
            render_progress: 0.0,
            render_success: false,
            fullscreen: false,
            played_note_indices: HashSet::new(),
            current_chord: None,
        }
    }
}

pub const ALL_INFO_FIELDS: &[InfoField] = &[
    InfoField::TrackName,
    InfoField::Author,
    InfoField::NoteCount,
    InfoField::TimeSig,
    InfoField::Bpm,
    InfoField::CurrentTime,
    InfoField::ProgressPercent,
    InfoField::ActiveNotes,
    InfoField::BarBeat,
    InfoField::Chord,
];