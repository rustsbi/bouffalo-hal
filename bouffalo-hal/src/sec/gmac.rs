//! GMAC (Galois Message Authentication Code) hardware accelerator driver.
//!
//! This module provides access to the GMAC hardware accelerator peripheral,
//! supporting message authentication using the Galois field multiplication.

use volatile_register::RW;

/// GMAC hardware registers block
#[repr(C)]
pub struct RegisterBlock {
    /// Control register
    pub control: RW<Control>,
    /// GMAC link configuration address (word aligned)
    pub link_config_address: RW<u32>,
    /// Status register
    pub status: RW<u32>,
    /// Control protection register
    pub control_protection: RW<ControlProtection>,
}

/// Endianness configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little = 0,
    Big = 1,
}

/// Control register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    // Register bit definitions
    const BUSY: u32 = 1 << 0;
    const TRIGGER: u32 = 1 << 1;
    const ENABLE: u32 = 1 << 2;
    const INTERRUPT: u32 = 1 << 8;
    const INTERRUPT_CLEAR: u32 = 1 << 9;
    const INTERRUPT_SET: u32 = 1 << 10;
    const INTERRUPT_MASK: u32 = 1 << 11;
    const T_ENDIAN: u32 = 1 << 12;
    const T_ENDIAN_SHIFT: u32 = 12;
    const H_ENDIAN: u32 = 1 << 13;
    const H_ENDIAN_SHIFT: u32 = 13;
    const X_ENDIAN: u32 = 1 << 14;
    const X_ENDIAN_SHIFT: u32 = 14;

    /// Check if SHA engine is busy
    #[inline]
    pub fn is_busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }

    /// Trigger SHA operation
    #[inline]
    pub fn trigger(&mut self) {
        self.0 |= Self::TRIGGER;
    }

    /// Enable SHA engine
    #[inline]
    pub fn enable(&mut self) {
        self.0 |= Self::ENABLE;
    }

    /// Disable SHA engine
    #[inline]
    pub fn disable(&mut self) {
        self.0 &= !Self::ENABLE;
    }

    /// Check if SHA engine is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }
    /// Check interrupt status
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        (self.0 & Self::INTERRUPT) != 0
    }

    /// Clear interrupt flag
    #[inline]
    pub fn clear_interrupt(&mut self) {
        self.0 |= Self::INTERRUPT_CLEAR;
    }

    /// Set interrupt flag
    #[inline]
    pub fn set_interrupt(&mut self) {
        self.0 |= Self::INTERRUPT_SET;
    }
    /// Enable interrupt mask
    #[inline]
    pub fn enable_interrupt_mask(&mut self) {
        self.0 |= Self::INTERRUPT_MASK;
    }

    /// Disable interrupt mask
    #[inline]
    pub fn disable_interrupt_mask(&mut self) {
        self.0 &= !Self::INTERRUPT_MASK;
    }

    /// Check if interrupt mask is enabled
    #[inline]
    pub fn is_interrupt_mask_enabled(&self) -> bool {
        (self.0 & Self::INTERRUPT_MASK) != 0
    }
    /// Set the endianness for T value
    #[inline]
    pub fn set_t_endian(&mut self, endian: Endian) {
        self.0 = (self.0 & !Self::T_ENDIAN) | ((endian as u32) << Self::T_ENDIAN_SHIFT);
    }

    /// Get the endianness for T value
    #[inline]
    pub fn get_t_endian(&self) -> Endian {
        match (self.0 & Self::T_ENDIAN) >> Self::T_ENDIAN_SHIFT {
            0 => Endian::Little,
            1 => Endian::Big,
            _ => Endian::Little,
        }
    }
    /// Set the endianness for H value
    #[inline]
    pub fn set_h_endian(&mut self, endian: Endian) {
        self.0 = (self.0 & !Self::H_ENDIAN) | ((endian as u32) << Self::H_ENDIAN_SHIFT);
    }

    /// Get the endianness for H value
    #[inline]
    pub fn get_h_endian(&self) -> Endian {
        match (self.0 & Self::H_ENDIAN) >> Self::H_ENDIAN_SHIFT {
            0 => Endian::Little,
            1 => Endian::Big,
            _ => Endian::Little,
        }
    }
    /// Set the endianness for X value
    #[inline]
    pub fn set_x_endian(&mut self, endian: Endian) {
        self.0 = (self.0 & !Self::X_ENDIAN) | ((endian as u32) << Self::X_ENDIAN_SHIFT);
    }

    /// Get the endianness for X value
    #[inline]
    pub fn get_x_endian(&self) -> Endian {
        match (self.0 & Self::X_ENDIAN) >> Self::X_ENDIAN_SHIFT {
            0 => Endian::Little,
            1 => Endian::Big,
            _ => Endian::Little,
        }
    }
}

/// Control protection register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ControlProtection(u32);

impl ControlProtection {
    const ENABLE_ID0_ACCESS_RIGHT: u32 = 1 << 1;
    const ENABLE_ID1_ACCESS_RIGHT: u32 = 1 << 2;

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
        assert_eq!(offset_of!(RegisterBlock, control), 0x00);
        assert_eq!(offset_of!(RegisterBlock, link_config_address), 0x04);
        assert_eq!(offset_of!(RegisterBlock, status), 0x08);
        assert_eq!(offset_of!(RegisterBlock, control_protection), 0x0C);
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

        // Test enable/disable functionality
        control = Control(0);
        assert!(!control.is_enabled());
        control.enable();
        assert!(control.is_enabled());
        assert_eq!(control.0, 0x4);
        control.disable();
        assert!(!control.is_enabled());
        assert_eq!(control.0, 0x0);

        // Test interrupt related functions
        control = Control(0);
        assert!(!control.is_interrupt());
        control.0 |= Control::INTERRUPT;
        assert!(control.is_interrupt());
        assert_eq!(control.0, 0x100);

        // Test interrupt mask functions
        control = Control(0);
        assert!(!control.is_interrupt_mask_enabled());
        control.enable_interrupt_mask();
        assert!(control.is_interrupt_mask_enabled());
        assert_eq!(control.0, 0x800);
        control.disable_interrupt_mask();
        assert!(!control.is_interrupt_mask_enabled());
        assert_eq!(control.0, 0x0);

        // Test t_endian functions
        control = Control(0);
        assert_eq!(control.get_t_endian(), Endian::Little);
        control.set_t_endian(Endian::Big);
        assert_eq!(control.get_t_endian(), Endian::Big);
        assert_eq!(control.0, 0x1000);

        // Test h_endian functions
        control = Control(0);
        assert_eq!(control.get_h_endian(), Endian::Little);
        control.set_h_endian(Endian::Big);
        assert_eq!(control.get_h_endian(), Endian::Big);
        assert_eq!(control.0, 0x2000);

        // Test x_endian functions
        control = Control(0);
        assert_eq!(control.get_x_endian(), Endian::Little);
        control.set_x_endian(Endian::Big);
        assert_eq!(control.get_x_endian(), Endian::Big);
        assert_eq!(control.0, 0x4000);
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
