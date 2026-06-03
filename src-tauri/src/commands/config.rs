use crate::error::{AppError, AppResult};
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
) -> AppResult<String> {
    let mut manager = state.0.lock().unwrap();
    manager.save_config(config).map_err(|e| AppError::ConfigWriteFailed(e))?;
    Ok("Config saved successfully".to_string())
}

#[tauri::command]
pub fn add_to_auto_close_list(
    item: AutoCloseItem,
    state: State<ConfigManagerState>,
) -> AppResult<String> {
    let mut manager = state.0.lock().unwrap();
    manager.add_to_auto_close_list(item).map_err(|e| AppError::ConfigWriteFailed(e))?;
    Ok("Added to auto close list".to_string())
}

#[tauri::command]
pub fn remove_from_auto_close_list(
    id: String,
    state: State<ConfigManagerState>,
) -> AppResult<String> {
    let mut manager = state.0.lock().unwrap();
    manager.remove_from_auto_close_list(&id).map_err(|e| AppError::ConfigWriteFailed(e))?;
    Ok("Removed from auto close list".to_string())
}

#[cfg(test)]
mod tests {
    // Note: Integration tests that require Tauri State are removed.
    // These tests cannot run outside of the Tauri runtime context.
    // See tests in modules/config_manager.rs for unit tests.
}
