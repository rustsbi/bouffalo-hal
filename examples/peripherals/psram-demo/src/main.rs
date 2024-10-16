#![no_std]
#![no_main]

use core::{arch::asm, ptr};

use bouffalo_hal::prelude::*;
use bouffalo_rt::{entry, Clocks, Peripherals};
use embedded_time::rate::*;
use panic_halt as _;
use psram::*;

mod psram;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    // light up led
    let mut led = p.gpio.io8.into_floating_output();
    let led_state = PinState::Low;
    led.set_state(led_state).ok();

    // init serial
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();
    let mut serial = p.uart0.freerun(
        Default::default(),
        2000000.Bd(),
        ((tx, sig2), (rx, sig3)),
        &c,
    );
    writeln!(serial, "Welcome to psram-demo🦀!").ok();

    uhs_psram_init();
    writeln!(serial, "uhs_psram_init success").ok();

    const MEMORY_SIZE: usize = 64 * 1024 * 1024;
    const START_ADDRESS: u32 = 0x50000000;
    const PROGRESS_INTERVAL: usize = MEMORY_SIZE / 4 / 10;
    writeln!(serial, "start memory test...").ok();

    writeln!(serial, "  write start...").ok();
    for i in 0..MEMORY_SIZE / 4 {
        if (i + 1) % PROGRESS_INTERVAL == 0 {
            writeln!(
                serial,
                "  write progress: {}%",
                ((i + 1) * 100) / (MEMORY_SIZE / 4) + 1
            )
            .ok();
        }
        let addr = START_ADDRESS + (i as u32 * 4);
        write_memory(addr, i as u32);
    }
    writeln!(serial, "  write finish").ok();

    writeln!(serial, "  read start...").ok();
    let mut error_cnt = 0;
    for i in 0..MEMORY_SIZE / 4 {
        if (i + 1) % PROGRESS_INTERVAL == 0 {
            writeln!(
                serial,
                "  read progress: {}%",
                ((i + 1) * 100) / (MEMORY_SIZE / 4) + 1
            )
            .ok();
        }
        let addr = START_ADDRESS + (i as u32 * 4);
        let val = read_memory(addr);
        if val != i as u32 {
            error_cnt = error_cnt + 1;
            if error_cnt < 10 {
                writeln!(
                    serial,
                    "failed at address {:#010X}, expected {:#010X}, got {:#010X}",
                    addr, i, val
                )
                .ok();
            }
        }
    }
    writeln!(serial, "  read finish").ok();

    if error_cnt == 0 {
        writeln!(serial, "memory test success.").ok();
    } else {
        writeln!(
            serial,
            "memory test failed, error_cnt: {} ({:.5}%). The first 10 errors are shown above.",
            error_cnt,
            error_cnt as f64 / (MEMORY_SIZE / 4) as f64
        )
        .ok();
    }

    loop {
        unsafe { asm!("nop") }
    }
}

#[inline]
pub(crate) fn read_memory(addr: u32) -> u32 {
    unsafe { ptr::read_volatile(addr as *const u32) }
}

#[inline]
pub(crate) fn write_memory(addr: u32, val: u32) {
    unsafe { ptr::write_volatile(addr as *mut u32, val) }
}

#[inline]
pub(crate) fn set_bits(val: u32, pos: u32, len: u32, val_in: u32) -> u32 {
    let mask = ((1 << len) - 1) << pos;
    (val & !mask) | ((val_in << pos) & mask)
}

#[inline]
pub(crate) fn sleep_us(_: u32) {
    for _ in 0..1000 {
        unsafe { asm!("nop") }
    }
}

#[inline]
pub(crate) fn sleep_ms(n: u32) {
    sleep_us(n);
}
