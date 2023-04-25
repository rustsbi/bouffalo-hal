#![no_std]

#[repr(C)]
struct HalBootheader {
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

#[repr(C)]
struct HalFlashConfig {
    magic: u32,
    cfg: SpiFlashCfgType,
    crc32: u32,
}

#[repr(C)]
struct SpiFlashCfgType {
    /// Serail flash uint32erface mode,bit0-3:IF mode,bit4:unwrap,bit5:32-bits addr mode support.
    io_mode: u8,
    /// Support continuous read mode,bit0:continuous read mode support,bit1:read mode cfg.
    c_read_support: u8,
    /// SPI clock delay,bit0-3:delay,bit4-6:pad delay.
    clk_delay: u8,
    /// SPI clock phase invert,bit0:clck invert,bit1:rx invert,bit2-4:pad delay,bit5-7:pad delay.
    clk_invert: u8,
    /// Flash enable reset command.
    reset_en_cmd: u8,
    /// Flash reset command.
    reset_cmd: u8,
    /// Flash reset continuous read command.
    reset_cread_cmd: u8,
    /// Flash reset continuous read command size.
    reset_cread_cmd_size: u8,
    /// JEDEC ID command.
    jedec_id_cmd: u8,
    /// JEDEC ID command dummy clock.
    jedec_id_cmd_dmy_clk: u8,
    /// Enter 32-bits addr command.
    enter_32_bits_addr_cmd: u8,
    /// Exit 32-bits addr command.
    exit_32_bits_addr_cmd: u8,
    /// *1024bytes
    sector_size: u8,
    /// Manufacturer ID.
    mid: u8,
    /// Page size.
    page_size: u16,
    /// Chip erase cmd.
    chip_erase_cmd: u8,
    /// Sector erase command.
    sector_erase_cmd: u8,
    /// Block 32K erase command,some Micron not support.
    blk32_erase_cmd: u8,
    /// Block 64K erase command.
    blk64_erase_cmd: u8,
    /// Need before every erase or program.
    write_enable_cmd: u8,
    /// Page program cmd.
    page_program_cmd: u8,
    /// QIO page program cmd.
    qpage_program_cmd: u8,
    /// QIO page program address mode.
    qpp_addr_mode: u8,
    /// Fast read command.
    fast_read_cmd: u8,
    /// Fast read command dummy clock.
    fr_dmy_clk: u8,
    /// QPI fast read command.
    qpi_fast_read_cmd: u8,
    /// QPI fast read command dummy clock.
    qpi_fr_dmy_clk: u8,
    /// Fast read dual output command.
    fast_read_do_cmd: u8,
    /// Fast read dual output command dummy clock.
    fr_do_dmy_clk: u8,
    /// Fast read dual io comamnd.
    fast_read_dio_cmd: u8,
    /// Fast read dual io command dummy clock.
    fr_dio_dmy_clk: u8,
    /// Fast read quad output comamnd.
    fast_read_qo_cmd: u8,
    /// Fast read quad output comamnd dummy clock.
    fr_qo_dmy_clk: u8,
    /// Fast read quad io comamnd.
    fast_read_qio_cmd: u8,
    /// Fast read quad io comamnd dummy clock.
    fr_qio_dmy_clk: u8,
    /// QPI fast read quad io comamnd.
    qpi_fast_read_qio_cmd: u8,
    /// QPI fast read QIO dummy clock.
    qpi_fr_qio_dmy_clk: u8,
    /// QPI program command.
    qpi_page_program_cmd: u8,
    /// Enable write reg.
    writev_reg_enable_cmd: u8,
    /// Write enable register index.
    wr_enable_index: u8,
    /// Quad mode enable register index.
    qe_index: u8,
    /// Busy status register index.
    busy_index: u8,
    /// Write enable bit pos.
    wr_enable_bit: u8,
    /// Quad enable bit pos.
    qe_bit: u8,
    /// Busy status bit pos.
    busy_bit: u8,
    /// Register length of write enable.
    wr_enable_write_reg_len: u8,
    /// Register length of write enable status.
    wr_enable_read_reg_len: u8,
    /// Register length of contain quad enable.
    qe_write_reg_len: u8,
    /// Register length of contain quad enable status.
    qe_read_reg_len: u8,
    /// Release power down command.
    release_power_down: u8,
    /// Register length of contain busy status.
    busy_read_reg_len: u8,
    /// Read register command buffer.
    read_reg_cmd: [u8; 4],
    /// Write register command buffer.
    write_reg_cmd: [u8; 4],
    /// Enter qpi command.
    enter_qpi: u8,
    /// Exit qpi command.
    exit_qpi: u8,
    /// Config data for continuous read mode.
    c_read_mode: u8,
    /// Config data for exit continuous read mode.
    cr_exit: u8,
    /// Enable burst wrap command.
    burst_wrap_cmd: u8,
    /// Enable burst wrap command dummy clock.
    burst_wrap_cmd_dmy_clk: u8,
    /// Data and address mode for this command.
    burst_wrap_data_mode: u8,
    /// Data to enable burst wrap.
    burst_wrap_data: u8,
    /// Disable burst wrap command.
    de_burst_wrap_cmd: u8,
    /// Disable burst wrap command dummy clock.
    de_burst_wrap_cmd_dmy_clk: u8,
    /// Data and address mode for this command.
    de_burst_wrap_data_mode: u8,
    /// Data to disable burst wrap.
    de_burst_wrap_data: u8,
    /// 4K erase time.
    time_e_sector: u16,
    /// 32K erase time.
    time_e_32k: u16,
    /// 64K erase time.
    time_e_64k: u16,
    /// Page program time.
    time_page_pgm: u16,
    /// Chip erase time in ms.
    time_ce: u16,
    /// Release power down command delay time for wake up.
    pd_delay: u8,
    /// QE set data.
    qe_data: u8,
}

#[repr(C)]
struct HalPllConfig {
    magic: u32,
    cfg: HalSysClkConfig,
    crc32: u32,
}

#[repr(C)]
struct HalSysClkConfig {
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

#[repr(C)]
struct HalBasicConfig {
    /// Flags 4bytes
    ///
    /// 2bits for sign
    /// 2bits for encrypt
    /// 2bits for key slot
    /// 1bit  for xts mode
    /// 1bit  for rsvd
    /// 1bit  for no segment info
    /// 1bit  for boot2 enable
    /// 1bit  for boot2 rollback
    /// 1bit  for master id
    /// 1bit  for notload in bootrom
    /// 1bit  for ignore crc
    /// 1bit  for hash ignore
    /// 1bit  for power on mm
    /// 3bits for em_sel
    /// 1bit  for command spliter enable
    /// 2bits for cmds wrap mode
    /// 4bits for cmds wrap len
    /// 1bit  for icache invalid
    /// 1bit  for dcache invalid
    /// 1bit  for FPGA halt release function
    flag: u32,
    /// Flash controller offset.
    group_image_offset: u32,
    /// Aes region length.
    aes_region_len: u32,
    /// Image length or segment count.
    img_len_cnt: u32,
    /// Hash of the image.
    hash: [u32; 8],
}

#[repr(C)]
struct HalCpuCfg {
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

#[repr(C)]
struct HalPatchCfg {
    addr: u32,
    value: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        HalBasicConfig, HalBootheader, HalCpuCfg, HalFlashConfig, HalPatchCfg, HalPllConfig,
        HalSysClkConfig,
    };
    use memoffset::offset_of;

