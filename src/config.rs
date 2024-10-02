use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub seed_phrase: String,
    pub expected_address: String,
    pub wordlist_path: String,
    pub num_threads: usize,
}