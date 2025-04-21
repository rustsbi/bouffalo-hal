use super::register::{BurstSize, DmaMode, TransferWidth};

/// Direct Memory Access channel configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DmaChannelConfig<T> {
    pub direction: DmaMode,
    pub src_req: Option<T>,
    pub dst_req: Option<T>,
    pub src_addr_inc: bool,
    pub dst_addr_inc: bool,
    pub src_burst_size: BurstSize,
    pub dst_burst_size: BurstSize,
    pub src_transfer_width: TransferWidth,
    pub dst_transfer_width: TransferWidth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Mem2MemChannelConfig {
    pub direction: DmaMode,
    pub src_addr_inc: bool,
    pub dst_addr_inc: bool,
    pub src_burst_size: BurstSize,
    pub dst_burst_size: BurstSize,
    pub src_transfer_width: TransferWidth,
    pub dst_transfer_width: TransferWidth,
}

/// Peripheral for DMA 0/1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Periph4Dma01 {
    /// UART0 receive.
    Uart0Rx,
    /// UART0 transmit.
    Uart0Tx,
    /// UART1 receive.
    Uart1Rx,
    /// UART1 transmit.
    Uart1Tx,
    /// UART2 receive.
    Uart2Rx,
    /// UART2 transmit.
    Uart2Tx,
    /// I2C0 receive.
    I2c0Rx,
    /// I2C0 transmit.
    I2c0Tx,
    /// IR transmit.
    IrTx,
    /// GPIO transmit.
    GpioTx,
    /// SPI0 receive.
    Spi0Rx,
    /// SPI0 transmit.
    Spi0Tx,
    /// AUDIO receive.
    AudioRx,
    /// AUDIO transmit.
    AudioTx,
    /// I2C1 receive.
    I2c1Rx,
    /// I2C1 transmit.
    I2c1Tx,
    /// I2S receive.
    I2sRx,
    /// I2S transmit.
    I2sTx,
    /// PDM receive.
    PdmRx,
    /// GPADC.
    GpAdc = 22,
    /// GPDAC.
    GpDac,
}

/// Peripheral for DMA 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Periph4Dma2 {
    /// UART3 receive.
    Uart3Rx,
    /// UART3 transmit.
    Uart3Tx,
    /// SPI1 receive.
    Spi1Rx,
    /// SPI1 transmit.
    Spi1Tx,
    /// I2C2 receive.
    I2c2Rx = 6,
    /// I2C2 transmit.
    I2c2Tx,
    /// I2C3 receive.
    I2c3Rx,
    /// I2C3 transmit.
    I2c3Tx,
    /// DSI receive.
    DsiRx,
    /// DSI transmit.
    DsiTx,
    /// DBI receive.
    DbiTx = 22,
}

pub trait PeripheralId {
    fn id(&self) -> u8;
}

impl PeripheralId for Periph4Dma01 {
    #[inline]
    fn id(&self) -> u8 {
        *self as u8
    }
}
impl PeripheralId for Periph4Dma2 {
    #[inline]
    fn id(&self) -> u8 {
        *self as u8
    }
}
