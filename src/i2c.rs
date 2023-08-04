//! Inter-Integrated Circuit bus.
#[cfg(any(doc, feature = "glb-v2"))]
use crate::{
    glb::v2::I2cClockSource,
    gpio::{self, Pin},
    GLB, I2C,
};
#[cfg(any(doc, feature = "glb-v2"))]
use base_address::BaseAddress;
use volatile_register::{RO, RW, WO};

/// Inter-integrated circuit registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Function configuration register.
    pub config: RW<Config>,
    /// Interrupt states.
    pub interrupt_state: RO<InterruptState>,
    /// Interrupt masks.
    pub interrupt_mask: RW<InterruptMask>,
    /// Interrupt clear.
    pub interrupt_clear: WO<InterruptClear>,
    /// Interrupt enable.
    pub interrupt_enable: RW<InterruptEnable>,
    /// Register address of slave device.
    pub sub_address: RW<u32>,
    /// Bus busy state indicator.
    pub bus_busy: RW<BusBusy>,
    /// Duration of start phase.
    pub period_start: RW<PeriodStart>,
    /// Duration of stop phase.
    pub period_stop: RW<PeriodStop>,
    /// Duration of data phase.
    pub period_data: RW<PeriodData>,
    _reserved: [u8; 0x64],
    /// FIFO configuration register 0.
    pub fifo_config_0: RW<FifoConfig0>,
    /// FIFO configuration register 1.
    pub fifo_config_1: RW<FifoConfig1>,
    /// FIFO write data register.
    pub data_write: WO<u32>,
    /// FIFO read data register.
    pub data_read: RO<u32>,
}

/// Function configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Config(u32);

impl Config {
    const MASTER_ENABLE: u32 = 1 << 0;
    const PACKET_DIRECTION: u32 = 1 << 1;
    const DEGLITCH_ENABLE: u32 = 1 << 2;
    const SCL_SYNC_ENABLE: u32 = 1 << 3;
    const SUB_ADDRESS_ENABLE: u32 = 1 << 4;
    const SUB_ADDRESS_BYTE_COUNT: u32 = 0x3 << 5;
    const TEN_BIT_ADDRESS_ENABLE: u32 = 1 << 7;
    const SLAVE_ADDRESS: u32 = 0x3ff << 8;
    const PACKET_LENGTH: u32 = 0xff << 20;
    const DEGLITCH_CYCLE: u32 = 0xf << 28;

