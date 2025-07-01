use crate::glb::v2::RegisterBlock;
use crate::gpio::FlexPad;
use core::marker::PhantomData;

/// Transmit signal, combined from a GPIO pad and a UART multiplexer.
pub struct Transmit<'a, const I: usize> {
    _mux_base: PhantomData<&'a RegisterBlock>,
    _pad: PhantomData<FlexPad<'a>>,
}

impl<'a, const I: usize> Transmit<'a, I> {
    #[doc(hidden)]
    #[inline]
    pub const fn __new(mux_base: &'a RegisterBlock, pad: FlexPad<'a>) -> Self {
        let _ = (mux_base, pad);
        Transmit {
            _mux_base: PhantomData,
            _pad: PhantomData,
        }
    }
}

/// Receive signal, combined from a GPIO pad and a UART multiplexer.
pub struct Receive<'a, const I: usize> {
    _mux_base: PhantomData<&'a RegisterBlock>,
    _pad: PhantomData<FlexPad<'a>>,
}

impl<'a, const I: usize> Receive<'a, I> {
    #[doc(hidden)]
    #[inline]
    pub const fn __new(mux_base: &'a RegisterBlock, pad: FlexPad<'a>) -> Self {
        let _ = (mux_base, pad);
        Receive {
            _mux_base: PhantomData,
            _pad: PhantomData,
        }
    }
}

/// Request-To-Send (RTS) signal, combined from a GPIO pad and a UART multiplexer.
pub struct RequestToSend<'a, const I: usize> {
    _mux_base: PhantomData<&'a RegisterBlock>,
    _pad: PhantomData<FlexPad<'a>>,
}

impl<'a, const I: usize> RequestToSend<'a, I> {
    #[doc(hidden)]
    #[inline]
    pub const fn __new(mux_base: &'a RegisterBlock, pad: FlexPad<'a>) -> Self {
        let _ = (mux_base, pad);
        RequestToSend {
            _mux_base: PhantomData,
            _pad: PhantomData,
        }
    }
}

/// Clear-To-Send (CTS) signal, combined from a GPIO pad and a UART multiplexer.
pub struct ClearToSend<'a, const I: usize> {
    _mux_base: PhantomData<&'a RegisterBlock>,
    _pad: PhantomData<FlexPad<'a>>,
}

impl<'a, const I: usize> ClearToSend<'a, I> {
    #[doc(hidden)]
    #[inline]
    pub const fn __new(mux_base: &'a RegisterBlock, pad: FlexPad<'a>) -> Self {
        let _ = (mux_base, pad);
        ClearToSend {
            _mux_base: PhantomData,
            _pad: PhantomData,
        }
    }
}

/// Signals that can be converted into valid UART signals.
pub trait IntoSignals<'a, const I: usize> {
    /// Checks if this configuration includes Request-to-Send feature.
    const RTS: bool;
    /// Checks if this configuration includes Clear-to-Send feature.
    const CTS: bool;
    /// Checks if this configuration includes Transmit feature.
    const TXD: bool;
    /// Checks if this configuration includes Receive feature.
    const RXD: bool;
}

impl<'a, const I: usize> IntoSignals<'a, I> for Transmit<'a, I> {
    const TXD: bool = true;
    const RXD: bool = false;
    const RTS: bool = false;
    const CTS: bool = false;
}

impl<'a, const I: usize> IntoSignals<'a, I> for Receive<'a, I> {
    const TXD: bool = false;
    const RXD: bool = true;
    const RTS: bool = false;
    const CTS: bool = false;
}

impl<'a, const I: usize> IntoSignals<'a, I> for (Transmit<'a, I>, Receive<'a, I>) {
    const TXD: bool = true;
    const RXD: bool = true;
    const RTS: bool = false;
    const CTS: bool = false;
}

impl<'a, const I: usize> IntoSignals<'a, I>
    for (
        Transmit<'a, I>,
        Receive<'a, I>,
        RequestToSend<'a, I>,
        ClearToSend<'a, I>,
    )
{
    const TXD: bool = true;
    const RXD: bool = true;
    const RTS: bool = true;
    const CTS: bool = true;
}
