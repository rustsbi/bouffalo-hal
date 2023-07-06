//! BL808 tri-core heterogeneous Wi-Fi 802.11b/g/n, Bluetooth 5, Zigbee AIoT system-on-chip.

use crate::{HalBasicConfig, HalFlashConfig, HalPatchCfg};

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0"))]
use core::arch::asm;

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0"))]
use crate::Stack;

use base_address::Static;

#[cfg(feature = "bl808-m0")]
const LEN_STACK_M0: usize = 1 * 1024;

#[cfg(feature = "bl808-d0")]
const LEN_STACK_D0: usize = 1 * 1024;

#[cfg(feature = "bl808-m0")]
#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn start() -> ! {
    #[link_section = ".bss.uninit"]
    static mut STACK: Stack<LEN_STACK_M0> = Stack([0; LEN_STACK_M0]);
    asm!(
        "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
        "   la      t1, sbss
            la      t2, ebss
        1:  bgeu    t1, t2, 1f
            sw      zero, 0(t1)
            addi    t1, t1, 4
            j       1b
        1:",
        "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 1f
            lw      t6, 0(t3)
            sw      t6, 0(t4)
            addi    t3, t3, 4
            addi    t4, t4, 4
            j       1b
        1:",
        "   call  {main}",
        stack = sym STACK,
        hart_stack_size = const LEN_STACK_M0,
        main = sym main,
        options(noreturn)
    )
}

#[cfg(feature = "bl808-d0")]
#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn start() -> ! {
    #[link_section = ".bss.uninit"]
    static mut STACK: Stack<LEN_STACK_D0> = Stack([0; LEN_STACK_D0]);
    asm!(
        "   la      sp, {stack}
            li      t0, {hart_stack_size}
            add     sp, sp, t0",
        "   la      t1, sbss
            la      t2, ebss
        1:  bgeu    t1, t2, 1f
            sd      zero, 0(t1)
            addi    t1, t1, 8 
            j       1b
        1:",
        "   la      t3, sidata
            la      t4, sdata
            la      t5, edata
        1:  bgeu    t4, t5, 1f
            ld      t6, 0(t3)
            sd      t6, 0(t4)
            addi    t3, t3, 8
            addi    t4, t4, 8
            j       1b
        1:",
        "   call    {main}",
        stack = sym STACK,
        hart_stack_size = const LEN_STACK_D0,
        main = sym main,
        options(noreturn)
    )
}

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0"))]
#[rustfmt::skip]
extern "Rust" {
    // This symbol is generated by `#[entry]` macro
    fn main() -> !;
}

/// Clock configuration at boot-time.
#[cfg(any(doc, feature = "bl808-m0", feature = "bl808-d0"))]
#[link_section = ".head.clock"]
pub static CLOCK_CONFIG: HalPllConfig = HalPllConfig::new(HalSysClkConfig {
    xtal_type: 0x07,
    mcu_clk: 0x04,
    mcu_clk_div: 0x00,
    mcu_bclk_div: 0x00,

    mcu_pbclk_div: 0x03,
    lp_div: 0x01,
    dsp_clk: 0x03,
    dsp_clk_div: 0x00,

    dsp_bclk_div: 0x01,
    dsp_pbclk: 0x02,
    dsp_pbclk_div: 0x02,
    emi_clk: 0x02,

    emi_clk_div: 0x01,
    flash_clk_type: 0x01,
    flash_clk_div: 0x00,
    wifipll_pu: 0x01,

    aupll_pu: 0x01,
    cpupll_pu: 0x01,
    mipipll_pu: 0x01,
    uhspll_pu: 0x01,
});

