use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::state::VisualSettings;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeFile {
    pub name: String,
    pub description: String,
    pub settings: VisualSettings,
}

pub fn get_themes_folder() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|mut path| {
            path.pop();
            path.push("themes");
            Some(path)
        })
        .unwrap_or_else(|| PathBuf::from("themes"))
}

pub fn list_available_themes() -> Vec<String> {
    let folder = get_themes_folder();
    if !folder.exists() {
        if let Err(e) = fs::create_dir_all(&folder) {
            eprintln!("创建主题文件夹失败: {}", e);
            return vec![];
        }
    }

    let mut themes = Vec::new();
    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("toml") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    themes.push(file_stem.to_string());
                }
            }
        }
    }
    themes
}

// ============================================================
// 核心修复：手动解析 toml::Value，允许主题文件只包含颜色字段
// ============================================================
pub fn apply_theme(theme_name: &str, settings: &mut VisualSettings) -> Result<(), String> {
    let mut path = get_themes_folder();
    path.push(theme_name);
    path.set_extension("toml");

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取主题文件失败: {}", e))?;

    // 解析为通用的 toml::Value，避免强类型反序列化时的字段缺失错误
    let value: toml::Value = toml::from_str(&content)
        .map_err(|e| format!("解析主题文件失败: {}", e))?;

    // 获取 [settings] 表
    let settings_table = value.get("settings")
        .and_then(|v| v.as_table())
        .ok_or("主题文件中缺少 [settings] 部分")?;

    // 辅助函数：从表中提取 u8 值
    fn get_u8(table: &toml::value::Table, key: &str) -> Option<u8> {
        table.get(key).and_then(|v| v.as_integer()).map(|i| i as u8)
    }

    // 只复制颜色与网格字段，缺失的字段保持当前值不变
    // ---- 背景色 ----
    if let Some(v) = get_u8(settings_table, "piano_bg_r") { settings.piano_bg_r = v; }
    if let Some(v) = get_u8(settings_table, "piano_bg_g") { settings.piano_bg_g = v; }
    if let Some(v) = get_u8(settings_table, "piano_bg_b") { settings.piano_bg_b = v; }

    // ---- 键盘颜色 ----
    if let Some(v) = get_u8(settings_table, "white_key_r") { settings.white_key_r = v; }
    if let Some(v) = get_u8(settings_table, "white_key_g") { settings.white_key_g = v; }
    if let Some(v) = get_u8(settings_table, "white_key_b") { settings.white_key_b = v; }

    if let Some(v) = get_u8(settings_table, "black_key_r") { settings.black_key_r = v; }
    if let Some(v) = get_u8(settings_table, "black_key_g") { settings.black_key_g = v; }
    if let Some(v) = get_u8(settings_table, "black_key_b") { settings.black_key_b = v; }

    // ---- 音符颜色 ----
    if let Some(v) = get_u8(settings_table, "note_r") { settings.note_r = v; }
    if let Some(v) = get_u8(settings_table, "note_g") { settings.note_g = v; }
    if let Some(v) = get_u8(settings_table, "note_b") { settings.note_b = v; }

    if let Some(v) = get_u8(settings_table, "active_note_r") { settings.active_note_r = v; }
    if let Some(v) = get_u8(settings_table, "active_note_g") { settings.active_note_g = v; }
    if let Some(v) = get_u8(settings_table, "active_note_b") { settings.active_note_b = v; }

    // ---- 文字颜色 ----
    if let Some(v) = get_u8(settings_table, "text_r") { settings.text_r = v; }
    if let Some(v) = get_u8(settings_table, "text_g") { settings.text_g = v; }
    if let Some(v) = get_u8(settings_table, "text_b") { settings.text_b = v; }

    // ---- 指针颜色 ----
    if let Some(v) = get_u8(settings_table, "cursor_r") { settings.cursor_r = v; }
    if let Some(v) = get_u8(settings_table, "cursor_g") { settings.cursor_g = v; }
    if let Some(v) = get_u8(settings_table, "cursor_b") { settings.cursor_b = v; }

    // ---- 网格颜色 ----
    if let Some(v) = get_u8(settings_table, "grid_bright_r") { settings.grid_bright_r = v; }
    if let Some(v) = get_u8(settings_table, "grid_bright_g") { settings.grid_bright_g = v; }
    if let Some(v) = get_u8(settings_table, "grid_bright_b") { settings.grid_bright_b = v; }

    if let Some(v) = get_u8(settings_table, "grid_dim_r") { settings.grid_dim_r = v; }
    if let Some(v) = get_u8(settings_table, "grid_dim_g") { settings.grid_dim_g = v; }
    if let Some(v) = get_u8(settings_table, "grid_dim_b") { settings.grid_dim_b = v; }

    // 注意：我们故意不处理 pitch_min, pitch_range, time_offset, time_zoom,
    // follow_playhead, show_cursor, enable_audio 等布局设置，
    // 因为 apply_theme 的职责是只应用颜色和网格，保留用户自己的布局偏好。
    // 主题文件中若包含这些字段，会被安全忽略。

    Ok(())
}

pub fn save_custom_theme(theme_name: &str, settings: &VisualSettings) -> Result<(), String> {
    let theme = ThemeFile {
        name: theme_name.to_string(),
        description: "由用户自定义创建".to_string(),
        settings: settings.clone(),
    };
    let toml_str = toml::to_string(&theme)
        .map_err(|e| format!("序列化主题失败: {}", e))?;

    let mut path = get_themes_folder();
    path.push(theme_name);
    path.set_extension("toml");

    fs::write(&path, toml_str).map_err(|e| format!("写入主题文件失败: {}", e))?;
    Ok(())
}