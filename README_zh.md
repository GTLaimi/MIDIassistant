# 🎹 MIDIassistant

> 轻量级、高精度的 MIDI 播放与可视化工具

![版本](https://img.shields.io/badge/version-v0.1.5-blue)
![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange)
![许可](https://img.shields.io/badge/license-MIT-green)

## ✨ 特性

- **🎵 MIDI 解析与播放**：支持标准 MIDI 文件（.mid/.midi），精确解析音符、速度、拍号等元数据
- **🎨 双布局卷帘窗**：
  - **水平模式**：经典键盘左侧布局，时间轴水平向右
  - **垂直瀑布流模式**：键盘底部，时间轴垂直向上流动，适合观察音符时值
- **🎚 可调节钢琴键盘**：拖拽分隔条自由调节键盘大小，最窄可完全隐藏
- **🎛 高音质音频引擎**：基于 rustysynth + cpal，支持混响/合唱，采样率自适应设备最高品质
- **🎨 主题系统**：内置7款精美主题，支持自定义主题文件（TOML 格式）
- **🎯 精准进度同步**：采用 f64 时间计算，长 MIDI 也无节奏偏移
- **📊 多种显示模式**：时间、进度百分比、音符计数、小节拍号等一键切换

## 🚀 快速开始

### 环境要求
- Rust 1.97+ (如需从源码编译)
- 音色库文件 `piano.sf2` (放置于程序根目录)

### 安装

**从源码编译**
```bash
git clone https://github.com/yourusername/MIDIassistant.git
cd MIDIassistant
cargo build --release
```
**下载预编译版本**
请访问 Releases 页面 下载对应平台安装包。

### 使用

1. 将 piano.sf2 放入程序根目录
2. 启动程序，点击 "加载 MIDI 文件" 选择 .mid 文件
3. 使用空格键播放/暂停，R 键重置
4. 在侧边栏切换水平/垂直布局
5. 拖拽键盘边缘分隔条调节大小

### 🎨 主题定制

主题文件位于 ./themes/ 目录，格式为 TOML。示例：
```toml
name = "我的主题"
description = "自定义配色"

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
### 📁 项目结构

```text
MIDIassistant/
├── src/                 # 源代码
│   ├── audio_engine.rs  # 音频引擎（渲染与播放）
│   ├── midi_parser.rs   # MIDI 解析
│   ├── playback.rs      # 时间同步
│   ├── state.rs         # 全局状态
│   ├── theme_manager.rs # 主题管理
│   └── ui/              # 界面组件
│       ├── piano_roll.rs    # 卷帘窗
│       ├── settings.rs      # 设置窗口
│       └── sidebar.rs       # 侧边栏
├── themes/              # 预设主题
├── piano.sf2            # 音色库（需自行准备）
└── Cargo.toml
```
### 🛠 开发路线

- v0.2.0：MIDI 编辑功能（音符拖拽、移动、拉伸）
- v0.3.0：高质量导出（MP3/MP4），VST 插件支持
- v1.0.0：跨平台稳定版发布

### 📄 许可

MIT License © 2024 MIDIassistant Contributors

