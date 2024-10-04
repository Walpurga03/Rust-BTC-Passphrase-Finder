use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::Arc;
use crate::config::Config;

fn generate_word(template: &str, replacements: &[(&str, char)]) -> String {
    let mut result = template.to_string();
    for (placeholder, replacement) in replacements {
        result = result.replace(placeholder, &replacement.to_string());
    }
    result
}

fn generate_words(template: &str, config: &Config) -> Vec<String> {
    let mut words = Vec::new();
    let mut placeholders = Vec::new();

    if template.contains("'uppercase'") {
        placeholders.push("'uppercase'");
    }
    if template.contains("'lowercase'") {
        placeholders.push("'lowercase'");
    }
    if template.contains("'digits'") {
        placeholders.push("'digits'");
    }
    if template.contains("'special'") && !config.special.is_empty() {
        placeholders.push("'special'");
    }

    for replacement1 in config.uppercase.chars() {
        let mut replacements = vec![("'uppercase'", replacement1)];
        if placeholders.contains(&"'lowercase'") {
            for replacement2 in config.lowercase.chars() {
                replacements.push(("'lowercase'", replacement2));
                if placeholders.contains(&"'digits'") {
                    for replacement3 in config.digits.chars() {
                        replacements.push(("'digits'", replacement3));
                        if placeholders.contains(&"'special'") {
                            for replacement4 in config.special.chars() {
                                replacements.push(("'special'", replacement4));
                                let word = generate_word(template, &replacements);
                                words.push(word);
                                replacements.pop();
                            }
                        } else {
                            let word = generate_word(template, &replacements);
                            words.push(word);
                        }
                        replacements.pop();
                    }
                } else {
                    let word = generate_word(template, &replacements);
                    words.push(word);
                }
                replacements.pop();
            }
        } else {
            let word = generate_word(template, &replacements);
            words.push(word);
        }
    }

    words
}

fn save_words_to_file(words: &[String], file_path: &str) -> io::Result<()> {
    // Ensure the directory exists
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;
    for word in words {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

pub fn generate_and_save_passphrases(config: &Arc<Config>) -> Result<(), Box<dyn std::error::Error>> {
    let words = generate_words(&config.passphrase, config);
    save_words_to_file(&words, &config.wordlist_path)?;
    println!("Words successfully written to file.");
    Ok(())
}