/// Processor core configuration.
#[cfg(any(doc, feature = "bl808-m0", feature = "bl808-d0"))]
#[link_section = ".head.cpu"]
pub static CPU_CONFIG: [HalCpuCfg; 3] = [
    #[cfg(feature = "bl808-m0")]
    HalCpuCfg {
        config_enable: 1,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 0,
        cache_range_l: 0,
        image_address_offset: 0,
        boot_entry: 0x58000000,
        msp_val: 0,
    },
    #[cfg(not(feature = "bl808-m0"))]
    HalCpuCfg::disabled(),
    #[cfg(feature = "bl808-d0")]
    HalCpuCfg {
        config_enable: 1,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 0,
        cache_range_l: 0,
        image_address_offset: 0,
        boot_entry: 0x58000000,
        msp_val: 0,
    },
    #[cfg(not(feature = "bl808-d0"))]
    HalCpuCfg::disabled(),
    #[cfg(feature = "bl808-lp")]
    HalCpuCfg {
        config_enable: 1,
        halt_cpu: 0,
        cache_flags: 0,
        _rsvd: 0,
        cache_range_h: 0,
        cache_range_l: 0,
        image_address_offset: 0,
        boot_entry: 0,
        msp_val: 0,
    },
    #[cfg(not(feature = "bl808-lp"))]
    HalCpuCfg::disabled(),
];

/// Code patches on flash reading.
#[cfg(any(doc, feature = "bl808-m0", feature = "bl808-d0"))]
#[link_section = ".head.patch.on-read"]
pub static PATCH_ON_READ: [HalPatchCfg; 4] = [
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
];

/// Code patches on jump and run stage.
#[cfg(any(doc, feature = "bl808-m0", feature = "bl808-d0"))]
#[link_section = ".head.patch.on-jump"]
pub static PATCH_ON_JUMP: [HalPatchCfg; 4] = [
    HalPatchCfg {
        addr: 0x20000320,
        value: 0x0,
    },
    HalPatchCfg {
        addr: 0x2000F038,
        value: 0x18000000,
    },
    HalPatchCfg { addr: 0, value: 0 },
    HalPatchCfg { addr: 0, value: 0 },
];

/// Full ROM bootloading header.
#[repr(C)]
pub struct HalBootheader {
    magic: u32,
    revision: u32,
    flash_cfg: HalFlashConfig,
    clk_cfg: HalPllConfig,
    basic_cfg: HalBasicConfig,
    cpu_cfg: [HalCpuCfg; 3],
    /// Address of partition table 0.
    boot2_pt_table_0: u32,
    /// Address of partition table 1.
    boot2_pt_table_1: u32,
    /// Address of flashcfg table list.
    flash_cfg_table_addr: u32,
    /// Flashcfg table list len.
    flash_cfg_table_len: u32,
    /// Do patch when read flash.
    patch_on_read: [HalPatchCfg; 4],
    /// Do patch when jump.
    patch_on_jump: [HalPatchCfg; 4],
    _reserved: [u32; 5],
    crc32: u32,
}

/// Hardware system clock configuration.
#[repr(C)]
pub struct HalSysClkConfig {
    xtal_type: u8,
    mcu_clk: u8,
    mcu_clk_div: u8,
    mcu_bclk_div: u8,

    mcu_pbclk_div: u8,
    lp_div: u8,
    dsp_clk: u8,
    dsp_clk_div: u8,

    dsp_bclk_div: u8,
    dsp_pbclk: u8,
    dsp_pbclk_div: u8,
    emi_clk: u8,

    emi_clk_div: u8,
    flash_clk_type: u8,
    flash_clk_div: u8,
    wifipll_pu: u8,

    aupll_pu: u8,
    cpupll_pu: u8,
    mipipll_pu: u8,
    uhspll_pu: u8,
}

impl HalSysClkConfig {
    #[inline]
    pub const fn crc32(&self) -> u32 {
        let mut buf = [0u8; 20];

        buf[0] = self.xtal_type;
        buf[1] = self.mcu_clk;
        buf[2] = self.mcu_clk_div;
        buf[3] = self.mcu_bclk_div;

        buf[4] = self.mcu_pbclk_div;
        buf[5] = self.lp_div;
        buf[6] = self.dsp_clk;
        buf[7] = self.dsp_clk_div;

        buf[8] = self.dsp_bclk_div;
        buf[9] = self.dsp_pbclk;
        buf[10] = self.dsp_pbclk_div;
        buf[11] = self.emi_clk;

        buf[12] = self.emi_clk_div;
        buf[13] = self.flash_clk_type;
        buf[14] = self.flash_clk_div;
        buf[15] = self.wifipll_pu;

        buf[16] = self.aupll_pu;
        buf[17] = self.cpupll_pu;
        buf[18] = self.mipipll_pu;
        buf[19] = self.uhspll_pu;

        crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&buf)
    }
}

