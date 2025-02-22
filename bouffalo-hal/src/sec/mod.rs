//! SEC (Security Engine) hardware accelerator driver.
//!
//! This module provides access to the SEC hardware accelerator peripheral,
//! which includes SHA, AES, TRNG, PKA, CDET and GMAC functionality.

use volatile_register::{RO, RW};

/// Endianness configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little = 0,
    Big = 1,
}

pub mod aes;
pub mod cdet;
pub mod gmac;
pub mod pka;
pub mod sha;
pub mod trng;

pub type Aes = aes::RegisterBlock;
pub type Cdet = cdet::RegisterBlock;
pub type Gmac = gmac::RegisterBlock;
pub type Pka = pka::RegisterBlock;
pub type Sha = sha::RegisterBlock;
pub type Trng = trng::RegisterBlock;

/// SEC ENG hardware registers block
#[repr(C)]
pub struct RegisterBlock {
    /// Secure Hash Algorithm (SHA) registers
    pub sha: Sha,
    /// Advanced Encryption Standard (AES) registers
    pub aes: Aes,
    /// True Random Number Generator (TRNG) registers
    pub trng: Trng,
    /// Public Key Accelerator (PKA) registers
    pub pka: Pka,
    /// Clock Detection (CDET) registers
    pub cdet: Cdet,
    /// Galois Message Authentication Code (GMAC) registers
    pub gmac: Gmac,
    _reserved0: [u8; 2304],
    /// Control protection register for read access
    pub control_protection_rd: RO<ControlProtectionRd>,
    pub control_reserved_0: RW<u32>,
    pub control_reserved_1: RW<u32>,
    pub control_reserved_2: RW<u32>,
}

/// Control protection register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ControlProtectionRd(u32);

impl ControlProtectionRd {
    const ENABLE_SHA_ID0_ACCESS_RIGHT: u32 = 1 << 0;
    const ENABLE_SHA_ID1_ACCESS_RIGHT: u32 = 1 << 1;
    const ENABLE_AES_ID0_ACCESS_RIGHT: u32 = 1 << 2;
    const ENABLE_AES_ID1_ACCESS_RIGHT: u32 = 1 << 3;
    const ENABLE_TRNG_ID0_ACCESS_RIGHT: u32 = 1 << 4;
    const ENABLE_TRNG_ID1_ACCESS_RIGHT: u32 = 1 << 5;
    const ENABLE_PKA_ID0_ACCESS_RIGHT: u32 = 1 << 6;
    const ENABLE_PKA_ID1_ACCESS_RIGHT: u32 = 1 << 7;
    const ENABLE_CDET_ID0_ACCESS_RIGHT: u32 = 1 << 8;
    const ENABLE_CDET_ID1_ACCESS_RIGHT: u32 = 1 << 9;
    const ENABLE_GMAC_ID0_ACCESS_RIGHT: u32 = 1 << 10;
    const ENABLE_GMAC_ID1_ACCESS_RIGHT: u32 = 1 << 11;
    const AES_DEBUG_DISABLE: u32 = 1 << 31;

