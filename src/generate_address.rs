use bip39::Mnemonic;
use rand::Rng;
use rand::rngs::OsRng;
use std::str::FromStr;
use std::error::Error;

/// Derives the Taproot key using the given master key.
fn derive_taproot_key_v028(master_key: &bitcoin_v028::util::bip32::ExtendedPrivKey, index: u32) -> Result<bitcoin_v028::secp256k1::PublicKey, Box<dyn Error>> {
    let secp = bitcoin_v028::secp256k1::Secp256k1::new();
    let path = bitcoin_v028::util::bip32::DerivationPath::from_str(&format!("m/86'/0'/0'/0/{}", index))?;
    let derived_key = master_key.derive_priv(&secp, &path)?;
    Ok(bitcoin_v028::secp256k1::PublicKey::from_secret_key(&secp, &derived_key.private_key))
}

/// Generates all types of Bitcoin addresses and prints them.
pub fn generate_all_addresses() -> Result<(), Box<dyn std::error::Error>> {
    use bitcoin_v028::secp256k1::Secp256k1 as Secp256k1_v028;
    use bitcoin_v028::util::bip32::ExtendedPrivKey as ExtendedPrivKey_v028;
    use bitcoin_v028::network::constants::Network as Network_v028;
    use bitcoin_v028::util::address::Address as Address_v028;
    use bitcoin_v028::util::taproot::TaprootBuilder;
    use bitcoin_v028::XOnlyPublicKey;
    use bitcoin_v027::secp256k1::Secp256k1 as Secp256k1_v027;
    use bitcoin_v027::util::bip32::{ExtendedPrivKey as ExtendedPrivKey_v027, DerivationPath};
    use bitcoin_v027::network::constants::Network as Network_v027;
    use bitcoin_v027::util::address::Address as Address_v027;
    use bitcoin_v027::Script;

    // Generate a random seed phrase
    let mut entropy = [0u8; 16]; // 128 bits of entropy for 12 words
    OsRng.fill(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    let seed_phrase = mnemonic.to_string();
    println!("Generated seed phrase: {}", seed_phrase);

    // Generate a random passphrase
    let passphrase: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    println!("Generated passphrase: {}", passphrase);

    // Generate the seed from the seed phrase and passphrase
    let seed = mnemonic.to_seed(&passphrase);

    // Taproot address (bitcoin_v028)
    let secp_v028 = Secp256k1_v028::new();
    let root_key_v028 = ExtendedPrivKey_v028::new_master(Network_v028::Bitcoin, &seed)?;

    // Legacy address (P2PKH)
    let secp_v027 = Secp256k1_v027::new();
    let root_key_v027 = ExtendedPrivKey_v027::new_master(Network_v027::Bitcoin, &seed)?;

    println!("Extended Private Key: {}", root_key_v027);
    println!("\nGenerated Addresses:");
    println!("====================");

    for i in 0..3 {
        // Taproot address
        let taproot_pubkey = derive_taproot_key_v028(&root_key_v028, i)?;
        let xonly_pubkey = XOnlyPublicKey::from(taproot_pubkey);  // Convert to XOnlyPublicKey
        let taproot_output_key = TaprootBuilder::new()
            .finalize(&secp_v028, xonly_pubkey)?;
        let taproot_address = Address_v028::p2tr_tweaked(taproot_output_key.output_key(), Network_v028::Bitcoin);

        // Legacy address (P2PKH)
        let derivation_path_legacy = DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", i))?;
        let derived_key_legacy = root_key_v027.derive_priv(&secp_v027, &derivation_path_legacy)?;
        let address_legacy = Address_v027::p2pkh(&derived_key_legacy.private_key.public_key(&secp_v027), Network_v027::Bitcoin);

        // Native SegWit address (P2WPKH)
        let derivation_path_segwit = DerivationPath::from_str(&format!("m/84'/0'/0'/0/{}", i))?;
        let derived_key_segwit = root_key_v027.derive_priv(&secp_v027, &derivation_path_segwit)?;
        let address_segwit = Address_v027::p2wpkh(&derived_key_segwit.private_key.public_key(&secp_v027), Network_v027::Bitcoin)?;

        // Pay-to-Script-Hash (P2SH)
        let derivation_path_p2sh = DerivationPath::from_str(&format!("m/49'/0'/0'/0/{}", i))?;
        let derived_key_p2sh = root_key_v027.derive_priv(&secp_v027, &derivation_path_p2sh)?;
        let address_p2sh = Address_v027::p2shwpkh(&derived_key_p2sh.private_key.public_key(&secp_v027), Network_v027::Bitcoin)?;

        // Pay-to-Witness-Script-Hash (P2WSH)
        let derivation_path_p2wsh = DerivationPath::from_str(&format!("m/48'/0'/0'/2/{}", i))?;
        let derived_key_p2wsh = root_key_v027.derive_priv(&secp_v027, &derivation_path_p2wsh)?;
        let wpkh = derived_key_p2wsh.private_key.public_key(&secp_v027).wpubkey_hash().ok_or("Failed to create WPubkeyHash")?;
        let script = Script::new_v0_wpkh(&wpkh);
        let address_p2wsh = Address_v027::p2wsh(&script, Network_v027::Bitcoin);

        // Print the addresses in a structured format
        println!("Address Path {}:", i + 1);
        println!("{:<15} {}", "Legacy(P2PKH):", address_legacy);
        println!("{:<15} {}", "P2SH:", address_p2sh);
        println!("{:<15} {}", "P2WSH:", address_p2wsh);
        println!("{:<15} {}", "SegWit(P2WPKH):", address_segwit);
        println!("{:<15} {}", "Taproot:", taproot_address);
        println!("--------------------");
    }

    Ok(())
}