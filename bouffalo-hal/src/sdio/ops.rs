use super::register::{Argument, AutoCMDMode, CmdType, Command, DataTransferMode, RegisterBlock};
use super::{SdhResp, SdhTransFlag};
use embedded_io::Write;
use embedded_sdmmc::Block;

#[inline]
pub(crate) fn read_block(sdh: &RegisterBlock, block: &mut Block, block_idx: u32) {
    unsafe {
        // SDH_SD_TRANSFER_MODE.
        sdh.transfer_mode.modify(|val| {
            val.set_data_transfer_mode(DataTransferMode::MISO) // SDH_TO_HOST_DIR.
                .set_auto_cmd_mode(AutoCMDMode::None) // SDH_AUTO_CMD_EN.
        });

        // Block_size.
        sdh.block_size.modify(|val| val.set_transfer_block(512));

        // Block_count.
        sdh.block_count.modify(|val| val.set_blocks_count(1));

        // SDH_ClearIntStatus(SDH_INT_BUFFER_READ_READY).
        sdh.normal_interrupt_status
            .modify(|val| val.clear_buffer_read_ready());
    }
    send_command(sdh, SdhResp::R1, CmdType::Normal, 17, block_idx, true);
    while !sdh.normal_interrupt_status.read().is_buffer_read_ready() {
        // SDH_INT_BUFFER_READ_READY.
        // Wait for buffer read ready.
        core::hint::spin_loop()
    }
    for j in 0..Block::LEN / 4 {
        let val = sdh.buffer_data_port.read().buffer_data();
        block[j * 4 + 0] = (val >> 0) as u8;
        block[j * 4 + 1] = (val >> 8) as u8;
        block[j * 4 + 2] = (val >> 16) as u8;
        block[j * 4 + 3] = (val >> 24) as u8;
    }
}

/// Send command to sdcard.
#[inline]
pub(crate) fn send_command(
    sdh: &RegisterBlock,
    resp_type: SdhResp,
    cmd_type: CmdType,
    cmd_idx: u32,
    argument: u32,
    has_data: bool,
) {
    let mut flag = SdhTransFlag::None as u32;
    if has_data {
        flag |= SdhTransFlag::DataPresent as u32;
    }
    match resp_type {
        SdhResp::None => {}
        SdhResp::R1 | SdhResp::R5 | SdhResp::R6 | SdhResp::R7 => {
            flag |= SdhTransFlag::Resp48Bits as u32
                | SdhTransFlag::EnCrcCheck as u32
                | SdhTransFlag::EnIndexCheck as u32;
        }
        SdhResp::R1B | SdhResp::R5B => {
            flag |= SdhTransFlag::Resp48BitsWithBusy as u32
                | SdhTransFlag::EnCrcCheck as u32
                | SdhTransFlag::EnIndexCheck as u32;
        }
        SdhResp::R2 => {
            flag |= SdhTransFlag::Resp136Bits as u32 | SdhTransFlag::EnCrcCheck as u32;
        }
        SdhResp::R3 | SdhResp::R4 => {
            flag |= SdhTransFlag::Resp48Bits as u32;
        }
    }

    unsafe {
        sdh.argument.write(Argument(argument));
        sdh.command.write(
            Command((flag >> 16) as u16)
                .set_cmd_type(cmd_type)
                .set_cmd_idx(cmd_idx as u16),
        )
    }
}

