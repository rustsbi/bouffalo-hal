//! Serial peripheral bus peripheral.

use volatile_register::{RO, RW, WO};

/// Serial peripheral bus registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Peripheral configuration register.
    pub config: RW<Config>,
    /// Interrupt configuration and state register.
    pub interrupt_config: RW<InterruptConfig>,
    /// Bus busy state indication register.
    pub bus_busy: RO<BusBusy>,
    _reserved0: [u8; 0x1],
    /// Duration of data phases and conditions in source clock.
    pub period_signal: RW<PeriodSignal>,
    /// Duration of interval between frame in source clock.
    pub period_interval: RW<PeriodInterval>,
    /// Receive ignore feature configuration register.
    pub receive_ignore: RW<ReceiveIgnore>,
    /// Slave mode time-out interrupt trigger configuration.
    pub slave_timeout: RW<SlaveTimeout>,
    _reserved1: [u8; 0x60],
    /// First-in first-out queue configuration register 0.
    pub fifo_config_0: RW<FifoConfig0>,
    /// First-in first-out queue configuration register 1.
    pub fifo_config_1: RW<FifoConfig1>,
    /// First-in first-out queue write data register.
    pub data_write: WO<u32>,
    /// First-in first-out queue read data register.
    pub data_read: RO<u32>,
}

/// Peripheral configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Config(u32);

impl Config {
    const MASTER_ENABLE: u32 = 1 << 0;
    const SLAVE_ENABLE: u32 = 1 << 1;
    const FRAME_SIZE: u32 = 0x3 << 2;
    const CLOCK_POLARITY: u32 = 1 << 4;
    const CLOCK_PHASE: u32 = 1 << 5;
    const BIT_INVERSE: u32 = 1 << 6;
    const BYTE_INVERSE: u32 = 1 << 7;
    const RECEIVE_IGNORE: u32 = 1 << 8;
    const MASTER_CONTINUOUS: u32 = 1 << 9;
    const SLAVE_THREE_PIN: u32 = 1 << 10;
    const DEGLITCH_ENABLE: u32 = 1 << 11;
    const DEGLITCH_COUNT: u32 = 0xf << 12;

