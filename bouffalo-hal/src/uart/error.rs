use core::fmt;

/// Serial error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Framing error.
    Framing,
    /// Noise error.
    Noise,
    /// RX buffer overrun.
    Overrun,
    /// Parity check error.
    Parity,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Framing => write!(f, "UART framing error"),
            Self::Noise => write!(f, "UART noise error"),
            Self::Overrun => write!(f, "UART receive buffer overrun"),
            Self::Parity => write!(f, "UART parity error"),
        }
    }
}

impl core::error::Error for Error {}

impl embedded_io::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl embedded_io_06::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_io_06::ErrorKind {
        embedded_io_06::ErrorKind::Other
    }
}

impl embedded_hal_nb::serial::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        match self {
            Error::Framing => embedded_hal_nb::serial::ErrorKind::FrameFormat,
            Error::Noise => embedded_hal_nb::serial::ErrorKind::Noise,
            Error::Overrun => embedded_hal_nb::serial::ErrorKind::Overrun,
            Error::Parity => embedded_hal_nb::serial::ErrorKind::Parity,
        }
    }
}
