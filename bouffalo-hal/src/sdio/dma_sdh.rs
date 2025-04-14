use super::ops::{card_init, send_command};
use super::pad::Pads;
use super::register::{
    AutoCMDMode, BusVoltage, ClkGenMode, CmdType, DataTransferMode, DmaMode, RegisterBlock,
};
use super::{SdhResp, config::Config};
use crate::dma::{LliPool, LliTransfer, UntypedChannel};
use crate::glb;
use core::ops::Deref;
use core::sync::atomic::{Ordering, fence};
use embedded_io::Write;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};

/// Managed Secure Digital Host Controller peripheral.
pub struct Sdh<SDH, PADS, CH> {
    sdh: SDH,
    pads: PADS,
    dma_channel: CH,
    block_count: u32,
}

impl<'a, SDH: Deref<Target = RegisterBlock>, PADS, CH: Deref<Target = UntypedChannel<'a>>>
    Sdh<SDH, PADS, CH>
{
    /// Create a new instance of the SDH peripheral.
    #[inline]
    pub fn new<const I: usize>(
        sdh: SDH,
        pads: PADS,
        dma_channel: CH,
        config: Config,
        glb: &glb::v2::RegisterBlock,
    ) -> Self
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
            dma_channel,
            block_count: 0,
        }
    }

    /// Initialize the SDH peripheral (enable debug to print card info).
    // TODO a more proper abstraction
    #[inline]
    pub fn init<W: Write>(&mut self, w: &mut W, debug: bool) {
        self.block_count = card_init(&self.sdh, w, debug)
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
        send_command(&self.sdh, SdhResp::R1, CmdType::Normal, 17, block_idx, true);
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

            self.dma_channel.lli_reload(rx_lli_pool, 1, rx_transfer, 1);
            self.dma_channel.start();

            while self.dma_channel.is_busy() {
                core::hint::spin_loop();
            }

            self.dma_channel.stop();

            // FIXME modify to a proper fence
            fence(Ordering::AcqRel);

            block[j * 4 + 0] = val[0];
            block[j * 4 + 1] = val[1];
            block[j * 4 + 2] = val[2];
            block[j * 4 + 3] = val[3];
        }
    }

    /// Release the SDH instance and return the pads and configs.
    #[inline]
    pub fn free(self) -> (SDH, PADS, CH) {
        (self.sdh, self.pads, self.dma_channel)
    }
}

impl<'a, SDH: Deref<Target = RegisterBlock>, PADS, CH: Deref<Target = UntypedChannel<'a>>>
    BlockDevice for Sdh<SDH, PADS, CH>
{
    type Error = core::convert::Infallible;

    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            self.read_block_sys_dma(block, start_block_idx.0 + i as u32);
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
