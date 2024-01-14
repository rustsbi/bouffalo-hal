//! Inter-IC sound bus peripheral.

use volatile_register::{RO, RW, WO};

/// Inter-IC sound bus peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Peripheral configuration register.
    pub config: RW<Config>,
    /// Interrupt states.
    pub interrupt_config: RW<InterruptConfig>,
    /// Bit clock configuration.
    pub bclk_config: RW<BclkConfig>,
    _reserved0: [u8; 0x74],
    /// First-in first-out queue configuration register 0.
    pub fifo_config_0: RW<FifoConfig0>,
    /// First-in first-out queue configuration register 1.
    pub fifo_config_1: RW<FifoConfig1>,
    /// First-in first-out queue write data register.
    pub data_write: WO<u32>,
    /// First-in first-out queue read data register.
    pub data_read: RO<u32>,
    _reserved1: [u8; 0x6c],
    /// Input/output signal configuration register.
    pub io_config: RO<u32>,
}

/// Peripheral configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Config(u32);

/// Interrupt configuration and state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptConfig(u32);

/// Bit clock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BclkConfig(u32);

/// First-in first-out queue configuration register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig0(u32);

/// First-in first-out queue configuration register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig1(u32);

/// Input/output signal configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IoConfig(u32);

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, config), 0x0);
        assert_eq!(offset_of!(RegisterBlock, interrupt_config), 0x4);
        assert_eq!(offset_of!(RegisterBlock, bclk_config), 0x08);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_0), 0x80);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_1), 0x84);
        assert_eq!(offset_of!(RegisterBlock, data_write), 0x88);
        assert_eq!(offset_of!(RegisterBlock, data_read), 0x8c);
        assert_eq!(offset_of!(RegisterBlock, io_config), 0xfc);
    }
}
