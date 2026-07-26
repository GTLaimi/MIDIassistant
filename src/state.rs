use crate::models::note::Note;

#[derive(Debug, Clone)]
pub struct VisualSettings {
    // 颜色
    pub bg_r: f32,
    pub bg_g: f32,
    pub bg_b: f32,
    // 键盘
    pub kbd_width: f32,
    pub black_key_width: f32,
    pub black_key_height_ratio: f32,
    // 卷帘窗与音符
    pub pitch_min: i32, // 【核心】基准音高（配合垂直滑动条）
    pub note_height_ratio: f32,
    pub label_y_offset: f32,
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            // 默认应用你刚刚调试出来的完美参数
            bg_r: 0.200,
            bg_g: 0.200,
            bg_b: 0.200,
            kbd_width: 50.0,
            black_key_width: 50.0,
            black_key_height_ratio: 0.88,
            pitch_min: 50,
            note_height_ratio: 0.80,
            label_y_offset: 121.0,
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
    // 将调试参数升级为正式视觉设置
    pub settings: VisualSettings,
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
        }
    }
}