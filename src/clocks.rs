//! System-on-Chip clock configuration.

use embedded_time::rate::Hertz;

/// Clock settings for current chip.
#[derive(Debug, Clone)]
pub struct Clocks {
    // todo: clock setting fields
    pub xtal: Hertz,
}

impl Clocks {
    /// Crystal oscillator clock frequency.
    #[inline]
    pub const fn xclk(&self) -> Hertz {
        self.xtal
    }
    /// Universal Asynchronous Receiver/Transmitter clock frequency.
    #[inline]
    pub const fn uart_clock<const I: usize>(&self) -> Option<Hertz> {
        // todo: calculate from Clocks structure fields
        Some(Hertz(80_000_000))
    }
}
