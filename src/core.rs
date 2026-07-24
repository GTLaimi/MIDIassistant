pub struct TempoCalculator;

impl TempoCalculator {
    // 你验证过的原子公式：每秒推进的 Ticks
    pub fn get_tick_rate(bpm: f32, ppq: u32) -> f32 {
        (bpm / 60.0) * ppq as f32
    }

    // 通过刻度和速率计算当前秒数
    pub fn get_current_seconds(current_tick: u32, tick_rate: f32) -> f32 {
        current_tick as f32 / tick_rate
    }
}