/// Clock configuration in ROM header.
#[repr(C)]
pub struct HalPllConfig {
    magic: u32,
    cfg: HalSysClkConfig,
    crc32: u32,
}

impl HalPllConfig {
    /// Create this structure with magic number and CRC32 filled in compile time.
    #[inline]
    pub const fn new(cfg: HalSysClkConfig) -> Self {
        let crc32 = cfg.crc32();
        HalPllConfig {
            magic: 0x47464350,
            cfg,
            crc32,
        }
    }
}

/// Processor core configuration in ROM header.
#[repr(C)]
pub struct HalCpuCfg {
    /// Config this cpu.
    config_enable: u8,
    /// Halt this cpu.
    halt_cpu: u8,
    /// Cache setting.
    cache_flags: u8,
    _rsvd: u8,
    /// Cache range high.
    cache_range_h: u32,
    /// Cache range low.
    cache_range_l: u32,
    /// Image address on flash.
    image_address_offset: u32,
    /// Entry point of the m0 image.
    boot_entry: u32,
    /// Msp value.
    msp_val: u32,
}

impl HalCpuCfg {
    #[allow(dead_code)]
    #[inline]
    const fn disabled() -> HalCpuCfg {
        HalCpuCfg {
            config_enable: 0,
            halt_cpu: 0,
            cache_flags: 0,
            _rsvd: 0,
            cache_range_h: 0,
            cache_range_l: 0,
            image_address_offset: 0,
            boot_entry: 0x0,
            msp_val: 0,
        }
    }
}

/// Peripherals available on ROM start.
pub struct Peripherals {
    /// Global configuration peripheral.
    pub glb: bl_soc::GLB<Static<0x20000000>>,
    /// General Purpose Input/Output pins.
    pub gpio: bl_soc::gpio::Pins<Static<0x20000000>>,
    /// UART0 configuration peripheral.
    pub uart0: bl_soc::UART<Static<0x2000A000>>,
    /// UART1 configuration peripheral.
    pub uart1: bl_soc::UART<Static<0x2000A100>>,
    /// UART signal multiplexers.
    pub uart_muxes: bl_soc::uart::UartMuxes<Static<0x20000000>>,
}

#[cfg(test)]
mod tests {
    use super::{HalBootheader, HalCpuCfg, HalPllConfig, HalSysClkConfig};
    use memoffset::offset_of;

    #[test]
    fn struct_lengths() {
        use core::mem::size_of;
        assert_eq!(size_of::<HalPllConfig>(), 28);
        assert_eq!(size_of::<HalBootheader>(), 352);
    }

    #[test]
    fn struct_hal_bootheader_offset() {
        assert_eq!(offset_of!(HalBootheader, magic), 0x00);
        assert_eq!(offset_of!(HalBootheader, revision), 0x04);
        assert_eq!(offset_of!(HalBootheader, flash_cfg), 0x08);
        assert_eq!(offset_of!(HalBootheader, clk_cfg), 0x64);
        assert_eq!(offset_of!(HalBootheader, basic_cfg), 0x80);
        assert_eq!(offset_of!(HalBootheader, cpu_cfg), 0xb0);
        assert_eq!(offset_of!(HalBootheader, boot2_pt_table_0), 0xf8);
        assert_eq!(offset_of!(HalBootheader, boot2_pt_table_1), 0xfc);
        assert_eq!(offset_of!(HalBootheader, flash_cfg_table_addr), 0x100);
        assert_eq!(offset_of!(HalBootheader, flash_cfg_table_len), 0x104);
        assert_eq!(offset_of!(HalBootheader, patch_on_read), 0x108);
        assert_eq!(offset_of!(HalBootheader, patch_on_jump), 0x128);
        assert_eq!(offset_of!(HalBootheader, crc32), 0x15c);
    }

