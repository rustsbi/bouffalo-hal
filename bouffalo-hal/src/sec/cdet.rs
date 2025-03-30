//! CDET (Clock Detection) hardware accelerator driver.
//!
//! This module provides an interface to the CDET hardware peripheral.
//! It allows configuring and controlling the clock detection functionality.

use volatile_register::RW;

/// CDET hardware registers block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 0.
    pub control_0: RW<Control0>,
    /// Control register 1.
    pub control_1: RW<Control1>,
    /// Control register 2.
    pub control_2: RW<Control2>,
    /// Control register 3.
    pub control_3: RW<Control3>,
    _reserved: [u8; 236],
    /// Control protection register.
    pub control_protection: RW<ControlProtection>,
}

/// Control register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control0(u32);

impl Control0 {
    const ENABLE: u32 = 1 << 0;
    const BUSY: u32 = 1 << 1;
    const STATUS: u32 = 0x1f << 3;
    const STATUS_OFFSET: u32 = 3;
    const INTERRUPT: u32 = 1 << 8;
    const INTERRUPT_CLEAR: u32 = 1 << 9;
    const INTERRUPT_SET: u32 = 1 << 10;
    const INTERRUPT_MASK: u32 = 1 << 11;
    const MODE: u32 = 1 << 12;

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
    /// Check if PKA engine is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }
    /// Get the current status.
    #[inline]
    pub fn get_status(&self) -> u32 {
        (self.0 & Self::STATUS) >> Self::STATUS_OFFSET
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

    /// Set the mode of the CDET.
    #[inline]
    pub fn set_mode(&mut self, mode: bool) {
        if mode {
            self.0 |= Self::MODE;
        } else {
            self.0 &= !Self::MODE;
        }
    }

    /// Get the current mode of the CDET.
    #[inline]
    pub fn mode(&self) -> bool {
        (self.0 & Self::MODE) != 0
    }
}

/// Control register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control1(u32);

impl Control1 {
    const G_LOOP_MAX: u32 = 0xffff << 0;
    const G_LOOP_MIN: u32 = 0xffff << 16;
    /// Set the maximum value for G loop.
    #[inline]
    pub fn set_g_loop_max(&mut self, val: u16) {
        self.0 = (self.0 & !Self::G_LOOP_MAX) | ((val as u32) << 0);
    }

    /// Get the maximum value for G loop.
    #[inline]
    pub fn g_loop_max(&self) -> u16 {
        ((self.0 & Self::G_LOOP_MAX) >> 0) as u16
    }

    /// Set the minimum value for G loop.
    #[inline]
    pub fn set_g_loop_min(&mut self, val: u16) {
        self.0 = (self.0 & !Self::G_LOOP_MIN) | ((val as u32) << 16);
    }

    /// Get the minimum value for G loop.
    #[inline]
    pub fn g_loop_min(&self) -> u16 {
        ((self.0 & Self::G_LOOP_MIN) >> 16) as u16
    }
}

/// Control register 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control2(u32);

impl Control2 {
    const T_LOOP_N: u32 = 0xffff << 0;
    const T_DLY_N: u32 = 0xff << 16;
    const G_SLP_N: u32 = 0xff << 24;
    /// Set the T loop N value.
    #[inline]
    pub fn set_t_loop_n(&mut self, val: u16) {
        self.0 = (self.0 & !Self::T_LOOP_N) | ((val as u32) << 0);
    }

    /// Get the T loop N value.
    #[inline]
    pub fn t_loop_n(&self) -> u16 {
        (self.0 & Self::T_LOOP_N) as u16
    }

    /// Set the T delay N value.
    #[inline]
    pub fn set_t_dly_n(&mut self, val: u8) {
        self.0 = (self.0 & !Self::T_DLY_N) | ((val as u32) << 16);
    }

    /// Get the T delay N value.
    #[inline]
    pub fn t_dly_n(&self) -> u8 {
        ((self.0 & Self::T_DLY_N) >> 16) as u8
    }

    /// Set the G sleep N value.
    #[inline]
    pub fn set_g_slp_n(&mut self, val: u8) {
        self.0 = (self.0 & !Self::G_SLP_N) | ((val as u32) << 24);
    }

    /// Get the G sleep N value.
    #[inline]
    pub fn g_slp_n(&self) -> u8 {
        ((self.0 & Self::G_SLP_N) >> 24) as u8
    }
}
/// Control register 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control3(u32);

impl Control3 {
    const T_COUNT: u32 = 0xffff << 0;
    const G_COUNT: u32 = 0xffff << 16;

    /// Set the T count value.
    #[inline]
    pub fn set_t_count(&mut self, val: u16) {
        self.0 = (self.0 & !Self::T_COUNT) | ((val as u32) << 0);
    }

