rust-objcopy -O binary target/thumbv6m-none-eabi/debug/rusty-vidor target/output.bin

bossac -i -U=true -e -w -v -d target/output.bin -R