#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    no_autostart_lib::run()
}
