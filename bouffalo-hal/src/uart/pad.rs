use super::{BlockingReceiveHalf, BlockingTransmitHalf, MuxCts, MuxRts, MuxRxd, MuxTxd, UartMux};
use crate::glb;
use crate::gpio::{Alternate, MmUart, Uart};
use core::ops::Deref;

/// Check if target gpio `Pin` is internally connected to UART signal index `I`.
pub trait HasUartSignal<const I: usize> {}

impl<GLB> HasUartSignal<0> for Alternate<GLB, 0, Uart> {}
impl<GLB> HasUartSignal<1> for Alternate<GLB, 1, Uart> {}
impl<GLB> HasUartSignal<2> for Alternate<GLB, 2, Uart> {}
impl<GLB> HasUartSignal<3> for Alternate<GLB, 3, Uart> {}
impl<GLB> HasUartSignal<4> for Alternate<GLB, 4, Uart> {}
impl<GLB> HasUartSignal<5> for Alternate<GLB, 5, Uart> {}
impl<GLB> HasUartSignal<6> for Alternate<GLB, 6, Uart> {}
impl<GLB> HasUartSignal<7> for Alternate<GLB, 7, Uart> {}
impl<GLB> HasUartSignal<8> for Alternate<GLB, 8, Uart> {}
impl<GLB> HasUartSignal<9> for Alternate<GLB, 9, Uart> {}
impl<GLB> HasUartSignal<10> for Alternate<GLB, 10, Uart> {}
impl<GLB> HasUartSignal<11> for Alternate<GLB, 11, Uart> {}
impl<GLB> HasUartSignal<0> for Alternate<GLB, 12, Uart> {}
impl<GLB> HasUartSignal<1> for Alternate<GLB, 13, Uart> {}
impl<GLB> HasUartSignal<2> for Alternate<GLB, 14, Uart> {}
impl<GLB> HasUartSignal<3> for Alternate<GLB, 15, Uart> {}
impl<GLB> HasUartSignal<4> for Alternate<GLB, 16, Uart> {}
impl<GLB> HasUartSignal<5> for Alternate<GLB, 17, Uart> {}
impl<GLB> HasUartSignal<6> for Alternate<GLB, 18, Uart> {}
impl<GLB> HasUartSignal<7> for Alternate<GLB, 19, Uart> {}
impl<GLB> HasUartSignal<8> for Alternate<GLB, 20, Uart> {}
impl<GLB> HasUartSignal<9> for Alternate<GLB, 21, Uart> {}
impl<GLB> HasUartSignal<10> for Alternate<GLB, 22, Uart> {}
impl<GLB> HasUartSignal<11> for Alternate<GLB, 23, Uart> {}
impl<GLB> HasUartSignal<0> for Alternate<GLB, 24, Uart> {}
impl<GLB> HasUartSignal<1> for Alternate<GLB, 25, Uart> {}
impl<GLB> HasUartSignal<2> for Alternate<GLB, 26, Uart> {}
impl<GLB> HasUartSignal<3> for Alternate<GLB, 27, Uart> {}
impl<GLB> HasUartSignal<4> for Alternate<GLB, 28, Uart> {}
impl<GLB> HasUartSignal<5> for Alternate<GLB, 29, Uart> {}
impl<GLB> HasUartSignal<6> for Alternate<GLB, 30, Uart> {}
impl<GLB> HasUartSignal<7> for Alternate<GLB, 31, Uart> {}
impl<GLB> HasUartSignal<8> for Alternate<GLB, 32, Uart> {}
impl<GLB> HasUartSignal<9> for Alternate<GLB, 33, Uart> {}
impl<GLB> HasUartSignal<10> for Alternate<GLB, 34, Uart> {}
impl<GLB> HasUartSignal<11> for Alternate<GLB, 35, Uart> {}
impl<GLB> HasUartSignal<0> for Alternate<GLB, 36, Uart> {}
impl<GLB> HasUartSignal<1> for Alternate<GLB, 37, Uart> {}
impl<GLB> HasUartSignal<2> for Alternate<GLB, 38, Uart> {}
impl<GLB> HasUartSignal<3> for Alternate<GLB, 39, Uart> {}
impl<GLB> HasUartSignal<4> for Alternate<GLB, 40, Uart> {}
impl<GLB> HasUartSignal<5> for Alternate<GLB, 41, Uart> {}
impl<GLB> HasUartSignal<6> for Alternate<GLB, 42, Uart> {}
impl<GLB> HasUartSignal<7> for Alternate<GLB, 43, Uart> {}
impl<GLB> HasUartSignal<8> for Alternate<GLB, 44, Uart> {}
impl<GLB> HasUartSignal<9> for Alternate<GLB, 45, Uart> {}

/// Check if an internal multi-media UART signal is connected to target gpio `Pin`.
pub trait HasMmUartSignal {}

impl<GLB, const N: usize> HasMmUartSignal for Alternate<GLB, N, MmUart> {}

