use std::fs::File;
use std::io::{BufReader};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig, BufferSize,
};
use rodio::{OutputStream, Sink};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

use crate::state::AppState;
use crate::models::note::Note;

pub struct AudioEngine {
    sound_font: Arc<SoundFont>,
    _stream: OutputStream,
    sink: Sink,
    cached_audio: Option<Arc<Vec<f32>>>,
    is_playing: bool,
    render_state: Option<RenderState>,
    playback_stream: Option<Stream>,
    playback_cursor: Arc<AtomicUsize>,
    output_sample_rate: u32,
}

struct RenderState {
    synthesizer: Synthesizer,
    left: Vec<f32>,
    right: Vec<f32>,
    offset: usize,
    total_samples: usize,
    ticks_per_second: f64,
    sample_rate: f64,
    notes: Vec<Note>,
    current_note_idx: usize,
}

impl AudioEngine {
    pub fn new(sf2_path: &str) -> Result<Self, String> {
        let file = File::open(sf2_path).map_err(|e| format!("打开SF2失败: {}", e))?;
        let mut reader = BufReader::new(file);
        let sound_font = SoundFont::new(&mut reader)
            .map_err(|e| format!("解析SF2失败: {}", e))?;
        let sound_font = Arc::new(sound_font);

        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("打开音频输出失败: {}", e))?;
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("创建Sink失败: {}", e))?;
        sink.play();

        // ---- 选择最高品质采样率（优先 96000 / 48000） ----
        let host = cpal::default_host();
        let sample_rate = if let Some(device) = host.default_output_device() {
            let mut best_rate = 44100;
            if let Ok(supported) = device.supported_output_configs() {
                for cfg in supported {
                    if cfg.channels() == 2 {
                        let max_rate = cfg.max_sample_rate().0;
                        if max_rate >= 96000 {
                            best_rate = 96000;
                            break;
                        } else if max_rate >= 48000 {
                            best_rate = 48000;
                        } else if max_rate > best_rate {
                            best_rate = max_rate;
                        }
                    }
                }
            }
            best_rate
        } else {
            44100
        };

        Ok(Self {
            sound_font,
            _stream,
            sink,
            cached_audio: None,
            is_playing: false,
            render_state: None,
            playback_stream: None,
            playback_cursor: Arc::new(AtomicUsize::new(0)),
            output_sample_rate: sample_rate,
        })
    }

    pub fn clear(&mut self) {
        self.stop_playback();
        self.cached_audio = None;
    }

    fn stop_playback(&mut self) {
        if let Some(stream) = self.playback_stream.take() {
            drop(stream);
        }
        self.playback_cursor.store(0, Ordering::Release);
        self.is_playing = false;
    }

    fn create_stream(&mut self, audio_data: Arc<Vec<f32>>, start_paused: bool) -> Result<(), String> {
        self.stop_playback();

        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or("没有可用的音频输出设备")?;

        let desired_rate = self.output_sample_rate;
        let mut config: Option<StreamConfig> = None;

        if let Ok(supported_iter) = device.supported_output_configs() {
            // supported_iter is an iterator that gets moved when iterated once,
            // so collect it to reuse below.
            let supported: Vec<_> = supported_iter.collect();
            for cfg in &supported {
                if cfg.channels() == 2
                    && cfg.min_sample_rate().0 <= desired_rate
                    && cfg.max_sample_rate().0 >= desired_rate
                {
                    config = Some(cfg.with_sample_rate(cpal::SampleRate(desired_rate)).config());
                    break;
                }
            }
            if config.is_none() {
                for cfg in &supported {
                    if cfg.channels() == 2 {
                        config = Some(cfg.with_max_sample_rate().config());
                        break;
                    }
                }
            }
        }

        let stream_config = if let Some(cfg) = config {
            cfg
        } else {
            match device.default_output_config() {
                Ok(cfg) => {
                    if cfg.channels() == 2 {
                        cfg.config()
                    } else {
                        let mut fallback = None;
                        if let Ok(supported) = device.supported_output_configs() {
                            for c in supported {
                                if c.channels() == 2 {
                                    fallback = Some(c.with_max_sample_rate().config());
                                    break;
                                }
                            }
                        }
                        fallback.unwrap_or(StreamConfig {
                            channels: 2,
                            sample_rate: cpal::SampleRate(desired_rate),
                            buffer_size: BufferSize::Default,
                        })
                    }
                }
                Err(e) => return Err(format!("无法获取默认配置: {}", e)),
            }
        };

        let data_clone = audio_data.clone();
        let cursor_clone = self.playback_cursor.clone();

        let stream = device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let pos = cursor_clone.load(Ordering::Acquire);
                let total = data_clone.len();
                let remaining = total.saturating_sub(pos);
                let to_copy = remaining.min(data.len());

                if to_copy > 0 {
                    data[..to_copy].copy_from_slice(&data_clone[pos..pos + to_copy]);
                    cursor_clone.store(pos + to_copy, Ordering::Release);
                } else {
                    data.fill(0.0);
                }
            },
            |err| eprintln!("[音频流错误] {}", err),
            None,
        ).map_err(|e| format!("创建流失败: {}", e))?;

        stream.play().map_err(|e| format!("启动流失败: {}", e))?;
        if start_paused {
            stream.pause().map_err(|e| format!("暂停流失败: {}", e))?;
        }

        self.playback_stream = Some(stream);
        self.is_playing = !start_paused;

        Ok(())
    }

    pub fn start_pre_render(&mut self, notes: &[Note], total_ticks: u32, bpm: f32, ppq: u32) -> Result<(), String> {
        self.clear();

        if notes.is_empty() || total_ticks == 0 {
            return Ok(());
        }

        let sample_rate = self.output_sample_rate as f64;
        let bps = bpm as f64 / 60.0;
        let ticks_per_second = bps * ppq as f64;
        let duration_secs = total_ticks as f64 / ticks_per_second;
        let total_samples = (duration_secs * sample_rate) as usize;

        if total_samples == 0 {
            return Ok(());
        }

        let mut sorted_notes = notes.to_vec();
        sorted_notes.sort_by_key(|n| n.start_tick);

        let mut settings = SynthesizerSettings::new(sample_rate as i32);
        settings.block_size = 1024;
        settings.maximum_polyphony = 256;
        settings.enable_reverb_and_chorus = true;

        let mut synthesizer = Synthesizer::new(&self.sound_font, &settings)
            .map_err(|e| format!("创建合成器失败: {}", e))?;

        synthesizer.process_midi_message(0, 0xB0, 0, 0);
        synthesizer.process_midi_message(0, 0xB0, 32, 0);
        synthesizer.process_midi_message(0, 0xC0, 0, 0);

        self.render_state = Some(RenderState {
            synthesizer,
            left: vec![0.0f32; total_samples],
            right: vec![0.0f32; total_samples],
            offset: 0,
            total_samples,
            ticks_per_second,
            sample_rate,
            notes: sorted_notes,
            current_note_idx: 0,
        });

        Ok(())
    }

    pub fn render_next_chunk(&mut self) -> (bool, f32) {
        let rs = match &mut self.render_state {
            Some(s) => s,
            None => return (true, 1.0),
        };

        if rs.offset >= rs.total_samples {
            let mut interleaved = Vec::with_capacity(rs.total_samples * 2);
            let gain = 2.0_f32;
            for i in 0..rs.total_samples {
                let l = (rs.left[i] * gain).clamp(-1.0, 1.0);
                let r = (rs.right[i] * gain).clamp(-1.0, 1.0);
                interleaved.push(l);
                interleaved.push(r);
            }

            self.stop_playback();
            let audio_data = Arc::new(interleaved);
            self.cached_audio = Some(audio_data.clone());
            self.render_state = None;

            // 预创建流（暂停状态）
            if let Err(e) = self.create_stream(audio_data, true) {
                eprintln!("[WARN] 预创建流失败: {}", e);
            }

            return (true, 1.0);
        }

        let chunk_size = 2048;
        let frames = (rs.total_samples - rs.offset).min(chunk_size);

        let start_tick = (rs.offset as f64 / rs.sample_rate) * rs.ticks_per_second;
        let end_tick = ((rs.offset + frames) as f64 / rs.sample_rate) * rs.ticks_per_second;

        while rs.current_note_idx < rs.notes.len() {
            let note = &rs.notes[rs.current_note_idx];
            let note_start = note.start_tick as f64;
            let note_end = note_start + note.duration as f64;
            let original_vel = note.velocity;
            let effective_vel = if original_vel == 0 { 80 } else { original_vel };

            if note_start >= end_tick {
                break;
            }
            if note_start >= start_tick && note_start < end_tick && effective_vel > 0 {
                rs.synthesizer.process_midi_message(0, 0x90, note.pitch as i32, effective_vel as i32);
            }
            if note_end >= start_tick && note_end < end_tick {
                rs.synthesizer.process_midi_message(0, 0x80, note.pitch as i32, 0);
            }
            rs.current_note_idx += 1;
        }

        let left_slice = &mut rs.left[rs.offset..rs.offset + frames];
        let right_slice = &mut rs.right[rs.offset..rs.offset + frames];
        rs.synthesizer.render(left_slice, right_slice);

        rs.offset += frames;
        let progress = rs.offset as f32 / rs.total_samples as f32;
        (false, progress)
    }

    pub fn process_playback(&mut self, state: &mut AppState) {
        if !state.file_loaded || !state.settings.enable_audio {
            self.stop_playback();
            self.is_playing = false;
            return;
        }

        let audio_data = match &self.cached_audio {
            Some(data) => data.clone(),
            None => {
                state.is_playing = false;
                self.is_playing = false;
                return;
            }
        };

        if audio_data.is_empty() {
            state.is_playing = false;
            self.is_playing = false;
            return;
        }

        // 若流尚未创建，补建
        if self.playback_stream.is_none() {
            if let Err(e) = self.create_stream(audio_data.clone(), true) {
                eprintln!("[ERROR] 补建流失败: {}", e);
                state.is_playing = false;
                self.is_playing = false;
                return;
            }
        }

        let should_play = state.is_playing && !state.is_rendering;

        if let Some(stream) = &self.playback_stream {
            if should_play {
                if let Err(e) = stream.play() {
                    eprintln!("[WARN] 恢复播放失败: {}", e);
                }
                self.is_playing = true;
            } else {
                if let Err(e) = stream.pause() {
                    eprintln!("[WARN] 暂停播放失败: {}", e);
                }
                self.is_playing = false;
            }
        }

        // 更新游标跟随 MIDI 进度
        let total_samples = audio_data.len();
        let target_pos = if state.total_ticks > 0 {
            let progress = state.current_tick as f64 / state.total_ticks as f64;
            (progress * total_samples as f64) as usize
        } else {
            0
        };
        let target_pos = target_pos.min(total_samples);

        let current_cursor = self.playback_cursor.load(Ordering::Acquire);
        if target_pos.abs_diff(current_cursor) > 4096 {
            self.playback_cursor.store(target_pos, Ordering::Release);
        }

        // 检测播放结束
        let cursor = self.playback_cursor.load(Ordering::Acquire);
        if cursor >= total_samples {
            state.is_playing = false;
            self.is_playing = false;
            if let Some(stream) = &self.playback_stream {
                let _ = stream.pause();
            }
            self.playback_cursor.store(0, Ordering::Release);
        }
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.sink.stop();
        self.clear();
    }
}