    /// Check if SHA ID0 access right is enabled
    #[inline]
    pub fn is_sha_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_SHA_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if SHA ID1 access right is enabled
    #[inline]
    pub fn is_sha_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_SHA_ID1_ACCESS_RIGHT) != 0
    }

    /// Check if AES ID0 access right is enabled
    #[inline]
    pub fn is_aes_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_AES_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if AES ID1 access right is enabled
    #[inline]
    pub fn is_aes_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_AES_ID1_ACCESS_RIGHT) != 0
    }

    /// Check if TRNG ID0 access right is enabled
    #[inline]
    pub fn is_trng_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_TRNG_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if TRNG ID1 access right is enabled
    #[inline]
    pub fn is_trng_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_TRNG_ID1_ACCESS_RIGHT) != 0
    }

    /// Check if PKA ID0 access right is enabled
    #[inline]
    pub fn is_pka_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_PKA_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if PKA ID1 access right is enabled
    #[inline]
    pub fn is_pka_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_PKA_ID1_ACCESS_RIGHT) != 0
    }

    /// Check if CDET ID0 access right is enabled
    #[inline]
    pub fn is_cdet_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_CDET_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if CDET ID1 access right is enabled
    #[inline]
    pub fn is_cdet_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_CDET_ID1_ACCESS_RIGHT) != 0
    }

    /// Check if GMAC ID0 access right is enabled
    #[inline]
    pub fn is_gmac_id0_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_GMAC_ID0_ACCESS_RIGHT) != 0
    }

    /// Check if GMAC ID1 access right is enabled
    #[inline]
    pub fn is_gmac_id1_access_right_enabled(&self) -> bool {
        (self.0 & Self::ENABLE_GMAC_ID1_ACCESS_RIGHT) != 0
    }

    /// Check if AES debug is disabled
    #[inline]
    pub fn is_aes_debug_enabled(&self) -> bool {
        (self.0 & Self::AES_DEBUG_DISABLE) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, sha), 0x000);
        assert_eq!(offset_of!(RegisterBlock, aes), 0x100);
        assert_eq!(offset_of!(RegisterBlock, trng), 0x200);
        assert_eq!(offset_of!(RegisterBlock, pka), 0x300);
        assert_eq!(offset_of!(RegisterBlock, cdet), 0x400);
        assert_eq!(offset_of!(RegisterBlock, gmac), 0x500);
        assert_eq!(offset_of!(RegisterBlock, control_protection_rd), 0xF00);
    }

    #[test]
    fn struct_control_protection_rd_functions() {
        let mut control_protection = ControlProtectionRd(0);

        // Test SHA access rights
        assert!(!control_protection.is_sha_id0_access_right_enabled());
        assert!(!control_protection.is_sha_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::ENABLE_SHA_ID0_ACCESS_RIGHT;
        assert!(control_protection.is_sha_id0_access_right_enabled());
        assert!(!control_protection.is_sha_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x1);
        control_protection.0 |= ControlProtectionRd::ENABLE_SHA_ID1_ACCESS_RIGHT;
        assert!(control_protection.is_sha_id0_access_right_enabled());
        assert!(control_protection.is_sha_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x3);

        // Test AES access rights
        control_protection = ControlProtectionRd(0);
        assert!(!control_protection.is_aes_id0_access_right_enabled());
        assert!(!control_protection.is_aes_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::ENABLE_AES_ID0_ACCESS_RIGHT;
        assert!(control_protection.is_aes_id0_access_right_enabled());
        assert!(!control_protection.is_aes_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x4);
        control_protection.0 |= ControlProtectionRd::ENABLE_AES_ID1_ACCESS_RIGHT;
        assert!(control_protection.is_aes_id0_access_right_enabled());
        assert!(control_protection.is_aes_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0xC);

        // Test TRNG access rights
        control_protection = ControlProtectionRd(0);
        assert!(!control_protection.is_trng_id0_access_right_enabled());
        assert!(!control_protection.is_trng_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::ENABLE_TRNG_ID0_ACCESS_RIGHT;
        assert!(control_protection.is_trng_id0_access_right_enabled());
        assert!(!control_protection.is_trng_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x10);
        control_protection.0 |= ControlProtectionRd::ENABLE_TRNG_ID1_ACCESS_RIGHT;
        assert!(control_protection.is_trng_id0_access_right_enabled());
        assert!(control_protection.is_trng_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x30);

        // Test PKA access rights
        control_protection = ControlProtectionRd(0);
        assert!(!control_protection.is_pka_id0_access_right_enabled());
        assert!(!control_protection.is_pka_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::ENABLE_PKA_ID0_ACCESS_RIGHT;
        assert!(control_protection.is_pka_id0_access_right_enabled());
        assert!(!control_protection.is_pka_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x40);
        control_protection.0 |= ControlProtectionRd::ENABLE_PKA_ID1_ACCESS_RIGHT;
        assert!(control_protection.is_pka_id0_access_right_enabled());
        assert!(control_protection.is_pka_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0xC0);

        // Test CDET access rights
        control_protection = ControlProtectionRd(0);
        assert!(!control_protection.is_cdet_id0_access_right_enabled());
        assert!(!control_protection.is_cdet_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::ENABLE_CDET_ID0_ACCESS_RIGHT;
        assert!(control_protection.is_cdet_id0_access_right_enabled());
        assert!(!control_protection.is_cdet_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x100);
        control_protection.0 |= ControlProtectionRd::ENABLE_CDET_ID1_ACCESS_RIGHT;
        assert!(control_protection.is_cdet_id0_access_right_enabled());
        assert!(control_protection.is_cdet_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x300);

        // Test GMAC access rights
        control_protection = ControlProtectionRd(0);
        assert!(!control_protection.is_gmac_id0_access_right_enabled());
        assert!(!control_protection.is_gmac_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::ENABLE_GMAC_ID0_ACCESS_RIGHT;
        assert!(control_protection.is_gmac_id0_access_right_enabled());
        assert!(!control_protection.is_gmac_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0x400);
        control_protection.0 |= ControlProtectionRd::ENABLE_GMAC_ID1_ACCESS_RIGHT;
        assert!(control_protection.is_gmac_id0_access_right_enabled());
        assert!(control_protection.is_gmac_id1_access_right_enabled());
        assert_eq!(control_protection.0, 0xC00);

        // Test AES debug
        control_protection = ControlProtectionRd(0);
        assert!(control_protection.is_aes_debug_enabled());
        assert_eq!(control_protection.0, 0x0);
        control_protection.0 |= ControlProtectionRd::AES_DEBUG_DISABLE;
        assert!(!control_protection.is_aes_debug_enabled());
        assert_eq!(control_protection.0, 0x80000000);
    }
}