    #[test]
    fn struct_hal_sys_clk_config_offset() {
        assert_eq!(offset_of!(HalSysClkConfig, xtal_type), 0x00);
        assert_eq!(offset_of!(HalSysClkConfig, mcu_clk), 0x01);
        assert_eq!(offset_of!(HalSysClkConfig, mcu_clk_div), 0x02);
        assert_eq!(offset_of!(HalSysClkConfig, mcu_bclk_div), 0x03);

        assert_eq!(offset_of!(HalSysClkConfig, mcu_pbclk_div), 0x04);
        assert_eq!(offset_of!(HalSysClkConfig, lp_div), 0x05);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_clk), 0x06);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_clk_div), 0x07);

        assert_eq!(offset_of!(HalSysClkConfig, dsp_bclk_div), 0x08);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_pbclk), 0x9);
        assert_eq!(offset_of!(HalSysClkConfig, dsp_pbclk_div), 0x0a);
        assert_eq!(offset_of!(HalSysClkConfig, emi_clk), 0x0b);

        assert_eq!(offset_of!(HalSysClkConfig, emi_clk_div), 0x0c);
        assert_eq!(offset_of!(HalSysClkConfig, flash_clk_type), 0x0d);
        assert_eq!(offset_of!(HalSysClkConfig, flash_clk_div), 0x0e);
        assert_eq!(offset_of!(HalSysClkConfig, wifipll_pu), 0x0f);

        assert_eq!(offset_of!(HalSysClkConfig, aupll_pu), 0x10);
        assert_eq!(offset_of!(HalSysClkConfig, cpupll_pu), 0x11);
        assert_eq!(offset_of!(HalSysClkConfig, mipipll_pu), 0x12);
        assert_eq!(offset_of!(HalSysClkConfig, uhspll_pu), 0x13);
    }

    #[test]
    fn struct_hal_pll_config_offset() {
        assert_eq!(offset_of!(HalPllConfig, magic), 0x00);
        assert_eq!(offset_of!(HalPllConfig, cfg), 0x04);
        assert_eq!(offset_of!(HalPllConfig, crc32), 0x18);
    }

    #[test]
    fn struct_hal_cpu_cfg_offset() {
        assert_eq!(offset_of!(HalCpuCfg, config_enable), 0x00);
        assert_eq!(offset_of!(HalCpuCfg, halt_cpu), 0x01);
        assert_eq!(offset_of!(HalCpuCfg, cache_flags), 0x02);
        assert_eq!(offset_of!(HalCpuCfg, cache_range_h), 0x04);
        assert_eq!(offset_of!(HalCpuCfg, cache_range_l), 0x08);
        assert_eq!(offset_of!(HalCpuCfg, image_address_offset), 0x0c);
        assert_eq!(offset_of!(HalCpuCfg, boot_entry), 0x10);
        assert_eq!(offset_of!(HalCpuCfg, msp_val), 0x14);
    }

    #[test]
    fn magic_crc32_hal_pll_config() {
        let test_sys_clk_config = HalSysClkConfig {
            xtal_type: 4,
            mcu_clk: 4,
            mcu_clk_div: 0,
            mcu_bclk_div: 0,
            mcu_pbclk_div: 3,
            lp_div: 1,
            dsp_clk: 3,
            dsp_clk_div: 0,
            dsp_bclk_div: 1,
            dsp_pbclk: 2,
            dsp_pbclk_div: 0,
            emi_clk: 2,
            emi_clk_div: 1,
            flash_clk_type: 1,
            flash_clk_div: 0,
            wifipll_pu: 1,
            aupll_pu: 1,
            cpupll_pu: 1,
            mipipll_pu: 1,
            uhspll_pu: 1,
        };
        let test_config = HalPllConfig::new(test_sys_clk_config);
        assert_eq!(test_config.magic, 0x47464350);
        assert_eq!(test_config.crc32, 0x29e2c4c0);
    }
}
