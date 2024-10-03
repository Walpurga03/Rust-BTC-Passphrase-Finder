use bitcoin_v028::secp256k1::{Secp256k1, PublicKey};
use bitcoin_v028::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin_v028::network::constants::Network;
use bitcoin_v028::util::address::Address;
use bitcoin_v028::util::taproot::TaprootBuilder;
use bitcoin_v028::XOnlyPublicKey;
use bip39::{Mnemonic, Language};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::Arc;
use std::fs::File;
use log::{warn};
use memmap::Mmap;
use crate::config::Config;

fn derive_taproot_key(master_key: &ExtendedPrivKey) -> XOnlyPublicKey {
    let secp = Secp256k1::new();
    let path: DerivationPath = "m/86'/0'/0'/0/0".parse().unwrap();
    let derived_key = master_key.derive_priv(&secp, &path).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &derived_key.private_key);
    XOnlyPublicKey::from(public_key)
}

pub fn find_taproot_passphrase(config: &Arc<Config>) {
    // Open and memory-map the wordlist
    let file = File::open(&config.wordlist_path).expect("Failed to open wordlist.txt");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to map the file") };
    let lines: Vec<&str> = mmap.split(|&byte| byte == b'\n')
        .map(|line| std::str::from_utf8(line).expect("Invalid UTF-8"))
        .collect();

    // Create a progress bar
    let pb = ProgressBar::new(lines.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    // Create a custom thread pool
    let pool = ThreadPoolBuilder::new().num_threads(config.num_threads).build().unwrap();

    // Flag to check if passphrase is found
    let passphrase_found = Arc::new(std::sync::atomic::AtomicBool::new(false));

    // Perform parallel processing within the custom thread pool
    pool.install(|| {
        lines.par_iter().for_each(|&passphrase| {
            let mnemonic = Mnemonic::parse_in(Language::English, &config.seed_phrase).expect("Failed to create mnemonic");
            let seed = mnemonic.to_seed(passphrase);
            let secp = Secp256k1::new();
            let root_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).expect("Failed to create root key");

            let xonly_pubkey = derive_taproot_key(&root_key);
            let taproot_output_key = TaprootBuilder::new()
                .finalize(&secp, xonly_pubkey)
                .unwrap();
            let taproot_address = Address::p2tr_tweaked(taproot_output_key.output_key(), Network::Bitcoin);

            if taproot_address.to_string() == config.expected_address {
                println!("\n===============================");
                println!("🎉 HURRA! Passphrase gefunden! 🎉");
                println!("===============================");
                println!(" Passphrase: {}", passphrase);
                println!("📬 Address format: taproot");
                println!("===============================");
                println!("✨ Wenn Sie mein Programm hilfreich fanden, würde ich mich riesig über eine Spende via Bitcoin Lightning freuen!");
                println!("⚡ Lightning-Adresse: aldobarazutti@getalby.com");
                println!("🙏 Vielen Dank!");
                println!("📬 Wenn Sie mich kontaktieren möchten, finden Sie mich auf Nostr!");
                println!("🔗 npub: npub1hht9umpeet75w55uzs9lq6ksayfpcvl9lk64hye75j0yj4husq5ss8xsry");
                println!("===============================");
                passphrase_found.store(true, std::sync::atomic::Ordering::SeqCst);
                std::process::exit(0);
            }

            pb.inc(1);
        });
    });

    pb.finish_with_message("Done");

    // Check if passphrase was found
    if !passphrase_found.load(std::sync::atomic::Ordering::SeqCst) {
        warn!("Passphrase not found.");
        println!("\n===============================");
        println!("⚠️ Oje! Passphrase nicht gefunden ⚠️");
        println!("===============================");
        println!("📬 Address format: taproot");
        println!("===============================");
    }
}