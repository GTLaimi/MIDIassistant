use crate::models::note::Note;

#[derive(Debug, Clone)]
pub struct AppState {
    pub notes: Vec<Note>,
    pub total_ticks: u32,
    pub current_tick: u32, // 播放头当前的刻度
    pub is_playing: bool,  // 是否为播放状态
    pub file_loaded: bool,
    pub file_name: String,
    pub bpm: f32,
    pub ppq: u32,
    pub time_sig: String,
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
        }
    }
}
