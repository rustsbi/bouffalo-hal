use volatile_register::{RO, RW};

/// Hardware LZ4 decompressor registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Decompressor peripheral configuration.
    pub config: RW<Config>,
    _reserved: [u8; 0xC],
    /// Writes source address before decompression.
    pub source_start: RW<SourceStart>,
    /// Reads the end address of source after decompression.
    pub source_end: RO<SourceEnd>,
    /// Writes destination address before decompression.
    pub destination_start: RW<DestinationStart>,
    /// Reads the end address of destination after decompression.
    pub destination_end: RW<DestinationEnd>,
    /// Interrupt enable register.
    pub interrupt_enable: RW<InterruptEnable>,
    /// Interrupt state register.
    pub interrupt_state: RO<InterruptState>,
}

/// Decompressor peripheral configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Config(u32);

impl Config {
    const ENABLE: u32 = 1 << 0;
    const SUSPENDED: u32 = 1 << 1;
    const HAS_CHECKSUM: u32 = 1 << 4;
    /// Enable this peripheral.
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable this peripheral.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Check if this peripheral is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::ENABLE != 0
    }
    /// Set suspend state of current peripheral.
    #[inline]
    pub const fn set_suspended(self, val: bool) -> Self {
        Self((self.0 & !Self::SUSPENDED) | ((if val { 1 } else { 0 }) << 1))
    }
    /// Get suspend state of current peripheral.
    #[inline]
    pub const fn suspended(self) -> bool {
        ((self.0 & Self::SUSPENDED) >> 1) != 0
    }
    /// Set if the decompressor recognizes LZ4 block checksum.
    #[inline]
    pub const fn set_has_checksum(self, val: bool) -> Self {
        Self((self.0 & !Self::HAS_CHECKSUM) | ((if val { 1 } else { 0 }) << 4))
    }
    /// Get if the decompressor recognizes LZ4 block checksum.
    #[inline]
    pub const fn has_checksum(self) -> bool {
        ((self.0 & Self::HAS_CHECKSUM) >> 4) != 0
    }
}

/// Writes source address before decompression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SourceStart(pub(crate) u32);

impl SourceStart {
    const START: u32 = 0x3ffffff << 0;
    const BASE: u32 = 0x3f << 26;
    /// Set start address.
    #[inline]
    pub const fn set_start(self, val: u32) -> Self {
        Self((self.0 & !Self::START) | (val << 0))
    }
    /// Get start address.
    #[inline]
    pub const fn start(self) -> u32 {
        ((self.0 & Self::START) >> 0) as u32
    }
    /// Set base address.
    #[inline]
    pub const fn set_base(self, val: u32) -> Self {
        Self((self.0 & !Self::BASE) | (val << 26))
    }
    /// Get base address.
    #[inline]
    pub const fn base(self) -> u32 {
        ((self.0 & Self::BASE) >> 26) as u32
    }
}

/// Reads the end address of source after decompression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SourceEnd(u32);

impl SourceEnd {
    const END: u32 = 0x3ffffff << 0;
    /// Set end address.
    #[inline]
    pub const fn end(self) -> u32 {
        ((self.0 & Self::END) >> 0) as u32
    }
}

/// Writes destination address before decompression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DestinationStart(pub(crate) u32);

impl DestinationStart {
    const START: u32 = 0x3ffffff << 0;
    const BASE: u32 = 0x3f << 26;
    /// Set start address.
    #[inline]
    pub const fn set_start(self, val: u32) -> Self {
        Self((self.0 & !Self::START) | (val << 0))
    }
    /// Get start address.
    #[inline]
    pub const fn start(self) -> u32 {
        ((self.0 & Self::START) >> 0) as u32
    }
    /// Set base address.
    #[inline]
    pub const fn set_base(self, val: u32) -> Self {
        Self((self.0 & !Self::BASE) | (val << 26))
    }
    /// Get base address.
    #[inline]
    pub const fn base(self) -> u32 {
        ((self.0 & Self::BASE) >> 26) as u32
    }
}

/// Reads the end address of destination after decompression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DestinationEnd(u32);

impl DestinationEnd {
    const END: u32 = 0x3ffffff << 0;
    /// Get the end address.
    #[inline]
    pub const fn end(self) -> u32 {
        ((self.0 & Self::END) >> 0) as u32
    }
}

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptEnable(u32);

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

/// Interrupt state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptState(u32);

impl InterruptState {
    /// Check if has interrupt flag.
    #[inline]
    pub const fn has_interrupt(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Interrupt {
    /// Decompression finished.
    Done = 0,
    /// Error occurred while decompression.
    Error = 1,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;
    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, config), 0x00);
        assert_eq!(offset_of!(RegisterBlock, source_start), 0x10);
        assert_eq!(offset_of!(RegisterBlock, source_end), 0x14);
        assert_eq!(offset_of!(RegisterBlock, destination_start), 0x18);
        assert_eq!(offset_of!(RegisterBlock, destination_end), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x20);
        assert_eq!(offset_of!(RegisterBlock, interrupt_state), 0x24);
    }

    // TODO register-level unit tests.
}
