use crate::api;
use crate::display;
use crate::utils;

fn definitions(definitions: &Vec<String>) {
    for definition in definitions.iter() {
        let definition_str = format!("\t_ {}\n", definition);
        display::write_color(&definition_str, termcolor::Color::White).expect("Not show color.");
    }
}

fn examples(examples: &Vec<api::oxford::model::Example>) {
    for example in examples.iter() {
        let exampe_str = format!("\te.g: {}", example.text);
        display::write_color(&exampe_str, termcolor::Color::White).expect("Not show color.")
    }
}

fn synonyms(synonyms: &Vec<api::oxford::model::Synonym>) {
    let mut synonym_str = String::new();
    for synonym in synonyms.iter() {
        if !synonym_str.is_empty() {
            synonym_str = format!("{}, {}", synonym_str, synonym.text)
        } else {
            synonym_str = synonym.text.to_string()
        }
    }

    if synonym_str.len() > 0 {
        display::write_color("\n\n\tSimilar: ", termcolor::Color::Yellow).expect("Not show color.");
        synonym_str = format!("{} \n", synonym_str);
        display::write_color(&synonym_str, termcolor::Color::White).expect("Not show color.")
    }
}

pub fn display(
    body: &String,
    case: u8,
    index: usize,
    is_cache: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw_response: String = if is_cache {
        body.to_string()
    } else {
        let value = format!("[{}]", &body);
        value
    };

    let responses: &Vec<api::oxford::model::Response> = &serde_json::from_str(&raw_response)?;
    let response: &api::oxford::model::Response = &responses[index];

    let result: &Vec<api::oxford::model::OResult> = &response.results;
    let word = &result[0];

    let mut phonetics = String::new();
    for lexical_entry in word.lexical_entries.iter() {
        for entry in lexical_entry.entries.iter() {
            for pronunciation in entry.pronunciations.iter() {
                if !phonetics.is_empty() {
                    phonetics = format!("{} /{}/", phonetics, pronunciation.phonetic_spelling);
                } else {
                    phonetics = format!("/{}/", pronunciation.phonetic_spelling);
                }
            }
        }
    }

    if utils::get_bit_at(case, 1) {
        display::write_color("\n", termcolor::Color::White).expect("Not show color.");
        print!("{} (", word.id);
        display::write_color(&phonetics, termcolor::Color::Blue).expect("Not show color.");
        display::write_color("):\n", termcolor::Color::White).expect("Not show color.");
    } else {
        println!("{}:", word.id);
    }

    let is_definitions = utils::get_bit_at(case, 0);
    let is_examples = utils::get_bit_at(case, 2);
    let is_synonyms = utils::get_bit_at(case, 3);
    let is_antonyms = utils::get_bit_at(case, 4);

    if is_definitions || is_examples || is_synonyms || is_antonyms {
        for lexical_entry in word.lexical_entries.iter() {
            let meaning_str = format!("({})\n", lexical_entry.lexical_category.id);
            display::write_color(&meaning_str, termcolor::Color::Green).expect("Not show color.");

            for entry in lexical_entry.entries.iter() {
                for sense in entry.senses.iter() {
                    if is_definitions {
                        definitions(&sense.definitions);
                    }

                    if is_examples {
                        examples(&sense.examples);
                    }

                    if is_synonyms {
                        synonyms(&sense.synonyms);
                    }
                }

                println!("");
            }
        }
    }

    Ok(())
}
