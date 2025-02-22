//! AES (Advanced Encryption Standard) hardware accelerator driver.
//!
//! This module provides an interface to the AES hardware acceleration unit.
//! It supports various AES modes including 128-bit, 192-bit, and 256-bit key sizes,
//! as well as ECB, CBC, CTR and XTS block cipher modes.

use crate::sec::Endian;
use volatile_register::{RO, RW};

/// AES hardware registers block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register for configuring AES operations.
    control: RW<Control>,
    /// Source address for input message data.
    message_source_address: RW<u32>,
    /// Destination address for output cipher/plain data.
    message_destination_address: RW<u32>,
    /// Status register indicating operation state.
    status: RO<u32>,
    /// Initial vector registers (big endian).
    initial_vector: [RW<u32>; 4],
    /// AES key registers (big endian).
    /// Stores the encryption/decryption key.
    key: [RW<u32>; 8],
    /// Key selection register 0.
    key_select_0: RW<u32>,
    /// Key selection register 1.
    key_select_1: RW<u32>,
    /// Endianness configuration register.
    endianness: RW<Endianness>,
    /// Secure boot configuration register.
    secure_boot: RW<SecureBoot>,
    /// Link mode configuration address (word aligned).
    link_config_address: RW<u32>,
    _reserved: [u8; 168],
    /// Access control protection register.
    control_protection: RW<ControlProtection>,
}

/// AES operation modes.
/// Defines the different key sizes and modes supported by the AES engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AesMode {
    /// Standard AES with 128-bit key.
    Aes128 = 0,
    /// Standard AES with 256-bit key.
    Aes256 = 1,
    /// Standard AES with 192-bit key.
    Aes192 = 2,
    /// AES-128 with double key mode.
    Aes128DoubleKey = 3,
}

/// Decryption key selection mode.
/// Controls whether to use a new key or reuse the previous key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecKeySelect {
    /// Use a new decryption key.
    NewKey = 0,
    /// Reuse the previous decryption key.
    SameKeyAsLastOne = 1,
}

/// AES block cipher operation modes.
/// Defines how the AES algorithm processes blocks of data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockMode {
    /// Electronic Codebook mode.
    ECB = 0,
    /// Counter mode.
    CTR = 1,
    /// Cipher Block Chaining mode.
    CBC = 2,
    /// XEX-based tweaked-codebook mode with ciphertext stealing.
    XTS = 3,
}

/// Initialization vector selection mode.
/// Controls whether to use a new IV or reuse the previous IV.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IvSelect {
    /// Use a new initialization vector.
    NewIv = 0,
    /// Reuse the previous initialization vector.
    SameIvAsLastOne = 1,
}

/// AES control register.
/// Contains configuration bits for controlling AES operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);
impl Control {
    const BUSY: u32 = 1 << 0;
    const TRIGGER: u32 = 1 << 1;
    const ENABLE: u32 = 1 << 2;
    const MODE: u32 = 0x3 << 3;
    const DEC_ENABLE: u32 = 1 << 5;
    const DEC_KEY_SELECT: u32 = 1 << 6;
    const HW_KEY_ENABLE: u32 = 1 << 7;
    const INTERRUPT: u32 = 1 << 8;
    const INTERRUPT_CLEAR: u32 = 1 << 9;
    const INTERRUPT_SET: u32 = 1 << 10;
    const INTERRUPT_MASK: u32 = 1 << 11;
    const BLOCK_MODE: u32 = 0x3 << 12;
    const IV_SELECT: u32 = 1 << 14;
    const LINK_MODE: u32 = 1 << 15;
    const MESSAGE_LENGTH: u32 = 0xffff << 16;

    /// Check if AES engine is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }

    /// Trigger AES operation.
    #[inline]
    pub fn trigger(&mut self) {
        self.0 |= Self::TRIGGER;
    }

    /// Enable AES engine.
    #[inline]
    pub fn enable(&mut self) {
        self.0 |= Self::ENABLE;
    }

