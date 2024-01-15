#![no_std]
#![no_main]

use bl_rom_rt::{entry, Clocks, Peripherals};
use bl_soc::prelude::*;
use embedded_time::rate::*;
use panic_halt as _;

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

    loop {
        writeln!(serial, "Welcome to bl-soc multicore demo from DSP core🦀!").ok();
        riscv::asm::delay(100_000);
    }
}
