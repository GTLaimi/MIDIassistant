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

// 【核心修复】只更新颜色和网格，保留布局设置
pub fn apply_theme(theme_name: &str, settings: &mut VisualSettings) -> Result<(), String> {
    let mut path = get_themes_folder();
    path.push(theme_name);
    path.set_extension("toml");

    let content = fs::read_to_string(&path).map_err(|e| format!("读取主题文件失败: {}", e))?;
    let theme: ThemeFile = toml::from_str(&content).map_err(|e| format!("解析主题文件失败: {}", e))?;
    
    // 只更新颜色与网格，完全不碰布局参数
    settings.piano_bg_r = theme.settings.piano_bg_r;
    settings.piano_bg_g = theme.settings.piano_bg_g;
    settings.piano_bg_b = theme.settings.piano_bg_b;

    settings.white_key_r = theme.settings.white_key_r;
    settings.white_key_g = theme.settings.white_key_g;
    settings.white_key_b = theme.settings.white_key_b;

    settings.black_key_r = theme.settings.black_key_r;
    settings.black_key_g = theme.settings.black_key_g;
    settings.black_key_b = theme.settings.black_key_b;

    settings.note_r = theme.settings.note_r;
    settings.note_g = theme.settings.note_g;
    settings.note_b = theme.settings.note_b;

    settings.active_note_r = theme.settings.active_note_r;
    settings.active_note_g = theme.settings.active_note_g;
    settings.active_note_b = theme.settings.active_note_b;

    settings.text_r = theme.settings.text_r;
    settings.text_g = theme.settings.text_g;
    settings.text_b = theme.settings.text_b;

    settings.cursor_r = theme.settings.cursor_r;
    settings.cursor_g = theme.settings.cursor_g;
    settings.cursor_b = theme.settings.cursor_b;

    settings.grid_bright_r = theme.settings.grid_bright_r;
    settings.grid_bright_g = theme.settings.grid_bright_g;
    settings.grid_bright_b = theme.settings.grid_bright_b;

    settings.grid_dim_r = theme.settings.grid_dim_r;
    settings.grid_dim_g = theme.settings.grid_dim_g;
    settings.grid_dim_b = theme.settings.grid_dim_b;

    Ok(())
}

pub fn save_custom_theme(theme_name: &str, settings: &VisualSettings) -> Result<(), String> {
    let theme = ThemeFile {
        name: theme_name.to_string(),
        description: "由用户自定义创建".to_string(),
        settings: settings.clone(),
    };
    let toml_str = toml::to_string(&theme).map_err(|e| format!("序列化主题失败: {}", e))?;
    
    let mut path = get_themes_folder();
    path.push(theme_name);
    path.set_extension("toml");

    fs::write(&path, toml_str).map_err(|e| format!("写入主题文件失败: {}", e))?;
    Ok(())
}