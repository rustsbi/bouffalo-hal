// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p jtag-demo

#![no_std]
#![no_main]

use bl_rom_rt::{entry, Clocks, Peripherals};
use bl_soc::prelude::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    // enable jtag
    p.gpio.io0.into_jtag_d0();
    p.gpio.io1.into_jtag_d0();
    p.gpio.io2.into_jtag_d0();
    p.gpio.io3.into_jtag_d0();

    let mut led = p.gpio.io8.into_floating_output();
    loop {
        led.set_low().ok();
        riscv::asm::delay(100_000);
        led.set_high().ok();
        riscv::asm::delay(100_000);
    }
}
