// Build this example with:
// rustup target install riscv32imac-unknown-none-elf
// cargo build -p blinky-bl616 --features bl616 --target riscv32imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bl_rom_rt::*;
use core::arch::asm;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {
        // TODO
        unsafe { asm!("nop") }
    }
}
