//! Secure Digital Input/Output peripheral.

mod config;
mod dma_sdh;
mod nodma_sdh;
mod ops;
mod pad;
mod register;
pub mod sdcard;
pub use config::*;
pub use dma_sdh::*;
pub use pad::*;
pub use register::*;

/// SDH peripheral type without system dma.
pub type NonSysDmaSdh<SDH, PADS> = nodma_sdh::Sdh<SDH, PADS>;
