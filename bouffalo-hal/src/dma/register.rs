use super::{
    PeripheralId,
    config::{Periph4Dma01, Periph4Dma2},
};
use volatile_register::{RO, RW, WO};

/// Direct Memory Access peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt register block.
    pub interrupts: InterruptRegisters,
    /// Channel enable states.
    pub enabled_channels: RO<EnabledChannels>,
    _reserved0: [u8; 0x3],
    /// Software burst request.
    pub soft_burst_request: RW<u32>,
    /// Software single request.
    pub soft_single_request: RW<u32>,
    /// Software last burst request.
    pub soft_last_burst_request: RW<u32>,
    /// Software last single request.
    pub soft_last_single_request: RW<u32>,
    /// Peripheral configuration register.
    pub global_config: RW<GlobalConfig>,
    /// DMA synchronization logic for DMA requests.
    pub dma_sync: RW<u32>,
    _reserved1: [u8; 0xc8],
    /// Channel registers block.
    pub channels: [ChannelRegisters; 8],
}

/// Interrupt register block.
#[repr(C)]
pub struct InterruptRegisters {
    /// Global interrupt state after masking.
    pub global_state: RO<GlobalState>,
    _reserved0: [u8; 3],
    /// Transfer complete interrupt state.
    pub transfer_complete_state: RO<TransferCompleteState>,
    _reserved1: [u8; 3],
    /// Clear transfer complete interrupt.
    pub transfer_complete_clear: WO<TransferCompleteClear>,
    _reserved2: [u8; 3],
    /// Error interrupt state.
    pub error_state: RO<ErrorState>,
    _reserved3: [u8; 3],
    /// Clear error interrupt.
    pub error_clear: WO<ErrorClear>,
    _reserved4: [u8; 3],
    /// Transfer complete interrupt state before masking.
    pub raw_transfer_complete: RO<RawTransferComplete>,
    _reserved5: [u8; 3],
    /// Error interrupt state before masking.
    pub raw_error: RO<RawError>,
    _reserved6: [u8; 3],
}

/// Global interrupt state after masking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlobalState(u8);

impl GlobalState {
    /// Check if channel interrupt occurs.
    #[inline]
    pub const fn if_int_occurs(self, ch: u8) -> bool {
        ((self.0 >> ch) & 1) != 0
    }
}

/// Transfer complete interrupt state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransferCompleteState(u8);

impl TransferCompleteState {
    /// Check if channel complete interrupt occurs.
    #[inline]
    pub const fn if_cplt_int_occurs(self, ch: u8) -> bool {
        ((self.0 >> ch) & 1) != 0
    }
}

/// Clear transfer complete interrupt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TransferCompleteClear(u8);

impl TransferCompleteClear {
    /// Clear channel complete interrupt states.
    #[inline]
    pub const fn clear_cplt_int(self, ch: u8) -> Self {
        Self(self.0 | (1 << ch))
    }
}

/// Error interrupt state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ErrorState(u8);

impl ErrorState {
    /// Check if channel error interrupt occurs.
    #[inline]
    pub const fn if_err_int_occurs(self, ch: u8) -> bool {
        ((self.0 >> ch) & 1) != 0
    }
}

/// Clear error interrupt.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct ErrorClear(u8);

impl ErrorClear {
    /// Clear channel complete interrupt states.
    #[inline]
    pub const fn clear_err_int(self, ch: u8) -> Self {
        Self(self.0 | (1 << ch))
    }
}

/// Transfer complete interrupt state before masking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RawTransferComplete(u8);

impl RawTransferComplete {
    /// Check if raw channel complete interrupt occurs.
    #[inline]
    pub const fn if_raw_cplt_int_occurs(self, ch: u8) -> bool {
        ((self.0 >> ch) & 1) != 0
    }
}

/// Error interrupt state before masking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RawError(u8);

impl RawError {
    /// Check if channel error interrupt occurs.
    #[inline]
    pub const fn if_raw_error_occurs(self, ch: u8) -> bool {
        ((self.0 >> ch) & 1) != 0
    }
}

/// Channel enable states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnabledChannels(u8);

impl EnabledChannels {
    /// Check if channel interrupt is enabled.
    #[inline]
    pub const fn is_ch_enabled(self, ch: u8) -> bool {
        ((self.0 >> ch) & 1) != 0
    }
}

/// Peripheral configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlobalConfig(u32);

/// AHB master endian mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndianMode {
    LittleEndian,
    BigEndian,
}

impl GlobalConfig {
    const AHB_MASTER_ENDIAN_CFG: u32 = 0x1 << 1;
    const DMA: u32 = 0x1;

    /// Set AHB master endian mode.
    #[inline]
    pub const fn set_ahb_master_endian_mode(self, mode: EndianMode) -> Self {
        Self(
            (self.0 & !Self::AHB_MASTER_ENDIAN_CFG)
                | (Self::AHB_MASTER_ENDIAN_CFG & ((mode as u32) << 1)),
        )
    }
    /// Get AHB master endian mode.
    #[inline]
    pub const fn ahb_master_endian_mode(self) -> EndianMode {
        match (self.0 & Self::AHB_MASTER_ENDIAN_CFG) >> 1 {
            0 => EndianMode::LittleEndian,
            _ => EndianMode::BigEndian,
        }
    }
    /// Enable DMA.
    #[inline]
    pub const fn enable_dma(self) -> Self {
        Self((self.0 & !Self::DMA) | 1)
    }
    /// Disable DMA.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        Self((self.0 & !Self::DMA) | 0)
    }
    /// Check if DMA is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        (self.0 & Self::DMA) != 0
    }
}

