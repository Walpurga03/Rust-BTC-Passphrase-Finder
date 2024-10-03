mod generate_address;
mod find_passphrase;
mod find_taproot_passphrase;
mod config;

use find_passphrase::find_passphrase;
use generate_address::generate_all_addresses;
use find_taproot_passphrase::find_taproot_passphrase;
use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::Arc;
use simplelog::{Config as LogConfig, LevelFilter, SimpleLogger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    SimpleLogger::init(LevelFilter::Info, LogConfig::default())?;

    // Read and deserialize the configuration
    let config: Arc<Config> = Arc::new(
        toml::from_str(&std::fs::read_to_string("config.toml")?)
            .expect("Failed to deserialize config.toml")
    );

    // Create a menu
    let selections = vec![
        "Generate All Addresses", 
        "Find Passphrase", 
        "Find Taproot Passphrase"
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&selections)
        .interact()?;

    // Match the user's selection and call the corresponding function
    match selection {
        0 => generate_all_addresses()?,
        1 => find_passphrase(&config),
        2 => find_taproot_passphrase(&config),
        _ => unreachable!(),
    }

    Ok(())
}