use super::{Config, ConfigError, Error, Pads, RegisterBlock, uart_config};
use crate::clocks::Clocks;

/// Managed blocking serial peripheral.
pub struct BlockingSerial<'a, PADS> {
    uart: &'a RegisterBlock,
    pads: PADS,
}

impl<'a, PADS> BlockingSerial<'a, PADS> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    #[doc(hidden)]
    #[inline]
    pub fn __new_freerun<const I: usize>(
        uart: &'a RegisterBlock,
        config: Config,
        pads: PADS,
        clocks: &Clocks,
    ) -> Result<Self, ConfigError>
    where
        PADS: Pads<I>,
    {
        // Calculate transmit interval and register values from configuration.
        let (bit_period, data_config, transmit_config, receive_config) =
            uart_config::<I, PADS>(config, &clocks)?;

        // Write bit period.
        unsafe { uart.bit_period.write(bit_period) };
        // Write the bit-order.
        unsafe { uart.data_config.write(data_config) };

        // Configure freerun transmit feature.
        let mut val = transmit_config;
        val = val.enable_freerun();
        unsafe { uart.transmit_config.write(val) };
        // Configure receive feature.
        unsafe { uart.receive_config.write(receive_config) };

        Ok(Self { uart, pads })
    }

    /// Enable transmit DMA.
    #[inline]
    pub fn enable_tx_dma(self) -> Self {
        unsafe {
            self.uart
                .fifo_config_1
                .modify(|val| val.set_transmit_threshold(7));
            self.uart
                .fifo_config_0
                .modify(|val| val.enable_transmit_dma().clear_transmit_fifo());
        }
        self
    }

    /// Enable receive DMA.
    #[inline]
    pub fn enable_rx_dma(self) -> Self {
        unsafe {
            self.uart
                .fifo_config_1
                .modify(|val| val.set_receive_threshold(7));
            self.uart
                .fifo_config_0
                .modify(|val| val.enable_receive_dma().clear_receive_fifo());
        }
        self
    }

    /// Release serial instance and return its peripheral and pads.
    #[inline]
    pub fn free(self) -> PADS {
        self.pads
    }

    /// Split serial instance into transmit and receive halves.
    #[inline]
    pub fn split<const I: usize>(self) -> <PADS as Pads<I>>::Split<'a>
    where
        PADS: Pads<I>,
    {
        self.pads.split(self.uart)
    }
}

/// Transmit half from splitted serial structure.
pub struct BlockingTransmitHalf<'a, PADS> {
    pub(crate) uart: &'a RegisterBlock,
    pub(crate) _pads: PADS,
}

/// Receive half from splitted serial structure.
pub struct BlockingReceiveHalf<'a, PADS> {
    pub(crate) uart: &'a RegisterBlock,
    pub(crate) _pads: PADS,
}

#[inline]
fn uart_write(uart: &RegisterBlock, buf: &[u8]) -> Result<usize, Error> {
    while uart.fifo_config_1.read().transmit_available_bytes() == 0 {
        core::hint::spin_loop();
    }
    let len = core::cmp::min(
        uart.fifo_config_1.read().transmit_available_bytes() as usize,
        buf.len(),
    );
    buf.iter()
        .take(len)
        .for_each(|&word| unsafe { uart.fifo_write.write(word) });
    Ok(len)
}

#[inline]
fn uart_write_nb(uart: &RegisterBlock, word: u8) -> nb::Result<(), Error> {
    if uart.fifo_config_1.read().transmit_available_bytes() == 0 {
        return Err(nb::Error::WouldBlock);
    }
    unsafe { uart.fifo_write.write(word) };
    Ok(())
}

#[inline]
fn uart_flush(uart: &RegisterBlock) -> Result<(), Error> {
    // There are maximum 32 bytes in transmit FIFO queue, wait until all bytes are available,
    // meaning that all data in queue has been sent into UART bus.
    while uart.fifo_config_1.read().transmit_available_bytes() != 32 {
        core::hint::spin_loop();
    }
    Ok(())
}

#[inline]
fn uart_flush_nb(uart: &RegisterBlock) -> nb::Result<(), Error> {
    if uart.fifo_config_1.read().transmit_available_bytes() != 32 {
        return Err(nb::Error::WouldBlock);
    }
    Ok(())
}

#[inline]
fn uart_read(uart: &RegisterBlock, buf: &mut [u8]) -> Result<usize, Error> {
    while uart.fifo_config_1.read().receive_available_bytes() == 0 {
        core::hint::spin_loop();
    }
    let len = core::cmp::min(
        uart.fifo_config_1.read().receive_available_bytes() as usize,
        buf.len(),
    );
    buf.iter_mut()
        .take(len)
        .for_each(|slot| *slot = uart.fifo_read.read());
    Ok(len)
}

#[inline]
fn uart_read_nb(uart: &RegisterBlock) -> nb::Result<u8, Error> {
    if uart.fifo_config_1.read().receive_available_bytes() == 0 {
        return Err(nb::Error::WouldBlock);
    }
    let ans = uart.fifo_read.read();
    Ok(ans)
}

impl<'a, PADS> embedded_io::ErrorType for BlockingSerial<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_hal_nb::serial::ErrorType for BlockingSerial<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_io::ErrorType for BlockingTransmitHalf<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_hal_nb::serial::ErrorType for BlockingTransmitHalf<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_io::ErrorType for BlockingReceiveHalf<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_hal_nb::serial::ErrorType for BlockingReceiveHalf<'a, PADS> {
    type Error = Error;
}

impl<'a, PADS> embedded_io::Write for BlockingSerial<'a, PADS> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        uart_write(&self.uart, buf)
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush(&self.uart)
    }
}

impl<'a, PADS> embedded_hal_nb::serial::Write for BlockingSerial<'a, PADS> {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        uart_write_nb(&self.uart, word)
    }
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        uart_flush_nb(&self.uart)
    }
}

impl<'a, PADS> embedded_io::Read for BlockingSerial<'a, PADS> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read(&self.uart, buf)
    }
}

impl<'a, PADS> embedded_hal_nb::serial::Read for BlockingSerial<'a, PADS> {
    #[inline]
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        uart_read_nb(&self.uart)
    }
}

impl<'a, PADS> embedded_io::Write for BlockingTransmitHalf<'a, PADS> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        uart_write(&self.uart, buf)
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush(&self.uart)
    }
}

impl<'a, PADS> embedded_hal_nb::serial::Write for BlockingTransmitHalf<'a, PADS> {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        uart_write_nb(&self.uart, word)
    }
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        uart_flush_nb(&self.uart)
    }
}

impl<'a, PADS> embedded_io::Read for BlockingReceiveHalf<'a, PADS> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read(&self.uart, buf)
    }
}

impl<'a, PADS> embedded_hal_nb::serial::Read for BlockingReceiveHalf<'a, PADS> {
    #[inline]
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        uart_read_nb(&self.uart)
    }
}
