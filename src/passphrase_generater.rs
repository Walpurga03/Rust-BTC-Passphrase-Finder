use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::Arc;
use crate::config::Config;

fn generate_word(template: &str, replacement1: char, replacement2: char, replacement3: char, replacement4: char) -> String {
    template.replace("'uppercase'", &replacement1.to_string())
            .replace("'lowercase'", &replacement2.to_string())
            .replace("'digits'", &replacement3.to_string())
            .replace("'special'", &replacement4.to_string())
}

fn generate_words(template: &str, uppercase: &str, lowercase: &str, digits: &str, special: &str) -> Vec<String> {
    let mut words = Vec::new();

    for replacement1 in uppercase.chars() {
        for replacement2 in lowercase.chars() {
            for replacement3 in digits.chars() {
                for replacement4 in special.chars() {
                    let word = generate_word(template, replacement1, replacement2, replacement3, replacement4);
                    words.push(word);
                }
            }
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
    let words = generate_words(&config.passphrase, &config.uppercase, &config.lowercase, &config.digits, &config.special);
    save_words_to_file(&words, &config.wordlist_path)?;
    println!("Words successfully written to file.");
    Ok(())
}