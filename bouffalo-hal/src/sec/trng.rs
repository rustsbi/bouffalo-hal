//! TRNG (True Random Number Generator) hardware accelerator driver.
//!
//! This module provides an interface to the TRNG hardware peripheral.
//! It allows generating true random numbers and configuring the TRNG.

use volatile_register::{RO, RW};
/// TRNG hardware registers block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register 0.
    pub control_0: RW<Control0>,
    /// Status register.
    pub status: RW<u32>,
    /// Random value output (256 bits).
    pub output_data: [RO<u32>; 8],
    /// Test configuration.
    pub test: RW<Test>,
    /// Control register 1.
    pub control_1: RW<Control1>,
    /// Control register 2.
    pub control_2: RW<Control2>,
    /// Control register 3.
    pub control_3: RW<Control3>,
    _reserved0: [u8; 8],
    /// Test output data.
    pub test_output: [RW<u32>; 4],
    _reserved1: [u8; 172],
    /// Control protection register.
    pub control_protection: RW<ControlProtection>,
}

/// Manual function selection for TRNG operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManualFunctionSelect {
    /// Instantiate the internal state.
    InstantiateState = 0,
    /// Generate random data.
    GenerateState = 1,
}

/// Control register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control0(u32);

impl Control0 {
    // Register bit definitions.
    const BUSY: u32 = 1 << 0;
    const TRIGGER: u32 = 1 << 1;
    const ENABLE: u32 = 1 << 2;
    const OUTPUT_DATA_CLEAR: u32 = 1 << 3;
    const HEALTH_TEST_ERROR: u32 = 1 << 4;
    const INTERRUPT: u32 = 1 << 8;
    const INTERRUPT_CLEAR: u32 = 1 << 9;
    const INTERRUPT_SET: u32 = 1 << 10;
    const INTERRUPT_MASK: u32 = 1 << 11;
    const MANUAL_FUNCTION_SELECT: u32 = 1 << 13;
    const MANUAL_RESEED: u32 = 1 << 14;
    const MANUAL_ENABLE: u32 = 1 << 15;

    /// Check if TRNG engine is busy.
    #[inline]
    pub fn is_busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }

    /// Trigger TRNG operation.
    #[inline]
    pub fn trigger(&mut self) {
        self.0 |= Self::TRIGGER;
    }

    /// Enable TRNG engine.
    #[inline]
    pub fn enable(&mut self) {
        self.0 |= Self::ENABLE;
    }

    /// Disable TRNG engine.
    #[inline]
    pub fn disable(&mut self) {
        self.0 &= !Self::ENABLE;
    }

    /// Check if TRNG engine is enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }
    /// Clear TRNG output data.
    #[inline]
    pub fn clear_output_data(&mut self) {
        self.0 |= Self::OUTPUT_DATA_CLEAR;
    }

    /// Get health test error.
    #[inline]
    pub fn health_test_error(&self) -> u32 {
        (self.0 & Self::HEALTH_TEST_ERROR) >> 4
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

    /// Set the manual function select mode.
    #[inline]
    pub fn set_manual_function_select(&mut self, value: ManualFunctionSelect) {
        self.0 = (self.0 & !(Self::MANUAL_FUNCTION_SELECT)) | ((value as u32) << 13);
    }

    /// Get the current manual function select mode.
    #[inline]
    pub fn manual_function_select(&self) -> ManualFunctionSelect {
        if (self.0 & Self::MANUAL_FUNCTION_SELECT) != 0 {
            ManualFunctionSelect::GenerateState
        } else {
            ManualFunctionSelect::InstantiateState
        }
    }

    /// Enable manual reseed.
    #[inline]
    pub fn enable_manual_reseed(&mut self) {
        self.0 |= Self::MANUAL_RESEED;
    }

    /// Disable manual reseed.
    #[inline]
    pub fn disable_manual_reseed(&mut self) {
        self.0 &= !Self::MANUAL_RESEED;
    }

    /// Check if manual reseed is enabled.
    #[inline]
    pub fn is_manual_reseed_enabled(&self) -> bool {
        (self.0 & Self::MANUAL_RESEED) != 0
    }

    /// Enable manual mode.
    #[inline]
    pub fn enable_manual(&mut self) {
        self.0 |= Self::MANUAL_ENABLE;
    }

    /// Disable manual mode.
    #[inline]
    pub fn disable_manual(&mut self) {
        self.0 &= !Self::MANUAL_ENABLE;
    }

    /// Check if manual mode is enabled.
    #[inline]
    pub fn is_manual_enabled(&self) -> bool {
        (self.0 & Self::MANUAL_ENABLE) != 0
    }
}

