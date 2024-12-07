//! sdh

use crate::*;
use embedded_io::Write;

const GLB_BASE: u32 = 0x20000000;
const SDH_BASE: u32 = 0x20060000;

pub struct MySdCard {
    block_count: u32,
}

impl MySdCard {
    fn read_block(&self, block: &mut Block, block_idx: u32) {
        let mut tmp_val = read_memory16(SDH_BASE + 0x0C); // SDH_SD_TRANSFER_MODE
        tmp_val = set_bits(tmp_val, 4, 1, 1); // SDH_TO_HOST_DIR
        tmp_val = set_bits(tmp_val, 2, 2, 0); // SDH_AUTO_CMD_EN
        tmp_val = set_bits(tmp_val, 5, 1, 0); // SDH_MULTI_BLK_SEL
        write_memory16(SDH_BASE + 0x0C, tmp_val);
        write_memory16(SDH_BASE + 0x04, 512); // block_size
        write_memory16(SDH_BASE + 0x06, 1); // block_count
        write_memory(SDH_BASE + 0x30, 0x00000020); // SDH_ClearIntStatus(SDH_INT_BUFFER_READ_READY)
        sdh_send_command(SDHResp::R1, SDHCmdType::Normal, 17, block_idx, true);
        tmp_val = read_memory(SDH_BASE + 0x30);
        while !is_bit_set(tmp_val, 5) {
            tmp_val = read_memory(SDH_BASE + 0x30);
        }
        for j in 0..Block::LEN / 4 {
            let val = read_memory(SDH_BASE + 0x20);
            block[j * 4 + 0] = (val >> 0) as u8;
            block[j * 4 + 1] = (val >> 8) as u8;
            block[j * 4 + 2] = (val >> 16) as u8;
            block[j * 4 + 3] = (val >> 24) as u8;
        }
    }
}

impl BlockDevice for MySdCard {
    type Error = core::convert::Infallible;
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            self.read_block(block, start_block_idx.0 + i as u32);
        }
        Ok(())
    }
    fn write(&self, _blocks: &[Block], _start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        unimplemented!();
    }
    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(self.block_count))
    }
}

