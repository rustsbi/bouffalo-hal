#![feature(noop_waker)]
#![no_std]
#![no_main]

use bouffalo_hal::{
    prelude::*,
    uart::{Config, SerialState},
};
use bouffalo_rt::{entry, interrupt, Clocks, Peripherals};
use embedded_time::rate::*;
use panic_halt as _;

async fn async_main(p: Peripherals, c: Clocks) {
    let tx = p.gpio.io16.into_mm_uart();
    let rx = p.gpio.io17.into_mm_uart();

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p
        .uart3
        .with_interrupt(config, (tx, rx), &c, &UART3_STATE)
        .unwrap();

    serial
        .write_all(b"Hello world from async/await uart demo!")
        .await;
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
    let mut fut = core::pin::pin!(async_main(p, c));
    let waker = Waker::noop();
    let mut ctx = Context::from_waker(waker);
    loop {
        match fut.as_mut().poll(&mut ctx) {
            Poll::Ready(_) => break,
            Poll::Pending => riscv::asm::wfi(),
        }
    }
    loop {
        riscv::asm::wfi();
    }
}
