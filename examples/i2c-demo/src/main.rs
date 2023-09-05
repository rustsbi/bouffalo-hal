// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p i2c-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{clocks::Clocks, gpio::Pins, i2c::I2c, prelude::*, uart::UartMuxes, GLB, I2C, UART};
use embedded_time::rate::*;
use panic_halt as _;

const SCREEN_TOUCH_SUB_ADDRESS: u8 = 0x01;
const SCREEN_ADDRESS: u8 = 0x15;

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: Pins<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let glb: GLB<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let uart0: UART<Static<0x2000A000>> = unsafe { core::mem::transmute(()) };
    let uart_muxes: UartMuxes<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let i2c: I2C<Static<0x30003000>> = unsafe { core::mem::transmute(()) };
    let clocks = Clocks {
        xtal: Hertz(40_000_000),
    };

    // enable jtag
    gpio.io0.into_jtag_d0();
    gpio.io1.into_jtag_d0();
    gpio.io2.into_jtag_d0();
    gpio.io3.into_jtag_d0();

    let tx = gpio.io14.into_uart();
    let rx = gpio.io15.into_uart();
    let sig2 = uart_muxes.sig2.into_transmit::<0>();
    let sig3 = uart_muxes.sig3.into_receive::<0>();

    let config = Default::defnault();
    let mut serial = uart0.freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &clocks);
    let mut led = gpio.io8.into_floating_output();

    let scl = gpio.io6.into_i2c::<2>();
    let sda = gpio.io7.into_i2c::<2>();
    let mut i2c = I2c::new(i2c, (scl, sda), &glb);
    i2c.enable_sub_address(SCREEN_TOUCH_SUB_ADDRESS);

    writeln!(serial, "Hello Rust🦀!").ok();
    let mut buf = [0u8; 6];
    loop {
        unsafe { riscv::asm::delay(100_000) };
        match i2c.read(SCREEN_ADDRESS, &mut buf) {
            Ok(_) => {
                if buf[2] >> 4 == 8 {
                    led.set_low().ok();
                } else {
                    led.set_high().ok();
                }
                writeln!(serial, "pos: ({}, {})[{}]", buf[3], buf[5], buf[2] >> 4).ok();
            }
            Err(_) => {
                led.set_high().ok();
            }
        }
    }
}
