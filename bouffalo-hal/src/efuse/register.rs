//! Electronic fuse peripheral.
use volatile_register::{RO, RW, WO};

/// Electronic fuse peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x800],
    /// Efuse interface 0 control register.
    pub control: RW<EfuseControl>,
    /// Efuse interface 0 cycle config0 register.
    pub cycle_config0: RW<EfuseCycleConfig0>,
    /// Efuse interface 0 cycle config1 register.
    pub cycle_config1: RW<EfuseCycleConfig1>,
    /// Efuse interface 0 manual config register.
    pub if0_manual_config: RW<EfuseIf0ManualConfig>,
    /// Efuse interface 0 status register.
    pub if0_status: RW<u32>,
    /// Efuse interface 0 config0 register.
    pub config0: RW<EfuseConfig0>,
    _reserved1: [u8; 0xE6],
    /// Only bl808 has the registers below.
    /// Efuse interface 1 control1 register.
    pub control1: RW<EfuseControl1>,
    /// Efuse interface 1 manual config register.
    pub if1_manual_config: RW<EfuseIf1ManualConfig>,
    /// Efuse interface 1 status register.
    pub if1_status: RW<u32>,
}

/// Efuse interface 0 control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseControl(u32);

impl EfuseControl {
    const IF_PROT_CODE_CYC_MASK: u32 = 0xFF << 24;
    const IF_0_INT_SET_MASK: u32 = 0x1 << 22;
    const IF_0_INT_CLR_MASK: u32 = 0x1 << 21;
    const IF_0_INT_MASK: u32 = 0x1 << 20;
    const IF_CYC_MODIFY_LOCK_MASK: u32 = 0x1 << 19;
    const IF_AUTO_RD_EN_MASK: u32 = 0x1 << 18;
    const PCLK_FORCE_ON_MASK: u32 = 0x1 << 17;
    const CLK_SAHB_DATA_GATE_MASK: u32 = 0x1 << 17;
    const IF_POR_DIG_MASK: u32 = 0x1 << 16;
    const IF_PROT_CODE_CTRL_MASK: u32 = 0xFF << 8;
    const CLK_SAHB_DATA_SEL_MASK: u32 = 0x1 << 7;
    const IF_0_CYC_MODIFY_MASK: u32 = 0x1 << 6;
    const IF_0_MANUAL_EN_MASK: u32 = 0x1 << 5;
    const IF_0_TRIG_MASK: u32 = 0x1 << 4;
    const IF_0_RW_MASK: u32 = 0x1 << 3;
    const IF_0_BUSY_MASK: u32 = 0x1 << 2;
    const IF_0_AUTOLOAD_DONE_MASK: u32 = 0x1 << 1;
    const IF_0_AUTOLOAD_P1_DONE_MASK: u32 = 0x1;

