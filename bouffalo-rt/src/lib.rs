//! Bouffalo chip ROM runtime library.
#![feature(naked_functions)]
#![no_std]

#[macro_use]
mod macros;

pub use bouffalo_rt_macros::{entry, exception, interrupt};

pub mod arch;
pub mod soc;

pub mod prelude {
    pub use bouffalo_hal::prelude::*;
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "bl808-mcu", feature = "bl808-dsp", feature = "bl808-lp"))] {
        pub use soc::bl808::{Peripherals, Clocks};
        #[doc(hidden)]
        pub use soc::bl808::__rom_init_params;
    } else if #[cfg(feature = "bl702")] {
        pub use soc::bl702::{Peripherals, Clocks};
        #[doc(hidden)]
        pub use soc::bl702::__rom_init_params;
    } else if #[cfg(feature = "bl616")] {
        pub use soc::bl616::{Peripherals, Clocks};
        #[doc(hidden)]
        pub use soc::bl616::__rom_init_params;
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "bl808-mcu", feature = "bl808-dsp", feature = "bl702", feature = "bl616"))] {
        pub use arch::rvi::TrapFrame;
    } else if #[cfg(feature = "bl808-lp")] {
        pub use arch::rve::TrapFrame;
    }
}

#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn default_handler() {}

/// Flash configuration in ROM header.
#[repr(C)]
pub struct HalFlashConfig {
    magic: u32,
    cfg: SpiFlashCfgType,
    crc32: u32,
}

impl HalFlashConfig {
    /// Create this structure with magic number and CRC32 filled in compile time.
    #[inline]
    const fn new(cfg: SpiFlashCfgType) -> Self {
        let mut buf = [0u8; 84];
        buf[0] = cfg.io_mode;
        buf[1] = cfg.c_read_support;
        buf[2] = cfg.clk_delay;
        buf[3] = cfg.clk_invert;
        buf[4] = cfg.reset_en_cmd;
        buf[5] = cfg.reset_cmd;
        buf[6] = cfg.reset_cread_cmd;
        buf[7] = cfg.reset_cread_cmd_size;
        buf[8] = cfg.jedec_id_cmd;
        buf[9] = cfg.jedec_id_cmd_dmy_clk;
        buf[10] = cfg.enter_32_bits_addr_cmd;
        buf[11] = cfg.exit_32_bits_addr_cmd;
        buf[12] = cfg.sector_size;
        buf[13] = cfg.mid;
        [buf[14], buf[15]] = cfg.page_size.to_le_bytes();
        buf[16] = cfg.chip_erase_cmd;
        buf[17] = cfg.sector_erase_cmd;
        buf[18] = cfg.blk32_erase_cmd;
        buf[19] = cfg.blk64_erase_cmd;
        buf[20] = cfg.write_enable_cmd;
        buf[21] = cfg.page_program_cmd;
        buf[22] = cfg.qpage_program_cmd;
        buf[23] = cfg.qpp_addr_mode;
        buf[24] = cfg.fast_read_cmd;
        buf[25] = cfg.fr_dmy_clk;
        buf[26] = cfg.qpi_fast_read_cmd;
        buf[27] = cfg.qpi_fr_dmy_clk;
        buf[28] = cfg.fast_read_do_cmd;
        buf[29] = cfg.fr_do_dmy_clk;
        buf[30] = cfg.fast_read_dio_cmd;
        buf[31] = cfg.fr_dio_dmy_clk;
        buf[32] = cfg.fast_read_qo_cmd;
        buf[33] = cfg.fr_qo_dmy_clk;
        buf[34] = cfg.fast_read_qio_cmd;
        buf[35] = cfg.fr_qio_dmy_clk;
        buf[36] = cfg.qpi_fast_read_qio_cmd;
        buf[37] = cfg.qpi_fr_qio_dmy_clk;
        buf[38] = cfg.qpi_page_program_cmd;
        buf[39] = cfg.writev_reg_enable_cmd;
        buf[40] = cfg.wr_enable_index;
        buf[41] = cfg.qe_index;
        buf[42] = cfg.busy_index;
        buf[43] = cfg.wr_enable_bit;
        buf[44] = cfg.qe_bit;
        buf[45] = cfg.busy_bit;
        buf[46] = cfg.wr_enable_write_reg_len;
        buf[47] = cfg.wr_enable_read_reg_len;
        buf[48] = cfg.qe_write_reg_len;
        buf[49] = cfg.qe_read_reg_len;
        buf[50] = cfg.release_power_down;
        buf[51] = cfg.busy_read_reg_len;
        [buf[52], buf[53], buf[54], buf[55]] = cfg.read_reg_cmd;
        [buf[56], buf[57], buf[58], buf[59]] = cfg.write_reg_cmd;
        buf[60] = cfg.enter_qpi;
        buf[61] = cfg.exit_qpi;
        buf[62] = cfg.c_read_mode;
        buf[63] = cfg.cr_exit;
        buf[64] = cfg.burst_wrap_cmd;
        buf[65] = cfg.burst_wrap_cmd_dmy_clk;
        buf[66] = cfg.burst_wrap_data_mode;
        buf[67] = cfg.burst_wrap_data;
        buf[68] = cfg.de_burst_wrap_cmd;
        buf[69] = cfg.de_burst_wrap_cmd_dmy_clk;
        buf[70] = cfg.de_burst_wrap_data_mode;
        buf[71] = cfg.de_burst_wrap_data;
        [buf[72], buf[73]] = cfg.time_e_sector.to_le_bytes();
        [buf[74], buf[75]] = cfg.time_e_32k.to_le_bytes();
        [buf[76], buf[77]] = cfg.time_e_64k.to_le_bytes();
        [buf[78], buf[79]] = cfg.time_page_pgm.to_le_bytes();
        [buf[80], buf[81]] = cfg.time_ce.to_le_bytes();
        buf[82] = cfg.pd_delay;
        buf[83] = cfg.qe_data;

        let crc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&buf);

