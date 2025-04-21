//! SHA (Secure Hash Algorithm) hardware accelerator driver.
//!
//! This module provides access to the SHA hardware accelerator peripheral,
//! supporting SHA-1, SHA-2 family, MD5 and CRC calculations.

use crate::sec::Endian;
use volatile_register::{RO, RW};

/// SHA hardware registers block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register.
    pub control: RW<Control>,
    /// Message source address register.
    pub message_source_address: RW<u32>,
    /// Status register.
    pub status: RO<u32>,
    /// Endianness register.
    pub endianness: RW<Endianness>,
    /// SHA hash result low 32-bit register group.
    pub hash_l: [RO<u32>; 8],
    /// SHA hash result high 32-bit register group.
    pub hash_h: [RO<u32>; 8],
    /// AES link configuration address (word aligned).
    pub link_config_address: RW<u32>,
    _reserved: [u8; 168],
    /// Control protection register.
    pub control_protection: RW<ControlProtection>,
}

/// Supported hash operation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashMode {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_224,
    SHA512_256,
    MD5,
    CRC16,
    CRC32,
}

/// Hash calculation mode selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashSelect {
    /// Start new hash calculation.
    NewHash = 0,
    /// Continue with previous hash value.
    AccumulateLastHash = 1,
}

/// Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    // Register bit definitions
    const BUSY: u32 = 1 << 0;
    const TRIGGER: u32 = 1 << 1;
    const MODE: u32 = 0x7 << 2;
    const ENABLE: u32 = 1 << 5;
    const HASH_SELECT: u32 = 1 << 6;
    const INTERRUPT: u32 = 1 << 8;
    const INTERRUPT_CLEAR: u32 = 1 << 9;
    const INTERRUPT_SET: u32 = 1 << 10;
    const INTERRUPT_MASK: u32 = 1 << 11;
    const MODE_EXTENSION: u32 = 0x3 << 12;
    const LINK_MODE: u32 = 1 << 15;
    const MESSAGE_LENGTH: u32 = 0xffff << 16;

    /// Check if SHA engine is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }

    /// Trigger SHA operation.
    #[inline]
    pub fn trigger(&mut self) {
        self.0 |= Self::TRIGGER;
    }

    /// Set SHA operation mode.
    #[inline]
    pub fn set_hash_mode(&mut self, mode: HashMode) {
        // Clear existing mode bits
        self.0 &= !(Self::MODE | Self::MODE_EXTENSION);

        match mode {
            HashMode::SHA256 => self.0 |= 0 << 2,
            HashMode::SHA224 => self.0 |= 1 << 2,
            HashMode::SHA1 => self.0 |= 2 << 2,
            HashMode::SHA512 => self.0 |= 4 << 2,
            HashMode::SHA384 => self.0 |= 5 << 2,
            HashMode::SHA512_224 => self.0 |= 6 << 2,
            HashMode::SHA512_256 => self.0 |= 7 << 2,
            HashMode::MD5 => {
                self.0 |= 1 << 12; // Set extension mode to 1
            }
            HashMode::CRC16 => {
                self.0 |= 2 << 12; // Set extension mode to 2
            }
            HashMode::CRC32 => {
                self.0 |= 3 << 12; // Set extension mode to 3
            }
        }
    }

    /// Get current operation mode.
    #[inline]
    pub fn hash_mode(&self) -> HashMode {
        let mode = (self.0 & Self::MODE) >> 2;
        let mode_ext = (self.0 & Self::MODE_EXTENSION) >> 12;

        match (mode_ext, mode) {
            (0, 0) => HashMode::SHA256,
            (0, 1) => HashMode::SHA224,
            (0, 2) => HashMode::SHA1,
            (0, 4) => HashMode::SHA512,
            (0, 5) => HashMode::SHA384,
            (0, 6) => HashMode::SHA512_224,
            (0, 7) => HashMode::SHA512_256,
            (1, _) => HashMode::MD5,
            (2, _) => HashMode::CRC16,
            (3, _) => HashMode::CRC32,
            _ => HashMode::SHA256, // Default value
        }
    }

    /// Enable SHA engine.
    #[inline]
    pub fn enable(&mut self) {
        self.0 |= Self::ENABLE;
    }

    /// Disable SHA engine.
    #[inline]
    pub fn disable(&mut self) {
        self.0 &= !Self::ENABLE;
    }

    /// Check if SHA engine is enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }

    /// Set hash calculation mode.
    #[inline]
    pub fn set_hash_select(&mut self, select: HashSelect) {
        match select {
            HashSelect::NewHash => self.0 &= !Self::HASH_SELECT,
            HashSelect::AccumulateLastHash => self.0 |= Self::HASH_SELECT,
        }
    }

    /// Get current hash calculation mode.
    #[inline]
    pub fn hash_select(&self) -> HashSelect {
        if (self.0 & Self::HASH_SELECT) != 0 {
            HashSelect::AccumulateLastHash
        } else {
            HashSelect::NewHash
        }
    }

    /// Check interrupt status.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        (self.0 & Self::INTERRUPT) != 0
    }

    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        self.0 |= Self::INTERRUPT_CLEAR;
    }

    /// Set interrupt flag.
    #[inline]
    pub fn set_interrupt(&mut self) {
        self.0 |= Self::INTERRUPT_SET;
    }
    /// Enable interrupt mask.
    #[inline]
    pub fn enable_interrupt_mask(&mut self) {
        self.0 |= Self::INTERRUPT_MASK;
    }

    /// Disable interrupt mask.
    #[inline]
    pub fn disable_interrupt_mask(&mut self) {
        self.0 &= !Self::INTERRUPT_MASK;
    }

    /// Check if interrupt mask is enabled.
    #[inline]
    pub fn is_interrupt_mask_enabled(&self) -> bool {
        (self.0 & Self::INTERRUPT_MASK) != 0
    }

    /// Enable link mode.
    #[inline]
    pub fn enable_link_mode(&mut self) {
        self.0 |= Self::LINK_MODE;
    }

    /// Disable link mode.
    #[inline]
    pub fn disable_link_mode(&mut self) {
        self.0 &= !Self::LINK_MODE;
    }

    /// Check if link mode is enabled.
    #[inline]
    pub fn is_link_mode_enabled(&self) -> bool {
        (self.0 & Self::LINK_MODE) != 0
    }

    /// Set number of 512-bit blocks to process.
    #[inline]
    pub fn set_message_length(&mut self, length: u32) {
        self.0 &= !Self::MESSAGE_LENGTH;
        self.0 |= (length << 16) & Self::MESSAGE_LENGTH;
    }

    /// Get number of 512-bit blocks to process.
    #[inline]
    pub fn message_length(&self) -> u32 {
        (self.0 & Self::MESSAGE_LENGTH) >> 16
    }
}

