# Multicore Demo

This demo runs on M0 and D0 cores on BL808 chip.

To begin with, install Rust compiler targets for both cores.

```sh
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv64imac-unknown-none-elf
rustup component add llvm-tools-preview
```

Compile each project using cargo commands.

```sh
cargo build -p multicore-demo-dsp --target riscv64imac-unknown-none-elf --release
cargo build -p multicore-demo-mcu --target riscv32imac-unknown-none-elf --release
```

Objcopy each project.

```sh
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64imac-unknown-none-elf/release/multicore-demo-dsp ./target/riscv64imac-unknown-none-elf/release/multicore-demo-dsp.bin
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/multicore-demo-mcu ./target/riscv32imac-unknown-none-elf/release/multicore-demo-mcu.bin
```

Flash using BLDevCube.

1. Switch to 'MCU' tab.
2. On 'M0 Group', set group to 'group0', set 'Image Addr' to '0x58001000', set program path to multicore-demo-mcu.bin file path.
3. On 'D0 Group', set group ro 'group0', set 'Image Addr' to '0x58000000', set program path to multicore-demo-dsp.bin file path.
4. Click button 'Create & Download'.
