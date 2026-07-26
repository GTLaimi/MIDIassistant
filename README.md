# MIDIassistant

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-v0.1.1-blue)
![Build](https://img.shields.io/github/actions/workflow/status/GTLaimi/MIDIassistant/rust.yml?branch=main)

> `[中文文档](README_zh.md)`

> A lightweight, real-time MIDI piano roll visualizer written in Rust using `egui` and `eframe`.

<img width="1920" height="1080" alt="屏幕截图 2026-07-26 104003" src="https://github.com/user-attachments/assets/00f73b4e-08e9-4bb5-bf04-bf62df8eac05" />



## ✨ Features (v0.1.1)
- **Load `.mid` files** via native file selector (no more hardcoding `test.mid`).
- **Real-time physical synchronization**: Precise BPM/PPQ conversion engine.
- **Piano roll view** with standard keyboard rendering and pitch grid alignment.
- **Active note highlighting**: Notes turn cyan and glow when the playhead passes over them.
- **Playback head** (bright yellow indicator).
- **Sidebar information**: Displays BPM, time signature, and total notes.
- **Error handling**: Clear error prompts in the UI if a MIDI file fails to load.

  ## 🎨 Powerful Visual Customization (v0.1.2)
The app now includes a built-in `⚙️ Visual Settings` menu. You can tweak the following parameters in real-time:
- **Background RGB**: Fine-tune the workspace color with 0.0 ~ 1.0 sliders and instant preview.
- **Keyboard Scaling**: Adjust the visual width and height of both White and Black keys.
- **Note & Label Tweaks**: Modify the note block height ratio and adjust the vertical position of note name labels (e.g., C4).
- **Vertical Range Slider**: Easily scroll the visible pitch range up and down.

> ❗ **Piano Roll Fix**: The keyboard rendering has been completely overhauled. It now uses the physically correct **7-white-key, 5-black-key** layout per octave, eliminating the previous alignment errors. 

## ⌨️ Controls
- `Space` : Play / Pause
- `R` : Reset to the beginning of the track
- `Button` : "打开 MIDI 文件" to open any MIDI file on your computer

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
(💡 Note: A pre-compiled .exe for Windows is available in the Releases page!)

## 📝 License
Licensed under the MIT license.