/// Endianness register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Endianness(u32);

impl Endianness {
    const OUTPUT_DATA_ENDIAN: u32 = 1 << 0;

    /// Set output data endianness.
    #[inline]
    pub fn set_data_out_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::OUTPUT_DATA_ENDIAN,
            Endian::Big => self.0 |= Self::OUTPUT_DATA_ENDIAN,
        }
    }

    /// Get output data endianness.
    #[inline]
    pub fn data_out_endian(&self) -> Endian {
        if (self.0 & Self::OUTPUT_DATA_ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }
}

/// Control protection register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ControlProtection(u32);

impl ControlProtection {
    const ENABLE_ID0_ACCESS_RIGHT: u32 = 1 << 1;
    const ENABLE_ID1_ACCESS_RIGHT: u32 = 1 << 2;

    /// Enable ID0 access right.
    #[inline]
    pub fn enable_id0_access_right(&mut self) {
        self.0 |= Self::ENABLE_ID0_ACCESS_RIGHT;
    }

    /// Disable ID0 access right.
    #[inline]
    pub fn disable_id0_access_right(&mut self) {
        self.0 &= !Self::ENABLE_ID0_ACCESS_RIGHT;
    }

    /// Enable ID1 access right.
    #[inline]
    pub fn enable_id1_access_right(&mut self) {
        self.0 |= Self::ENABLE_ID1_ACCESS_RIGHT;
    }

    /// Disable ID1 access right.
    #[inline]
    pub fn disable_id1_access_right(&mut self) {
        self.0 &= !Self::ENABLE_ID1_ACCESS_RIGHT;
    }

