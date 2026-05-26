use crate::modules::config_manager::{AutoCloseConfig, AutoCloseItem, ConfigManager};
use std::sync::Mutex;
use tauri::State;

pub struct ConfigManagerState(pub Mutex<ConfigManager>);

#[tauri::command]
pub fn get_config(state: State<ConfigManagerState>) -> AutoCloseConfig {
    let manager = state.0.lock().unwrap();
    manager.get_config().clone()
}

#[tauri::command]
pub fn save_config(
    config: AutoCloseConfig,
    state: State<ConfigManagerState>,
) -> Result<String, String> {
    let mut manager = state.0.lock().unwrap();
    manager.save_config(config)?;
    Ok("Config saved successfully".to_string())
}

#[tauri::command]
pub fn add_to_auto_close_list(
    item: AutoCloseItem,
    state: State<ConfigManagerState>,
) -> Result<String, String> {
    let mut manager = state.0.lock().unwrap();
    manager.add_to_auto_close_list(item)?;
    Ok("Added to auto close list".to_string())
}

#[tauri::command]
pub fn remove_from_auto_close_list(
    id: String,
    state: State<ConfigManagerState>,
) -> Result<String, String> {
    let mut manager = state.0.lock().unwrap();
    manager.remove_from_auto_close_list(&id)?;
    Ok("Removed from auto close list".to_string())
}
