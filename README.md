## Übersicht
Dieses Programm ist ein Bitcoin-Passphrase-Finder, der eine Liste möglicher Passphrasen durchläuft, um diejenige zu finden, die zu einer bestimmten Bitcoin-Adresse passt. Es verwendet parallele Verarbeitung, um die Passphrasen effizient zu überprüfen.

## Funktionsweise
1. **Konfiguration**: Das Programm liest eine Konfigurationsdatei ([`config.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Flinux%2Fprojects%2Frust-btc-passphrase-finder%2Fconfig.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%2200aeeb26-5c33-4a0d-a0da-e705caef91db%22%5D "/home/linux/projects/rust-btc-passphrase-finder/config.toml")), die die Seed-Phrase, die erwartete Bitcoin-Adresse, den Pfad zur Wortliste und die Anzahl der Threads für die parallele Verarbeitung enthält.
2. **Lesen der Wortliste**: Die Wortliste wird geöffnet und speicherabbildet. Jede Zeile der Datei wird in einen Vektor von Zeichenketten umgewandelt.
3. **Fortschrittsbalken**: Ein Fortschrittsbalken wird erstellt, um den Fortschritt des Brute-Force-Prozesses anzuzeigen.
4. **Parallele Verarbeitung**: Die Wortliste wird parallel verarbeitet. Für jede Passphrase wird ein Mnemonic-Objekt erstellt und ein Seed daraus generiert. Mit diesem Seed wird ein privater Schlüssel abgeleitet, und aus diesem Schlüssel wird eine Bitcoin-Adresse generiert.
5. **Überprüfung**: Wenn die generierte Adresse mit der erwarteten Adresse übereinstimmt, wird die Passphrase protokolliert und das Programm beendet. Der Fortschrittsbalken wird bei jeder Iteration aktualisiert und am Ende mit einer Nachricht abgeschlossen.

## Unterstützte Adressformate
Das Programm unterstützt die folgenden vier Bitcoin-Adressformate:
1. **Legacy (P2PKH)**: Adressen, die mit `1` beginnen.
2. **Pay-to-Script-Hash (P2SH)**: Adressen, die mit `3` beginnen.
3. **Native SegWit (P2WPKH)**: Adressen, die mit `bc1q` beginnen und 42 Zeichen lang sind.
4. **Pay-to-Witness-Script-Hash (P2WSH)**: Adressen, die mit `bc1q` beginnen und länger als 42 Zeichen sind.

## Voraussetzungen
- Linux
- Rust-Umgebung
- Abhängigkeiten, die in der [`Cargo.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Flinux%2Fprojects%2Frust-btc-passphrase-finder%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%2200aeeb26-5c33-4a0d-a0da-e705caef91db%22%5D "/home/linux/projects/rust-btc-passphrase-finder/Cargo.toml")-Datei definiert sind

## Installation
### Mit Internetzugang
1. **Rust installieren**: Wenn Rust noch nicht installiert ist, verwenden Sie den folgenden Befehl:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Projekt klonen**:
   ```
   git clone https://github.com/Walpurga03/Rust-BTC-Passphrase-Finder.git
   cd Rust-BTC-Passphrase-Finder
   ```
3. **Projekt bauen**:
   ```
   cargo build --release
   ```

### Ohne Internetzugang (Offline-Nutzung)
#### Auf einem PC mit Internetverbindung
1. **GitHub-Repository auf einen USB-Stick klonen**:
   ```
   git clone https://github.com/Walpurga03/Rust-BTC-Passphrase-Finder.git /pfad/zum/usb-stick
   cd /pfad/zum/usb-stick/Rust-BTC-Passphrase-Finder
   ```
2. **Rustup und Cargo installieren** (falls noch nicht installiert):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
3. **Projekt bauen und alle Abhängigkeiten herunterladen**:
   ```
   cd Rust-BTC-Passphrase-Finder
   cargo build --release
   ```
4. **Rust-Toolchain auf den USB-Stick kopieren**:
   ```
   cp -r $HOME/.rustup /pfad/zum/usb-stick/Rust-BTC-Passphrase-Finder/rustup
   cp -r $HOME/.cargo /pfad/zum/usb-stick/Rust-BTC-Passphrase-Finder/cargo
   ```

#### Auf einem PC ohne Internetverbindung
1. **USB-Stick einstecken und zum Repository-Verzeichnis navigieren**:
   ```
   cd /pfad/zum/usb-stick/Rust-BTC-Passphrase-Finder
   ```
2. **Umgebungsvariablen laden**:
   ```
   source setup_env.sh
   ls -l ./target/release/rust_btc_passphrase_finder
   chmod +x ./target/release/rust_btc_passphrase_finder
   ```
3. **Programm ausführen**:
   ```
   ./target/release/rust_btc_passphrase_finder
   ```

## Hinweise
- Stellen Sie sicher, dass die Wortliste im UTF-8-Format vorliegt.
- Parallele Verarbeitung kann die CPU stark auslasten. Passen Sie die Anzahl der Threads nach Bedarf an.
