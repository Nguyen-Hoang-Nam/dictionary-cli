use crate::model;
use std::env;
use std::io::Write;
use termcolor::WriteColor;

fn create_not_exist_path(path: &String) {
	use colored::Colorize;
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path).unwrap_or_else(|e| panic!("Can not create directory at {} due to error {}.", path.magenta(), e.to_string().red()));
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
        let path = "~/Library/Caches/dictionary-cli".to_string();
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

const fn get_bit_at(input: u8, n: u8) -> bool {
    input & (1 << n) != 0
}

pub fn display_meaning(word: &model::DictionaryAPI, case: u8) {
    let mut phonetics = String::new();
    for phonetic in word.phonetics.iter() {
        if !phonetics.is_empty() {
            phonetics = format!("{} {}", phonetics, phonetic.text);
        } else {
            phonetics = phonetic.text.to_string();
        }
    }

    if get_bit_at(case, 1) {
        write_color("\n", termcolor::Color::White).expect("Not show color.");
        print!("{} (", word.word);
        write_color(&phonetics, termcolor::Color::Blue).expect("Not show color.");
        write_color("):\n", termcolor::Color::White).expect("Not show color.");
    } else {
        println!("{}:", word.word);
    }

    let bit_0 = get_bit_at(case, 0);
    let bit_2 = get_bit_at(case, 2);
    let bit_3 = get_bit_at(case, 3);

    if bit_0 || bit_2 || bit_3 {
        for meanings in word.meanings.iter() {
            let meaning_str = format!("({})\n", meanings.part_of_speech);
            write_color(&meaning_str, termcolor::Color::Green).expect("Not show color.");

            for definition in meanings.definitions.iter() {
                if bit_0 {
                    let definition_str = format!("\t_ {}\n", definition.definition);
                    write_color(&definition_str, termcolor::Color::White)
                        .expect("Not show color.");
                }

                if bit_2 {
                    match definition.example {
                        Some(ref example) => {
                            let exampe_str = format!("\te.g: {}\n", example);
                            write_color(&exampe_str, termcolor::Color::White)
                                .expect("Not show color.")
                        }
                        None => println!(),
                    }
                }

                if bit_3 {
                    match &definition.synonyms {
                        Some(synonyms) => {
                            let mut synonym_str = String::new();
                            for synonym in synonyms.iter() {
                                if !synonym_str.is_empty() {
                                    synonym_str = format!("{}, {}", synonym_str, synonym)
                                } else {
                                    synonym_str = synonym.to_string()
                                }
                            }
                            synonym_str = format!("\n\tSimilar: {}", synonym_str);
                            write_color(&synonym_str, termcolor::Color::White)
                                .expect("Not show color.")
                        }
                        None => println!(),
                    }
                }

                println!("\n");
            }
        }
    }
}
