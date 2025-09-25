//! Universal Asynchronous Receiver/Transmitter.

mod register;
pub use register::*;
mod mux;
pub use mux::*;
mod pad;
pub use pad::*;
mod config;
pub use config::*;
mod error;
pub use error::*;
mod blocking;
pub use blocking::*;
mod asynch;
pub use asynch::*;
mod signal;
pub use signal::*;

use embedded_time::rate::Hertz;

/// Extend constructor to owned UART register blocks.
pub trait UartExt<'a, const I: usize> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    fn freerun(
        self,
        config: Config,
        pads: impl IntoSignals<'a, I>,
        clocks: impl Clock,
    ) -> Result<BlockingSerial<'a>, ConfigError>;
    /// Creates an interrupt driven async/await serial instance without DMA configurations.
    fn with_interrupt(
        self,
        config: Config,
        pads: impl IntoSignals<'a, I>,
        clocks: impl Clock,
        state: &'static SerialState,
    ) -> Result<AsyncSerial<'a>, ConfigError>;
}

/// Peripheral instance for UART.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// UART instance with a peripheral number.
pub trait Numbered<'a, const I: usize>: Instance<'a> {}

/// UART clock source.
pub trait Clock {
    /// Clock frequency in hertz.
    fn uart_clock<const I: usize>(self) -> Hertz;
}
