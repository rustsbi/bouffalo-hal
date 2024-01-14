//! Direct Memory Access peripheral.

use volatile_register::{RO, RW, WO};

/// Direct Memory Access peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt register block.
    pub interrupts: InterruptRegisters,
    /// Channel enable states.
    pub enabled_channels: RO<u8>,
    _reserved0: [u8; 0x13],
    /// Peripheral configuration register.
    pub global_config: RW<GlobalConfig>,
    _reserved1: [u8; 0xcc],
    /// Channel register block.
    pub channels: [ChannelRegisters; 8],
}

/// Interrupt register block.
#[repr(C)]
pub struct InterruptRegisters {
    /// Global interrupt state after masking.
    pub global_state: RO<u8>,
    _reserved0: [u8; 3],
    /// Transfer complete interrupt state.
    pub transfer_complete_state: RO<u8>,
    _reserved1: [u8; 3],
    /// Clear transfer complete interrupt.
    pub transfer_complete_clear: WO<u8>,
    _reserved2: [u8; 3],
    /// Error interrupt state.
    pub error_state: RO<u8>,
    _reserved3: [u8; 3],
    /// Clear error interrupt.
    pub error_clear: WO<u8>,
    _reserved4: [u8; 3],
    /// Transfer complete interrupt state before masking.
    pub raw_transfer_complete: RO<u8>,
    _reserved5: [u8; 3],
    /// Error interrupt state before masking.
    pub raw_error: WO<u8>,
    _reserved6: [u8; 3],
}

/// Peripheral configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlobalConfig(u32);

/// Channel register block.
#[repr(C)]
pub struct ChannelRegisters {
    /// Source address.
    pub source_address: RW<u32>,
    /// Destination address.
    pub destination_address: RW<u32>,
    /// Physical address to first linked list item.
    pub linked_list_item: RW<u32>,
    /// Linked list item control register.
    pub control: RW<LliControl>,
    /// Channel configuration register.
    pub config: RW<ChannelConfig>,
    _reserved0: [u8; 0xec],
}

/// Linked list item descriptor.
#[repr(C)]
pub struct LliItem {
    /// Source address.
    pub source_address: u32,
    /// Destination address.
    pub destination_address: u32,
    /// Physical address to next linked list item.
    pub linked_list_item: u32,
    /// Linked list item control register.
    pub control: LliControl,
}

/// Control register in linked list item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LliControl(u32);

/// Channel configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelConfig(u32);

#[cfg(test)]
mod tests {
    use super::{ChannelRegisters, InterruptRegisters, RegisterBlock};
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, interrupts), 0x00);
        assert_eq!(offset_of!(RegisterBlock, enabled_channels), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, global_config), 0x30);
        assert_eq!(offset_of!(RegisterBlock, channels), 0x100);
    }

    #[rustfmt::skip]
    #[test]
    fn struct_interrupt_registers_offset() {
        assert_eq!(offset_of!(InterruptRegisters, global_state), 0x00);
        assert_eq!(offset_of!(InterruptRegisters, transfer_complete_state), 0x04);
        assert_eq!(offset_of!(InterruptRegisters, transfer_complete_clear), 0x08);
        assert_eq!(offset_of!(InterruptRegisters, error_state), 0x0c);
        assert_eq!(offset_of!(InterruptRegisters, error_clear), 0x10);
        assert_eq!(offset_of!(InterruptRegisters, raw_transfer_complete), 0x14);
        assert_eq!(offset_of!(InterruptRegisters, raw_error), 0x18);
    }

    #[test]
    fn struct_channel_registers_offset_size() {
        assert_eq!(offset_of!(ChannelRegisters, source_address), 0x00);
        assert_eq!(offset_of!(ChannelRegisters, destination_address), 0x04);
        assert_eq!(offset_of!(ChannelRegisters, linked_list_item), 0x08);
        assert_eq!(offset_of!(ChannelRegisters, control), 0x0c);
        assert_eq!(offset_of!(ChannelRegisters, config), 0x10);
        assert_eq!(core::mem::size_of::<ChannelRegisters>(), 0x100);
    }
}
