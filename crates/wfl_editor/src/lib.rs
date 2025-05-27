
mod app;
mod config;
mod editor;
mod syntax;
mod theme;
mod ui;
mod utils;

#[cfg(feature = "git-integration")]
mod git;

#[cfg(feature = "lsp-bridge")]
pub mod lsp;

pub use app::WflEditorApp;
pub use config::EditorConfig;

pub fn launch(file_path: Option<String>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        min_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "WFL Editor",
        options,
        Box::new(|cc| Box::new(WflEditorApp::new(cc, file_path)))
    )
}
