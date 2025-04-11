use super::pad::Pads;
use super::register::{
    Argument, AutoCMDMode, BusVoltage, ClkGenMode, CmdType, Command, DataTransferMode, DmaMode,
    RegisterBlock,
};
use super::{Config, DmaConfig, DmaType, SdhResp, SdhTransFlag, parse_csd_v2, sleep_ms};
use crate::dma::{self, Dma, DmaChannel, LliPool, LliTransfer};
use crate::glb;
use core::ops::Deref;
use embedded_io::Write;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};

/// Managed Secure Digital Host Controller peripheral.
pub struct Sdh<'a, SDH, PADS, const I: usize, DMA, CH, const D: usize, const C: usize>
where
    DMA: Deref<Target = dma::RegisterBlock>,
    CH: DmaChannel<D, C>,
{
    sdh: SDH,
    pads: PADS,
    config: Config,
    system_dma: Option<Dma<'a, DMA, CH, D, C>>,
    dma_config: DmaConfig,
    block_count: u32,
}

impl<
    'a,
    SDH: Deref<Target = RegisterBlock>,
    PADS,
    const I: usize,
    DMA: Deref<Target = dma::RegisterBlock>,
    CH: DmaChannel<D, C>,
    const D: usize,
    const C: usize,
> Sdh<'a, SDH, PADS, I, DMA, CH, D, C>
{
    /// Create a new instance of the SDH peripheral.
    #[inline]
    pub fn new(sdh: SDH, pads: PADS, config: Config, glb: &glb::v2::RegisterBlock) -> Self
    where
        PADS: Pads<I>,
    {
        // Reset SDH peripheral.
        unsafe {
            sdh.software_reset.modify(|val| val.reset_all());
        }
        while !sdh.software_reset.read().is_reset_all_finished() {
            core::hint::spin_loop()
        }
        // Set SDH clock.
        unsafe {
            glb.sdh_config.modify(|val| {
                val.set_sdh_clk_sel(0) // GLB_REG_SDH_CLK_SEL.
                    .set_sdh_clk_div_len(7) // GLB_REG_SDH_CLK_DIV.
                    .enable_sdh_clk() // GLB_REG_SDH_CLK_EN.
            });
            sdh.clock_control.modify(|val| {
                val.set_sd_clk_freq(0) // SDH_SD_FREQ_SEL_LO.
                    .set_sd_clk_freq_upper(0) // SDH_SD_FREQ_SEL_HI.
                    .set_clk_gen_mode(ClkGenMode::DividedClk) // SDH_CLK_GEN_SEL.
                    .enable_internal_clk() // SDH_INT_CLK_EN.
                    .enable_sd_clk() // SDH_SD_CLK_EN.
            });
        }
        while !sdh.clock_control.read().is_sd_clk_enabled() {
            core::hint::spin_loop()
        }
        // Miscellaneous settings.
        unsafe {
            // SDH_DMA_EN.
            sdh.transfer_mode.modify(|val| val.disable_dma());
            sdh.host_control_1.modify(|val| {
                val.set_bus_width(config.bus_width_mode) // SDH_EX_DATA_WIDTH.
                    .set_transfer_width(config.transfer_width) // SDH_DATA_WIDTH.
                    .set_speed_mode(config.speed_mode) // SDH_HI_SPEED_EN.
                    .set_dma_mode(DmaMode::None)
            });
            // SDH_SD_BUS_VLT.
            sdh.power_control
                .modify(|val| val.set_bus_voltage(BusVoltage::V3_3));
            // SDH_TX_INT_CLK_SEL.
            sdh.tx_configuration.modify(|val| val.set_tx_int_clk_sel(1));
            // SDH enable interrupt.
            sdh.normal_interrupt_status_enable
                .modify(|val| val.enable_buffer_read_ready());
            // SDH_Set_Timeout.
            sdh.timeout_control.modify(|val| val.set_timeout_val(0x0e));
            // SDH_Powon.
            sdh.power_control.modify(|val| val.enable_bus_power());
        }
        Self {
            sdh,
            pads,
            config,
            system_dma: None,
            dma_config: DmaConfig::default(),
            block_count: 0,
        }
    }

    /// Initialize the SDH peripheral (enable debug to print card info).
    // TODO a more proper abstraction
    #[inline]
    pub fn init<W: Write>(&mut self, w: &mut W, debug: bool) {
        // Sdcard idle.
        loop {
            self.send_command(SdhResp::None, CmdType::Normal, 0, 0, false);
            sleep_ms(100);

            // Send CMD8.
            self.send_command(SdhResp::R7, CmdType::Normal, 8, 0x1AA, false);
            sleep_ms(100);
            let data = self.get_resp();
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
            self.send_command(SdhResp::R1, CmdType::Normal, 55, 0, false);
            sleep_ms(100);
            self.send_command(
                SdhResp::R3,
                CmdType::Normal,
                41,
                OCR_VOLTAGE_MASK & 0x00ff8000 | OCR_HCS,
                false,
            );
            sleep_ms(100);
            let ocr = self.get_resp();
            if (ocr as u32 & OCR_NBUSY) == OCR_NBUSY {
                break;
            }
            sleep_ms(100);
        }

        // Send CMD2 to get CID.
        self.send_command(SdhResp::R2, CmdType::Normal, 2, 0, false);
        sleep_ms(100);
        let cid = self.get_resp();
        if debug {
            writeln!(*w, "cid: {:#034X}", cid).ok();
        }

        // Send CMD3 to get RCA.
        self.send_command(SdhResp::R6, CmdType::Normal, 3, 0, false);
        sleep_ms(100);
        let rca = self.get_resp() as u32 >> 16;
        if debug {
            writeln!(*w, "rca: {:#010X}", rca).ok();
        }

        // Send CMD9 to get CSD.
        self.send_command(SdhResp::R2, CmdType::Normal, 9, rca << 16, false);
        sleep_ms(100);
        let csd_raw = self.get_resp();
        let (csd_structure, c_size) = parse_csd_v2(csd_raw);
        if csd_structure != 1 {
            writeln!(*w, "unexpected CSD: {:#034X}", csd_raw).ok();
            loop {}
        }
        if debug {
            writeln!(*w, "csd: {:#034X}, c_size: {}", csd_raw, c_size).ok();
        }

        let block_size = 512;
        self.block_count = (c_size + 1) * 1024;

        // Send CMD7 to select card.
        self.send_command(SdhResp::R1B, CmdType::Normal, 7, rca << 16, false);
        sleep_ms(100);

        // Set 1 data len, CMD55 -> ACMD6.
        self.send_command(SdhResp::R1, CmdType::Normal, 55, rca << 16, false);
        sleep_ms(100);
        self.send_command(SdhResp::R1, CmdType::Normal, 6, 0x0, false);
        sleep_ms(100);

        let kb_size = (self.block_count as f64) * (block_size as f64) / 1024.0;
        let mb_size = kb_size / 1024.0;
        let gb_size = mb_size / 1024.0;

        let cap = self.sdh.capabilities.read();
        let version = self.sdh.host_controller_version.read();

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
    }

    /// Send command to sdcard.
    #[inline]
    fn send_command(
        &self,
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
            self.sdh.argument.write(Argument(argument));
            self.sdh.command.write(
                Command((flag >> 16) as u16)
                    .set_cmd_type(cmd_type)
                    .set_cmd_idx(cmd_idx as u16),
            )
        }
    }

    /// Get response from sdcard.
    #[inline]
    fn get_resp(&self) -> u128 {
        self.sdh.response.read().response()
    }

    /// Enable dma using system dma controller.
    #[inline]
    pub fn enable_sys_dma(mut self, dma: Dma<'a, DMA, CH, D, C>) -> Self {
        self.system_dma = Some(dma);
        self.dma_config = self.dma_config.dma_type(DmaType::SystemDma);
        self
    }

    /// Read block from sdcard.
    #[inline]
    fn read_block(&self, block: &mut Block, block_idx: u32) {
        unsafe {
            // SDH_SD_TRANSFER_MODE.
            self.sdh.transfer_mode.modify(|val| {
                val.set_data_transfer_mode(DataTransferMode::MISO) // SDH_TO_HOST_DIR.
                    .set_auto_cmd_mode(AutoCMDMode::None) // SDH_AUTO_CMD_EN.
            });

            // Block_size.
            self.sdh
                .block_size
                .modify(|val| val.set_transfer_block(512));

            // Block_count.
            self.sdh.block_count.modify(|val| val.set_blocks_count(1));

            // SDH_ClearIntStatus(SDH_INT_BUFFER_READ_READY).
            self.sdh
                .normal_interrupt_status
                .modify(|val| val.clear_buffer_read_ready());
        }
        self.send_command(SdhResp::R1, CmdType::Normal, 17, block_idx, true);
        while !self
            .sdh
            .normal_interrupt_status
            .read()
            .is_buffer_read_ready()
        {
            // SDH_INT_BUFFER_READ_READY.
            // Wait for buffer read ready.
            core::hint::spin_loop()
        }
        for j in 0..Block::LEN / 4 {
            let val = self.sdh.buffer_data_port.read().buffer_data();
            block[j * 4 + 0] = (val >> 0) as u8;
            block[j * 4 + 1] = (val >> 8) as u8;
            block[j * 4 + 2] = (val >> 16) as u8;
            block[j * 4 + 3] = (val >> 24) as u8;
        }
    }

    /// Read block from sdcard using system dma controller.
    #[inline]
    fn read_block_sys_dma(&self, block: &mut Block, block_idx: u32) {
        unsafe {
            // SDH_SD_TRANSFER_MODE.
            self.sdh.transfer_mode.modify(|val| {
                val.set_data_transfer_mode(DataTransferMode::MISO) // SDH_TO_HOST_DIR.
                    .set_auto_cmd_mode(AutoCMDMode::None) // SDH_AUTO_CMD_EN.
            });

            // Block_size.
            self.sdh
                .block_size
                .modify(|val| val.set_transfer_block(512));

            // Block_count.
            self.sdh.block_count.modify(|val| val.set_blocks_count(1));

            // SDH_ClearIntStatus (SDH_INT_BUFFER_READ_READY).
            self.sdh
                .normal_interrupt_status
                .modify(|val| val.clear_buffer_read_ready());
        }
        self.send_command(SdhResp::R1, CmdType::Normal, 17, block_idx, true);
        while !self
            .sdh
            .normal_interrupt_status
            .read()
            .is_buffer_read_ready()
        {
            // SDH_INT_BUFFER_READ_READY.
            // Wait for buffer read ready.
            core::hint::spin_loop()
        }

        for j in 0..Block::LEN / 4 {
            let rx_lli_pool = &mut [LliPool::new(); 1];
            let val = &mut [0u8; 4];
            let rx_transfer = &mut [LliTransfer {
                src_addr: 0x20060020,
                dst_addr: val.as_mut_ptr() as u32,
                nbytes: 4,
            }];

            let sys_dma = self.system_dma.as_ref().unwrap();

            sys_dma.lli_reload(rx_lli_pool, 1, rx_transfer, 1);
            sys_dma.start();

            while sys_dma.is_busy() {
                core::hint::spin_loop();
            }

            sys_dma.stop();

            block[j * 4 + 0] = val[0];
            block[j * 4 + 1] = val[1];
            block[j * 4 + 2] = val[2];
            block[j * 4 + 3] = val[3];
        }
    }

    /// Release the SDH instance and return the pads and configs.
    #[inline]
    pub fn free(self) -> (SDH, PADS, Config, Dma<'a, DMA, CH, D, C>) {
        (self.sdh, self.pads, self.config, self.system_dma.unwrap())
    }
}

impl<
    'a,
    SDH: Deref<Target = RegisterBlock>,
    PADS,
    const I: usize,
    DMA: Deref<Target = dma::RegisterBlock>,
    CH: DmaChannel<D, C>,
    const D: usize,
    const C: usize,
> BlockDevice for Sdh<'a, SDH, PADS, I, DMA, CH, D, C>
{
    type Error = core::convert::Infallible;

    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        if self.dma_config.dma_type == DmaType::SystemDma {
            for (i, block) in blocks.iter_mut().enumerate() {
                self.read_block_sys_dma(block, start_block_idx.0 + i as u32);
            }
        } else {
            for (i, block) in blocks.iter_mut().enumerate() {
                self.read_block(block, start_block_idx.0 + i as u32);
            }
        }
        Ok(())
    }

    #[inline]
    fn write(&self, _blocks: &[Block], _start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        todo!();
    }

    #[inline]
    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(self.block_count))
    }
}
