#![warn(clippy::nursery)]

mod api;
mod cache;
mod error;
mod model;
mod utils;

use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Dictionary-cli")
        .version("1.2.0")
        .arg(
            Arg::with_name("definitions")
                .short('d')
                .long("definitions")
                .help("Show definitons of the word"),
        )
        .arg(
            Arg::with_name("phonetics")
                .short('p')
                .long("phonetics")
                .help("Show phonetics of the word"),
        )
        .arg(
            Arg::with_name("examples")
                .short('e')
                .long("examples")
                .help("Show examples of the word"),
        )
        .arg(
            Arg::with_name("similars")
                .short('s')
                .long("similars")
                .help("Show similar words"),
        )
        .arg(
            Arg::with_name("api")
                .short('a')
                .long("api")
                .help("free | oxford | google")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The word that you seek")
                .required(true)
                .index(1),
        )
        .get_matches();

    let case = 0;

    let case = if matches.occurrences_of("definitions") == 1 {
        case + 1
    } else {
        case
    };

    let case = if matches.occurrences_of("phonetics") == 1 {
        case + 2
    } else {
        case
    };

    let case = if matches.occurrences_of("examples") == 1 {
        case + 4
    } else {
        case
    };

    let case = if matches.occurrences_of("similars") == 1 {
        case + 8
    } else {
        case
    };

    let case = if case == 0 { 15 } else { case };

    let api = api::to_enum(matches.value_of("api").unwrap_or_else(|| ""));

    let word = matches.value_of("INPUT").unwrap();
    let mut is_cache = false;
    let mut cache_index = 0;
    let mut exist_words = String::new();
    let mut is_file_exist = false;

    if cache::check_file_exist("words.bin") {
        is_file_exist = true;

        exist_words = cache::load("words.bin");
        let words: Vec<String> = serde_json::from_str(&exist_words.to_string())?;

        for (index, item) in words.iter().enumerate() {
            if item == word {
                is_cache = true;
                cache_index = index;
                break;
            }
        }
    }

    if is_cache {
        let cache = cache::load("api.bin");
        let cache_api: Vec<model::DictionaryAPI> = serde_json::from_str(&cache)?;

        utils::display(&cache_api[cache_index], case);
    } else {
        let body: String = api::call(word, api).await?;
        let apis: Vec<model::DictionaryAPI> = serde_json::from_str(&body)?;

        utils::display(&apis[0], case);

        let cache_api = if is_file_exist {
            exist_words = format!("{},\"{}\"]", &exist_words[0..exist_words.len() - 1], word);

            let cache = cache::load("api.bin");
            format!("{},{}", &cache[0..cache.len() - 1], &body[1..body.len()])
        } else {
            exist_words = format!("[\"{}\"]", word);
            body
        };

        cache::save(&exist_words, "words.bin");
        cache::save(&cache_api, "api.bin");
    }

    Ok(())
}
