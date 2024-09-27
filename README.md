# Bitcoin Passphrase Finder (with Offline Capability)

## Overview
This program is a Bitcoin passphrase finder that iterates through a list of possible passphrases to find the one that matches a specific Bitcoin address. It uses parallel processing to efficiently check the passphrases.

## How It Works
1. **Configuration**: The program reads a configuration file (`config.toml`) that contains the seed phrase, derivation path, expected Bitcoin address, path to the wordlist, and the number of threads for parallel processing.

2. **Reading the Wordlist**: The wordlist is opened and memory-mapped. Each line of the file is converted into a vector of strings.

3. **Progress Bar**: A progress bar is created to display the progress of the brute-force process.

4. **Parallel Processing**: The wordlist is processed in parallel. For each passphrase, a mnemonic object is created and a seed is generated from it. Using this seed, a private key is derived, and from this key, a Bitcoin address is generated.

5. **Verification**: If the generated address matches the expected address, the passphrase is logged, and the program exits. The progress bar is updated with each iteration and is completed with a message at the end.

## Prerequisites
- Rust environment
- Dependencies defined in the `Cargo.toml` file

## Installation
1. **Install Rust**: If Rust is not already installed, it can be installed using the following command:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the Project:
   ```
   git clone https://github.com/Walpurga03/Rust-BTC-Passphrase-Finder.git
   cd Rust-BTC-Passphrase-Finder
   ```

3. cargo build
   ```
   cargo build --release
   ```
## Configuration
- Create a config.toml file in the project directory with the following content:
  seed_phrase = "your seed phrase here"
  derivation_path = "m/44'/0'/0'/0/0"
  expected_address = "your expected Bitcoin address here"
  wordlist_path = "path/to/wordlist.txt"
  num_threads = 4

## Execution
To run the program, use the following command:
   ```
   cargo run --release 
   ```
The program will iterate through the wordlist and search for the matching passphrase. The progress will be displayed using a progress bar. If the matching passphrase is found, it will be logged, and the program will exit.

## Notes
- Ensure that the wordlist is in UTF-8 format.
- Parallel processing can heavily utilize the CPU. Adjust the number of threads as needed.

## Instructions for Offline Usage
### Prerequisites
- A PC with an internet connection
- A USB stick
- A PC without an internet connection

### Steps on the PC with Internet Connection
1. Clone the GitHub repository to the USB stick:
      ```
   git clone https://github.com/Walpurga03/Rust-BTC-Passphrase-Finder.git
   cd /Rust-BTC-Passphrase-Finder
   ```
2. Install Rustup and Cargo (if not already installed):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
2. Build the project and download all dependencies:
   ```
   cargo build --release
   ```
3. Copy the Rust toolchain to the USB stick:
   ```
   cp -r $HOME/.rustup /path/to/usb-2/Rust-BTC-Passphrase-Finder/rustup
   cp -r $HOME/.cargo /path/to/usb-2/Rust-BTC-Passphrase-Finder/cargo
   ```
### Steps on the PC without Internet Connection
1. Insert the USB stick and navigate to the repository directory:
   ``` 
   cd /path/to/usb-2/Rust-BTC-Passphrase-Finder
   ```
2. Load the environment variables:
   ```
   source setup_env.sh
   ```
3. Run the program:
   ```
   ./target/release/rust_btc_passphrase_finder
   ```