    /// Disable AES engine.
    #[inline]
    pub fn disable(&mut self) {
        self.0 &= !Self::ENABLE;
    }

    /// Check if AES engine is enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }

    /// Set AES operation mode.
    #[inline]
    pub fn set_aes_mode(&mut self, mode: AesMode) {
        self.0 &= !Self::MODE;
        self.0 |= ((mode as u32) << 3) & Self::MODE;
    }

    /// Get current AES operation mode.
    #[inline]
    pub fn aes_mode(&self) -> AesMode {
        match (self.0 & Self::MODE) >> 3 {
            0 => AesMode::Aes128,
            1 => AesMode::Aes256,
            2 => AesMode::Aes192,
            3 => AesMode::Aes128DoubleKey,
            _ => AesMode::Aes128, // Default fallback
        }
    }

    /// Enable decryption mode.
    #[inline]
    pub fn enable_dec(&mut self) {
        self.0 |= Self::DEC_ENABLE;
    }

    /// Disable decryption mode.
    #[inline]
    pub fn disable_dec(&mut self) {
        self.0 &= !Self::DEC_ENABLE;
    }

    /// Check if decryption mode is enabled.
    #[inline]
    pub fn is_dec_enabled(&self) -> bool {
        (self.0 & Self::DEC_ENABLE) != 0
    }

    /// Set decryption key selection.
    #[inline]
    pub fn set_dec_key_select(&mut self, dec_key: DecKeySelect) {
        match dec_key {
            DecKeySelect::NewKey => self.0 &= !Self::DEC_KEY_SELECT,
            DecKeySelect::SameKeyAsLastOne => self.0 |= Self::DEC_KEY_SELECT,
        }
    }

    /// Get current decryption key selection.
    #[inline]
    pub fn dec_key_select(&self) -> DecKeySelect {
        if (self.0 & Self::DEC_KEY_SELECT) != 0 {
            DecKeySelect::SameKeyAsLastOne
        } else {
            DecKeySelect::NewKey
        }
    }

    /// Enable hardware key.
    #[inline]
    pub fn enable_hw_key(&mut self) {
        self.0 |= Self::HW_KEY_ENABLE;
    }

    /// Disable hardware key.
    #[inline]
    pub fn disable_hw_key(&mut self) {
        self.0 &= !Self::HW_KEY_ENABLE;
    }

    /// Check if hardware key is enabled.
    #[inline]
    pub fn is_hw_key_enabled(&self) -> bool {
        (self.0 & Self::HW_KEY_ENABLE) != 0
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

    /// Set block operation mode.
    #[inline]
    pub fn set_block_mode(&mut self, block_mode: BlockMode) {
        self.0 &= !Self::BLOCK_MODE;
        self.0 |= ((block_mode as u32) << 12) & Self::BLOCK_MODE;
    }

    /// Get current block operation mode.
    #[inline]
    pub fn block_mode(&self) -> BlockMode {
        match (self.0 & Self::BLOCK_MODE) >> 12 {
            0 => BlockMode::ECB,
            1 => BlockMode::CTR,
            2 => BlockMode::CBC,
            3 => BlockMode::XTS,
            _ => BlockMode::ECB, // Default fallback
        }
    }

    /// Set IV selection mode.
    #[inline]
    pub fn set_iv_select(&mut self, iv_select: IvSelect) {
        match iv_select {
            IvSelect::NewIv => self.0 &= !Self::IV_SELECT,
            IvSelect::SameIvAsLastOne => self.0 |= Self::IV_SELECT,
        }
    }

    /// Get current IV selection mode.
    #[inline]
    pub fn iv_select(&self) -> IvSelect {
        if (self.0 & Self::IV_SELECT) != 0 {
            IvSelect::SameIvAsLastOne
        } else {
            IvSelect::NewIv
        }
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

    /// Set message length in bytes.
    #[inline]
    pub fn set_message_length(&mut self, message_length: u32) {
        self.0 &= !Self::MESSAGE_LENGTH;
        self.0 |= (message_length << 16) & Self::MESSAGE_LENGTH;
    }

    /// Get message length in bytes.
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
    const INPUT_DATA_ENDIAN: u32 = 1 << 1;
    const KEY_ENDIAN: u32 = 1 << 2;
    const INITIAL_VECTOR_ENDIAN: u32 = 1 << 3;
    const TWEAK_ENDIAN: u32 = 1 << 4;
    const COUNTER_LENGTH: u32 = 0x3 << 30;

    /// Set endianness for output data.
    #[inline]
    pub fn set_output_data_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::OUTPUT_DATA_ENDIAN,
            Endian::Big => self.0 |= Self::OUTPUT_DATA_ENDIAN,
        }
    }

