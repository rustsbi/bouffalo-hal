use volatile_register::{RO, RW};

/// Power Down Sleep peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// PDS Control Register.
    pub ctrl: RW<u32>,
    /// PDS Time1 Register.
    pub time1: RW<u32>,
    _reserved0: [u32; 0x1],
    /// PDS Interrupt Register.
    pub int: RW<u32>,
    /// PDS Control2 Register.
    pub ctrl2: RW<u32>,
    /// PDS Control3 Register.
    pub ctrl3: RW<u32>,
    /// PDS Control4 Register.
    pub ctrl4: RW<u32>,
    /// PDS Status Register.
    pub status: RO<u32>,
    /// PDS RAM1 Register.
    pub ram1: RW<u32>,
    /// PDS Control5 Register.
    pub ctrl5: RW<u32>,
    /// PDS RAM2 Register.
    pub ram2: RW<u32>,
    _reserved1: [u32; 0x1],
    /// PDS GPIO Input Set Register.
    pub gpio_input_set: RW<u32>,
    /// PDS GPIO Pull Down Set Register.
    pub gpio_pd_set: RW<u32>,
    _reserved2: [u32; 0x2],
    /// PDS GPIO Interrupt Register.
    pub gpio_int: RW<u32>,
    /// PDS GPIO Status Register.
    pub gpio_status: RW<u32>,
    _reserved3: [u32; 0x32],
    /// CPU Core Configuration 0 Register.
    pub cpu_core_cfg0: RW<u32>,
    /// CPU Core Configuration 1 Register.
    pub cpu_core_cfg1: RW<u32>,
    _reserved4: [u32; 0x5],
    /// CPU Core Configuration 7 Register.
    pub cpu_core_cfg7: RW<u32>,
    /// CPU Core Configuration 8 Register.
    pub cpu_core_cfg8: RW<u32>,
    /// CPU Core Configuration 9 Register.
    pub cpu_core_cfg9: RW<u32>,
    /// CPU Core Configuration 10 Register.
    pub cpu_core_cfg10: RW<u32>,
    _reserved5: [u32; 0x1],
    /// CPU Core Configuration 12 Register.
    pub cpu_core_cfg12: RW<u32>,
    /// CPU Core Configuration 13 Register.
    pub cpu_core_cfg13: RW<u32>,
    /// CPU Core Configuration 14 Register.
    pub cpu_core_cfg14: RW<u32>,
    /// TZC PDS Register.
    pub tzc_pds: RW<u32>,
    _reserved6: [u32; 0x6C],
    /// RC32M Control 0 Register.
    pub rc32m_ctrl0: RW<u32>,
    /// RC32M Control 1 Register.
    pub rc32m_ctrl1: RW<u32>,
    _reserved7: [u32; 0x3E],
    /// PU Reset Clock PLL Register.
    pub pu_rst_clkpll: RW<u32>,
    _reserved8: [u32; 0x3F],
    /// USB Control Register.
    pub usb_ctrl: RW<u32>,
    /// USB PHY Control Register.
    pub usb_phy_ctrl: RW<u32>,
    _reserved9: [u32; 0x13E],
    /// Touch 1 Register.
    pub touch1: RW<u32>,
    /// Touch 2 Register.
    pub touch2: RW<u32>,
    /// Touch 3 Register.
    pub touch3: RW<u32>,
    /// Touch Sleep Time Register.
    pub touch_sleep_time: RW<u32>,
    /// Touch Data Hysteresis Register.
    pub touch_data_hystersis: RW<u32>,
    /// Channel Force Data Registers.
    pub channel_force_data: [RW<u32>; 6],
    /// Channel VTH Data Registers.
    pub channel_vth_data: [RW<u32>; 3],
    /// Channel Raw Data Registers.
    pub channel_raw_data: [RO<u32>; 12],
    /// Channel LTA Data Registers.
    pub channel_lta_data: [RO<u32>; 12],
    /// Channel FLT Data Registers.
    pub channel_flt_data: [RO<u32>; 12],
    /// Touch Reserved Register.
    pub touch_rsvd: RW<u32>,
    /// Touch Interrupt Setting Register.
    pub touch_int_setting: RW<u32>,
    /// Touch Interrupt Status Register.
    pub touch_int_status: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;
    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, time1), 0x4);
        assert_eq!(offset_of!(RegisterBlock, int), 0xC);
        assert_eq!(offset_of!(RegisterBlock, ctrl2), 0x10);
        assert_eq!(offset_of!(RegisterBlock, ctrl3), 0x14);
        assert_eq!(offset_of!(RegisterBlock, ctrl4), 0x18);
        assert_eq!(offset_of!(RegisterBlock, status), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, ram1), 0x20);
        assert_eq!(offset_of!(RegisterBlock, ctrl5), 0x24);
        assert_eq!(offset_of!(RegisterBlock, ram2), 0x28);
        assert_eq!(offset_of!(RegisterBlock, gpio_input_set), 0x30);
        assert_eq!(offset_of!(RegisterBlock, gpio_pd_set), 0x34);
        assert_eq!(offset_of!(RegisterBlock, gpio_int), 0x40);
        assert_eq!(offset_of!(RegisterBlock, gpio_status), 0x44);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg0), 0x110);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg1), 0x114);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg7), 0x12C);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg8), 0x130);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg9), 0x134);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg10), 0x138);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg12), 0x140);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg13), 0x144);
        assert_eq!(offset_of!(RegisterBlock, cpu_core_cfg14), 0x148);
        assert_eq!(offset_of!(RegisterBlock, tzc_pds), 0x14C);
        assert_eq!(offset_of!(RegisterBlock, rc32m_ctrl0), 0x300);
        assert_eq!(offset_of!(RegisterBlock, rc32m_ctrl1), 0x304);
        assert_eq!(offset_of!(RegisterBlock, pu_rst_clkpll), 0x400);
        assert_eq!(offset_of!(RegisterBlock, usb_ctrl), 0x500);
        assert_eq!(offset_of!(RegisterBlock, usb_phy_ctrl), 0x504);
        assert_eq!(offset_of!(RegisterBlock, touch1), 0xA00);
        assert_eq!(offset_of!(RegisterBlock, touch2), 0xA04);
        assert_eq!(offset_of!(RegisterBlock, touch3), 0xA08);
        assert_eq!(offset_of!(RegisterBlock, touch_sleep_time), 0xA0C);
        assert_eq!(offset_of!(RegisterBlock, touch_data_hystersis), 0xA10);
        assert_eq!(offset_of!(RegisterBlock, channel_force_data), 0xA14);
        assert_eq!(offset_of!(RegisterBlock, channel_vth_data), 0xA2C);
        assert_eq!(offset_of!(RegisterBlock, channel_raw_data), 0xA38);
        assert_eq!(offset_of!(RegisterBlock, channel_lta_data), 0xA68);
        assert_eq!(offset_of!(RegisterBlock, channel_flt_data), 0xA98);
        assert_eq!(offset_of!(RegisterBlock, touch_rsvd), 0xAC8);
        assert_eq!(offset_of!(RegisterBlock, touch_int_setting), 0xACC);
        assert_eq!(offset_of!(RegisterBlock, touch_int_status), 0xAD0);
    }
}
