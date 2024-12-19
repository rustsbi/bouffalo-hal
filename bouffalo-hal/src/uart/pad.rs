use super::{BlockingReceiveHalf, BlockingTransmitHalf, MuxCts, MuxRts, MuxRxd, MuxTxd, UartMux};
use crate::gpio::{Alternate, MmUart, Uart};

/// Check if target gpio `Pin` is internally connected to UART signal index `I`.
pub trait HasUartSignal<const I: usize> {}

impl<'a> HasUartSignal<0> for Alternate<'a, 0, Uart> {}
impl<'a> HasUartSignal<1> for Alternate<'a, 1, Uart> {}
impl<'a> HasUartSignal<2> for Alternate<'a, 2, Uart> {}
impl<'a> HasUartSignal<3> for Alternate<'a, 3, Uart> {}
impl<'a> HasUartSignal<4> for Alternate<'a, 4, Uart> {}
impl<'a> HasUartSignal<5> for Alternate<'a, 5, Uart> {}
impl<'a> HasUartSignal<6> for Alternate<'a, 6, Uart> {}
impl<'a> HasUartSignal<7> for Alternate<'a, 7, Uart> {}
impl<'a> HasUartSignal<8> for Alternate<'a, 8, Uart> {}
impl<'a> HasUartSignal<9> for Alternate<'a, 9, Uart> {}
impl<'a> HasUartSignal<10> for Alternate<'a, 10, Uart> {}
impl<'a> HasUartSignal<11> for Alternate<'a, 11, Uart> {}
impl<'a> HasUartSignal<0> for Alternate<'a, 12, Uart> {}
impl<'a> HasUartSignal<1> for Alternate<'a, 13, Uart> {}
impl<'a> HasUartSignal<2> for Alternate<'a, 14, Uart> {}
impl<'a> HasUartSignal<3> for Alternate<'a, 15, Uart> {}
impl<'a> HasUartSignal<4> for Alternate<'a, 16, Uart> {}
impl<'a> HasUartSignal<5> for Alternate<'a, 17, Uart> {}
impl<'a> HasUartSignal<6> for Alternate<'a, 18, Uart> {}
impl<'a> HasUartSignal<7> for Alternate<'a, 19, Uart> {}
impl<'a> HasUartSignal<8> for Alternate<'a, 20, Uart> {}
impl<'a> HasUartSignal<9> for Alternate<'a, 21, Uart> {}
impl<'a> HasUartSignal<10> for Alternate<'a, 22, Uart> {}
impl<'a> HasUartSignal<11> for Alternate<'a, 23, Uart> {}
impl<'a> HasUartSignal<0> for Alternate<'a, 24, Uart> {}
impl<'a> HasUartSignal<1> for Alternate<'a, 25, Uart> {}
impl<'a> HasUartSignal<2> for Alternate<'a, 26, Uart> {}
impl<'a> HasUartSignal<3> for Alternate<'a, 27, Uart> {}
impl<'a> HasUartSignal<4> for Alternate<'a, 28, Uart> {}
impl<'a> HasUartSignal<5> for Alternate<'a, 29, Uart> {}
impl<'a> HasUartSignal<6> for Alternate<'a, 30, Uart> {}
impl<'a> HasUartSignal<7> for Alternate<'a, 31, Uart> {}
impl<'a> HasUartSignal<8> for Alternate<'a, 32, Uart> {}
impl<'a> HasUartSignal<9> for Alternate<'a, 33, Uart> {}
impl<'a> HasUartSignal<10> for Alternate<'a, 34, Uart> {}
impl<'a> HasUartSignal<11> for Alternate<'a, 35, Uart> {}
impl<'a> HasUartSignal<0> for Alternate<'a, 36, Uart> {}
impl<'a> HasUartSignal<1> for Alternate<'a, 37, Uart> {}
impl<'a> HasUartSignal<2> for Alternate<'a, 38, Uart> {}
impl<'a> HasUartSignal<3> for Alternate<'a, 39, Uart> {}
impl<'a> HasUartSignal<4> for Alternate<'a, 40, Uart> {}
impl<'a> HasUartSignal<5> for Alternate<'a, 41, Uart> {}
impl<'a> HasUartSignal<6> for Alternate<'a, 42, Uart> {}
impl<'a> HasUartSignal<7> for Alternate<'a, 43, Uart> {}
impl<'a> HasUartSignal<8> for Alternate<'a, 44, Uart> {}
impl<'a> HasUartSignal<9> for Alternate<'a, 45, Uart> {}

/// Check if an internal multi-media UART signal is connected to target gpio `Pin`.
pub trait HasMmUartSignal {}

impl<'a, const N: usize> HasMmUartSignal for Alternate<'a, N, MmUart> {}

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

impl<'a, 'b, const I: usize, const U: usize, const N: usize> Pads<U>
    for (Alternate<'a, N, Uart>, UartMux<'b, I, MuxTxd<U>>)
where
    Alternate<'a, N, Uart>: HasUartSignal<I>,
{
    const RTS: bool = false;
    const CTS: bool = false;
    const TXD: bool = true;
    const RXD: bool = false;
    type Split<T> = (
        BlockingTransmitHalf<T, (Alternate<'a, N, Uart>, UartMux<'b, I, MuxTxd<U>>)>,
        BlockingReceiveHalf<T, ()>,
    );
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        from_pads(uart, self, ())
    }
}