/// Channel registers block.
#[repr(C)]
pub struct ChannelRegisters {
    /// Source address.
    pub source_address: RW<u32>,
    /// Destination address.
    pub destination_address: RW<u32>,
    /// Physical address to first linked list item.
    pub linked_list_item: RW<u32>,
    /// Linked list item control register.
    pub control: RW<LliControl>,
    /// Channel configuration register.
    pub config: RW<ChannelConfig>,
    _reserved0: [u8; 0xec],
}

/// Linked list item pool descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
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
            control: LliControl(0),
        }
    }
}

/// Linked list item transfer descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LliTransfer {
    /// Source address.
    pub src_addr: u32,
    /// Destination address.
    pub dst_addr: u32,
    /// How many bytes should be transferred.
    pub nbytes: u32,
}

/// Control register in linked list item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LliControl(u32);

/// DMA transfer width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferWidth {
    /// Transfer 1 byte (8 bits) at a time.
    Byte,
    /// Transfer 2 byte (16 bits) at a time.
    HalfWord,
    /// Transfer 4 byte (32 bits) at a time.
    Word,
    /// Transfer 8 byte (64 bits) at a time.
    DoubleWord,
}

/// DMA burst size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BurstSize {
    /// Transfer 1 data unit at a time.
    INCR1,
    /// Transfer 4 data units at a time.
    INCR4,
    /// Transfer 8 data units at a time.
    INCR8,
    /// Transfer 16 data units at a time.
    INCR16,
}

impl LliControl {
    const CPLT_INT_EN: u32 = 0x1 << 31;
    const DST_ADDR_INC_EN: u32 = 0x1 << 27;
    const SRC_ADDR_INC_EN: u32 = 0x1 << 26;
    const FIX_CNT: u32 = 0x7 << 23;
    const DST_TRANSFER_WIDTH: u32 = 0x3 << 21;
    const SRC_TRANSFER_WIDTH: u32 = 0x3 << 18;
    const DST_ADD_MODE: u32 = 0x1 << 17;
    const DST_BST_SIZE: u32 = 0x3 << 15;
    const DST_MIN_MODE: u32 = 0x1 << 14;
    const SRC_BST_SIZE: u32 = 0x3 << 12;
    const TRANSFER_SIZE: u32 = 0xFFF;

    /// Enable completion interrupt.
    #[inline]
    pub const fn enable_cplt_int(self) -> Self {
        Self((self.0 & !Self::CPLT_INT_EN) | (1 << 31))
    }
    /// Disable completion interrupt.
    #[inline]
    pub const fn disable_cplt_int(self) -> Self {
        Self((self.0 & !Self::CPLT_INT_EN) | (0 << 31))
    }
    /// Check if completion interrupt is enabled.
    #[inline]
    pub const fn is_cplt_int_enabled(self) -> bool {
        ((self.0 & Self::CPLT_INT_EN) >> 31) != 0
    }
    /// Enable destination address increment.
    #[inline]
    pub const fn enable_dst_addr_inc(self) -> Self {
        Self((self.0 & !Self::DST_ADDR_INC_EN) | (1 << 27))
    }
    /// Disable destination address increment.
    #[inline]
    pub const fn disable_dst_addr_inc(self) -> Self {
        Self((self.0 & !Self::DST_ADDR_INC_EN) | (0 << 27))
    }
    /// Check if destination address increment is enabled.
    #[inline]
    pub const fn is_dst_addr_inc_enabled(self) -> bool {
        ((self.0 & Self::DST_ADDR_INC_EN) >> 27) != 0
    }
    /// Enable source address increment.
    #[inline]
    pub const fn enable_src_addr_inc(self) -> Self {
        Self((self.0 & !Self::SRC_ADDR_INC_EN) | (1 << 26))
    }
    /// Disable source address increment.
    #[inline]
    pub const fn disable_src_addr_inc(self) -> Self {
        Self((self.0 & !Self::SRC_ADDR_INC_EN) | (0 << 26))
    }
    /// Check if source address increment is enabled.
    #[inline]
    pub const fn is_src_addr_inc_enabled(self) -> bool {
        ((self.0 & Self::SRC_ADDR_INC_EN) >> 26) != 0
    }
    /// Set fixed count.
    #[inline]
    pub const fn set_fix_cnt(self, cnt: u8) -> Self {
        Self((self.0 & !Self::FIX_CNT) | ((cnt as u32) << 23))
    }
    /// Get fixed count.
    #[inline]
    pub const fn fix_cnt(self) -> u8 {
        ((self.0 & Self::FIX_CNT) >> 23) as u8
    }
    /// Set destination transfer width.
    #[inline]
    pub const fn set_dst_transfer_width(self, width: TransferWidth) -> Self {
        Self((self.0 & !Self::DST_TRANSFER_WIDTH) | ((width as u32) << 21))
    }
    /// Get destination transfer width.
    #[inline]
    pub const fn dst_transfer_width(self) -> TransferWidth {
        match ((self.0 & Self::DST_TRANSFER_WIDTH) >> 21) as u8 {
            0 => TransferWidth::Byte,
            1 => TransferWidth::HalfWord,
            2 => TransferWidth::Word,
            _ => TransferWidth::DoubleWord,
        }
    }
    /// Set source transfer width.
    #[inline]
    pub const fn set_src_transfer_width(self, width: TransferWidth) -> Self {
        Self((self.0 & !Self::SRC_TRANSFER_WIDTH) | ((width as u32) << 18))
    }
    /// Get source transfer width.
    #[inline]
    pub const fn src_transfer_width(self) -> TransferWidth {
        match ((self.0 & Self::SRC_TRANSFER_WIDTH) >> 18) as u8 {
            0 => TransferWidth::Byte,
            1 => TransferWidth::HalfWord,
            2 => TransferWidth::Word,
            _ => TransferWidth::DoubleWord,
        }
    }
    /// Enable destination address mode (issue remain destination traffic).
    #[inline]
    pub const fn enable_dst_add_mode(self) -> Self {
        Self((self.0 & !Self::DST_ADD_MODE) | (1 << 17))
    }
    /// Disable destination address mode (issue remain destination traffic).
    #[inline]
    pub const fn disable_dst_add_mode(self) -> Self {
        Self((self.0 & !Self::DST_ADD_MODE) | (0 << 17))
    }
    /// Check if destination add mode (issue remain destination traffic) is enabled.
    #[inline]
    pub const fn is_dst_add_mode_enabled(self) -> bool {
        ((self.0 & Self::DST_ADD_MODE) >> 17) != 0
    }
    /// Set destination burst size.
    #[inline]
    pub const fn set_dst_bst_size(self, size: BurstSize) -> Self {
        Self((self.0 & !Self::DST_BST_SIZE) | ((size as u32) << 15))
    }
    /// Get destination burst size.
    #[inline]
    pub const fn dst_bst_size(self) -> BurstSize {
        match ((self.0 & Self::DST_BST_SIZE) >> 15) as u8 {
            0 => BurstSize::INCR1,
            1 => BurstSize::INCR4,
            2 => BurstSize::INCR8,
            _ => BurstSize::INCR16,
        }
    }
    /// Enable destination min mode (Not issue all destination traffic).
    #[inline]
    pub const fn enable_dst_min_mode(self) -> Self {
        Self((self.0 & !Self::DST_MIN_MODE) | (1 << 14))
    }
    /// Disable destination min mode (Not issue all destination traffic).
    #[inline]
    pub const fn disable_dst_min_mode(self) -> Self {
        Self((self.0 & !Self::DST_MIN_MODE) | (0 << 14))
    }
    /// Check if destination min mode (Not issue all destination traffic) is enabled.
    #[inline]
    pub const fn is_dst_min_mode_enabled(self) -> bool {
        ((self.0 & Self::DST_MIN_MODE) >> 14) != 0
    }
    /// Set source burst size.
    #[inline]
    pub const fn set_src_bst_size(self, size: BurstSize) -> Self {
        Self((self.0 & !Self::SRC_BST_SIZE) | ((size as u32) << 12))
    }
    /// Get source burst size.
    #[inline]
    pub const fn src_bst_size(self) -> BurstSize {
        match ((self.0 & Self::SRC_BST_SIZE) >> 12) as u8 {
            0 => BurstSize::INCR1,
            1 => BurstSize::INCR4,
            2 => BurstSize::INCR8,
            _ => BurstSize::INCR16,
        }
    }
    /// Set transfer size.
    #[inline]
    pub const fn set_transfer_size(self, size: u16) -> Self {
        Self((self.0 & !Self::TRANSFER_SIZE) | (size as u32))
    }
    /// Get transfer size.
    #[inline]
    pub const fn transfer_size(self) -> u16 {
        (self.0 & Self::TRANSFER_SIZE) as u16
    }
}

