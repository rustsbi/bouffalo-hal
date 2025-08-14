# I2C screen demo

## Build this example for `D0` core (default) with BL Dev Cube

```bash
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p i2c-screen-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64imac-unknown-none-elf/release/i2c-screen-demo ./target/riscv64imac-unknown-none-elf/release/i2c-screen-demo.bin
```

Open BL Dev Cube GUI, choose `D0` group, address `0x58000000`, then flash the binary to the board.

## Build this example for `D0` core (default) with Cli

Replace `PORT_NAME` with your com name, COMx for Windows, /dev/ttyx for Linux.

```bash
cargo run --target riscv64imac-unknown-none-elf --release -p i2c-screen-demo -- --port PORT_NAME
```
