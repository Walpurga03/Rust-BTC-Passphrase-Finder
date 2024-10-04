use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub seed_phrase: String,
    pub expected_address: String,
    pub wordlist_path: String,
    pub num_threads: usize,
    pub passphrase: String,
    pub upp1: String,
    pub upp2: String,
    pub low1: String,
    pub low2: String,
    pub dig1: String,
    pub dig2: String,
    pub spe1: String,
    pub spe2: String,
    pub address_paths_to_search: usize,
}