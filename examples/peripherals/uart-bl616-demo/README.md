# UART peripheral demo for BL616

## Build this example for BL616 with BL Dev Cube

```bash
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release -p uart-bl616-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/uart-bl616-demo ./target/riscv32imac-unknown-none-elf/release/uart-bl616-demo.bin
```

Open BL Dev Cube GUI, choose `MCU` group, address `0x2000`, then flash the binary to the board.
