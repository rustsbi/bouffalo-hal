use crate::{dma::UntypedChannel, sdio::NonSysDmaSdh, sdio::RegisterBlock, sdio::dma_sdh::Sdh};
use core::ops::Deref;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};

/// A block device that uses the SDIO interface.
pub trait InnerSdh<'a> {
    /// Read one block at the given block index.
    fn sdh_read_block(&self, block: &mut Block, block_idx: u32);
    /// Write one block at the given block index.
    fn sdh_write_block(&self, block: &Block, block_idx: u32);
    /// Determine how many blocks this device can hold.
    fn sdh_num_blocks(&self) -> embedded_sdmmc::BlockCount;
}

impl<'a, SDH, PADS, CH> InnerSdh<'a> for Sdh<SDH, PADS, CH>
where
    SDH: Deref<Target = RegisterBlock>,
    CH: Deref<Target = UntypedChannel<'a>>,
{
    #[inline]
    fn sdh_read_block(&self, block: &mut Block, block_idx: u32) {
        self.read_block_sys_dma(block, block_idx);
    }
    #[inline]
    fn sdh_write_block(&self, block: &Block, block_idx: u32) {
        self.write_block_sys_dma(block, block_idx);
    }
    #[inline]
    fn sdh_num_blocks(&self) -> embedded_sdmmc::BlockCount {
        self.num_blocks()
    }
}

impl<'a, SDH, PADS> InnerSdh<'a> for NonSysDmaSdh<SDH, PADS>
where
    SDH: Deref<Target = RegisterBlock>,
{
    #[inline]
    fn sdh_read_block(&self, block: &mut Block, block_idx: u32) {
        self.read_block(block, block_idx);
    }
    #[inline]
    fn sdh_write_block(&self, block: &Block, block_idx: u32) {
        self.write_block(block, block_idx);
    }
    #[inline]
    fn sdh_num_blocks(&self) -> embedded_sdmmc::BlockCount {
        self.num_blocks()
    }
}

/// SD card instance using sdh.
pub struct Sdcard<'a, T: InnerSdh<'a>> {
    pub sdh: &'a mut T,
}

impl<'a, T: InnerSdh<'a>> Sdcard<'a, T> {
    /// Create a new SD card instance.
    pub fn new(sdh: &'a mut T) -> Self {
        Self { sdh }
    }

    /// Free the SD card.
    pub fn free(self) -> &'a mut T {
        self.sdh
    }
}

impl<'a, T: InnerSdh<'a>> BlockDevice for Sdcard<'a, T> {
    type Error = core::convert::Infallible;

    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            self.sdh.sdh_read_block(block, start_block_idx.0 + i as u32);
        }
        Ok(())
    }

    #[inline]
    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter().enumerate() {
            self.sdh
                .sdh_write_block(block, start_block_idx.0 + i as u32);
        }
        Ok(())
    }

    #[inline]
    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(self.sdh.sdh_num_blocks())
    }
}
