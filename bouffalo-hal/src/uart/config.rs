use super::{BitPeriod, ClockSource, DataConfig, ReceiveConfig, TransmitConfig};
use embedded_time::rate::{Baud, Extensions};

/// Serial configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Config {
    /// Baudrate on the transmit half.
    pub transmit_baudrate: Baud,
    /// Baudrate on the receive half.
    pub receive_baudrate: Baud,
    /// Data bit order.
    pub bit_order: BitOrder,
    /// Parity settings on the transmit half.
    pub transmit_parity: Parity,
    /// Parity settings on the receive half.
    pub receive_parity: Parity,
    /// Serial stop bits.
    pub stop_bits: StopBits,
    /// Data word length on the transmit half.
    pub transmit_word_length: WordLength,
    /// Data word length on the receive half.
    pub receive_word_length: WordLength,
}

impl Config {
    /// Set baudrate for both the transmit and receive halves.
    ///
    /// This function sets the same baudrate for the transmit and receive halves.
    #[inline]
    pub const fn set_baudrate(self, baudrate: Baud) -> Self {
        Self {
            transmit_baudrate: baudrate,
            receive_baudrate: baudrate,
            ..self
        }
    }
    /// Set parity for both the transmit and receive halves.
    #[inline]
    pub const fn set_parity(self, parity: Parity) -> Self {
        Self {
            transmit_parity: parity,
            receive_parity: parity,
            ..self
        }
    }
    /// Set word length for both the transmit and receive halves.
    #[inline]
    pub const fn set_word_length(self, word_length: WordLength) -> Self {
        Self {
            transmit_word_length: word_length,
            receive_word_length: word_length,
            ..self
        }
    }
    #[inline]
    fn into_registers(self) -> (DataConfig, TransmitConfig, ReceiveConfig) {
        let data_config = DataConfig::default().set_bit_order(self.bit_order);
        let transmit_config = TransmitConfig::default()
            .set_parity(self.transmit_parity)
            .set_stop_bits(self.stop_bits)
            .set_word_length(self.transmit_word_length);
        let receive_config = ReceiveConfig::default()
            .set_parity(self.receive_parity)
            .set_word_length(self.receive_word_length);
        (data_config, transmit_config, receive_config)
    }
}

impl Default for Config {
    /// Serial configuration defaults to 115200 Bd, 8-bit word, no parity check, 1 stop bit, LSB first.
    #[inline]
    fn default() -> Self {
        Config {
            transmit_baudrate: 115_200.Bd(),
            receive_baudrate: 115_200.Bd(),
            bit_order: BitOrder::LsbFirst,
            transmit_parity: Parity::None,
            receive_parity: Parity::None,
            stop_bits: StopBits::One,
            transmit_word_length: WordLength::Eight,
            receive_word_length: WordLength::Eight,
        }
    }
}

#[inline]
pub(crate) fn uart_config<'a, const I: usize, T: super::signal::IntoSignals<'a, I>>(
    config: Config,
    clocks: impl ClockSource,
    _pads: &T,
) -> Result<(BitPeriod, DataConfig, TransmitConfig, ReceiveConfig), ConfigError> {
    let uart_clock = clocks.uart_clock::<I>();
    let transmit_interval = uart_clock.0 / config.transmit_baudrate.0;
    let receive_interval = uart_clock.0 / config.receive_baudrate.0;
    if transmit_interval > 65535 {
        return Err(ConfigError::TransmitBaudrateTooLow);
    } else if transmit_interval < 1 {
        return Err(ConfigError::TransmitBaudrateTooHigh);
    }
    if receive_interval > 65535 {
        return Err(ConfigError::ReceiveBaudrateTooLow);
    } else if receive_interval < 1 {
        return Err(ConfigError::ReceiveBaudrateTooHigh);
    }
    let bit_period = BitPeriod::default()
        .set_transmit_time_interval(transmit_interval as u16)
        .set_receive_time_interval(receive_interval as u16);
    let (data_config, mut transmit_config, mut receive_config) = config.into_registers();
    if T::TXD {
        transmit_config = transmit_config.enable_txd();
    }
    if T::CTS {
        transmit_config = transmit_config.enable_cts();
    }
    if T::RXD {
        receive_config = receive_config.enable_rxd();
    }
    Ok((bit_period, data_config, transmit_config, receive_config))
}

/// Errors on serial configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// Impossibly high baudrate for current bus clock frequency.
    TransmitBaudrateTooHigh,
    /// Impossibly low baudrate for current bus clock frequency.
    TransmitBaudrateTooLow,
    /// Impossibly high baudrate for current bus clock frequency.
    ReceiveBaudrateTooHigh,
    /// Impossibly low baudrate for current bus clock frequency.
    ReceiveBaudrateTooLow,
}

/// Order of the bits transmitted and received on the wire.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BitOrder {
    /// Each byte is sent out LSB-first.
    LsbFirst,
    /// Each byte is sent out MSB-first.
    MsbFirst,
}

/// Parity check.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Parity {
    /// No parity check.
    None,
    /// Even parity bit.
    Even,
    /// Odd parity bit.
    Odd,
}

/// Stop bits.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StopBits {
    /// 0.5 stop bits.
    ZeroPointFive,
    /// 1 stop bit.
    One,
    /// 1.5 stop bits.
    OnePointFive,
    /// 2 stop bits.
    Two,
}

/// Word length.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WordLength {
    /// Five bits per word.
    Five,
    /// Six bits per word.
    Six,
    /// Seven bits per word.
    Seven,
    /// Eight bits per word.
    Eight,
}
