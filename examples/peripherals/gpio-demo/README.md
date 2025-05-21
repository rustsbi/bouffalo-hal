Build this example with:

BL808 (dsp core):

```
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release --features bl808-dsp -p gpio-demo
```

BL702:

```
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release --no-default-features --features bl702 -p gpio-demo
```
