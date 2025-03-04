use magic_mirror::{commands, config};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::fetch_calendar_events])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // create the default config json file, if the not exist
            config::create_default_config_file(app.path());

            // add the config to the state
            app.manage(config::to_config(app.path()));

            // disable the visibility of the cursor in the application
            if let Some(window) = app.get_webview_window("main") {
                window.set_cursor_visible(false).unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri magic mirror application");
}