    /// Enable master mode.
    #[inline]
    pub const fn enable_master(self) -> Self {
        Self(self.0 | Self::MASTER_ENABLE)
    }
    /// Disable master mode.
    #[inline]
    pub const fn disable_master(self) -> Self {
        Self(self.0 & !Self::MASTER_ENABLE)
    }
    /// Check if master mode is enabled.
    #[inline]
    pub const fn is_master_enabled(self) -> bool {
        self.0 & Self::MASTER_ENABLE != 0
    }
    /// Enable slave mode.
    #[inline]
    pub const fn enable_slave(self) -> Self {
        Self(self.0 | Self::SLAVE_ENABLE)
    }
    /// Disable slave mode.
    #[inline]
    pub const fn disable_slave(self) -> Self {
        Self(self.0 & !Self::SLAVE_ENABLE)
    }
    /// Check if slave mode is enabled.
    #[inline]
    pub const fn is_slave_enabled(self) -> bool {
        self.0 & Self::SLAVE_ENABLE != 0
    }
    /// Set data frame size.
    #[inline]
    pub const fn set_frame_size(self, val: FrameSize) -> Self {
        let val = match val {
            FrameSize::Eight => 0,
            FrameSize::Sixteen => 1,
            FrameSize::TwentyFour => 2,
            FrameSize::ThirtyTwo => 3,
        };
        Self(self.0 & !Self::FRAME_SIZE | val << 2)
    }
    /// Get data frame size.
    #[inline]
    pub const fn frame_size(self) -> FrameSize {
        let val = (self.0 & Self::FRAME_SIZE) >> 2;
        match val {
            0 => FrameSize::Eight,
            1 => FrameSize::Sixteen,
            2 => FrameSize::TwentyFour,
            3 => FrameSize::ThirtyTwo,
            _ => unreachable!(),
        }
    }
    /// Set clock polarity.
    #[inline]
    pub const fn set_clock_polarity(self, val: Polarity) -> Self {
        match val {
            Polarity::IdleLow => Self(self.0 & !Self::CLOCK_POLARITY),
            Polarity::IdleHigh => Self(self.0 | Self::CLOCK_POLARITY),
        }
    }
    /// Get clock polarity.
    #[inline]
    pub const fn clock_polarity(self) -> Polarity {
        if self.0 & Self::CLOCK_POLARITY != 0 {
            Polarity::IdleHigh
        } else {
            Polarity::IdleLow
        }
    }
    /// Set clock phase.
    #[inline]
    pub const fn set_clock_phase(self, val: Phase) -> Self {
        match val {
            Phase::CaptureOnSecondTransition => Self(self.0 & !Self::CLOCK_PHASE),
            Phase::CaptureOnFirstTransition => Self(self.0 | Self::CLOCK_PHASE),
        }
    }
    /// Get clock phase.
    #[inline]
    pub const fn clock_phase(self) -> Phase {
        if self.0 & Self::CLOCK_PHASE != 0 {
            Phase::CaptureOnFirstTransition
        } else {
            Phase::CaptureOnSecondTransition
        }
    }
    /// Enable bit inverse.
    #[inline]
    pub const fn enable_bit_inverse(self) -> Self {
        Self(self.0 | Self::BIT_INVERSE)
    }
    /// Disable bit inverse.
    #[inline]
    pub const fn disable_bit_inverse(self) -> Self {
        Self(self.0 & !Self::BIT_INVERSE)
    }
    /// Check if bit inverse is enabled.
    #[inline]
    pub const fn is_bit_inverse_enabled(self) -> bool {
        self.0 & Self::BIT_INVERSE != 0
    }
    /// Enable byte inverse.
    #[inline]
    pub const fn enable_byte_inverse(self) -> Self {
        Self(self.0 | Self::BYTE_INVERSE)
    }
    /// Disable byte inverse.
    #[inline]
    pub const fn disable_byte_inverse(self) -> Self {
        Self(self.0 & !Self::BYTE_INVERSE)
    }
    /// Check if byte inverse is enabled.
    #[inline]
    pub const fn is_byte_inverse_enabled(self) -> bool {
        self.0 & Self::BYTE_INVERSE != 0
    }
    /// Enable receive ignore feature.
    #[inline]
    pub const fn enable_receive_ignore(self) -> Self {
        Self(self.0 | Self::RECEIVE_IGNORE)
    }
    /// Disable receive ignore feature.
    #[inline]
    pub const fn disable_receive_ignore(self) -> Self {
        Self(self.0 & !Self::RECEIVE_IGNORE)
    }
    /// Check if receive ignore feature is enabled.
    #[inline]
    pub const fn is_receive_ignore_enabled(self) -> bool {
        self.0 & Self::RECEIVE_IGNORE != 0
    }
    /// Enable master continuous mode.
    #[inline]
    pub const fn enable_master_continuous(self) -> Self {
        Self(self.0 | Self::MASTER_CONTINUOUS)
    }
    /// Disable master continuous mode.
    #[inline]
    pub const fn disable_master_continuous(self) -> Self {
        Self(self.0 & !Self::MASTER_CONTINUOUS)
    }
    /// Check if master continuous mode is enabled.
    #[inline]
    pub const fn is_master_continuous_enabled(self) -> bool {
        self.0 & Self::MASTER_CONTINUOUS != 0
    }
    /// Enable slave three-pin mode.
    #[inline]
    pub const fn enable_slave_three_pin(self) -> Self {
        Self(self.0 | Self::SLAVE_THREE_PIN)
    }
    /// Disable slave three-pin mode.
    #[inline]
    pub const fn disable_slave_three_pin(self) -> Self {
        Self(self.0 & !Self::SLAVE_THREE_PIN)
    }
    /// Check if slave three-pin mode is enabled.
    #[inline]
    pub const fn is_slave_three_pin_enabled(self) -> bool {
        self.0 & Self::SLAVE_THREE_PIN != 0
    }
    /// Enable deglitch.
    #[inline]
    pub const fn enable_deglitch(self) -> Self {
        Self(self.0 | Self::DEGLITCH_ENABLE)
    }
    /// Disable deglitch.
    #[inline]
    pub const fn disable_deglitch(self) -> Self {
        Self(self.0 & !Self::DEGLITCH_ENABLE)
    }
    /// Check if deglitch is enabled.
    #[inline]
    pub const fn is_deglitch_enabled(self) -> bool {
        self.0 & Self::DEGLITCH_ENABLE != 0
    }
    /// Set deglitch cycle count.
    #[inline]
    pub const fn set_deglitch_cycle(self, val: u8) -> Self {
        Self((self.0 & !Self::DEGLITCH_COUNT) | ((val as u32) << 12))
    }

