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
use embedded_io::blocking::{Read, Write};
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
    let mut led_state = PinState::Low;
    let mut buf = [0u8; 32];
    let mut ch = b'\r';

    #[rustfmt::skip]
    writeln!(serial, "Welcome to console example by bl-soc & embedded-io🦀!").ok();
    writeln!(serial, "Command helps: ").ok();
    writeln!(serial, "    led [<none>|on|off|switch]: operate on LED").ok();

    loop {
        led.set_state(led_state).ok();
        serial.flush().ok();

        write!(serial, "> ").ok();

        let mut idx = 0;
        while ch == b'\r' || ch == b'\n' {
            serial.read_exact(core::slice::from_mut(&mut ch)).ok();
        }
        while ch != b'\r' && ch != b'\n' && idx < buf.len() {
            if ch == 0x08 && idx > 0 {
                // backspace
                write!(serial, "\x08 \x08").unwrap();
                idx -= 1;
            } else if ch != 0x08 {
                write!(serial, "{}", ch as char).unwrap();
                buf[idx] = ch;
                idx += 1;
            }
            serial.read_exact(core::slice::from_mut(&mut ch)).ok();
        }
        ch = b'\r';
        writeln!(serial, "").ok();
        let command = core::str::from_utf8(&buf[..idx]).unwrap();

        match command.trim() {
            "led" => writeln!(serial, "LED state: {:?}", led_state).unwrap(),
            "led on" => led_state = PinState::Low,
            "led off" => led_state = PinState::High,
            "led switch" => led_state = !led_state,
            _ => writeln!(serial, "Unknown command: {}", command).unwrap(),
        }
    }
}
