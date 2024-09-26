use bitcoin::secp256k1::Secp256k1;
use bip39::{Mnemonic, Language};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use memmap::Mmap;
use std::sync::Arc;
use log::{info};
use simplelog::{Config as LogConfig, LevelFilter, SimpleLogger};
use std::fs::File;
use std::str::FromStr;
use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    seed_phrase: String,
    derivation_path: String,
    expected_address: String,
    wordlist_path: String,
}

fn main() {
    // Initialisieren Sie das Logging
    SimpleLogger::init(LevelFilter::Info, LogConfig::default()).expect("Failed to initialize logger");

    // Lesen und deserialisieren Sie die Konfiguration
    let config: Arc<Config> = Arc::new(
        toml::from_str(&std::fs::read_to_string("config.toml").expect("Failed to read config.toml"))
            .expect("Failed to deserialize config.toml")
    );

    // Öffnen und Speicher-mappen Sie die Wortliste
    let file = File::open(&config.wordlist_path).expect("Failed to open wordlist.txt");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to map the file") };
    let lines: Vec<&str> = mmap.split(|&byte| byte == b'\n')
        .map(|line| std::str::from_utf8(line).expect("Invalid UTF-8"))
        .collect();

    // Erstellen Sie einen Fortschrittsbalken
    let pb = ProgressBar::new(lines.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    // Erstellen Sie einen benutzerdefinierten Thread-Pool
    let num_threads = 4; // Beispiel: 4 Threads
    let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    // Führen Sie die parallele Verarbeitung innerhalb des benutzerdefinierten Thread-Pools aus
    pool.install(|| {
        lines.par_iter().for_each(|&passphrase| {
            let mnemonic = Mnemonic::parse_in(Language::English, &config.seed_phrase).expect("Failed to create mnemonic");
            let seed = mnemonic.to_seed(passphrase);
            let secp = Secp256k1::new();
            let root_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).expect("Failed to create root key");
            let derivation_path = DerivationPath::from_str(&config.derivation_path).expect("Failed to create derivation path");
            let derived_key = root_key.derive_priv(&secp, &derivation_path).expect("Failed to derive key");
            let address = Address::p2pkh(&derived_key.private_key.public_key(&secp), Network::Bitcoin);

            if address.to_string() == config.expected_address {
                info!("Found passphrase: {}", passphrase);
                std::process::exit(0);
            }

            pb.inc(1);
        });
    });

    pb.finish_with_message("Done");
}