use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub action: String,
    pub key: String,
}

impl Profile {
    pub fn new(name: &str, action: &str, key: &str) -> Self {
        Self {
            name: name.to_string(),
            action: action.to_string(),
            key: key.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub action: String,
    pub broadcast_key: String,
    pub last_device: String,
    #[serde(default)]
    pub profiles: Vec<Profile>,
    #[serde(default)]
    pub active_profile: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let default_profile = Profile::new(
            "Default",
            "android.intent.ACTION_DECODE_DATA",
            "barcode_string",
        );
        Self {
            action: default_profile.action.clone(),
            broadcast_key: default_profile.key.clone(),
            last_device: String::new(),
            profiles: vec![default_profile],
            active_profile: "Default".to_string(),
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
            Ok(content) => {
                let mut config: AppConfig =
                    toml::from_str(&content).unwrap_or_default();
                config.migrate();
                config
            }
            Err(_) => Self::default(),
        }
    }

    fn migrate(&mut self) {
        if self.profiles.is_empty() {
            let profile = Profile::new("Default", &self.action, &self.broadcast_key);
            self.profiles.push(profile);
            self.active_profile = "Default".to_string();
        }
        if self.active_profile.is_empty() {
            self.active_profile = self
                .profiles
                .first()
                .map(|p| p.name.clone())
                .unwrap_or_default();
        }
        self.apply_active_profile();
    }

    pub fn apply_active_profile(&mut self) {
        if let Some(profile) = self.profiles.iter().find(|p| p.name == self.active_profile) {
            self.action = profile.action.clone();
            self.broadcast_key = profile.key.clone();
        }
    }

    pub fn add_profile(&mut self, name: &str, action: &str, key: &str) -> bool {
        if self.profiles.iter().any(|p| p.name == name) {
            return false;
        }
        self.profiles.push(Profile::new(name, action, key));
        self.active_profile = name.to_string();
        self.apply_active_profile();
        true
    }

    pub fn delete_profile(&mut self, index: usize) -> bool {
        if self.profiles.len() <= 1 || index >= self.profiles.len() {
            return false;
        }
        self.profiles.remove(index);
        if self.active_profile == self.profiles[index.min(self.profiles.len() - 1)].name {
            self.active_profile = self.profiles[index.min(self.profiles.len() - 1)].name.clone();
        }
        self.apply_active_profile();
        true
    }

    pub fn select_profile(&mut self, index: usize) {
        if let Some(profile) = self.profiles.get(index) {
            self.active_profile = profile.name.clone();
            self.apply_active_profile();
        }
    }

    pub fn active_profile_index(&self) -> usize {
        self.profiles
            .iter()
            .position(|p| p.name == self.active_profile)
            .unwrap_or(0)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Cannot determine config path")?;
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())
    }
}
