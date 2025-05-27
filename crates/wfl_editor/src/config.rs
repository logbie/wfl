#[cfg(feature = "gui")]
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[cfg_attr(feature = "gui", derive(Clone, Debug, Serialize, Deserialize))]
#[cfg_attr(not(feature = "gui"), derive(Clone, Debug))]
pub struct EditorConfig {
    pub font_size: u32,

    pub tab_width: u32,

    pub dark_mode: bool,

    pub auto_save: bool,

    pub auto_format: bool,

    pub telemetry_enabled: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font_size: 14,
            tab_width: 4,
            dark_mode: true,
            auto_save: true,
            auto_format: true,
            telemetry_enabled: false,
        }
    }
}

impl EditorConfig {
    pub fn load() -> Self {
        let mut config = if let Some(config) = Self::load_from_project() {
            config
        } else if let Some(config) = Self::load_from_user_config() {
            config
        } else {
            Self::default()
        };

        if let Ok(telemetry_env) = std::env::var("WFL_EDITOR_TELEMETRY") {
            config.telemetry_enabled = match telemetry_env.as_str() {
                "1" | "true" | "yes" | "on" => true,
                "0" | "false" | "no" | "off" => false,
                _ => config.telemetry_enabled,
            };
        }

        config
    }

    fn load_from_project() -> Option<Self> {
        let config_path = Path::new("wfl-editor.toml");
        if config_path.exists() {
            return Self::load_from_file(config_path);
        }
        None
    }

    fn load_from_user_config() -> Option<Self> {
        #[cfg(feature = "gui")]
        {
            if let Some(config_dir) = dirs::config_dir() {
                let config_path = config_dir.join("wfl").join("editor.toml");
                return Self::load_from_file(&config_path);
            }
        }
        None
    }

    fn load_from_file(path: &Path) -> Option<Self> {
        match fs::read_to_string(path) {
            #[cfg(feature = "gui")]
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => Some(config),
                Err(_) => None,
            },
            #[cfg(not(feature = "gui"))]
            Ok(_) => Some(Self::default()),
            Err(_) => None,
        }
    }

    pub fn save(&self) -> bool {
        let config_path = Path::new("wfl-editor.toml");
        if config_path.exists() {
            return self.save_to_file(config_path);
        }

        #[cfg(feature = "gui")]
        {
            if let Some(config_dir) = dirs::config_dir() {
                let wfl_config_dir = config_dir.join("wfl");
                if !wfl_config_dir.exists() {
                    if let Err(_) = fs::create_dir_all(&wfl_config_dir) {
                        return false;
                    }
                }

                let config_path = wfl_config_dir.join("editor.toml");
                return self.save_to_file(&config_path);
            }
        }

        false
    }

    fn save_to_file(&self, path: &Path) -> bool {
        #[cfg(feature = "gui")]
        {
            match toml::to_string(self) {
                Ok(content) => match fs::write(path, content) {
                    Ok(_) => true,
                    Err(_) => false,
                },
                Err(_) => false,
            }
        }

        #[cfg(not(feature = "gui"))]
        {
            match fs::write(path, "# WFL Editor config\n# This is a placeholder file\n") {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }
}