/// Returns the block_count of SD card.
#[inline]
pub(crate) fn card_init<W: Write>(sdh: &RegisterBlock, w: &mut W, debug: bool) -> u32 {
    // Sdcard idle.
    loop {
        send_command(sdh, SdhResp::None, CmdType::Normal, 0, 0, false);
        sleep_ms(100);

        // Send CMD8.
        send_command(sdh, SdhResp::R7, CmdType::Normal, 8, 0x1AA, false);
        sleep_ms(100);
        let data = get_resp(sdh);
        if data != 0x1AA {
            writeln!(
                *w,
                "unexpected response to CMD8: {:#010X}, expected 0x1AA",
                data
            )
            .ok();
        } else {
            break;
        }
        sleep_ms(1000);
    }

    loop {
        const OCR_NBUSY: u32 = 0x80000000;
        const OCR_VOLTAGE_MASK: u32 = 0x007FFF80;
        const OCR_HCS: u32 = 0x40000000;
        send_command(sdh, SdhResp::R1, CmdType::Normal, 55, 0, false);
        sleep_ms(100);
        send_command(
            sdh,
            SdhResp::R3,
            CmdType::Normal,
            41,
            OCR_VOLTAGE_MASK & 0x00ff8000 | OCR_HCS,
            false,
        );
        sleep_ms(100);
        let ocr = get_resp(sdh);
        if (ocr as u32 & OCR_NBUSY) == OCR_NBUSY {
            break;
        }
        sleep_ms(100);
    }

    // Send CMD2 to get CID.
    send_command(sdh, SdhResp::R2, CmdType::Normal, 2, 0, false);
    sleep_ms(100);
    let cid = get_resp(sdh);
    if debug {
        writeln!(*w, "cid: {:#034X}", cid).ok();
    }

    // Send CMD3 to get RCA.
    send_command(sdh, SdhResp::R6, CmdType::Normal, 3, 0, false);
    sleep_ms(100);
    let rca = get_resp(sdh) as u32 >> 16;
    if debug {
        writeln!(*w, "rca: {:#010X}", rca).ok();
    }

    // Send CMD9 to get CSD.
    send_command(sdh, SdhResp::R2, CmdType::Normal, 9, rca << 16, false);
    sleep_ms(100);
    let csd_raw = get_resp(sdh);
    let (csd_structure, c_size) = parse_csd_v2(csd_raw);
    if csd_structure != 1 {
        writeln!(*w, "unexpected CSD: {:#034X}", csd_raw).ok();
        loop {}
    }
    if debug {
        writeln!(*w, "csd: {:#034X}, c_size: {}", csd_raw, c_size).ok();
    }

    let block_size = 512;
    let block_count = (c_size + 1) * 1024;

    // Send CMD7 to select card.
    send_command(sdh, SdhResp::R1B, CmdType::Normal, 7, rca << 16, false);
    sleep_ms(100);

    // Set 1 data len, CMD55 -> ACMD6.
    send_command(sdh, SdhResp::R1, CmdType::Normal, 55, rca << 16, false);
    sleep_ms(100);
    send_command(sdh, SdhResp::R1, CmdType::Normal, 6, 0x0, false);
    sleep_ms(100);

    let kb_size = (block_count as f64) * (block_size as f64) / 1024.0;
    let mb_size = kb_size / 1024.0;
    let gb_size = mb_size / 1024.0;

    let cap = sdh.capabilities.read();
    let version = sdh.host_controller_version.read();

    writeln!(*w, "SpecifiicVersion: {:?}", version.specific_version()).ok();
    writeln!(*w, "SlotType: {:?}", cap.slot_type()).ok();
    writeln!(*w, "SDMA support: {}", cap.is_sdma_supported()).ok();
    writeln!(*w, "ADMA2 support: {}", cap.is_adma2_supported()).ok();

    if debug {
        if kb_size < 1024.0 {
            writeln!(*w, "sdcard init done, size: {:.2} KB", kb_size).ok();
        } else if mb_size < 1024.0 {
            writeln!(*w, "sdcard init done, size: {:.2} MB", mb_size).ok();
        } else {
            writeln!(*w, "sdcard init done, size: {:.2} GB", gb_size).ok();
        }
    }

    block_count
}

#[inline]
fn get_resp(sdh: &RegisterBlock) -> u128 {
    sdh.response.read().response()
}

/// Sleep for n milliseconds.
fn sleep_ms(n: u32) {
    for _ in 0..n * 125 {
        unsafe { core::arch::asm!("nop") }
    }
}

/// Parse CSD version 2.0.
#[inline]
fn parse_csd_v2(csd: u128) -> (u32, u32) {
    let csd_structure = (((csd >> (32 * 3)) & 0xC00000) >> 22) as u32;
    let c_size = (((csd >> 32) & 0x3FFFFF00) >> 8) as u32;
    (csd_structure, c_size)
}
