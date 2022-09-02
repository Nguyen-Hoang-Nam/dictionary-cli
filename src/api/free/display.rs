use crate::api;
use crate::display;
use crate::utils;

fn examples(example: &Option<String>) {
    match example {
        Some(ref example) => {
            let exampe_str = format!("\te.g: {}", example);
            display::write_color(&exampe_str, termcolor::Color::White).expect("Not show color.")
        }
        None => print!(""),
    }
}

fn synonyms(synonyms: &Option<Vec<String>>) {
    match synonyms {
        Some(synonyms) => {
            let mut synonym_str = String::new();
            for synonym in synonyms.iter() {
                if !synonym_str.is_empty() {
                    synonym_str = format!("{}, {}", synonym_str, synonym)
                } else {
                    synonym_str = synonym.to_string()
                }
            }

            if synonym_str.len() > 0 {
                display::write_color("\n\tSimilar: ", termcolor::Color::Yellow)
                    .expect("Not show color.");
                synonym_str = format!("{} \n", synonym_str);
                display::write_color(&synonym_str, termcolor::Color::White)
                    .expect("Not show color.")
            }
        }
        None => print!(""),
    }
}

fn antonyms(antonyms: &Option<Vec<String>>) {
    match antonyms {
        Some(antonyms) => {
            let mut antonym_str = String::new();
            for antonym in antonyms.iter() {
                if !antonym_str.is_empty() {
                    antonym_str = format!("{}, {}", antonym_str, antonym)
                } else {
                    antonym_str = antonym.to_string()
                }
            }

            if antonym_str.len() > 0 {
                display::write_color("\n\tSimilar: ", termcolor::Color::Yellow)
                    .expect("Not show color.");
                antonym_str = format!("{} \n", antonym_str);
                display::write_color(&antonym_str, termcolor::Color::White)
                    .expect("Not show color.")
            }
        }
        None => print!(""),
    }
}

pub fn display(
    body: &String,
    case: u8,
    index: usize,
    _: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let apis: Vec<api::free::model::DictionaryAPI> = serde_json::from_str(&body)?;
    let word = &apis[index];

    let mut phonetics = String::new();
    for phonetic in word.phonetics.iter() {
        match &phonetic.text {
            Some(t) => {
                if !phonetics.is_empty() {
                    phonetics = format!("{} {}", phonetics, t);
                } else {
                    phonetics = t.to_string();
                }
            }
            None => {}
        };
    }

    if utils::get_bit_at(case, 1) {
        display::write_color("\n", termcolor::Color::White).expect("Not show color.");
        print!("{} (", word.word);
        display::write_color(&phonetics, termcolor::Color::Blue).expect("Not show color.");
        display::write_color("):\n", termcolor::Color::White).expect("Not show color.");
    } else {
        println!("{}:", word.word);
    }

    let is_definitions = utils::get_bit_at(case, 0);
    let is_examples = utils::get_bit_at(case, 2);
    let is_synonyms = utils::get_bit_at(case, 3);
    let is_antonyms = utils::get_bit_at(case, 4);

    if is_definitions || is_examples || is_synonyms || is_antonyms {
        for meanings in word.meanings.iter() {
            let meaning_str = format!("({})\n", meanings.part_of_speech);
            display::write_color(&meaning_str, termcolor::Color::Green).expect("Not show color.");

            for definition in meanings.definitions.iter() {
                if is_definitions {
                    let definition_str = format!("\t_ {}\n", definition.definition);
                    display::write_color(&definition_str, termcolor::Color::White)
                        .expect("Not show color.");
                }

                if is_examples {
                    examples(&definition.example);
                }

                if is_synonyms {
                    synonyms(&definition.synonyms);
                }

                if is_antonyms {
                    antonyms(&definition.antonyms);
                }

                println!("");
            }

            if is_synonyms {
                synonyms(&meanings.synonyms);
            }

            if is_antonyms {
                antonyms(&meanings.antonyms);
            }
        }
    }

    Ok(())
}
