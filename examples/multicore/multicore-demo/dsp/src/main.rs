#![no_std]
#![no_main]

use bouffalo_hal::{prelude::*, uart::Config};
use bouffalo_rt::{Clocks, Peripherals, entry};
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();

    loop {
        writeln!(
            serial,
            "Welcome to bouffalo-hal multicore demo from DSP core🦀!"
        )
        .ok();
        riscv::asm::delay(100_000);
    }
}
