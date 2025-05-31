//! Universal Asynchronous Receiver/Transmitter.
use crate::clocks::Clocks;

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

/// Extend constructor to owned UART register blocks.
pub trait UartExt<'a, PADS, const I: usize> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    fn freerun(
        self,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
    ) -> Result<BlockingSerial<'a, PADS>, ConfigError>
    where
        PADS: Pads<I>;
    /// Creates an interrupt driven async/await serial instance without DMA configurations.
    fn with_interrupt(
        self,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
        state: &'static SerialState,
    ) -> Result<AsyncSerial<'a, PADS>, ConfigError>
    where
        PADS: Pads<I>;
}

/// Peripheral instance for UART.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// UART instance with a peripheral number.
pub trait Numbered<'a, const I: usize>: Instance<'a> {}
