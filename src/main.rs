#![warn(clippy::nursery)]

mod model;
mod utils;

use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Dictionary-cli")
        .version("1.1.0")
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
                .help("Show  examples of the word"),
        )
        .arg(
            Arg::with_name("similars")
                .short('s')
                .long("similars")
                .help("Show  similar words"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The word that you seek")
                .required(true)
                .index(1),
        )
        .get_matches();

    // let word = std::env::args().nth(1).expect("Expect word as first argument");
    let word = matches.value_of("INPUT").unwrap();
    let mut is_cache = false;
    let mut cache_index = 0;
    let mut exist_words = String::new();
    let mut is_file_exist = false;

    if utils::check_file_exist("words.bin") {
        is_file_exist = true;

        exist_words = utils::load("words.bin");
        let words: Vec<String> = serde_json::from_str(&exist_words.to_string())?;

        for (index, item) in words.iter().enumerate() {
            if item == word {
                is_cache = true;
                cache_index = index;
                break;
            }
        }
    }

    let mut case = 0;

    let show_definition = matches.occurrences_of("definitions");
    if show_definition == 1 {
        case += 1;
    }

    let show_phonetic = matches.occurrences_of("phonetics");
    if show_phonetic == 1 {
        case += 2
    }

    let show_example = matches.occurrences_of("examples");
    if show_example == 1 {
        case += 4
    }

    let show_similar = matches.occurrences_of("similars");
    if show_similar == 1 {
        case += 8
    }

    if case == 0 {
        case = 15
    }

    if is_cache {
        let cache = utils::load("api.bin");
        let cache_api: Vec<model::DictionaryAPI> = serde_json::from_str(&cache)?;

        utils::display_meaning(&cache_api[cache_index], case);
    } else {
        let url = format!(
            "https://api.dictionaryapi.dev/api/v2/entries/en_US/{}",
            word
        );

        let client = reqwest::Client::builder().build()?;
        let res = client.get(url).send().await?;
        let status = format!("{}", res.status());

        if status == "200 OK" {
            let api = res.text().await?;
            let apis: Vec<model::DictionaryAPI> = serde_json::from_str(&api)?;

            utils::display_meaning(&apis[0], case);

            let cache_api = if is_file_exist {
                exist_words = format!("{},\"{}\"]", &exist_words[0..exist_words.len() - 1], word);

                let cache = utils::load("api.bin");
                format!("{},{}", &cache[0..cache.len() - 1], &api[1..api.len()])
            } else {
                exist_words = format!("[\"{}\"]", word);
                api
            };

            utils::save(&exist_words, "words.bin");
            utils::save(&cache_api, "api.bin");
        } else {
            println!("No Definitions Found");
        }
    }

    Ok(())
}
