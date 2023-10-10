// Build this example with:
// rustup target install riscv64imac-unknown-none-elf
// cargo build --target riscv64imac-unknown-none-elf --release -p lz4d-demo

#![no_std]
#![no_main]

use base_address::Static;
use bl_rom_rt::entry;
use bl_soc::{clocks::Clocks, gpio::Pins, prelude::*, uart::UartMuxes, GLB, LZ4D, UART};
use embedded_time::rate::*;
use panic_halt as _;

static LZ4_INPUT: &[u8; 1182] = include_bytes!("text.lz4");
static mut LZ4_OUTPUT: [u8; 2048] = [0u8; 2048];

#[entry]
fn main() -> ! {
    // values initialized by ROM runtime
    let gpio: Pins<Static<0x20000000>> = unsafe { core::mem::transmute(()) };
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

    for i in 0..12 {
        let a = i * 0x04 + 0x2000AD00;
        let val = unsafe { *(a as *const u32) };
        writeln!(serial, "(0x{:08x}) = 0x{:08x}", a, val).ok();
    }

    unsafe { lz4d.config.modify(|v| v.disable()) };
    writeln!(
        serial,
        "Input address: 0x{:08x}",
        LZ4_INPUT as *const _ as usize as u32
    )
    .ok();
    unsafe {
        lz4d.source_start
            .write(core::mem::transmute(LZ4_INPUT as *const _ as usize as u32))
    };
    writeln!(serial, "Output address: 0x{:08x}", unsafe {
        &LZ4_OUTPUT as *const _ as usize as u32
    })
    .ok();
    unsafe {
        lz4d.destination_start.write(core::mem::transmute(
            &mut LZ4_OUTPUT as *mut _ as usize as u32,
        ))
    };
    writeln!(
        serial,
        "Input address {:?}, Output address {:?}",
        lz4d.source_start.read(),
        lz4d.destination_start.read()
    )
    .ok();
    unsafe { lz4d.config.modify(|v| v.enable()) };

    for i in 0..12 {
        let a = i * 0x04 + 0x2000AD00;
        let val = unsafe { *(a as *const u32) };
        writeln!(serial, "(0x{:08x}) = 0x{:08x}", a, val).ok();
    }

    loop {
        let finished = lz4d
            .interrupt_state
            .read()
            .has_interrupt(bl_soc::lz4d::Interrupt::Done);
        if finished {
            let len = lz4d.destination_end.read().end();
            writeln!(
                serial,
                "Decompression finished, output length is {} bytes. The decompressed text is:",
                len
            )
            .ok();
            serial.write_all(unsafe { &LZ4_OUTPUT }).unwrap();
            loop {
                unsafe { riscv::asm::wfi() }
            }
        } else {
            writeln!(serial, "Decompression is in progress...").ok();
            unsafe { riscv::asm::delay(100_000) }
        }
    }
}
