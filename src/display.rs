use crate::model;
use std::io::Write;
use termcolor::WriteColor;

pub fn write_color(text: &str, color: termcolor::Color) -> std::io::Result<()> {
    let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{}", text)
}

const fn get_bit_at(input: u8, n: u8) -> bool {
    input & (1 << n) != 0
}

fn examples(example: &Option<String>) {
    match example {
        Some(ref example) => {
            let exampe_str = format!("\te.g: {}", example);
            write_color(&exampe_str, termcolor::Color::White).expect("Not show color.")
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
                write_color("\n\tSimilar: ", termcolor::Color::Yellow).expect("Not show color.");
                synonym_str = format!("{} \n", synonym_str);
                write_color(&synonym_str, termcolor::Color::White).expect("Not show color.")
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
                write_color("\n\tSimilar: ", termcolor::Color::Yellow).expect("Not show color.");
                antonym_str = format!("{} \n", antonym_str);
                write_color(&antonym_str, termcolor::Color::White).expect("Not show color.")
            }
        }
        None => print!(""),
    }
}

pub fn display(word: &model::DictionaryAPI, case: u8) {
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

    if get_bit_at(case, 1) {
        write_color("\n", termcolor::Color::White).expect("Not show color.");
        print!("{} (", word.word);
        write_color(&phonetics, termcolor::Color::Blue).expect("Not show color.");
        write_color("):\n", termcolor::Color::White).expect("Not show color.");
    } else {
        println!("{}:", word.word);
    }

    let is_phonetics = get_bit_at(case, 0);
    let is_examples = get_bit_at(case, 2);
    let is_synonyms = get_bit_at(case, 3);
    let is_antonyms = get_bit_at(case, 4);

    if is_phonetics || is_examples || is_synonyms || is_antonyms {
        for meanings in word.meanings.iter() {
            let meaning_str = format!("({})\n", meanings.part_of_speech);
            write_color(&meaning_str, termcolor::Color::Green).expect("Not show color.");

            for definition in meanings.definitions.iter() {
                if is_phonetics {
                    let definition_str = format!("\t_ {}\n", definition.definition);
                    write_color(&definition_str, termcolor::Color::White).expect("Not show color.");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_at_1() {
        assert_eq!(true, get_bit_at(1, 0));
    }

    #[test]
    fn test_get_bit_at_2() {
        assert_eq!(false, get_bit_at(1, 1));
    }
}
