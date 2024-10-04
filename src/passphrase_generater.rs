use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::Arc;
use crate::config::Config;
use rayon::prelude::*;

fn generate_word<'a>(template: &'a str, replacements: &'a HashMap<&'a str, char>) -> Cow<'a, str> {
    let mut result = Cow::Borrowed(template);
    for (placeholder, replacement) in replacements {
        result = Cow::Owned(result.replace(placeholder, &replacement.to_string()));
    }
    result
}

fn initialize_placeholders<'a>(template: &'a str, config: &'a Config) -> (Vec<&'a str>, HashMap<&'a str, Vec<char>>) {
    let mut placeholders = Vec::new();
    let mut char_maps: HashMap<&str, Vec<char>> = HashMap::new();

    let placeholder_map = [
        ("'upp1'", &config.upp1),
        ("'upp2'", &config.upp2),
        ("'low1'", &config.low1),
        ("'low2'", &config.low2),
        ("'dig1'", &config.dig1),
        ("'dig2'", &config.dig2),
        ("'spe1'", &config.spe1),
        ("'spe2'", &config.spe2),
    ];

    for &(placeholder, chars) in &placeholder_map {
        if template.contains(placeholder) && !chars.is_empty() {
            placeholders.push(placeholder);
            char_maps.insert(placeholder, chars.chars().collect());
        }
    }

    (placeholders, char_maps)
}

fn generate_words(template: &str, config: &Config) -> Vec<String> {
    let (placeholders, char_maps) = initialize_placeholders(template, config);

    let max_indices: Vec<usize> = placeholders.iter().map(|&p| char_maps[p].len()).collect();
    let total_combinations: usize = max_indices.iter().product();

    (0..total_combinations).into_par_iter().map(|index| {
        let mut replacements = HashMap::new();
        let mut remainder = index;
        for (i, &placeholder) in placeholders.iter().enumerate() {
            let char_index = remainder % max_indices[i];
            remainder /= max_indices[i];
            replacements.insert(placeholder, char_maps[placeholder][char_index]);
        }
        generate_word(template, &replacements).into_owned()
    }).collect()
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