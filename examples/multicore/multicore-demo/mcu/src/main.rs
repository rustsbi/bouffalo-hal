#![no_std]
#![no_main]

use bouffalo_hal::prelude::*;
use bouffalo_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::High;
    loop {
        led.set_state(led_state).ok();
        led_state = !led_state;
        riscv::asm::delay(100_000);
    }
}