/// Represents the test configuration for the TRNG (True Random Number Generator).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Test(u32);

impl Test {
    const TEST_ENABLE: u32 = 1 << 0;
    const CONDITIONAL_COMPONENT_TEST_ENABLE: u32 = 1 << 1;
    const CONDITIONAL_COMPONENT_BYPASS: u32 = 1 << 2;
    const HEALTH_TEST_DISABLE: u32 = 1 << 3;
    const HEALTH_TEST_ALARM_NUMBER: u32 = 0xff << 4;
    /// Enable test mode.
    #[inline]
    pub fn enable_test(&mut self) {
        self.0 |= Self::TEST_ENABLE;
    }

    /// Disable test mode.
    #[inline]
    pub fn disable_test(&mut self) {
        self.0 &= !Self::TEST_ENABLE;
    }

    /// Check if test mode is enabled.
    #[inline]
    pub fn is_test_enabled(&self) -> bool {
        (self.0 & Self::TEST_ENABLE) != 0
    }

    /// Enable conditional component test.
    #[inline]
    pub fn enable_conditional_component_test(&mut self) {
        self.0 |= Self::CONDITIONAL_COMPONENT_TEST_ENABLE;
    }

    /// Disable conditional component test.
    #[inline]
    pub fn disable_conditional_component_test(&mut self) {
        self.0 &= !Self::CONDITIONAL_COMPONENT_TEST_ENABLE;
    }

    /// Check if conditional component test is enabled.
    #[inline]
    pub fn is_conditional_component_test_enabled(&self) -> bool {
        (self.0 & Self::CONDITIONAL_COMPONENT_TEST_ENABLE) != 0
    }

    /// Enable conditional component bypass.
    #[inline]
    pub fn enable_conditional_component_bypass(&mut self) {
        self.0 |= Self::CONDITIONAL_COMPONENT_BYPASS;
    }

    /// Disable conditional component bypass.
    #[inline]
    pub fn disable_conditional_component_bypass(&mut self) {
        self.0 &= !Self::CONDITIONAL_COMPONENT_BYPASS;
    }

    /// Check if conditional component bypass is enabled.
    #[inline]
    pub fn is_conditional_component_bypass_enabled(&self) -> bool {
        (self.0 & Self::CONDITIONAL_COMPONENT_BYPASS) != 0
    }

    /// Enable health test.
    #[inline]
    pub fn enable_health_test(&mut self) {
        self.0 &= !Self::HEALTH_TEST_DISABLE;
    }

    /// Disable health test.
    #[inline]
    pub fn disable_health_test(&mut self) {
        self.0 |= Self::HEALTH_TEST_DISABLE;
    }

    /// Check if health test is enabled.
    #[inline]
    pub fn is_health_test_enabled(&self) -> bool {
        (self.0 & Self::HEALTH_TEST_DISABLE) == 0
    }

    /// Set health test alarm number.
    #[inline]
    pub fn set_health_test_alarm_number(&mut self, value: u8) {
        self.0 = (self.0 & !Self::HEALTH_TEST_ALARM_NUMBER) | ((value as u32) << 4);
    }

    /// Get health test alarm number.
    #[inline]
    pub fn health_test_alarm_number(&self) -> u8 {
        ((self.0 & Self::HEALTH_TEST_ALARM_NUMBER) >> 4) as u8
    }
}

/// Control register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control1(u32);

impl Control1 {
    const RESEED_NUMBER_LSB: u32 = 0xffffffff << 0;

    /// Set the least significant bits of the reseed number.
    #[inline]
    pub fn set_reseed_number_lsb(&mut self, value: u32) {
        self.0 = (self.0 & !Self::RESEED_NUMBER_LSB) | (value & Self::RESEED_NUMBER_LSB);
    }

    /// Get the least significant bits of the reseed number.
    #[inline]
    pub fn reseed_number_lsb(&self) -> u32 {
        self.0 & Self::RESEED_NUMBER_LSB
    }
}

/// Control register 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control2(u32);