pub fn sdh_init<W: Write>(w: &mut W) -> MySdCard {
    let mut tmp_val;

    // SDH_RESET
    tmp_val = read_memory16(SDH_BASE + 0x2e);
    tmp_val = set_bits(tmp_val, 8, 1, 1);
    write_memory16(SDH_BASE + 0x2e, tmp_val);
    while is_bit_set(tmp_val, 8) {
        tmp_val = read_memory16(SDH_BASE + 0x2e);
    }

    // GLB_Set_SDH_CLK
    tmp_val = read_memory(GLB_BASE + 0x430);
    tmp_val = set_bits(tmp_val, 13, 1, 0); // GLB_REG_SDH_CLK_EN
    write_memory(GLB_BASE + 0x430, tmp_val);
    tmp_val = read_memory(GLB_BASE + 0x430);
    tmp_val = set_bits(tmp_val, 12, 1, 0); // GLB_REG_SDH_CLK_SEL
    tmp_val = set_bits(tmp_val, 9, 3, 7); // GLB_REG_SDH_CLK_DIV
    write_memory(GLB_BASE + 0x430, tmp_val);
    let mut tmp_val = read_memory(GLB_BASE + 0x430);
    tmp_val = set_bits(tmp_val, 13, 1, 1); // GLB_REG_SDH_CLK_EN
    write_memory(GLB_BASE + 0x430, tmp_val);

    // SDH_Ctrl_Init
    tmp_val = read_memory16(SDH_BASE + 0x2c);
    tmp_val = set_bits(tmp_val, 8, 8, 0); // SDH_SD_FREQ_SEL_LO
    tmp_val = set_bits(tmp_val, 6, 2, 0); // SDH_SD_FREQ_SEL_HI
    tmp_val = set_bits(tmp_val, 5, 1, 0); // SDH_CLK_GEN_SEL
    tmp_val = set_bits(tmp_val, 0, 1, 1); // SDH_INT_CLK_EN
    tmp_val = set_bits(tmp_val, 2, 1, 1); // SDH_SD_CLK_EN
    write_memory16(SDH_BASE + 0x2c, tmp_val);
    tmp_val = read_memory16(SDH_BASE + 0x2c);
    while !is_bit_set(tmp_val, 1) {
        tmp_val = read_memory16(SDH_BASE + 0x2c);
    }
    tmp_val = read_memory16(SDH_BASE + 0xc);
    tmp_val = set_bits(tmp_val, 0, 1, 0); // SDH_DMA_EN
    write_memory16(SDH_BASE + 0xc, tmp_val);
    tmp_val = read_memory16(SDH_BASE + 0x28);
    tmp_val = set_bits(tmp_val, 5, 1, 0); // SDH_EX_DATA_WIDTH
    tmp_val = set_bits(tmp_val, 1, 1, 0); // SDH_DATA_WIDTH
    tmp_val = set_bits(tmp_val, 2, 1, 1); // SDH_HI_SPEED_EN
    tmp_val = set_bits(tmp_val, 9, 3, 7); // SDH_SD_BUS_VLT
    write_memory16(SDH_BASE + 0x28, tmp_val);
    tmp_val = read_memory(SDH_BASE + 0x118);
    tmp_val = set_bits(tmp_val, 30, 1, 1); // SDH_TX_INT_CLK_SEL
    write_memory(SDH_BASE + 0x118, tmp_val);

    // SDH Enable Interrupt
    tmp_val = 0;
    tmp_val = set_bits(tmp_val, 5, 1, 1); // SDH_INT_BUFFER_READ_READY
    write_memory(SDH_BASE + 0x34, tmp_val);

    // SDH_Set_Timeout
    tmp_val = read_memory16(SDH_BASE + 0x2e);
    tmp_val = set_bits(tmp_val, 0, 4, 0x0e); // SDH_TIMEOUT_VALUE
    write_memory16(SDH_BASE + 0x2e, tmp_val);

    // SDH_Powon
    tmp_val = read_memory16(SDH_BASE + 0x28);
    tmp_val = set_bits(tmp_val, 8, 1, 1); // SDH_SD_BUS_POWER
    write_memory16(SDH_BASE + 0x28, tmp_val);

    // sdcard idle
    sdh_send_command(SDHResp::None, SDHCmdType::Normal, 0, 0, false);
    sleep_ms(100);

    // send CMD8
    sdh_send_command(SDHResp::R7, SDHCmdType::Normal, 8, 0x1AA, false);
    sleep_ms(100);
    let data: u128 = sdh_get_resp();
    if data != 0x1AA {
        writeln!(
            *w,
            "unexpected response to CMD8: {:#010X}, expected 0x1AA",
            data
        )
        .ok();
        loop {}
    }

    loop {
        const OCR_NBUSY: u32 = 0x80000000;
        const OCR_VOLTAGE_MASK: u32 = 0x007FFF80;
        const OCR_HCS: u32 = 0x40000000;
        sdh_send_command(SDHResp::R1, SDHCmdType::Normal, 55, 0, false);
        sleep_ms(100);
        sdh_send_command(
            SDHResp::R3,
            SDHCmdType::Normal,
            41,
            OCR_VOLTAGE_MASK & 0x00ff8000 | OCR_HCS,
            false,
        );
        sleep_ms(100);
        let ocr = sdh_get_resp();
        if (ocr as u32 & OCR_NBUSY) == OCR_NBUSY {
            break;
        }
        sleep_ms(100);
    }

    // send CMD2 to get CID
    sdh_send_command(SDHResp::R2, SDHCmdType::Normal, 2, 0, false);
    sleep_ms(100);
    let cid = sdh_get_resp();
    writeln!(*w, "cid: {:#034X}", cid).ok();

    // send CMD3 to get RCA
    sdh_send_command(SDHResp::R6, SDHCmdType::Normal, 3, 0, false);
    sleep_ms(100);
    let rca = sdh_get_resp() as u32 >> 16;
    writeln!(*w, "rca: {:#010X}", rca).ok();

    // send CMD9 to get CSD
    sdh_send_command(SDHResp::R2, SDHCmdType::Normal, 9, rca << 16, false);
    sleep_ms(100);
    let csd_raw = sdh_get_resp();
    let (csd_structure, c_size) = parse_csd_v2(csd_raw);
    if csd_structure != 1 {
        writeln!(*w, "unexpected CSD: {:#034X}", csd_raw).ok();
        loop {}
    }

    writeln!(*w, "csd: {:#034X}, c_size: {}", csd_raw, c_size).ok();

    let block_size = 512;
    let block_count = (c_size + 1) * 1024;

    // send CMD7 to select card
    sdh_send_command(SDHResp::R1B, SDHCmdType::Normal, 7, rca << 16, false);
    sleep_ms(100);

    // set 1 data len, CMD55 -> ACMD6
    sdh_send_command(SDHResp::R1, SDHCmdType::Normal, 55, rca << 16, false);
    sleep_ms(100);
    sdh_send_command(SDHResp::R1, SDHCmdType::Normal, 6, 0x0, false);
    sleep_ms(100);

    writeln!(
        *w,
        "sdcard init done, size: {} MB",
        block_count as u128 * block_size / 1024 / 1024
    )
    .ok();
    MySdCard { block_count }
}

