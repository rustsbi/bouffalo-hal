//! Ethernet Media Access Control peripheral.
use volatile_register::{RO, RW};

/// Ethernet Media Access Control peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Configuration
    pub mode: RW<Mode>,
    /// Transmit control
    pub interrupt_source: RW<InterruptSource>,
    /// Interrupt mask
    pub interrupt_mask: RW<InterruptMask>,
    /// Inter-packet gap (backed-gap only, non-backed-gap not seen in bl-docs)
    pub backed_gap: RW<BackedGap>,
    _reserved0: [u8; 0x08],
    /// Frame length, seen in bl-docs as PACKETLEN
    pub frame_length: RW<FrameLength>,
    /// Collision config
    pub collision: RW<Collision>,
    /// transmit_buffer, seen in bl-docs as TX_BD_NUM
    pub transmit_buffer: RW<TransmitBuffer>,
    // Flow control, not seen in bl-docs, temporarily reserved
    _reserved1: [u8; 0x04],
    /// MII mode
    pub mii_mode: RW<MiiMode>,
    /// MII command
    pub mii_command: RW<MiiCommand>,
    /// MII address
    pub mii_address: RW<MiiAddress>,
    /// MII write control
    pub control_write: RW<ControlWrite>,
    /// MII read control
    pub control_read: RW<ControlRead>,
    /// MII state
    pub mii_state: RO<MiiState>,
    /// Media Access Control address 0 and 1
    pub mac_address: [RW<MacAddress>; 2],
    /// Hash 0 and 1
    pub hash: [RW<Hash>; 2],
    /// Transmit control
    pub transmit_control: RW<TransmitControl>,
}

/// EMAC mode configuration register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Mode(u32);

/// EMAC transmit control register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptSource(u32);

/// EMAC interrupt mask register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptMask(u32);

/// EMAC inter packet gap (backed gap) register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BackedGap(u32);

/// EMAC frame length buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FrameLength(u32);

/// EMAC collision register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Collision(u32);

/// EMAC transmit buffer register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TransmitBuffer(u32);

/// MII clock divider and premable register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MiiMode(u32);

/// MII control data, read and scan state register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MiiCommand(u32);

/// MII physical layer bus address register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MiiAddress(u32);

/// MII write control register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ControlWrite(u32);

/// MII read control register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ControlRead(u32);

/// MII state register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MiiState(u32);

/// Media Access Control address register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct MacAddress(u32);

/// hash register (64-bit to double 32-bit)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Hash(u32);
/// Transit control register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct TransmitControl(u32);

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, mode), 0x00);
        assert_eq!(offset_of!(RegisterBlock, interrupt_source), 0x04);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x08);
        assert_eq!(offset_of!(RegisterBlock, backed_gap), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, frame_length), 0x18);
        assert_eq!(offset_of!(RegisterBlock, collision), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, transmit_buffer), 0x20);
        assert_eq!(offset_of!(RegisterBlock, mii_mode), 0x28);
        assert_eq!(offset_of!(RegisterBlock, mii_command), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, mii_address), 0x30);
        assert_eq!(offset_of!(RegisterBlock, control_write), 0x34);
        assert_eq!(offset_of!(RegisterBlock, control_read), 0x38);
        assert_eq!(offset_of!(RegisterBlock, mii_state), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, mac_address), 0x40);
        assert_eq!(offset_of!(RegisterBlock, hash), 0x48);
        assert_eq!(offset_of!(RegisterBlock, transmit_control), 0x50);
    }
}