    /// Set protect code cycle.
    #[inline]
    pub const fn set_prot_code_cyc(self, cycle: u8) -> Self {
        Self((self.0 & !Self::IF_PROT_CODE_CYC_MASK) | ((cycle as u32) << 24))
    }
    /// Get protect code cycle.
    #[inline]
    pub const fn prot_code_cyc(self) -> u8 {
        ((self.0 & Self::IF_PROT_CODE_CYC_MASK) >> 24) as u8
    }
    /// Set interface 0 interrupt set bit.
    #[inline]
    pub const fn set_if0_int_set(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_INT_SET_MASK) | (Self::IF_0_INT_SET_MASK & ((val as u32) << 22)))
    }
    /// Get interface 0 interrupt set bit.
    #[inline]
    pub const fn if0_int_set(self) -> u8 {
        ((self.0 & Self::IF_0_INT_SET_MASK) >> 22) as u8
    }
    /// Set interface 0 interrupt clear bit.
    #[inline]
    pub const fn set_if0_int_clr(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_INT_CLR_MASK) | (Self::IF_0_INT_CLR_MASK & ((val as u32) << 21)))
    }
    /// Get interface 0 interrupt clear bit.
    #[inline]
    pub const fn if0_int_clr(self) -> u8 {
        ((self.0 & Self::IF_0_INT_CLR_MASK) >> 21) as u8
    }
    /// Set interface 0 interrupt bit.
    #[inline]
    pub const fn set_if0_int(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_INT_MASK) | (Self::IF_0_INT_MASK & ((val as u32) << 20)))
    }
    /// Get interface 0 interrupt bit.
    #[inline]
    pub const fn if0_int(self) -> u8 {
        ((self.0 & Self::IF_0_INT_MASK) >> 20) as u8
    }
    /// Set cycle modify lock bit.
    #[inline]
    pub const fn set_cyc_modify_lock(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::IF_CYC_MODIFY_LOCK_MASK)
                | (Self::IF_CYC_MODIFY_LOCK_MASK & ((val as u32) << 19)),
        )
    }
    /// Get cycle modify lock bit.
    #[inline]
    pub const fn cyc_modify_lock(self) -> u8 {
        ((self.0 & Self::IF_CYC_MODIFY_LOCK_MASK) >> 19) as u8
    }
    /// Enable automatic read.
    #[inline]
    pub const fn enable_auto_read(self) -> Self {
        Self(self.0 | Self::IF_AUTO_RD_EN_MASK)
    }
    /// Disable automatic read.
    #[inline]
    pub const fn disable_auto_read(self) -> Self {
        Self(self.0 & !Self::IF_AUTO_RD_EN_MASK)
    }
    /// Check if automatic read is enabled.
    #[inline]
    pub const fn is_auto_read_enabled(self) -> bool {
        (self.0 & Self::IF_AUTO_RD_EN_MASK) != 0
    }
    /// Enable PCLK force on.
    #[inline]
    pub const fn enable_pclk_force_on(self) -> Self {
        Self(self.0 | Self::PCLK_FORCE_ON_MASK)
    }
    /// Disable PCLK force on.
    #[inline]
    pub const fn disable_pclk_force_on(self) -> Self {
        Self(self.0 & !Self::PCLK_FORCE_ON_MASK)
    }
    /// Check if PCLK force on is enabled.
    #[inline]
    pub const fn is_pclk_force_on_enabled(self) -> bool {
        (self.0 & Self::PCLK_FORCE_ON_MASK) != 0
    }
    /// Set clock SAHB data gate bit.
    #[inline]
    pub const fn set_clk_sahb_data_gate(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::CLK_SAHB_DATA_GATE_MASK)
                | (Self::CLK_SAHB_DATA_GATE_MASK & ((val as u32) << 17)),
        )
    }
    /// Get clock SAHB data gate bit.
    #[inline]
    pub const fn clk_sahb_data_gate(self) -> u8 {
        ((self.0 & Self::CLK_SAHB_DATA_GATE_MASK) >> 17) as u8
    }
    /// Set POR_DIG bit.
    #[inline]
    pub const fn set_por_dig(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_POR_DIG_MASK) | (Self::IF_POR_DIG_MASK & ((val as u32) << 16)))
    }
    /// Get POR_DIG bit.
    #[inline]
    pub const fn por_dig(self) -> u8 {
        ((self.0 & Self::IF_POR_DIG_MASK) >> 16) as u8
    }
    /// Set protect code control bit.
    #[inline]
    pub const fn set_prot_code_ctrl(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::IF_PROT_CODE_CTRL_MASK)
                | (Self::IF_PROT_CODE_CTRL_MASK & ((val as u32) << 8)),
        )
    }
    /// Get protect code control bit.
    #[inline]
    pub const fn prot_code_ctrl(self) -> u8 {
        ((self.0 & Self::IF_PROT_CODE_CTRL_MASK) >> 8) as u8
    }
    /// Set clock SAHB data select bit.
    #[inline]
    pub const fn set_clk_sahb_data_sel(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::CLK_SAHB_DATA_SEL_MASK)
                | (Self::CLK_SAHB_DATA_SEL_MASK & ((val as u32) << 7)),
        )
    }
    /// Get clock SAHB data select bit.
    #[inline]
    pub const fn clk_sahb_data_sel(self) -> u8 {
        ((self.0 & Self::CLK_SAHB_DATA_SEL_MASK) >> 7) as u8
    }
    /// Set interface 0 cycle modify bit.
    #[inline]
    pub const fn set_if0_cyc_modify(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::IF_0_CYC_MODIFY_MASK)
                | (Self::IF_0_CYC_MODIFY_MASK & ((val as u32) << 6)),
        )
    }
    /// Get interface 0 cycle modify bit.
    #[inline]
    pub const fn if0_cyc_modify(self) -> u8 {
        ((self.0 & Self::IF_0_CYC_MODIFY_MASK) >> 6) as u8
    }
    /// Enable interface 0 manually.
    #[inline]
    pub const fn enable_if0_manually(self) -> Self {
        Self(self.0 | Self::IF_0_MANUAL_EN_MASK)
    }
    /// Disable interface 0 manually.
    #[inline]
    pub const fn disable_if0_manually(self) -> Self {
        Self(self.0 & !Self::IF_0_MANUAL_EN_MASK)
    }
    /// Check if interface 0 is manually enabled.
    #[inline]
    pub const fn is_if0_manually_enabled(self) -> bool {
        (self.0 & Self::IF_0_MANUAL_EN_MASK) != 0
    }
    /// Set interface 0 trig bit.
    #[inline]
    pub const fn set_if0_trig(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_TRIG_MASK) | (Self::IF_0_TRIG_MASK & ((val as u32) << 4)))
    }
    /// Get interface 0 trig bit.
    #[inline]
    pub const fn if0_trig(self) -> u8 {
        ((self.0 & Self::IF_0_TRIG_MASK) >> 4) as u8
    }
    /// Set interface 0 read/write bit.
    #[inline]
    pub const fn set_if0_rw(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_RW_MASK) | (Self::IF_0_RW_MASK & ((val as u32) << 3)))
    }
    /// Get interface 0 read/write bit.
    #[inline]
    pub const fn if0_rw(self) -> u8 {
        ((self.0 & Self::IF_0_RW_MASK) >> 3) as u8
    }
    /// Check if efuse is busy.
    #[inline]
    pub const fn is_efuse_busy(self) -> bool {
        (self.0 & Self::IF_0_BUSY_MASK) != 0
    }
    /// Check if efuse autoload is done.
    #[inline]
    pub const fn is_efuse_autoload_done(self) -> bool {
        (self.0 & Self::IF_0_AUTOLOAD_DONE_MASK) != 0
    }
    /// Check if efuse autoload p1 is done.
    #[inline]
    pub const fn is_efuse_autoload_p1_done(self) -> bool {
        (self.0 & Self::IF_0_AUTOLOAD_P1_DONE_MASK) != 0
    }
}

