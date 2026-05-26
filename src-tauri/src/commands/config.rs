use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub auto_close_list: Vec<String>,
    pub start_with_windows: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_close_list: vec![],
            start_with_windows: false,
        }
    }
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    // Placeholder - will be implemented in Task 9
    AppConfig::default()
}

#[tauri::command]
pub fn save_config(config: AppConfig) -> Result<(), String> {
    // Placeholder - will be implemented in Task 9
    let _ = config;
    Err("Save config not yet implemented".to_string())
}

#[tauri::command]
pub fn add_to_auto_close_list(process_name: String) -> Result<(), String> {
    // Placeholder - will be implemented in Task 9
    let _ = process_name;
    Err("Add to auto close list not yet implemented".to_string())
}

#[tauri::command]
pub fn remove_from_auto_close_list(process_name: String) -> Result<(), String> {
    // Placeholder - will be implemented in Task 9
    let _ = process_name;
    Err("Remove from auto close list not yet implemented".to_string())
}
