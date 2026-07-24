use crate::models::note::Note;
use anyhow::Result;
use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind};
use std::collections::HashMap;
use std::fs;

pub fn parse_midi(file_path: &str) -> Result<(Vec<Note>, u32, f32, u32, String)> {
    let data = fs::read(file_path)?;
    let midi = Smf::parse(&data)?;

    let mut notes = Vec::new();
    let mut active_notes: HashMap<u8, u32> = HashMap::new();
    let mut total_tick = 0u32;
    let mut tempo_micros = 500000u32; // 默认 500000 微秒 (120 BPM)
    let mut time_sig = "4/4".to_string();

    // 获取 PPQ (Ticks Per Beat)
    let ppq = match midi.header.timing {
        midly::Timing::Metrical(t) => t.as_int() as u32,
        _ => 480, // 默认退路
    };

    for track in &midi.tracks {
        let mut cur_tick = 0u32;
        for event in track.iter() {
            cur_tick += event.delta.as_int();
            if cur_tick > total_tick {
                total_tick = cur_tick;
            }

            match &event.kind {
                TrackEventKind::Midi { message, .. } => match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        active_notes.insert(key.as_int(), cur_tick);
                    }
                    MidiMessage::NoteOff { key, vel } | MidiMessage::NoteOn { key, vel }
                        if vel.as_int() == 0 =>
                    {
                        if let Some(&start_tick) = active_notes.get(&key.as_int()) {
                            notes.push(Note::new(
                                key.as_int(),
                                start_tick,
                                cur_tick - start_tick,
                                vel.as_int(),
                            ));
                            active_notes.remove(&key.as_int());
                        }
                    }
                    _ => {}
                },
                TrackEventKind::Meta(meta) => {
                    match meta {
                        MetaMessage::Tempo(t) => {
                            tempo_micros = t.as_int();
                        }
                        MetaMessage::TimeSignature(numerator, denominator, _, _) => {
                            let denom = 1u8 << denominator; // 2的幂次
                            time_sig = format!("{}/{}", numerator, denom);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    let bpm = 60_000_000.0 / tempo_micros as f32;
    Ok((notes, total_tick, bpm, ppq, time_sig))
}
