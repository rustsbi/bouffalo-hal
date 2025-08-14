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
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();
    let mut led = p.gpio.io8.into_floating_output();

    let scl = p.gpio.io6;
    let sda = p.gpio.io7;
    let mut touch_int = p.gpio.io32.into_pull_up_input();
    let mut i2c = I2c::new(p.i2c0, (scl, sda), 100_000.Hz(), &p.glb);

    writeln!(serial, "Hello Rust🦀!").ok();
    writeln!(
        serial,
        "Welcome to I2C demo, touch the screen to turn on the LED"
    )
    .ok();
    led.set_high().ok();
    let mut buf = [0u8; 6];
    loop {
        while touch_int.is_high().unwrap() {
            core::hint::spin_loop();
        }

        match i2c.write_read(SCREEN_ADDRESS, &[SCREEN_TOUCH_SUB_ADDRESS], &mut buf) {
            Ok(_) => {
                if buf[2] >> 6 != 1 {
                    led.set_low().ok();
                } else {
                    led.set_high().ok();
                }
                writeln!(
                    serial,
                    "pos: ({}, {}) event flag[{}]",
                    buf[3],
                    buf[5],
                    buf[2] >> 6
                )
                .ok();
            }
            Err(_) => {}
        }
    }
}