/// Efuse interface 0 cycle config0 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseCycleConfig0(u32);

/// Efuse interface 0 cycle config1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseCycleConfig1(u32);

impl EfuseCycleConfig1 {
    const IF_CYC_PS_CS_H_MASK: u32 = 0x3F << 26;
    const IF_CYC_PS_CS_MASK: u32 = 0x3F << 20;
    const IF_CYC_WR_ADR_MASK: u32 = 0x3F << 14;
    const IF_CYC_PP_MASK: u32 = 0xFF << 6;
    const IF_CYC_PI_MASK: u32 = 0x3F;

    /// Set cycle PS_CS_H bit.
    #[inline]
    pub const fn set_cyc_ps_cs_h(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::IF_CYC_PS_CS_H_MASK)
                | (Self::IF_CYC_PS_CS_H_MASK & ((val as u32) << 26)),
        )
    }
    /// Get cycle PS_CS_H bit.
    #[inline]
    pub const fn cyc_ps_cs_h(self) -> u8 {
        ((self.0 & Self::IF_CYC_PS_CS_H_MASK) >> 26) as u8
    }
    /// Set cycle PS_CS bit.
    #[inline]
    pub const fn set_cyc_cs(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_CYC_PS_CS_MASK) | (Self::IF_CYC_PS_CS_MASK & ((val as u32) << 20)))
    }
    /// Get cycle PS_CS bit.
    #[inline]
    pub const fn cyc_cs(self) -> u8 {
        ((self.0 & Self::IF_CYC_PS_CS_MASK) >> 20) as u8
    }
    /// Set cycle write/read address.
    #[inline]
    pub const fn set_cyc_wr_addr(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::IF_CYC_WR_ADR_MASK)
                | (Self::IF_CYC_WR_ADR_MASK & ((val as u32) << 14)),
        )
    }
    /// Get cycle write/read address.
    #[inline]
    pub const fn cyc_wr_addr(self) -> u8 {
        ((self.0 & Self::IF_CYC_WR_ADR_MASK) >> 14) as u8
    }
    /// Set cycle PP bit.
    #[inline]
    pub const fn set_cyc_pp(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_CYC_PP_MASK) | (Self::IF_CYC_PP_MASK & ((val as u32) << 6)))
    }
    /// Get cycle PP bit.
    #[inline]
    pub const fn cyc_pp(self) -> u8 {
        ((self.0 & Self::IF_CYC_PP_MASK) >> 6) as u8
    }
    /// Set cycle PI bit.
    #[inline]
    pub const fn set_cyc_pi(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_CYC_PI_MASK) | (Self::IF_CYC_PI_MASK & (val as u32)))
    }
    /// Get cycle PI bit.
    #[inline]
    pub const fn cyc_pi(self) -> u8 {
        (self.0 & Self::IF_CYC_PI_MASK) as u8
    }
}

