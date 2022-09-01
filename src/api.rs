mod free;

use crate::{error, model};

pub fn to_enum(str: &str) -> model::Api {
    match str {
        "free" => model::Api::Free,
        "oxford" => model::Api::Oxford,
        "google" => model::Api::Google,
        _ => model::Api::Free,
    }
}

pub async fn call(word: &str, api: model::Api) -> Result<String, Box<dyn std::error::Error>> {
    match api {
        model::Api::Free => free::call(word).await,
        _ => Err(Box::new(error::ApiNotFoundError)),
    }
}
