//! System-on-Chip clock configuration.

use embedded_time::rate::Hertz;

/// Clock settings for current chip.
#[derive(Debug, Clone)]
pub struct Clocks {
    // todo: clock setting fields
}

impl Clocks {
    /// Universal Asynchronous Receiver/Transmitter clock frequency.
    #[inline]
    pub const fn uart_clock(&self) -> Hertz {
        // todo: calculate from Clocks structure fields
        Hertz(80_000_000)
    }
    /// Crystal oscillator clock frequency.
    #[inline]
    pub const fn xclk(&self) -> Hertz {
        // todo: calculate from Clocks structure fields
        Hertz(40_000_000)
    }
}
