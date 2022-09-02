pub mod display;
pub mod model;

use crate::error;

const ORIGIN: &str = "https://api.dictionaryapi.dev/api/v2/entries/en_US";

pub async fn call(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", ORIGIN, word);

    let client = reqwest::Client::builder().build()?;
    let res = client.get(url).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.text().await?;

            Ok(body)
        }
        _ => Err(Box::new(error::NotFoundError)),
    }
}
