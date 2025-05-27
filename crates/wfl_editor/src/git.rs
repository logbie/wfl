use egui::Ui;
use git2::{Repository, Status, StatusOptions};
use std::path::{Path, PathBuf};

pub struct GitPanel {
    repo: Option<Repository>,

    status_entries: Vec<StatusEntry>,

    error: Option<String>,
}

struct StatusEntry {
    path: String,

    status: Status,

    is_staged: bool,
}

impl GitPanel {
    pub fn new() -> Self {
        Self {
            repo: None,
            status_entries: Vec::new(),
            error: None,
        }
    }

    pub fn find_repo_root(&self, path: &Path) -> Option<PathBuf> {
        let mut current = if path.is_file() {
            path.parent()?.to_path_buf()
        } else {
            path.to_path_buf()
        };

        loop {
            if current.join(".git").exists() {
                return Some(current);
            }

            if !current.pop() {
                break;
            }
        }

        None
    }

    pub fn update_status(&mut self, repo_path: &Path) -> Result<(), git2::Error> {
        match Repository::open(repo_path) {
            Ok(repo) => {
                self.repo = Some(repo);
                self.error = None;

                let repo = self.repo.as_ref().unwrap();

                let mut opts = StatusOptions::new();
                opts.include_untracked(true)
                    .recurse_untracked_dirs(true)
                    .include_ignored(false);

                let statuses = repo.statuses(Some(&mut opts))?;

                self.status_entries.clear();

                for entry in statuses.iter() {
                    let path = entry.path().unwrap_or("").to_string();
                    let status = entry.status();

                    let is_staged = status.is_index_new()
                        || status.is_index_modified()
                        || status.is_index_deleted()
                        || status.is_index_renamed()
                        || status.is_index_typechange();

                    self.status_entries.push(StatusEntry {
                        path,
                        status,
                        is_staged,
                    });
                }

                Ok(())
            }
            Err(e) => {
                self.repo = None;
                self.status_entries.clear();
                self.error = Some(e.to_string());
                Err(e)
            }
        }
    }

    pub fn ui(&self, ui: &mut Ui) {
        if let Some(error) = &self.error {
            ui.label(format!("Git error: {}", error));
            ui.label("Check your SSH agent or repository permissions.");
            return;
        }

        if self.status_entries.is_empty() {
            ui.label("No changes");
            return;
        }

        ui.collapsing("Staged", |ui| {
            let staged = self.status_entries.iter().filter(|e| e.is_staged);
            for entry in staged {
                let status_text = self.status_text(&entry.status, true);
                ui.label(format!("{} {}", status_text, entry.path));
            }
        });

        ui.collapsing("Unstaged", |ui| {
            let unstaged = self.status_entries.iter().filter(|e| !e.is_staged);
            for entry in unstaged {
                let status_text = self.status_text(&entry.status, false);
                ui.label(format!("{} {}", status_text, entry.path));
            }
        });
    }

    fn status_text(&self, status: &Status, staged: bool) -> &'static str {
        if staged {
            if status.is_index_new() {
                "A"
            } else if status.is_index_modified() {
                "M"
            } else if status.is_index_deleted() {
                "D"
            } else if status.is_index_renamed() {
                "R"
            } else if status.is_index_typechange() {
                "T"
            } else {
                " "
            }
        } else {
            if status.is_wt_new() {
                "?"
            } else if status.is_wt_modified() {
                "M"
            } else if status.is_wt_deleted() {
                "D"
            } else if status.is_wt_renamed() {
                "R"
            } else if status.is_wt_typechange() {
                "T"
            } else {
                " "
            }
        }
    }
}
