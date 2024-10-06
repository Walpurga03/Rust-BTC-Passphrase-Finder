## Übersicht
Dieses Programm ist ein Bitcoin-Passphrase-Finder, der eine Liste möglicher Passphrasen durchläuft, um diejenige zu finden, die zu einer bestimmten Bitcoin-Adresse passt. Es verwendet parallele Verarbeitung, um die Passphrasen effizient zu überprüfen.

## Problemstellung
Bitcoin-Adressen werden oft durch eine Kombination aus einer Seed-Phrase und einer Passphrase generiert. Diese Passphrase kann als zusätzliche Sicherheitsschicht dienen, um den Zugriff auf die Bitcoin-Wallet zu schützen. Wenn die Passphrase jedoch verloren geht oder vergessen wird, kann der Zugriff auf die Wallet und die darin enthaltenen Bitcoins unmöglich werden.

Das Problem, das dieses Programm löst, besteht darin, die verlorene oder vergessene Passphrase zu finden, indem es eine Liste möglicher Passphrasen durchläuft und überprüft, ob sie zu einer bestimmten Bitcoin-Adresse passt. Dies ist besonders nützlich für Benutzer, die ihre Passphrase vergessen haben, aber noch die Seed-Phrase und die erwartete Bitcoin-Adresse kennen.


## Video-Demonstration
https://old.bitchute.com/video/849qfl1yiVqf/


## Funktionsweise
1. **Konfiguration**: Das Programm liest eine Konfigurationsdatei ([`config.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Flinux%2Fprojects%2Frust-btc-passphrase-finder%2Fconfig.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%2200aeeb26-5c33-4a0d-a0da-e705caef91db%22%5D "/home/linux/projects/rust-btc-passphrase-finder/config.toml")), die die Seed-Phrase, die erwartete Bitcoin-Adresse, den Pfad zur Wortliste und die Anzahl der Threads für die parallele Verarbeitung enthält.
2. **Lesen der Wortliste**: Die Wortliste wird geöffnet und speicherabbildet. Jede Zeile der Datei wird in einen Vektor von Zeichenketten umgewandelt.
3. **Fortschrittsbalken**: Ein Fortschrittsbalken wird erstellt, um den Fortschritt des Brute-Force-Prozesses anzuzeigen.
4. **Parallele Verarbeitung**: Die Wortliste wird parallel verarbeitet. Für jede Passphrase wird ein Mnemonic-Objekt erstellt und ein Seed daraus generiert. Mit diesem Seed wird ein privater Schlüssel abgeleitet, und aus diesem Schlüssel wird eine Bitcoin-Adresse generiert.
5. **Überprüfung**: Wenn die generierte Adresse mit der erwarteten Adresse übereinstimmt, wird die Passphrase protokolliert und das Programm beendet. Der Fortschrittsbalken wird bei jeder Iteration aktualisiert und am Ende mit einer Nachricht abgeschlossen.

## Unterstützte Adressformate
Das Programm unterstützt die folgenden fünf Bitcoin-Adressformate:
1. **Legacy (P2PKH)**: Adressen, die mit `1` beginnen.
2. **Pay-to-Script-Hash (P2SH)**: Adressen, die mit `3` beginnen.
3. **Native SegWit (P2WPKH)**: Adressen, die mit `bc1q` beginnen und 42 Zeichen lang sind.
4. **Pay-to-Witness-Script-Hash (P2WSH)**: Adressen, die mit `bc1q` beginnen und länger als 42 Zeichen sind.
5. **Taproot (P2TR)**: Adressen, die mit `bc1p` beginnen.

## Konfigurationsoptionen
Die Konfigurationsdatei [`config.toml`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Flinux%2Fprojects%2Frust-btc-passphrase-finder%2Fconfig.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%2200aeeb26-5c33-4a0d-a0da-e705caef91db%22%5D "/home/linux/projects/rust-btc-passphrase-finder/config.toml")) enthält die folgenden Optionen:
- `seed_phrase`: Die Seed-Phrase, die zur Generierung der Passphrase verwendet wird.
- `expected_address`: Die erwartete Bitcoin-Adresse, die mit der generierten Passphrase übereinstimmen soll.
- `wordlist_path`: Der Pfad zur Datei mit der Wortliste.
- `num_threads`: Die Anzahl der Threads, die für die parallele Verarbeitung verwendet werden sollen.
- `passphrase`: Das Template für die Passphrase mit Platzhaltern.
- `uppercase`: Zeichen, die für den Platzhalter `uppercase` verwendet werden sollen.
- `lowercase`: Zeichen, die für den Platzhalter `lowercase` verwendet werden sollen.
- `digits`: Zeichen, die für den Platzhalter `digits` verwendet werden sollen.
- `special`: Zeichen, die für den Platzhalter `special` verwendet werden sollen.
- `address_paths_to_search`: Die Anzahl der zu durchsuchenden Adresspfade (1, 2 oder 3).

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

## Testen des Programms
Um das Programm zu testen, können Sie für jeden der fünf Adresstypen (Legacy, P2SH, SegWit, P2WSH, Taproot) drei Adressen mit zufälligen Seed-Phrasen und Passphrasen generieren. Diese Adressen und Passphrasen können Sie dann in die Wortliste und die Konfigurationsdatei eintragen, um das Programm zu testen.

## Passphrase-Generator
Das Programm enthält auch einen Passphrase-Generator, der eine Wortliste mit vorhandenen Buchstaben sowie Groß- und Kleinbuchstaben, Zahlen und Sonderzeichen generieren kann. Diese Funktion kann über das Menü ausgewählt werden.

## Menüoptionen
1. Generate Addresses: Generiert Bitcoin-Adressen basierend auf der Seed-Phrase und den Ableitungspfaden.
2. Find Passphrase: Durchsucht die Wortliste nach der Passphrase, die zur erwarteten Bitcoin-Adresse passt.
3. Generate Passphrases: Generiert eine Wortliste mit Passphrasen basierend auf den angegebenen Zeichen und Platzhaltern.