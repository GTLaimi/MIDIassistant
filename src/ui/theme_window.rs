// src/ui/theme_window.rs
use eframe::egui;
use egui::Align2;
use crate::state::AppState;
use crate::theme_manager;

pub fn draw_theme_window(ctx: &egui::Context, state: &mut AppState, open: &mut bool) {
    egui::Window::new("主题管理器")
        .open(open)
        .collapsible(false)
        .resizable(true)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            let themes = theme_manager::list_available_themes();

            if themes.is_empty() {
                ui.label("未在程序运行目录下找到 `.toml` 主题文件。");
                ui.label("提示：程序会自动在当前运行目录下创建 `themes` 文件夹。");
                ui.label("请将你的 `.toml` 主题文件放入该文件夹中，然后重新打开此窗口即可。");
            } else {
                ui.label("选择预设主题 (点击即可即时预览):");
                let mut current_selection = state.active_theme_name.clone();
                for theme_name in themes {
                    if ui.selectable_label(current_selection == theme_name, theme_name.clone()).clicked() {
                        match theme_manager::apply_theme(&theme_name, &mut state.settings) {
                            Ok(_) => {
                                current_selection = theme_name.clone();
                                state.active_theme_name = theme_name;
                                ctx.request_repaint();
                            }
                            Err(e) => {
                                // 【已修改】删除 UI 红色闪烁报错，仅在终端输出错误信息
                                eprintln!("[ERROR] 主题应用错误: {}", e);
                            }
                        }
                    }
                }
            }

            ui.separator();
            ui.checkbox(&mut state.plugin_overrides_theme, "启用插件样式自动接管主题");
            if state.plugin_overrides_theme {
                ui.label("当前样式由插件管理，将忽略上述预设主题选择。");
            }
        });
}