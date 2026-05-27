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

        Self::new_with_path(data_dir)
    }

    // Test-friendly constructor
    pub fn new_with_path(data_dir: PathBuf) -> Result<Self, String> {
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

    pub fn save_config(&mut self, mut config: AutoCloseConfig) -> Result<(), String> {
        config.last_updated = chrono::Utc::now().to_rfc3339();
        self.config = config;

        self.persist()
    }

    fn persist(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, content)
            .map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }

    pub fn add_to_auto_close_list(&mut self, item: AutoCloseItem) -> Result<(), String> {
        self.config.auto_close_list.push(item);
        self.persist()
    }

    pub fn remove_from_auto_close_list(&mut self, id: &str) -> Result<(), String> {
        self.config.auto_close_list.retain(|item| item.id != id);
        self.persist()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn get_test_dir() -> PathBuf {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = env::temp_dir();
        path.push(format!("no_autostart_test_{}_{}", std::process::id(), counter));
        path
    }

    fn cleanup_test_dir(path: &PathBuf) {
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn test_auto_close_config_default_values() {
        let config = AutoCloseConfig::default();

        assert_eq!(config.version, "1.0.0");
        assert!(config.auto_close_list.is_empty());
        assert!(!config.last_updated.is_empty());
        assert_eq!(config.settings.check_interval, 0);
        assert!(config.settings.auto_run_on_login);
        assert!(config.settings.auto_close_on_start);
        assert!(config.settings.show_notification);
    }

    #[test]
    fn test_app_settings_default_values() {
        let settings = AppSettings::default();

        assert!(settings.auto_run_on_login);
        assert!(settings.auto_close_on_start);
        assert_eq!(settings.check_interval, 0);
        assert!(settings.show_notification);
    }

    #[test]
    fn test_config_manager_new_with_path_creates_config() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let result = ConfigManager::new_with_path(test_dir.clone());
        assert!(result.is_ok(), "Should create ConfigManager successfully");

        let manager = result.unwrap();
        let config = manager.get_config();

        assert_eq!(config.version, "1.0.0");
        assert!(config.auto_close_list.is_empty());

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_add_to_auto_close_list() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = ConfigManager::new_with_path(test_dir.clone()).unwrap();

        let item = AutoCloseItem {
            id: "test-id-1".to_string(),
            process_name: "test.exe".to_string(),
            executable_path: "C:\\test\\test.exe".to_string(),
            added_at: chrono::Utc::now().to_rfc3339(),
            permanent_action: None,
        };

        let result = manager.add_to_auto_close_list(item);
        assert!(result.is_ok(), "Should add item to list successfully");

        let config = manager.get_config();
        assert_eq!(config.auto_close_list.len(), 1);
        assert_eq!(config.auto_close_list[0].id, "test-id-1");
        assert_eq!(config.auto_close_list[0].process_name, "test.exe");

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_remove_from_auto_close_list() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = ConfigManager::new_with_path(test_dir.clone()).unwrap();

        // Add two items
        let item1 = AutoCloseItem {
            id: "id-1".to_string(),
            process_name: "test1.exe".to_string(),
            executable_path: "C:\\test\\test1.exe".to_string(),
            added_at: chrono::Utc::now().to_rfc3339(),
            permanent_action: None,
        };

        let item2 = AutoCloseItem {
            id: "id-2".to_string(),
            process_name: "test2.exe".to_string(),
            executable_path: "C:\\test\\test2.exe".to_string(),
            added_at: chrono::Utc::now().to_rfc3339(),
            permanent_action: None,
        };

        manager.add_to_auto_close_list(item1).unwrap();
        manager.add_to_auto_close_list(item2).unwrap();

        assert_eq!(manager.get_config().auto_close_list.len(), 2);

        // Remove first item
        let result = manager.remove_from_auto_close_list("id-1");
        assert!(result.is_ok(), "Should remove item successfully");

        let config = manager.get_config();
        assert_eq!(config.auto_close_list.len(), 1);
        assert_eq!(config.auto_close_list[0].id, "id-2");

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_remove_from_auto_close_list_nonexistent_id() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = ConfigManager::new_with_path(test_dir.clone()).unwrap();

        // Try to remove non-existent item (should not fail, just do nothing)
        let result = manager.remove_from_auto_close_list("non-existent-id");
        assert!(result.is_ok(), "Should not fail for non-existent ID");

        assert!(manager.get_config().auto_close_list.is_empty());

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_save_config_updates_last_updated() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = ConfigManager::new_with_path(test_dir.clone()).unwrap();

        let original_timestamp = manager.get_config().last_updated.clone();

        // Wait a tiny bit to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));

        let new_config = AutoCloseConfig {
            version: "2.0.0".to_string(),
            last_updated: original_timestamp.clone(),
            auto_close_list: vec![],
            settings: AppSettings::default(),
        };

        let result = manager.save_config(new_config);
        assert!(result.is_ok(), "Should save config successfully");

        let updated_config = manager.get_config();
        assert_eq!(updated_config.version, "2.0.0");
        assert!(
            updated_config.last_updated != original_timestamp,
            "last_updated should be updated to current timestamp"
        );

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_config_persistence() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        // Create manager and add item
        {
            let mut manager = ConfigManager::new_with_path(test_dir.clone()).unwrap();

            let item = AutoCloseItem {
                id: "persist-id".to_string(),
                process_name: "persist.exe".to_string(),
                executable_path: "C:\\persist\\persist.exe".to_string(),
                added_at: chrono::Utc::now().to_rfc3339(),
                permanent_action: None,
            };

            manager.add_to_auto_close_list(item).unwrap();
        }

        // Create new manager pointing to same directory - should load persisted config
        {
            let manager = ConfigManager::new_with_path(test_dir.clone()).unwrap();
            let config = manager.get_config();

            assert_eq!(config.auto_close_list.len(), 1);
            assert_eq!(config.auto_close_list[0].id, "persist-id");
        }

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_auto_close_item_creation() {
        let item = AutoCloseItem {
            id: "item-123".to_string(),
            process_name: "myapp.exe".to_string(),
            executable_path: "C:\\Program Files\\MyApp\\myapp.exe".to_string(),
            added_at: "2024-01-01T00:00:00Z".to_string(),
            permanent_action: None,
        };

        assert_eq!(item.id, "item-123");
        assert_eq!(item.process_name, "myapp.exe");
        assert_eq!(item.executable_path, "C:\\Program Files\\MyApp\\myapp.exe");
        assert_eq!(item.added_at, "2024-01-01T00:00:00Z");
        assert!(item.permanent_action.is_none());
    }

    #[test]
    fn test_auto_close_config_serialization() {
        let config = AutoCloseConfig {
            version: "1.0.0".to_string(),
            last_updated: "2024-01-01T00:00:00Z".to_string(),
            auto_close_list: vec![],
            settings: AppSettings::default(),
        };

        let json = serde_json::to_string(&config).expect("Should serialize AutoCloseConfig");
        let deserialized: AutoCloseConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.version, deserialized.version);
        assert_eq!(config.last_updated, deserialized.last_updated);
    }
}