/// Efuse interface 0 manual config register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseIf0ManualConfig(u32);

impl EfuseIf0ManualConfig {
    const IF_PROT_CODE_MANUAL_MASK: u32 = 0xFF << 24;
    const IF_0_Q_MASK: u32 = 0xFF << 16;
    const IF_CSB_MASK: u32 = 0x1 << 15;
    const IF_LOAD_MASK: u32 = 0x1 << 14;
    const IF_PGENB_MASK: u32 = 0x1 << 13;
    const IF_STROBE_MASK: u32 = 0x1 << 12;
    const IF_PS_MASK: u32 = 0x1 << 11;
    const IF_PD_MASK: u32 = 0x1 << 10;
    const IF_A_MASK: u32 = 0x3FF;

    /// Set protect code manually.
    #[inline]
    pub const fn set_prot_code_manually(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::IF_PROT_CODE_MANUAL_MASK)
                | (Self::IF_PROT_CODE_MANUAL_MASK & ((val as u32) << 24)),
        )
    }
    /// Get protect code manually.
    #[inline]
    pub const fn prot_code_manually(self) -> u8 {
        ((self.0 & Self::IF_PROT_CODE_MANUAL_MASK) >> 24) as u8
    }
    /// Set interface 0 Q bit.
    #[inline]
    pub const fn set_if0_q(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_0_Q_MASK) | (Self::IF_0_Q_MASK & ((val as u32) << 16)))
    }
    /// Get interface 0 Q bit.
    #[inline]
    pub const fn if0_q(self) -> u8 {
        ((self.0 & Self::IF_0_Q_MASK) >> 16) as u8
    }
    /// Set CSB bit.
    #[inline]
    pub const fn set_csb(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_CSB_MASK) | (Self::IF_CSB_MASK & ((val as u32) << 15)))
    }
    /// Get CSB bit.
    #[inline]
    pub const fn csb(self) -> u8 {
        ((self.0 & Self::IF_CSB_MASK) >> 15) as u8
    }
    /// Set LOAD bit.
    #[inline]
    pub const fn set_load(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_LOAD_MASK) | (Self::IF_LOAD_MASK & ((val as u32) << 14)))
    }
    /// Get LOAD bit.
    #[inline]
    pub const fn load(self) -> u8 {
        ((self.0 & Self::IF_LOAD_MASK) >> 14) as u8
    }
    /// Set PGENB bit.
    #[inline]
    pub const fn set_pgenb(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_PGENB_MASK) | (Self::IF_PGENB_MASK & ((val as u32) << 13)))
    }
    /// Get PGENB bit.
    #[inline]
    pub const fn pgenb(self) -> u8 {
        ((self.0 & Self::IF_PGENB_MASK) >> 13) as u8
    }
    /// Set STROBE bit.
    #[inline]
    pub const fn set_strobe(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_STROBE_MASK) | (Self::IF_STROBE_MASK & ((val as u32) << 12)))
    }
    /// Get STROBE bit.
    #[inline]
    pub const fn strobe(self) -> u8 {
        ((self.0 & Self::IF_STROBE_MASK) >> 12) as u8
    }
    /// Set PS bit.
    #[inline]
    pub const fn set_ps(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_PS_MASK) | (Self::IF_PS_MASK & ((val as u32) << 11)))
    }
    /// Get PS bit.
    #[inline]
    pub const fn ps(self) -> u8 {
        ((self.0 & Self::IF_PS_MASK) >> 11) as u8
    }
    /// Set PD bit.
    #[inline]
    pub const fn set_pd(self, val: u8) -> Self {
        Self((self.0 & !Self::IF_PD_MASK) | (Self::IF_PD_MASK & ((val as u32) << 10)))
    }
    /// Get PD bit.
    #[inline]
    pub const fn pd(self) -> u8 {
        ((self.0 & Self::IF_PD_MASK) >> 10) as u8
    }
    /// Set address field.
    #[inline]
    pub const fn set_address(self, val: u16) -> Self {
        Self((self.0 & !Self::IF_A_MASK) | (Self::IF_A_MASK & (val as u32)))
    }
    /// Get address field.
    #[inline]
    pub const fn address(self) -> u16 {
        (self.0 & Self::IF_A_MASK) as u16
    }
}

