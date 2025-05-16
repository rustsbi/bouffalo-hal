//! Serial Peripheral Interface peripheral.

mod register;
pub use register::*;

use crate::glb::{self, v2::SpiMode};
use crate::gpio::{self, Alternate};
use core::cmp::max;
use core::ops::Deref;
use embedded_hal::spi::Mode;

/// Managed Serial Peripheral Interface peripheral.
pub struct Spi<SPI, PADS, const I: usize> {
    spi: SPI,
    pads: PADS,
}

impl<SPI: Deref<Target = RegisterBlock>, PADS, const I: usize> Spi<SPI, PADS, I> {
    /// Create a new Serial Peripheral Interface instance.
    #[inline]
    pub fn new<GLB>(spi: SPI, pads: PADS, mode: Mode, glb: &GLB) -> Self
    where
        PADS: Pads<I>,
        GLB: Deref<Target = glb::v2::RegisterBlock>,
    {
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
        Spi { spi, pads }
    }

    /// Release the SPI instance and return the pads.
    #[inline]
    pub fn free(self) -> (SPI, PADS) {
        (self.spi, self.pads)
    }
}

/// SPI error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Other,
}

impl embedded_hal::spi::Error for Error {
    #[inline(always)]
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        use embedded_hal::spi::ErrorKind;
        match self {
            Error::Other => ErrorKind::Other,
        }
    }
}

impl<SPI: Deref<Target = RegisterBlock>, PADS, const I: usize> embedded_hal::spi::ErrorType
    for Spi<SPI, PADS, I>
{
    type Error = Error;
}

impl<SPI: Deref<Target = RegisterBlock>, PADS, const I: usize> embedded_hal::spi::SpiBus
    for Spi<SPI, PADS, I>
{
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

impl<SPI: Deref<Target = RegisterBlock>, PADS, const I: usize> embedded_hal::spi::SpiDevice
    for Spi<SPI, PADS, I>
{
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
impl<SPI: Deref<Target = RegisterBlock>, PADS, const I: usize>
    embedded_hal_027::blocking::spi::Write<u8> for Spi<SPI, PADS, I>
{
    type Error = Error;
    #[inline]
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        <Self as embedded_hal::spi::SpiBus>::write(self, words)?;
        Ok(())
    }
}

impl<SPI: Deref<Target = RegisterBlock>, PINS, const I: usize>
    embedded_hal_027::blocking::spi::Transfer<u8> for Spi<SPI, PINS, I>
{
    type Error = Error;
    #[inline]
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        <Self as embedded_hal::spi::SpiBus>::transfer_in_place(self, words)?;
        Ok(words)
    }
}

/// Valid SPI pads.
pub trait Pads<const I: usize> {}

impl<'a, 'b, 'c, const N1: usize, const N2: usize, const N3: usize> Pads<1>
    for (
        Alternate<'a, N1, gpio::Spi<1>>,
        Alternate<'b, N2, gpio::Spi<1>>,
        Alternate<'c, N3, gpio::Spi<1>>,
    )
where
    Alternate<'a, N1, gpio::Spi<1>>: HasClkSignal,
    Alternate<'b, N2, gpio::Spi<1>>: HasMosiSignal,
    Alternate<'c, N3, gpio::Spi<1>>: HasCsSignal,
{
}

impl<'a, 'b, 'c, 'd, const N1: usize, const N2: usize, const N3: usize, const N4: usize> Pads<1>
    for (
        Alternate<'a, N1, gpio::Spi<1>>,
        Alternate<'b, N2, gpio::Spi<1>>,
        Alternate<'c, N3, gpio::Spi<1>>,
        Alternate<'d, N4, gpio::Spi<1>>,
    )
where
    Alternate<'a, N1, gpio::Spi<1>>: HasClkSignal,
    Alternate<'b, N2, gpio::Spi<1>>: HasMosiSignal,
    Alternate<'c, N3, gpio::Spi<1>>: HasMisoSignal,
    Alternate<'d, N4, gpio::Spi<1>>: HasCsSignal,
{
}

/// Check if target gpio `Pin` is internally connected to SPI clock signal.
pub trait HasClkSignal {}

impl<'a> HasClkSignal for Alternate<'a, 3, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 7, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 11, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 15, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 19, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 23, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 27, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 31, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 35, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 39, gpio::Spi<1>> {}
impl<'a> HasClkSignal for Alternate<'a, 43, gpio::Spi<1>> {}

/// Check if target gpio `Pin` is internally connected to SPI MISO signal.
pub trait HasMisoSignal {}

impl<'a> HasMisoSignal for Alternate<'a, 2, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 6, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 10, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 14, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 18, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 22, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 26, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 30, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 34, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 38, gpio::Spi<1>> {}
impl<'a> HasMisoSignal for Alternate<'a, 42, gpio::Spi<1>> {}

/// Check if target gpio `Pin` is internally connected to SPI MOSI signal.
pub trait HasMosiSignal {}

impl<'a> HasMosiSignal for Alternate<'a, 1, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 5, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 9, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 13, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 17, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 21, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 25, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 29, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 33, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 37, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 41, gpio::Spi<1>> {}
impl<'a> HasMosiSignal for Alternate<'a, 45, gpio::Spi<1>> {}

/// Check if target gpio `Pin` is internally connected to SPI CS signal.
pub trait HasCsSignal {}

impl<'a> HasCsSignal for Alternate<'a, 0, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 4, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 8, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 12, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 16, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 20, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 24, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 28, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 32, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 36, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 40, gpio::Spi<1>> {}
impl<'a> HasCsSignal for Alternate<'a, 44, gpio::Spi<1>> {}