    /// Get deglitch cycle count.
    #[inline]
    pub const fn deglitch_cycle(self) -> u8 {
        ((self.0 & Self::DEGLITCH_COUNT) >> 12) as u8
    }
}

/// Data frame size in bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FrameSize {
    /// 1 byte (8 bits) per frame.
    Eight,
    /// 2 bytes (16 bits) per frame.
    Sixteen,
    /// 3 bytes (24 bits) per frame.
    TwentyFour,
    /// 4 bytes (32 bits) per frame.
    ThirtyTwo,
}

/// Clock polarity settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Polarity {
    /// Clock signal low when idle.
    IdleLow,
    /// Clock signal high when idle.
    IdleHigh,
}

/// Clock phase settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    /// Data in "captured" on the second clock transition.
    CaptureOnSecondTransition,
    /// Data in "captured" on the first clock transition.
    CaptureOnFirstTransition,
}

/// Interrupt configuration and state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptConfig(u32);

impl InterruptConfig {
    /// Check if interrupt flag is set.
    #[inline]
    pub const fn has_interrupt(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
    /// Set interrupt mask.
    #[inline]
    pub const fn mask_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32 + 8)))
    }
    /// Clear interrupt mask.
    #[inline]
    pub const fn unmask_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 & !(1 << (val as u32 + 8)))
    }
    /// Check if interrupt is masked.
    #[inline]
    pub const fn is_interrupted(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32 + 8))) != 0
    }
    /// Clear interrupt flag.
    ///
    /// Note that `TransmitFifoReady`, `ReceiveFifoReady` and `FifoError` interrupts
    /// are auto-cleared when certain queue flags in other registers are cleared.
    /// Those three interrupts cannot be cleared by this function.
    #[inline]
    pub const fn clear_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32 + 16)))
    }
    /// Enable interrupt.
    #[inline]
    pub const fn enable_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32 + 24)))
    }
    /// Disable interrupt.
    #[inline]
    pub const fn disable_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 & !(1 << (val as u32 + 24)))
    }
    /// Check if interrupt is enabled.
    #[inline]
    pub const fn is_interrupt_enabled(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32 + 24))) != 0
    }
}

/// Interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Interrupt {
    /// Transfer end interrupt.
    ///
    /// On master mode, this is triggered when the final frame is transferred.
    /// On slave mode, triggered when Chip Select (CS) signal is deasserted.
    TransferEnd = 0,
    /// Transmit first-in first-out queue ready interrupt.
    ///
    /// This interrupt flag is auto cleared when data is popped.
    TransmitFifoReady = 1,
    /// Receive first-in first-out queue ready interrupt.
    ///
    /// This interrupt flag is auto cleared when data is pushed.
    ReceiveFifoReady = 2,
    /// Slave mode transfer timeout interrupt.
    ///
    /// Triggered when bus is idle for given time, see `SlaveTimeout` structure.
    SlaveTimeout = 3,
    /// Slave mode transfer underrun error.
    ///
    /// Triggered when transmit queue is not ready during transfer in slave mode.
    SlaveUnderrun = 4,
    /// Transmit or receive first-in first-out queue error interrupt.
    ///
    /// Auto cleared when queue overflow or underflow error flag is cleared.
    FifoError = 5,
}

/// Bus busy state indication register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BusBusy(u32);

impl BusBusy {
    const BUS_BUSY: u32 = 1 << 0;

    /// Check if the bus is busy.
    #[inline]
    pub const fn is_bus_busy(&self) -> bool {
        self.0 & Self::BUS_BUSY != 0
    }
}

