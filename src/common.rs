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
use std::io::{self, ErrorKind};
use crate::config::Config;

pub fn derive_key(master_key: &ExtendedPrivKey, path: &str) -> Result<ExtendedPrivKey, bitcoin_v028::util::bip32::Error> {
    let secp = Secp256k1::new();
    let derivation_path: DerivationPath = path.parse()?;
    master_key.derive_priv(&secp, &derivation_path)
}

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));
    pb
}

pub fn load_wordlist(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let wordlist = mmap.split(|&byte| byte == b'\n')
        .map(|line| std::str::from_utf8(line).map_err(|_| io::Error::new(ErrorKind::InvalidData, "Invalid UTF-8")).map(|s| s.to_string()))
        .collect::<Result<Vec<String>, io::Error>>()?;
    Ok(wordlist)
}