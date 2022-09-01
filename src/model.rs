use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Definitions {
    pub definition: String,
    pub example: Option<String>,
    pub synonyms: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meanings {
    pub part_of_speech: String,
    pub definitions: Vec<Definitions>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Phonetics {
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DictionaryAPI {
    pub word: String,
    pub phonetics: Vec<Phonetics>,
    pub meanings: Vec<Meanings>,
}

pub enum Api {
    Free,
    Oxford,
    Google,
}
