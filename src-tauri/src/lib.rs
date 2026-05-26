mod commands;
mod modules;
mod tray;
mod utils;

use commands::process::ProcessManagerState;
use modules::process_manager::ProcessManager;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(ProcessManagerState(std::sync::Mutex::new(
            ProcessManager::new(),
        )))
        .setup(|app| {
            tray::setup_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::process::get_all_processes,
            commands::process::close_process,
            commands::config::get_config,
            commands::config::save_config,
            commands::config::add_to_auto_close_list,
            commands::config::remove_from_auto_close_list,
            commands::history::get_history,
            commands::history::revert_operation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