/// Valid UART pads.
#[diagnostic::on_unimplemented(
    message = "the I/O pad and signal multiplexer group {Self} is not connected to any UART peripherals on hardware"
)]
pub trait Pads<const U: usize> {
    /// Checks if this pin configuration includes Request-to-Send feature.
    const RTS: bool;
    /// Checks if this pin configuration includes Clear-to-Send feature.
    const CTS: bool;
    /// Checks if this pin configuration includes Transmit feature.
    const TXD: bool;
    /// Checks if this pin configuration includes Receive feature.
    const RXD: bool;
    /// Valid split configuration type for current pads and multiplexers.
    type Split<T>;

    fn split<T>(self, uart: T) -> Self::Split<T>;
}

#[inline]
fn from_pads<T, TX, RX>(
    uart: T,
    tx: TX,
    rx: RX,
) -> (BlockingTransmitHalf<T, TX>, BlockingReceiveHalf<T, RX>) {
    (
        BlockingTransmitHalf {
            uart: unsafe { core::ptr::read_volatile(&uart) },
            _pads: tx,
        },
        BlockingReceiveHalf { uart, _pads: rx },
    )
}

impl<A1, GLB2, const I: usize, const U: usize, const N: usize> Pads<U>
    for (Alternate<A1, N, Uart>, UartMux<GLB2, I, MuxTxd<U>>)
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N, Uart>: HasUartSignal<I>,
{
    const RTS: bool = false;
    const CTS: bool = false;
    const TXD: bool = true;
    const RXD: bool = false;
    type Split<T> = (
        BlockingTransmitHalf<T, (Alternate<A1, N, Uart>, UartMux<GLB2, I, MuxTxd<U>>)>,
        BlockingReceiveHalf<T, ()>,
    );
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        from_pads(uart, self, ())
    }
}

impl<
        A1,
        GLB2,
        A3,
        GLB4,
        const I1: usize,
        const I2: usize,
        const U: usize,
        const N1: usize,
        const N2: usize,
    > Pads<U>
    for (
        (Alternate<A1, N1, Uart>, UartMux<GLB2, I1, MuxTxd<U>>),
        (Alternate<A3, N2, Uart>, UartMux<GLB4, I2, MuxRxd<U>>),
    )
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    A3: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N1, Uart>: HasUartSignal<I1>,
    Alternate<A3, N2, Uart>: HasUartSignal<I2>,
{
    const RTS: bool = false;
    const CTS: bool = false;
    const TXD: bool = true;
    const RXD: bool = true;
    type Split<T> = (
        BlockingTransmitHalf<T, (Alternate<A1, N1, Uart>, UartMux<GLB2, I1, MuxTxd<U>>)>,
        BlockingReceiveHalf<T, (Alternate<A3, N2, Uart>, UartMux<GLB4, I2, MuxRxd<U>>)>,
    );
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        from_pads(uart, self.0, self.1)
    }
}

impl<
        A1,
        GLB2,
        A3,
        GLB4,
        const I1: usize,
        const I2: usize,
        const U: usize,
        const N1: usize,
        const N2: usize,
    > Pads<U>
    for (
        (Alternate<A1, N1, Uart>, UartMux<GLB2, I1, MuxTxd<U>>),
        (Alternate<A3, N2, Uart>, UartMux<GLB4, I2, MuxCts<U>>),
    )
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    A3: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N1, Uart>: HasUartSignal<I1>,
    Alternate<A3, N2, Uart>: HasUartSignal<I2>,
{
    const RTS: bool = false;
    const CTS: bool = true;
    const TXD: bool = true;
    const RXD: bool = false;
    type Split<T> = BlockingTransmitHalf<
        T,
        (
            (Alternate<A1, N1, Uart>, UartMux<GLB2, I1, MuxTxd<U>>),
            (Alternate<A3, N2, Uart>, UartMux<GLB4, I2, MuxCts<U>>),
        ),
    >;
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        BlockingTransmitHalf { uart, _pads: self }
    }
}

impl<
        A1,
        GLB2,
        A3,
        GLB4,
        A5,
        GLB6,
        A7,
        GLB8,
        const I1: usize,
        const I2: usize,
        const I3: usize,
        const I4: usize,
        const U: usize,
        const N1: usize,
        const N2: usize,
        const N3: usize,
        const N4: usize,
    > Pads<U>
    for (
        (Alternate<A1, N1, Uart>, UartMux<GLB2, I1, MuxTxd<U>>),
        (Alternate<A3, N2, Uart>, UartMux<GLB4, I2, MuxRxd<U>>),
        (Alternate<A5, N3, Uart>, UartMux<GLB6, I3, MuxRts<U>>),
        (Alternate<A7, N4, Uart>, UartMux<GLB8, I4, MuxCts<U>>),
    )
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    A3: Deref<Target = glb::v2::RegisterBlock>,
    A5: Deref<Target = glb::v2::RegisterBlock>,
    A7: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N1, Uart>: HasUartSignal<I1>,
    Alternate<A3, N2, Uart>: HasUartSignal<I2>,
    Alternate<A5, N3, Uart>: HasUartSignal<I3>,
    Alternate<A7, N4, Uart>: HasUartSignal<I4>,
{
    const RTS: bool = false;
    const CTS: bool = true;
    const TXD: bool = true;
    const RXD: bool = false;
    type Split<T> = (
        BlockingTransmitHalf<
            T,
            (
                (Alternate<A1, N1, Uart>, UartMux<GLB2, I1, MuxTxd<U>>),
                (Alternate<A7, N4, Uart>, UartMux<GLB8, I4, MuxCts<U>>),
            ),
        >,
        BlockingReceiveHalf<
            T,
            (
                (Alternate<A3, N2, Uart>, UartMux<GLB4, I2, MuxRxd<U>>),
                (Alternate<A5, N3, Uart>, UartMux<GLB6, I3, MuxRts<U>>),
            ),
        >,
    );
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        from_pads(uart, (self.0, self.3), (self.1, self.2))
    }
}

