use bitcoin_v028::secp256k1::{Secp256k1, PublicKey};
use bitcoin_v028::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin_v028::network::constants::Network;
use bitcoin_v028::util::address::Address;
use bitcoin_v028::util::taproot::TaprootBuilder;
use bitcoin_v028::XOnlyPublicKey;
use bip39::Mnemonic;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::config::Config;

fn derive_taproot_key(master_key: &ExtendedPrivKey) -> XOnlyPublicKey {
    let secp = Secp256k1::new();
    let path: DerivationPath = "m/86'/0'/0'/0/0".parse().unwrap();
    let derived_key = master_key.derive_priv(&secp, &path).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &derived_key.private_key);
    XOnlyPublicKey::from(public_key)
}

pub fn find_taproot_passphrase(config: &Config) {
    let mnemonic = Mnemonic::from_str(&config.seed_phrase).expect("Invalid seed phrase");
    let secp = Secp256k1::new();

    // Passphrasen aus der Wortliste lesen
    let file = File::open(&config.wordlist_path).expect("Failed to open wordlist file");
    let reader = BufReader::new(file);
    let passphrases: Vec<String> = reader.lines().map(|line| line.expect("Failed to read line")).collect();

    for passphrase in passphrases {
        let seed = mnemonic.to_seed(&passphrase);
        let root_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).expect("Failed to create root key");

        let xonly_pubkey = derive_taproot_key(&root_key);
        let taproot_output_key = TaprootBuilder::new()
            .finalize(&secp, xonly_pubkey)
            .unwrap();
        let taproot_address = Address::p2tr_tweaked(taproot_output_key.output_key(), Network::Bitcoin);

        if taproot_address.to_string() == config.expected_address {
            println!("Found matching passphrase: {}", passphrase);
            return;
        }
    }

    println!("No matching passphrase found.");
}