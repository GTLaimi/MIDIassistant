use crate::state::AppState;
use std::time::Instant;

pub struct PlaybackEngine {
    tick_rate: f32,
    last_time: Instant,
    elapsed_accumulator: f32, // 解决帧率不均导致的时间碎片问题
}

impl PlaybackEngine {
    pub fn new(bpm: f32, ppq: u32) -> Self {
        Self {
            tick_rate: (bpm / 60.0) * ppq as f32, // 我们的原子公式
            last_time: Instant::now(),
            elapsed_accumulator: 0.0,
        }
    }

    // 核心更新函数：根据真实时间流逝，推动 AppState 的 current_tick
    pub fn update(&mut self, state: &mut AppState) {
        if !state.is_playing {
            // 如果暂停，保持 last_time 不变，避免重新播放时产生时间跳跃
            self.last_time = Instant::now();
            self.elapsed_accumulator = 0.0;
            return;
        }

        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;

        // 把现实世界的时间累积起来（精确到微秒级）
        self.elapsed_accumulator += delta.as_secs_f32();

        // 只要累积的时间够推进 1 个 Tick，就推进一次
        let tick_duration = 1.0 / self.tick_rate; // 1个Tick等于现实世界的多少秒
        while self.elapsed_accumulator >= tick_duration {
            state.current_tick += 1;
            self.elapsed_accumulator -= tick_duration;

            // 防止超出曲目总时长
            if state.current_tick >= state.total_ticks {
                state.current_tick = state.total_ticks;
                state.is_playing = false; // 自动停止播放
                break;
            }
        }
    }

    // 重置播放头
    pub fn reset(&mut self) {
        self.last_time = Instant::now();
        self.elapsed_accumulator = 0.0;
    }
}
