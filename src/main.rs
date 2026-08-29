#![forbid(unsafe_code)]
#![deny(dead_code)]
#![deny(warnings)]

mod midi_parser;
mod models;
mod playback;
mod state;
mod theme_manager;
mod ui;
mod audio_engine;
mod app;
mod font_loader;
mod chord_detector;

use eframe::egui;
use app::MidiApp;
use app::handlers::handle_input;
use app::state_management::{update_playback, update_rendering, collect_state_snapshot};
use app::render::{render_ui, compute_info_data};

impl eframe::App for MidiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut requested_load_file = None;

        // 1. 处理键盘输入
        {
            let mut state = self.state.lock().unwrap();
            let mut engine = self.engine.lock().unwrap();
            handle_input(ctx, &mut state, &mut engine);
        }

        // 2. 更新播放状态
        update_playback(self, ctx);

        // 3. 更新渲染进度
        update_rendering(self, ctx);

        // 4. 收集状态快照
        let (_file_loaded, _file_name, _load_error, notes, total_ticks, current_tick, is_playing, _display_mode, time_sig, bpm, ppq, fullscreen) =
            collect_state_snapshot(self);

        // 5. 计算信息数据
        let (active_note_names, time_str, bar_beat_str) =
            compute_info_data(&notes, current_tick, bpm, ppq, &time_sig);

        // 6. 渲染 UI
        render_ui(
            self,
            ctx,
            &mut requested_load_file,
            &notes,
            total_ticks,
            current_tick,
            is_playing,
            &time_sig,
            ppq,
            fullscreen,
            bpm,
            &active_note_names,
            &bar_beat_str,
            &time_str,
        );

        // 7. 处理文件加载请求
        if let Some(path) = requested_load_file {
            self.load_file(path);
            ctx.request_repaint();
        }
    }
}

fn main() -> eframe::Result<()> {
    // 加载自定义字体（从 ./fonts/ 目录），没有目录也安全
    let font_defs = font_loader::load_fonts();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("hesychiamids"),
        ..Default::default()
    };

    eframe::run_native("hesychiamids", options, Box::new(|cc| {
        cc.egui_ctx.set_fonts(font_defs);
        Ok(Box::new(MidiApp::new()))
    }))
}