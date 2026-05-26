use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCloseItem {
    pub id: String,
    pub process_name: String,
    pub executable_path: String,
    pub added_at: String,
    pub permanent_action: Option<PermanentAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub description: String,
    pub executed_at: String,
    pub original_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_run_on_login: bool,
    pub auto_close_on_start: bool,
    pub check_interval: u64,
    pub show_notification: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_run_on_login: true,
            auto_close_on_start: true,
            check_interval: 0,
            show_notification: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCloseConfig {
    pub version: String,
    pub last_updated: String,
    pub auto_close_list: Vec<AutoCloseItem>,
    pub settings: AppSettings,
}

impl Default for AutoCloseConfig {
    fn default() -> Self {
        Self {
            version: String::from("1.0.0"),
            last_updated: chrono::Utc::now().to_rfc3339(),
            auto_close_list: Vec::new(),
            settings: AppSettings::default(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: AutoCloseConfig,
}

impl ConfigManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data dir: {}", e))?;

        let config_path = data_dir.join("config.json");

        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config: {}", e))?;
            serde_json::from_str(&content).unwrap_or_else(|_| AutoCloseConfig::default())
        } else {
            AutoCloseConfig::default()
        };

        Ok(Self {
            config_path,
            config,
        })
    }

    pub fn get_config(&self) -> &AutoCloseConfig {
        &self.config
    }

    pub fn save_config(&mut self, config: AutoCloseConfig) -> Result<(), String> {
        self.config = config;
        self.config.last_updated = chrono::Utc::now().to_rfc3339();

        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, content).map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }

    pub fn add_to_auto_close_list(&mut self, item: AutoCloseItem) -> Result<(), String> {
        self.config.auto_close_list.push(item);
        self.save_config(self.config.clone())
    }

    pub fn remove_from_auto_close_list(&mut self, id: &str) -> Result<(), String> {
        self.config.auto_close_list.retain(|item| item.id != id);
        self.save_config(self.config.clone())
    }
}
