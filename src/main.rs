mod model;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let word = std::env::args().nth(1).expect("Expect word as first argument");
    let mut is_cache = false;
    let mut cache_index = 0;
    let mut exist_words = String::new();
    let mut is_file_exist = false;

    if utils::check_file_exist("words.bin") {
        is_file_exist = true;

        exist_words = utils::load("words.bin");
        let words: Vec<String> = serde_json::from_str(&exist_words.to_string())?;

        for (index, item) in words.iter().enumerate() {
            if item == &word {
                is_cache = true;
                cache_index = index;
                break;
            }
        }
    }

    if is_cache {
        let cache = utils::load("api.bin");
        let cache_api: Vec<model::DictionaryAPI> = serde_json::from_str(&cache.to_string())?;

        utils::display_meaning(&cache_api[cache_index]);
    } else {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en_US/{}", word);

        let client = reqwest::Client::builder().build()?;
        let res = client.get(url).send().await?;
        let status = format!("{}", res.status()).to_string();

        if status == "200 OK" {
            let api = res.text().await?;
            let apis: Vec<model::DictionaryAPI> = serde_json::from_str(&api.to_string())?;

            utils::display_meaning(&apis[0]);

            let cache_api;

            if is_file_exist {
                exist_words = format!("{},\"{}\"]", &exist_words[0..exist_words.len() - 1], word);

                let cache = utils::load("api.bin");
                cache_api = format!("{},{}", &cache[0..cache.len() - 1], &api[1..api.len()]);
            } else {
                exist_words = format!("[\"{}\"]", word );
                cache_api = format!("{}", &api);
            }

            utils::save(&exist_words, "words.bin");
            utils::save(&cache_api, "api.bin");
        } else {
            println!("No Definitions Found");
        }
    }

    // let api = res.json::<Vec<DictionaryAPI>>().await?;

    Ok(())
}
