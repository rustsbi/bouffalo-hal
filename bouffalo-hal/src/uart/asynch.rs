use super::{
    Config, ConfigError, Error, Instance, Interrupt, InterruptClear, Pads, RegisterBlock,
    uart_config,
};
use crate::clocks::Clocks;
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};

/// Managed async/await serial peripheral.
pub struct AsyncSerial<'a, PADS> {
    uart: &'a RegisterBlock,
    pads: PADS,
    state: &'a SerialState,
}

impl<'a, PADS> AsyncSerial<'a, PADS> {
    /// Creates the async/await serial peripheral from owned peripheral structure, configuration, pads
    /// and a waker registry.
    #[inline]
    pub fn new<const I: usize>(
        uart: impl Instance<'a>,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
        state: &'a SerialState,
    ) -> Result<Self, ConfigError>
    where
        PADS: Pads<I>,
    {
        let uart = uart.register_block();
        // Calculate transmit interval and register values from configuration.
        let (bit_period, data_config, transmit_config, receive_config) =
            uart_config::<I, PADS>(config, &clocks)?;

        // Write bit period.
        unsafe { uart.bit_period.write(bit_period) };
        // Write the bit-order.
        unsafe { uart.data_config.write(data_config) };
        // Configure transmit feature with freerun.
        unsafe { uart.transmit_config.write(transmit_config.enable_freerun()) };
        // Configure receive feature.
        unsafe { uart.receive_config.write(receive_config) };

        state
            .ref_to_serial
            .store(&*uart as *const _ as usize, Ordering::Release);

        Ok(AsyncSerial { uart, pads, state })
    }

    /// Release serial instance and return its peripheral and pads.
    #[inline]
    pub fn free(self) -> PADS {
        self.pads
    }
}

/// Set of wakers as the state for an async/await serial peripheral.
#[derive(Debug)]
pub struct SerialState {
    transmit_ready: atomic_waker::AtomicWaker,
    receive_ready: atomic_waker::AtomicWaker,
    ref_to_serial: AtomicUsize,
}

impl SerialState {
    /// Creates the set of wakers for a serial peripheral.
    #[inline]
    pub const fn new() -> SerialState {
        SerialState {
            transmit_ready: atomic_waker::AtomicWaker::new(),
            receive_ready: atomic_waker::AtomicWaker::new(),
            ref_to_serial: AtomicUsize::new(0),
        }
    }
    /// Use this waker set to handle interrupt.
    #[inline]
    pub fn on_interrupt(&self) {
        let uart =
            unsafe { &*(self.ref_to_serial.load(Ordering::Acquire) as *const RegisterBlock) };
        let state = uart.interrupt_state.read();
        for (interrupt, waker) in [
            (Interrupt::ReceiveFifoReady, &self.receive_ready),
            (Interrupt::TransmitFifoReady, &self.transmit_ready),
        ] {
            if state.has_interrupt(interrupt) {
                waker.wake();
                unsafe {
                    uart.interrupt_clear
                        .write(InterruptClear::default().clear_interrupt(interrupt))
                };
            }
        }
    }
}

struct WaitForInterrupt<'r> {
    uart: &'r RegisterBlock,
    interrupt: Interrupt,
    registry: &'r atomic_waker::AtomicWaker,
}

impl<'r> WaitForInterrupt<'r> {
    #[inline]
    pub const fn new(
        uart: &'r RegisterBlock,
        interrupt: Interrupt,
        registry: &'r atomic_waker::AtomicWaker,
    ) -> Self {
        Self {
            uart,
            interrupt,
            registry,
        }
    }
}

impl Future for WaitForInterrupt<'_> {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self
            .uart
            .interrupt_state
            .read()
            .has_interrupt(self.interrupt)
        {
            Poll::Ready(())
        } else {
            self.registry.register(cx.waker());
            Poll::Pending
        }
    }
}

#[inline]
async fn uart_write_async(
    uart: &RegisterBlock,
    buf: &[u8],
    registry: &atomic_waker::AtomicWaker,
) -> Result<usize, Error> {
    let buf = match buf.len() {
        0 => return Ok(0),
        _ => buf,
    };
    unsafe {
        uart.interrupt_enable
            .modify(|val| val.enable_interrupt(Interrupt::TransmitFifoReady))
    };
    WaitForInterrupt::new(uart, Interrupt::TransmitFifoReady, registry).await;
    let len = core::cmp::min(
        uart.fifo_config_1.read().transmit_available_bytes() as usize,
        buf.len(),
    );
    buf.iter()
        .take(len)
        .for_each(|&word| unsafe { uart.fifo_write.write(word) });
    Ok(len)
}

#[inline]
async fn uart_read_async(
    uart: &RegisterBlock,
    buf: &mut [u8],
    registry: &atomic_waker::AtomicWaker,
) -> Result<usize, Error> {
    let buf = match buf.len() {
        0 => return Ok(0),
        _ => buf,
    };
    unsafe {
        uart.interrupt_enable
            .modify(|val| val.enable_interrupt(Interrupt::ReceiveFifoReady))
    };
    WaitForInterrupt::new(uart, Interrupt::ReceiveFifoReady, registry).await;
    let len = core::cmp::min(
        uart.fifo_config_1.read().receive_available_bytes() as usize,
        buf.len(),
    );
    buf.iter_mut()
        .take(len)
        .for_each(|slot| *slot = uart.fifo_read.read());
    Ok(len)
}

impl<'a, PADS> embedded_io_async::ErrorType for AsyncSerial<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_io_async::Write for AsyncSerial<'a, PADS> {
    #[inline]
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        uart_write_async(&self.uart, buf, &self.state.transmit_ready).await
    }
}

impl<'a, PADS> embedded_io_async::Read for AsyncSerial<'a, PADS> {
    #[inline]
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read_async(&self.uart, buf, &self.state.receive_ready).await
    }
}
