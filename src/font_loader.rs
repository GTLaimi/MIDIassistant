// src/font_loader.rs
use std::sync::OnceLock;
use eframe::egui;

/// 全局静态变量：存储所有可用字体名称（包括内置的 "proportional" 和 "monospace"）
pub static FONT_NAMES: OnceLock<Vec<String>> = OnceLock::new();

pub fn load_fonts() -> egui::FontDefinitions {
    let mut font_definitions = egui::FontDefinitions::default();

    // ===== 1. 添加系统中文后备字体（仅 Windows） =====
    #[cfg(target_os = "windows")]
    {
        let chinese_font_path = "C:/Windows/Fonts/msyh.ttc";
        if let Ok(data) = std::fs::read(chinese_font_path) {
            // 将微软雅黑添加到字体数据
            font_definitions
                .font_data
                .insert("msyh".to_owned(), egui::FontData::from_owned(data));

            // 将微软雅黑添加到 Proportional 和 Monospace 族（放在最前面，优先使用）
            if let Some(proportional) = font_definitions
                .families
                .get_mut(&egui::FontFamily::Proportional)
            {
                proportional.insert(0, "msyh".to_owned());
            }
            if let Some(monospace) = font_definitions
                .families
                .get_mut(&egui::FontFamily::Monospace)
            {
                monospace.insert(0, "msyh".to_owned());
            }
        }
    }

    // ===== 2. 扫描 ./fonts/ 目录，加载自定义字体 =====
    let mut font_names = vec!["proportional".to_string(), "monospace".to_string()];
    if let Ok(entries) = std::fs::read_dir("./fonts") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "ttf" || ext == "otf" {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        let name = name.to_string();
                        if let Ok(data) = std::fs::read(&path) {
                            // 插入自定义字体数据（作为独立族）
                            font_definitions
                                .font_data
                                .insert(name.clone(), egui::FontData::from_owned(data));
                            // 创建一个同名 FontFamily，并将该字体添加到该族中
                            font_definitions
                                .families
                                .entry(egui::FontFamily::Name(name.clone().into()))
                                .or_default()
                                .push(name.clone());
                            font_names.push(name);
                        }
                    }
                }
            }
        }
    }

    // 将字体名称列表存入全局静态变量（供设置界面使用）
    let _ = FONT_NAMES.set(font_names);

    font_definitions
}

/// 获取所有可用字体名称列表（包括内置的 "proportional" 和 "monospace"）
pub fn get_available_font_names() -> Vec<String> {
    FONT_NAMES
        .get()
        .cloned()
        .unwrap_or_else(|| vec!["proportional".to_string(), "monospace".to_string()])
}