/// Channel configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelConfig(u32);

/// DMA transfer mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaMode {
    /// Memory to memory (DMA).
    /// Notice: When the DMA mode is set to memory to memory, the source and destination address must be incremented.
    Mem2Mem,
    /// Peripheral to memory (DMA).
    Mem2Periph,
    /// Memory to peripheral (DMA).
    Periph2Mem,
    /// Source peripheral to destination peripheral (DMA).
    Periph2Periph,
    /// Source peripheral to destination peripheral (destination peripheral).
    Periph2PeriphCtrlByDst,
    /// Memory to peripheral (peripheral).
    Mem2PeriphCtrlByPeriph,
    /// Peripheral to memory (peripheral).
    Periph2MemCtrlByPeriph,
    /// Source peripheral to destination peripheral (source peripheral).
    Periph2PeriphCtrlBySrc,
}

/// DMA peripheral request definition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaPeriphReq {
    /// Dma request for peripheral for DMA 0/1.
    Dma01(Periph4Dma01),
    /// Dma request for peripheral for DMA 2.
    Dma2(Periph4Dma2),
    /// No dma request for peripheral .
    None,
}

impl ChannelConfig {
    const LLI_CNT: u32 = 0x3FF << 20;
    const HALT: u32 = 0x1 << 18;
    const ACTIVE: u32 = 0x1 << 17;
    const LOCK: u32 = 0x1 << 16;
    const CPLT_INT_EN: u32 = 0x1 << 15;
    const ERR_INT_EN: u32 = 0x1 << 14;
    const FLW_CTRL: u32 = 0x7 << 11;
    const DST_PERIPH: u32 = 0x1F << 6;
    const SRC_PERIPH: u32 = 0x1F << 1;
    const CH_EN: u32 = 0x1;

