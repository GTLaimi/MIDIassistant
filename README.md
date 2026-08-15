# 🎹 MIDIassistant

> Lightweight, high-precision MIDI player and visualizer

![Version](https://img.shields.io/badge/version-v0.2.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-green)

MIDIassistant is a high-precision, low-latency MIDI playback and visualization tool written in Rust using `egui` and `eframe`. It supports multiple layouts, real-time chord detection, customizable themes, and information overlays, allowing you to enjoy MIDI music in a more intuitive way.

---

## ✨ Features

- **🎵 MIDI Parsing & Playback**: Standard MIDI file support (.mid/.midi), accurate parsing of notes, tempo, time signature, with f64 timing calculations for zero drift even in long files.
- **🎨 Three Layouts**:
  - **Horizontal**: Classic keyboard-left layout with horizontal timeline.
  - **Vertical Waterfall**: Keyboard at bottom, vertical upward timeline for note duration observation.
  - **Bar View**: Focuses on notes in the current bar, with rotation and drag-to-position controls.
- **Adjustable Keyboard**: Drag handle to resize width/height; can be fully hidden.
- **High-quality Audio Engine**: `rustysynth` + `cpal`, with reverb/chorus support and adaptive sample rate.
- **🎨 Theme System**: 7+ built-in themes (forest, pink_dream, sunny, moon, etc.) and full custom TOML theme support.
- **Playhead Following Modes**: Edge / Center / Right-biased to suit different viewing habits.
- **Floating Info Overlay**: Independently show/hide 10 categories of info (track title, composer, time, tempo, chord, etc.), draggable and scalable.
- **Persistent Note Highlighting**: Played notes stay highlighted for easy review.
- **Fullscreen Mode** (F11) for immersive experience.
- **Multiple Display Modes**: Time, percentage, note count, bar/beat – one-click switch.

---

## ✨ New in v0.2.0

- Three playhead following modes (Edge / Center / Right-biased)
- Fullscreen mode (F11)
- Floating info overlay with 10 independent toggles, drag & scale
- Bar View with rotation and drag positioning
- Persistent note highlighting
- Customizable track title and author display
- 7+ new themes and full color customization (background, keyboard, notes, grid, playhead, etc.)
- Adjustable keyboard size via drag (both horizontal and vertical)

---

## 📥 Download & Installation

### Windows Users
Download the latest `.exe` installer from the [Releases](https://github.com/yourusername/MIDIassistant/releases) page and double‑click to run.

### Build from Source (macOS / Linux / Windows)

Prerequisites:
- Rust 1.70+
- Cargo

Clone and build:

    git clone https://github.com/yourusername/MIDIassistant.git
    cd MIDIassistant
    cargo build --release
    ./target/release/midi_assistant

### Soundfont
Place a `piano.sf2` soundfont file in the program's working directory for audio rendering.

---

## 🎮 Basic Controls

| Shortcut      | Function |
|---------------|----------|
| Space         | Play / Pause |
| R             | Reset playback position |
| F11           | Toggle fullscreen mode |
| Mouse drag    | Drag progress bar / keyboard divider / info overlay / bar box |

> **Note**: Audio pre‑rendering cannot be disabled before loading a MIDI file for the first time; otherwise, lag or crashes may occur (already being fixed 😊).

---

## 🖥️ Interface Overview

- **Sidebar**: Load MIDI, switch layouts, toggle fullscreen, persistent highlighting, show track info and chords.
- **Piano Roll**: Main display area – choose from three layouts.
- **Bottom Transport**: Playback controls, progress bar, time/progress/bar information.
- **Settings Window**: Comprehensive customization of colors, layout, track info, info overlay, and more.

---

## 🎨 Theme Customization

Theme files are stored in `./themes/` directory in TOML format. Example:

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

You can also adjust colors directly from the settings window without editing files.

---

## 📁 Project Structure

    MIDIassistant/
    ├── src/                 # Source code
    │   ├── audio_engine.rs  # Audio rendering & playback
    │   ├── midi_parser.rs   # MIDI parsing
    │   ├── playback.rs      # Time synchronization
    │   ├── state.rs         # Global state
    │   ├── theme_manager.rs # Theme management
    │   └── ui/              # UI components
    │       ├── piano_roll.rs    # Piano roll rendering
    │       ├── settings.rs      # Settings window
    │       └── sidebar.rs       # Sidebar panel
    ├── themes/              # Preset themes
    ├── piano.sf2            # Soundfont (user-provided)
    └── Cargo.toml

---

## 🧩 System Requirements

- Windows 7 / 10 / 11 (recommended) – other platforms require Rust toolchain for compilation.
- Audio output: WASAPI / ALSA / CoreAudio compatible device.

---

## 🛠 Roadmap

- **v0.2.0** (current): Enhanced visualization, info overlay, bar view, persistent highlights, more themes.
- **v0.3.0**: MIDI editing (drag, move, stretch notes), MP3/MP4 export.
- **v1.0.0**: Cross-platform stable release with VST plugin support.

---

## 📄 License

MIT License © 2024–2026 MIDIassistant Contributors

---

## 🙏 Acknowledgements

- [egui](https://github.com/emilk/egui) – immediate‑mode GUI
- [midly](https://github.com/Stupremee/midly) – MIDI parsing
- [rustysynth](https://github.com/rustysynth/rustysynth) – MIDI synthesis
- [cpal](https://github.com/RustAudio/cpal) – audio output

---

**Enjoy your music! 🎵**
