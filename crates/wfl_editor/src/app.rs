use crate::config::EditorConfig;
use crate::editor::TextEditor;
use crate::theme::Theme;
use egui::{Context, Ui, ViewportCommand};
use std::path::{Path, PathBuf};

#[cfg(feature = "git-integration")]
use crate::git::GitPanel;

pub struct WflEditorApp {
    config: EditorConfig,

    editor: TextEditor,

    current_file: Option<PathBuf>,

    unsaved_changes: bool,

    theme: Theme,

    show_settings: bool,

    #[cfg(feature = "git-integration")]
    git_panel: GitPanel,
}

impl WflEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>, file_path: Option<String>) -> Self {
        let config = EditorConfig::load();

        let theme = if config.dark_mode {
            Theme::dark()
        } else {
            Theme::light()
        };

        theme.apply(cc.egui_ctx.clone());

        let mut editor = TextEditor::new(cc.egui_ctx.clone(), &config);

        let current_file = file_path.map(|path| {
            let path = PathBuf::from(path);
            if path.exists() {
                editor.open_file(&path);
            }
            path
        });

        Self {
            config,
            editor,
            current_file,
            unsaved_changes: false,
            theme,
            show_settings: false,

            #[cfg(feature = "git-integration")]
            git_panel: GitPanel::new(),
        }
    }

    fn update_title(&self, ctx: &Context) {
        let mut title = "WFL Editor".to_string();

        if let Some(path) = &self.current_file {
            title.push_str(" - ");
            title.push_str(
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            );

            if self.unsaved_changes {
                title.push_str(" *");
            }
        }

        ctx.send_viewport_cmd(ViewportCommand::Title(title));
    }

    fn show_menu_bar(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("New").clicked() {
                    self.editor.clear();
                    self.current_file = None;
                    self.unsaved_changes = false;
                    ui.close_menu();
                }

                if ui.button("Open...").clicked() {
                    ui.close_menu();
                }

                ui.add_enabled_ui(self.current_file.is_some(), |ui| {
                    if ui.button("Save").clicked() {
                        if let Some(path) = &self.current_file {
                            self.editor.save_file(path);
                            self.unsaved_changes = false;
                        }
                        ui.close_menu();
                    }
                });

                if ui.button("Save As...").clicked() {
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                }
            });

            egui::menu::menu_button(ui, "Edit", |ui| {
                if ui.button("Undo").clicked() {
                    self.editor.undo();
                    ui.close_menu();
                }

                if ui.button("Redo").clicked() {
                    self.editor.redo();
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Cut").clicked() {
                    self.editor.cut();
                    ui.close_menu();
                }

                if ui.button("Copy").clicked() {
                    self.editor.copy();
                    ui.close_menu();
                }

                if ui.button("Paste").clicked() {
                    self.editor.paste();
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Format Code").clicked() {
                    self.editor.format_code();
                    ui.close_menu();
                }
            });

            egui::menu::menu_button(ui, "View", |ui| {
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                    ui.close_menu();
                }

                let dark_mode = self.config.dark_mode;
                if ui
                    .checkbox(&mut self.config.dark_mode, "Dark Mode")
                    .changed()
                {
                    if dark_mode != self.config.dark_mode {
                        self.theme = if self.config.dark_mode {
                            Theme::dark()
                        } else {
                            Theme::light()
                        };
                        self.theme.apply(ui.ctx().clone());
                    }
                }
            });

            egui::menu::menu_button(ui, "Help", |ui| {
                if ui.button("About").clicked() {
                    ui.close_menu();
                }
            });
        });
    }

    fn show_settings_panel(&mut self, ctx: &Context) {
        egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Editor Settings");

                ui.checkbox(&mut self.config.auto_save, "Auto Save");
                ui.checkbox(&mut self.config.auto_format, "Auto Format on Save");

                ui.add(egui::Slider::new(&mut self.config.font_size, 8..=32).text("Font Size"));
                ui.add(egui::Slider::new(&mut self.config.tab_width, 2..=8).text("Tab Width"));

                ui.separator();

                ui.heading("Telemetry");
                ui.checkbox(&mut self.config.telemetry_enabled, "Enable Telemetry");

                ui.separator();

                if ui.button("Save Settings").clicked() {
                    self.config.save();
                    self.show_settings = false;
                }
            });
    }
}

impl eframe::App for WflEditorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.update_title(ctx);

        if self.show_settings {
            self.show_settings_panel(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_menu_bar(ui);

            egui::SidePanel::right("git_panel")
                .resizable(true)
                .default_width(250.0)
                .show_animated(ctx, cfg!(feature = "git-integration"), |ui| {
                    #[cfg(feature = "git-integration")]
                    {
                        ui.heading("Git");

                        if let Some(path) = &self.current_file {
                            if let Some(repo_path) = self.git_panel.find_repo_root(path) {
                                match self.git_panel.update_status(&repo_path) {
                                    Ok(_) => {
                                        self.git_panel.ui(ui);
                                    }
                                    Err(e) => {
                                        ui.label(format!("Git error: {}", e));
                                        ui.label("Check your SSH agent or repository permissions.");
                                    }
                                }
                            } else {
                                ui.label("Not a git repository");
                            }
                        } else {
                            ui.label("No file open");
                        }
                    }
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                if self.editor.ui(ui) {
                    self.unsaved_changes = true;
                }

                if self.unsaved_changes && self.config.auto_save {
                    if let Some(path) = &self.current_file {
                        self.editor.save_file(path);
                        self.unsaved_changes = false;
                    }
                }
            });
        });
    }
}
