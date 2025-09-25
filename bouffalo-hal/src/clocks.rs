//! System-on-Chip clock configuration.

pub mod v2;

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
        match I {
            0..=2 => Some(Hertz(80_000_000)),
            3..=4 => Some(Hertz(160_000_000)),
            _ => unreachable!(),
        }
    }
}

impl<'a> crate::uart::Clock for &'a Clocks {
    #[inline]
    fn uart_clock<const I: usize>(self) -> Hertz {
        self.uart_clock::<I>().unwrap()
    }
}

impl<'a> crate::pwm::Clock for &'a Clocks {
    #[inline]
    fn pwm_clock(self, choice: crate::pwm::ClockSource) -> Hertz {
        use crate::pwm::ClockSource;
        match choice {
            ClockSource::Xclk => Hertz(40_000_000),
            ClockSource::Bclk => todo!(),
            ClockSource::F32kClk => todo!(),
        }
    }
}
