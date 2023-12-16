use std::fs;

use tauri::PathResolver;

pub fn create_default_config(path_resolver: PathResolver) {
    let app_config_dir = path_resolver.app_config_dir().unwrap();
    let app_config_file = app_config_dir.join("config.json");

    if app_config_file.is_file() {
        println!("Magic mirror config file exist: {:?}", app_config_file);
        return;
    }

    if !app_config_dir.is_dir() && fs::create_dir(&app_config_dir).is_ok() {
        println!(
            "Magic mirror config directory created: {:?}",
            app_config_dir
        );
    }

    if let Some(default_app_config_file) = path_resolver.resolve_resource("resources/config.json") {
        match fs::copy(default_app_config_file, app_config_file.clone()) {
            Ok(_) => println!(
                "Magic mirror default config file successfully created: {:?}",
                app_config_file
            ),
            Err(_) => panic!("Failed to create the default config file"),
        }
    }
}