/// Duration of data phases and conditions in source clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PeriodSignal(u32);

impl PeriodSignal {
    const START_CONDITION: u32 = 0xFF;
    const STOP_CONDITION: u32 = 0xFF << 8;
    const DATA_PHASE_0: u32 = 0xFF << 16;
    const DATA_PHASE_1: u32 = 0xFF << 24;

    /// Set start condition clock length.
    #[inline]
    pub const fn set_start_condition(self, val: u8) -> Self {
        Self((self.0 & !Self::START_CONDITION) | ((val as u32) << 0))
    }
    /// Get start condition clock length.
    #[inline]
    pub const fn start_condition(self) -> u8 {
        ((self.0 & Self::START_CONDITION) >> 0) as u8
    }
    /// Set stop condition clock length.
    #[inline]
    pub const fn set_stop_condition(self, val: u8) -> Self {
        Self((self.0 & !Self::STOP_CONDITION) | ((val as u32) << 8))
    }
    /// Get stop condition clock length.
    #[inline]
    pub const fn stop_condition(self) -> u8 {
        ((self.0 & Self::STOP_CONDITION) >> 8) as u8
    }
    /// Set data phase 0 clock length.
    #[inline]
    pub const fn set_data_phase_0(self, val: u8) -> Self {
        Self((self.0 & !Self::DATA_PHASE_0) | ((val as u32) << 16))
    }
    /// Get data phase 0 clock length.
    #[inline]
    pub const fn data_phase_0(self) -> u8 {
        ((self.0 & Self::DATA_PHASE_0) >> 16) as u8
    }
    /// Set data phase 1 clock length.
    #[inline]
    pub const fn set_data_phase_1(self, val: u8) -> Self {
        Self((self.0 & !Self::DATA_PHASE_1) | ((val as u32) << 24))
    }
    /// Get data phase 1 clock length.
    #[inline]
    pub const fn data_phase_1(self) -> u8 {
        ((self.0 & Self::DATA_PHASE_1) >> 24) as u8
    }
}

/// Duration of interval between frame in source clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PeriodInterval(u32);

impl PeriodInterval {
    const FRAME_INTERVAL: u32 = 0xFF;

    /// Set frame interval.
    #[inline]
    pub const fn set_frame_interval(self, val: u8) -> Self {
        Self((self.0 & !Self::FRAME_INTERVAL) | ((val as u32) << 0))
    }
    /// Get frame interval.
    #[inline]
    pub const fn frame_interval(self) -> u8 {
        ((self.0 & Self::FRAME_INTERVAL) >> 0) as u8
    }
}

/// Receive ignore feature configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReceiveIgnore(u32);

impl ReceiveIgnore {
    const STOP_POINT: u32 = 0x1F << 0;
    const START_POINT: u32 = 0x1F << 16;

    /// Set stop point for ignore function.
    #[inline]
    pub const fn set_stop_point(self, val: u8) -> Self {
        Self((self.0 & !Self::STOP_POINT) | (((val as u32) << 0) & Self::STOP_POINT))
    }
    /// Get stop point for ignore function.
    #[inline]
    pub const fn stop_point(self) -> u8 {
        ((self.0 & Self::STOP_POINT) >> 0) as u8
    }
    /// Set start point for ignore function.
    #[inline]
    pub const fn set_start_point(self, val: u8) -> Self {
        Self((self.0 & !Self::START_POINT) | (((val as u32) << 16) & Self::START_POINT))
    }
    /// Get start point for ignore function.
    #[inline]
    pub const fn start_point(self) -> u8 {
        ((self.0 & Self::START_POINT) >> 16) as u8
    }
}

/// Slave mode time-out interrupt trigger configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SlaveTimeout(u32);

impl SlaveTimeout {
    const THRESHOLD: u32 = 0xFFF << 0;

    /// Set timeout threshold.
    #[inline]
    pub const fn set_threshold(self, val: u16) -> Self {
        Self((self.0 & !Self::THRESHOLD) | ((val as u32) & Self::THRESHOLD))
    }
    /// Get timeout threshold.
    #[inline]
    pub const fn threshold(self) -> u16 {
        (self.0 & Self::THRESHOLD) as u16
    }
}

