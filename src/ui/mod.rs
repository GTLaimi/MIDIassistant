// src/ui/mod.rs
mod settings;
mod theme_window;
mod note_list_window;
mod sidebar;
mod piano_roll;
mod horizontal;
mod vertical;
pub mod bar;
mod info_overlay;

pub use settings::draw_settings_window;
pub use theme_window::draw_theme_window;
pub use note_list_window::draw_note_list_window;
pub use sidebar::draw_sidebar;
pub use piano_roll::draw_piano_roll;