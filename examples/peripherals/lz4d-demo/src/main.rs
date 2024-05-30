// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p lz4d-demo

#![no_std]
#![no_main]

use bl_rom_rt::{entry, Peripherals};
use bouffalo_hal::{clocks::Clocks, prelude::*};
use core::pin::Pin;
use embedded_time::rate::*;
use panic_halt as _;

static LZ4_INPUT: &[u8; 1182] = include_bytes!("text.lz4");
static mut LZ4_OUTPUT: [u8; 2048] = [0u8; 2048];

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

    writeln!(serial, "Hardware accelerated LZ4 decompression example.").ok();
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
            let (resource, len, _lz4d) = decompress.wait().unwrap();
            writeln!(serial, "Decompression finished, output {} bytes.", len).ok();
            writeln!(serial, "The decompressed text is:").ok();
            serial
                .write_all(&Pin::into_inner(resource.output)[..len])
                .unwrap();
            loop {
                riscv::asm::wfi()
            }
        }
    }
}
