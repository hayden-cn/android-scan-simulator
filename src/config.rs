use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub action: String,
    pub broadcast_key: String,
    pub last_device: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            action: "android.intent.action.SCAN_RESULT".to_string(),
            broadcast_key: "SCAN_BARCODE".to_string(),
            last_device: String::new(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> Option<PathBuf> {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join(CONFIG_FILE)))
    }

    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };
        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Cannot determine config path")?;
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())
    }
}
