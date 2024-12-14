#![feature(noop_waker)]
#![no_std]
#![no_main]

use bouffalo_hal::{
    prelude::*,
    uart::{Config, SerialState},
};
use bouffalo_rt::{
    entry, interrupt,
    soc::bl808::{D0Machine, DspInterrupt},
    Clocks, Peripherals,
};
use embedded_time::rate::*;
use panic_halt as _;

async fn async_main(p: Peripherals, c: Clocks) {
    // enable jtag
    p.gpio.io0.into_jtag_d0();
    p.gpio.io1.into_jtag_d0();
    p.gpio.io2.into_jtag_d0();
    p.gpio.io3.into_jtag_d0();

    let tx = p.gpio.io16.into_mm_uart();
    let rx = p.gpio.io17.into_mm_uart();

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p
        .uart3
        .with_interrupt(config, (tx, rx), &c, &UART3_STATE)
        .unwrap();
    // TODO: is T-Head C906 PLIC different from standard PLIC?
    p.plic.set_priority(DspInterrupt::UART3, 1);
    p.plic.enable(DspInterrupt::UART3, D0Machine);

    serial
        .write_all(b"Hello world from async/await uart demo!")
        .await
        .ok();
}

static UART3_STATE: SerialState = SerialState::new();

#[interrupt]
fn uart3() {
    UART3_STATE.on_interrupt();
}

// ---- Async/await runtime environment ----

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    use core::{
        future::Future,
        task::{Context, Poll, Waker},
    };
    p.plic.set_threshold(D0Machine, 0);
    let mut fut = core::pin::pin!(async_main(p, c));
    let waker = Waker::noop();
    let mut ctx = Context::from_waker(waker);
    unsafe {
        riscv::register::mie::set_mext();
        riscv::register::mstatus::set_mie();
    }
    loop {
        match fut.as_mut().poll(&mut ctx) {
            Poll::Ready(_) => break,
            Poll::Pending => riscv::asm::wfi(),
        }
    }
    unsafe { riscv::register::mstatus::clear_mie() };
    loop {
        riscv::asm::wfi();
    }
}
