// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p uart-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{
    clocks::Clocks,
    gpio::Pins,
    uart::{BitOrder, Config, Parity, Serial, StopBits, UartMuxes, WordLength},
    GLB, UART,
};
use embedded_hal::digital::OutputPin;
use embedded_hal::serial::Write;
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: Pins<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let glb: GLB<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let uart0: UART<Static<0x2000A000>> = unsafe { core::mem::transmute(()) };
    let uart_muxes: UartMuxes<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let clocks = Clocks {};

    // enable jtag
    gpio.io0.into_jtag_d0();
    gpio.io1.into_jtag_d0();
    gpio.io2.into_jtag_d0();
    gpio.io3.into_jtag_d0();

    let tx = gpio.io14.into_uart();
    let rx = gpio.io15.into_uart();
    let sig2 = uart_muxes.sig2.into_transmit::<0>();
    let sig3 = uart_muxes.sig3.into_receive::<0>();

    let config = Config {
        bit_order: BitOrder::LsbFirst,
        parity: Parity::None,
        stop_bits: StopBits::One,
        word_length: WordLength::Eight,
    };
    let mut serial = Serial::new(
        uart0,
        config,
        2000000.Bd(),
        ((tx, sig2), (rx, sig3)),
        &clocks,
        &glb,
    );

    let mut led = gpio.io8.into_floating_output();
    loop {
        serial.write("Hello Rust🦀!\r\n".as_bytes()).ok();
        serial.flush().ok();
        led.set_low().ok();
        for _ in 0..100_000 {
            unsafe { core::arch::asm!("nop") }
        }
        led.set_high().ok();
        for _ in 0..100_000 {
            unsafe { core::arch::asm!("nop") }
        }
    }
}
