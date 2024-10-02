use bitcoin_v028::secp256k1::{Secp256k1, PublicKey};
use bitcoin_v028::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin_v028::network::constants::Network;
use bitcoin_v028::util::address::Address;
use bitcoin_v028::util::taproot::TaprootBuilder;
use bitcoin_v028::XOnlyPublicKey;
use bip39::Mnemonic;
use rand::rngs::OsRng;
use rand::Rng;
use std::str::FromStr;

// Ableitung nach BIP-86 (m/86'/0'/0'/0/0)
fn derive_taproot_key(master_key: &ExtendedPrivKey) -> PublicKey {
    let secp = Secp256k1::new();
    let path = DerivationPath::from_str("m/86'/0'/0'/0/0").unwrap();
    let derived_key = master_key.derive_priv(&secp, &path).unwrap();
    PublicKey::from_secret_key(&secp, &derived_key.private_key)
}

// Taproot-Adresse erstellen
pub fn generate_taproot_address() {
    // Generiere eine zufällige Seed-Phrase
    let mut entropy = [0u8; 16]; // 128 bits of entropy for 12 words
    OsRng.fill(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&entropy).expect("Failed to generate mnemonic");
    let seed_phrase = mnemonic.to_string();
    println!("Generated seed phrase: {}", seed_phrase);

    // Generiere eine zufällige Passphrase
    let passphrase: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    println!("Generated passphrase: {}", passphrase);

    // Erzeuge den Seed aus der Seed-Phrase und der Passphrase
    let seed = mnemonic.to_seed(&passphrase);
    let secp = Secp256k1::new();
    let root_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).expect("Failed to create root key");
    println!("Generated ExtendedPrivKey: {}", root_key);

    // Ableitung des Taproot-Schlüssels
    let taproot_pubkey = derive_taproot_key(&root_key);
    let xonly_pubkey = XOnlyPublicKey::from(taproot_pubkey);  // Umwandlung in XOnlyPublicKey
    let taproot_output_key = TaprootBuilder::new()
        .finalize(&secp, xonly_pubkey)
        .unwrap();
    let taproot_address = Address::p2tr_tweaked(taproot_output_key.output_key(), Network::Bitcoin);
    println!("Taproot Address: {}", taproot_address);
}