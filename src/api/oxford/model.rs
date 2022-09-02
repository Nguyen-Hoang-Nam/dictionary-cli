use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub id: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThesaurusLink {
    pub entry_id: String,
    pub sense_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Synonym {
    pub language: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subsense {
    pub definitions: Vec<String>,
    pub examples: Vec<Example>,
    pub id: String,
    pub short_definitions: Vec<String>,
    pub synonyms: Option<Vec<Synonym>>,
    pub thesaurus_links: Option<Vec<ThesaurusLink>>,
    pub registers: Option<Vec<Register>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sense {
    pub definitions: Vec<String>,
    pub examples: Vec<Example>,
    pub id: String,
    pub short_definitions: Vec<String>,
    pub subsenses: Vec<Subsense>,
    pub synonyms: Vec<Synonym>,
    pub thesaurus_links: Option<Vec<ThesaurusLink>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pronunciation {
    pub audio_file: String,
    pub dialects: Vec<String>,
    pub phonetic_notation: String,
    pub phonetic_spelling: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LexicalCategory {
    pub id: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub pronunciations: Vec<Pronunciation>,
    pub senses: Vec<Sense>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Derivative {
    pub id: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LexicalEntry {
    pub derivatives: Vec<Derivative>,
    pub entries: Vec<Entry>,
    pub lexical_category: LexicalCategory,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OResult {
    pub id: String,
    pub language: String,
    pub lexical_entries: Vec<LexicalEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: String,
    pub results: Vec<OResult>,
}
