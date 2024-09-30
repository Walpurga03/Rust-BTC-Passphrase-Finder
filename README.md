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
### With Internet Access
1. **Install Rust**: If Rust is not already installed, use the following command:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Clone the Project**:
   ```
   git clone https://github.com/Walpurga03/Rust-BTC-Passphrase-Finder.git
   cd Rust-BTC-Passphrase-Finder
   ```
3. **Build the Project**:
   ```
   cargo build --release
   ```

### Without Internet Access (Offline Usage)
#### On a PC with Internet Connection
1. **Clone the GitHub Repository to a USB Stick**:
   ```
   git clone https://github.com/Walpurga03/Rust-BTC-Passphrase-Finder.git /path/to/usb-stick
   cd /path/to/usb-stick/Rust-BTC-Passphrase-Finder
   ```
2. **Install Rustup and Cargo** (if not already installed):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
3. **Build the Project and Download All Dependencies**:
   ```
   cargo build --release
   ```
4. **Copy the Rust Toolchain to the USB Stick**:
   ```
   cp -r $HOME/.rustup /path/to/usb-stick/Rust-BTC-Passphrase-Finder/rustup
   cp -r $HOME/.cargo /path/to/usb-stick/Rust-BTC-Passphrase-Finder/cargo
   ```

#### On a PC without Internet Connection
1. **Insert the USB Stick and Navigate to the Repository Directory**:
   ```
   cd /path/to/usb-stick/Rust-BTC-Passphrase-Finder
   ```
2. **Load the Environment Variables**:
   ```
   source setup_env.sh
   ```
3. **Run the Program**:
   ```
   ./target/release/rust_btc_passphrase_finder
   ```

## Configuration
- Create a `config.toml` file in the project directory with the following content:
  ```toml
  seed_phrase = "your seed phrase here"
  derivation_path = "m/44'/0'/0'/0/0"
  expected_address = "your expected Bitcoin address here"
  wordlist_path = "path/to/wordlist.txt"
  num_threads = 4
  ```

## Execution
To run the program, use the following command:
   ```
   cargo run --release 
   ```
For offline usage, execute the program with:
   ```
   ./target/release/rust_btc_passphrase_finder
   ```

The program will iterate through the wordlist and search for the matching passphrase. The progress will be displayed using a progress bar. If the matching passphrase is found, it will be logged, and the program will exit.

## Notes
- Ensure that the wordlist is in UTF-8 format.
- Parallel processing can heavily utilize the CPU. Adjust the number of threads as needed.