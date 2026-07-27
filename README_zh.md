# MIDIassistant

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-v0.1.4-blue)

> 一款轻量级、高精度的 Rust 桌面 MIDI 钢琴卷帘窗可视化工具。具备极强的视觉自定义能力，适合音乐创作者和音频工程师。

## ✨ v0.1.4 核心特性

- **MIDI 文件加载**：通过侧边栏直接打开任意 `.mid` 文件。
- **物理时间同步**：精准的 BPM/PPQ 音频引擎，播放指针平滑跟随。
- **钢琴卷帘窗**：支持动态音名标注，以及全自定义色彩体系。
- **终极自定义引擎**：
  - **设置面板重构**：分为“颜色”、“布局”、“关于”三个标签页，操作更清晰。
  - **全 RGB 色彩定制**：可独立调节背景、白键、黑键、常规音符、高亮音符、指针、文字、网格亮线/暗线。
  - **主题管理器与生态系统**：一键应用 `themes` 文件夹下的 `.toml` 主题预设，或将当前调好的颜色保存为新主题。
  - **布局与颜色解耦**：应用主题时 **只会改变颜色和网格**，你调好的音域缩放、水平缩放等布局参数会完美保留！
- **布局与缩放**：
  - 垂直音高平移与垂直缩放。
  - 水平缩放 (0.05 ~ 0.9)。
  - 播放头自动跟随模式切换。
- **极简走带区**：播放/暂停、重置按键，以及精确无跳动的可拖拽进度条。
- **音符列表分析器**：独立窗口展示所有 MIDI 音符的序号、音高名称、时长、力度等信息，并支持一键导出为 `.txt` 文本文件。

## 🚀 编译与运行

```bash
# 克隆仓库
git clone https://github.com/GTLaimi/MIDIassistant.git
cd MIDIassistant

# 编译发布版本
cargo build --release

# 运行
cargo run --release
```
(Windows 平台的预编译 .exe 程序可在 Releases 页面直接下载！)

## 📝 许可证

Licensed under the MIT license.
