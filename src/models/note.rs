#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub pitch: u8,
    pub start_tick: u32,
    pub duration: u32,
    pub velocity: u8,
    pub pitch_name: String,
}

impl Note {
    pub fn new(pitch: u8, start_tick: u32, duration: u32, velocity: u8) -> Self {
        let notes = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let note_name = notes[(pitch % 12) as usize];
        let octave = (pitch / 12) as i8 - 1;
        Self {
            pitch,
            start_tick,
            duration,
            velocity,
            pitch_name: format!("{}{}", note_name, octave),
        }
    }
}
