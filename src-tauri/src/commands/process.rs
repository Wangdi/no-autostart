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
