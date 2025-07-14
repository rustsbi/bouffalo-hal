# SDCARD GPT demo

This is a example of how to read a SD card with a GPT partition table and an EFI partition.

If you are using the [Sipeed M1s Dock](https://wiki.sipeed.com/hardware/en/maix/m1s/m1s_dock.html)
development board, you may need to reconnect the corresponding pins according to the table below.

| pin | pin name | SD function(expected SD card pin) | SPI function(expected SD card pin) |
|-----|----------|-----------------------------------|------------------------------------|
| 1   | io4      | DAT2(1)                           | X                                  |
| 2   | io5      | DAT3(2)                           | X                                  |
| 3   | io1      | CMD(3)                            | MOSI(3)                            |
| 4   | VDD      | VDD(4)                            | VDD(4)                             |
| 5   | io0      | CLK(5)                            | CS(2)                              |
| 6   | GND      | VSS(6)                            | VSS(6)                             |
| 7   | io2      | DAT0(7)                           | MISO(7)                            |
| 8   | io3      | DAT1(8)                           | SCLK(5)                            |

## Build this example for `D0` core (default) with BL Dev Cube

```bash
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p sdcard-gpt-demo
```

Objcopy and prepare flash image with:

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64imac-unknown-none-elf/release/sdcard-gpt-demo ./target/riscv64imac-unknown-none-elf/release/sdcard-gpt-demo.bin
cargo blri ./target/riscv64imac-unknown-none-elf/release/sdcard-gpt-demo.bin
```

Flash the binary file to the board with [Bouffalo Lab Dev Cube](https://dev.bouffalolab.com/download) on Windows:

1. Connect the board to the computer via UART (Here takes M1s Dock as an example):
    - Normally, you can see 2 new serial ports. If not, visit [Burn onboard bl702](https://wiki.sipeed.com/hardware/en/maix/m1s/other/start.html#Burn-onboard-bl702) for help.
  
2. Run the `BLDevCube.exe`, choose `BL808`, and click `Finish`.

3. In MCU page, browse `target\riscv64imac-unknown-none-elf\release\sdcard-gpt-demo.bin` as the target of `D0 Group`. Choose the bigger number serial port, and set uart rate 2000000.

4. Press BOOT and RST on the board, then release RST first and BOOT after to be into UART burning mode.

5. Click `Create & Download`, wait for flash the binary file to success.

6. After flashing, repower the board and open the serial port to see the output like this:

```bash
Hello world!
Card size: 7822376960
Primary header: GptHeader { signature: GptHeaderSignature(16933534594248999176), revision: GptHeaderRevision(3942645758), header_size: 3942645758, header_crc32: Crc32(3942645758), reserved: 3942645758, my_lba: LbaLe(16933534594265776126), alternate_lba: LbaLe(0), first_usable_lba: LbaLe(0), last_usable_lba: LbaLe(0), disk_guid: Guid { time_low: 0, time_mid: [0, 0], time_high_and_version: [0, 0], clock_seq_high_and_reserved: 0, clock_seq_low: 2, node: [1, 0, 84, 84, 0, 0] }, partition_entry_lba: LbaLe(18446462603027742720), number_of_partition_entries: 0, size_of_partition_entry: 50331648, partition_entry_array_crc32: Crc32(26508785) }
```

## Build this example for `D0` core (default) with Cli

Replace `PORT_NAME` with your com name, COMx for Windows, /dev/ttyx for Linux.

```bash
cargo run --target riscv64imac-unknown-none-elf --release -p sdcard-gpt-demo -- --port PORT_NAME
```