fn sdh_send_command(
    resp_type: SDHResp,
    cmd_type: SDHCmdType,
    cmd_idx: u32,
    argument: u32,
    has_data: bool,
) {
    let mut tmp_val;
    let mut flag = SDHTransFlag::None as u32;
    if has_data {
        flag |= SDHTransFlag::DataPresent as u32;
    }
    match resp_type {
        SDHResp::None => {}
        SDHResp::R1 | SDHResp::R5 | SDHResp::R6 | SDHResp::R7 => {
            flag |= SDHTransFlag::Resp48Bits as u32
                | SDHTransFlag::EnCrcCheck as u32
                | SDHTransFlag::EnIndexCheck as u32;
        }
        SDHResp::R1B | SDHResp::R5B => {
            flag |= SDHTransFlag::Resp48BitsWithBusy as u32
                | SDHTransFlag::EnCrcCheck as u32
                | SDHTransFlag::EnIndexCheck as u32;
        }
        SDHResp::R2 => {
            flag |= SDHTransFlag::Resp136Bits as u32 | SDHTransFlag::EnCrcCheck as u32;
        }
        SDHResp::R3 | SDHResp::R4 => {
            flag |= SDHTransFlag::Resp48Bits as u32;
        }
    }
    tmp_val = flag >> 16;

    tmp_val = set_bits(tmp_val, 6, 2, cmd_type as u32);
    tmp_val = set_bits(tmp_val, 8, 6, cmd_idx);
    write_memory(SDH_BASE + 0x08, argument);
    write_memory16(SDH_BASE + 0x0E, tmp_val);
}

fn sdh_get_resp() -> u128 {
    let a = read_memory(SDH_BASE + 0x10);
    let b = read_memory(SDH_BASE + 0x14);
    let c = read_memory(SDH_BASE + 0x18);
    let d = read_memory(SDH_BASE + 0x1C);
    (a as u128) | ((b as u128) << 32) | ((c as u128) << 64) | ((d as u128) << 96)
}

enum SDHCmdType {
    Normal,
    Suspend,
    Resume,
    Abort,
    Empty,
}

enum SDHTransFlag {
    None = 0x00000000,
    EnDma = 0x00000001,              // Enable DMA
    EnBlkCount = 0x00000002,         // Enable block count
    EnAutoCmd12 = 0x00000004,        // Enable auto CMD12
    EnAutoCmd23 = 0x00000008,        // Enable auto CMD23
    ReadData = 0x00000010,           // Enable read data
    MultiBlk = 0x00000020,           // Enable multi-block data operation
    Resp136Bits = 0x00010000,        // Response is 136 bits length
    Resp48Bits = 0x00020000,         // Response is 48 bits length
    Resp48BitsWithBusy = 0x00030000, // Response is 48 bits length with busy status
    EnCrcCheck = 0x00080000,         // Enable CRC check
    EnIndexCheck = 0x00100000,       // Enable index check
    DataPresent = 0x00200000,        // Data present
    Suspend = 0x00400000,            // Suspend command
    Resume = 0x00800000,             // Resume command
    Abort = 0x00C00000,              // Abort command
}

enum SDHResp {
    None,
    R1,
    R5,
    R6,
    R7,
    R1B,
    R5B,
    R2,
    R3,
    R4,
}

fn write_memory16(addr: u32, val: u32) {
    unsafe { ptr::write_volatile(addr as *mut u16, val as u16) }
}

fn read_memory16(addr: u32) -> u32 {
    unsafe { ptr::read_volatile(addr as *const u16) as u32 }
}

pub fn read_memory(addr: u32) -> u32 {
    unsafe { ptr::read_volatile(addr as *const u32) }
}

pub fn write_memory(addr: u32, val: u32) {
    unsafe { ptr::write_volatile(addr as *mut u32, val) }
}

fn parse_csd_v2(csd: u128) -> (u32, u32) {
    let csd_structure = (((csd >> (32 * 3)) & 0xC00000) >> 22) as u32;
    let c_size = (((csd >> 32) & 0x3FFFFF00) >> 8) as u32;
    (csd_structure, c_size)
}
