// Build this example with:
// m0:
// rustup target install riscv32imac-unknown-none-elf
// cargo build --example blinky-bl808 --features bl808-m0 --target riscv32imac-unknown-none-elf --release
// d0:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --example blinky-bl808 --features bl808-d0 --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bl_rom_rt::{entry, Peripherals};
use embedded_hal::digital::OutputPin;

#[entry]
fn main(p: Peripherals) -> ! {
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

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