/// First-in first-out queue configuration register 0.
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

    /// Enable DMA transmit feature.
    #[inline]
    pub const fn enable_dma_transmit(self) -> Self {
        Self(self.0 | Self::DMA_TRANSMIT_ENABLE)
    }
    /// Disable DMA transmit feature.
    #[inline]
    pub const fn disable_dma_transmit(self) -> Self {
        Self(self.0 & !Self::DMA_TRANSMIT_ENABLE)
    }
    /// Check if DMA transmit feature is enabled.
    #[inline]
    pub const fn is_dma_transmit_enabled(self) -> bool {
        self.0 & Self::DMA_TRANSMIT_ENABLE != 0
    }
    /// Enable DMA receive feature.
    #[inline]
    pub const fn enable_dma_receive(self) -> Self {
        Self(self.0 | Self::DMA_RECEIVE_ENABLE)
    }
    /// Disable DMA receive feature.
    #[inline]
    pub const fn disable_dma_receive(self) -> Self {
        Self(self.0 & !Self::DMA_RECEIVE_ENABLE)
    }
    /// Check if DMA receive feature is enabled.
    #[inline]
    pub const fn is_dma_receive_enabled(self) -> bool {
        self.0 & Self::DMA_RECEIVE_ENABLE != 0
    }
    /// Clear transmit first-in first-out queue.
    #[inline]
    pub const fn clear_transmit_fifo(self) -> Self {
        Self(self.0 | Self::TRANSMIT_FIFO_CLEAR)
    }
    /// Clear receive first-in first-out queue.
    #[inline]
    pub const fn clear_receive_fifo(self) -> Self {
        Self(self.0 | Self::RECEIVE_FIFO_CLEAR)
    }
    /// Check if transmit first-in first-out queue has overflowed.
    #[inline]
    pub const fn is_transmit_overflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_OVERFLOW != 0
    }
    /// Check if transmit first-in first-out queue has underflowed.
    #[inline]
    pub const fn is_transmit_underflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_UNDERFLOW != 0
    }
    /// Check if receive first-in first-out queue has overflowed.
    #[inline]
    pub const fn is_receive_overflow(self) -> bool {
        self.0 & Self::RECEIVE_FIFO_OVERFLOW != 0
    }
    /// Check if receive first-in first-out queue has underflowed.
    #[inline]
    pub const fn is_receive_underflow(self) -> bool {
        self.0 & Self::RECEIVE_FIFO_UNDERFLOW != 0
    }
}

/// First-in first-out queue configuration register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig1(u32);

