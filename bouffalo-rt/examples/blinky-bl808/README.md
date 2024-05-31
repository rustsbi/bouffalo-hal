Build this example with:

'MCU' cores (M0):

```
rustup target install riscv32imac-unknown-none-elf
cargo build -p blinky-bl808 --features bl808-mcu --no-default-features --target riscv32imac-unknown-none-elf --release
```

'DSP' cores (D0):

```
rustup target install riscv64imac-unknown-none-elf
cargo build -p blinky-bl808 --features bl808-dsp --target riscv64imac-unknown-none-elf --release
```
