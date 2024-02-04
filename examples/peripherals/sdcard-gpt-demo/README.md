This is a example of how to read a SD card with a GPT partition table and an EFI partition.

## Build 

Build this example with:

```
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p sdcard-gpt-demo
```