impl<
        'a,
        'b,
        'c,
        'd,
        const I1: usize,
        const I2: usize,
        const U: usize,
        const N1: usize,
        const N2: usize,
    > Pads<U>
    for (
        (Alternate<'a, N1, Uart>, UartMux<'b, I1, MuxTxd<U>>),
        (Alternate<'c, N2, Uart>, UartMux<'d, I2, MuxRxd<U>>),
    )
where
    Alternate<'a, N1, Uart>: HasUartSignal<I1>,
    Alternate<'c, N2, Uart>: HasUartSignal<I2>,
{
    const RTS: bool = false;
    const CTS: bool = false;
    const TXD: bool = true;
    const RXD: bool = true;
    type Split<T> = (
        BlockingTransmitHalf<T, (Alternate<'a, N1, Uart>, UartMux<'b, I1, MuxTxd<U>>)>,
        BlockingReceiveHalf<T, (Alternate<'c, N2, Uart>, UartMux<'d, I2, MuxRxd<U>>)>,
    );
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        from_pads(uart, self.0, self.1)
    }
}

impl<
        'a,
        'b,
        'c,
        'd,
        const I1: usize,
        const I2: usize,
        const U: usize,
        const N1: usize,
        const N2: usize,
    > Pads<U>
    for (
        (Alternate<'a, N1, Uart>, UartMux<'b, I1, MuxTxd<U>>),
        (Alternate<'c, N2, Uart>, UartMux<'d, I2, MuxCts<U>>),
    )
where
    Alternate<'a, N1, Uart>: HasUartSignal<I1>,
    Alternate<'d, N2, Uart>: HasUartSignal<I2>,
{
    const RTS: bool = false;
    const CTS: bool = true;
    const TXD: bool = true;
    const RXD: bool = false;
    type Split<T> = BlockingTransmitHalf<
        T,
        (
            (Alternate<'a, N1, Uart>, UartMux<'b, I1, MuxTxd<U>>),
            (Alternate<'c, N2, Uart>, UartMux<'d, I2, MuxCts<U>>),
        ),
    >;
    #[inline]
    fn split<T>(self, uart: T) -> Self::Split<T> {
        BlockingTransmitHalf { uart, _pads: self }
    }
}

impl<
        'a,
        'b,
        'c,
        'd,
        'e,
        'f,
        'g,
        'h,
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
        (Alternate<'a, N1, Uart>, UartMux<'b, I1, MuxTxd<U>>),
        (Alternate<'c, N2, Uart>, UartMux<'d, I2, MuxRxd<U>>),
        (Alternate<'e, N3, Uart>, UartMux<'f, I3, MuxRts<U>>),
        (Alternate<'g, N4, Uart>, UartMux<'h, I4, MuxCts<U>>),
    )
where
    Alternate<'a, N1, Uart>: HasUartSignal<I1>,
    Alternate<'c, N2, Uart>: HasUartSignal<I2>,
    Alternate<'e, N3, Uart>: HasUartSignal<I3>,
    Alternate<'g, N4, Uart>: HasUartSignal<I4>,
{
    const RTS: bool = false;
    const CTS: bool = true;
    const TXD: bool = true;
    const RXD: bool = false;
    type Split<T> = (
        BlockingTransmitHalf<
            T,
            (
                (Alternate<'a, N1, Uart>, UartMux<'b, I1, MuxTxd<U>>),
                (Alternate<'g, N4, Uart>, UartMux<'h, I4, MuxCts<U>>),
            ),
        >,
        BlockingReceiveHalf<
            T,
            (
                (Alternate<'c, N2, Uart>, UartMux<'d, I2, MuxRxd<U>>),
                (Alternate<'e, N3, Uart>, UartMux<'f, I3, MuxRts<U>>),
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

impl<'a, const N: usize> Pads<MMUART_UART_ID> for Alternate<'a, N, MmUart>
where
    Alternate<'a, N, MmUart>: HasMmUartSignal,
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

impl<'a, 'b, const N1: usize, const N2: usize> Pads<MMUART_UART_ID>
    for (Alternate<'a, N1, MmUart>, Alternate<'b, N2, MmUart>)
where
    Alternate<'a, N1, MmUart>: HasMmUartSignal,
    Alternate<'b, N2, MmUart>: HasMmUartSignal,
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

impl<'a, 'b, 'c, const N1: usize, const N2: usize, const N3: usize> Pads<MMUART_UART_ID>
    for (
        Alternate<'a, N1, MmUart>,
        Alternate<'b, N2, MmUart>,
        Alternate<'c, N3, MmUart>,
    )
where
    Alternate<'a, N1, MmUart>: HasMmUartSignal,
    Alternate<'b, N2, MmUart>: HasMmUartSignal,
    Alternate<'c, N3, MmUart>: HasMmUartSignal,
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

impl<'a, 'b, 'c, 'd, const N1: usize, const N2: usize, const N3: usize, const N4: usize>
    Pads<MMUART_UART_ID>
    for (
        Alternate<'a, N1, MmUart>,
        Alternate<'b, N2, MmUart>,
        Alternate<'c, N3, MmUart>,
        Alternate<'d, N4, MmUart>,
    )
where
    Alternate<'a, N1, MmUart>: HasMmUartSignal,
    Alternate<'b, N2, MmUart>: HasMmUartSignal,
    Alternate<'c, N3, MmUart>: HasMmUartSignal,
    Alternate<'d, N4, MmUart>: HasMmUartSignal,
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
