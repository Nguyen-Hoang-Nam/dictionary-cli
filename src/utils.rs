use crate::model;
use std::env;
use std::io::Write;
use termcolor::WriteColor;

fn create_not_exist_path(path: &String) {
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path).expect("Can not create directory.");
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
                let path = format!("$HOME/.dictionary-cli");
                create_not_exist_path(&path);

                result = format!("{}/{}", path, file_name)
            }
        }
    } else if os == "windows" {
        let path = format!("%USERPROFILE\\AppData\\dictionary-cli");
        create_not_exist_path(&path);

        result = format!("{}\\{}", path, file_name)
    } else if os == "macos" {
        let path = format!("~/Library/Caches/dictionary-cli");
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

pub fn write_color(text: &str, color: termcolor::Color) -> std::io::Result<()> {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{}", text)
}

pub fn display_meaning(word: &model::DictionaryAPI) {
    let mut phonetics = String::new();
    for phonetic in word.phonetics.iter() {
        if phonetics.len() > 0 {
            phonetics = format!("{} {}", phonetics, phonetic.text);
        } else {
            phonetics = format!("{}", phonetic.text);
        }
    }

    write_color("\n", termcolor::Color::White).expect("Not show color.");
    print!("{} (", word.word);
    write_color(&phonetics, termcolor::Color::Blue).expect("Not show color.");
    write_color("):\n", termcolor::Color::White).expect("Not show color.");

    for meanings in word.meanings.iter() {
        let meaning_str = format!("({})\n", meanings.part_of_speech);
        write_color(&meaning_str, termcolor::Color::Green).expect("Not show color.");

        for definition in meanings.definitions.iter() {
            let definition_str = format!("\t_ {}\n\n", definition.definition);
            write_color(&&definition_str, termcolor::Color::White).expect("Not show color.");
        }
    }
}
