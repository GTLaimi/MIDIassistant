# 🎹 HesychiaMidS

> 轻量级、高精度 MIDI 播放与可视化工具

![Version](https://img.shields.io/badge/version-v0.2.2-blue)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-green)

HesychiaMidS 是一款使用 Rust 和 `egui`/`eframe` 编写的高精度、低延迟 MIDI 播放与可视化工具。它支持多种布局、实时和弦检测、可自定义主题与信息覆盖层，让您以更直观的方式欣赏 MIDI 音乐。

---

## ✨ 功能特性

- **🎵 MIDI 解析与播放**：支持标准 MIDI 文件（.mid/.midi），精确解析音符、速度、拍号，使用 f64 精度计算时间，长时间播放也无漂移。
- **🎨 三种布局**：
  - **水平布局**：经典键盘在左，时间轴水平向右。
  - **垂直瀑布流**：键盘在底部，时间轴竖直向上，便于观察音符持续时间。
  - **小节视图**：聚焦当前小节的音符，支持旋转和拖拽定位。
- **可调键盘**：拖拽手柄调整宽度/高度，也可完全隐藏。
- **高质量音频引擎**：基于 `rustysynth` + `cpal`，支持混响/合唱，自适应采样率。
- **🎨 主题系统**：7+ 内置主题（森林、粉梦、阳光、月光等），支持完全自定义 TOML 主题文件。
- **播放头跟随模式**：边缘 / 居中 / 偏右，适应不同观看习惯。
- **浮动信息覆盖层**：可独立显示/隐藏 10 类信息（曲目名、作曲者、时间、速度、和弦等），支持拖拽位置和缩放。
- **音符持续高亮**：播放过的音符保持高亮，便于回顾。
- **全屏模式**（F11）提供沉浸式体验。
- **多种显示模式**：时间、百分比、音符数、小节/节拍，一键切换。

---

## ✨ v0.2.0 新特性

- 三种播放头跟随模式（边缘 / 居中 / 偏右）
- 全屏模式（F11）
- 浮动信息覆盖层，10 类信息独立开关，支持拖拽与缩放
- 小节视图，支持旋转和拖拽定位
- 音符持续高亮
- 可自定义曲目名和作曲者显示
- 7+ 款新主题，全颜色自定义（背景、键盘、音符、网格、播放头等）
- 键盘尺寸可拖拽调整（水平/垂直）
- 通过向 `fonts` 目录添加字体文件来更换信息覆盖层的字体

---

## 📥 下载与安装

### Windows 用户

从 [Releases](https://github.com/GTLaimi/HesychiaMidS/releases) 页面下载最新 `.exe` 安装包，双击运行即可。

### 从源码编译（macOS / Linux / Windows）

前置要求：
- Rust 1.70+
- Cargo

克隆并构建：

```bash
git clone https://github.com/GTLaimi/HesychiaMidS.git
cd HesychiaMidS
cargo build --release
./target/release/hesychiamids
```

或直接通过 cargo 安装：

```bash
cargo install hesychiamids
```

### 音色库（Soundfont）
请将 `piano.sf2` 音色库文件放置在程序运行目录下，以启用音频渲染。

### 字体（Fonts）
请将 `fonts` 目录放置在程序运行目录下，用于更换信息覆盖层的字体。

---

## 🐍 命令行版本：`hesym`

需要命令行解析 MIDI？试试 **[hesym](https://github.com/GTLaimi/hesym)** —— 基于相同核心解析逻辑的 Python CLI 工具。

```bash
pip install hesym
hesym song.mid --stats
```

- 轻量级，可在任何 Python 环境运行
- 支持 JSON 输出，便于脚本集成
- 适合服务器、CI/CD 或 Termux 环境

---

## 🎮 基本操作

| 快捷键      | 功能 |
|------------|------|
| Space      | 播放 / 暂停 |
| R          | 重置播放位置 |
| F11        | 切换全屏模式 |
| 鼠标拖拽   | 拖动进度条 / 键盘分隔条 / 信息覆盖 / 小节方框 |

> **注意**：在首次加载 MIDI 文件之前，不能关闭音频预渲染，否则可能导致卡顿或闪退（已在修复中 😊）。  
> 如果您下载了 v0.2.1 版本，请切换到 v0.2.2 以获取稳定的音频引擎。

---

## 🖥️ 界面说明

- **侧边栏**：加载 MIDI、切换布局、切换全屏、持续高亮、显示曲目信息和和弦。
- **卷帘窗**：主显示区域，支持三种布局。
- **底部走带**：播放控制、进度条、时间/进度/小节信息。
- **设置窗口**：颜色、布局、曲目信息、信息覆盖等全面自定义。

---

## 🎨 主题自定义

主题文件存储在 `./themes/` 目录下，格式为 TOML。例如：

```toml
name = "我的主题"
description = "自定义配色方案"

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
# ... 更多颜色字段
```

您也可以直接在设置窗口中调整颜色，无需编辑文件。

---

## 📁 项目结构

```
hesychiamids/
├── src/                 # 源代码
│   ├── audio_engine.rs  # 音频渲染与播放
│   ├── midi_parser.rs   # MIDI 解析
│   ├── playback.rs      # 时间同步
│   ├── state.rs         # 全局状态
│   ├── theme_manager.rs # 主题管理
│   └── ui/              # UI 组件
│       ├── piano_roll.rs    # 卷帘窗渲染
│       ├── settings.rs      # 设置窗口
│       └── sidebar.rs       # 侧边栏
├── themes/              # 预设主题
├── piano.sf2            # 音色库（用户提供）
├── Cargo.toml
└── fonts                # 字体（用户提供）
```

---

## 🧩 系统要求

- Windows 7 / 10 / 11（推荐） – 其他平台需要 Rust 工具链编译。
- 音频输出：支持 WASAPI / ALSA / CoreAudio 的设备。

---

## 🛠 路线图

- **v0.2.0**（当前）：可视化增强、信息覆盖、小节视图、持续高亮、更多主题。
- **v0.3.0**：MP3/MP4 导出、视频导出等功能。
- **v1.0.0**：跨平台稳定版本，支持 VST 插件。

---

## 📄 许可证

MIT License © 2024–2026 hesychiamids 贡献者

---

## 🙏 致谢

- [egui](https://github.com/emilk/egui) – 即时模式 GUI
- [midly](https://github.com/Stupremee/midly) – MIDI 解析
- [rustysynth](https://github.com/rustysynth/rustysynth) – MIDI 合成
- [cpal](https://github.com/RustAudio/cpal) – 音频输出

---

### 🔗 [源码仓库](https://github.com/GTLaimi/HesychiaMidS)

---

**享受你的音乐之旅！🎵**