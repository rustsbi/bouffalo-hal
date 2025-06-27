use super::{Error, Numbered, register::*};
use crate::{
    glb::{self, v2::SpiMode},
    gpio::FlexPad,
    spi::pads::{IntoPads, IntoTransmitOnly},
};
use core::{cmp::max, marker::PhantomData};
use embedded_hal::spi::Mode;

/// Managed Serial Peripheral Interface peripheral.
pub struct Spi<'a> {
    spi: &'a RegisterBlock,
    _pads: PhantomData<FlexPad<'a>>,
}

impl<'a> Spi<'a> {
    /// Create a new Serial Peripheral Interface instance with full duplex function.
    #[inline]
    pub fn new<const I: usize>(
        spi: impl Numbered<'a, I>,
        pads: impl IntoPads<'a, I>,
        mode: Mode,
        glb: &glb::v2::RegisterBlock,
    ) -> Self {
        let config = Config::default()
            .disable_deglitch()
            .disable_slave_three_pin()
            .enable_master_continuous()
            .disable_byte_inverse()
            .disable_bit_inverse()
            .set_frame_size(FrameSize::Eight)
            .disable_master()
            .set_clock_phase(mode.phase)
            .set_clock_polarity(mode.polarity);

        let spi = spi.register_block();
        unsafe {
            glb.param_config
                .modify(|c| c.set_spi_mode::<I>(SpiMode::Master));

            spi.config.write(config);
            spi.fifo_config_0.write(
                FifoConfig0::default()
                    .disable_dma_receive()
                    .disable_dma_transmit(),
            );
            spi.fifo_config_1.write(
                FifoConfig1::default()
                    .set_receive_threshold(0)
                    .set_transmit_threshold(0),
            );
            spi.period_signal.write(
                PeriodSignal::default()
                    .set_data_phase_0(4)
                    .set_data_phase_1(4)
                    .set_start_condition(4)
                    .set_stop_condition(4),
            );
            spi.period_interval
                .write(PeriodInterval::default().set_frame_interval(4));
        }

        let pads = pads.into_full_duplex_pads();
        core::mem::forget(pads);
        Spi {
            spi,
            _pads: PhantomData,
        }
    }

    /// Create a new Serial Peripheral Interface instance with only transmit pads.
    // TODO simplify device register initialization, reuse with `new`
    #[inline]
    pub fn transmit_only<const I: usize>(
        spi: impl Numbered<'a, I>,
        pads: impl IntoTransmitOnly<'a, I>,
        mode: Mode,
        glb: &glb::v2::RegisterBlock,
    ) -> Self {
        let config = Config::default()
            .disable_deglitch()
            .disable_slave_three_pin()
            .enable_master_continuous()
            .disable_byte_inverse()
            .disable_bit_inverse()
            .set_frame_size(FrameSize::Eight)
            .disable_master()
            .set_clock_phase(mode.phase)
            .set_clock_polarity(mode.polarity);

        let spi = spi.register_block();
        unsafe {
            glb.param_config
                .modify(|c| c.set_spi_mode::<I>(SpiMode::Master));

            spi.config.write(config);
            spi.fifo_config_0.write(
                FifoConfig0::default()
                    .disable_dma_receive()
                    .disable_dma_transmit(),
            );
            spi.fifo_config_1.write(
                FifoConfig1::default()
                    .set_receive_threshold(0)
                    .set_transmit_threshold(0),
            );
            spi.period_signal.write(
                PeriodSignal::default()
                    .set_data_phase_0(4)
                    .set_data_phase_1(4)
                    .set_start_condition(4)
                    .set_stop_condition(4),
            );
            spi.period_interval
                .write(PeriodInterval::default().set_frame_interval(4));
        }

        let pads = pads.into_transmit_only_pads();
        core::mem::forget(pads);
        Spi {
            spi,
            _pads: PhantomData,
        }
    }
}

impl<'a> embedded_hal::spi::ErrorType for Spi<'a> {
    type Error = Error;
}

