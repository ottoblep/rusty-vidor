rust-objcopy -O binary target/thumbv6m-none-eabi/debug/rusty-vidor target/output.bin

arduino-cli core install arduino:samd
arduino-cli upload -t -p /dev/ttyACM0 -i target/output.bin -b arduino:samd:mkrvidor4000