    /// Get endianness of output data.
    #[inline]
    pub fn output_data_endian(&self) -> Endian {
        if (self.0 & Self::OUTPUT_DATA_ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    /// Set endianness for input data.
    #[inline]
    pub fn set_input_data_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::INPUT_DATA_ENDIAN,
            Endian::Big => self.0 |= Self::INPUT_DATA_ENDIAN,
        }
    }

    /// Get endianness of input data.
    #[inline]
    pub fn input_data_endian(&self) -> Endian {
        if (self.0 & Self::INPUT_DATA_ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    /// Set endianness for key.
    #[inline]
    pub fn set_key_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::KEY_ENDIAN,
            Endian::Big => self.0 |= Self::KEY_ENDIAN,
        }
    }

    /// Get endianness of key.
    #[inline]
    pub fn key_endian(&self) -> Endian {
        if (self.0 & Self::KEY_ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    /// Set endianness for initialization vector.
    #[inline]
    pub fn set_iv_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::INITIAL_VECTOR_ENDIAN,
            Endian::Big => self.0 |= Self::INITIAL_VECTOR_ENDIAN,
        }
    }

    /// Get endianness of initialization vector.
    #[inline]
    pub fn iv_endian(&self) -> Endian {
        if (self.0 & Self::INITIAL_VECTOR_ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    /// Set endianness for XTS tweak value.
    #[inline]
    pub fn set_tweak_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::TWEAK_ENDIAN,
            Endian::Big => self.0 |= Self::TWEAK_ENDIAN,
        }
    }

    /// Get endianness of XTS tweak value.
    #[inline]
    pub fn tweak_endian(&self) -> Endian {
        if (self.0 & Self::TWEAK_ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    /// Set counter length for CTR mode.
    #[inline]
    pub fn set_counter_length(&mut self, len: u32) {
        self.0 &= !Self::COUNTER_LENGTH;
        self.0 |= (len << 30) & Self::COUNTER_LENGTH;
    }

    /// Get counter length for CTR mode.
    #[inline]
    pub fn counter_length(&self) -> u32 {
        (self.0 & Self::COUNTER_LENGTH) >> 30
    }
}

/// XTS (XEX-based tweaked-codebook mode with ciphertext stealing) operation modes.
///
/// Defines the operation mode for XTS encryption/decryption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XtsMode {
    /// Normal XTS mode with multiple data units (value = 0).
    Normal = 0,
    /// XTS mode processing single data unit only (value = 1).
    SingleUnit = 1,
}

/// Secure boot configuration structure.
///
/// Contains settings for secure boot operations including key selection
/// and XTS mode configuration. Internally represented as a 32-bit value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SecureBoot(u32);

impl SecureBoot {
    const SECURE_BOOT_KEY_SELECT: u32 = 1 << 0;
    const XTS_MODE: u32 = 1 << 15;
    const XTS_DATA_UINT_LENGTH: u32 = 0xffff << 16;

    /// Set secure boot key selection.
    #[inline]
    pub fn set_secure_boot_key_select(&mut self, sboot_key_select: bool) {
        if sboot_key_select {
            self.0 |= Self::SECURE_BOOT_KEY_SELECT;
        } else {
            self.0 &= !Self::SECURE_BOOT_KEY_SELECT;
        }
    }

