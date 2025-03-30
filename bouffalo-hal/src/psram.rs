//! Pseudo Static Random Access Memory.

use core::ptr;

use crate::glb;
use volatile_register::RW;

/// Pseudo Static Random Access Memory registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Basic configuration register.
    pub basic_config: RW<u32>,
    _reserved0: [u8; 0x1C],
    /// Psram configuration register.
    pub psram_config: RW<u32>,
    _reserved1: [u8; 0xDC],
    /// Phy configuration register.
    pub phy_config: [RW<u32>; 21],
}

/// Initializes the PSRAM.
#[inline]
pub fn init_psram(psram: &RegisterBlock, glb: &glb::v2::RegisterBlock) {
    unsafe {
        glb.ldo12uhs_config
            .modify(|w| w.power_up().set_output_voltage(6));

        // configuration value reference: https://github.com/bouffalolab/bouffalo_sdk/blob/master/drivers/soc/bl808/std/src/bl808_psram_uhs.c
        psram.basic_config.write(0xB03F0403);
        psram.psram_config.write(0x00000023);

        psram.phy_config[0].write(0x60290200);
        psram.phy_config[1].write(0x50205020);
        psram.phy_config[2].write(0x50025002);
        psram.phy_config[3].write(0x50025002);
        psram.phy_config[4].write(0x50025002);
        psram.phy_config[5].write(0x50025002);
        psram.phy_config[6].write(0x50025002);
        psram.phy_config[7].write(0x50025002);
        psram.phy_config[8].write(0x50025002);
        psram.phy_config[9].write(0x50025002);
        psram.phy_config[10].write(0x34000000);
        psram.phy_config[11].write(0x34000006);
        psram.phy_config[12].write(0x0F271222);
        psram.phy_config[13].write(0x09020303);
        psram.phy_config[14].write(0x050E0418);
        psram.phy_config[15].write(0x0A6A1C1C);
        psram.phy_config[16].write(0xA2FF0000);
        psram.phy_config[17].write(0x07110710);
        psram.phy_config[18].write(0x00208A08);
        psram.phy_config[19].write(0x00000000);
        psram.phy_config[20].write(0x01334433);

        ptr::write_volatile(0x200007E8 as *mut u32, 0x32000); // TODO: fix magic and hardcode
    }
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, basic_config), 0x0);
        assert_eq!(offset_of!(RegisterBlock, psram_config), 0x20);
        assert_eq!(offset_of!(RegisterBlock, phy_config), 0x100);
    }
}
