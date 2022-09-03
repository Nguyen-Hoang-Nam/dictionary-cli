#![warn(clippy::nursery)]

mod api;
mod cache;
mod display;
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
            Arg::with_name("synonyms")
                .short('s')
                .long("synonyms")
                .help("Show synonym words"),
        )
        .arg(
            Arg::with_name("antonyms")
                .short('a')
                .long("antonyms")
                .help("Show antonym words"),
        )
        .arg(
            Arg::with_name("force")
                .short('f')
                .long("force")
                .help("Force find word in web instead of cache"),
        )
        .arg(
            Arg::with_name("list-api")
                .long("list-api")
                .help("List all supported api"),
        )
        .arg(
            Arg::with_name("api")
                .long("api")
                .help("free | oxford | google")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The word that yo&u seek")
                .index(1),
        )
        .get_matches();

    if matches.occurrences_of("list-api") == 1 {
        api::list_api();
        return Ok(());
    };

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

    let case = if matches.occurrences_of("synonyms") == 1 {
        case + 8
    } else {
        case
    };

    let case = if matches.occurrences_of("antonyms") == 1 {
        case + 16
    } else {
        case
    };

    let case = if case == 0 { 15 } else { case };

    let force = matches.occurrences_of("force") == 1;
    let api = api::to_enum(matches.value_of("api").unwrap_or_else(|| ""));
    let file_words = &cache::get_file_words(&api);
    let file_api = &cache::get_file_api(&api);

    let word = match matches.value_of("INPUT") {
        Some(w) => w,
        None => panic!("Missing word"),
    };

    let mut is_cache = false;
    let mut cache_index = 0;
    let mut exist_words = String::new();
    let mut is_file_exist = false;

    if cache::check_file_exist(file_words) {
        is_file_exist = true;

        exist_words = cache::load(file_words);

        if !force {
            let words: Vec<String> = serde_json::from_str(&exist_words.to_string())?;

            for (index, item) in words.iter().enumerate() {
                if item == word {
                    is_cache = true;
                    cache_index = index;
                    break;
                }
            }
        }
    }

    if !force && is_cache {
        let cache = cache::load(file_api);

        display::display(&cache, case, cache_index, &api).unwrap();
    } else {
        let body: String = api::call(word, &api).await?;

        display::display(&body, case, 0, &api).unwrap();

        let cache_api = if is_file_exist {
            exist_words = format!("{},\"{}\"]", &exist_words[0..exist_words.len() - 1], word);

            let cache = cache::load(file_api);
            format!("{},{}", &cache[0..cache.len() - 1], &body[1..body.len()])
        } else {
            exist_words = format!("[\"{}\"]", word);
            body
        };

        cache::save(&exist_words, file_words);
        cache::save(&cache_api, file_api);
    }

    Ok(())
}
