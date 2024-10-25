UART peripheral demo with embedded-cli library

Build this example with:

```
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p uart-cli-demo
```

Convert the elf file to a binary file:

```
cargo install cargo-binutils
rustup component add llvm-tools-preview
rust-objcopy .\target\riscv64imac-unknown-none-elf\release\uart-cli-demo -O binary .\target\riscv64imac-unknown-none-elf\release\uart-cli-demo.bin
```

Flash the binary file to the board with [Bouffalo Lab Dev Cube](https://dev.bouffalolab.com/download) on Windows:

1. Connect the board to the computer via UART (Here takes M1s Dock as an example):
    - Normally, you can see 2 new serial ports. If not, visit [Burn onboard bl702](https://wiki.sipeed.com/hardware/en/maix/m1s/other/start.html#Burn-onboard-bl702) for help.
  
2. Run the `BLDevCube.exe`, choose `BL808`, and click `Finish`.
   
3. In MCU page, browse `target\riscv64imac-unknown-none-elf\release\uart-cli-demo.bin` as the target of `D0 Group`. Choose the bigger number serial port, and set uart rate 2000000.

4. Press BOOT and RST on the board, then release RST first and BOOT after to be into UART burning mode.

5. Click `Create & Download`, wait for flash the binary file to success.

6. After flashing, repower the board and open the serial port monitor to see the output and interact with the CLI:
    - `led [<none>|on|off|switch]`: operate on LED.