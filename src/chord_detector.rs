// src/chord_detector.rs

/// 检测当前激活的音符集合所构成的和弦。
/// 输入：活跃音符的 MIDI 音高列表（可包含重复）。
/// 输出：和弦名称（如 "Cmaj7"）或 `None`（无法识别）。
pub fn detect_chord(active_pitches: &[u8]) -> Option<String> {
    if active_pitches.is_empty() {
        return None;
    }

    // 提取音高类（0-11），去重并排序
    let mut pcs: Vec<u8> = active_pitches.iter().map(|&p| p % 12).collect();
    pcs.sort_unstable();
    pcs.dedup();

    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    // 和弦模板： (音程集合, 和弦后缀, 优先级)
    let templates: &[(&[u8], &str, u8)] = &[
        // 三和弦
        (&[0, 4, 7], "maj", 10),
        (&[0, 3, 7], "min", 10),
        (&[0, 3, 6], "dim", 10),
        (&[0, 4, 8], "aug", 10),
        (&[0, 2, 7], "sus2", 9),
        (&[0, 5, 7], "sus4", 9),
        // 七和弦
        (&[0, 4, 7, 10], "7", 20),
        (&[0, 4, 7, 11], "maj7", 20),
        (&[0, 3, 7, 10], "m7", 20),
        (&[0, 3, 7, 11], "m(maj7)", 20),
        (&[0, 3, 6, 9], "dim7", 20),
        (&[0, 3, 6, 10], "m7b5", 20),
        (&[0, 4, 8, 10], "aug7", 20),
        (&[0, 4, 7, 10, 14 % 12], "9", 30),      // 14%12=2
        (&[0, 4, 7, 11, 14 % 12], "maj9", 30),
        (&[0, 3, 7, 10, 14 % 12], "m9", 30),
        // 挂留七
        (&[0, 5, 7, 10], "7sus4", 25),
        (&[0, 2, 7, 10], "7sus2", 25),
        // 添加音
        (&[0, 4, 7, 14 % 12], "add9", 15),
        (&[0, 3, 7, 14 % 12], "madd9", 15),
        (&[0, 4, 7, 17 % 12], "add11", 15),
        // 六和弦
        (&[0, 4, 7, 9], "6", 12),
        (&[0, 3, 7, 9], "m6", 12),
        (&[0, 4, 7, 9, 14 % 12], "6/9", 18),
        // 变化七
        (&[0, 4, 7, 10, 15 % 12], "7#9", 22),
        (&[0, 4, 7, 10, 13 % 12], "7b9", 22),
        (&[0, 4, 7, 11, 13 % 12], "maj7#11", 22),
        // 增七
        (&[0, 4, 8, 10], "+7", 20),
        (&[0, 4, 8, 11], "maj7#5", 20),
    ];

    // 对每个可能的根音进行匹配
    let mut best_match: Option<(String, f32, u8)> = None;

    for root in 0..12 {
        // 计算相对根音的音程集合（归一化到0-11）
        let normalized: Vec<u8> = pcs
            .iter()
            .map(|&pc| ((pc + 12 - root) % 12) as u8)
            .collect();

        for (intervals, suffix, priority) in templates {
            // 计算匹配度：模板中的音程在 normalized 中出现的比例
            let mut matched_count = 0;
            for &interval in *intervals {
                if normalized.contains(&interval) {
                    matched_count += 1;
                }
            }
            let match_ratio = matched_count as f32 / intervals.len() as f32;

            // 只接受匹配度 >= 0.8 的候选（允许省略音）
            if match_ratio >= 0.8 {
                // 计算额外惩罚：如果输入音符比模板多很多，降低得分
                let extra_penalty = if normalized.len() > intervals.len() {
                    (normalized.len() - intervals.len()) as f32 * 0.05
                } else {
                    0.0
                };
                // 修正：解引用 priority
                let score = match_ratio * 100.0 + (*priority as f32) - extra_penalty;

                // 选择最高分
                if best_match.is_none() || score > best_match.as_ref().unwrap().1 {
                    // 修正：存储解引用后的 priority
                    let name = format!("{}{}", note_names[root as usize], suffix);
                    best_match = Some((name, score, *priority));
                }
            }
        }
    }

    // 如果找到了匹配，返回最高分的
    if let Some((name, _, _)) = best_match {
        return Some(name);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_major() {
        let pitches = vec![60, 64, 67];
        assert_eq!(detect_chord(&pitches), Some("Cmaj".to_string()));
    }

    #[test]
    fn test_a_minor() {
        let pitches = vec![57, 60, 64];
        assert_eq!(detect_chord(&pitches), Some("Amin".to_string()));
    }

    #[test]
    fn test_g7() {
        let pitches = vec![55, 59, 62, 65];
        assert_eq!(detect_chord(&pitches), Some("G7".to_string()));
    }

    #[test]
    fn test_c_major7() {
        let pitches = vec![60, 64, 67, 71];
        assert_eq!(detect_chord(&pitches), Some("Cmaj7".to_string()));
    }

    #[test]
    fn test_c_sus4() {
        let pitches = vec![60, 65, 67];
        assert_eq!(detect_chord(&pitches), Some("Csus4".to_string()));
    }

    #[test]
    fn test_d_minor7() {
        let pitches = vec![62, 65, 69, 72];
        assert_eq!(detect_chord(&pitches), Some("Dm7".to_string()));
    }
}