/// Efuse interface 0 config0 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseConfig0(u32);

/// Efuse interface 1 control1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseControl1(u32);

/// Efuse interface 1 manual config register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EfuseIf1ManualConfig(u32);

#[cfg(test)]
mod tests {
    use super::{
        EfuseConfig0, EfuseControl, EfuseCycleConfig0, EfuseCycleConfig1, EfuseIf0ManualConfig,
        RegisterBlock,
    };
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x800);
        assert_eq!(offset_of!(RegisterBlock, cycle_config0), 0x804);
        assert_eq!(offset_of!(RegisterBlock, cycle_config1), 0x808);
        assert_eq!(offset_of!(RegisterBlock, if0_manual_config), 0x80C);
        assert_eq!(offset_of!(RegisterBlock, if0_status), 0x810);
        assert_eq!(offset_of!(RegisterBlock, control1), 0x900);
        assert_eq!(offset_of!(RegisterBlock, if1_manual_config), 0x904);
        assert_eq!(offset_of!(RegisterBlock, if1_status), 0x908);
    }

    #[test]
    fn struct_efuse_control_functions() {
        let mut val = EfuseControl(0x0);

        val = val.set_prot_code_cyc(0xFF);
        assert_eq!(val.prot_code_cyc(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = EfuseControl(0x0).set_if0_int_set(0x1);
        assert_eq!(val.if0_int_set(), 0x1);
        assert_eq!(val.0, 0x0040_0000);

        val = EfuseControl(0x0).set_if0_int_clr(0x1);
        assert_eq!(val.if0_int_clr(), 0x1);
        assert_eq!(val.0, 0x0020_0000);

        val = EfuseControl(0x0).set_if0_int(0x1);
        assert_eq!(val.if0_int(), 0x1);
        assert_eq!(val.0, 0x0010_0000);

        val = EfuseControl(0x0).set_cyc_modify_lock(0x1);
        assert_eq!(val.cyc_modify_lock(), 0x1);
        assert_eq!(val.0, 0x0008_0000);

        val = EfuseControl(0x0).enable_auto_read();
        assert!(val.is_auto_read_enabled());
        assert_eq!(val.0, 0x0004_0000);

        val = val.disable_auto_read();
        assert!(!val.is_auto_read_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_pclk_force_on();
        assert!(val.is_pclk_force_on_enabled());
        assert_eq!(val.0, 0x0002_0000);

        val = val.disable_pclk_force_on();
        assert!(!val.is_pclk_force_on_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_sahb_data_gate(0x1);
        assert_eq!(val.clk_sahb_data_gate(), 0x1);
        assert_eq!(val.0, 0x0002_0000);

        val = EfuseControl(0x0).set_por_dig(0x1);
        assert_eq!(val.por_dig(), 0x1);
        assert_eq!(val.0, 0x0001_0000);

        val = EfuseControl(0x0).set_prot_code_ctrl(0xFF);
        assert_eq!(val.prot_code_ctrl(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = EfuseControl(0x0).set_clk_sahb_data_sel(0x1);
        assert_eq!(val.clk_sahb_data_sel(), 0x1);
        assert_eq!(val.0, 0x0000_0080);

        val = EfuseControl(0x0).set_if0_cyc_modify(0x1);
        assert_eq!(val.if0_cyc_modify(), 0x1);
        assert_eq!(val.0, 0x0000_0040);

        val = EfuseControl(0x0).enable_if0_manually();
        assert!(val.is_if0_manually_enabled());
        assert_eq!(val.0, 0x0000_0020);

        val = EfuseControl(0x0).disable_if0_manually();
        assert!(!val.is_if0_manually_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = EfuseControl(0x0).set_if0_trig(0x1);
        assert_eq!(val.if0_trig(), 0x1);
        assert_eq!(val.0, 0x0000_0010);

        val = EfuseControl(0x0).set_if0_rw(0x1);
        assert_eq!(val.if0_rw(), 0x1);
        assert_eq!(val.0, 0x0000_0008);

        val = EfuseControl(0x0000_0004);
        assert!(val.is_efuse_busy());

        val = EfuseControl(0x0000_0002);
        assert!(val.is_efuse_autoload_done());

        val = EfuseControl(0x0000_0001);
        assert!(val.is_efuse_autoload_p1_done());
    }

    #[test]
    fn struct_efuse_cycle_config1_functions() {
        let mut val = EfuseCycleConfig1(0x0);

        val = val.set_cyc_ps_cs_h(0x3F);
        assert_eq!(val.cyc_ps_cs_h(), 0x3F);
        assert_eq!(val.0, 0xFC00_0000);

        val = EfuseCycleConfig1(0x0).set_cyc_cs(0x3F);
        assert_eq!(val.cyc_cs(), 0x3F);
        assert_eq!(val.0, 0x03F0_0000);

        val = EfuseCycleConfig1(0x0).set_cyc_wr_addr(0x3F);
        assert_eq!(val.cyc_wr_addr(), 0x3F);
        assert_eq!(val.0, 0x000F_C000);

        val = EfuseCycleConfig1(0x0).set_cyc_pp(0xFF);
        assert_eq!(val.cyc_pp(), 0xFF);
        assert_eq!(val.0, 0x0000_3FC0);

        val = EfuseCycleConfig1(0x0).set_cyc_pi(0x3F);
        assert_eq!(val.cyc_pi(), 0x3F);
        assert_eq!(val.0, 0x0000_003F);
    }

    #[test]
    fn struct_efuse_if0_manual_config_functions() {
        let mut val = EfuseIf0ManualConfig(0x0);

        val = val.set_prot_code_manually(0xFF);
        assert_eq!(val.prot_code_manually(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = EfuseIf0ManualConfig(0x0).set_if0_q(0xFF);
        assert_eq!(val.if0_q(), 0xFF);
        assert_eq!(val.0, 0x00FF_0000);

        val = EfuseIf0ManualConfig(0x0).set_csb(0x1);
        assert_eq!(val.csb(), 0x1);
        assert_eq!(val.0, 0x0000_8000);

        val = EfuseIf0ManualConfig(0x0).set_load(0x1);
        assert_eq!(val.load(), 0x1);
        assert_eq!(val.0, 0x0000_4000);

        val = EfuseIf0ManualConfig(0x0).set_pgenb(0x1);
        assert_eq!(val.pgenb(), 0x1);
        assert_eq!(val.0, 0x0000_2000);

        val = EfuseIf0ManualConfig(0x0).set_strobe(0x1);
        assert_eq!(val.strobe(), 0x1);
        assert_eq!(val.0, 0x0000_1000);

        val = EfuseIf0ManualConfig(0x0).set_ps(0x1);
        assert_eq!(val.ps(), 0x1);
        assert_eq!(val.0, 0x0000_0800);

        val = EfuseIf0ManualConfig(0x0).set_pd(0x1);
        assert_eq!(val.pd(), 0x1);
        assert_eq!(val.0, 0x0000_0400);

        val = EfuseIf0ManualConfig(0x0).set_address(0x3FF);
        assert_eq!(val.address(), 0x3FF);
        assert_eq!(val.0, 0x0000_03FF);
    }
}
