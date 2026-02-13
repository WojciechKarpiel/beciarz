# Beciarz

Tłumacz sposobów zapisu języka polskiego.
Obczaj jak działa [tu](https://www.wojciechkarpiel.pl/beciarz/).

## W przegladarce

### Jak uruchomić u siebie

```
cargo install wasm-pack
wasm-pack build wasm --target web
python3 -m http.server -d wasm
```

### Szczegóły techniczne

Aplikacja działa w całości w przeglądarce.


### Czemu Rust?

Bo chciałem obczaić jak działa kompilacja Rusta do WASMa

## W wierszu poleceń

### Jak uruchomić u siebie

```
cargo build -p beciarz-cli --release
target/release/beciarz-cli --help
target/release/beciarz-cli "pozdrawiam cieplutko"
target/release/beciarz-cli -n -ig -o oficjalny << EOF
ποζδραβάμ τέπλύτκο
EOF
```