        HalFlashConfig {
            magic: 0x47464346,
            cfg,
            crc32,
        }
    }
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

/// Program or ROM code patches.
#[repr(C)]
pub struct HalPatchCfg {
    addr: u32,
    value: u32,
}

/// Flash configuration at boot-time.
#[cfg_attr(target_os = "none", unsafe(link_section = ".head.flash"))]
#[used]
pub static FLASH_CONFIG: HalFlashConfig = HalFlashConfig::new(SpiFlashCfgType {
    io_mode: 0x11,
    c_read_support: 0x00,
    clk_delay: 0x01,
    clk_invert: 0x01,
    reset_en_cmd: 0x66,
    reset_cmd: 0x99,
    reset_cread_cmd: 0xff,
    reset_cread_cmd_size: 0x03,
    jedec_id_cmd: 0x9f,
    jedec_id_cmd_dmy_clk: 0x00,
    enter_32_bits_addr_cmd: 0xb7,
    exit_32_bits_addr_cmd: 0xe9,
    sector_size: 0x04,
    mid: 0x00,
    page_size: 0x100,
    chip_erase_cmd: 0xc7,
    sector_erase_cmd: 0x20,
    blk32_erase_cmd: 0x52,
    blk64_erase_cmd: 0xd8,
    write_enable_cmd: 0x06,
    page_program_cmd: 0x02,
    qpage_program_cmd: 0x32,
    qpp_addr_mode: 0x00,
    fast_read_cmd: 0x0b,
    fr_dmy_clk: 0x01,
    qpi_fast_read_cmd: 0x0b,
    qpi_fr_dmy_clk: 0x01,
    fast_read_do_cmd: 0x3b,
    fr_do_dmy_clk: 0x01,
    fast_read_dio_cmd: 0xbb,
    fr_dio_dmy_clk: 0x00,
    fast_read_qo_cmd: 0x6b,
    fr_qo_dmy_clk: 0x01,
    fast_read_qio_cmd: 0xeb,
    fr_qio_dmy_clk: 0x02,
    qpi_fast_read_qio_cmd: 0xeb,
    qpi_fr_qio_dmy_clk: 0x02,
    qpi_page_program_cmd: 0x02,
    writev_reg_enable_cmd: 0x50,
    wr_enable_index: 0x00,
    qe_index: 0x01,
    busy_index: 0x00,
    wr_enable_bit: 0x01,
    qe_bit: 0x01,
    busy_bit: 0x00,
    wr_enable_read_reg_len: 0x01,
    wr_enable_write_reg_len: 0x02,
    qe_write_reg_len: 0x02,
    qe_read_reg_len: 0x01,
    release_power_down: 0xab,
    busy_read_reg_len: 0x01,
    read_reg_cmd: [0x05, 0x35, 0x00, 0x00],
    write_reg_cmd: [0x01, 0x01, 0x00, 0x00],
    enter_qpi: 0x38,
    exit_qpi: 0xff,
    c_read_mode: 0x20,
    cr_exit: 0xf0,
    burst_wrap_cmd: 0x77,
    burst_wrap_cmd_dmy_clk: 0x03,
    burst_wrap_data_mode: 0x02,
    burst_wrap_data: 0x40,
    de_burst_wrap_cmd: 0x77,
    de_burst_wrap_cmd_dmy_clk: 0x03,
    de_burst_wrap_data_mode: 0x02,
    de_burst_wrap_data: 0xf0,
    time_e_sector: 300,
    time_e_32k: 1200,
    time_e_64k: 1200,
    time_ce: 33000,
    time_page_pgm: 50,
    pd_delay: 20,
    qe_data: 0,
});

