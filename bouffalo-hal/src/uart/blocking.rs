use super::{
    Config, ConfigError, Error, Instance, Numbered, RegisterBlock, signal::IntoSignals, uart_config,
};
use crate::clocks::Clocks;
use core::marker::PhantomData;

/// Managed blocking serial peripheral.
pub struct BlockingSerial<'a> {
    uart: &'a RegisterBlock,
    pads: PhantomData<()>,
}

impl<'a> BlockingSerial<'a> {
    /// Creates a polling serial instance, without interrupt or DMA configurations.
    #[inline]
    pub fn new_freerun<const I: usize>(
        uart: impl Numbered<'a, I>,
        config: Config,
        pads: impl IntoSignals<'a, I>,
        clocks: &Clocks,
    ) -> Result<Self, ConfigError> {
        let uart = uart.register_block();
        // Calculate transmit interval and register values from configuration.
        let (bit_period, data_config, transmit_config, receive_config) =
            uart_config(config, &clocks, &pads)?;

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

        Ok(Self {
            uart,
            pads: PhantomData,
        })
    }

    /// Steal `BlockingSerial` instance from existing register block that is already configured.
    ///
    /// # Unsafety
    ///
    /// Caller must ensure that no I/O pad conflicts exists during the lifetime of this peripheral,
    /// and that the peripheral clocks are already properly configured.
    #[inline]
    pub unsafe fn steal_freerun(uart: impl Instance<'a>) -> Self {
        Self {
            uart: uart.register_block(),
            pads: PhantomData,
        }
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

    /// Split serial instance into transmit and receive halves.
    // TODO if no transmit signal, no receive half shall return, vice versa
    #[inline]
    pub fn split(self) -> (BlockingTransmitHalf<'a>, BlockingReceiveHalf<'a>) {
        let tx = BlockingTransmitHalf {
            uart: self.uart,
            _pads: PhantomData,
        };
        let rx = BlockingReceiveHalf {
            uart: self.uart,
            _pads: PhantomData,
        };
        (tx, rx)
    }
}

/// Transmit half from splitted serial structure.
pub struct BlockingTransmitHalf<'a> {
    pub(crate) uart: &'a RegisterBlock,
    pub(crate) _pads: PhantomData<()>,
}

/// Receive half from splitted serial structure.
pub struct BlockingReceiveHalf<'a> {
    pub(crate) uart: &'a RegisterBlock,
    pub(crate) _pads: PhantomData<()>,
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

impl<'a> embedded_io::ErrorType for BlockingSerial<'a> {
    type Error = Error;
}

impl<'a> embedded_hal_nb::serial::ErrorType for BlockingSerial<'a> {
    type Error = Error;
}

impl<'a> embedded_io::ErrorType for BlockingTransmitHalf<'a> {
    type Error = Error;
}

impl<'a> embedded_hal_nb::serial::ErrorType for BlockingTransmitHalf<'a> {
    type Error = Error;
}

impl<'a> embedded_io::ErrorType for BlockingReceiveHalf<'a> {
    type Error = Error;
}

impl<'a> embedded_hal_nb::serial::ErrorType for BlockingReceiveHalf<'a> {
    type Error = Error;
}

impl<'a> embedded_io::Write for BlockingSerial<'a> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        uart_write(&self.uart, buf)
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush(&self.uart)
    }
}

impl<'a> embedded_hal_nb::serial::Write for BlockingSerial<'a> {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        uart_write_nb(&self.uart, word)
    }
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        uart_flush_nb(&self.uart)
    }
}

impl<'a> embedded_io::Read for BlockingSerial<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read(&self.uart, buf)
    }
}

impl<'a> embedded_hal_nb::serial::Read for BlockingSerial<'a> {
    #[inline]
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        uart_read_nb(&self.uart)
    }
}

impl<'a> embedded_io::Write for BlockingTransmitHalf<'a> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        uart_write(&self.uart, buf)
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        uart_flush(&self.uart)
    }
}

impl<'a> embedded_hal_nb::serial::Write for BlockingTransmitHalf<'a> {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        uart_write_nb(&self.uart, word)
    }
    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        uart_flush_nb(&self.uart)
    }
}

impl<'a> embedded_io::Read for BlockingReceiveHalf<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        uart_read(&self.uart, buf)
    }
}

impl<'a> embedded_hal_nb::serial::Read for BlockingReceiveHalf<'a> {
    #[inline]
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        uart_read_nb(&self.uart)
    }
}
