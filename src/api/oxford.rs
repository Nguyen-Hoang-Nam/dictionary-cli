pub mod display;
pub mod model;

use crate::error;

const ORIGIN: &str = "https://od-api.oxforddictionaries.com/api/v2/entries/en";

pub async fn call(word: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", ORIGIN, word);

    let app_id_env = "OXFORD_APP_ID";
    let app_id = match std::env::var(app_id_env) {
        Ok(v) => v,
        Err(e) => panic!("{} is not set ({})", app_id_env, e),
    };

    let app_key_env = "OXFORD_APP_KEY";
    let app_key = match std::env::var(app_key_env) {
        Ok(v) => v,
        Err(e) => panic!("{} is not set ({})", app_key_env, e),
    };

    let client = reqwest::Client::builder().build()?;
    let request = client
        .get(url)
        .header("APP_ID", app_id)
        .header("APP_KEY", app_key);
    let res = request.send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let body = res.text().await?;

            Ok(format!("[{}]", body))
        }
        _ => Err(Box::new(error::NotFoundError)),
    }
}
