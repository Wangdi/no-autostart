use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    CloseProcess,
    DisableAutostart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub operation_type: OperationType,
    pub target: String,
    pub timestamp: DateTime<Utc>,
    pub reverted: bool,
}

#[tauri::command]
pub fn get_history() -> Vec<HistoryEntry> {
    // Placeholder - will be implemented in Task 10
    vec![]
}

#[tauri::command]
pub fn revert_operation(entry_id: String) -> Result<(), String> {
    // Placeholder - will be implemented in Task 10
    let _ = entry_id;
    Err("Revert operation not yet implemented".to_string())
}
