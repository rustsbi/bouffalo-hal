# UART peripheral demo

## Build this example for `D0` core (default) with BL Dev Cube

```bash
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p uart-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64imac-unknown-none-elf/release/uart-demo ./target/riscv64imac-unknown-none-elf/release/uart-demo.bin
```

Open BL Dev Cube GUI, choose `D0` group, address `0x58000000`, then flash the binary to the board.

## Build this example for `D0` core (default) with Cli

Replace `PORT_NAME` with your com name, COMx for Windows, /dev/ttyx for Linux.

```bash
cargo run --target riscv64imac-unknown-none-elf --release -p uart-demo -- --port PORT_NAME
```

## Build this example for `M0` core

Change `bouffalo-rt` feature in `Cargo.toml` to `bl808-mcu` firstly.

```bash
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release -p uart-demo
```

Compile the binary with:

```bash
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/uart-demo ./target/riscv32imac-unknown-none-elf/release/uart-demo.bin
```

Open BL Dev Cube GUI, choose `M0` group, address `0x58000000`, then flash the binary to the board.
