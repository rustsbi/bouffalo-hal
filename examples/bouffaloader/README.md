# Bouffaloader demo

## Usage with Cargo

### Quick Start

```bash
cargo make
cargo blri run
```

Consistent with the following steps.

### Compile

```bash
cargo build -p bouffaloader --target riscv64imac-unknown-none-elf --release
```

### Convert ELF file to binary file

```bash
cargo blri elf2bin ./target/riscv64imac-unknown-none-elf/release/bouffaloader
```

### Fix the image header (CRC and other fields)

```bash
cargo blri patch ./target/riscv64imac-unknown-none-elf/release/bouffaloader.bin
```

### Flash to the development board

```bash
cargo blri flash ./target/riscv64imac-unknown-none-elf/release/bouffaloader.bin
```

## Usage with Bouffalo Lab Dev Cube

### Build

```bash
rustup target install riscv64imac-unknown-none-elf
cargo build -p bouffaloader --target riscv64imac-unknown-none-elf --release
```

### Convert the elf file to a binary file

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
rust-objcopy .\target\riscv64imac-unknown-none-elf\release\bouffaloader -O binary .\target\riscv64imac-unknown-none-elf\release\bouffaloader.bin
```

### Flash the binary file to the board with [Bouffalo Lab Dev Cube](https://dev.bouffalolab.com/download) on Windows

1. Connect the board to the computer via UART (Here takes M1s Dock as an example):
    - Normally, you can see 2 new serial ports. If not, visit [Burn onboard bl702](https://wiki.sipeed.com/hardware/en/maix/m1s/other/start.html#Burn-onboard-bl702) for help.

2. Run the `BLDevCube.exe`, choose `BL808`, and click `Finish`.

3. In MCU page, browse `target\riscv64imac-unknown-none-elf\release\bouffaloader.bin` as the target of `D0 Group`. Choose the bigger number serial port, and set uart rate 2000000.

4. Press BOOT and RST on the board, then release RST first and BOOT after to be into UART burning mode.

5. Click `Create & Download`, wait for flash the binary file to success.

## Interact

After flashing, repower the board and open the serial port monitor to see the output and interact with the CLI:

- `help`: print out all commands.

- `hello`: print out 'Hello world!'.

- `led [<none>|on|off|switch]`: operate on LED.

- `reload`: reload from sdcard.

- `read <addr>`: fetch data from address.

- `write <addr> <val>`: write value to address.

- `boot`: boot M-mode firmware.

- `bootargs get|set <val>`: print or set the bootargs in memory.

- `print`: print the configs.bootargs.
