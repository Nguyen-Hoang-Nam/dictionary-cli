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

fn create_not_exist_path(path: &std::path::Path) {
    use colored::Colorize;
    if !path.exists() {
        std::fs::create_dir(path).unwrap_or_else(|e| {
            panic!(
                "Can not create directory at {} due to error {}.",
                path.as_os_str().to_str().unwrap(),
                e.to_string().red()
            )
        });
    }
}

fn cache_path(file_name: &str) -> String {
    let mut path = dirs::cache_dir().unwrap();
    path.push("dictionary-cli");

    create_not_exist_path(&path);
    path.push(file_name);

    path.into_os_string().into_string().unwrap()
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