/// Decrypt-on-fly region length.
///
/// Fixed at 0 by now.
#[cfg_attr(target_os = "none", unsafe(link_section = ".head.base.aes-region"))]
pub static BASIC_AES_REGION: u32 = 0;

/// Image payload hash value.
///
/// It filles in 8 values of `0xdeadbeef` for we don't have method to emit
/// hash value in compilation stages. The real value should be filled by
/// following ROM image processing programs.
#[cfg_attr(target_os = "none", unsafe(link_section = ".head.base.hash"))]
pub static BASIC_HASH: [u32; 8] = [0xdeadbeef; 8];

/// Checksum of image header.
///
/// Real value should be fixed by ROM image processing programs.
#[cfg_attr(target_os = "none", unsafe(link_section = ".head.crc32"))]
pub static CRC32: u32 = 0xdeadbeef;

#[cfg(test)]
mod tests {
    use crate::{HalBasicConfig, HalFlashConfig, HalPatchCfg, SpiFlashCfgType};
    use memoffset::offset_of;

    #[test]
    fn struct_lengths() {
        use core::mem::size_of;
        assert_eq!(size_of::<HalFlashConfig>(), 92);
        assert_eq!(size_of::<HalBasicConfig>(), 48);
        assert_eq!(size_of::<HalPatchCfg>(), 8);
        assert_eq!(size_of::<SpiFlashCfgType>(), 84);
    }

    #[test]
    fn magic_crc32_hal_flash_config() {
        let test_spi_flash_config = SpiFlashCfgType {
            io_mode: 0x11,
            c_read_support: 0x00,
            clk_delay: 0x01,
            clk_invert: 0x01,
            reset_en_cmd: 0x66,
            reset_cmd: 0x99,
            reset_cread_cmd: 0xff,
            reset_cread_cmd_size: 0x03,
            jedec_id_cmd: 0x9f,
            jedec_id_cmd_dmy_clk: 0x00,
            enter_32_bits_addr_cmd: 0xb7,
            exit_32_bits_addr_cmd: 0xe9,
            sector_size: 0x04,
            mid: 0x00,
            page_size: 0x100,
            chip_erase_cmd: 0xc7,
            sector_erase_cmd: 0x20,
            blk32_erase_cmd: 0x52,
            blk64_erase_cmd: 0xd8,
            write_enable_cmd: 0x06,
            page_program_cmd: 0x02,
            qpage_program_cmd: 0x32,
            qpp_addr_mode: 0x00,
            fast_read_cmd: 0x0b,
            fr_dmy_clk: 0x01,
            qpi_fast_read_cmd: 0x0b,
            qpi_fr_dmy_clk: 0x01,
            fast_read_do_cmd: 0x3b,
            fr_do_dmy_clk: 0x01,
            fast_read_dio_cmd: 0xbb,
            fr_dio_dmy_clk: 0x00,
            fast_read_qo_cmd: 0x6b,
            fr_qo_dmy_clk: 0x01,
            fast_read_qio_cmd: 0xeb,
            fr_qio_dmy_clk: 0x02,
            qpi_fast_read_qio_cmd: 0xeb,
            qpi_fr_qio_dmy_clk: 0x02,
            qpi_page_program_cmd: 0x02,
            writev_reg_enable_cmd: 0x50,
            wr_enable_index: 0x00,
            qe_index: 0x01,
            busy_index: 0x00,
            wr_enable_bit: 0x01,
            qe_bit: 0x01,
            busy_bit: 0x00,
            wr_enable_read_reg_len: 0x01,
            wr_enable_write_reg_len: 0x02,
            qe_write_reg_len: 0x02,
            qe_read_reg_len: 0x01,
            release_power_down: 0xab,
            busy_read_reg_len: 0x01,
            read_reg_cmd: [0x05, 0x35, 0x00, 0x00],
            write_reg_cmd: [0x01, 0x01, 0x00, 0x00],
            enter_qpi: 0x38,
            exit_qpi: 0xff,
            c_read_mode: 0x20,
            cr_exit: 0xf0,
            burst_wrap_cmd: 0x77,
            burst_wrap_cmd_dmy_clk: 0x03,
            burst_wrap_data_mode: 0x02,
            burst_wrap_data: 0x40,
            de_burst_wrap_cmd: 0x77,
            de_burst_wrap_cmd_dmy_clk: 0x03,
            de_burst_wrap_data_mode: 0x02,
            de_burst_wrap_data: 0xf0,
            time_e_sector: 300,
            time_e_32k: 1200,
            time_e_64k: 1200,
            time_ce: 33000,
            time_page_pgm: 50,
            pd_delay: 20,
            qe_data: 0,
        };
        let test_config = HalFlashConfig::new(test_spi_flash_config);
        assert_eq!(test_config.magic, 0x47464346);
        assert_eq!(test_config.crc32, 0x482adef8);
    }

