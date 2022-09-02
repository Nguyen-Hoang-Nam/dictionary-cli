use std::env;

use crate::model;

pub fn get_file_words(api: &model::Api) -> String {
    match api {
        model::Api::Oxford => "words-oxford.bin".to_string(),
        _ => "words.bin".to_string(),
    }
}

pub fn get_file_api(api: &model::Api) -> String {
    match api {
        model::Api::Oxford => "api-oxford.bin".to_string(),
        _ => "api.bin".to_string(),
    }
}

fn create_not_exist_path(path: &String) {
    use colored::Colorize;
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path).unwrap_or_else(|e| {
            panic!(
                "Can not create directory at {} due to error {}.",
                path.magenta(),
                e.to_string().red()
            )
        });
    }
}

fn cache_path(file_name: &str) -> String {
    let os = env::consts::OS;
    let mut result = String::new();

    if os == "linux" {
        match env::var("XDG_CACHE_HOME") {
            Ok(cache_directory) => {
                let path = format!("{}/dictionary-cli", cache_directory);
                create_not_exist_path(&path);

                result = format!("{}/{}", path, file_name);
            }
            Err(..) => {
                let path = format!("{}/.dictionary-cli", env::var("HOME").unwrap());
                create_not_exist_path(&path);

                result = format!("{}/{}", path, file_name)
            }
        }
    } else if os == "windows" {
        let path = "%USERPROFILE\\AppData\\dictionary-cli".to_string();
        create_not_exist_path(&path);

        result = format!("{}\\{}", path, file_name)
    } else if os == "macos" {
        let path = format!(
            "{}/{}",
            dirs::cache_dir()
                .expect("Cache dir not found")
                .display()
                .to_string(),
            "dictionary_cli".to_string()
        );
        create_not_exist_path(&path);
        result = format!("{}/{}", path, file_name)
    }

    result
}

pub fn check_file_exist(file_name: &str) -> bool {
    let file_path = cache_path(file_name);

    std::path::Path::new(&file_path).exists()
}

pub fn save(dictionary_api: &String, file_name: &str) {
    let file_path = cache_path(file_name);

    savefile::prelude::save_file(&file_path, 0, dictionary_api).unwrap();
}

pub fn load(file_name: &str) -> String {
    let file_path = cache_path(file_name);

    savefile::prelude::load_file(&file_path, 0).unwrap()
}
