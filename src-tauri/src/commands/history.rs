use crate::modules::history_manager::{HistoryManager, HistoryRecord, OperationHistory};
use std::sync::Mutex;
use tauri::State;

pub struct HistoryManagerState(pub Mutex<HistoryManager>);

#[tauri::command]
pub fn get_history(state: State<HistoryManagerState>) -> OperationHistory {
    let manager = state.0.lock().unwrap();
    manager.get_history().clone()
}

#[tauri::command]
pub fn add_history_record(
    record: HistoryRecord,
    state: State<HistoryManagerState>,
) -> Result<String, String> {
    let mut manager = state.0.lock().unwrap();
    manager.add_record(record)?;
    Ok("History record added".to_string())
}

#[tauri::command]
pub fn revert_operation(
    id: String,
    state: State<HistoryManagerState>,
) -> Result<HistoryRecord, String> {
    let mut manager = state.0.lock().unwrap();
    manager.revert_operation(&id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::history_manager::ProcessSnapshot;
    use std::fs;
    use std::path::PathBuf;

    fn get_test_dir() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("no_autostart_history_cmd_test_{}", std::process::id()));
        path
    }

    fn cleanup_test_dir(path: &PathBuf) {
        let _ = fs::remove_dir_all(path);
    }

    fn create_test_manager() -> HistoryManager {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);
        HistoryManager::new_with_path(test_dir).unwrap()
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
    fn test_get_history_returns_current_history() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        let history = get_history(tauri::State::from(&state));

        assert!(history.records.is_empty());
    }

    #[test]
    fn test_add_history_record_adds_record() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        let record = create_test_record("test-add", "completed");

        let result = add_history_record(record, tauri::State::from(&state));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "History record added");

        // Verify record was added
        let history = get_history(tauri::State::from(&state));
        assert_eq!(history.records.len(), 1);
        assert_eq!(history.records[0].id, "test-add");
    }

    #[test]
    fn test_revert_operation_updates_status() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        // Add a record first
        let record = create_test_record("test-revert", "completed");
        add_history_record(record, tauri::State::from(&state)).unwrap();

        // Revert the operation
        let result = revert_operation("test-revert".to_string(), tauri::State::from(&state));

        assert!(result.is_ok());
        let reverted_record = result.unwrap();
        assert_eq!(reverted_record.status, "reverted");
        assert!(reverted_record.reverted_at.is_some());

        // Verify saved in history
        let history = get_history(tauri::State::from(&state));
        assert_eq!(history.records[0].status, "reverted");
    }

    #[test]
    fn test_revert_operation_returns_error_for_already_reverted() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        // Add a record that's already reverted
        let record = create_test_record("test-already-reverted", "reverted");
        add_history_record(record, tauri::State::from(&state)).unwrap();

        // Try to revert again
        let result = revert_operation("test-already-reverted".to_string(), tauri::State::from(&state));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already reverted"));
    }

    #[test]
    fn test_revert_operation_returns_error_for_nonexistent_record() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        // Try to revert non-existent record
        let result = revert_operation("nonexistent-999".to_string(), tauri::State::from(&state));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_add_history_record_generates_uuid_when_empty() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        let mut record = create_test_record("", "completed");
        record.id = String::new(); // Empty ID

        let result = add_history_record(record, tauri::State::from(&state));

        assert!(result.is_ok());

        let history = get_history(tauri::State::from(&state));
        assert_eq!(history.records.len(), 1);
        assert!(!history.records[0].id.is_empty(), "ID should be auto-generated");
    }

    #[test]
    fn test_multiple_records_and_revert() {
        let manager = create_test_manager();
        let state = HistoryManagerState(Mutex::new(manager));

        // Add multiple records
        for i in 0..3 {
            let record = create_test_record(&format!("record-{}", i), "completed");
            add_history_record(record, tauri::State::from(&state)).unwrap();
        }

        // Verify all added
        let history = get_history(tauri::State::from(&state));
        assert_eq!(history.records.len(), 3);

        // Revert the second one
        let result = revert_operation("record-1".to_string(), tauri::State::from(&state));
        assert!(result.is_ok());

        // Verify only that one changed
        let history = get_history(tauri::State::from(&state));
        let record0 = history.records.iter().find(|r| r.id == "record-0").unwrap();
        let record1 = history.records.iter().find(|r| r.id == "record-1").unwrap();
        let record2 = history.records.iter().find(|r| r.id == "record-2").unwrap();

        assert_eq!(record0.status, "completed");
        assert_eq!(record1.status, "reverted");
        assert_eq!(record2.status, "completed");
    }
}
