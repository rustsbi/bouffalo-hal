#![no_std]
#![no_main]

use bouffalo_hal::{
    efuse::Efuse,
    gpip::{
        AdcChannels, AdcConfig, AdcResult, Gpip,
        asynch::{AdcState, AsyncAdc},
    },
    hbn::{GpadcChannel, GpadcVref},
    prelude::*,
    uart::Config,
};
use bouffalo_rt::{
    Clocks, Peripherals, entry, interrupt,
    soc::bl808::{M0Machine, McuLpInterrupt},
};
use core::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};
use embedded_time::rate::*;
use panic_halt as _;

static ADC_STATE: AdcState = AdcState::new();

/// TASK_WAKE_FLAG is used to synchronize between the ADC interrupt handler and the main async task.
///
/// - The interrupt handler sets this flag to `true` when an interrupt occurs, indicating that the main task should wake up and poll the future again.
/// - The main task checks this flag in its loop; if set, it clears the flag and continues polling without sleeping.
/// - This mechanism ensures that the main task responds promptly to ADC events signaled by the interrupt.
///
/// This flag is essential for proper async operation because:
/// 1. Without it, the main task would use `Waker::noop()` which never wakes the task
/// 2. The task would sleep indefinitely with `wfi()` waiting for interrupts
/// 3. When interrupts occur, this flag bridges the gap between hardware events and software task scheduling
static TASK_WAKE_FLAG: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn gpadc_dma() {
    ADC_STATE.on_interrupt();
    // Wake the main task after handling interrupt
    TASK_WAKE_FLAG.store(true, Ordering::Release);
}

async fn async_main(p: Peripherals, c: Clocks) {
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();

    writeln!(serial, "Welcome to ADC async demo!").ok();

    let mut gpip = Gpip::new(
        p.gpip,
        Some(AdcConfig::default().set_vref(GpadcVref::V3p2)),
        None,
        &p.glb,
        &p.hbn,
    );

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    // Set up PLIC interrupt
    p.plic.set_threshold(M0Machine, 0);
    p.plic.set_priority(McuLpInterrupt::GpadcDma, 1);
    p.plic.enable(McuLpInterrupt::GpadcDma, M0Machine);

    // Create async ADC
    let mut async_adc = AsyncAdc::new(&mut gpip, &ADC_STATE);

    let channels = [
        AdcChannels {
            pos_ch: GpadcChannel::Channel0,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel1,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel2,
            neg_ch: GpadcChannel::ChannelVGND,
        },
    ];

    for (_i, &channel) in channels.iter().enumerate() {
        writeln!(serial, "Converting channel {:?}...", channel.pos_ch).ok();

        let mut results = [AdcResult {
            pos_chan: None,
            neg_chan: None,
            value: 0,
            millivolt: 0,
        }; 32];

        match async_adc.convert(&[channel], &p.hbn, &mut results).await {
            Ok(count) => {
                writeln!(serial, "Successfully converted {} samples", count).ok();

                // Print first few results
                for j in 0..core::cmp::min(count, 5) {
                    writeln!(
                        serial,
                        "Channel {:?} value = 0x{:08X}, millivolt = {}mv.",
                        results[j].pos_chan.unwrap(),
                        results[j].value,
                        results[j].millivolt
                    )
                    .ok();
                }
            }
            Err(e) => {
                writeln!(serial, "Conversion failed: {:?}", e).ok();
            }
        }

        writeln!(serial, "Conversion attempt completed").ok();
    }

    writeln!(serial, "ADC async demo completed!").ok();
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    p.plic.set_threshold(M0Machine, 0);
    let mut fut = core::pin::pin!(async_main(p, c));

    // Create a simple waker that sets a flag when called
    unsafe fn wake_fn(_: *const ()) {
        TASK_WAKE_FLAG.store(true, Ordering::Release);
    }

    unsafe fn clone_fn(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }

    unsafe fn noop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_fn, wake_fn, wake_fn, noop);

    let raw_waker = RawWaker::new(core::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut ctx = Context::from_waker(&waker);

    unsafe {
        riscv::register::mie::set_mext();
        riscv::register::mstatus::set_mie();
    }

    loop {
        match fut.as_mut().poll(&mut ctx) {
            Poll::Ready(_) => break,
            Poll::Pending => {
                // Check if we were woken up before going to sleep
                if TASK_WAKE_FLAG.load(Ordering::Acquire) {
                    TASK_WAKE_FLAG.store(false, Ordering::Release);
                    continue; // Don't sleep, poll again
                }
                riscv::asm::wfi();
            }
        }
    }

    unsafe { riscv::register::mstatus::clear_mie() };
    loop {
        riscv::asm::wfi();
    }
}
