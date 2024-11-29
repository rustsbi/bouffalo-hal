use crate::glb::{self, v2::UartSignal};
use core::{marker::PhantomData, ops::Deref};

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
/// This structure only owns the GLB signal multiplexer for signal number `N`.
pub struct UartMux<GLB, const N: usize, M> {
    base: GLB,
    _mode: PhantomData<M>,
}

impl<GLB: Deref<Target = glb::v2::RegisterBlock>, const N: usize, M> UartMux<GLB, N, M> {
    /// Configure the internal UART signal to Request-to-Send (RTS).
    #[inline]
    pub fn into_request_to_send<const U: usize>(self) -> UartMux<GLB, N, MuxRts<U>> {
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
    pub fn into_transmit<const U: usize>(self) -> UartMux<GLB, N, MuxTxd<U>> {
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
    pub fn into_receive<const U: usize>(self) -> UartMux<GLB, N, MuxRxd<U>> {
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
    pub fn into_clear_to_send<const U: usize>(self) -> UartMux<GLB, N, MuxCts<U>> {
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
pub struct UartMuxes<GLB> {
    /// Multiplexer of UART signal 0.
    pub sig0: UartMux<GLB, 0, MuxRts<0>>,
    /// Multiplexer of UART signal 1.
    pub sig1: UartMux<GLB, 1, MuxRts<0>>,
    /// Multiplexer of UART signal 2.
    pub sig2: UartMux<GLB, 2, MuxRts<0>>,
    /// Multiplexer of UART signal 3.
    pub sig3: UartMux<GLB, 3, MuxRts<0>>,
    /// Multiplexer of UART signal 4.
    pub sig4: UartMux<GLB, 4, MuxRts<0>>,
    /// Multiplexer of UART signal 5.
    pub sig5: UartMux<GLB, 5, MuxRts<0>>,
    /// Multiplexer of UART signal 6.
    pub sig6: UartMux<GLB, 6, MuxRts<0>>,
    /// Multiplexer of UART signal 7.
    pub sig7: UartMux<GLB, 7, MuxRts<0>>,
    /// Multiplexer of UART signal 8.
    pub sig8: UartMux<GLB, 8, MuxRts<0>>,
    /// Multiplexer of UART signal 9.
    pub sig9: UartMux<GLB, 9, MuxRts<0>>,
    /// Multiplexer of UART signal 10.
    pub sig10: UartMux<GLB, 10, MuxRts<0>>,
    /// Multiplexer of UART signal 11.
    pub sig11: UartMux<GLB, 11, MuxRts<0>>,
}
