use bip39::Mnemonic;
use rand::Rng;
use rand::rngs::OsRng;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::Script;
use std::str::FromStr;

pub fn generate_address() {
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

    // Legacy-Adresse (P2PKH)
    let derivation_path_legacy = DerivationPath::from_str("m/44'/0'/0'/0/0").expect("Failed to create derivation path");
    let derived_key_legacy = root_key.derive_priv(&secp, &derivation_path_legacy).expect("Failed to derive key");
    let address_legacy = Address::p2pkh(&derived_key_legacy.private_key.public_key(&secp), Network::Bitcoin);
    println!("Generated legacy address: {}", address_legacy);

    // Native SegWit-Adresse (P2WPKH)
    let derivation_path_segwit = DerivationPath::from_str("m/84'/0'/0'/0/0").expect("Failed to create derivation path");
    let derived_key_segwit = root_key.derive_priv(&secp, &derivation_path_segwit).expect("Failed to derive key");
    let address_segwit = Address::p2wpkh(&derived_key_segwit.private_key.public_key(&secp), Network::Bitcoin).expect("Failed to create SegWit address");
    println!("Generated SegWit address: {}", address_segwit);

    // Pay-to-Script-Hash (P2SH)
    let derivation_path_p2sh = DerivationPath::from_str("m/49'/0'/0'/0/0").expect("Failed to create derivation path");
    let derived_key_p2sh = root_key.derive_priv(&secp, &derivation_path_p2sh).expect("Failed to derive key");
    let address_p2sh = Address::p2shwpkh(&derived_key_p2sh.private_key.public_key(&secp), Network::Bitcoin).expect("Failed to create P2SH address");
    println!("Generated P2SH address: {}", address_p2sh);

    // Pay-to-Witness-Script-Hash (P2WSH)
    let derivation_path_p2wsh = DerivationPath::from_str("m/48'/0'/0'/2/0").expect("Failed to create derivation path");
    let derived_key_p2wsh = root_key.derive_priv(&secp, &derivation_path_p2wsh).expect("Failed to derive key");
    let script = Script::new_v0_wpkh(&derived_key_p2wsh.private_key.public_key(&secp).wpubkey_hash().expect("Failed to create WPubkeyHash"));
    let address_p2wsh = Address::p2wsh(&script, Network::Bitcoin);
    println!("Generated P2WSH address: {}", address_p2wsh);
}