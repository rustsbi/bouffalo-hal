use crate::hbn;
use embedded_time::rate::Hertz;

// Ref: bouffalo_hal, drivers\soc\bl808\std\src\bl808_clock.c

/// BL808 SoC clocks.
pub struct Clocks {
    /// External crystal clock.
    pub xtal: Hertz,
    /// Root clock source.
    pub root_source: hbn::RootClockSource,
    /// Shared clock source for UART peripherals.
    pub uart_source: hbn::UartClockSource,
    /// Shared clock divide parameter for UART peripherals.
    ///
    /// Must be less than or equal to `7`. The actual divide factor equals `uart_divide + 1`.
    pub uart_divide: u8,
    /// Shared clock gate for UART peripherals.
    pub uart_enable: bool,
    // TODO dsp uart
}

impl Clocks {
    /// Default clocks at ROM init.
    #[doc(hidden)]
    #[inline]
    pub fn __new(xtal_hz: u32) -> Self {
        Clocks {
            xtal: Hertz(xtal_hz),
            root_source: hbn::RootClockSource::Xtal,
            uart_source: hbn::UartClockSource::Xclk,
            uart_divide: 0,
            uart_enable: true,
        }
    }
    /// Get `xclk` frequency of this clock configuration.
    #[inline]
    pub fn xclk(&self) -> Hertz {
        match self.root_source {
            hbn::RootClockSource::Xtal => self.xtal,
            hbn::RootClockSource::RC32M => Hertz(32_000_000),
        }
    }
    /// Get output frequency of the UART clock multiplexer.
    #[inline]
    pub fn uart_clock_mux_output(&self) -> Hertz {
        match self.uart_source {
            hbn::UartClockSource::McuBclk => todo!(),
            hbn::UartClockSource::MuxPll160M => todo!(),
            hbn::UartClockSource::Xclk => self.xclk(),
        }
    }
}

impl<'a> crate::uart::ClockSource for &'a Clocks {
    #[inline]
    fn uart_clock<const I: usize>(self) -> Hertz {
        match I {
            // TODO verify this match arm
            0..=2 => self.uart_clock_mux_output() / (self.uart_divide as u32 + 1),
            // TODO calculate from Clocks structure fields
            3..=4 => Hertz(160_000_000),
            _ => unreachable!(),
        }
    }
}
