//! Display bus interface.

use volatile_register::RW;

/// Display bus interface registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Function configuration register.
    pub config: RW<Config>,
    _reserved: [u8; 0x7b],
    /// First-in first-out queue configuration 0.
    pub fifo_config_0: RW<FifoConfig0>,
    /// First-in first-out queue configuration 1.
    pub fifo_config_1: RW<FifoConfig1>,
}

/// Function configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Config(u32);

impl Config {
    const MASTER_ENABLE: u32 = 1 << 0;
    const SELECT_TYPE: u32 = 1 << 1;
    const COMMAND_ENABLE: u32 = 1 << 2;
    const DATA_ENABLE: u32 = 1 << 3;
    const DATA_PHASE: u32 = 1 << 4;
    const DATA_TYPE: u32 = 1 << 5;
    const DATA_BYTE_COUNT: u32 = 3 << 6;
    const COMMAND: u32 = 0xff << 8;
    const SCL_POLARITY: u32 = 1 << 16;
    const SCL_PHASE: u32 = 1 << 17;
    const CONTINUOUS_TRANSFER: u32 = 1 << 18;
    const DUMMY_ENABLE: u32 = 1 << 19;
    const DUMMY_CYCLE: u32 = 0xf << 20;
    const THREE_WIRE_MODE: u32 = 1 << 27;
    const DEGLITCH_ENABLE: u32 = 1 << 28;
    const DEGLITCH_CYCLE: u32 = 0x7 << 29;

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
    /// Set type B.
    #[inline]
    pub fn set_type_b(self) -> Self {
        Self(self.0 & !Self::SELECT_TYPE)
    }
    /// Set type C.
    #[inline]
    pub fn set_type_c(self) -> Self {
        Self(self.0 | Self::SELECT_TYPE)
    }
    /// Check if type C is selected.
    #[inline]
    pub fn is_type_c(self) -> bool {
        self.0 & Self::SELECT_TYPE != 0
    }
    /// Check if type B is selected.
    #[inline]
    pub fn is_type_b(self) -> bool {
        self.0 & Self::SELECT_TYPE == 0
    }
    /// Enable command.
    #[inline]
    pub fn enable_command(self) -> Self {
        Self(self.0 | Self::COMMAND_ENABLE)
    }
    /// Disable command.
    #[inline]
    pub fn disable_command(self) -> Self {
        Self(self.0 & !Self::COMMAND_ENABLE)
    }
    /// Check if command is enabled.
    #[inline]
    pub fn is_command_enabled(self) -> bool {
        self.0 & Self::COMMAND_ENABLE != 0
    }
    /// Enable data.
    #[inline]
    pub fn enable_data(self) -> Self {
        Self(self.0 | Self::DATA_ENABLE)
    }
    /// Disable data.
    #[inline]
    pub fn disable_data(self) -> Self {
        Self(self.0 & !Self::DATA_ENABLE)
    }
    /// Check if data is enabled.
    #[inline]
    pub fn is_data_enabled(self) -> bool {
        self.0 & Self::DATA_ENABLE != 0
    }
    /// Set data phase to read.
    #[inline]
    pub fn set_data_read(self) -> Self {
        Self(self.0 | Self::DATA_PHASE)
    }
    /// Set data phase to write.
    #[inline]
    pub fn set_data_write(self) -> Self {
        Self(self.0 & !Self::DATA_PHASE)
    }
    /// Check if data phase is read.
    #[inline]
    pub fn is_data_read(self) -> bool {
        self.0 & Self::DATA_PHASE != 0
    }
    /// Check if data phase is write.
    #[inline]
    pub fn is_data_write(self) -> bool {
        self.0 & Self::DATA_PHASE == 0
    }
    /// Set data type to normal.
    #[inline]
    pub fn set_data_normal(self) -> Self {
        Self(self.0 & !Self::DATA_TYPE)
    }
    /// Set data type to pixel.
    #[inline]
    pub fn set_data_pixel(self) -> Self {
        Self(self.0 | Self::DATA_TYPE)
    }
    /// Check if data type is normal.
    #[inline]
    pub fn is_data_normal(self) -> bool {
        self.0 & Self::DATA_TYPE == 0
    }
    /// Check if data type is pixel.
    #[inline]
    pub fn is_data_pixel(self) -> bool {
        self.0 & Self::DATA_TYPE != 0
    }
    /// Set data byte count.
    #[inline]
    pub fn set_data_byte_count(self, count: u8) -> Self {
        Self((self.0 & !Self::DATA_BYTE_COUNT) | ((count as u32) << 6))
    }
    /// Get data byte count.
    #[inline]
    pub fn data_byte_count(self) -> u8 {
        ((self.0 & Self::DATA_BYTE_COUNT) >> 6) as u8
    }
    /// Set command.
    #[inline]
    pub fn set_command(self, command: u8) -> Self {
        Self((self.0 & !Self::COMMAND) | ((command as u32) << 8))
    }
    /// Get command.
    #[inline]
    pub fn command(self) -> u8 {
        ((self.0 & Self::COMMAND) >> 8) as u8
    }
    /// Set SCL polarity.
    #[inline]
    pub fn set_scl_polarity(self, polarity: bool) -> Self {
        if polarity {
            Self(self.0 | Self::SCL_POLARITY)
        } else {
            Self(self.0 & !Self::SCL_POLARITY)
        }
    }
    /// Get SCL polarity.
    #[inline]
    pub fn scl_polarity(self) -> bool {
        self.0 & Self::SCL_POLARITY != 0
    }
    /// Set SCL clock phase inverse.
    #[inline]
    pub fn set_scl_phase(self, phase: bool) -> Self {
        if phase {
            Self(self.0 | Self::SCL_PHASE)
        } else {
            Self(self.0 & !Self::SCL_PHASE)
        }
    }
    /// Get SCL clock phase inverse.
    #[inline]
    pub fn scl_phase(self) -> bool {
        self.0 & Self::SCL_PHASE != 0
    }
    /// Enable continuous transfer.
    #[inline]
    pub fn enable_continuous_transfer(self) -> Self {
        Self(self.0 | Self::CONTINUOUS_TRANSFER)
    }
    /// Disable continuous transfer.
    #[inline]
    pub fn disable_continuous_transfer(self) -> Self {
        Self(self.0 & !Self::CONTINUOUS_TRANSFER)
    }
    /// Check if continuous transfer is enabled.
    #[inline]
    pub fn is_continuous_transfer_enabled(self) -> bool {
        self.0 & Self::CONTINUOUS_TRANSFER != 0
    }
    /// Enable dummy cycle.
    #[inline]
    pub fn enable_dummy_cycle(self) -> Self {
        Self(self.0 | Self::DUMMY_ENABLE)
    }
    /// Disable dummy cycle.
    #[inline]
    pub fn disable_dummy_cycle(self) -> Self {
        Self(self.0 & !Self::DUMMY_ENABLE)
    }
    /// Check if dummy cycle is enabled.
    #[inline]
    pub fn is_dummy_cycle_enabled(self) -> bool {
        self.0 & Self::DUMMY_ENABLE != 0
    }
    /// Set dummy cycle count.
    #[inline]
    pub fn set_dummy_cycle_count(self, count: u8) -> Self {
        Self((self.0 & !Self::DUMMY_CYCLE) | ((count as u32) << 20))
    }
    /// Get dummy cycle count.
    #[inline]
    pub fn dummy_cycle_count(self) -> u8 {
        ((self.0 & Self::DUMMY_CYCLE) >> 20) as u8
    }
    /// Set type c 3-wire mode.
    #[inline]
    pub fn set_type_c_3_wire_mode(self) -> Self {
        Self(self.0 | Self::THREE_WIRE_MODE)
    }
    /// Set type c 4-wire mode.
    #[inline]
    pub fn set_type_c_4_wire_mode(self) -> Self {
        Self(self.0 & !Self::THREE_WIRE_MODE)
    }
    /// Check if type c 3-wire mode is selected.
    #[inline]
    pub fn is_type_c_3_wire_mode(self) -> bool {
        self.0 & Self::THREE_WIRE_MODE != 0
    }
    /// Check if type c 4-wire mode is selected.
    #[inline]
    pub fn is_type_c_4_wire_mode(self) -> bool {
        self.0 & Self::THREE_WIRE_MODE == 0
    }
    /// Enable deglitch.
    #[inline]
    pub fn enable_deglitch(self) -> Self {
        Self(self.0 | Self::DEGLITCH_ENABLE)
    }
    /// Disable deglitch.
    #[inline]
    pub fn disable_deglitch(self) -> Self {
        Self(self.0 & !Self::DEGLITCH_ENABLE)
    }
    /// Check if deglitch is enabled.
    #[inline]
    pub fn is_deglitch_enabled(self) -> bool {
        self.0 & Self::DEGLITCH_ENABLE != 0
    }
    /// Set deglitch cycle count.
    #[inline]
    pub fn set_deglitch_cycle_count(self, count: u8) -> Self {
        Self((self.0 & !Self::DEGLITCH_CYCLE) | ((count as u32) << 29))
    }
    /// Get deglitch cycle count.
    #[inline]
    pub fn deglitch_cycle_count(self) -> u8 {
        ((self.0 & Self::DEGLITCH_CYCLE) >> 29) as u8
    }
}

