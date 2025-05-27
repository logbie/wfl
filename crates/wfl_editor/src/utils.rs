use std::path::{Path, PathBuf};

pub fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string())
}

pub fn parent_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}
