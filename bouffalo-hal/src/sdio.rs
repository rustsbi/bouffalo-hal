//! Secure Digital Input/Output peripheral.

mod config;
mod dma_sdh;
mod nodma_sdh;
mod ops;
mod pad;
mod register;
pub use config::*;
pub use dma_sdh::*;
pub use pad::*;
pub use register::*;

/// SDH transfer flag.
// TODO remove allow(dead_code)
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SdhTransFlag {
    None = 0x00000000,
    EnDma = 0x00000001,              // Enable DMA.
    EnBlkCount = 0x00000002,         // Enable block count.
    EnAutoCmd12 = 0x00000004,        // Enable auto CMD12.
    EnAutoCmd23 = 0x00000008,        // Enable auto CMD23.
    ReadData = 0x00000010,           // Enable read data.
    MultiBlk = 0x00000020,           // Enable multi-block data operation.
    Resp136Bits = 0x00010000,        // Response is 136 bits length.
    Resp48Bits = 0x00020000,         // Response is 48 bits length.
    Resp48BitsWithBusy = 0x00030000, // Response is 48 bits length with busy status.
    EnCrcCheck = 0x00080000,         // Enable crc check.
    EnIndexCheck = 0x00100000,       // Enable index check.
    DataPresent = 0x00200000,        // Data present.
    Suspend = 0x00400000,            // Suspend command.
    Resume = 0x00800000,             // Resume command.
    Abort = 0x00C00000,              // Abort command.
}

/// SDH response type.
// TODO construct R5, R5B, R4 responses, remove allow(dead_code)
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
enum SdhResp {
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

/// Dma type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaType {
    /// Blocking read / write.
    Disabled,
    /// Use system dma controller to transmit.
    SystemDma,
}

/// SDH dma config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaConfig {
    dma_type: DmaType,
}

impl DmaConfig {
    /// Default dma config.
    #[inline]
    pub const fn default() -> Self {
        Self {
            dma_type: DmaType::Disabled,
        }
    }

    /// Set dma type.
    #[inline]
    pub const fn dma_type(mut self, dma_type: DmaType) -> Self {
        self.dma_type = dma_type;
        self
    }
}

/// SDH peripheral type without system dma.
pub type NonSysDmaSdh<SDH, PADS> = nodma_sdh::Sdh<SDH, PADS>;
