use crate::gpio::{self, Alternate};

/// Valid SDH pads.
pub trait Pads<const I: usize> {}

impl<
    'a,
    'b,
    'c,
    'd,
    'e,
    'f,
    const N1: usize,
    const N2: usize,
    const N3: usize,
    const N4: usize,
    const N5: usize,
    const N6: usize,
> Pads<1>
    for (
        Alternate<'a, N1, gpio::Sdh>,
        Alternate<'b, N2, gpio::Sdh>,
        Alternate<'c, N3, gpio::Sdh>,
        Alternate<'d, N4, gpio::Sdh>,
        Alternate<'e, N5, gpio::Sdh>,
        Alternate<'f, N6, gpio::Sdh>,
    )
where
    Alternate<'a, N1, gpio::Sdh>: HasClkSignal,
    Alternate<'b, N2, gpio::Sdh>: HasCmdSignal,
    Alternate<'c, N3, gpio::Sdh>: HasDat0Signal,
    Alternate<'d, N4, gpio::Sdh>: HasDat1Signal,
    Alternate<'e, N5, gpio::Sdh>: HasDat2Signal,
    Alternate<'f, N6, gpio::Sdh>: HasDat3Signal,
{
}

/// Check if target gpio `Pin` is internally connected to SDH clock signal.
pub trait HasClkSignal {}

impl<'a> HasClkSignal for Alternate<'a, 0, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH command signal.
pub trait HasCmdSignal {}

impl<'a> HasCmdSignal for Alternate<'a, 1, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 0 signal.
pub trait HasDat0Signal {}

impl<'a> HasDat0Signal for Alternate<'a, 2, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 1 signal.
pub trait HasDat1Signal {}

impl<'a> HasDat1Signal for Alternate<'a, 3, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 2 signal.
pub trait HasDat2Signal {}

impl<'a> HasDat2Signal for Alternate<'a, 4, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 3 signal.
pub trait HasDat3Signal {}

impl<'a> HasDat3Signal for Alternate<'a, 5, gpio::Sdh> {}
