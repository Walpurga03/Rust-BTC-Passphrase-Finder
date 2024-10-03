use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub seed_phrase: String,
    pub expected_address: String,
    pub wordlist_path: String,
    pub num_threads: usize,
    pub passphrase: String,
    pub uppercase: String,
    pub lowercase: String,
    pub digits: String,
    pub special: String,
    pub address_paths_to_search: usize,  // Add this line
}