use crate::glb::{self, v2::UartSignal};
use core::marker::PhantomData;

/// Multiplex to Request-to-Send (type state).
pub struct MuxRts<const I: usize>;

/// Multiplex to Clear-to-Send (type state).
pub struct MuxCts<const I: usize>;

/// Multiplex to Transmit (type state).
pub struct MuxTxd<const I: usize>;

/// Multiplex to Receive (type state).
pub struct MuxRxd<const I: usize>;

impl<const I: usize> MuxRts<I> {
    #[inline]
    fn signal() -> UartSignal {
        match I {
            0 => UartSignal::Rts0,
            1 => UartSignal::Rts1,
            2 => UartSignal::Rts2,
            _ => unreachable!(),
        }
    }
}

impl<const I: usize> MuxCts<I> {
    #[inline]
    fn signal() -> UartSignal {
        match I {
            0 => UartSignal::Cts0,
            1 => UartSignal::Cts1,
            2 => UartSignal::Cts2,
            _ => unreachable!(),
        }
    }
}

impl<const I: usize> MuxTxd<I> {
    #[inline]
    fn signal() -> UartSignal {
        match I {
            0 => UartSignal::Txd0,
            1 => UartSignal::Txd1,
            2 => UartSignal::Txd2,
            _ => unreachable!(),
        }
    }
}

impl<const I: usize> MuxRxd<I> {
    #[inline]
    fn signal() -> UartSignal {
        match I {
            0 => UartSignal::Rxd0,
            1 => UartSignal::Rxd1,
            2 => UartSignal::Rxd2,
            _ => unreachable!(),
        }
    }
}

/// Global peripheral UART signal multiplexer.
///
/// This structure only owns the 'a signal multiplexer for signal number `N`.
pub struct UartMux<'a, const N: usize, M> {
    base: &'a glb::v2::RegisterBlock,
    _mode: PhantomData<M>,
}

impl<'a, const N: usize, M> UartMux<'a, N, M> {
    /// Configure the internal UART signal to Request-to-Send (RTS).
    #[inline]
    pub fn into_request_to_send<const U: usize>(self) -> UartMux<'a, N, MuxRts<U>> {
        let config = self.base.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxRts::<U>::signal());
        unsafe { self.base.uart_mux_group[N >> 3].write(config) };
        UartMux {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configure the internal UART signal to Transmit (TXD).
    #[inline]
    pub fn into_transmit<const U: usize>(self) -> UartMux<'a, N, MuxTxd<U>> {
        let config = self.base.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxTxd::<U>::signal());
        unsafe { self.base.uart_mux_group[N >> 3].write(config) };
        UartMux {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configure the internal UART signal to Receive (RXD).
    #[inline]
    pub fn into_receive<const U: usize>(self) -> UartMux<'a, N, MuxRxd<U>> {
        let config = self.base.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxRxd::<U>::signal());
        unsafe { self.base.uart_mux_group[N >> 3].write(config) };
        UartMux {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configure the internal UART signal to Clear-to-Send (CTS).
    #[inline]
    pub fn into_clear_to_send<const U: usize>(self) -> UartMux<'a, N, MuxCts<U>> {
        let config = self.base.uart_mux_group[N >> 3]
            .read()
            .set_signal(N & 0x7, MuxCts::<U>::signal());
        unsafe { self.base.uart_mux_group[N >> 3].write(config) };
        UartMux {
            base: self.base,
            _mode: PhantomData,
        }
    }
}

/// Available UART signal multiplexers.
pub struct UartMuxes<'a> {
    /// Multiplexer of UART signal 0.
    pub sig0: UartMux<'a, 0, MuxRts<0>>,
    /// Multiplexer of UART signal 1.
    pub sig1: UartMux<'a, 1, MuxRts<0>>,
    /// Multiplexer of UART signal 2.
    pub sig2: UartMux<'a, 2, MuxRts<0>>,
    /// Multiplexer of UART signal 3.
    pub sig3: UartMux<'a, 3, MuxRts<0>>,
    /// Multiplexer of UART signal 4.
    pub sig4: UartMux<'a, 4, MuxRts<0>>,
    /// Multiplexer of UART signal 5.
    pub sig5: UartMux<'a, 5, MuxRts<0>>,
    /// Multiplexer of UART signal 6.
    pub sig6: UartMux<'a, 6, MuxRts<0>>,
    /// Multiplexer of UART signal 7.
    pub sig7: UartMux<'a, 7, MuxRts<0>>,
    /// Multiplexer of UART signal 8.
    pub sig8: UartMux<'a, 8, MuxRts<0>>,
    /// Multiplexer of UART signal 9.
    pub sig9: UartMux<'a, 9, MuxRts<0>>,
    /// Multiplexer of UART signal 10.
    pub sig10: UartMux<'a, 10, MuxRts<0>>,
    /// Multiplexer of UART signal 11.
    pub sig11: UartMux<'a, 11, MuxRts<0>>,
}

// Macro internal functions, do not use.

impl<'a, const N: usize, M> UartMux<'a, N, M> {
    #[doc(hidden)]
    #[inline]
    pub fn __from_glb(base: &'a glb::v2::RegisterBlock) -> Self {
        Self {
            base,
            _mode: PhantomData,
        }
    }
}
impl<'a> UartMuxes<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn __uart_muxes_from_glb(base: &'a glb::v2::RegisterBlock) -> Self {
        Self {
            sig0: UartMux::__from_glb(base),
            sig1: UartMux::__from_glb(base),
            sig2: UartMux::__from_glb(base),
            sig3: UartMux::__from_glb(base),
            sig4: UartMux::__from_glb(base),
            sig5: UartMux::__from_glb(base),
            sig6: UartMux::__from_glb(base),
            sig7: UartMux::__from_glb(base),
            sig8: UartMux::__from_glb(base),
            sig9: UartMux::__from_glb(base),
            sig10: UartMux::__from_glb(base),
            sig11: UartMux::__from_glb(base),
        }
    }
}
