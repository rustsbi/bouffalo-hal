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
}