    #[test]
    fn struct_hal_flash_config_offset() {
        assert_eq!(offset_of!(HalFlashConfig, magic), 0x00);
        assert_eq!(offset_of!(HalFlashConfig, cfg), 0x04);
        assert_eq!(offset_of!(HalFlashConfig, crc32), 0x58);
    }

    #[test]
    fn struct_spi_flash_config_offset() {
        assert_eq!(offset_of!(SpiFlashCfgType, io_mode), 0x00);
        assert_eq!(offset_of!(SpiFlashCfgType, c_read_support), 0x01);
        assert_eq!(offset_of!(SpiFlashCfgType, clk_delay), 0x02);
        assert_eq!(offset_of!(SpiFlashCfgType, clk_invert), 0x03);
        assert_eq!(offset_of!(SpiFlashCfgType, reset_en_cmd), 0x04);
        assert_eq!(offset_of!(SpiFlashCfgType, reset_cmd), 0x05);
        assert_eq!(offset_of!(SpiFlashCfgType, reset_cread_cmd), 0x06);
        assert_eq!(offset_of!(SpiFlashCfgType, reset_cread_cmd_size), 0x07);
        assert_eq!(offset_of!(SpiFlashCfgType, jedec_id_cmd), 0x08);
        assert_eq!(offset_of!(SpiFlashCfgType, jedec_id_cmd_dmy_clk), 0x09);
        assert_eq!(offset_of!(SpiFlashCfgType, enter_32_bits_addr_cmd), 0x0a);
        assert_eq!(offset_of!(SpiFlashCfgType, exit_32_bits_addr_cmd), 0x0b);
        assert_eq!(offset_of!(SpiFlashCfgType, sector_size), 0x0c);
        assert_eq!(offset_of!(SpiFlashCfgType, mid), 0x0d);
        assert_eq!(offset_of!(SpiFlashCfgType, page_size), 0x0e);
        assert_eq!(offset_of!(SpiFlashCfgType, chip_erase_cmd), 0x10);
        assert_eq!(offset_of!(SpiFlashCfgType, sector_erase_cmd), 0x11);
        assert_eq!(offset_of!(SpiFlashCfgType, blk32_erase_cmd), 0x12);
        assert_eq!(offset_of!(SpiFlashCfgType, blk64_erase_cmd), 0x13);
        assert_eq!(offset_of!(SpiFlashCfgType, write_enable_cmd), 0x14);
        assert_eq!(offset_of!(SpiFlashCfgType, page_program_cmd), 0x15);
        assert_eq!(offset_of!(SpiFlashCfgType, qpage_program_cmd), 0x16);
        assert_eq!(offset_of!(SpiFlashCfgType, qpp_addr_mode), 0x17);
        assert_eq!(offset_of!(SpiFlashCfgType, fast_read_cmd), 0x18);
        assert_eq!(offset_of!(SpiFlashCfgType, fr_dmy_clk), 0x19);
        assert_eq!(offset_of!(SpiFlashCfgType, qpi_fast_read_cmd), 0x1a);
        assert_eq!(offset_of!(SpiFlashCfgType, qpi_fr_dmy_clk), 0x1b);
        assert_eq!(offset_of!(SpiFlashCfgType, fast_read_do_cmd), 0x1c);
        assert_eq!(offset_of!(SpiFlashCfgType, fr_do_dmy_clk), 0x1d);
        assert_eq!(offset_of!(SpiFlashCfgType, fast_read_dio_cmd), 0x1e);
        assert_eq!(offset_of!(SpiFlashCfgType, fr_dio_dmy_clk), 0x1f);
        assert_eq!(offset_of!(SpiFlashCfgType, fast_read_qo_cmd), 0x20);
        assert_eq!(offset_of!(SpiFlashCfgType, fr_qo_dmy_clk), 0x21);
        assert_eq!(offset_of!(SpiFlashCfgType, fast_read_qio_cmd), 0x22);
        assert_eq!(offset_of!(SpiFlashCfgType, fr_qio_dmy_clk), 0x23);
        assert_eq!(offset_of!(SpiFlashCfgType, qpi_fast_read_qio_cmd), 0x24);
        assert_eq!(offset_of!(SpiFlashCfgType, qpi_fr_qio_dmy_clk), 0x25);
        assert_eq!(offset_of!(SpiFlashCfgType, qpi_page_program_cmd), 0x26);
        assert_eq!(offset_of!(SpiFlashCfgType, writev_reg_enable_cmd), 0x27);
        assert_eq!(offset_of!(SpiFlashCfgType, wr_enable_index), 0x28);
        assert_eq!(offset_of!(SpiFlashCfgType, qe_index), 0x29);
        assert_eq!(offset_of!(SpiFlashCfgType, busy_index), 0x2a);
        assert_eq!(offset_of!(SpiFlashCfgType, wr_enable_bit), 0x2b);
        assert_eq!(offset_of!(SpiFlashCfgType, qe_bit), 0x2c);
        assert_eq!(offset_of!(SpiFlashCfgType, busy_bit), 0x2d);
        assert_eq!(offset_of!(SpiFlashCfgType, wr_enable_write_reg_len), 0x2e);
        assert_eq!(offset_of!(SpiFlashCfgType, wr_enable_read_reg_len), 0x2f);
        assert_eq!(offset_of!(SpiFlashCfgType, qe_write_reg_len), 0x30);
        assert_eq!(offset_of!(SpiFlashCfgType, qe_read_reg_len), 0x31);
        assert_eq!(offset_of!(SpiFlashCfgType, release_power_down), 0x32);
        assert_eq!(offset_of!(SpiFlashCfgType, busy_read_reg_len), 0x33);
        assert_eq!(offset_of!(SpiFlashCfgType, read_reg_cmd), 0x34);
        assert_eq!(offset_of!(SpiFlashCfgType, write_reg_cmd), 0x38);
        assert_eq!(offset_of!(SpiFlashCfgType, enter_qpi), 0x3c);
        assert_eq!(offset_of!(SpiFlashCfgType, exit_qpi), 0x3d);
        assert_eq!(offset_of!(SpiFlashCfgType, c_read_mode), 0x3e);
        assert_eq!(offset_of!(SpiFlashCfgType, cr_exit), 0x3f);
        assert_eq!(offset_of!(SpiFlashCfgType, burst_wrap_cmd), 0x40);
        assert_eq!(offset_of!(SpiFlashCfgType, burst_wrap_cmd_dmy_clk), 0x41);
        assert_eq!(offset_of!(SpiFlashCfgType, burst_wrap_data_mode), 0x42);
        assert_eq!(offset_of!(SpiFlashCfgType, burst_wrap_data), 0x43);
        assert_eq!(offset_of!(SpiFlashCfgType, de_burst_wrap_cmd), 0x44);
        assert_eq!(offset_of!(SpiFlashCfgType, de_burst_wrap_cmd_dmy_clk), 0x45);
        assert_eq!(offset_of!(SpiFlashCfgType, de_burst_wrap_data_mode), 0x46);
        assert_eq!(offset_of!(SpiFlashCfgType, de_burst_wrap_data), 0x47);
        assert_eq!(offset_of!(SpiFlashCfgType, time_e_sector), 0x48);
        assert_eq!(offset_of!(SpiFlashCfgType, time_e_32k), 0x4a);
        assert_eq!(offset_of!(SpiFlashCfgType, time_e_64k), 0x4c);
        assert_eq!(offset_of!(SpiFlashCfgType, time_page_pgm), 0x4e);
        assert_eq!(offset_of!(SpiFlashCfgType, time_ce), 0x50);
        assert_eq!(offset_of!(SpiFlashCfgType, pd_delay), 0x52);
        assert_eq!(offset_of!(SpiFlashCfgType, qe_data), 0x53);
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
    fn struct_hal_patch_cfg_offset() {
        assert_eq!(offset_of!(HalPatchCfg, addr), 0x00);
        assert_eq!(offset_of!(HalPatchCfg, value), 0x04);
    }
}
