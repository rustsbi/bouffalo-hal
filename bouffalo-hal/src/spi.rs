//! Serial Peripheral Interface peripheral.

mod master;
mod pads;
mod register;

pub use master::Spi;
pub use pads::Pads;
pub use register::*;

/// SPI error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Other,
}

impl embedded_hal::spi::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        use embedded_hal::spi::ErrorKind;
        match self {
            Error::Other => ErrorKind::Other,
        }
    }
}

/// Peripheral instance for SPI.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}
