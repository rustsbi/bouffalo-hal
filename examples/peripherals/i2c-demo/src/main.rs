// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p i2c-demo

#![no_std]
#![no_main]

use bl_rom_rt::{entry, Clocks, Peripherals};
use bl_soc::{i2c::I2c, prelude::*};
use embedded_time::rate::*;
use panic_halt as _;

const SCREEN_TOUCH_SUB_ADDRESS: u8 = 0x01;
const SCREEN_ADDRESS: u8 = 0x15;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();

    let config = Default::default();
    let mut serial = p
        .uart0
        .freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &c);
    let mut led = p.gpio.io8.into_floating_output();

    let scl = p.gpio.io6.into_i2c::<2>();
    let sda = p.gpio.io7.into_i2c::<2>();
    let mut i2c = I2c::new(p.i2c0, (scl, sda), &p.glb);
    i2c.enable_sub_address(SCREEN_TOUCH_SUB_ADDRESS);

    writeln!(serial, "Hello Rust🦀!").ok();
    let mut buf = [0u8; 6];
    loop {
        riscv::asm::delay(100_000);
        match i2c.read(SCREEN_ADDRESS, &mut buf) {
            Ok(_) => {
                if buf[2] >> 4 == 8 {
                    led.set_low().ok();
                } else {
                    led.set_high().ok();
                }
                writeln!(serial, "pos: ({}, {})[{}]", buf[3], buf[5], buf[2] >> 4).ok();
            }
            Err(_) => {
                led.set_high().ok();
            }
        }
    }
}
