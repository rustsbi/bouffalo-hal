// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p lz4d-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{clocks::Clocks, gpio::Pads, prelude::*, uart::UartMuxes, GLB, LZ4D, UART};
use core::pin::Pin;
use embedded_time::rate::*;
use panic_halt as _;

static LZ4_INPUT: &[u8; 1182] = include_bytes!("text.lz4");
static mut LZ4_OUTPUT: [u8; 2048] = [0u8; 2048];

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: PADS<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let uart0: UART<Static<0x2000A000>> = unsafe { core::mem::transmute(()) };
    let uart_muxes: UartMuxes<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
    let clocks = Clocks {
        xtal: Hertz(40_000_000),
    };
    let lz4d: LZ4D<Static<0x2000AD00>> = unsafe { core::mem::transmute(()) };
    let glb: GLB<Static<0x20000000>> = unsafe { core::mem::transmute(()) };

    // enable jtag
    gpio.io0.into_jtag_d0();
    gpio.io1.into_jtag_d0();
    gpio.io2.into_jtag_d0();
    gpio.io3.into_jtag_d0();

    let tx = gpio.io14.into_uart();
    let rx = gpio.io15.into_uart();
    let sig2 = uart_muxes.sig2.into_transmit::<0>();
    let sig3 = uart_muxes.sig3.into_receive::<0>();

    let config = Default::default();
    let mut serial = uart0.freerun(config, 2000000.Bd(), ((tx, sig2), (rx, sig3)), &clocks);

    writeln!(serial, "Hardware accelerated LZ4 decompression example.").ok();
    unsafe { glb.clock_config_1.modify(|v| v.enable_lz4d()) };

    let decompress = lz4d.decompress(Pin::new(&LZ4_INPUT), Pin::new(unsafe { &mut LZ4_OUTPUT }));

    loop {
        if decompress.is_ongoing() {
            writeln!(serial, "Decompression is in progress...").ok();
            unsafe { riscv::asm::delay(100_000) }
        } else {
            let (resource, len) = decompress.wait().unwrap();
            writeln!(serial, "Decompression finished, output {} bytes.", len).ok();
            writeln!(serial, "The decompressed text is:").ok();
            serial
                .write_all(&Pin::into_inner(resource.output)[..len])
                .unwrap();
            loop {
                unsafe { riscv::asm::wfi() }
            }
        }
    }
}
