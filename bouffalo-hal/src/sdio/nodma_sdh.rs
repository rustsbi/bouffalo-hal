use super::config::Config;
use super::ops::{card_init, read_block};
use super::pad::Pads;
use super::register::{BusVoltage, ClkGenMode, DmaMode, RegisterBlock};
use crate::glb;
use core::ops::Deref;
use embedded_io::Write;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};

/// Managed Secure Digital Host Controller peripheral.
pub struct Sdh<SDH, PADS> {
    sdh: SDH,
    pads: PADS,
    block_count: u32,
}

impl<SDH: Deref<Target = RegisterBlock>, PADS> Sdh<SDH, PADS> {
    /// Create a new instance of the SDH peripheral.
    #[inline]
    pub fn new<const I: usize>(
        sdh: SDH,
        pads: PADS,
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
            block_count: 0,
        }
    }

    /// Initialize the SDH peripheral (enable debug to print card info).
    // TODO a more proper abstraction
    #[inline]
    pub fn init<W: Write>(&mut self, w: &mut W, debug: bool) {
        self.block_count = card_init(&self.sdh, w, debug)
    }

    /// Release the SDH instance and return the pads and configs.
    #[inline]
    pub fn free(self) -> (SDH, PADS) {
        (self.sdh, self.pads)
    }
}

impl<SDH: Deref<Target = RegisterBlock>, PADS> BlockDevice for Sdh<SDH, PADS> {
    type Error = core::convert::Infallible;

    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            read_block(&self.sdh, block, start_block_idx.0 + i as u32);
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
