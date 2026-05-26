mod commands;
mod modules;
mod tray;
mod utils;

use commands::config::ConfigManagerState;
use commands::history::HistoryManagerState;
use commands::process::ProcessManagerState;
use modules::config_manager::ConfigManager;
use modules::history_manager::HistoryManager;
use modules::process_manager::ProcessManager;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let process_manager = ProcessManager::new();
            let config_manager = ConfigManager::new(app)?;
            let history_manager = HistoryManager::new(app)?;

            app.manage(ProcessManagerState(std::sync::Mutex::new(
                process_manager,
            )));
            app.manage(ConfigManagerState(std::sync::Mutex::new(config_manager)));
            app.manage(HistoryManagerState(std::sync::Mutex::new(history_manager)));

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
            commands::history::add_history_record,
            commands::history::revert_operation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
