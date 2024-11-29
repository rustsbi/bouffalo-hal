//! Universal Asynchronous Receiver/Transmitter.
use crate::clocks::Clocks;
use core::ops::Deref;

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
pub trait UartExt<PADS>: Sized {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    fn freerun<const I: usize>(
        self,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
    ) -> Result<BlockingSerial<Self, PADS>, ConfigError>
    where
        PADS: Pads<I>;
    /// Creates an interrupt driven async/await serial instance without DMA configurations.
    fn with_interrupt<const I: usize>(
        self,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
        state: &'static SerialState,
    ) -> Result<AsyncSerial<Self, PADS>, ConfigError>
    where
        PADS: Pads<I>;
}

impl<UART: Deref<Target = RegisterBlock>, PADS> UartExt<PADS> for UART {
    #[inline]
    fn freerun<const I: usize>(
        self,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
    ) -> Result<BlockingSerial<Self, PADS>, ConfigError>
    where
        PADS: Pads<I>,
    {
        BlockingSerial::freerun(self, config, pads, clocks)
    }
    #[inline]
    fn with_interrupt<const I: usize>(
        self,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
        state: &'static SerialState,
    ) -> Result<AsyncSerial<Self, PADS>, ConfigError>
    where
        PADS: Pads<I>,
    {
        AsyncSerial::new(self, config, pads, clocks, state)
    }
}
