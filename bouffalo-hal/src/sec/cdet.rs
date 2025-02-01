//! CDET (Clock Detection) hardware accelerator driver.
//!
//! This module provides an interface to the CDET hardware peripheral.
//! It allows configuring and controlling the clock detection functionality.

use volatile_register::RW;

/// CDET hardware registers block
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 0
    pub control_0: RW<u32>,
    /// Control register 1
    pub control_1: RW<u32>,
    /// Control protection register
    pub control_protection: RW<ControlProtection>,
}

/// Control protection register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ControlProtection(u32);

impl ControlProtection {
    const PROTECTION_ENABLE: u32 = 1 << 0;
    const ENABLE_ID0_ACCESS_RIGHT: u32 = 1 << 1;
    const ENABLE_ID1_ACCESS_RIGHT: u32 = 1 << 2;

    /// Enable protection
    #[inline]
    pub fn enable_protection(&mut self) {
        self.0 |= Self::PROTECTION_ENABLE;
    }

    /// Disable protection
    #[inline]
    pub fn disable_protection(&mut self) {
        self.0 &= !Self::PROTECTION_ENABLE;
    }

    /// Check if protection is enabled
    #[inline]
    pub fn is_protection_enabled(&self) -> bool {
        (self.0 & Self::PROTECTION_ENABLE) != 0
    }

    /// Enable ID0 access right
    #[inline]
    pub fn enable_id0_access_right(&mut self) {
        self.0 |= Self::ENABLE_ID0_ACCESS_RIGHT;
    }

    /// Disable ID0 access right
    #[inline]
    pub fn disable_id0_access_right(&mut self) {
        self.0 &= !Self::ENABLE_ID0_ACCESS_RIGHT;
    }

    /// Enable ID1 access right
    #[inline]
    pub fn enable_id1_access_right(&mut self) {
        self.0 |= Self::ENABLE_ID1_ACCESS_RIGHT;
    }

    /// Disable ID1 access right
    #[inline]
    pub fn disable_id1_access_right(&mut self) {
        self.0 &= !Self::ENABLE_ID1_ACCESS_RIGHT;
    }

    /// Check if ID0 access right is enabled
    #[inline]
    pub fn is_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if ID1 access right is enabled
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
        assert_eq!(offset_of!(RegisterBlock, control_0), 0x00);
        assert_eq!(offset_of!(RegisterBlock, control_1), 0x04);
        assert_eq!(offset_of!(RegisterBlock, control_protection), 0x08);
    }

    #[test]
    fn struct_control_protection_functions() {
        let mut control_protection = ControlProtection(0x0);

        // Test enable_protection function
        assert!(!control_protection.is_protection_enabled());
        control_protection.enable_protection();
        assert!(control_protection.is_protection_enabled());
        assert_eq!(control_protection.0, 0x01);

        // Test disable_protection function
        control_protection = ControlProtection(0x0);
        control_protection.disable_protection();
        assert!(!control_protection.is_protection_enabled());
        assert_eq!(control_protection.0, 0x0);

        // Test ID0 access right control
        control_protection = ControlProtection(0x0);
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
