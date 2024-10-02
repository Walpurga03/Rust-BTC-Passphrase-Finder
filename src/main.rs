mod generate_address;
mod find_passphrase;
mod generate_taproot_address;
mod find_taproot_passphrase;
mod config;

use find_passphrase::find_passphrase;
use generate_address::generate_address;
use generate_taproot_address::generate_taproot_address;
use find_taproot_passphrase::find_taproot_passphrase;
use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::Arc;
use simplelog::{Config as LogConfig, LevelFilter, SimpleLogger};

fn main() {
    // Initialize logging
    SimpleLogger::init(LevelFilter::Info, LogConfig::default()).expect("Failed to initialize logger");

    // Read and deserialize the configuration
    let config: Arc<Config> = Arc::new(
        toml::from_str(&std::fs::read_to_string("config.toml").expect("Failed to read config.toml"))
            .expect("Failed to deserialize config.toml")
    );

    // Create a menu
    let selections = vec![
        "Generate Address", 
        "Generate Taproot Address", 
        "Find Passphrase", 
        "Find Taproot Passphrase"
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections)
        .interact()
        .expect("Failed to select an option");

    match selection {
        0 => generate_address(),
        1 => generate_taproot_address(),
        2 => find_passphrase(&config),
        3 => {
            find_taproot_passphrase(&config);
        },
        _ => unreachable!(),
    }
}