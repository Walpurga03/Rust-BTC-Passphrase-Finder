use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::Arc;
use crate::config::Config;
use crate::regex_parser::{RegexExpander, RegexError};

const MAX_COMBINATIONS: usize = 2_000_000; // Increased limit to handle patterns like [0-9]{4,6}

fn generate_words(pattern: &str) -> Result<Vec<String>, RegexError> {
    let expander = RegexExpander::new(MAX_COMBINATIONS);
    expander.expand_pattern(pattern)
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
    println!("Generating passphrases using regex pattern: {}", config.passphrase);
    let words = generate_words(&config.passphrase)?;
    println!("Generated {} possible passphrases", words.len());
    save_words_to_file(&words, &config.wordlist_path)?;
    println!("Words successfully written to file: {}", config.wordlist_path);
    Ok(())
}