use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Definitions {
    pub definition: String,
    pub example: Option<String>,
    pub synonyms: Option<Vec<String>>,
    pub antonyms: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meanings {
    pub part_of_speech: String,
    pub definitions: Vec<Definitions>,
    pub synonyms: Option<Vec<String>>,
    pub antonyms: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Phonetics {
    pub text: Option<String>,
    pub audio: Option<String>,
    pub source_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct License {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryAPI {
    pub word: String,
    pub phonetic: String,
    pub phonetics: Vec<Phonetics>,
    pub meanings: Vec<Meanings>,
    pub license: License,
    pub source_urls: Option<Vec<String>>,
}

pub enum Api {
    Free,
    Oxford,
    Google,
}
