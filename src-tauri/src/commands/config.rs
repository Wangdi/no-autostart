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
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn get_test_dir() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("no_autostart_cmd_test_{}", std::process::id()));
        path
    }

    fn cleanup_test_dir(path: &PathBuf) {
        let _ = fs::remove_dir_all(path);
    }

    fn create_test_manager() -> ConfigManager {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);
        ConfigManager::new_with_path(test_dir).unwrap()
    }

    #[test]
    fn test_get_config_returns_current_config() {
        let manager = create_test_manager();
        let state = ConfigManagerState(Mutex::new(manager));

        let config = get_config(tauri::State::from(&state));

        assert_eq!(config.version, "1.0.0");
        assert!(config.auto_close_list.is_empty());
    }

    #[test]
    fn test_save_config_persists_and_returns_success() {
        let manager = create_test_manager();
        let state = ConfigManagerState(Mutex::new(manager));

        let new_config = AutoCloseConfig {
            version: "2.0.0".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
            auto_close_list: vec![],
            settings: crate::modules::config_manager::AppSettings::default(),
        };

        let result = save_config(new_config, tauri::State::from(&state));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Config saved successfully");

        // Verify config was updated
        let updated_config = get_config(tauri::State::from(&state));
        assert_eq!(updated_config.version, "2.0.0");
    }

    #[test]
    fn test_add_to_auto_close_list_adds_item() {
        let manager = create_test_manager();
        let state = ConfigManagerState(Mutex::new(manager));

        let item = AutoCloseItem {
            id: "test-123".to_string(),
            process_name: "test.exe".to_string(),
            executable_path: "C:\\test\\test.exe".to_string(),
            added_at: chrono::Utc::now().to_rfc3339(),
            permanent_action: None,
        };

        let result = add_to_auto_close_list(item, tauri::State::from(&state));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Added to auto close list");

        // Verify item was added
        let config = get_config(tauri::State::from(&state));
        assert_eq!(config.auto_close_list.len(), 1);
        assert_eq!(config.auto_close_list[0].id, "test-123");
    }

    #[test]
    fn test_remove_from_auto_close_list_removes_item() {
        let manager = create_test_manager();
        let state = ConfigManagerState(Mutex::new(manager));

        // Add an item first
        let item = AutoCloseItem {
            id: "remove-123".to_string(),
            process_name: "remove.exe".to_string(),
            executable_path: "C:\\test\\remove.exe".to_string(),
            added_at: chrono::Utc::now().to_rfc3339(),
            permanent_action: None,
        };

        add_to_auto_close_list(item, tauri::State::from(&state)).unwrap();

        // Now remove it
        let result = remove_from_auto_close_list("remove-123".to_string(), tauri::State::from(&state));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Removed from auto close list");

        // Verify item was removed
        let config = get_config(tauri::State::from(&state));
        assert!(config.auto_close_list.is_empty());
    }

    #[test]
    fn test_remove_from_auto_close_list_nonexistent_returns_success() {
        let manager = create_test_manager();
        let state = ConfigManagerState(Mutex::new(manager));

        // Try to remove non-existent item - should not fail
        let result = remove_from_auto_close_list("nonexistent-999".to_string(), tauri::State::from(&state));

        // The manager's remove_from_auto_close_list doesn't error for non-existent IDs
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Removed from auto close list");
    }

    #[test]
    fn test_multiple_adds_and_removes() {
        let manager = create_test_manager();
        let state = ConfigManagerState(Mutex::from(manager));

        // Add multiple items
        for i in 0..3 {
            let item = AutoCloseItem {
                id: format!("item-{}", i),
                process_name: format!("process{}.exe", i),
                executable_path: format!("C:\\\\test\\\\process{}.exe", i),
                added_at: chrono::Utc::now().to_rfc3339(),
                permanent_action: None,
            };
            add_to_auto_close_list(item, tauri::State::from(&state)).unwrap();
        }

        // Verify all added
        let config = get_config(tauri::State::from(&state));
        assert_eq!(config.auto_close_list.len(), 3);

        // Remove middle item
        remove_from_auto_close_list("item-1".to_string(), tauri::State::from(&state)).unwrap();

        let config = get_config(tauri::State::from(&state));
        assert_eq!(config.auto_close_list.len(), 2);
        assert!(!config.auto_close_list.iter().any(|i| i.id == "item-1"));
    }
}
