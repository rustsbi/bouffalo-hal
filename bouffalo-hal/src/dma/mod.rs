//! Direct Memory Access peripheral.

mod channel;
mod config;
mod register;

pub use channel::*;
pub use config::*;
pub use register::*;

use crate::glb;

/// Peripheral instance for DMA.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// DMA peripheral data register address definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaAddr {
    Uart0Tx = 0x2000A000 + 0x88,
    Uart0Rx = 0x2000A000 + 0x8C,
    Uart1Tx = 0x2000A100 + 0x88,
    Uart1Rx = 0x2000A100 + 0x8C,
    Uart2Tx = 0x2000AA00 + 0x88,
    Uart2Rx = 0x2000AA00 + 0x8C,
    Uart3Tx = 0x30002000 + 0x88,
    Uart3Rx = 0x30002000 + 0x8C,
    I2c0Tx = 0x2000A300 + 0x88,
    I2c0Rx = 0x2000A300 + 0x8C,
    I2c1Tx = 0x2000A900 + 0x88,
    I2c1Rx = 0x2000A900 + 0x8C,
    I2c2Tx = 0x30003000 + 0x88,
    I2c2Rx = 0x30003000 + 0x8C,
    I2c3Tx = 0x30004000 + 0x88,
    I2c3Rx = 0x30004000 + 0x8C,
    Spi0Tx = 0x2000A200 + 0x88,
    Spi0Rx = 0x2000A200 + 0x8C,
    Spi1Tx = 0x30008000 + 0x88,
    Spi1Rx = 0x30008000 + 0x8C,
    I2sTx = 0x2000AB00 + 0x88,
    I2sRx = 0x2000AB00 + 0x8C,
    AdcRx = 0x20002000 + 0x04,
    DacTx = 0x20002000 + 0x48,
    IrTx = 0x2000A600 + 0x88,
    WoTx = 0x20000000 + 0xB04,
}

/// Extend constructor to DMA ownership structures.
pub trait DmaExt {
    type Group;
    fn split(self, glb: &glb::v2::RegisterBlock) -> Self::Group;
}

/// Linked list item pool descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C, align(32))]
pub struct LliPool {
    /// Source address.
    pub src_addr: u32,
    /// Destination address.
    pub dst_addr: u32,
    /// Physical address to next linked list item.
    pub next_lli: u32,
    /// Linked list item control register.
    pub control: LliControl,
}

impl LliPool {
    #[inline]
    pub fn new() -> Self {
        Self {
            src_addr: 0,
            dst_addr: 0,
            next_lli: 0,
            control: LliControl::default(),
        }
    }
}
