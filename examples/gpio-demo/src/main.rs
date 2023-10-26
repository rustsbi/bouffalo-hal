// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p gpio-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{gpio::Pads, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let gpio: Pads<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    // enable jtag
    gpio.io0.into_jtag_d0();
    gpio.io1.into_jtag_d0();
    gpio.io2.into_jtag_d0();
    gpio.io3.into_jtag_d0();

    let mut led = gpio.io8.into_floating_output();
    let mut button_1 = gpio.io22.into_pull_up_input();
    let mut button_2 = gpio.io23.into_pull_up_input();
    button_1.enable_schmitt();
    button_2.enable_schmitt();
    let mut led_state = PinState::High;
    loop {
        let button_1_pressed = button_1.is_low().unwrap();
        let button_2_pressed = button_2.is_low().unwrap();
        if button_1_pressed && button_2_pressed {
            led.set_state(led_state).ok();
            led_state = !led_state;
            unsafe { riscv::asm::delay(10_000) }
        } else if button_1_pressed {
            led.set_low().ok();
        } else if button_2_pressed {
            led.set_high().ok();
        }
        unsafe { riscv::asm::delay(100_000) }
    }
}