impl Control2 {
    const RESEED_NUMBER_MSB: u32 = 0xffff << 0;

    /// Set the most significant bits of the reseed number.
    #[inline]
    pub fn set_reseed_number_msb(&mut self, value: u16) {
        // Clear the existing MSB and set the new value
        self.0 = (self.0 & !Self::RESEED_NUMBER_MSB) | ((value as u32) & Self::RESEED_NUMBER_MSB);
    }

    /// Get the most significant bits of the reseed number.
    #[inline]
    pub fn reseed_number_msb(&self) -> u16 {
        // Extract and return the MSB
        ((self.0 & Self::RESEED_NUMBER_MSB) >> 0) as u16
    }
}

/// Control register 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control3(u32);
impl Control3 {
    const CONDITIONAL_COMPONENT_COMPRESSION_RATION: u32 = 0xff << 0;
    const HT_RCT_CUT_OFF_VALUE: u32 = 0xff << 8;
    const HT_APT_CUT_OFF_VALUE: u32 = 0x3ff << 16;
    const HT_OD_TEST_ENABLE: u32 = 1 << 26;
    const ROSC_ENABLE: u32 = 1 << 31;

    /// Set the conditional component compression ratio.
    #[inline]
    pub fn set_conditional_component_compression_ration(&mut self, value: u8) {
        self.0 = (self.0 & !Self::CONDITIONAL_COMPONENT_COMPRESSION_RATION) | ((value as u32) << 0);
    }

    /// Get the conditional component compression ratio.
    #[inline]
    pub fn conditional_component_compression_ration(&self) -> u8 {
        ((self.0 & Self::CONDITIONAL_COMPONENT_COMPRESSION_RATION) >> 0) as u8
    }

    /// Set the HT RCT cut-off value.
    #[inline]
    pub fn set_ht_rct_cut_off_value(&mut self, value: u8) {
        self.0 = (self.0 & !Self::HT_RCT_CUT_OFF_VALUE) | ((value as u32) << 8);
    }

    /// Get the HT RCT cut-off value.
    #[inline]
    pub fn ht_rct_cut_off_value(&self) -> u8 {
        ((self.0 & Self::HT_RCT_CUT_OFF_VALUE) >> 8) as u8
    }

    /// Set the HT APT cut-off value.
    #[inline]
    pub fn set_ht_apt_cut_off_value(&mut self, value: u16) {
        self.0 = (self.0 & !Self::HT_APT_CUT_OFF_VALUE) | ((value as u32) << 16);
    }

    /// Get the HT APT cut-off value.
    #[inline]
    pub fn ht_apt_cut_off_value(&self) -> u16 {
        ((self.0 & Self::HT_APT_CUT_OFF_VALUE) >> 16) as u16
    }

    /// Enable HT OD test.
    #[inline]
    pub fn enable_ht_od_test(&mut self) {
        self.0 |= Self::HT_OD_TEST_ENABLE;
    }

    /// Disable HT OD test.
    #[inline]
    pub fn disable_ht_od_test(&mut self) {
        self.0 &= !Self::HT_OD_TEST_ENABLE;
    }

    /// Check if HT OD test is enabled.
    #[inline]
    pub fn is_ht_od_test_enabled(&self) -> bool {
        (self.0 & Self::HT_OD_TEST_ENABLE) != 0
    }

    /// Enable ROSC.
    #[inline]
    pub fn enable_rosc(&mut self) {
        self.0 |= Self::ROSC_ENABLE;
    }

    /// Disable ROSC.
    #[inline]
    pub fn disable_rosc(&mut self) {
        self.0 &= !Self::ROSC_ENABLE;
    }

