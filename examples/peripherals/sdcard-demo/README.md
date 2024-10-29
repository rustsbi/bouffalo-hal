This is an example of how to read an SD card with an MBR partition table and a FAT32 partition.

Build this example with:

```
rustup target install riscv64imac-unknown-none-elf
cargo build --target riscv64imac-unknown-none-elf --release -p sdcard-demo
```

If you are using the [Sipeed M1s Dock](https://wiki.sipeed.com/hardware/en/maix/m1s/m1s_dock.html) 
development board, you may need to reconnect the corresponding pins according to the table below.

+-----+----------+-----------------------------------+------------------------------------+
| pin | pin name | SD function(expected SD card pin) | SPI function(expected SD card pin) |
+-----+----------+-----------------------------------+------------------------------------+
| 1   | io4      | DAT2(1)                           | X                                  |
| 2   | io5      | DAT3(2)                           | X                                  |
| 3   | io1      | CMD(3)                            | MOSI(3)                            |
| 4   | VDD      | VDD(4)                            | VDD(4)                             |
| 5   | io0      | CLK(5)                            | CS(2)                              |
| 6   | GND      | VSS(6)                            | VSS(6)                             |
| 7   | io2      | DAT0(7)                           | MISO(7)                            |
| 8   | io3      | DAT1(8)                           | SCLK(5)                            |
+-----+----------+-----------------------------------+------------------------------------+
