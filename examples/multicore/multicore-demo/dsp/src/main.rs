#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{clocks::Clocks, gpio::Pads, prelude::*, uart::UartMuxes, UART};
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: PADS<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let uart0: UART<Static<0x2000A000>> = unsafe { core::mem::transmute(()) };
    let uart_muxes: UartMuxes<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let clocks = Clocks {
        xtal: Hertz(40_000_000),
    };

    let tx = gpio.io14.into_uart();
    let rx = gpio.io15.into_uart();
    let sig2 = uart_muxes.sig2.into_transmit::<0>();
    let sig3 = uart_muxes.sig3.into_receive::<0>();

    let config = Default::default();
    let mut serial = uart0.freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &clocks);

    loop {
        writeln!(serial, "Welcome to bl-soc multicore demo from DSP core🦀!").ok();
        unsafe { riscv::asm::delay(100_000) };
    }
}
