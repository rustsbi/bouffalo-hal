UART peripheral demo with DMA

Build this example with:

```
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release -p uart-dma-demo
```

Compile the binary with:
```
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/uart-dma-demo ./target/riscv32imac-unknown-none-elf/release/uart-dma-demo.bin
```

Open BL Dev Cube GUI, choose `M0` group, address `0x58000000`, then flash the binary to the board