// Build this example with:
// 'MCU' cores (M0):
// rustup target install riscv32imac-unknown-none-elf
// cargo build -p blinky-bl808 --features bl808-mcu --no-default-features --target riscv32imac-unknown-none-elf --release
// 'DSP' cores (D0):
// rustup target install riscv64imac-unknown-none-elf
// cargo build -p blinky-bl808 --features bl808-dsp --target riscv64imac-unknown-none-elf --release

#![no_std]
#![no_main]

use bl_rom_rt::{entry, exception, interrupt, soc::bl808::TrapFrame, Clocks, Peripherals};
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
fn uart3() {
    // TODO: interrupt handler content
}

#[exception]
fn exceptions(tf: &mut TrapFrame) {
    let _ = tf;
    // TODO: handle exceptions
}
