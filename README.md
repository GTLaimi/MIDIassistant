# MIDIassistant

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-v0.1.0-blue)

A lightweight Rust-based MIDI player with a piano roll visualization interface.

This is an early alpha release (v0.1.0). While functional, some parts are AI-assisted and the codebase is currently undergoing active refinement.

## Features
- Load `.mid` files
- Real-time physical tempo synchronization (BPM/PPQ)
- Standard piano keyboard rendering
- Playback indicator (playhead)

## Usage
1. Place a `.mid` file named `test.mid` in the project root.
2. Run the application:
```bash
cargo run --release
```
3. Use the Spacebar to Play/Pause and the 'R' key to Reset.

## License
MIT

## Development / Contributing
This project uses `#![deny(warnings)]` to ensure code quality. 
Before submitting a Pull Request, please make sure there are no compiler warnings.
To test locally:
1. Place a `test.mid` in the project root.
2. Run `cargo run --release` to build and launch the app.
