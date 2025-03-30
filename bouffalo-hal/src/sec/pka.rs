//! PKA (Public Key Accelerator) hardware driver.
//!
//! This module provides an interface to the PKA hardware peripheral.
//! It allows configuring and controlling public key cryptographic operations.

use crate::sec::Endian;
use volatile_register::RW;

/// PKA hardware registers block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 0.
    pub control_0: RW<Control0>,
    _reserved0: [u8; 8],
    /// Seed register.
    pub seed: RW<u32>,
    /// Control register 1.
    pub control_1: RW<Control1>,
    _reserved1: [u8; 44],
    /// single write for command.
    pub rw: RW<u32>,
    _reserved2: [u8; 28],
    /// burst write for data.
    pub rw_burst: RW<u32>,
    _reserved3: [u8; 152],
    /// Control protection register.
    pub control_protection: RW<ControlProtection>,
}

/// Control register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control0(u32);

impl Control0 {
    // Register bit definitions
    const DONE: u32 = 1 << 0;
    const DONE_CLEAR: u32 = 1 << 1;

    const BUSY: u32 = 1 << 2;
    const ENABLE: u32 = 1 << 3;
    const PROTECTION_MODE: u32 = 0xf << 4;
    const INTERRUPT: u32 = 1 << 8;
    const INTERRUPT_CLEAR: u32 = 1 << 9;
    const INTERRUPT_SET: u32 = 1 << 10;
    const INTERRUPT_MASK: u32 = 1 << 11;
    const ENDIAN: u32 = 1 << 12;
    const RAM_CLEAR_MODE: u32 = 1 << 13;
    const STATUS_CLEAR: u32 = 1 << 15;
    const STATUS: u32 = 0xffff << 16;
    const STATUS_OFFSET: u32 = 16;
    /// Check if PKA operation is done.
    #[inline]
    pub fn is_done(&self) -> bool {
        (self.0 & Self::DONE) != 0
    }

    /// Clear the done flag.
    #[inline]
    pub fn clear_done(&mut self) {
        self.0 |= Self::DONE_CLEAR;
    }
    /// Check if PKA engine is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }

    /// Enable PKA engine.
    #[inline]
    pub fn enable(&mut self) {
        self.0 |= Self::ENABLE;
    }

    /// Disable PKA engine.
    #[inline]
    pub fn disable(&mut self) {
        self.0 &= !Self::ENABLE;
    }

    /// Check if PKA engine is enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }
    /// Set protection mode.
    #[inline]
    pub fn set_protection_mode(&mut self, mode: u32) {
        self.0 &= !Self::PROTECTION_MODE;
        self.0 |= (mode & 0xf) << 4;
    }

    /// Get protection mode.
    #[inline]
    pub fn protection_mode(&self) -> u32 {
        (self.0 & Self::PROTECTION_MODE) >> 4
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
    /// Set the endianness of the PKA operation.
    #[inline]
    pub fn set_endian(&mut self, endian: Endian) {
        match endian {
            Endian::Little => self.0 &= !Self::ENDIAN,
            Endian::Big => self.0 |= Self::ENDIAN,
        }
    }

    /// Get the current endianness of the PKA operation.
    #[inline]
    pub fn endian(&self) -> Endian {
        if (self.0 & Self::ENDIAN) != 0 {
            Endian::Big
        } else {
            Endian::Little
        }
    }

    /// Set the RAM clear mode.
    #[inline]
    pub fn set_ram_clr_mode(&mut self, mode: bool) {
        if mode {
            self.0 |= Self::RAM_CLEAR_MODE;
        } else {
            self.0 &= !Self::RAM_CLEAR_MODE;
        }
    }

    /// Get the current RAM clear mode.
    #[inline]
    pub fn ram_clear_mode(&self) -> bool {
        (self.0 & Self::RAM_CLEAR_MODE) != 0
    }
    /// Clear the status register.
    #[inline]
    pub fn clear_status(&mut self) {
        self.0 |= Self::STATUS_CLEAR;
    }

    /// Get the current status.
    #[inline]
    pub fn status(&self) -> u32 {
        (self.0 & Self::STATUS) >> Self::STATUS_OFFSET
    }
}

/// Defines different burst modes for data transfer in PKA operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurstMode {
    /// Single transfer mode.
    Single = 0,
    /// Incrementing burst mode.
    Incr = 1,
    /// 4-beat wrap burst mode.
    Beat4Wrap = 2,
    /// 4-beat incrementing burst mode.
    Beat4Incr = 3,
    /// 8-beat wrap burst mode.
    Beat8Wrap = 4,
    /// 8-beat incrementing burst mode.
    Beat8Incr = 5,
}

/// Control register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control1(u32);

impl Control1 {
    // Register bit definitions.
    const BURST_MODE: u32 = 0x7 << 0;
    const BYPASS_ENABLE: u32 = 1 << 3;

    /// Set the burst mode for PKA operations.
    #[inline]
    pub fn set_burst_mode(&mut self, mode: BurstMode) {
        self.0 &= !Self::BURST_MODE;
        self.0 |= (mode as u32) & Self::BURST_MODE;
    }

    /// Get the current burst mode.
    #[inline]
    pub fn burst_mode(&self) -> BurstMode {
        match self.0 & Self::BURST_MODE {
            0 => BurstMode::Single,
            1 => BurstMode::Incr,
            2 => BurstMode::Beat4Wrap,
            3 => BurstMode::Beat4Incr,
            4 => BurstMode::Beat8Wrap,
            5 => BurstMode::Beat8Incr,
            _ => BurstMode::Beat8Incr, // Default to Single mode for unexpected values
        }
    }

