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

pub fn display(word: &model::DictionaryAPI, case: u8) {
    let mut phonetics = String::new();
    for phonetic in word.phonetics.iter() {
        if !phonetics.is_empty() {
            phonetics = format!("{} {}", phonetics, phonetic.text);
        } else {
            phonetics = phonetic.text.to_string();
        }
    }

    if get_bit_at(case, 1) {
        write_color("\n", termcolor::Color::White).expect("Not show color.");
        print!("{} (", word.word);
        write_color(&phonetics, termcolor::Color::Blue).expect("Not show color.");
        write_color("):\n", termcolor::Color::White).expect("Not show color.");
    } else {
        println!("{}:", word.word);
    }

    let bit_0 = get_bit_at(case, 0);
    let bit_2 = get_bit_at(case, 2);
    let bit_3 = get_bit_at(case, 3);

    if bit_0 || bit_2 || bit_3 {
        for meanings in word.meanings.iter() {
            let meaning_str = format!("({})\n", meanings.part_of_speech);
            write_color(&meaning_str, termcolor::Color::Green).expect("Not show color.");

            for definition in meanings.definitions.iter() {
                if bit_0 {
                    let definition_str = format!("\t_ {}\n", definition.definition);
                    write_color(&definition_str, termcolor::Color::White).expect("Not show color.");
                }

                if bit_2 {
                    match definition.example {
                        Some(ref example) => {
                            let exampe_str = format!("\n\te.g: {}\n", example);
                            write_color(&exampe_str, termcolor::Color::White)
                                .expect("Not show color.")
                        }
                        None => print!(""),
                    }
                }

                if bit_3 {
                    match &definition.synonyms {
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
                                write_color("\n\tSimilar: ", termcolor::Color::Yellow)
                                    .expect("Not show color.");
                                synonym_str = format!("{} \n", synonym_str);
                                write_color(&synonym_str, termcolor::Color::White)
                                    .expect("Not show color.")
                            }
                        }
                        None => print!(""),
                    }
                }

                println!("");
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
