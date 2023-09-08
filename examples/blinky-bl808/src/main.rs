// Build this example with:
// m0:
// rustup target install riscv32imac-unknown-none-elf
// cargo build -p blinky-bl808 --features bl808-m0 --no-default-features --target riscv32imac-unknown-none-elf --release
// d0:
// rustup target install riscv64imac-unknown-none-elf
// cargo build -p blinky-bl808 --features bl808-d0 --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bl_rom_rt::{entry, interrupt, Clocks, Peripherals};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = p.gpio.io8.into_floating_output();
    loop {
        led.set_low().ok();
        unsafe { riscv::asm::delay(100_000) };
        led.set_high().ok();
        unsafe { riscv::asm::delay(100_000) };
    }
}

#[interrupt]
fn uart0() {
    // TODO: interrupt handler content
}
