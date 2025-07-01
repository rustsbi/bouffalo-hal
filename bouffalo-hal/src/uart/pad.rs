use super::signal::{ClearToSend, Receive, RequestToSend, Transmit};
use crate::{
    gpio::{Alternate, FlexPad, MmUart},
    uart::IntoSignals,
};

/// Pad that can be configured into UART alternate function.
///
/// `S` represents the signal identifier for this pad, 0..=11.
#[diagnostic::on_unimplemented(
    message = "this GPIO pad has no hardware connection to UART multiplexer signal {S}"
)]
pub trait IntoUartPad<'a, const S: usize> {
    /// Configure the GPIO pad into UART state.
    fn into_uart_pad(self) -> FlexPad<'a>;
}

/// Configure and convert the multiplexer into a UART signal.
pub trait IntoUartSignal<'a, const S: usize> {
    /// Converts this multiplexer into transmit state, combine with the pad to build Transmit signal.
    fn into_transmit<const I: usize>(self, pad: impl IntoUartPad<'a, S>) -> Transmit<'a, I>;
    /// Converts this multiplexer into receive state, combine with the pad to build Receive signal.
    fn into_receive<const I: usize>(self, pad: impl IntoUartPad<'a, S>) -> Receive<'a, I>;
    /// Converts this multiplexer into request-to-send state, combine with the pad to build RequestToSend signal.
    fn into_request_to_send<const I: usize>(
        self,
        pad: impl IntoUartPad<'a, S>,
    ) -> RequestToSend<'a, I>;
    /// Converts this multiplexer into clear-to-send state, combine with the pad to build ClearToSend signal.
    fn into_clear_to_send<const I: usize>(self, pad: impl IntoUartPad<'a, S>)
    -> ClearToSend<'a, I>;
}

const MMUART_UART_ID: usize = 3;

impl<'a, const N: usize> IntoSignals<'a, MMUART_UART_ID> for Alternate<'a, N, MmUart> {
    const TXD: bool = { N % 4 == 0 };
    const RXD: bool = { N % 4 == 1 };
    const RTS: bool = { N % 4 == 2 };
    const CTS: bool = { N % 4 == 3 };
}

impl<'a, const N1: usize, const N2: usize> IntoSignals<'a, MMUART_UART_ID>
    for (Alternate<'a, N1, MmUart>, Alternate<'a, N2, MmUart>)
{
    const TXD: bool = { N1 % 4 == 0 || N2 % 4 == 0 };
    const RXD: bool = { N1 % 4 == 1 || N2 % 4 == 1 };
    const RTS: bool = { N1 % 4 == 2 || N2 % 4 == 2 };
    const CTS: bool = { N1 % 4 == 3 || N2 % 4 == 3 };
}

impl<'a, const N1: usize, const N2: usize, const N3: usize> IntoSignals<'a, MMUART_UART_ID>
    for (
        Alternate<'a, N1, MmUart>,
        Alternate<'a, N2, MmUart>,
        Alternate<'a, N3, MmUart>,
    )
{
    const TXD: bool = { N1 % 4 == 0 || N2 % 4 == 0 || N3 % 4 == 0 };
    const RXD: bool = { N1 % 4 == 1 || N2 % 4 == 1 || N3 % 4 == 1 };
    const RTS: bool = { N1 % 4 == 2 || N2 % 4 == 2 || N3 % 4 == 2 };
    const CTS: bool = { N1 % 4 == 3 || N2 % 4 == 3 || N3 % 4 == 3 };
}

impl<'a, const N1: usize, const N2: usize, const N3: usize, const N4: usize>
    IntoSignals<'a, MMUART_UART_ID>
    for (
        Alternate<'a, N1, MmUart>,
        Alternate<'a, N2, MmUart>,
        Alternate<'a, N3, MmUart>,
        Alternate<'a, N4, MmUart>,
    )
{
    const TXD: bool = { N1 % 4 == 0 || N2 % 4 == 0 || N3 % 4 == 0 || N4 % 4 == 0 };
    const RXD: bool = { N1 % 4 == 1 || N2 % 4 == 1 || N3 % 4 == 1 || N4 % 4 == 1 };
    const RTS: bool = { N1 % 4 == 2 || N2 % 4 == 2 || N3 % 4 == 2 || N4 % 4 == 2 };
    const CTS: bool = { N1 % 4 == 3 || N2 % 4 == 3 || N3 % 4 == 3 || N4 % 4 == 3 };
}
