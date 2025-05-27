pub mod config;
pub mod version;

#[cfg(feature = "git-integration")]
pub mod git;

#[cfg(feature = "lsp-bridge")]
pub mod lsp;

#[cfg(all(feature = "gui-full", not(headless)))]
mod app;
#[cfg(all(feature = "gui-full", not(headless)))]
mod editor;
#[cfg(all(feature = "gui-full", not(headless)))]
mod syntax;
#[cfg(all(feature = "gui-full", not(headless)))]
mod theme;
#[cfg(all(feature = "gui-full", not(headless)))]
mod ui;
#[cfg(all(feature = "gui-full", not(headless)))]
mod utils;

pub use crate::version::VERSION;
pub use crate::version::full_version;

pub use crate::config::EditorConfig;

#[cfg(all(feature = "gui-full", not(headless)))]
pub use app::WflEditorApp;

#[cfg(feature = "gui")]
pub fn launch(_file_path: Option<String>) -> Result<(), String> {
    #[cfg(any(headless, not(feature = "gui-full")))]
    {
        return Err("GUI is not available in headless environments or without gui-full feature. Use the LSP server instead.".to_string());
    }

    #[cfg(all(feature = "gui-full", not(headless)))]
    {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(1280.0, 720.0)),
            min_window_size: Some(egui::vec2(400.0, 300.0)),
            ..Default::default()
        };

        eframe::run_native(
            "WFL Editor",
            options,
            Box::new(|cc| Box::new(WflEditorApp::new(cc, file_path))),
        )
        .map_err(|e| format!("Failed to launch editor: {}", e))
    }
}
