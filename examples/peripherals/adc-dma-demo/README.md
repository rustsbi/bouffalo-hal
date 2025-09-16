# ADC dma demo

## Build this example for `D0` core

```bash
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p adc-dma-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64imac-unknown-none-elf/release/adc-dma-demo ./target/riscv64imac-unknown-none-elf/release/adc-dma-demo.bin
```

Open BL Dev Cube GUI, choose `D0` group, address `0x58000000`, then flash the binary to the board.