/// First-in first-out queue configuration 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig0(u32);

impl FifoConfig0 {
    const DMA_TRANSMIT_ENABLE: u32 = 1 << 0;
    const TRANSMIT_FIFO_CLEAR: u32 = 1 << 2;
    const TRANSMIT_FIFO_OVERFLOW: u32 = 1 << 4;
    const TRANSMIT_FIFO_UNDERFLOW: u32 = 1 << 5;

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
    /// Clear transmit first-in first-out queue.
    #[inline]
    pub fn clear_transmit_fifo(self) -> Self {
        Self(self.0 | Self::TRANSMIT_FIFO_CLEAR)
    }
    /// Check if transmit first-in first-out queue has overflowed.
    #[inline]
    pub fn is_transmit_fifo_overflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_OVERFLOW != 0
    }
    /// Check if transmit first-in first-out queue has underflowed.
    #[inline]
    pub fn is_transmit_fifo_underflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_UNDERFLOW != 0
    }
}

/// First-in first-out queue configuration 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoConfig1(u32);

impl FifoConfig1 {
    const TRANSMIT_COUNT: u32 = 0xf;
    const TRANSMIT_THRESHOLD: u32 = 0x7 << 16;

    /// Get number of empty spaces remained in transmit FIFO queue.
    #[inline]
    pub const fn transmit_available_bytes(self) -> u8 {
        (self.0 & Self::TRANSMIT_COUNT) as u8
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
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, config), 0x00);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_0), 0x80);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_1), 0x84);
    }
}
