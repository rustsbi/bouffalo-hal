// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p uart-embedded-io

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
use embedded_hal::digital::{OutputPin, PinState};
use embedded_io::blocking::Write;
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
    let mut led_state = PinState::High;
    let mut counter = 0;

    loop {
        serial
            .write_all("Hello Rust from bl-soc by embedded-io🦀!\r\n".as_bytes())
            .ok();
        serial
            .write_fmt(format_args!("Counter value: {}\r\n", counter))
            .ok();
        writeln!(serial, "LED state: {:?}", led_state).ok();
        serial.flush().ok();

        led.set_state(led_state).ok();
        counter += 1;
        led_state = !led_state;

        for _ in 0..100_000 {
            unsafe { core::arch::asm!("nop") }
        }
    }
}