impl FifoConfig1 {
    const TRANSMIT_COUNT: u32 = 0x3f;
    const RECEIVE_COUNT: u32 = 0x3f << 8;
    const TRANSMIT_THRESHOLD: u32 = 0x1f << 16;
    const RECEIVE_THRESHOLD: u32 = 0x1f << 24;

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

#[cfg(test)]
mod tests {
    use super::{Config, FrameSize, Phase, Polarity, RegisterBlock};
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, config), 0x0);
        assert_eq!(offset_of!(RegisterBlock, interrupt_config), 0x4);
        assert_eq!(offset_of!(RegisterBlock, bus_busy), 0x08);
        assert_eq!(offset_of!(RegisterBlock, period_signal), 0x10);
        assert_eq!(offset_of!(RegisterBlock, period_interval), 0x14);
        assert_eq!(offset_of!(RegisterBlock, receive_ignore), 0x18);
        assert_eq!(offset_of!(RegisterBlock, slave_timeout), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_0), 0x80);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_1), 0x84);
        assert_eq!(offset_of!(RegisterBlock, data_write), 0x88);
        assert_eq!(offset_of!(RegisterBlock, data_read), 0x8c);
    }

    #[test]
    fn struct_config_functions() {
        let mut config = Config(0x0);

        config = config.enable_master();
        assert_eq!(config.0, 0x00000001);

        config = Config(0x0);
        config = config.disable_master();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_master_enable = config.is_master_enabled();
        assert_eq!(is_master_enable, false);

        config = Config(0x0);
        config = config.enable_slave();
        assert_eq!(config.0, 0x00000002);

        config = Config(0x0);
        config = config.disable_slave();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_slave_enable = config.is_slave_enabled();
        assert_eq!(is_slave_enable, false);

        config = Config(0x0);
        config.set_frame_size(FrameSize::Eight);
        assert_eq!(config.0, 0x0);
        assert_eq!(config.frame_size(), FrameSize::Eight);

        config = Config(0x1);
        config.set_frame_size(FrameSize::Sixteen);
        assert_eq!(config.0, 0x1);
        assert_eq!(config.frame_size(), FrameSize::Eight);

        config = Config(0x2);
        config.set_frame_size(FrameSize::TwentyFour);
        assert_eq!(config.0, 0x2);
        assert_eq!(config.frame_size(), FrameSize::Eight);

        config = Config(0x3);
        config.set_frame_size(FrameSize::ThirtyTwo);
        assert_eq!(config.0, 0x3);
        assert_eq!(config.frame_size(), FrameSize::Eight);

        config = Config(0x0);
        config = config.set_clock_polarity(Polarity::IdleHigh);
        assert_eq!(config.0, 0x00000010);
        assert_eq!(config.clock_polarity(), Polarity::IdleHigh);

        config = Config(0x1);
        config = config.set_clock_polarity(Polarity::IdleLow);
        assert_eq!(config.0, 0x00000001);
        assert_eq!(config.clock_polarity(), Polarity::IdleLow);

        config = Config(0x0);
        config = config.set_clock_phase(Phase::CaptureOnFirstTransition);
        assert_eq!(config.0, 0x000000020);
        assert_eq!(config.clock_phase(), Phase::CaptureOnFirstTransition);

        config = Config(0x1);
        config = config.set_clock_phase(Phase::CaptureOnSecondTransition);
        assert_eq!(config.0, 0x00000001);
        assert_eq!(config.clock_phase(), Phase::CaptureOnSecondTransition);

        config = Config(0x0);
        config = config.enable_bit_inverse();
        assert_eq!(config.0, 0x00000040);

        config = Config(0x0);
        config = config.disable_bit_inverse();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_bit_inverse_enabled = config.is_bit_inverse_enabled();
        assert_eq!(is_bit_inverse_enabled, false);

        config = Config(0x0);
        config = config.enable_byte_inverse();
        assert_eq!(config.0, 0x00000080);

        config = Config(0x0);
        config = config.disable_byte_inverse();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_byte_inverse_enabled = config.is_byte_inverse_enabled();
        assert_eq!(is_byte_inverse_enabled, false);

        config = Config(0x0);
        config = config.enable_receive_ignore();
        assert_eq!(config.0, 0x00000100);

        config = Config(0x0);
        config = config.disable_receive_ignore();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_receive_ignore_enabled = config.is_receive_ignore_enabled();
        assert_eq!(is_receive_ignore_enabled, false);

        config = Config(0x0);
        config = config.enable_master_continuous();
        assert_eq!(config.0, 0x00000200);

        config = Config(0x0);
        config = config.disable_master_continuous();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_master_continuous_enabled = config.is_master_continuous_enabled();
        assert_eq!(is_master_continuous_enabled, false);

        config = Config(0x0);
        config = config.enable_slave_three_pin();
        assert_eq!(config.0, 0x00000400);

        config = Config(0x0);
        config = config.disable_slave_three_pin();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_slave_three_pin_enabled = config.is_slave_three_pin_enabled();
        assert_eq!(is_slave_three_pin_enabled, false);

        config = Config(0x0);
        config = config.enable_deglitch();
        assert_eq!(config.0, 0x00000800);

        config = Config(0x0);
        config = config.disable_deglitch();
        assert_eq!(config.0, 0x00000000);

        config = Config(0x0);
        let is_deglitch_enabled = config.is_deglitch_enabled();
        assert_eq!(is_deglitch_enabled, false);

        config = Config(0x0);
        config = config.set_deglitch_cycle(0);
        assert_eq!(config.0, 0x0);
        assert_eq!(config.deglitch_cycle(), 0);
    }
}