    /// Get current secure boot key selection.
    #[inline]
    pub fn secure_boot_key_select(&self) -> bool {
        (self.0 & Self::SECURE_BOOT_KEY_SELECT) != 0
    }

    /// Set XTS operation mode.
    #[inline]
    pub fn set_xts_mode(&mut self, xts_mode: XtsMode) {
        match xts_mode {
            XtsMode::Normal => self.0 &= !Self::XTS_MODE,
            XtsMode::SingleUnit => self.0 |= Self::XTS_MODE,
        }
    }

    /// Get current XTS operation mode.
    #[inline]
    pub fn xts_mode(&self) -> XtsMode {
        if (self.0 & Self::XTS_MODE) != 0 {
            XtsMode::SingleUnit
        } else {
            XtsMode::Normal
        }
    }

    /// Set XTS data unit length (in bytes).
    #[inline]
    pub fn set_xts_data_uint_length(&mut self, len: u32) {
        self.0 &= !Self::XTS_DATA_UINT_LENGTH;
        self.0 |= (len << 16) & Self::XTS_DATA_UINT_LENGTH;
    }

    /// Get current XTS data unit length (in bytes).
    #[inline]
    pub fn xts_data_uint_length(&self) -> u32 {
        (self.0 & Self::XTS_DATA_UINT_LENGTH) >> 16
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
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x00);
        assert_eq!(offset_of!(RegisterBlock, message_source_address), 0x04);
        assert_eq!(offset_of!(RegisterBlock, message_destination_address), 0x08);
        assert_eq!(offset_of!(RegisterBlock, status), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, initial_vector), 0x10);
        assert_eq!(offset_of!(RegisterBlock, key), 0x20);
        assert_eq!(offset_of!(RegisterBlock, key_select_0), 0x40);
        assert_eq!(offset_of!(RegisterBlock, key_select_1), 0x44);
        assert_eq!(offset_of!(RegisterBlock, endianness), 0x48);
        assert_eq!(offset_of!(RegisterBlock, secure_boot), 0x4C);
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

        // Test AES mode setting and getting
        control = Control(0);
        control.set_aes_mode(AesMode::Aes128);
        assert_eq!(control.aes_mode(), AesMode::Aes128);
        assert_eq!(control.0, 0x0);

        control.set_aes_mode(AesMode::Aes192);
        assert_eq!(control.aes_mode(), AesMode::Aes192);
        assert_eq!(control.0, 0x10);

        control.set_aes_mode(AesMode::Aes256);
        assert_eq!(control.aes_mode(), AesMode::Aes256);
        assert_eq!(control.0, 0x8);

        // Test enable/disable functionality
        control = Control(0);
        assert!(!control.is_enabled());
        control.enable();
        assert!(control.is_enabled());
        assert_eq!(control.0, 0x4);
        control.disable();
        assert!(!control.is_enabled());
        assert_eq!(control.0, 0x0);

        // Test decryption mode related functions
        control = Control(0);
        assert!(!control.is_dec_enabled());
        control.enable_dec();
        assert!(control.is_dec_enabled());
        assert_eq!(control.0, 0x20);
        control.disable_dec();
        assert!(!control.is_dec_enabled());
        assert_eq!(control.0, 0x0);

        // Test decryption key selection related functions
        control = Control(0);
        control.set_dec_key_select(DecKeySelect::NewKey);
        assert_eq!(control.dec_key_select(), DecKeySelect::NewKey);
        assert_eq!(control.0, 0x0);
        control.set_dec_key_select(DecKeySelect::SameKeyAsLastOne);
        assert_eq!(control.dec_key_select(), DecKeySelect::SameKeyAsLastOne);
        assert_eq!(control.0, 0x40);

