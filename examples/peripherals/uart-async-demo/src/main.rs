#![no_std]
#![no_main]

use bouffalo_hal::{
    prelude::*,
    uart::{Config, SerialState},
};
use bouffalo_rt::{
    Clocks, Peripherals, entry, interrupt,
    soc::bl808::{D0Machine, DspInterrupt},
};
use core::sync::atomic::{AtomicBool, Ordering};
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
    p.plic.set_priority(DspInterrupt::Uart3, 1);
    p.plic.enable(DspInterrupt::Uart3, D0Machine);

    serial.write_all(b"Hello async/await world!\n").await.ok();

    let paragraph = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eu eleifend quam. Maecenas in maximus ex. In quis dolor sit amet risus condimentum egestas sed ut ex. Aenean placerat, mauris vel rutrum sodales, odio felis tempor lectus, et ullamcorper magna sem id mi. Donec laoreet justo vel finibus gravida. Nam eleifend accumsan orci vitae fermentum. Vestibulum dictum arcu sed rhoncus aliquet.
Aenean sollicitudin felis nec nisi scelerisque, quis sodales diam sagittis. Nullam consequat, ex eget porttitor laoreet, purus ipsum lacinia eros, et porttitor metus risus sit amet enim. Maecenas ligula diam, eleifend id massa ac, bibendum varius sapien. Aliquam eget erat vitae nunc consequat maximus. Curabitur erat lacus, laoreet nec purus id, fermentum imperdiet sem. In finibus enim urna, eget varius tortor efficitur a. In sit amet auctor dui. Quisque elementum felis vel lectus dapibus, et consectetur nisi semper. Nunc lacus ante, aliquet non est sed, rhoncus rhoncus mauris. Suspendisse dignissim nibh nec velit convallis, ac accumsan orci auctor. Aliquam ornare consequat hendrerit. Sed lectus nibh, lacinia elementum tristique et, scelerisque id mauris. Mauris ac mollis ipsum.
Ut eu sagittis mi. Cras quis mollis libero. Etiam sed lectus tincidunt, maximus eros et, accumsan mauris. Ut interdum nulla augue, nec sollicitudin diam venenatis sed. Praesent eget elit ut ipsum rutrum egestas nec non dui. Duis ac diam magna. Pellentesque nibh purus, sollicitudin sed vehicula a, pulvinar nec eros. Nullam vitae suscipit enim, eget accumsan diam. Duis imperdiet aliquam efficitur. Cras interdum malesuada elit, non ultricies justo hendrerit quis. Mauris lectus ante, consequat ac lectus sollicitudin, elementum faucibus tellus. Sed pretium placerat diam ultricies sagittis.
Aenean sagittis fringilla ex pharetra gravida. Aenean feugiat tincidunt nulla non elementum. Fusce ut lectus neque. Nam nec aliquam nisi. Vivamus suscipit quam vehicula, pulvinar elit eget, iaculis magna. Curabitur congue, elit vel faucibus ultricies, arcu nisi congue risus, sit amet efficitur mi turpis quis sapien. Sed eu elit eu sem mattis laoreet. Nam ullamcorper, arcu ut eleifend vestibulum, mi augue tempor eros, ac maximus metus lectus quis sapien.
Suspendisse potenti. Nam bibendum, velit quis ullamcorper blandit, nunc odio ultricies diam, vitae euismod arcu neque eu ex. Vivamus et quam massa. Curabitur eget semper nulla, quis convallis nibh. Praesent rutrum dolor in ultrices tincidunt. Suspendisse placerat blandit mi, eget blandit arcu consequat eu. Aliquam suscipit eget velit et mattis. Etiam pulvinar velit a odio consequat, quis consectetur metus consectetur. Maecenas convallis eleifend metus, et dapibus sem eleifend eget.
";

    serial.write_all(paragraph).await.ok();
}

static UART3_STATE: SerialState = SerialState::new();

/// TASK_WAKE_FLAG is used to synchronize between the UART3 interrupt handler and the main async task.
///
/// - The interrupt handler sets this flag to `true` when an interrupt occurs, indicating that the main task should wake up and poll the future again.
/// - The main task checks this flag in its loop; if set, it clears the flag and continues polling without sleeping.
/// - This mechanism ensures that the main task responds promptly to UART events signaled by the interrupt.
///
/// This flag is essential for proper async operation because:
/// 1. Without it, the main task would use `Waker::noop()` which never wakes the task
/// 2. The task would sleep indefinitely with `wfi()` waiting for interrupts
/// 3. When interrupts occur, this flag bridges the gap between hardware events and software task scheduling
static TASK_WAKE_FLAG: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn uart3() {
    UART3_STATE.on_interrupt();
    // Wake the main task after handling interrupt
    TASK_WAKE_FLAG.store(true, Ordering::Release);
}

// ---- Async/await runtime environment ----

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    use core::{
        future::Future,
        task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    };
    p.plic.set_threshold(D0Machine, 0);
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
