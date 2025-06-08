//! Inter-Integrated Circuit bus.

mod blocking;
mod pads;
mod register;
pub use blocking::I2c;
pub use pads::{IntoI2cScl, IntoI2cSda, IntoPads};
pub use register::*;

/// I2C error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Other,
}

impl embedded_hal::i2c::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        use embedded_hal::i2c::ErrorKind;
        match self {
            Error::Other => ErrorKind::Other,
        }
    }
}

/// Peripheral instance for I2C.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// I2C peripheral instance with a number.
pub trait Numbered<'a, const N: usize>: Instance<'a> {}
