use serde::Serialize;
use tauri::State;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub path: Option<String>,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

#[tauri::command]
pub fn get_all_processes() -> Vec<ProcessInfo> {
    // Placeholder - will be implemented in Task 2
    vec![]
}

#[tauri::command]
pub fn close_process(pid: u32) -> Result<(), String> {
    // Placeholder - will be implemented in Task 2
    Err(format!("Close process {} not yet implemented", pid))
}
