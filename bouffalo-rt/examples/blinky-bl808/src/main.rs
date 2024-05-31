// See README.md file for build instructions on this example.

#![no_std]
#![no_main]

use bouffalo_rt::{entry, exception, interrupt, soc::bl808::TrapFrame, Clocks, Peripherals};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = p.gpio.io8.into_floating_output();
    loop {
        led.set_low().ok();
        riscv::asm::delay(100_000);
        led.set_high().ok();
        riscv::asm::delay(100_000);
    }
}

#[interrupt]
fn uart3() {
    // TODO: interrupt handler content
}

#[exception]
fn exceptions(tf: &mut TrapFrame) {
    let _ = tf;
    // TODO: handle exceptions
}
