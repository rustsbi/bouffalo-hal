use crate::gpio::{self, Alternate};

/// Valid SPI pads.
pub trait Pads<const I: usize> {}

impl<'a, 'b, 'c, const N1: usize, const N2: usize, const N3: usize> Pads<1>
    for (
        Alternate<'a, N1, gpio::Spi<1>>,
        Alternate<'b, N2, gpio::Spi<1>>,
        Alternate<'c, N3, gpio::Spi<1>>,
    )
where
    Alternate<'a, N1, gpio::Spi<1>>: HasClkSignal,
    Alternate<'b, N2, gpio::Spi<1>>: HasMosiSignal,
    Alternate<'c, N3, gpio::Spi<1>>: HasCsSignal,
{
}

impl<'a, 'b, 'c, 'd, const N1: usize, const N2: usize, const N3: usize, const N4: usize> Pads<1>
    for (
        Alternate<'a, N1, gpio::Spi<1>>,
        Alternate<'b, N2, gpio::Spi<1>>,
        Alternate<'c, N3, gpio::Spi<1>>,
        Alternate<'d, N4, gpio::Spi<1>>,
    )
where
    Alternate<'a, N1, gpio::Spi<1>>: HasClkSignal,
    Alternate<'b, N2, gpio::Spi<1>>: HasMosiSignal,
    Alternate<'c, N3, gpio::Spi<1>>: HasMisoSignal,
    Alternate<'d, N4, gpio::Spi<1>>: HasCsSignal,
{
}

/// Check if target gpio `Pin` is internally connected to SPI clock signal.
pub trait HasClkSignal {}

impl<'a> HasClkSignal for Alternate<'a, 3, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 7, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 11, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 15, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 19, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 23, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 27, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 31, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 35, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 39, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 43, gpio::Spi<1>> {}

/// Check if target gpio `Pin` is internally connected to SPI MISO signal.
pub trait HasMisoSignal {}

impl<'a> HasMisoSignal for Alternate<'a, 2, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 6, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 10, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 14, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 18, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 22, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 26, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 30, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 34, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 38, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 42, gpio::Spi<1>> {}

/// Check if target gpio `Pin` is internally connected to SPI MOSI signal.
pub trait HasMosiSignal {}

impl<'a> HasMosiSignal for Alternate<'a, 1, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 5, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 9, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 13, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 17, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 21, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 25, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 29, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 33, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 37, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 41, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 45, gpio::Spi<1>> {}

/// Check if target gpio `Pin` is internally connected to SPI CS signal.
pub trait HasCsSignal {}

impl<'a> HasCsSignal for Alternate<'a, 0, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 4, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 8, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 12, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 16, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 20, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 24, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 28, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 32, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 36, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 40, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 44, gpio::Spi<1>> {}
