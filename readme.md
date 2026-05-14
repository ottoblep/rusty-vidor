# Rust MKR Vidor Example

## Rust Build
### Build binary
```bash
cargo build
```

## Arduino-CLI
### Install board 
```bash
arduino-cli core install arduino:samd
```
### Flash binary
```bash
arduino-cli upload -t -p /dev/ttyACM0 -i target/output.bin -b arduino:samd:mkrvidor4000
```