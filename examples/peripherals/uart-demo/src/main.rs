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

    let mut led = p.gpio.io8.into_floating_output();
    let mut led_state = PinState::Low;
    let mut buf = [0u8; 32];
    let mut ch = b'\r';

    writeln!(serial, "Welcome to console example by bl-soc🦀!").ok();
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