    /// Check if ROSC is enabled.
    #[inline]
    pub fn is_rosc_enabled(&self) -> bool {
        (self.0 & Self::ROSC_ENABLE) != 0
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
        assert_eq!(offset_of!(RegisterBlock, status), 0x04);
        assert_eq!(offset_of!(RegisterBlock, output_data), 0x08);
        assert_eq!(offset_of!(RegisterBlock, test), 0x28);
        assert_eq!(offset_of!(RegisterBlock, control_1), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, control_2), 0x30);
        assert_eq!(offset_of!(RegisterBlock, control_3), 0x34);
        assert_eq!(offset_of!(RegisterBlock, test_output), 0x40);
        assert_eq!(offset_of!(RegisterBlock, control_protection), 0xFC);
    }

    #[test]
    fn struct_control0_functions() {
        let mut control0 = Control0(0);

        // Test is_busy function
        assert_eq!(control0.is_busy(), false);
        control0.0 |= Control0::BUSY;
        assert_eq!(control0.is_busy(), true);
        assert_eq!(control0.0, 0x1);

        // Test trigger function
        control0 = Control0(0);
        control0.trigger();
        assert_eq!(control0.0, 0x2);

        // Test enable and disable functions
        control0 = Control0(0);
        control0.enable();
        assert_eq!(control0.is_enabled(), true);
        assert_eq!(control0.0, 0x4);
        control0.disable();
        assert_eq!(control0.is_enabled(), false);
        assert_eq!(control0.0, 0x0);

        // Test clear_output_data function
        control0 = Control0(0);
        control0.clear_output_data();
        assert_eq!(control0.0, 0x8);

        // Test health_test_error function
        control0 = Control0(0);
        assert_eq!(control0.health_test_error(), 0);
        assert_eq!(control0.0, 0x00);

        // Test interrupt related functions
        control0 = Control0(0);
        assert_eq!(control0.is_interrupt(), false);
        control0.set_interrupt();
        control0.clear_interrupt();
        assert_eq!(control0.0, 0x600);

        // Test interrupt mask functions
        control0 = Control0(0);
        control0.enable_interrupt_mask();
        assert_eq!(control0.is_interrupt_mask_enabled(), true);
        assert_eq!(control0.0, 0x800);
        control0.disable_interrupt_mask();
        assert_eq!(control0.is_interrupt_mask_enabled(), false);
        assert_eq!(control0.0, 0x0);

        // Test manual function select
        control0 = Control0(0);
        control0.set_manual_function_select(ManualFunctionSelect::GenerateState);
        assert_eq!(control0.0, 0x2000);
        control0.set_manual_function_select(ManualFunctionSelect::InstantiateState);
        assert_eq!(control0.0, 0x0);

        // Test manual reseed functions
        control0 = Control0(0);
        control0.enable_manual_reseed();
        assert_eq!(control0.is_manual_reseed_enabled(), true);
        assert_eq!(control0.0, 0x4000);
        control0.disable_manual_reseed();
        assert_eq!(control0.is_manual_reseed_enabled(), false);
        assert_eq!(control0.0, 0x0);

        // Test manual mode functions
        control0 = Control0(0);
        control0.enable_manual();
        assert_eq!(control0.is_manual_enabled(), true);
        assert_eq!(control0.0, 0x8000);
        control0.disable_manual();
        assert_eq!(control0.is_manual_enabled(), false);
        assert_eq!(control0.0, 0x0);
    }

    #[test]
    fn struct_test_functions() {
        let mut test = Test(0);

        // Test enable_test and is_test_enabled functions
        test.enable_test();
        assert!(test.is_test_enabled());
        assert_eq!(test.0, 0x1);
        test.disable_test();
        assert!(!test.is_test_enabled());
        assert_eq!(test.0, 0x0);

        // Test conditional component test functions
        test = Test(0);
        test.enable_conditional_component_test();
        assert!(test.is_conditional_component_test_enabled());
        assert_eq!(test.0, 0x2);
        test.disable_conditional_component_test();
        assert!(!test.is_conditional_component_test_enabled());
        assert_eq!(test.0, 0x0);

        // Test conditional component bypass functions
        test = Test(0);
        test.enable_conditional_component_bypass();
        assert!(test.is_conditional_component_bypass_enabled());
        assert_eq!(test.0, 0x4);
        test.disable_conditional_component_bypass();
        assert!(!test.is_conditional_component_bypass_enabled());
        assert_eq!(test.0, 0x0);

        // Test health test functions
        test = Test(0);
        test.disable_health_test();
        assert!(!test.is_health_test_enabled());
        assert_eq!(test.0, 0x8);
        test.enable_health_test();
        assert!(test.is_health_test_enabled());
        assert_eq!(test.0, 0x0);

        // Test health test alarm number functions
        test = Test(0);
        let alarm_number = 0x2A;
        test.set_health_test_alarm_number(alarm_number);
        assert_eq!(test.health_test_alarm_number(), alarm_number);
        assert_eq!(test.0, 0x2A0);
    }

    #[test]
    fn struct_control1_functions() {
        let mut control1 = Control1(0);

        // Test setting and getting reseed number LSB
        let test_value = 0xABCDEF12;
        control1.set_reseed_number_lsb(test_value);
        assert_eq!(control1.reseed_number_lsb(), test_value);
        assert_eq!(control1.0, 0xABCDEF12);

        // Test setting and getting reseed number LSB with max value
        control1 = Control1(0);
        let max_value = 0xFFFFFFFF;
        control1.set_reseed_number_lsb(max_value);
        assert_eq!(control1.reseed_number_lsb(), max_value);
        assert_eq!(control1.0, 0xFFFFFFFF);

        // Test setting and getting reseed number LSB with zero
        control1 = Control1(0);
        control1.set_reseed_number_lsb(0);
        assert_eq!(control1.reseed_number_lsb(), 0);
        assert_eq!(control1.0, 0x0);

        // Test that other bits are not affected
        control1 = Control1(0);
        let initial_value = 0xF0F0F0F0;
        control1.0 = initial_value;
        let test_value = 0x0A0A0A0A;
        control1.set_reseed_number_lsb(test_value);
        assert_eq!(control1.0, 0x0A0A0A0A);
    }

    #[test]
    fn struct_control2_functions() {
        let mut control2 = Control2(0);

        // Test setting and getting reseed number MSB
        let test_value = 0xABCD;
        control2.set_reseed_number_msb(test_value);
        assert_eq!(control2.reseed_number_msb(), test_value);
        assert_eq!(control2.0, 0xABCD);

        // Test setting and getting reseed number MSB with max value
        control2 = Control2(0);
        let max_value = 0xFFFF;
        control2.set_reseed_number_msb(max_value);
        assert_eq!(control2.reseed_number_msb(), max_value);
        assert_eq!(control2.0, 0xFFFF);

        // Test setting and getting reseed number MSB with zero
        control2 = Control2(0);
        control2.set_reseed_number_msb(0);
        assert_eq!(control2.reseed_number_msb(), 0);
        assert_eq!(control2.0, 0x0);

        // Test that other bits are not affected
        control2 = Control2(0);
        let initial_value = 0xF0F0F0F0;
        control2.0 = initial_value;
        let test_value = 0x0A0A;
        control2.set_reseed_number_msb(test_value);
        assert_eq!(control2.0, 0xF0F00A0A);
    }
    #[test]
    fn struct_control3_functions() {
        let mut control3 = Control3(0);

        // Test setting and getting conditional component compression ratio
        let test_ratio = 0xAA;
        control3.set_conditional_component_compression_ration(test_ratio);
        assert_eq!(control3.0, 0xAA);

        // Test setting and getting HT RCT cut-off value
        control3 = Control3(0);
        let test_rct = 0xBB;
        control3.set_ht_rct_cut_off_value(test_rct);
        assert_eq!(control3.ht_rct_cut_off_value(), test_rct);
        assert_eq!(control3.0, 0xBB00);

        // Test setting and getting HT APT cut-off value
        control3 = Control3(0);
        let test_apt = 0x3FF;
        control3.set_ht_apt_cut_off_value(test_apt);
        assert_eq!(control3.ht_apt_cut_off_value(), test_apt);
        assert_eq!(control3.0, 0x3FF0000);

        // Test enabling and disabling HT OD test
        control3 = Control3(0);
        control3.enable_ht_od_test();
        assert!(control3.is_ht_od_test_enabled());
        assert_eq!(control3.0, 0x4000000);
        control3.disable_ht_od_test();
        assert!(!control3.is_ht_od_test_enabled());
        assert_eq!(control3.0, 0x0);

        // Test enabling and disabling ROSC
        control3 = Control3(0);
        control3.enable_rosc();
        assert!(control3.is_rosc_enabled());
        assert_eq!(control3.0, 0x80000000);
        control3.disable_rosc();
        assert!(!control3.is_rosc_enabled());
        assert_eq!(control3.0, 0x0);

        // Test that other bits are not affected
        control3 = Control3(0);
        let initial_value = 0xF0F0F0F0;
        control3.0 = initial_value;
        let test_value = 0xAA;
        control3.set_conditional_component_compression_ration(test_value);
        assert_eq!(control3.0, 0xF0F0F0AA);
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
