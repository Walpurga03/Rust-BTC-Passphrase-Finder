use bitcoin_v027::secp256k1::Secp256k1;
use bip39::{Mnemonic, Language};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use memmap::Mmap;
use std::sync::Arc;
use log::{warn};
use std::fs::File;
use std::str::FromStr;
use bitcoin_v027::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin_v027::network::constants::Network;
use bitcoin_v027::util::address::Address;
use bitcoin_v027::Script;
use crate::config::Config;
use std::io::{self, ErrorKind};

fn get_address_format(address: &str) -> &str {
    if address.starts_with("1") {
        "legacy"
    } else if address.starts_with("3") {
        "p2sh"
    } else if address.starts_with("bc1q") && address.len() == 42 {
        "segwit"
    } else if address.starts_with("bc1q") && address.len() > 42 {
        "p2wsh"
    } else {
        panic!("Unsupported address format");
    }
}

pub fn find_passphrase(config: &Arc<Config>) -> Result<(), Box<dyn std::error::Error>> {
    // Open and memory-map the wordlist
    let file = File::open(&config.wordlist_path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let lines: Vec<&str> = mmap.split(|&byte| byte == b'\n')
        .map(|line| std::str::from_utf8(line).map_err(|_| io::Error::new(ErrorKind::InvalidData, "Invalid UTF-8")))
        .collect::<Result<Vec<&str>, io::Error>>()?;

    // Create a progress bar
    let pb = ProgressBar::new(lines.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    // Create a custom thread pool
    let pool = ThreadPoolBuilder::new().num_threads(config.num_threads).build()?;

    // Flag to check if passphrase is found
    let passphrase_found = Arc::new(std::sync::atomic::AtomicBool::new(false));

    // Determine the address format
    let address_format = get_address_format(&config.expected_address);
    println!("Address format: {}", address_format);

    // Perform parallel processing within the custom thread pool
    pool.install(|| {
        lines.par_iter().for_each(|&passphrase| {
            let mnemonic = Mnemonic::parse_in(Language::English, &config.seed_phrase).expect("Failed to create mnemonic");
            let seed = mnemonic.to_seed(passphrase);
            let secp = Secp256k1::new();
            let root_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).expect("Failed to create root key");

            // Define the derivation paths based on the address format and the number of paths to search
            let derivation_paths: Vec<DerivationPath> = match address_format {
                "legacy" => (0..config.address_paths_to_search).map(|i| DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", i)).expect("Failed to create derivation path")).collect(),
                "p2sh" => (0..config.address_paths_to_search).map(|i| DerivationPath::from_str(&format!("m/49'/0'/0'/0/{}", i)).expect("Failed to create derivation path")).collect(),
                "segwit" => (0..config.address_paths_to_search).map(|i| DerivationPath::from_str(&format!("m/84'/0'/0'/0/{}", i)).expect("Failed to create derivation path")).collect(),
                "p2wsh" => (0..config.address_paths_to_search).map(|i| DerivationPath::from_str(&format!("m/48'/0'/0'/2/{}", i)).expect("Failed to create derivation path")).collect(),
                _ => panic!("Unsupported address format"),
            };

            for derivation_path in derivation_paths {
                let derived_key = root_key.derive_priv(&secp, &derivation_path).expect("Failed to derive key");

                let address = match address_format {
                    "legacy" => Address::p2pkh(&derived_key.private_key.public_key(&secp), Network::Bitcoin),
                    "p2sh" => Address::p2shwpkh(&derived_key.private_key.public_key(&secp), Network::Bitcoin).expect("Failed to create P2SH address"),
                    "segwit" => Address::p2wpkh(&derived_key.private_key.public_key(&secp), Network::Bitcoin).expect("Failed to create SegWit address"),
                    "p2wsh" => {
                        let script = Script::new_v0_wpkh(&derived_key.private_key.public_key(&secp).wpubkey_hash().expect("Failed to create WPubkeyHash"));
                        let address_p2wsh = Address::p2wsh(&script, Network::Bitcoin);
                        address_p2wsh
                    },
                    _ => panic!("Unsupported address format"),
                };

                if address.to_string() == config.expected_address {
                    println!("\n===============================");
                    println!("🎉 HURRA! Passphrase found! 🎉");
                    println!("===============================");
                    println!("🔑 Passphrase: {}", passphrase);
                    println!("📬 Address format: {}", address_format);
                    println!("===============================");
                    println!("✨ If you found my program helpful, I would greatly appreciate a donation via Bitcoin Lightning!");
                    println!("⚡ Lightning address: aldobarazutti@getalby.com");
                    println!("🙏 Thank you very much!");
                    println!("📬 If you want to contact me, you can find me on Nostr!");
                    println!("🔗 npub: npub1hht9umpeet75w55uzs9lq6ksayfpcvl9lk64hye75j0yj4husq5ss8xsry");
                    println!("===============================");
                    passphrase_found.store(true, std::sync::atomic::Ordering::SeqCst);
                    std::process::exit(0);
                }
            }

            pb.inc(1);
        });
    });

    pb.finish_with_message("Done");

    // Check if passphrase was found
    if !passphrase_found.load(std::sync::atomic::Ordering::SeqCst) {
        warn!("Passphrase not found.");
        println!("\n===============================");
        println!("⚠️ Oops! Passphrase not found ⚠️");
        println!("===============================");
        println!("📬 Address format: {}", address_format);
        println!("===============================");
    }

    Ok(())
}