//! Inter-Integrated Circuit bus.

mod blocking;
mod pads;
mod register;
pub use blocking::I2c;
pub use pads::{SclPin, SdaPin};
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