// TODO: support split for MmUart pads.

const MMUART_UART_ID: usize = 3;

impl<A1, const N: usize> Pads<MMUART_UART_ID> for Alternate<A1, N, MmUart>
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N, MmUart>: HasMmUartSignal,
{
    const RTS: bool = { N % 4 == 2 };
    const CTS: bool = { N % 4 == 3 };
    const TXD: bool = { N % 4 == 0 };
    const RXD: bool = { N % 4 == 1 };
    type Split<T> = ();
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        let _ = uart;
        ()
    }
}

impl<A1, A2, const N1: usize, const N2: usize> Pads<MMUART_UART_ID>
    for (Alternate<A1, N1, MmUart>, Alternate<A2, N2, MmUart>)
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    A2: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N1, MmUart>: HasMmUartSignal,
    Alternate<A2, N2, MmUart>: HasMmUartSignal,
{
    const RTS: bool = { N1 % 4 == 2 || N2 % 4 == 2 };
    const CTS: bool = { N1 % 4 == 3 || N2 % 4 == 3 };
    const TXD: bool = { N1 % 4 == 0 || N2 % 4 == 0 };
    const RXD: bool = { N1 % 4 == 1 || N2 % 4 == 1 };
    type Split<T> = ();
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        let _ = uart;
        ()
    }
}

impl<A1, A2, A3, const N1: usize, const N2: usize, const N3: usize> Pads<MMUART_UART_ID>
    for (
        Alternate<A1, N1, MmUart>,
        Alternate<A2, N2, MmUart>,
        Alternate<A3, N3, MmUart>,
    )
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    A2: Deref<Target = glb::v2::RegisterBlock>,
    A3: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N1, MmUart>: HasMmUartSignal,
    Alternate<A2, N2, MmUart>: HasMmUartSignal,
    Alternate<A3, N3, MmUart>: HasMmUartSignal,
{
    const RTS: bool = { N1 % 4 == 2 || N2 % 4 == 2 || N3 % 4 == 2 };
    const CTS: bool = { N1 % 4 == 3 || N2 % 4 == 3 || N3 % 4 == 3 };
    const TXD: bool = { N1 % 4 == 0 || N2 % 4 == 0 || N3 % 4 == 0 };
    const RXD: bool = { N1 % 4 == 1 || N2 % 4 == 1 || N3 % 4 == 1 };
    type Split<T> = ();
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        let _ = uart;
        ()
    }
}

impl<A1, A2, A3, A4, const N1: usize, const N2: usize, const N3: usize, const N4: usize>
    Pads<MMUART_UART_ID>
    for (
        Alternate<A1, N1, MmUart>,
        Alternate<A2, N2, MmUart>,
        Alternate<A3, N3, MmUart>,
        Alternate<A4, N4, MmUart>,
    )
where
    A1: Deref<Target = glb::v2::RegisterBlock>,
    A2: Deref<Target = glb::v2::RegisterBlock>,
    A3: Deref<Target = glb::v2::RegisterBlock>,
    A4: Deref<Target = glb::v2::RegisterBlock>,
    Alternate<A1, N1, MmUart>: HasMmUartSignal,
    Alternate<A2, N2, MmUart>: HasMmUartSignal,
    Alternate<A3, N3, MmUart>: HasMmUartSignal,
    Alternate<A4, N4, MmUart>: HasMmUartSignal,
{
    const RTS: bool = { N1 % 4 == 2 || N2 % 4 == 2 || N3 % 4 == 2 || N4 % 4 == 2 };
    const CTS: bool = { N1 % 4 == 3 || N2 % 4 == 3 || N3 % 4 == 3 || N4 % 4 == 3 };
    const TXD: bool = { N1 % 4 == 0 || N2 % 4 == 0 || N3 % 4 == 0 || N4 % 4 == 0 };
    const RXD: bool = { N1 % 4 == 1 || N2 % 4 == 1 || N3 % 4 == 1 || N4 % 4 == 1 };
    type Split<T> = ();
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        let _ = uart;
        ()
    }
}
