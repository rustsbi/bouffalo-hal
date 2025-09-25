#![no_std]
#![no_main]

use bouffalo_hal::{prelude::*, uart::Config};
use bouffalo_rt::{Clocks, Peripherals, entry};
use core::pin::Pin;
use embedded_time::rate::*;
use panic_halt as _;

static LZ4_INPUT: &'static [u8] = include_bytes!("text.lz4");
static mut LZ4_OUTPUT: [u8; 2048] = [0u8; 2048];

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();

    unsafe { p.glb.clock_config_1.modify(|v| v.enable_lz4d()) };

    let decompress = p.lz4d.decompress(
        Pin::new(&LZ4_INPUT),
        Pin::new(unsafe { &mut *core::ptr::addr_of_mut!(LZ4_OUTPUT) }),
    );

    loop {
        if decompress.is_ongoing() {
            writeln!(serial, "Decompression is in progress...").ok();
            riscv::asm::delay(100_000)
        } else {
            let (resource, len) = decompress.wait().unwrap();
            writeln!(serial, "Decompression finished, output {} bytes.", len).ok();
            writeln!(serial, "The decompressed text is:").ok();
            serial
                .write_all(&Pin::into_inner(resource.output)[..len])
                .unwrap();
            break;
        }
    }
    loop {
        riscv::asm::wfi()
    }
}
