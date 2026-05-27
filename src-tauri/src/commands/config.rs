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

#[cfg(test)]
mod tests {
    // Note: Integration tests that require Tauri State are removed.
    // These tests cannot run outside of the Tauri runtime context.
    // See tests in modules/config_manager.rs for unit tests.
}
