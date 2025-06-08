use crate::gpio::FlexPad;

/// Pads that can be converted into valid full-duplex SPI pads.
pub trait IntoPads<'a, const I: usize> {
    /// Convert this set of pad into SPI alternate function with full-duplex signal support.
    fn into_full_duplex_pads(self) -> (FlexPad<'a>, FlexPad<'a>, FlexPad<'a>, FlexPad<'a>);
}

impl<'a, A, B, C, D, const I: usize> IntoPads<'a, I> for (A, B, C, D)
where
    A: IntoSpiClkSignal<'a, I>,
    B: IntoSpiMosiSignal<'a, I>,
    C: IntoSpiMisoSignal<'a, I>,
    D: IntoSpiCsSignal<'a, I>,
{
    #[inline]
    fn into_full_duplex_pads(self) -> (FlexPad<'a>, FlexPad<'a>, FlexPad<'a>, FlexPad<'a>) {
        let a = self.0.into_spi_clk_signal();
        let b = self.1.into_spi_mosi_signal();
        let c = self.2.into_spi_miso_signal();
        let d = self.3.into_spi_cs_signal();
        (a, b, c, d)
    }
}

/// Pads that can be converted into valid SPI pads with transmit feature only.
pub trait IntoTransmitOnly<'a, const I: usize> {
    /// Convert this set of pad into SPI alternate function with transmit-only signal support.
    fn into_transmit_only_pads(self) -> (FlexPad<'a>, FlexPad<'a>, FlexPad<'a>);
}

impl<'a, A, B, C, const I: usize> IntoTransmitOnly<'a, I> for (A, B, C)
where
    A: IntoSpiClkSignal<'a, I>,
    B: IntoSpiMosiSignal<'a, I>,
    C: IntoSpiCsSignal<'a, I>,
{
    #[inline]
    fn into_transmit_only_pads(self) -> (FlexPad<'a>, FlexPad<'a>, FlexPad<'a>) {
        let a = self.0.into_spi_clk_signal();
        let b = self.1.into_spi_mosi_signal();
        let c = self.2.into_spi_cs_signal();
        (a, b, c)
    }
}

/// Pad that can be configured into SPI clock alternate function.
pub trait IntoSpiClkSignal<'a, const I: usize> {
    /// Configure this pad into SPI clock signal.
    fn into_spi_clk_signal(self) -> FlexPad<'a>;
}

/// Pad that can be configured into SPI MOSI alternate function.
pub trait IntoSpiMosiSignal<'a, const I: usize> {
    /// Configure this pad into SPI MOSI signal.
    fn into_spi_mosi_signal(self) -> FlexPad<'a>;
}

/// Pad that can be configured into SPI MISO alternate function.
pub trait IntoSpiMisoSignal<'a, const I: usize> {
    /// Configure this pad into SPI MISO signal.
    fn into_spi_miso_signal(self) -> FlexPad<'a>;
}

/// Pad that can be configured into SPI chip select alternate function.
pub trait IntoSpiCsSignal<'a, const I: usize> {
    /// Configure this pad into SPI chip select signal.
    fn into_spi_cs_signal(self) -> FlexPad<'a>;
}
