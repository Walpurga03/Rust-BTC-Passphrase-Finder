mod generate_address;
mod find_passphrase;

use find_passphrase::find_passphrase;
use generate_address::generate_address;
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::Arc;
use simplelog::{Config as LogConfig, LevelFilter, SimpleLogger};

fn main() {
    // Initialize logging
    SimpleLogger::init(LevelFilter::Info, LogConfig::default()).expect("Failed to initialize logger");

    // Read and deserialize the configuration
    let config: Arc<find_passphrase::Config> = Arc::new(
        toml::from_str(&std::fs::read_to_string("config.toml").expect("Failed to read config.toml"))
            .expect("Failed to deserialize config.toml")
    );

    // Create a menu
    let selections = vec!["Generate Address", "Find Passphrase"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections)
        .interact()
        .expect("Failed to select an option");

    match selection {
        0 => generate_address(),
        1 => find_passphrase(&config),
        _ => unreachable!(),
    }
}