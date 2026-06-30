use crate::error::{AppError, AppResult};
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
) -> AppResult<String> {
    let mut manager = state.0.lock().unwrap();
    manager.add_record(record).map_err(|e| AppError::HistoryWriteFailed(e))?;
    Ok("History record added".to_string())
}

#[tauri::command]
pub fn revert_operation(
    id: String,
    state: State<HistoryManagerState>,
) -> AppResult<HistoryRecord> {
    let mut manager = state.0.lock().unwrap();
    manager.revert_operation(&id).map_err(|e| AppError::HistoryError(e))
}

#[cfg(test)]
mod tests {
    // Note: Integration tests that require Tauri State are removed.
    // These tests cannot run outside of the Tauri runtime context.
    // See tests in modules/history_manager.rs for unit tests.
}