impl<'a> embedded_hal::spi::SpiBus for Spi<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        unsafe { self.spi.config.modify(|config| config.enable_master()) };

        buf.iter_mut().for_each(|slot| {
            while self.spi.fifo_config_1.read().receive_available_bytes() == 0 {
                core::hint::spin_loop();
            }
            *slot = self.spi.fifo_read.read()
        });

        unsafe { self.spi.config.modify(|config| config.disable_master()) };
        Ok(())
    }

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        unsafe { self.spi.config.modify(|config| config.enable_master()) };

        buf.iter().for_each(|&word| {
            while self.spi.fifo_config_1.read().transmit_available_bytes() == 0 {
                core::hint::spin_loop();
            }
            unsafe { self.spi.fifo_write.write(word) }
            _ = self.spi.fifo_read.read();
        });

        unsafe { self.spi.config.modify(|config| config.disable_master()) };
        Ok(())
    }

    #[inline]
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        const MAX_RETRY: usize = 1000;
        unsafe { self.spi.config.modify(|config| config.enable_master()) };

        let (mut tx, mut rx) = (0, 0);
        let mut fifo_config = self.spi.fifo_config_1.read();
        let mut retry = 0;
        while tx < write.len() || rx < read.len() {
            while fifo_config.receive_available_bytes() == 0
                && fifo_config.transmit_available_bytes() == 0
            {
                fifo_config = self.spi.fifo_config_1.read();
            }
            if fifo_config.transmit_available_bytes() != 0 && tx < write.len() {
                unsafe { self.spi.fifo_write.write(write[tx]) }
                tx += 1;
            }
            if fifo_config.receive_available_bytes() != 0 && rx < read.len() {
                read[rx] = self.spi.fifo_read.read();
                rx += 1;
            }
            retry += 1;
            if retry > MAX_RETRY * max(write.len(), read.len()) {
                return Err(Error::Other);
            }
        }

        unsafe { self.spi.config.modify(|config| config.disable_master()) };
        Ok(())
    }

    #[inline]
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        const MAX_RETRY: usize = 1000;
        unsafe { self.spi.config.modify(|config| config.enable_master()) };

        let (mut tx, mut rx) = (0, 0);
        let mut fifo_config = self.spi.fifo_config_1.read();
        let mut retry = 0;
        while tx < words.len() || rx < words.len() {
            while fifo_config.receive_available_bytes() == 0
                && fifo_config.transmit_available_bytes() == 0
            {
                fifo_config = self.spi.fifo_config_1.read();
            }
            if fifo_config.transmit_available_bytes() != 0 && tx < words.len() {
                unsafe { self.spi.fifo_write.write(words[tx]) }
                tx += 1;
            }
            if fifo_config.receive_available_bytes() != 0 && rx < tx {
                words[rx] = self.spi.fifo_read.read();
                rx += 1;
            }
            retry += 1;
            if retry > MAX_RETRY * words.len() {
                return Err(Error::Other);
            }
        }

        unsafe { self.spi.config.modify(|config| config.disable_master()) };
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        while self.spi.fifo_config_1.read().transmit_available_bytes() != 32 {
            core::hint::spin_loop();
        }
        while self.spi.fifo_config_1.read().receive_available_bytes() != 32 {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<'a> embedded_hal::spi::SpiDevice for Spi<'a> {
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                embedded_hal::spi::Operation::Read(buf) => {
                    embedded_hal::spi::SpiBus::read(self, buf)?
                }
                embedded_hal::spi::Operation::Write(buf) => {
                    embedded_hal::spi::SpiBus::write(self, buf)?
                }
                embedded_hal::spi::Operation::Transfer(read, write) => {
                    embedded_hal::spi::SpiBus::transfer(self, read, write)?
                }
                embedded_hal::spi::Operation::TransferInPlace(buf) => {
                    embedded_hal::spi::SpiBus::transfer_in_place(self, buf)?
                }
                embedded_hal::spi::Operation::DelayNs(_delay) => {
                    for _ in 0..*_delay {
                        // TODO: more accurate delay
                        core::hint::spin_loop();
                    }
                }
            }
        }
        Ok(())
    }
}

// This part of implementation using `embedded_hal_027` is designed for backward compatibility of
// ecosystem crates, as some of them depends on embedded-hal v0.2.7 traits.
// We encourage ecosystem developers to use embedded-hal v1.0.0 traits; after that, this part of code
// would be removed in the future.
impl<'a> embedded_hal_027::blocking::spi::Write<u8> for Spi<'a> {
    type Error = Error;
    #[inline]
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        <Self as embedded_hal::spi::SpiBus>::write(self, words)?;
        Ok(())
    }
}

impl<'a> embedded_hal_027::blocking::spi::Transfer<u8> for Spi<'a> {
    type Error = Error;
    #[inline]
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        <Self as embedded_hal::spi::SpiBus>::transfer_in_place(self, words)?;
        Ok(words)
    }
}