    /// Get link list item count.
    #[inline]
    pub const fn lli_cnt(self) -> u16 {
        ((self.0 & Self::LLI_CNT) >> 20) as u16
    }
    /// Set link list item count.
    #[inline]
    pub const fn set_lli_cnt(self, cnt: u16) -> Self {
        Self((self.0 & !Self::LLI_CNT) | ((cnt as u32) << 20))
    }
    /// Stop DMA.
    #[inline]
    pub const fn stop_dma(self) -> Self {
        Self((self.0 & !Self::HALT) | (1 << 18))
    }
    /// Resume DMA.
    #[inline]
    pub const fn resume_dma(self) -> Self {
        Self((self.0 & !Self::HALT) | (0 << 18))
    }
    /// Check if DMA is stopped.
    #[inline]
    pub const fn is_dma_stopped(self) -> bool {
        ((self.0 & Self::HALT) >> 18) != 0
    }
    /// Check if FIFO is empty.
    #[inline]
    pub const fn is_fifo_empty(self) -> bool {
        ((self.0 & Self::ACTIVE) >> 17) == 0
    }
    /// Lock DMA.
    #[inline]
    pub const fn lock_dma(self) -> Self {
        Self((self.0 & !Self::LOCK) | (1 << 16))
    }
    /// Unlock DMA.
    #[inline]
    pub const fn unlock_dma(self) -> Self {
        Self((self.0 & !Self::LOCK) | (0 << 16))
    }
    /// Check if DMA is locked.
    #[inline]
    pub const fn is_dma_locked(self) -> bool {
        ((self.0 & Self::LOCK) >> 16) != 0
    }
    /// Enable completion interrupt.
    #[inline]
    pub const fn enable_cplt_int(self) -> Self {
        Self((self.0 & !Self::CPLT_INT_EN) | (1 << 15))
    }
    /// Disable completion interrupt.
    #[inline]
    pub const fn disable_cplt_int(self) -> Self {
        Self((self.0 & !Self::CPLT_INT_EN) | (0 << 15))
    }
    /// Check if completion interrupt is enabled.
    #[inline]
    pub const fn is_cplt_int_enabled(self) -> bool {
        ((self.0 & Self::CPLT_INT_EN) >> 15) != 0
    }
    /// Enable error interrupt.
    #[inline]
    pub const fn enable_err_int(self) -> Self {
        Self((self.0 & !Self::ERR_INT_EN) | (1 << 14))
    }
    /// Disable error interrupt.
    #[inline]
    pub const fn disable_err_int(self) -> Self {
        Self((self.0 & !Self::ERR_INT_EN) | (0 << 14))
    }
    /// Check if error interrupt is enabled.
    #[inline]
    pub const fn is_err_int_enabled(self) -> bool {
        ((self.0 & Self::ERR_INT_EN) >> 14) != 0
    }
    /// Set DMA mode.
    #[inline]
    pub const fn set_dma_mode(self, mode: DmaMode) -> Self {
        Self((self.0 & !Self::FLW_CTRL) | ((mode as u32) << 11))
    }
    /// Get DMA mode.
    #[inline]
    pub const fn dma_mode(self) -> DmaMode {
        match ((self.0 & Self::FLW_CTRL) >> 11) as u8 {
            0 => DmaMode::Mem2Mem,
            1 => DmaMode::Mem2Periph,
            2 => DmaMode::Periph2Mem,
            3 => DmaMode::Periph2Periph,
            4 => DmaMode::Periph2PeriphCtrlByDst,
            5 => DmaMode::Mem2PeriphCtrlByPeriph,
            6 => DmaMode::Periph2MemCtrlByPeriph,
            _ => DmaMode::Periph2PeriphCtrlBySrc,
        }
    }
    /// Set destination peripheral for any DMA.
    #[inline]
    pub fn set_dst_periph(self, periph: impl PeripheralId) -> Self {
        Self((self.0 & !Self::DST_PERIPH) | ((periph.id() as u32) << 6))
    }
    /// Clear destination peripheral for any DMA.
    #[inline]
    pub fn clear_dst_periph(self) -> Self {
        Self(self.0 & !Self::DST_PERIPH)
    }
    /// Set destination peripheral for DMA 0/1.
    #[inline]
    pub const fn set_dst_periph4dma01(self, periph: Periph4Dma01) -> Self {
        Self((self.0 & !Self::DST_PERIPH) | (Self::DST_PERIPH & ((periph as u32) << 6)))
    }
    /// Set destination peripheral for DMA2.
    #[inline]
    pub const fn set_dst_periph4dma2(self, periph: Periph4Dma2) -> Self {
        Self((self.0 & !Self::DST_PERIPH) | (Self::DST_PERIPH & ((periph as u32) << 6)))
    }
    /// Get destination peripheral for DMA 0/1.
    #[inline]
    pub const fn dst_periph4dma01(self) -> Periph4Dma01 {
        match ((self.0 & Self::DST_PERIPH) >> 6) as u8 {
            0 => Periph4Dma01::Uart0Rx,
            1 => Periph4Dma01::Uart0Tx,
            2 => Periph4Dma01::Uart1Rx,
            3 => Periph4Dma01::Uart1Tx,
            4 => Periph4Dma01::Uart2Rx,
            5 => Periph4Dma01::Uart2Tx,
            6 => Periph4Dma01::I2c0Rx,
            7 => Periph4Dma01::I2c0Tx,
            8 => Periph4Dma01::IrTx,
            9 => Periph4Dma01::GpioTx,
            10 => Periph4Dma01::Spi0Rx,
            11 => Periph4Dma01::Spi0Tx,
            12 => Periph4Dma01::AudioRx,
            13 => Periph4Dma01::AudioTx,
            14 => Periph4Dma01::I2c1Rx,
            15 => Periph4Dma01::I2c1Tx,
            16 => Periph4Dma01::I2sRx,
            17 => Periph4Dma01::I2sTx,
            18 => Periph4Dma01::PdmRx,
            22 => Periph4Dma01::GpAdc,
            23 => Periph4Dma01::GpDac,
            _ => unreachable!(),
        }
    }
    /// Get destination peripheral for DMA2.
    #[inline]
    pub const fn dst_periph4dma2(self) -> Periph4Dma2 {
        match ((self.0 & Self::DST_PERIPH) >> 6) as u8 {
            0 => Periph4Dma2::Uart3Rx,
            1 => Periph4Dma2::Uart3Tx,
            2 => Periph4Dma2::Spi1Rx,
            3 => Periph4Dma2::Spi1Tx,
            6 => Periph4Dma2::I2c2Rx,
            7 => Periph4Dma2::I2c2Tx,
            8 => Periph4Dma2::I2c3Rx,
            9 => Periph4Dma2::I2c3Tx,
            10 => Periph4Dma2::DsiRx,
            11 => Periph4Dma2::DsiTx,
            22 => Periph4Dma2::DbiTx,
            _ => unreachable!(),
        }
    }
    /// Set source peripheral for any DMA.
    #[inline]
    pub fn set_src_periph(self, periph: impl PeripheralId) -> Self {
        Self((self.0 & !Self::SRC_PERIPH) | ((periph.id() as u32) << 1))
    }
    /// Clear source peripheral for any DMA.
    #[inline]
    pub fn clear_src_periph(self) -> Self {
        Self(self.0 & !Self::SRC_PERIPH)
    }
    /// Set source peripheral for DMA 0/1.
    #[inline]
    pub const fn set_src_periph4dma01(self, periph: Periph4Dma01) -> Self {
        Self((self.0 & !Self::SRC_PERIPH) | ((periph as u32) << 1))
    }
    /// Set source peripheral for DMA2.
    #[inline]
    pub const fn set_src_periph4dma2(self, periph: Periph4Dma2) -> Self {
        Self((self.0 & !Self::SRC_PERIPH) | ((periph as u32) << 1))
    }
    /// Get source peripheral for DMA 0/1.
    #[inline]
    pub const fn src_periph4dma01(self) -> Periph4Dma01 {
        match ((self.0 & Self::SRC_PERIPH) >> 1) as u8 {
            0 => Periph4Dma01::Uart0Rx,
            1 => Periph4Dma01::Uart0Tx,
            2 => Periph4Dma01::Uart1Rx,
            3 => Periph4Dma01::Uart1Tx,
            4 => Periph4Dma01::Uart2Rx,
            5 => Periph4Dma01::Uart2Tx,
            6 => Periph4Dma01::I2c0Rx,
            7 => Periph4Dma01::I2c0Tx,
            8 => Periph4Dma01::IrTx,
            9 => Periph4Dma01::GpioTx,
            10 => Periph4Dma01::Spi0Rx,
            11 => Periph4Dma01::Spi0Tx,
            12 => Periph4Dma01::AudioRx,
            13 => Periph4Dma01::AudioTx,
            14 => Periph4Dma01::I2c1Rx,
            15 => Periph4Dma01::I2c1Tx,
            16 => Periph4Dma01::I2sRx,
            17 => Periph4Dma01::I2sTx,
            18 => Periph4Dma01::PdmRx,
            22 => Periph4Dma01::GpAdc,
            23 => Periph4Dma01::GpDac,
            _ => unreachable!(),
        }
    }
    /// Get source peripheral for DMA2.
    #[inline]
    pub const fn src_periph4dma2(self) -> Periph4Dma2 {
        match ((self.0 & Self::SRC_PERIPH) >> 1) as u8 {
            0 => Periph4Dma2::Uart3Rx,
            1 => Periph4Dma2::Uart3Tx,
            2 => Periph4Dma2::Spi1Rx,
            3 => Periph4Dma2::Spi1Tx,
            6 => Periph4Dma2::I2c2Rx,
            7 => Periph4Dma2::I2c2Tx,
            8 => Periph4Dma2::I2c3Rx,
            9 => Periph4Dma2::I2c3Tx,
            10 => Periph4Dma2::DsiRx,
            11 => Periph4Dma2::DsiTx,
            22 => Periph4Dma2::DbiTx,
            _ => unreachable!(),
        }
    }
    /// Enable channel.
    #[inline]
    pub const fn enable_ch(self) -> Self {
        Self((self.0 & !Self::CH_EN) | 1)
    }
    /// Disable channel.
    #[inline]
    pub const fn disable_ch(self) -> Self {
        Self((self.0 & !Self::CH_EN) | 0)
    }
    /// Check if channel is enabled.
    #[inline]
    pub const fn is_ch_enabled(self) -> bool {
        (self.0 & Self::CH_EN) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BurstSize, ChannelConfig, ChannelRegisters, DmaMode, EnabledChannels, EndianMode,
        ErrorClear, ErrorState, GlobalConfig, GlobalState, InterruptRegisters, LliControl,
        Periph4Dma01, Periph4Dma2, RawError, RawTransferComplete, RegisterBlock,
        TransferCompleteClear, TransferCompleteState, TransferWidth,
    };
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, interrupts), 0x00);
        assert_eq!(offset_of!(RegisterBlock, enabled_channels), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, soft_burst_request), 0x20);
        assert_eq!(offset_of!(RegisterBlock, soft_single_request), 0x24);
        assert_eq!(offset_of!(RegisterBlock, soft_last_burst_request), 0x28);
        assert_eq!(offset_of!(RegisterBlock, soft_last_single_request), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, global_config), 0x30);
        assert_eq!(offset_of!(RegisterBlock, channels), 0x100);
    }

    #[rustfmt::skip]
    #[test]
    fn struct_interrupt_registers_offset() {
        assert_eq!(offset_of!(InterruptRegisters, global_state), 0x00);
        assert_eq!(offset_of!(InterruptRegisters, transfer_complete_state), 0x04);
        assert_eq!(offset_of!(InterruptRegisters, transfer_complete_clear), 0x08);
        assert_eq!(offset_of!(InterruptRegisters, error_state), 0x0c);
        assert_eq!(offset_of!(InterruptRegisters, error_clear), 0x10);
        assert_eq!(offset_of!(InterruptRegisters, raw_transfer_complete), 0x14);
        assert_eq!(offset_of!(InterruptRegisters, raw_error), 0x18);
    }

    #[test]
    fn struct_channel_registers_offset_size() {
        assert_eq!(offset_of!(ChannelRegisters, source_address), 0x00);
        assert_eq!(offset_of!(ChannelRegisters, destination_address), 0x04);
        assert_eq!(offset_of!(ChannelRegisters, linked_list_item), 0x08);
        assert_eq!(offset_of!(ChannelRegisters, control), 0x0c);
        assert_eq!(offset_of!(ChannelRegisters, config), 0x10);
        assert_eq!(core::mem::size_of::<ChannelRegisters>(), 0x100);
    }

    #[test]
    fn struct_interrupt_registers_function() {
        let val = GlobalState(0x10);
        assert!(val.if_int_occurs(4));

        let val = TransferCompleteState(0x10);
        assert!(val.if_cplt_int_occurs(4));

        let val = TransferCompleteClear(0x0).clear_cplt_int(4);
        assert_eq!(val.0, 0x10);

        let val = ErrorState(0x10);
        assert!(val.if_err_int_occurs(4));

        let val = ErrorClear(0x0).clear_err_int(4);
        assert_eq!(val.0, 0x10);

        let val = RawTransferComplete(0x10);
        assert!(val.if_raw_cplt_int_occurs(4));

        let val = RawError(0x10);
        assert!(val.if_raw_error_occurs(4));
    }

    #[test]
    fn struct_enable_channels_function() {
        let val = EnabledChannels(0x10);
        assert!(val.is_ch_enabled(4));
    }

    #[test]
    fn struct_global_config_functions() {
        let mut val = GlobalConfig(0x0);
        val = val.set_ahb_master_endian_mode(EndianMode::BigEndian);
        assert_eq!(val.ahb_master_endian_mode(), EndianMode::BigEndian);
        assert_eq!(val.0, 0x00000002);
        val = val.set_ahb_master_endian_mode(EndianMode::LittleEndian);
        assert_eq!(val.ahb_master_endian_mode(), EndianMode::LittleEndian);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_dma();
        assert!(val.is_dma_enabled());
        assert_eq!(val.0, 0x00000001);
        val = val.disable_dma();
        assert!(!val.is_dma_enabled());
        assert_eq!(val.0, 0x00000000);
    }

    #[test]
    fn struct_channel_registers_functions() {
        let mut val = LliControl(0x0);
        val = val.enable_cplt_int();
        assert!(val.is_cplt_int_enabled());
        assert_eq!(val.0, 0x80000000);
        val = val.disable_cplt_int();
        assert!(!val.is_cplt_int_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_dst_addr_inc();
        assert!(val.is_dst_addr_inc_enabled());
        assert_eq!(val.0, 0x08000000);
        val = val.disable_dst_addr_inc();
        assert!(!val.is_dst_addr_inc_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_src_addr_inc();
        assert!(val.is_src_addr_inc_enabled());
        assert_eq!(val.0, 0x04000000);
        val = val.disable_src_addr_inc();
        assert!(!val.is_src_addr_inc_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_fix_cnt(0x7);
        assert_eq!(val.fix_cnt(), 0x7);
        assert_eq!(val.0, 0x03800000);

        val = LliControl(0x0);
        val = val.set_dst_transfer_width(TransferWidth::DoubleWord);
        assert_eq!(val.dst_transfer_width(), TransferWidth::DoubleWord);
        assert_eq!(val.0, 0x00600000);
        val = val.set_dst_transfer_width(TransferWidth::Word);
        assert_eq!(val.dst_transfer_width(), TransferWidth::Word);
        assert_eq!(val.0, 0x00400000);
        val = val.set_dst_transfer_width(TransferWidth::HalfWord);
        assert_eq!(val.dst_transfer_width(), TransferWidth::HalfWord);
        assert_eq!(val.0, 0x00200000);
        val = val.set_dst_transfer_width(TransferWidth::Byte);
        assert_eq!(val.dst_transfer_width(), TransferWidth::Byte);
        assert_eq!(val.0, 0x00000000);

        val = val.set_src_transfer_width(TransferWidth::DoubleWord);
        assert_eq!(val.src_transfer_width(), TransferWidth::DoubleWord);
        assert_eq!(val.0, 0x000C0000);
        val = val.set_src_transfer_width(TransferWidth::Word);
        assert_eq!(val.src_transfer_width(), TransferWidth::Word);
        assert_eq!(val.0, 0x00080000);
        val = val.set_src_transfer_width(TransferWidth::HalfWord);
        assert_eq!(val.src_transfer_width(), TransferWidth::HalfWord);
        assert_eq!(val.0, 0x00040000);
        val = val.set_src_transfer_width(TransferWidth::Byte);
        assert_eq!(val.src_transfer_width(), TransferWidth::Byte);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_dst_add_mode();
        assert!(val.is_dst_add_mode_enabled());
        assert_eq!(val.0, 0x00020000);
        val = val.disable_dst_add_mode();
        assert!(!val.is_dst_add_mode_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_dst_bst_size(BurstSize::INCR16);
        assert_eq!(val.dst_bst_size(), BurstSize::INCR16);
        assert_eq!(val.0, 0x00018000);
        val = val.set_dst_bst_size(BurstSize::INCR8);
        assert_eq!(val.dst_bst_size(), BurstSize::INCR8);
        assert_eq!(val.0, 0x00010000);
        val = val.set_dst_bst_size(BurstSize::INCR4);
        assert_eq!(val.dst_bst_size(), BurstSize::INCR4);
        assert_eq!(val.0, 0x00008000);
        val = val.set_dst_bst_size(BurstSize::INCR1);
        assert_eq!(val.dst_bst_size(), BurstSize::INCR1);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_dst_min_mode();
        assert!(val.is_dst_min_mode_enabled());
        assert_eq!(val.0, 0x00004000);
        val = val.disable_dst_min_mode();
        assert!(!val.is_dst_min_mode_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_src_bst_size(BurstSize::INCR16);
        assert_eq!(val.src_bst_size(), BurstSize::INCR16);
        assert_eq!(val.0, 0x00003000);
        val = val.set_src_bst_size(BurstSize::INCR8);
        assert_eq!(val.src_bst_size(), BurstSize::INCR8);
        assert_eq!(val.0, 0x00002000);
        val = val.set_src_bst_size(BurstSize::INCR4);
        assert_eq!(val.src_bst_size(), BurstSize::INCR4);
        assert_eq!(val.0, 0x00001000);
        val = val.set_src_bst_size(BurstSize::INCR1);
        assert_eq!(val.src_bst_size(), BurstSize::INCR1);
        assert_eq!(val.0, 0x00000000);

        val = val.set_transfer_size(0x7FF);
        assert_eq!(val.transfer_size(), 0x7FF);
        assert_eq!(val.0, 0x000007FF);
    }

    #[test]
    fn struct_channel_config_functions() {
        let mut val = ChannelConfig(0x0);
        val = val.set_lli_cnt(0x3FF);
        assert_eq!(val.lli_cnt(), 0x3FF);
        assert_eq!(val.0, 0x3FF00000);

        val = ChannelConfig(0x0);
        val = val.stop_dma();
        assert!(val.is_dma_stopped());
        assert_eq!(val.0, 0x00040000);
        val = val.resume_dma();
        assert!(!val.is_dma_stopped());
        assert_eq!(val.0, 0x00000000);

        val = ChannelConfig(0x00020000);
        assert!(!val.is_fifo_empty());
        val = ChannelConfig(0x0);
        assert!(val.is_fifo_empty());

        val = val.lock_dma();
        assert!(val.is_dma_locked());
        assert_eq!(val.0, 0x00010000);
        val = val.unlock_dma();
        assert!(!val.is_dma_locked());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_cplt_int();
        assert!(val.is_cplt_int_enabled());
        assert_eq!(val.0, 0x00008000);
        val = val.disable_cplt_int();
        assert!(!val.is_cplt_int_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_err_int();
        assert!(val.is_err_int_enabled());
        assert_eq!(val.0, 0x00004000);
        val = val.disable_err_int();
        assert!(!val.is_err_int_enabled());
        assert_eq!(val.0, 0x00000000);

        // The number 'i' is not related to the actual register, but only to make the code more concise.
        for i in 0..8 as u8 {
            let tmp_mode = match i {
                0 => DmaMode::Mem2Mem,
                1 => DmaMode::Mem2Periph,
                2 => DmaMode::Periph2Mem,
                3 => DmaMode::Periph2Periph,
                4 => DmaMode::Periph2PeriphCtrlByDst,
                5 => DmaMode::Mem2PeriphCtrlByPeriph,
                6 => DmaMode::Periph2MemCtrlByPeriph,
                _ => DmaMode::Periph2PeriphCtrlBySrc,
            };
            let tmp_val = match i {
                0 => 0x00000000,
                1 => 0x00000800,
                2 => 0x00001000,
                3 => 0x00001800,
                4 => 0x00002000,
                5 => 0x00002800,
                6 => 0x00003000,
                7 => 0x00003800,
                _ => unreachable!(),
            };
            val = val.set_dma_mode(tmp_mode);
            assert_eq!(val.dma_mode(), tmp_mode);
            assert_eq!(val.0, tmp_val);
        }

        val = ChannelConfig(0x0);
        // The number 'i' is not related to the actual register, but only to make the code more concise.
        for i in 0..21 as u8 {
            let tmp_periph = match i {
                0 => Periph4Dma01::Uart0Rx,
                1 => Periph4Dma01::Uart0Tx,
                2 => Periph4Dma01::Uart1Rx,
                3 => Periph4Dma01::Uart1Tx,
                4 => Periph4Dma01::Uart2Rx,
                5 => Periph4Dma01::Uart2Tx,
                6 => Periph4Dma01::I2c0Rx,
                7 => Periph4Dma01::I2c0Tx,
                8 => Periph4Dma01::IrTx,
                9 => Periph4Dma01::GpioTx,
                10 => Periph4Dma01::Spi0Rx,
                11 => Periph4Dma01::Spi0Tx,
                12 => Periph4Dma01::AudioRx,
                13 => Periph4Dma01::AudioTx,
                14 => Periph4Dma01::I2c1Rx,
                15 => Periph4Dma01::I2c1Tx,
                16 => Periph4Dma01::I2sRx,
                17 => Periph4Dma01::I2sTx,
                18 => Periph4Dma01::PdmRx,
                19 => Periph4Dma01::GpAdc,
                _ => Periph4Dma01::GpDac,
            };
            let tmp_val = match i {
                0 => 0x00000000,
                1 => 0x00000040,
                2 => 0x00000080,
                3 => 0x000000C0,
                4 => 0x00000100,
                5 => 0x00000140,
                6 => 0x00000180,
                7 => 0x000001C0,
                8 => 0x00000200,
                9 => 0x00000240,
                10 => 0x00000280,
                11 => 0x000002C0,
                12 => 0x00000300,
                13 => 0x00000340,
                14 => 0x00000380,
                15 => 0x000003C0,
                16 => 0x00000400,
                17 => 0x00000440,
                18 => 0x00000480,
                19 => 0x00000580,
                _ => 0x000005C0,
            };
            val = val.set_dst_periph4dma01(tmp_periph);
            assert_eq!(val.dst_periph4dma01(), tmp_periph);
            assert_eq!(val.0, tmp_val);
        }

        val = ChannelConfig(0x0);
        // The number 'i' is not related to the actual register, but only to make the code more concise.
        for i in 0..11 as u8 {
            let tmp_periph = match i {
                0 => Periph4Dma2::Uart3Rx,
                1 => Periph4Dma2::Uart3Tx,
                2 => Periph4Dma2::Spi1Rx,
                3 => Periph4Dma2::Spi1Tx,
                4 => Periph4Dma2::I2c2Rx,
                5 => Periph4Dma2::I2c2Tx,
                6 => Periph4Dma2::I2c3Rx,
                7 => Periph4Dma2::I2c3Tx,
                8 => Periph4Dma2::DsiRx,
                9 => Periph4Dma2::DsiTx,
                _ => Periph4Dma2::DbiTx,
            };
            let tmp_val = match i {
                0 => 0x00000000,
                1 => 0x00000040,
                2 => 0x00000080,
                3 => 0x000000C0,
                4 => 0x00000180,
                5 => 0x000001C0,
                6 => 0x00000200,
                7 => 0x00000240,
                8 => 0x00000280,
                9 => 0x000002C0,
                _ => 0x00000580,
            };
            val = val.set_dst_periph4dma2(tmp_periph);
            assert_eq!(val.dst_periph4dma2(), tmp_periph);
            assert_eq!(val.0, tmp_val);
        }

        val = ChannelConfig(0x0);
        // The number 'i' is not related to the actual register, but only to make the code more concise.
        for i in 0..21 as u8 {
            let tmp_periph = match i {
                0 => Periph4Dma01::Uart0Rx,
                1 => Periph4Dma01::Uart0Tx,
                2 => Periph4Dma01::Uart1Rx,
                3 => Periph4Dma01::Uart1Tx,
                4 => Periph4Dma01::Uart2Rx,
                5 => Periph4Dma01::Uart2Tx,
                6 => Periph4Dma01::I2c0Rx,
                7 => Periph4Dma01::I2c0Tx,
                8 => Periph4Dma01::IrTx,
                9 => Periph4Dma01::GpioTx,
                10 => Periph4Dma01::Spi0Rx,
                11 => Periph4Dma01::Spi0Tx,
                12 => Periph4Dma01::AudioRx,
                13 => Periph4Dma01::AudioTx,
                14 => Periph4Dma01::I2c1Rx,
                15 => Periph4Dma01::I2c1Tx,
                16 => Periph4Dma01::I2sRx,
                17 => Periph4Dma01::I2sTx,
                18 => Periph4Dma01::PdmRx,
                19 => Periph4Dma01::GpAdc,
                _ => Periph4Dma01::GpDac,
            };
            let tmp_val = match i {
                0 => 0x00000000,
                1 => 0x00000002,
                2 => 0x00000004,
                3 => 0x00000006,
                4 => 0x00000008,
                5 => 0x0000000A,
                6 => 0x0000000C,
                7 => 0x0000000E,
                8 => 0x00000010,
                9 => 0x00000012,
                10 => 0x00000014,
                11 => 0x00000016,
                12 => 0x00000018,
                13 => 0x0000001A,
                14 => 0x0000001C,
                15 => 0x0000001E,
                16 => 0x00000020,
                17 => 0x00000022,
                18 => 0x00000024,
                19 => 0x0000002C,
                _ => 0x0000002E,
            };
            val = val.set_src_periph4dma01(tmp_periph);
            assert_eq!(val.src_periph4dma01(), tmp_periph);
            assert_eq!(val.0, tmp_val);
        }

        val = ChannelConfig(0x0);
        // The number 'i' is not related to the actual register, but only to make the code more concise.
        for i in 0..11 as u8 {
            let tmp_periph = match i {
                0 => Periph4Dma2::Uart3Rx,
                1 => Periph4Dma2::Uart3Tx,
                2 => Periph4Dma2::Spi1Rx,
                3 => Periph4Dma2::Spi1Tx,
                4 => Periph4Dma2::I2c2Rx,
                5 => Periph4Dma2::I2c2Tx,
                6 => Periph4Dma2::I2c3Rx,
                7 => Periph4Dma2::I2c3Tx,
                8 => Periph4Dma2::DsiRx,
                9 => Periph4Dma2::DsiTx,
                _ => Periph4Dma2::DbiTx,
            };
            let tmp_val = match i {
                0 => 0x00000000,
                1 => 0x00000002,
                2 => 0x00000004,
                3 => 0x00000006,
                4 => 0x0000000C,
                5 => 0x0000000E,
                6 => 0x00000010,
                7 => 0x00000012,
                8 => 0x00000014,
                9 => 0x00000016,
                _ => 0x0000002C,
            };
            val = val.set_src_periph4dma2(tmp_periph);
            assert_eq!(val.src_periph4dma2(), tmp_periph);
            assert_eq!(val.0, tmp_val);
        }

        val = ChannelConfig(0x0);
        val = val.enable_ch();
        assert!(val.is_ch_enabled());
        assert_eq!(val.0, 0x00000001);
        val = val.disable_ch();
        assert!(!val.is_ch_enabled());
        assert_eq!(val.0, 0x00000000);
    }
}
