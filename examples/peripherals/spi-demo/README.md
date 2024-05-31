# SPI Demo

The image `src/ferris.raw` is from [this project](https://github.com/almindor/st7789-examples).

## Build

Build this example with:

```
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p spi-demo
```

Objcopy and prepare flash image with:

```
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64imac-unknown-none-elf/release/spi-demo ./target/riscv64imac-unknown-none-elf/release/spi-demo.bin
blri ./target/riscv64imac-unknown-none-elf/release/spi-demo.bin
```
