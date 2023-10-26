#![no_std]
#![no_main]

use base_address::Static;
use bl_soc::{gpio::Pads, prelude::*};
use panic_halt as _;

#[bl_rom_rt::entry]
fn main() -> ! {
    let gpio: PADS<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let mut led = gpio.io8.into_floating_output();
    let mut led_state = PinState::High;
    loop {
        led.set_state(led_state).ok();
        led_state = !led_state;
        unsafe { riscv::asm::delay(100_000) };
    }
}
