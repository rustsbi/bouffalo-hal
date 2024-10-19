#![no_std]
#![no_main]

use bouffalo_hal::prelude::*;
use bouffalo_rt::{entry, Clocks, Peripherals};
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();

    let config = Default::default();
    let serial = p
        .uart0
        .freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &c);

    let (mut tx, mut rx) = serial.split();

    let mut buf = [0u8; 1];

    writeln!(tx, "Hello world!").ok();
    rx.read_exact(&mut buf).ok();
    writeln!(tx, "Character: {}", buf[0]).ok();

    loop {
        // TODO: use embedded-cli to build a serial console
    }
}
