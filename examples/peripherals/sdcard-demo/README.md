This is an example of how to read an SD card with an MBR partition table and a FAT32 partition.

Build this example with:

```
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p sdcard-demo
```
