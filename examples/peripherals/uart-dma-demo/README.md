# UART peripheral demo with DMA

## Build this example for `M0` core

Change `bouffalo-rt` feature in `Cargo.toml` to `bl808-mcu` firstly.

```bash
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release -p uart-dma-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/uart-dma-demo ./target/riscv32imac-unknown-none-elf/release/uart-dma-demo.bin
```

Open BL Dev Cube GUI, choose `M0` group, address `0x58000000`, then flash the binary to the board.