    /// Check if ID0 access right is enabled.
    #[inline]
    pub fn is_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if ID1 access right is enabled.
    #[inline]
    pub fn is_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_ID1_ACCESS_RIGHT) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x00);
        assert_eq!(offset_of!(RegisterBlock, message_source_address), 0x04);
        assert_eq!(offset_of!(RegisterBlock, status), 0x08);
        assert_eq!(offset_of!(RegisterBlock, endianness), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, hash_l), 0x10);
        assert_eq!(offset_of!(RegisterBlock, hash_h), 0x30);
        assert_eq!(offset_of!(RegisterBlock, link_config_address), 0x50);
        assert_eq!(offset_of!(RegisterBlock, control_protection), 0xFC);
    }

    #[test]
    fn struct_control_functions() {
        let mut control = Control(0);

        // Test busy status check
        assert!(!control.is_busy());
        control.0 |= Control::BUSY;
        assert!(control.is_busy());
        assert_eq!(control.0, 0x1);

        // Test trigger operation
        control = Control(0);
        control.trigger();
        assert_eq!(control.0, 0x2);

        // Test hash mode setting and getting
        control = Control(0);
        control.set_hash_mode(HashMode::SHA256);
        assert_eq!(control.hash_mode(), HashMode::SHA256);
        assert_eq!(control.0, 0x0);

        control.set_hash_mode(HashMode::MD5);
        assert_eq!(control.hash_mode(), HashMode::MD5);
        assert_eq!(control.0, 0x1000);

        // Test enable/disable functionality
        control = Control(0);
        assert!(!control.is_enabled());
        control.enable();
        assert!(control.is_enabled());
        assert_eq!(control.0, 0x20);
        control.disable();
        assert!(!control.is_enabled());
        assert_eq!(control.0, 0x0);

        // Test hash selection mode
        control = Control(0);
        control.set_hash_select(HashSelect::NewHash);
        assert_eq!(control.hash_select(), HashSelect::NewHash);
        assert_eq!(control.0, 0x0);
        control.set_hash_select(HashSelect::AccumulateLastHash);
        assert_eq!(control.hash_select(), HashSelect::AccumulateLastHash);
        assert_eq!(control.0, 0x40);

        // Test interrupt related functions
        control = Control(0);
        assert!(!control.is_interrupt());
        control.0 |= Control::INTERRUPT;
        assert!(control.is_interrupt());
        assert_eq!(control.0, 0x100);
        control.clear_interrupt();
        assert_eq!(control.0, 0x300);
        control.set_interrupt();
        assert_eq!(control.0, 0x700);

        assert!(!control.is_interrupt_mask_enabled());
        control.enable_interrupt_mask();
        assert!(control.is_interrupt_mask_enabled());
        assert_eq!(control.0, 0xf00);
        control.disable_interrupt_mask();
        assert!(!control.is_interrupt_mask_enabled());
        assert_eq!(control.0, 0x700);

        // Test link mode
        control = Control(0);
        assert!(!control.is_link_mode_enabled());
        control.enable_link_mode();
        assert!(control.is_link_mode_enabled());
        assert_eq!(control.0, 0x8000);
        control.disable_link_mode();
        assert!(!control.is_link_mode_enabled());
        assert_eq!(control.0, 0x0);

        // Test message length setting
        control = Control(0);
        control.set_message_length(123);
        assert_eq!(control.message_length(), 123);
        assert_eq!(control.0, 0x7b0000);
    }

    #[test]
    fn struct_endianness_functions() {
        let mut endianness = Endianness(0);

        // Test setting and getting data output endianness
        endianness.set_data_out_endian(Endian::Little);
        assert_eq!(endianness.data_out_endian(), Endian::Little);
        assert_eq!(endianness.0, 0x0);

        endianness.set_data_out_endian(Endian::Big);
        assert_eq!(endianness.data_out_endian(), Endian::Big);
        assert_eq!(endianness.0, 0x1);
    }

    #[test]
    fn struct_control_protection_functions() {
        let mut control_protection = ControlProtection(0);

        // Test ID0 access right control
        control_protection.enable_id0_access_right();
        assert!(control_protection.is_id0_access_right_enabled());
        assert_eq!(control_protection.0, 0x2);

        control_protection.disable_id0_access_right();
        assert!(!control_protection.is_id0_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);

        // Test ID1 access right control
        control_protection = ControlProtection(0);
        control_protection.enable_id1_access_right();
        assert!(control_protection.is_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x4);

        control_protection.disable_id1_access_right();
        assert!(!control_protection.is_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
    }
}
