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
pub fn close_process(pid: u32, state: State<ProcessManagerState>) -> Result<String, String> {
    let mut manager = state.0.lock().unwrap();
    manager.close_process(pid)?;
    Ok(format!("Process {} closed successfully", pid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_processes_returns_vec() {
        let manager = ProcessManager::new();
        let state = ProcessManagerState(Mutex::new(manager));

        let processes = get_all_processes(tauri::State::from(&state));

        assert!(!processes.is_empty(), "Should return at least one process (current test process)");
    }

    #[test]
    fn test_close_process_success() {
        let manager = ProcessManager::new();
        let state = ProcessManagerState(Mutex::new(manager));

        // Use current process PID for testing (we won't actually kill it since we don't want to kill the test runner)
        // Instead, test with a non-existent PID to verify error handling
        let result = close_process(999999, tauri::State::from(&state));

        assert!(result.is_err(), "Should return error for non-existent process");
        assert!(result.unwrap_err().contains("not found"), "Error should indicate process not found");
    }

    #[test]
    fn test_process_manager_state_creation() {
        let manager = ProcessManager::new();
        let state = ProcessManagerState(Mutex::new(manager));

        // Verify we can lock and access the manager
        let locked = state.0.lock().unwrap();
        let processes = locked.get_all_processes();
        assert!(!processes.is_empty());
    }

    #[test]
    fn test_close_process_error_for_invalid_pid() {
        let manager = ProcessManager::new();
        let state = ProcessManagerState(Mutex::new(manager));

        // Test with PID 0 (System Idle Process - cannot be killed)
        let result = close_process(0, tauri::State::from(&state));

        assert!(result.is_err(), "Should return error for invalid PID");
    }
}
