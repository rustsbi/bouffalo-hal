#![no_std]
#![no_main]

use bouffalo_hal::{i2c::I2c, prelude::*, uart::Config};
use bouffalo_rt::{Clocks, Peripherals, entry};
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
    let pads = ((tx, sig2), (rx, sig3));

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, pads, &c).unwrap();
    let mut led = p.gpio.io8.into_floating_output();

    let scl = p.gpio.io6;
    let sda = p.gpio.io7;
    let mut i2c = I2c::new(p.i2c0, (scl, sda), &p.glb);
    i2c.enable_sub_address(SCREEN_TOUCH_SUB_ADDRESS);

    writeln!(serial, "Hello Rust🦀!").ok();
    writeln!(
        serial,
        "Welcome to I2C demo, touch the screen to turn on the LED"
    )
    .ok();
    led.set_high().ok();
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
            Err(_) => {}
        }
    }
}
