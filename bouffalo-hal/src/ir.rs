//! Infrared remote peripheral.

use volatile_register::{RO, RW};

/// Infrared remote peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    /// Receive configuration register.
    pub receive_config: RW<ReceiveConfig>,
    /// Receive interrupt states and configurations.
    pub receive_interrupt: RW<ReceiveInterrupt>,
    /// Receive pulse width threshold configuration.
    pub receive_threshold: RW<ReceiveThreshold>,
    _reserved1: [u8; 0x4],
    /// Receive data count.
    pub receive_data_count: RO<u8>,
    _reserved2: [u8; 0x3],
    /// Low 32-bit of receive data.
    pub receive_word_0: RO<u32>,
    /// High 32-bit of receive data.
    pub receive_word_1: RO<u32>,
    _reserved3: [u8; 0x24],
    /// First-in first-out queue configuration register 0.
    pub fifo_config_0: RW<FifoConfig0>,
    /// First-in first-out queue configuration register 1.
    pub fifo_config_1: RW<FifoConfig1>,
    _reserved4: [u8; 0x4],
    /// First-in first-out queue read data register.
    pub fifo_read: RO<u32>,
}

/// Receive configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReceiveConfig(u32);

/// Receive interrupt state and configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReceiveInterrupt(u32);

/// Receive pulse width threshold configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReceiveThreshold(u32);

/// First-in first-out queue configuration register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig0(u32);

/// First-in first-out queue configuration register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig1(u32);

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, receive_config), 0x40);
        assert_eq!(offset_of!(RegisterBlock, receive_interrupt), 0x44);
        assert_eq!(offset_of!(RegisterBlock, receive_threshold), 0x48);
        assert_eq!(offset_of!(RegisterBlock, receive_data_count), 0x50);
        assert_eq!(offset_of!(RegisterBlock, receive_word_0), 0x54);
        assert_eq!(offset_of!(RegisterBlock, receive_word_1), 0x58);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_0), 0x80);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_1), 0x84);
        assert_eq!(offset_of!(RegisterBlock, fifo_read), 0x8c);
    }
}