    #[test]
    fn struct_lengths() {
        use core::mem::size_of;
        assert_eq!(size_of::<HalFlashConfig>(), 92);
        assert_eq!(size_of::<HalPllConfig>(), 28);
        assert_eq!(size_of::<HalBasicConfig>(), 48);
        assert_eq!(size_of::<HalCpuCfg>(), 24);
        assert_eq!(size_of::<HalPatchCfg>(), 8);
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
    fn struct_hal_flash_config_offset() {
        assert_eq!(offset_of!(HalFlashConfig, magic), 0x00);
        assert_eq!(offset_of!(HalFlashConfig, cfg), 0x04);
        assert_eq!(offset_of!(HalFlashConfig, crc32), 0x58);
    }

    #[test]
    fn struct_hal_pll_config_offset() {
        assert_eq!(offset_of!(HalPllConfig, magic), 0x00);
        assert_eq!(offset_of!(HalPllConfig, cfg), 0x04);
        assert_eq!(offset_of!(HalPllConfig, crc32), 0x18);
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
    fn struct_hal_basic_config_offset() {
        assert_eq!(offset_of!(HalBasicConfig, flag), 0x00);
        assert_eq!(offset_of!(HalBasicConfig, group_image_offset), 0x04);
        assert_eq!(offset_of!(HalBasicConfig, aes_region_len), 0x08);
        assert_eq!(offset_of!(HalBasicConfig, img_len_cnt), 0x0c);
        assert_eq!(offset_of!(HalBasicConfig, hash), 0x10);
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
    fn struct_hal_patch_cfg_offset() {
        assert_eq!(offset_of!(HalPatchCfg, addr), 0x00);
        assert_eq!(offset_of!(HalPatchCfg, value), 0x04);
    }
}
