# MIDIassistant

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-v0.1.4-blue)

> A lightweight, real-time MIDI piano roll visualizer written in Rust using `egui` and `eframe`. Fully customizable and ready for your creative workflow.

<img width="1920" height="1080" alt="屏幕截图 2026-07-27 100738" src="https://github.com/user-attachments/assets/26f9763c-742e-4e5d-9f62-db537c344f92" />

## ✨ Features (v0.1.4 Update)

- **MIDI File Loading**: Open any `.mid` file directly via the sidebar.
- **Real-time Physical Synchronization**: Precise BPM/PPQ audio engine with a smooth playback head.
- **Piano Roll View**: Piano keyboard with fully customizable colors and dynamic pitch labels.
- **Ultimate Visual Customization**:
  - **Global Settings Window**: Reorganized into three tabs: Colors, Layout, and About.
  - **Full RGB Color Control**: Customize Background, White Keys, Black Keys, Notes, Active Notes, Cursor, Text, Bright Grid Lines, and Dim Grid Lines independently.
  - **Theme Manager & Ecosystem**: Apply pre-made `.toml` themes or save your current color setup as a new theme. Theme files are stored in the `themes/` folder.
  - **Independent Layout Presets**: Applying a theme will **only** change colors and grid settings; your Layout options (Zoom, Pitch Range, etc.) will remain untouched!
- **Layout & Scaling**:
  - Vertical Pitch Shift and Vertical Zoom.
  - Horizontal Zoom (0.05 ~ 0.9).
  - Playhead auto-following switch.
- **Transport Bar**: Simple playback controls with a draggable progress bar. The progress bar width is fixed to prevent text-size jitter.
- **Note List Inspector**: Open a window to view all parsed notes (Index, Start Tick, Pitch Name, Duration, Velocity) and export them as a `.txt` file.

## 🚀 How to Build & Run

```bash
# Clone the repository
git clone https://github.com/GTLaimi/MIDIassistant.git
cd MIDIassistant

# Build the release version
cargo build --release

# Run the application
cargo run --release
```
(Pre-compiled .exe for Windows is available in the Releases page!)

## 📝 License

Licensed under the MIT license.