    /// Enable bypass mode.
    #[inline]
    pub fn enable_bypass(&mut self) {
        self.0 |= Self::BYPASS_ENABLE;
    }

    /// Disable bypass mode.
    #[inline]
    pub fn disable_bypass(&mut self) {
        self.0 &= !Self::BYPASS_ENABLE;
    }

    /// Check if bypass mode is enabled.
    #[inline]
    pub fn is_bypass_enabled(&self) -> bool {
        (self.0 & Self::BYPASS_ENABLE) != 0
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
        assert_eq!(offset_of!(RegisterBlock, control_0), 0x00);
        assert_eq!(offset_of!(RegisterBlock, seed), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, control_1), 0x10);
        assert_eq!(offset_of!(RegisterBlock, rw), 0x40);
        assert_eq!(offset_of!(RegisterBlock, rw_burst), 0x60);
        assert_eq!(offset_of!(RegisterBlock, control_protection), 0xFC);
    }
    #[test]
    fn struct_control_0_functions() {
        let mut control_0 = Control0(0);

        // Test done flag
        assert!(!control_0.is_done());
        control_0.0 |= Control0::DONE;
        assert!(control_0.is_done());
        assert_eq!(control_0.0, 0x1);

        // Test clear done flag
        control_0.clear_done();
        assert_eq!(control_0.0 & Control0::DONE_CLEAR, Control0::DONE_CLEAR);
        assert_eq!(control_0.0, 0x3);

        // Test busy flag
        assert!(!control_0.is_busy());
        control_0.0 |= Control0::BUSY;
        assert!(control_0.is_busy());
        assert_eq!(control_0.0, 0x7);

        // Test enable/disable
        control_0 = Control0(0);
        assert!(!control_0.is_enabled());
        control_0.enable();
        assert!(control_0.is_enabled());
        assert_eq!(control_0.0, 0x8);
        control_0.disable();
        assert!(!control_0.is_enabled());
        assert_eq!(control_0.0, 0x0);

        // Test protection mode
        control_0 = Control0(0);
        control_0.set_protection_mode(5);
        assert_eq!(control_0.protection_mode(), 5);
        assert_eq!(control_0.0, 0x50);

        // Test interrupt
        control_0 = Control0(0);
        assert!(!control_0.is_interrupt());
        control_0 = Control0(0x100);
        assert!(control_0.is_interrupt());
        assert_eq!(control_0.0, 0x100);

        control_0.clear_interrupt();
        assert_eq!(control_0.0, 0x300);

        control_0 = Control0(0);
        assert!(!control_0.is_interrupt_mask_enabled());
        control_0.enable_interrupt_mask();
        assert!(control_0.is_interrupt_mask_enabled());
        assert_eq!(control_0.0, 0x800);

        control_0.disable_interrupt_mask();
        assert!(!control_0.is_interrupt_mask_enabled());
        assert_eq!(control_0.0, 0x0);

        // Test endianness
        control_0 = Control0(0);
        control_0.set_endian(Endian::Little);
        assert_eq!(control_0.endian(), Endian::Little);
        assert_eq!(control_0.0, 0x0);
        control_0.set_endian(Endian::Big);
        assert_eq!(control_0.endian(), Endian::Big);
        assert_eq!(control_0.0, 0x1000);

        // Test RAM clear mode
        control_0 = Control0(0);
        control_0.set_ram_clr_mode(true);
        assert!(control_0.ram_clear_mode());
        assert_eq!(control_0.0, 0x2000);
        control_0.set_ram_clr_mode(false);
        assert!(!control_0.ram_clear_mode());
        assert_eq!(control_0.0, 0x0);

        // Test status
        control_0 = Control0(0x12340000);
        assert_eq!(control_0.status(), 0x1234);
        assert_eq!(control_0.0, 0x12340000);
    }

    #[test]
    fn struct_control_1_functions() {
        let mut control_1 = Control1(0);

        // Test burst mode setting and getting
        control_1.set_burst_mode(BurstMode::Single);
        assert_eq!(control_1.burst_mode(), BurstMode::Single);
        assert_eq!(control_1.0, 0x0);

        control_1.set_burst_mode(BurstMode::Incr);
        assert_eq!(control_1.burst_mode(), BurstMode::Incr);
        assert_eq!(control_1.0, 0x1);

        control_1.set_burst_mode(BurstMode::Beat4Wrap);
        assert_eq!(control_1.burst_mode(), BurstMode::Beat4Wrap);
        assert_eq!(control_1.0, 0x2);

        control_1.set_burst_mode(BurstMode::Beat4Incr);
        assert_eq!(control_1.burst_mode(), BurstMode::Beat4Incr);
        assert_eq!(control_1.0, 0x3);

        control_1.set_burst_mode(BurstMode::Beat8Wrap);
        assert_eq!(control_1.burst_mode(), BurstMode::Beat8Wrap);
        assert_eq!(control_1.0, 0x4);

        control_1.set_burst_mode(BurstMode::Beat8Incr);
        assert_eq!(control_1.burst_mode(), BurstMode::Beat8Incr);
        assert_eq!(control_1.0, 0x5);

        // Test bypass enable and disable
        control_1 = Control1(0);
        assert!(!control_1.is_bypass_enabled());
        assert_eq!(control_1.0, 0x0);
        control_1.enable_bypass();
        assert!(control_1.is_bypass_enabled());
        assert_eq!(control_1.0, 0x8);
        control_1.disable_bypass();
        assert!(!control_1.is_bypass_enabled());
        assert_eq!(control_1.0, 0x0);
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