        // Test hardware key related functions
        control = Control(0);
        assert!(!control.is_hw_key_enabled());
        control.enable_hw_key();
        assert!(control.is_hw_key_enabled());
        assert_eq!(control.0, 0x80);
        control.disable_hw_key();
        assert!(!control.is_hw_key_enabled());
        assert_eq!(control.0, 0x0);

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

        // Test block operation mode related functions
        control = Control(0);
        control.set_block_mode(BlockMode::ECB);
        assert_eq!(control.block_mode(), BlockMode::ECB);
        assert_eq!(control.0, 0x0);
        control.set_block_mode(BlockMode::CBC);
        assert_eq!(control.block_mode(), BlockMode::CBC);
        assert_eq!(control.0, 0x2000);

        // Test IV selection mode related functions
        control = Control(0);
        control.set_iv_select(IvSelect::NewIv);
        assert_eq!(control.iv_select(), IvSelect::NewIv);
        assert_eq!(control.0, 0x0);
        control.set_iv_select(IvSelect::SameIvAsLastOne);
        assert_eq!(control.iv_select(), IvSelect::SameIvAsLastOne);
        assert_eq!(control.0, 0x4000);

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

        // Test setting and getting data input endianness
        endianness.set_input_data_endian(Endian::Little);
        assert_eq!(endianness.input_data_endian(), Endian::Little);
        assert_eq!(endianness.0, 0x0);

        endianness.set_input_data_endian(Endian::Big);
        assert_eq!(endianness.input_data_endian(), Endian::Big);
        assert_eq!(endianness.0, 0x2);

        // Test setting and getting data output endianness
        endianness = Endianness(0);
        endianness.set_output_data_endian(Endian::Little);
        assert_eq!(endianness.output_data_endian(), Endian::Little);
        assert_eq!(endianness.0, 0x0);

        endianness.set_output_data_endian(Endian::Big);
        assert_eq!(endianness.output_data_endian(), Endian::Big);
        assert_eq!(endianness.0, 0x1);

        // Test setting and getting key endianness
        endianness = Endianness(0);
        endianness.set_key_endian(Endian::Little);
        assert_eq!(endianness.key_endian(), Endian::Little);
        assert_eq!(endianness.0, 0x0);

        endianness.set_key_endian(Endian::Big);
        assert_eq!(endianness.key_endian(), Endian::Big);
        assert_eq!(endianness.0, 0x4);

        // Test setting and getting IV endianness
        endianness = Endianness(0);
        endianness.set_iv_endian(Endian::Little);
        assert_eq!(endianness.iv_endian(), Endian::Little);
        assert_eq!(endianness.0, 0x0);

        endianness.set_iv_endian(Endian::Big);
        assert_eq!(endianness.iv_endian(), Endian::Big);
        assert_eq!(endianness.0, 0x8);
    }

    #[test]
    fn struct_secure_boot_functions() {
        let mut secure_boot = SecureBoot(0);

        // Test XTS mode setting and getting
        secure_boot.set_xts_mode(XtsMode::Normal);
        assert_eq!(secure_boot.xts_mode(), XtsMode::Normal);
        assert_eq!(secure_boot.0, 0x0);

        secure_boot.set_xts_mode(XtsMode::SingleUnit);
        assert_eq!(secure_boot.xts_mode(), XtsMode::SingleUnit);
        assert_eq!(secure_boot.0, 0x8000);

        // Test XTS data unit length setting and getting
        secure_boot = SecureBoot(0);
        secure_boot.set_xts_data_uint_length(1024);
        assert_eq!(secure_boot.xts_data_uint_length(), 1024);
        assert_eq!(secure_boot.0, 1024 << 16);

        // Test combination of settings
        secure_boot = SecureBoot(0);
        secure_boot.set_xts_mode(XtsMode::Normal);
        secure_boot.set_xts_data_uint_length(2048);
        assert_eq!(secure_boot.xts_mode(), XtsMode::Normal);
        assert_eq!(secure_boot.xts_data_uint_length(), 2048);
        assert_eq!(secure_boot.0, 2048 << 16);
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
