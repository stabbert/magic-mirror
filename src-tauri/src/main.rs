// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            config::create_default_config(app.path_resolver());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri magic mirror application");
}
