## Testcases for Elf2Bin module

Files under `blri/tests/elf2bin/elf`:
- built from `examples/peripherals`
- with `cargo build --target riscv64imac-unknown-none-elf --release` 
- at commit hash `7ac5ff6c190d237733a57630a49f8a56b0f2c2e3`

Files under `blri/tests/elf2bin/rust-objcopy-bin` :
- generated by
`rust-objcopy`(`llvm-objcopy`) 
- with `rust-objcopy -O binary <elf> <bin>`