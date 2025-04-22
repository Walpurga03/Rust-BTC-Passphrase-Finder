# Regex-Muster für Bitcoin Passphrase Finder


### Grundlagen der Regex-Syntax

Ein Regex-Muster besteht aus normalen Zeichen und speziellen Symbolen, die Regeln für die Erzeugung von Passphrasen definieren:

#### Zeichenklassen:
- `[A-Z]` - Erzeugt **einen** Großbuchstaben (A, B, C, ..., Z)
- `[a-z]` - Erzeugt **einen** Kleinbuchstaben (a, b, c, ..., z)
- `[0-9]` - Erzeugt **eine** Ziffer (0, 1, 2, ..., 9)
- `[abc]` - Erzeugt **ein** Zeichen aus dieser Auswahl (a, b oder c)
- `[!@#$%]` - Erzeugt **ein** Sonderzeichen aus dieser Auswahl (!, @, #, $, oder %)

#### Wiederholungsangaben:
- `{n}` - Genau n-mal wiederholen
- `{n,m}` - Zwischen n und m mal wiederholen
- `{n;}` - Mindestens n-mal wiederholen

#### Konstante Zeichen:
- Normale Buchstaben oder Symbole bleiben unverändert (z.B. "us", "mz", "QQ")

### Beispiele erklärt

#### 1. `[A-Z][0-9]{2}`
- Erzeugt: "A01", "B42", "Z99", "G27"

#### 2. `Q[a-z]{3}[0-9]`
- Erzeugt: "Qabc5", "Qxyz7", "Qdef9"

#### 3. [A-Z]us[A-Z]t[0-9]mz[A-Z]{2}[0-9]{2}QQ
- [A-Z] - Ein zufälliger Großbuchstabe
- us - Die konstante Zeichenfolge "us"
- [A-Z] - Ein weiterer zufälliger Großbuchstabe
- t - Der konstante Buchstabe "t"
- [0-9] - Eine zufällige Ziffer
- mz - Die konstante Zeichenfolge "mz"
- [A-Z]{2} - Zwei zufällige Großbuchstaben
- [0-9]{2} - Zwei zufällige Ziffern
- QQ - Die konstante Zeichenfolge "QQ"
- Beispiel: "AusBt3mzXY42QQ"

## Weitere nützliche Beispiele
# 4-stellige PIN
# [0-9]{4}
# Erzeugt: "0123", "9876", "1234", usw.

# Bitcoin-Wallet mit BTC-Präfix und gemischten Zeichen
# BTC[A-Za-z]{3}[0-9]{2,4}
# Erzeugt: "BTCabc12", "BTCxYz456", usw.

# Passwort mit Sonderzeichen
# pass[a-z]{3}[!@#$%]{1,2}
# Erzeugt: "passabc!", "passxyz@#", usw.

# Jahresbasiertes Passwort mit Sonderzeichen
# [A-Z][a-z]{3,6}20[0-9]{2}[!@#$]

# Complex password with letters, numbers and special characters
# [A-Z][a-z]{2,4}[!@#$%&*]{1,2}[0-9]{2,3}
# Erzeugt: "Katze2022!", "Hunde2019@", usw.

# Komplexes Passwort mit Buchstaben, Zahlen und Sonderzeichen
# [A-Z][a-z]{2,4}[!@#$%&*]{1,2}[0-9]{2,3}
# Erzeugt: "Abcd@12", "Test#$789", usw.

