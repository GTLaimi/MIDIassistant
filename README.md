
# 🎹 MIDIassistant

> Lightweight, high-precision MIDI player and visualizer

![Version](https://img.shields.io/badge/version-v0.1.5-blue)
![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-v0.1.4-blue)

> A lightweight, real-time MIDI piano roll visualizer written in Rust using `egui` and `eframe`. Fully customizable and ready for your creative workflow.


## ✨ Features

- **🎵 MIDI Parsing & Playback**: Standard MIDI file support (.mid/.midi), accurate parsing of notes, tempo, time signature
- **🎨 Dual-layout Piano Roll**:
  - **Horizontal**: Classic keyboard-left layout with horizontal timeline
  - **Vertical Waterfall**: Keyboard at bottom, vertical upward timeline for note duration observation
- **Adjustable Keyboard**: Drag handle to resize, can be fully hidden
- **High-quality Audio Engine**: rustysynth + cpal, reverb/chorus support, adaptive sample rate
- **🎨 Theme System**: 7 built-in themes, custom TOML theme support
- **🎯 Precise Sync**: f64 timing calculations, no tempo drift even in long MIDI files
- **📊 Multiple Display Modes**: Time, percentage, note count, bar/beat, one-click switch

## 🚀 Quick Start

### Requirements
- Rust 1.97+ (for compilation)
- Soundfont file `piano.sf2` (place in program root)

### Installation

**Build from source**

```bash
git clone https://github.com/yourusername/MIDIassistant.git
cd MIDIassistant
cargo build --release
```
(Pre-compiled .exe for Windows is available in the Releases page!)


### Usage

1. Place piano.sf2 in program root
2. Launch program, click "加载 MIDI 文件" (Load MIDI) to select .mid file
3. Use Space to play/pause, R to reset
4. Switch horizontal/vertical layout in sidebar
5. Drag separator to adjust keyboard size

### 🎨 Theme Customization

Theme files are in ./themes/ directory in TOML format. Example:
```toml
name = "My Theme"
description = "Custom color scheme"

[settings]
piano_bg_r = 22
piano_bg_g = 22
piano_bg_b = 22
white_key_r = 240
white_key_g = 240
white_key_b = 240
note_r = 100
note_g = 210
note_b = 100
# ... more color fields
```

### 📁 Project Structure
```text
MIDIassistant/
├── src/                 # Source code
│   ├── audio_engine.rs  # Audio engine (render & play)
│   ├── midi_parser.rs   # MIDI parsing
│   ├── playback.rs      # Time synchronization
│   ├── state.rs         # Global state
│   ├── theme_manager.rs # Theme management
│   └── ui/              # UI components
│       ├── piano_roll.rs    # Piano roll
│       ├── settings.rs      # Settings window
│       └── sidebar.rs       # Sidebar
├── themes/              # Preset themes
├── piano.sf2            # Soundfont (need to provide)
└── Cargo.toml
```
### 🛠 Roadmap
- v0.2.0: MIDI editing (drag, move, stretch notes)
- v0.3.0: High-quality export (MP3/MP4), VST plugin support
- v1.0.0: Cross-platform stable release

### 📄 License
MIT License © 2024 MIDIassistant Contributors
