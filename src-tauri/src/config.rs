use serde::Deserialize;
use std::fs::{self, File};
use tauri::{
    Wry,
    path::{BaseDirectory, PathResolver},
};

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Deserialize)]
pub struct Config {
    pub calendar: CalendarConfig,
}

#[derive(Deserialize)]
pub struct CalendarConfig {
    pub calendars: Vec<CalendarUrlConfig>,
}

#[derive(Deserialize)]
pub struct CalendarUrlConfig {
    pub url: String,
}

pub fn create_default_config_file(path_resolver: &PathResolver<Wry>) {
    let app_config_dir = path_resolver.app_config_dir().unwrap();
    let app_config_file = app_config_dir.join(CONFIG_FILE_NAME);

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

    if let Ok(default_app_config_file) =
        path_resolver.resolve("resources/config.json", BaseDirectory::Resource)
    {
        match fs::copy(&default_app_config_file, &app_config_file) {
            Ok(_) => println!(
                "Magic mirror default config file successfully created: {:?}",
                app_config_file
            ),
            Err(e) => panic!("Failed to create the default config file: {:?}", e),
        }
    }
}

pub fn to_config(path_resolver: &PathResolver<Wry>) -> Config {
    let app_config_dir = path_resolver.app_config_dir().unwrap();
    let app_config_file = app_config_dir.join(CONFIG_FILE_NAME);

    if app_config_file.is_file() {
        let config_file =
            File::open(app_config_file.as_path()).expect("Error while open the config file");
        let config: Config = serde_json::from_reader(config_file)
            .expect("Error while reading or parsing the config file");

        return config;
    }

    panic!("Failed to serialze config file to json");
}
