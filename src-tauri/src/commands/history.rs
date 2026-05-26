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
