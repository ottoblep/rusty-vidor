# Rust MKR Vidor Example

### Enter Nix Devshell
```bash
nix develop
```

### Build Binary
```bash
cargo build
```

### Install board 
```bash
arduino-cli core install arduino:samd
```

### Flash binary
```bash
rust-objcopy -O binary target/thumbv6m-none-eabi/debug/rusty-vidor target/output.bin
arduino-cli upload -t -p /dev/ttyACM0 -i target/output.bin -b arduino:samd:mkrvidor4000
```