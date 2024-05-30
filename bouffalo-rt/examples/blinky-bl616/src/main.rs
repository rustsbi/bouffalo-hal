// Build this example with:
// rustup target install riscv32imac-unknown-none-elf
// cargo build -p blinky-bl616 --features bl616 --target riscv32imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bouffalo_rt::{entry, Clocks, Peripherals};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    p.gpio.io10.into_jtag_m0();
    p.gpio.io11.into_jtag_m0();
    p.gpio.io12.into_jtag_m0();
    p.gpio.io13.into_jtag_m0();

    let mut io27 = p.gpio.io27.into_pull_down_output();
    let mut io28 = p.gpio.io28.into_pull_down_output();

    loop {
        io27.set_low().ok();
        io28.set_low().ok();
        riscv::asm::delay(100_000);
        io27.set_high().ok();
        io28.set_high().ok();
        riscv::asm::delay(100_000);
    }
}
