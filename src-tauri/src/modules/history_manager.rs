use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub executable_path: String,
    pub startup_type: String,
    pub startup_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentActionBackup {
    #[serde(rename = "type")]
    pub action_type: String,
    pub backup_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: String,
    pub timestamp: String,
    pub operation_type: String,
    pub process_snapshot: ProcessSnapshot,
    pub permanent_action: Option<PermanentActionBackup>,
    pub status: String,
    pub reverted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationHistory {
    pub records: Vec<HistoryRecord>,
}

pub struct HistoryManager {
    history_path: PathBuf,
    history: OperationHistory,
}

impl HistoryManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        Self::new_with_path(data_dir)
    }

    // Test-friendly constructor
    pub fn new_with_path(data_dir: PathBuf) -> Result<Self, String> {
        let history_dir = data_dir.join("history");
        fs::create_dir_all(&history_dir)
            .map_err(|e| format!("Failed to create history dir: {}", e))?;

        let history_path = history_dir.join("history.json");

        let history = if history_path.exists() {
            let content = fs::read_to_string(&history_path)
                .map_err(|e| format!("Failed to read history: {}", e))?;
            serde_json::from_str(&content).unwrap_or_else(|_| OperationHistory::default())
        } else {
            OperationHistory::default()
        };

        Ok(Self {
            history_path,
            history,
        })
    }

    pub fn get_history(&self) -> &OperationHistory {
        &self.history
    }

    pub fn add_record(&mut self, mut record: HistoryRecord) -> Result<(), String> {
        if record.id.is_empty() {
            record.id = Uuid::new_v4().to_string();
        }
        if record.timestamp.is_empty() {
            record.timestamp = Utc::now().to_rfc3339();
        }
        self.history.records.insert(0, record);
        self.save()
    }

    pub fn revert_operation(&mut self, id: &str) -> Result<HistoryRecord, String> {
        let record = self
            .history
            .records
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Record not found")?;

        if record.status == "reverted" {
            return Err("Operation already reverted".to_string());
        }

        record.status = String::from("reverted");
        record.reverted_at = Some(Utc::now().to_rfc3339());

        // Clone before save() to release the mutable borrow
        let record_clone = record.clone();

        self.save()?;

        Ok(record_clone)
    }

    fn save(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.history)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;

        fs::write(&self.history_path, content)
            .map_err(|e| format!("Failed to write history: {}", e))?;

        Ok(())
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
        path.push(format!("no_autostart_history_test_{}_{}", std::process::id(), counter));
        path
    }

    fn cleanup_test_dir(path: &PathBuf) {
        let _ = fs::remove_dir_all(path);
    }

    fn create_test_record(id: &str, status: &str) -> HistoryRecord {
        HistoryRecord {
            id: id.to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            operation_type: "close_process".to_string(),
            process_snapshot: ProcessSnapshot {
                pid: 1234,
                name: "test.exe".to_string(),
                executable_path: "C:\\test\\test.exe".to_string(),
                startup_type: "normal".to_string(),
                startup_location: None,
            },
            permanent_action: None,
            status: status.to_string(),
            reverted_at: None,
        }
    }

    #[test]
    fn test_operation_history_default() {
        let history = OperationHistory::default();
        assert!(history.records.is_empty());
    }

    #[test]
    fn test_history_manager_new_with_path() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let result = HistoryManager::new_with_path(test_dir.clone());
        assert!(result.is_ok(), "Should create HistoryManager successfully");

        let manager = result.unwrap();
        let history = manager.get_history();
        assert!(history.records.is_empty());

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_add_record_generates_id_and_timestamp() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

        let record = HistoryRecord {
            id: String::new(), // Empty ID - should be generated
            timestamp: String::new(), // Empty timestamp - should be generated
            operation_type: "close_process".to_string(),
            process_snapshot: ProcessSnapshot {
                pid: 1234,
                name: "test.exe".to_string(),
                executable_path: "C:\\test\\test.exe".to_string(),
                startup_type: "normal".to_string(),
                startup_location: None,
            },
            permanent_action: None,
            status: "completed".to_string(),
            reverted_at: None,
        };

        let result = manager.add_record(record);
        assert!(result.is_ok(), "Should add record successfully");

        let history = manager.get_history();
        assert_eq!(history.records.len(), 1);

        let saved_record = &history.records[0];
        assert!(!saved_record.id.is_empty(), "ID should be generated");
        assert!(!saved_record.timestamp.is_empty(), "Timestamp should be generated");
        assert_eq!(saved_record.status, "completed");

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_add_record_preserves_existing_id_and_timestamp() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

        let record = create_test_record("my-custom-id", "completed");

        let result = manager.add_record(record);
        assert!(result.is_ok(), "Should add record successfully");

        let history = manager.get_history();
        assert_eq!(history.records[0].id, "my-custom-id");
        assert_eq!(history.records[0].timestamp, "2024-01-01T00:00:00Z");

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_revert_operation_changes_status() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

        let record = create_test_record("record-1", "completed");
        manager.add_record(record).unwrap();

        // Wait slightly to ensure timestamp differs
        std::thread::sleep(std::time::Duration::from_millis(10));

        let result = manager.revert_operation("record-1");
        assert!(result.is_ok(), "Should revert operation successfully");

        let reverted_record = result.unwrap();
        assert_eq!(reverted_record.status, "reverted");
        assert!(
            reverted_record.reverted_at.is_some(),
            "reverted_at should be set"
        );

        // Check it's saved
        let history = manager.get_history();
        assert_eq!(history.records[0].status, "reverted");

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_revert_operation_fails_for_already_reverted() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

        let record = create_test_record("record-1", "reverted");
        manager.add_record(record).unwrap();

        let result = manager.revert_operation("record-1");
        assert!(result.is_err(), "Should fail for already reverted record");
        assert!(
            result.unwrap_err().contains("already reverted"),
            "Error should indicate operation already reverted"
        );

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_revert_operation_fails_for_nonexistent_record() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

        let result = manager.revert_operation("non-existent-id");
        assert!(result.is_err(), "Should fail for non-existent record");
        assert!(
            result.unwrap_err().contains("not found"),
            "Error should indicate record not found"
        );

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_history_record_serialization() {
        let record = HistoryRecord {
            id: "uuid-123".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            operation_type: "close_process".to_string(),
            process_snapshot: ProcessSnapshot {
                pid: 1234,
                name: "test.exe".to_string(),
                executable_path: "C:\\test\\test.exe".to_string(),
                startup_type: "normal".to_string(),
                startup_location: None,
            },
            permanent_action: None,
            status: "completed".to_string(),
            reverted_at: None,
        };

        let json = serde_json::to_string(&record).expect("Should serialize");
        let deserialized: HistoryRecord = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.timestamp, deserialized.timestamp);
        assert_eq!(record.operation_type, deserialized.operation_type);
        assert_eq!(record.status, deserialized.status);
    }

    #[test]
    fn test_history_persistence() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        // Create manager and add records
        {
            let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

            let record = create_test_record("persist-record", "completed");
            manager.add_record(record).unwrap();
        }

        // Create new manager - should load persisted history
        {
            let manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();
            let history = manager.get_history();

            assert_eq!(history.records.len(), 1);
            assert_eq!(history.records[0].id, "persist-record");
        }

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_add_record_inserts_at_front() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let mut manager = HistoryManager::new_with_path(test_dir.clone()).unwrap();

        let record1 = create_test_record("record-1", "completed");
        let record2 = create_test_record("record-2", "completed");

        manager.add_record(record1).unwrap();
        manager.add_record(record2).unwrap();

        let history = manager.get_history();
        assert_eq!(history.records.len(), 2);
        // record-2 should be at front (index 0) since it was added last
        assert_eq!(history.records[0].id, "record-2");
        assert_eq!(history.records[1].id, "record-1");

        cleanup_test_dir(&test_dir);
    }
}
