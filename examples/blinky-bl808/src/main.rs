// Build this example with:
// m0:
// rustup target install riscv32imac-unknown-none-elf
// cargo build -p blinky-bl808 --features bl808-m0 --no-default-features --target riscv32imac-unknown-none-elf --release
// d0:
// rustup target install riscv64imac-unknown-none-elf
// cargo build -p blinky-bl808 --features bl808-d0 --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bl_rom_rt::{entry, Clocks, Peripherals};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = p.gpio.io8.into_floating_output();
    loop {
        led.set_low().ok();
        for _ in 0..100_000 {
            unsafe { core::arch::asm!("nop") }
        }
        led.set_high().ok();
        for _ in 0..100_000 {
            unsafe { core::arch::asm!("nop") }
        }
    }
}