    /// Get the T count value.
    #[inline]
    pub fn t_count(&self) -> u16 {
        (self.0 & Self::T_COUNT) as u16
    }

    /// Set the G count value.
    #[inline]
    pub fn set_g_count(&mut self, val: u16) {
        self.0 = (self.0 & !Self::G_COUNT) | ((val as u32) << 16);
    }

    /// Get the G count value.
    #[inline]
    pub fn g_count(&self) -> u16 {
        ((self.0 & Self::G_COUNT) >> 16) as u16
    }
}

/// Control protection register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ControlProtection(u32);

impl ControlProtection {
    const PROTECTION_ENABLE: u32 = 1 << 0;
    const ENABLE_ID0_ACCESS_RIGHT: u32 = 1 << 1;
    const ENABLE_ID1_ACCESS_RIGHT: u32 = 1 << 2;

    /// Enable protection.
    #[inline]
    pub fn enable_protection(&mut self) {
        self.0 |= Self::PROTECTION_ENABLE;
    }

    /// Disable protection.
    #[inline]
    pub fn disable_protection(&mut self) {
        self.0 &= !Self::PROTECTION_ENABLE;
    }

    /// Check if protection is enabled.
    #[inline]
    pub fn is_protection_enabled(&self) -> bool {
        (self.0 & Self::PROTECTION_ENABLE) != 0
    }

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
        assert_eq!(offset_of!(RegisterBlock, control_1), 0x04);
        assert_eq!(offset_of!(RegisterBlock, control_2), 0x08);
        assert_eq!(offset_of!(RegisterBlock, control_3), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, control_protection), 0xFC);
    }
    #[test]
    fn struct_control_0_functions() {
        let mut control_0 = Control0(0);

        // Test enable and disable functions
        assert!(!control_0.is_enabled());
        control_0.enable();
        assert!(control_0.is_enabled());
        assert_eq!(control_0.0, 0x1);
        control_0.disable();
        assert!(!control_0.is_enabled());
        assert_eq!(control_0.0, 0x0);

        // Test busy flag
        control_0 = Control0(0);
        control_0.0 = Control0::BUSY;
        assert!(control_0.is_busy());
        assert_eq!(control_0.0, 0x2);
        control_0.0 = 0x0;
        assert!(!control_0.is_busy());

        // Test status
        control_0 = Control0(0);
        control_0.0 = 0x78; // 1111000 in binary
        assert_eq!(control_0.get_status(), 0xF);

        // Test interrupt functions
        control_0 = Control0(0);
        control_0.0 = Control0::INTERRUPT;
        assert!(control_0.is_interrupt());
        assert_eq!(control_0.0, 0x100);
        control_0.clear_interrupt();
        assert_eq!(control_0.0, Control0::INTERRUPT | Control0::INTERRUPT_CLEAR);
        assert_eq!(control_0.0, 0x300);
        control_0.0 = 0x0;
        control_0.set_interrupt();
        assert_eq!(control_0.0, Control0::INTERRUPT_SET);
        assert_eq!(control_0.0, 0x400);

        // Test interrupt mask functions
        control_0 = Control0(0);
        control_0.0 = 0x0;
        assert!(!control_0.is_interrupt_mask_enabled());
        control_0.enable_interrupt_mask();
        assert!(control_0.is_interrupt_mask_enabled());
        assert_eq!(control_0.0, 0x800);
        control_0.disable_interrupt_mask();
        assert!(!control_0.is_interrupt_mask_enabled());
        assert_eq!(control_0.0, 0x0);

        // Test mode functions
        control_0 = Control0(0);
        control_0.0 = 0x0;
        assert!(!control_0.mode());
        control_0.set_mode(true);
        assert!(control_0.mode());
        assert_eq!(control_0.0, 0x1000);
        control_0.set_mode(false);
        assert!(!control_0.mode());
        assert_eq!(control_0.0, 0x0);
    }
    #[test]
    fn struct_control_1_functions() {
        let mut control_1 = Control1(0);

        // Test setting and getting G loop max value
        control_1.set_g_loop_max(0x04D2);
        assert_eq!(control_1.g_loop_max(), 0x04D2);
        assert_eq!(control_1.0, 0x000004D2);

        // Test setting and getting G loop min value
        control_1 = Control1(0);
        control_1.set_g_loop_min(0x162E);
        assert_eq!(control_1.g_loop_min(), 0x162E);
        assert_eq!(control_1.0, 0x162E0000);

        // Test setting both max and min values
        control_1 = Control1(0);
        control_1.set_g_loop_max(0x270F);
        control_1.set_g_loop_min(0x0457);
        assert_eq!(control_1.g_loop_max(), 0x270F);
        assert_eq!(control_1.g_loop_min(), 0x0457);
        assert_eq!(control_1.0, 0x0457270F);

        // Test that max and min values don't interfere with each other
        control_1 = Control1(0);
        control_1.set_g_loop_max(0xFFFF);
        assert_eq!(control_1.g_loop_max(), 0xFFFF);
        assert_eq!(control_1.g_loop_min(), 0x0000);
        assert_eq!(control_1.0, 0x0000FFFF);

        control_1.set_g_loop_min(0xFFFF);
        assert_eq!(control_1.g_loop_max(), 0xFFFF);
        assert_eq!(control_1.g_loop_min(), 0xFFFF);
        assert_eq!(control_1.0, 0xFFFFFFFF);

        // Test setting values to 0
        control_1 = Control1(0);
        control_1.set_g_loop_max(0);
        control_1.set_g_loop_min(0);
        assert_eq!(control_1.g_loop_max(), 0);
        assert_eq!(control_1.g_loop_min(), 0);
        assert_eq!(control_1.0, 0x00000000);
    }
    #[test]
    fn struct_control_2_functions() {
        let mut control_2 = Control2(0);

        // Test setting and getting T loop N value
        control_2.set_t_loop_n(1234);
        assert_eq!(control_2.t_loop_n(), 1234);
        assert_eq!(control_2.0, 0x000004D2); // Assert the hexadecimal value

        // Test setting and getting T delay N value
        control_2 = Control2(0);
        control_2.set_t_dly_n(56);
        assert_eq!(control_2.t_dly_n(), 56);
        assert_eq!(control_2.0, 0x00380000); // Assert the hexadecimal value

        // Test setting and getting G sleep N value
        control_2 = Control2(0);
        control_2.set_g_slp_n(78);
        assert_eq!(control_2.g_slp_n(), 78);
        assert_eq!(control_2.0, 0x4E000000); // Assert the hexadecimal value

        // Test setting all values together
        control_2 = Control2(0);
        control_2.set_t_loop_n(0xFFFF);
        control_2.set_t_dly_n(0xFF);
        control_2.set_g_slp_n(0xFF);
        assert_eq!(control_2.t_loop_n(), 0xFFFF);
        assert_eq!(control_2.t_dly_n(), 0xFF);
        assert_eq!(control_2.g_slp_n(), 0xFF);
        assert_eq!(control_2.0, 0xFFFFFFFF); // Assert the hexadecimal value

        // Test that values don't interfere with each other
        control_2 = Control2(0);
        control_2.set_t_loop_n(0x1234);
        assert_eq!(control_2.t_loop_n(), 0x1234);
        assert_eq!(control_2.t_dly_n(), 0);
        assert_eq!(control_2.g_slp_n(), 0);
        assert_eq!(control_2.0, 0x00001234); // Assert the hexadecimal value

        // Test setting values to 0
        control_2 = Control2(0);
        control_2.set_t_loop_n(0);
        control_2.set_t_dly_n(0);
        control_2.set_g_slp_n(0);
        assert_eq!(control_2.t_loop_n(), 0);
        assert_eq!(control_2.t_dly_n(), 0);
        assert_eq!(control_2.g_slp_n(), 0);
        assert_eq!(control_2.0, 0x00000000); // Assert the hexadecimal value
    }
    #[test]
    fn struct_control_3_functions() {
        let mut control_3 = Control3(0);

        // Test setting and getting T count value
        control_3.set_t_count(1234);
        assert_eq!(control_3.t_count(), 1234);
        assert_eq!(control_3.0, 0x000004D2); // Assert the hexadecimal value

        // Test setting and getting G count value
        control_3 = Control3(0);
        control_3.set_g_count(5678);
        assert_eq!(control_3.g_count(), 5678);
        assert_eq!(control_3.0, 0x162E0000); // Assert the hexadecimal value

        // Test setting all values together
        control_3 = Control3(0);
        control_3.set_t_count(0xFFFF);
        control_3.set_g_count(0xFFFF);
        assert_eq!(control_3.t_count(), 0xFFFF);
        assert_eq!(control_3.g_count(), 0xFFFF);
        assert_eq!(control_3.0, 0xFFFFFFFF); // Assert the hexadecimal value

        // Test that values don't interfere with each other
        control_3 = Control3(0);
        control_3.set_t_count(0x1234);
        assert_eq!(control_3.t_count(), 0x1234);
        assert_eq!(control_3.g_count(), 0);
        assert_eq!(control_3.0, 0x00001234); // Assert the hexadecimal value

        // Test setting values to 0
        control_3 = Control3(0);
        control_3.set_t_count(0);
        control_3.set_g_count(0);
        assert_eq!(control_3.t_count(), 0);
        assert_eq!(control_3.g_count(), 0);
        assert_eq!(control_3.0, 0x00000000); // Assert the hexadecimal value
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
