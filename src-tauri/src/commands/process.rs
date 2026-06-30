use crate::error::{AppError, AppResult};
use crate::modules::process_manager::{ProcessInfo, ProcessManager};
use std::sync::Mutex;
use tauri::State;

pub struct ProcessManagerState(pub Mutex<ProcessManager>);

#[tauri::command]
pub fn get_all_processes(state: State<ProcessManagerState>) -> Vec<ProcessInfo> {
    let mut manager = state.0.lock().unwrap();
    manager.get_all_processes()
}

#[tauri::command]
pub fn close_process(pid: u32, state: State<ProcessManagerState>) -> AppResult<String> {
    let mut manager = state.0.lock().unwrap();
    manager.close_process(pid).map_err(AppError::from)?;
    Ok(format!("Process {} closed successfully", pid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_state_creation() {
        let manager = ProcessManager::new();
        let state = ProcessManagerState(Mutex::new(manager));

        let mut locked = state.0.lock().unwrap();
        let processes = locked.get_all_processes();
        assert!(!processes.is_empty());
    }
}
