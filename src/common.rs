use bitcoin_v028::secp256k1::{Secp256k1, PublicKey};
use bitcoin_v028::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin_v028::network::constants::Network;
use bitcoin_v028::util::address::Address;
use bitcoin_v028::XOnlyPublicKey;
use bip39::{Mnemonic, Language};
use indicatif::{ProgressBar, ProgressStyle};
use memmap::Mmap;
use std::sync::Arc;
use std::fs::File;
use std::str::FromStr;
use crate::config::Config;

pub fn derive_key(master_key: &ExtendedPrivKey, path: &str) -> ExtendedPrivKey {
    let secp = Secp256k1::new();
    let derivation_path: DerivationPath = path.parse().unwrap();
    master_key.derive_priv(&secp, &derivation_path).unwrap()
}

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));
    pb
}

pub fn load_wordlist(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open wordlist.txt");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to map the file") };
    mmap.split(|&byte| byte == b'\n')
        .map(|line| std::str::from_utf8(line).expect("Invalid UTF-8").to_string())
        .collect()
}