    /// Enable master function.
    #[inline]
    pub fn enable_master(self) -> Self {
        Self(self.0 | Self::MASTER_ENABLE)
    }
    /// Disable master function.
    #[inline]
    pub fn disable_master(self) -> Self {
        Self(self.0 & !Self::MASTER_ENABLE)
    }
    /// Check if master function is enabled.
    #[inline]
    pub fn is_master_enabled(self) -> bool {
        self.0 & Self::MASTER_ENABLE != 0
    }
    /// Set packet direction to read.
    #[inline]
    pub fn set_read_direction(self) -> Self {
        Self(self.0 | Self::PACKET_DIRECTION)
    }
    /// Set packet direction to write.
    #[inline]
    pub fn set_write_direction(self) -> Self {
        Self(self.0 & !Self::PACKET_DIRECTION)
    }
    /// Check if packet direction is set to read.
    #[inline]
    pub fn is_read_direction(self) -> bool {
        self.0 & Self::PACKET_DIRECTION != 0
    }
    /// Check if packet direction is set to write.
    #[inline]
    pub fn is_write_direction(self) -> bool {
        self.0 & Self::PACKET_DIRECTION == 0
    }
    /// Enable de-glitch function.
    #[inline]
    pub fn enable_deglitch(self) -> Self {
        Self(self.0 | Self::DEGLITCH_ENABLE)
    }
    /// Disable de-glitch function.
    #[inline]
    pub fn disable_deglitch(self) -> Self {
        Self(self.0 & !Self::DEGLITCH_ENABLE)
    }
    /// Check if de-glitch function is enabled.
    #[inline]
    pub fn is_deglitch_enabled(self) -> bool {
        self.0 & Self::DEGLITCH_ENABLE != 0
    }
    /// Enable SCL synchronization.
    #[inline]
    pub fn enable_scl_sync(self) -> Self {
        Self(self.0 | Self::SCL_SYNC_ENABLE)
    }
    /// Disable SCL synchronization.
    #[inline]
    pub fn disable_scl_sync(self) -> Self {
        Self(self.0 & !Self::SCL_SYNC_ENABLE)
    }
    /// Check if SCL synchronization is enabled.
    #[inline]
    pub fn is_scl_sync_enabled(self) -> bool {
        self.0 & Self::SCL_SYNC_ENABLE != 0
    }
    /// Enable sub-address field.
    #[inline]
    pub fn enable_sub_address(self) -> Self {
        Self(self.0 | Self::SUB_ADDRESS_ENABLE)
    }
    /// Disable sub-address field.
    #[inline]
    pub fn disable_sub_address(self) -> Self {
        Self(self.0 & !Self::SUB_ADDRESS_ENABLE)
    }
    /// Check if sub-address field is enabled.
    #[inline]
    pub fn is_sub_address_enabled(self) -> bool {
        self.0 & Self::SUB_ADDRESS_ENABLE != 0
    }
    /// Set sub-address byte count.
    #[inline]
    pub fn set_sub_address_byte_count(self, count: SubAddressByteCount) -> Self {
        Self((self.0 & !Self::SUB_ADDRESS_BYTE_COUNT) | ((count as u32) << 5))
    }
    /// Get sub-address byte count.
    #[inline]
    pub fn get_sub_address_byte_count(self) -> SubAddressByteCount {
        match (self.0 & Self::SUB_ADDRESS_BYTE_COUNT) >> 5 {
            0 => SubAddressByteCount::One,
            1 => SubAddressByteCount::Two,
            2 => SubAddressByteCount::Three,
            3 => SubAddressByteCount::Four,
            _ => unreachable!(),
        }
    }
    /// Enable 10-bit address mode.
    #[inline]
    pub fn enable_ten_bit_address(self) -> Self {
        Self(self.0 | Self::TEN_BIT_ADDRESS_ENABLE)
    }
    /// Disable 10-bit address mode.
    #[inline]
    pub fn disable_ten_bit_address(self) -> Self {
        Self(self.0 & !Self::TEN_BIT_ADDRESS_ENABLE)
    }
    /// Check if 10-bit address mode is enabled.
    #[inline]
    pub fn is_ten_bit_address_enabled(self) -> bool {
        self.0 & Self::TEN_BIT_ADDRESS_ENABLE != 0
    }
    /// Set slave address.
    #[inline]
    pub fn set_slave_address(self, address: u32) -> Self {
        Self((self.0 & !Self::SLAVE_ADDRESS) | (address << 8))
    }
    /// Get slave address.
    #[inline]
    pub fn get_slave_address(self) -> u32 {
        (self.0 & Self::SLAVE_ADDRESS) >> 8
    }
    /// Set packet length.
    #[inline]
    pub fn set_packet_length(self, length: usize) -> Self {
        Self((self.0 & !Self::PACKET_LENGTH) | ((length as u32) << 20))
    }
    /// Get packet length.
    #[inline]
    pub fn get_packet_length(self) -> u8 {
        ((self.0 & Self::PACKET_LENGTH) >> 20) as u8
    }
    /// Set de-glitch cycle count.
    #[inline]
    pub fn set_deglitch_cycle_count(self, count: u8) -> Self {
        Self((self.0 & !Self::DEGLITCH_CYCLE) | ((count as u32) << 28))
    }
    /// Get de-glitch cycle count.
    #[inline]
    pub fn get_deglitch_cycle_count(self) -> u8 {
        ((self.0 & Self::DEGLITCH_CYCLE) >> 28) as u8
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubAddressByteCount {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}

/// Interrupt states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptState(u8);

impl InterruptState {
    /// Check if has interrupt.
    #[inline]
    pub const fn has_interrupt(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptMask(u8);

impl InterruptMask {
    /// Set interrupt mask.
    #[inline]
    pub const fn mask_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
    /// Clear interrupt mask.
    #[inline]
    pub const fn unmask_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 & !(1 << (val as u32)))
    }
    /// Check if interrupt is masked.
    #[inline]
    pub const fn is_interrupt_masked(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt clear register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptClear(u8);

impl InterruptClear {
    /// Clear interrupt.
    #[inline]
    pub const fn clear_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
}

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptEnable(u8);

impl InterruptEnable {
    /// Enable interrupt.
    #[inline]
    pub const fn enable_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
    /// Disable interrupt.
    #[inline]
    pub const fn disable_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 & !(1 << (val as u32)))
    }
    /// Check if interrupt is enabled.
    #[inline]
    pub const fn is_interrupt_enabled(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Interrupt {
    TransferEnd = 0,
    TransmitFifoReady = 1,
    ReceiveFifoReady = 2,
    NackReceived = 3,
    ArbitrationLost = 4,
    FifoError = 5,
}

/// Bus busy state indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BusBusy(u32);

impl BusBusy {
    const BUS_BUSY: u32 = 1 << 0;
    const BUS_BUSY_CLEAR: u32 = 1 << 1;

    /// Check if bus is busy.
    #[inline]
    pub const fn is_bus_busy(self) -> bool {
        self.0 & Self::BUS_BUSY != 0
    }
    /// Clear bus busy status.
    #[inline]
    pub const fn clear_bus_busy(self) -> Self {
        Self(self.0 | Self::BUS_BUSY_CLEAR)
    }
}

/// Duration of start phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PeriodStart(u32);

impl PeriodStart {
    const START_PHASE: u32 = 0xff;

    /// Set duration of start phase.
    #[inline]
    pub const fn set_phase(self, idx: usize, val: u8) -> Self {
        Self((self.0 & !(Self::START_PHASE << (idx * 8))) | ((val as u32) << (idx * 8)))
    }
    /// Get duration of start phase.
    #[inline]
    pub const fn phase(self, idx: usize) -> u8 {
        ((self.0 & (Self::START_PHASE << (idx * 8))) >> (idx * 8)) as u8
    }
}

/// Duration of stop phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PeriodStop(u32);

impl PeriodStop {
    const STOP_PHASE: u32 = 0xff;

    /// Set duration of stop phase.
    #[inline]
    pub const fn set_phase(self, idx: usize, val: u8) -> Self {
        Self((self.0 & !(Self::STOP_PHASE << (idx * 8))) | ((val as u32) << (idx * 8)))
    }
    /// Get duration of stop phase.
    #[inline]
    pub const fn phase(self, idx: usize) -> u8 {
        ((self.0 & (Self::STOP_PHASE << (idx * 8))) >> (idx * 8)) as u8
    }
}

/// Duration of data phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PeriodData(u32);

impl PeriodData {
    const DATA_PHASE: u32 = 0xff;

    /// Set duration of data phase.
    #[inline]
    pub const fn set_phase(self, idx: usize, val: u8) -> Self {
        Self((self.0 & !(Self::DATA_PHASE << (idx * 8))) | ((val as u32) << (idx * 8)))
    }
    /// Get duration of data phase.
    #[inline]
    pub const fn phase(self, idx: usize) -> u8 {
        ((self.0 & (Self::DATA_PHASE << (idx * 8))) >> (idx * 8)) as u8
    }
}

/// FIFO configuration register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoConfig0(u32);

impl FifoConfig0 {
    const DMA_TRANSMIT_ENABLE: u32 = 1 << 0;
    const DMA_RECEIVE_ENABLE: u32 = 1 << 1;
    const TRANSMIT_FIFO_CLEAR: u32 = 1 << 2;
    const RECEIVE_FIFO_CLEAR: u32 = 1 << 3;
    const TRANSMIT_FIFO_OVERFLOW: u32 = 1 << 4;
    const TRANSMIT_FIFO_UNDERFLOW: u32 = 1 << 5;
    const RECEIVE_FIFO_OVERFLOW: u32 = 1 << 6;
    const RECEIVE_FIFO_UNDERFLOW: u32 = 1 << 7;

    /// Enable DMA transmit.
    #[inline]
    pub fn enable_dma_transmit(self) -> Self {
        Self(self.0 | Self::DMA_TRANSMIT_ENABLE)
    }
    /// Disable DMA transmit.
    #[inline]
    pub fn disable_dma_transmit(self) -> Self {
        Self(self.0 & !Self::DMA_TRANSMIT_ENABLE)
    }
    /// Check if DMA transmit is enabled.
    #[inline]
    pub fn is_dma_transmit_enabled(self) -> bool {
        self.0 & Self::DMA_TRANSMIT_ENABLE != 0
    }
    /// Enable DMA receive.
    #[inline]
    pub fn enable_dma_receive(self) -> Self {
        Self(self.0 | Self::DMA_RECEIVE_ENABLE)
    }
    /// Disable DMA receive.
    #[inline]
    pub fn disable_dma_receive(self) -> Self {
        Self(self.0 & !Self::DMA_RECEIVE_ENABLE)
    }
    /// Check if DMA receive is enabled.
    #[inline]
    pub fn is_dma_receive_enabled(self) -> bool {
        self.0 & Self::DMA_RECEIVE_ENABLE != 0
    }
    /// Clear transmit FIFO.
    #[inline]
    pub fn clear_transmit_fifo(self) -> Self {
        Self(self.0 | Self::TRANSMIT_FIFO_CLEAR)
    }
    /// Clear receive FIFO.
    #[inline]
    pub fn clear_receive_fifo(self) -> Self {
        Self(self.0 | Self::RECEIVE_FIFO_CLEAR)
    }
    /// Check if transmit FIFO is overflow.
    #[inline]
    pub fn is_transmit_fifo_overflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_OVERFLOW != 0
    }
    /// Check if transmit FIFO is underflow.
    #[inline]
    pub fn is_transmit_fifo_underflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_UNDERFLOW != 0
    }
    /// Check if receive FIFO is overflow.
    #[inline]
    pub fn is_receive_fifo_overflow(self) -> bool {
        self.0 & Self::RECEIVE_FIFO_OVERFLOW != 0
    }
    /// Check if receive FIFO is underflow.
    #[inline]
    pub fn is_receive_fifo_underflow(self) -> bool {
        self.0 & Self::RECEIVE_FIFO_UNDERFLOW != 0
    }
}

/// FIFO configuration register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoConfig1(u32);

impl FifoConfig1 {
    const TRANSMIT_COUNT: u32 = 0x3;
    const RECEIVE_COUNT: u32 = 0x3 << 8;
    const TRANSMIT_THRESHOLD: u32 = 0x1 << 16;
    const RECEIVE_THRESHOLD: u32 = 0x1 << 24;

    /// Get number of empty spaces remained in transmit FIFO queue.
    #[inline]
    pub const fn transmit_available_bytes(self) -> u8 {
        (self.0 & Self::TRANSMIT_COUNT) as u8
    }
    /// Get number of available bytes received in receive FIFO queue.
    #[inline]
    pub const fn receive_available_bytes(self) -> u8 {
        ((self.0 & Self::RECEIVE_COUNT) >> 8) as u8
    }
    /// Set transmit FIFO threshold.
    #[inline]
    pub const fn set_transmit_threshold(self, val: u8) -> Self {
        Self(self.0 & !Self::TRANSMIT_THRESHOLD | ((val as u32) << 16))
    }
    /// Get transmit FIFO threshold.
    #[inline]
    pub const fn transmit_threshold(self) -> u8 {
        ((self.0 & Self::TRANSMIT_THRESHOLD) >> 16) as u8
    }
    /// Set receive FIFO threshold.
    #[inline]
    pub const fn set_receive_threshold(self, val: u8) -> Self {
        Self(self.0 & !Self::RECEIVE_THRESHOLD | ((val as u32) << 24))
    }
    /// Get receive FIFO threshold.
    #[inline]
    pub const fn receive_threshold(self) -> u8 {
        ((self.0 & Self::RECEIVE_THRESHOLD) >> 24) as u8
    }
}

#[cfg(any(doc, feature = "glb-v2"))]
pub struct I2c<A: BaseAddress, PINS> {
    i2c: I2C<A>,
    pins: PINS,
}

#[cfg(any(doc, feature = "glb-v2"))]
impl<A: BaseAddress, SCL, SDA> I2c<A, (SCL, SDA)> {
    /// Create a new Inter-Integrated Circuit instance.
    #[cfg(any(doc, feature = "glb-v1", feature = "glb-v2"))]
    #[inline]
    pub fn new<const I: usize>(i2c: I2C<A>, pins: (SCL, SDA), glb: &GLB<impl BaseAddress>) -> Self
    where
        SCL: SclPin<I>,
        SDA: SdaPin<I>,
    {
        // TODO: support custom clock and frequency
        // Enable clock
        unsafe {
            glb.i2c_config.modify(|config| {
                config
                    .enable_clock()
                    .set_clock_source(I2cClockSource::Xclk)
                    .set_clock_divide(0xff)
            });
            glb.clock_config_1.modify(|config| config.enable_i2c());
            i2c.period_start.write(
                PeriodStart(0)
                    .set_phase(0, 0xff)
                    .set_phase(1, 0xff)
                    .set_phase(2, 0xff)
                    .set_phase(3, 0xff),
            );
            i2c.period_stop.write(
                PeriodStop(0)
                    .set_phase(0, 0xff)
                    .set_phase(1, 0xff)
                    .set_phase(2, 0xff)
                    .set_phase(3, 0xff),
            );
            i2c.period_data.write(
                PeriodData(0)
                    .set_phase(0, 0xff)
                    .set_phase(1, 0xff)
                    .set_phase(2, 0xff)
                    .set_phase(3, 0xff),
            );
            i2c.config.write(
                Config(0)
                    .disable_ten_bit_address()
                    .disable_scl_sync()
                    .disable_sub_address(),
            );
        }

        Self { i2c, pins }
    }

    /// Release the I2C instance and return the pins.
    #[cfg(any(doc, feature = "glb-v1", feature = "glb-v2"))]
    #[inline]
    pub fn free(self, glb: &GLB<impl BaseAddress>) -> (I2C<A>, (SCL, SDA)) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                todo!()
            } else if #[cfg(feature = "glb-v2")] {
                unsafe {
                    glb.i2c_config.modify(|config| config.disable_clock());
                    glb.clock_config_1.modify(|config| config.disable_i2c());
                }
                (self.i2c, self.pins)
            }
        }
    }

    /// Enable sub-address.
    #[cfg(any(doc, feature = "glb-v1", feature = "glb-v2"))]
    #[inline]
    pub fn enable_sub_address(&mut self, sub_address: u8) {
        // TODO: support sub-address with more than one byte
        unsafe {
            self.i2c.config.modify(|config| {
                config
                    .enable_sub_address()
                    .set_sub_address_byte_count(SubAddressByteCount::One)
            });
            self.i2c.sub_address.write(sub_address as u32);
        }
    }

    /// Disable sub-address.
    #[cfg(any(doc, feature = "glb-v1", feature = "glb-v2"))]
    #[inline]
    pub fn disable_sub_address(&mut self) {
        unsafe {
            self.i2c
                .config
                .modify(|config| config.disable_sub_address());
        }
    }
}

pub trait SclPin<const I: usize> {}

pub trait SdaPin<const I: usize> {}

#[cfg(any(doc, feature = "glb-v2"))]
#[rustfmt::skip]
mod i2c_impls {
    use super::*;

    // 0, 2, 4, ..., 2n: SCL
    // 1, 3, 5, ..., 2n+1: SDA
    // TODO: support other pins if needed
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 0, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 1, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 2, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 3, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 4, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 5, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 6, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 7, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 8, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 9, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 10, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 11, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 12, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 13, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SclPin<I> for Pin<A, 14, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
    impl<A: BaseAddress, const I: usize> SdaPin<I> for Pin<A, 15, gpio::I2c<I>> where gpio::I2c<I>: gpio::Alternate {}
}
