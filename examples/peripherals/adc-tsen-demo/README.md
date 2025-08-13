# ADC internal temperature sensor demo

## Build this example for `M0` core

```bash
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release -p adc-tsen-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/adc-tsen-demo ./target/riscv32imac-unknown-none-elf/release/adc-tsen-demo.bin
```

Open BL Dev Cube GUI, choose `M0` group, address `0x58000000`, then flash the binary to the board.
