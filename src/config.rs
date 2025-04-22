use serde::Deserialize;

/// Configuration structure for the Bitcoin Passphrase Finder
#[derive(Deserialize)]
pub struct Config {
    /// Seed phrase used for generating the passphrase
    pub seed_phrase: String,
    /// Expected Bitcoin address that should match the generated passphrase
    pub expected_address: String,
    /// Path to the file where the generated passphrases will be saved
    pub wordlist_path: String,
    /// Number of threads to be used for generation
    pub num_threads: usize,
    /// Regex pattern for passphrase generation
    /// Examples:
    /// - [A-Z][0-9]{2} - Uppercase letter followed by two digits
    /// - Q[a-z]{3}[0-9] - Letter Q followed by three lowercase letters and one digit
    pub passphrase: String,
    /// Number of address paths to search (1, 2, or 3)
    pub address_paths_to_search: usize,
}