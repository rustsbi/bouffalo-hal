//! Secure Digital Input/Output peripheral.

use crate::glb;
use crate::gpio::{self, Alternate};
use core::arch::asm;
use core::ops::Deref;
use embedded_io::Write;
use embedded_sdmmc::{Block, BlockDevice, BlockIdx};
use volatile_register::{RO, RW, WO};

/// Secure Digital Input/Output peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// 32-bit block count / (SDMA system address) register.
    pub system_address: RW<SystemAddress>,
    /// Configuration register for number of bytes in a data block.
    pub block_size: RW<BlockSize>,
    /// Configuration register for number of data blocks.
    pub block_count: RW<BlockCount>,
    /// Register that contains the SD command argument.
    pub argument: RW<Argument>,
    /// Control register for the operation of data transfers.
    pub transfer_mode: RW<TransferMode>,
    /// Command register.
    pub command: RW<Command>,
    /// Register that stores responses from SD cards.
    pub response: RW<Response>,
    /// 32-bit data port register to accesses internal buffer.
    pub buffer_data_port: RW<BufferDataPort>,
    /// 32-bit read only register to get status of the host controller.
    pub present_state: RO<PresentState>,
    /// Host control 1 register.
    pub host_control_1: RW<HostControl1>,
    /// Power control register.
    pub power_control: RW<PowerControl>,
    /// Block gap control register.
    pub block_gap: RW<BlockGap>,
    /// Register which is mandatory for the host controller.
    pub wakeup_control: RW<WakeupControl>,
    /// Control register for SDCLK in SD mode and RCLK in UHS-II mode.
    pub clock_control: RW<ClockControl>,
    /// Timeout control register.
    pub timeout_control: RW<TimeoutControl>,
    /// Writting 1 to each bit of this register to generate a reset pulse.
    pub software_reset: RW<SoftwareReset>,
    /// Register that shows the defined normal interrupt status.
    pub normal_interrupt_status: RW<NormalInterruptStatus>,
    /// Register that shows the defined error interrupt status.
    pub error_interrupt_status: RW<ErrorInterruptStatus>,
    /// Register that sets to 1 enables normal interrupt status.
    pub normal_interrupt_status_enable: RW<NormalInterruptStatusEnable>,
    /// Register that sets to 1 enables error interrupt status.
    pub error_interrupt_status_enable: RW<ErrorInterruptStatusEnable>,
    /// Register that selects which interrupt status is indicated to the host system as the interrupt.
    pub normal_interrupt_signal_enable: RW<NormalInterruptSignalEnable>,
    /// Register that selects which interrupt status is notified to the host system as the interrupt.
    pub error_interrupt_signal_enable: RW<ErrorInterruptSignalEnable>,
    /// Register that indicates CMD12 response error of auto CMD12 and CMD23 response error of auto CMD23.
    pub auto_cmd_error_status: RO<AutoCmdErrorStatus>,
    /// Host control 2 register.
    pub host_control_2: RW<HostControl2>,
    /// Register that provides the host driver with information specific to the host controller implementation.
    pub capabilities: RO<Capabilities>,
    /// Registers that indicates maximum current capability fo each voltage.
    pub max_current_capabilities: RO<MaxCurrentCapabilities>,
    /// Register that simplifies test of the auto command error status register.
    pub force_event_auto_cmd_error_status: WO<ForceEventAutoCmdErrorStatus>,
    /// Register that simplifies test of the error interrupt status register.
    pub force_event_error_interrupt_status: WO<ForceEventErrorInterruptStatus>,
    /// Register that holds the ADMA state when ADMA error interrupt is occurred.
    pub adma2_error_status: RO<Adma2ErrorStatus>,
    /// Register that contains the physical descriptor address used for ADMA data transfer.
    pub adma2_system_address: RW<Adma2SystemAddress>,
    /// Preset value register.
    pub preset_value: RW<PresetValue>,
    _reserved0: [u8; 8],
    /// ADMA3 intergrated descriptor address register.
    pub adma3_integrated_descriptor_address: RW<ADMA3IntegratedDescriptorAddress>,
    _reserved1: [u8; 96],
    /// Shared bus control register.
    pub shared_bus_control: RW<SharedBusControl>,
    _reserved2: [u8; 24],
    /// Slot interrupt status register.
    pub slot_interrupt_status: RO<SlotInterruptStatus>,
    /// Host controller version register.
    pub host_controller_version: RO<HostControllerVersion>,
    /// SD extra parameters register.
    pub sd_extra_parameters: RW<SDExtraParameters>,
    /// FIFO parameters register.
    pub fifo_parameters: RW<FifoParameters>,
    /// SPI mode register.
    pub spi_mode: RW<SpiMode>,
    /// Clock and burst size setup register.
    pub clock_and_burst_size_setup: RW<ClockAndBurstSizeSetup>,
    /// CE-ATA register.
    pub ce_ata: RW<CeAta>,
    /// PAD I/O setup register.
    pub pad_io_setup: RW<PadIoSetup>,
    /// RX configuration register.
    pub rx_configuration: RW<RxConfiguration>,
    /// TX configuration register.
    pub tx_configuration: RW<TxConfiguration>,
    /// Tuning config register.
    pub tuning_configuration: RW<TuningConfiguration>,
}

/// 32-bit block count / (SDMA system address) register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SystemAddress(u32);

impl SystemAddress {
    /// Get sdma system address.
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn addr(self) -> u32 {
        self.0
    }
    /// Set argument2.
    /// Used with the auto CMD23 to set a 32-bit block count value to the argument of the CMD23 while executing auto CMD23.
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn set_arg2(self, val: u32) -> Self {
        Self(val)
    }
    /// Get argument2.
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn arg2(self) -> u32 {
        self.0
    }
}

/// Configuration register for number of bytes in a data block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockSize(u16);

impl BlockSize {
    const HOST_SDMA: u16 = 0x7 << 12;
    const TRANSFER_BLOCK: u16 = 0xFFF;

    /// In case of this register is set to 0 (buffer size = 4K bytes), lower 12-bit of byte address points data in the contiguous buffer and the upper 20-bit points the location of the buffer in the system memory.
    /// The SDMA transfer stops when the host controller detects carry out of the address from bit 11 to 12.
    /// These bits shall be supported when the SDMA support in the capabilities register is set to 1 and this function is active when the DMA Enable in the transfer mode register is set to 1.
    /// ADMA does not use this register.
    #[inline]
    pub const fn set_host_sdma(self, val: u8) -> Self {
        Self((self.0 & !Self::HOST_SDMA) | (Self::HOST_SDMA & ((val as u16) << 12)))
    }
    /// Get host SDMA register.
    #[inline]
    pub const fn host_sdma(self) -> u8 {
        ((self.0 & Self::HOST_SDMA) >> 12) as u8
    }
    /// Specifies the block size of data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53.
    /// Values ranging from 1 up to the maximum buffer size can be set.
    /// In case of memory, it shall be set up to 512 bytes (Refer to implementation note in Section 1.7.2).
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn set_transfer_block(self, val: u16) -> Self {
        Self((self.0 & !Self::TRANSFER_BLOCK) | (Self::TRANSFER_BLOCK & val))
    }
    /// Get the block size of data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53.
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn transfer_block(self) -> u16 {
        self.0 & Self::TRANSFER_BLOCK
    }
}

/// Configuration register for number of data blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockCount(u16);

impl BlockCount {
    /// Set left blocks count for current transfer.
    /// This register is enabled when block count enable in the transfer mode register is set to 1 and is valid only for multiple block transfers.
    /// The host driver shall set this register to a value between 1 and the maximum block count.
    /// This register should be accessed only when no transaction is executing.
    #[inline]
    pub const fn set_blocks_count(self, val: u16) -> Self {
        Self(val)
    }
    /// Get left blocks count for current transfer.
    /// This register should be accessed only when no transaction is executing.
    #[inline]
    pub const fn blocks_count(self) -> u16 {
        self.0
    }
}

/// Register that contains the SD command argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Argument(u32);

impl Argument {
    /// Set command argument 1.
    /// The SD command argument is specified as bit39-8 of command-format in the physical layer specification.
    #[inline]
    pub const fn set_cmd_arg(self, val: u32) -> Self {
        Self(val)
    }
    /// Get command argument 1.
    #[inline]
    pub const fn cmd_arg(self) -> u32 {
        self.0
    }
}

/// Control register for the operation of data transfers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferMode(u16);

/// Multi/Single block select.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockMode {
    /// Other commands.
    Other,
    /// Multiple-block transfer commands using data line.
    MultiBlock,
}

/// Data transfer mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataTransferMode {
    /// Other modes.
    Other,
    /// Master in, slave out.
    MISO,
}

/// Auto command mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AutoCMDMode {
    /// Auto CMD12 enable.
    CMD12 = 1,
    /// Auto CMD23 enable.
    CMD23 = 2,
    /// Set this bit to zero.
    None = 0,
}

impl TransferMode {
    const BLOCK_SELECT: u16 = 0x1 << 5;
    const DATA_TRANSFER: u16 = 0x1 << 4;
    const AUTO_CMD: u16 = 0x3 << 2;
    const BLOCK_COUNT: u16 = 0x1 << 1;
    const DMA_ENABLE: u16 = 0x1;

    /// Set multi/single block mode.
    #[inline]
    pub const fn set_block_mode(self, val: BlockMode) -> Self {
        Self((self.0 & !Self::BLOCK_SELECT) | (Self::BLOCK_SELECT & ((val as u16) << 5)))
    }
    /// Get multi/single block mode.
    #[inline]
    pub const fn block_mode(self) -> BlockMode {
        match (self.0 & Self::BLOCK_SELECT) >> 5 {
            1 => BlockMode::MultiBlock,
            _ => BlockMode::Other,
        }
    }
    /// Set data transfer direction.
    #[inline]
    pub const fn set_data_transfer_mode(self, val: DataTransferMode) -> Self {
        Self((self.0 & !Self::DATA_TRANSFER) | (Self::DATA_TRANSFER & ((val as u16) << 4)))
    }
    /// Get data transfer direction.
    #[inline]
    pub const fn data_transfer_mode(self) -> DataTransferMode {
        match (self.0 & Self::DATA_TRANSFER) >> 4 {
            1 => DataTransferMode::MISO,
            _ => DataTransferMode::Other,
        }
    }
    /// Set auto command mode.
    #[inline]
    pub const fn set_auto_cmd_mode(self, val: AutoCMDMode) -> Self {
        Self((self.0 & !Self::AUTO_CMD) | (Self::AUTO_CMD & ((val as u16) << 2)))
    }
    /// Get auto command mode.
    #[inline]
    pub const fn auto_cmd_mode(self) -> AutoCMDMode {
        match (self.0 & Self::AUTO_CMD) >> 2 {
            1 => AutoCMDMode::CMD12,
            2 => AutoCMDMode::CMD23,
            _ => AutoCMDMode::None,
        }
    }
    /// Enable block count register.
    #[inline]
    pub const fn enable_block_count(self) -> Self {
        Self((self.0 & !Self::BLOCK_COUNT) | Self::BLOCK_COUNT & (1 << 1))
    }
    /// Disable block count register.
    #[inline]
    pub const fn disable_block_count(self) -> Self {
        Self((self.0 & !Self::BLOCK_COUNT) | Self::BLOCK_COUNT & (0 << 1))
    }
    /// Check if block count register is enabled.
    #[inline]
    pub const fn is_block_count_enabled(self) -> bool {
        (self.0 & Self::BLOCK_COUNT) >> 1 == 1
    }
    /// Enable DMA.
    #[inline]
    pub fn enable_dma(self) -> Self {
        Self((self.0 & !Self::DMA_ENABLE) | Self::DMA_ENABLE & 1)
    }
    /// Disable DMA.
    #[inline]
    pub fn disable_dma(self) -> Self {
        Self((self.0 & !Self::DMA_ENABLE) | Self::DMA_ENABLE & 0)
    }
    /// Check if DMA is enabled.
    #[inline]
    pub fn is_dma_enabled(self) -> bool {
        self.0 & Self::DMA_ENABLE == 1
    }
}

/// Command register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Command(u16);

/// Command type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CmdType {
    Normal,
    Suspend,
    Resume,
    Abort,
    Empty,
}
/// Response type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResponseType {
    /// No response.
    NoResponse,
    /// Response length 136.
    ResponseLen136,
    /// Response length 48.
    ResponseLen48,
    /// Response length 48 check Busy after response.
    ResponseLen48Check,
}

impl Command {
    const CMD_INDEX: u16 = 0x3F << 8;
    const CMD_TYPE: u16 = 0x3 << 6;
    const DATA_PRSENT: u16 = 0x1 << 5;
    const CMD_INDEX_CHECK: u16 = 0x1 << 4;
    const CMD_CRC: u16 = 0x1 << 3;
    const RESPONSE_TYPE: u16 = 0x3;

    /// These bits shall be set to the command number (CMD0-63, ACMD0-63) that is specified in bits 45-40 of the command-format in the physical layer specification and SDIO Card Specification.
    #[inline]
    pub const fn set_cmd_idx(self, val: u16) -> Self {
        Self((self.0 & !Self::CMD_INDEX) | (Self::CMD_INDEX & (val << 8)))
    }
    /// Get command number.
    #[inline]
    pub const fn cmd_idx(self) -> u16 {
        (self.0 & Self::CMD_INDEX) >> 8
    }
    /// Set command type.
    #[inline]
    pub const fn set_cmd_type(self, val: CmdType) -> Self {
        Self((self.0 & !Self::CMD_TYPE) | (Self::CMD_TYPE & ((val as u16) << 6)))
    }
    /// Get command type.
    #[inline]
    pub const fn cmd_type(self) -> CmdType {
        match (self.0 & Self::CMD_TYPE) >> 6 {
            0 => CmdType::Normal,
            1 => CmdType::Suspend,
            2 => CmdType::Resume,
            3 => CmdType::Abort,
            _ => CmdType::Empty,
        }
    }
    /// Set this bit to 1 to indicate that data is present and shall be transferred using the data line.
    #[inline]
    pub const fn set_data_present(self) -> Self {
        Self((self.0 & !Self::DATA_PRSENT) | (Self::DATA_PRSENT & (1 << 5)))
    }
    /// Set this bit to 0 for the following:
    /// (1) Commands using only command line (ex.CMD52).
    /// (2) Commands with no data transfer but using busy signal on data line (R1b or R5b ex. CMD38).
    /// (3) Resume command.
    #[inline]
    pub const fn unset_data_present(self) -> Self {
        Self((self.0 & !Self::DATA_PRSENT) | (Self::DATA_PRSENT & (0 << 5)))
    }
    /// Check if data present bit is set.
    #[inline]
    pub const fn is_data_present(self) -> bool {
        (self.0 & Self::DATA_PRSENT) >> 5 == 1
    }
    /// Enable check the index field.
    #[inline]
    pub const fn enable_index_check(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_CHECK) | (Self::CMD_INDEX_CHECK & (1 << 4)))
    }
    /// Disable check the index field.
    #[inline]
    pub const fn disable_index_check(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_CHECK) | (Self::CMD_INDEX_CHECK & (0 << 4)))
    }
    /// Check if check the index field is enabled.
    #[inline]
    pub const fn is_index_check_enabled(self) -> bool {
        (self.0 & Self::CMD_INDEX_CHECK) >> 4 == 1
    }
    /// Enable command crc.
    #[inline]
    pub const fn enable_cmd_crc(self) -> Self {
        Self((self.0 & !Self::CMD_CRC) | (Self::CMD_CRC & (1 << 3)))
    }
    /// Disable command crc.
    #[inline]
    pub const fn disable_cmd_crc(self) -> Self {
        Self((self.0 & !Self::CMD_CRC) | (Self::CMD_CRC & (0 << 3)))
    }
    /// Check if command crc is enabled.
    #[inline]
    pub const fn is_cmd_crc_enabled(self) -> bool {
        (self.0 & Self::CMD_CRC) >> 3 == 1
    }
    /// Set Response Type.
    #[inline]
    pub const fn set_response_type(self, val: ResponseType) -> Self {
        Self((self.0 & !Self::RESPONSE_TYPE) | (Self::RESPONSE_TYPE & (val as u16)))
    }
    /// Get Response Type.
    #[inline]
    pub const fn response_type(self) -> ResponseType {
        match self.0 & Self::RESPONSE_TYPE {
            1 => ResponseType::ResponseLen136,
            2 => ResponseType::ResponseLen48,
            3 => ResponseType::ResponseLen48Check,
            _ => ResponseType::NoResponse,
        }
    }
}

/// Register that stores responses from SD cards.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Response(u128);

impl Response {
    /// Set command response.
    #[inline]
    pub const fn set_response(self, val: u128) -> Self {
        Self(val)
    }
    /// Get command response.
    #[inline]
    pub const fn response(self) -> u128 {
        self.0
    }
}

/// 32-bit data port register to accesses internal buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BufferDataPort(u32);

impl BufferDataPort {
    /// Set buffer data.
    /// The host controller buffer can be accessed through this 32-bit data port register.
    #[inline]
    pub const fn set_buffer_data(self, val: u32) -> Self {
        Self(val)
    }
    /// Get Buffer Data.
    #[inline]
    pub const fn buffer_data(self) -> u32 {
        self.0
    }
}

/// 32-bit read only register to get status of the host controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PresentState(u32);

impl PresentState {
    const CMD_LINE: u32 = 0x1 << 24;
    const DAT_LINE: u32 = 0xF << 20;
    const WRITE_PROTECT: u32 = 0x1 << 19;
    const CARD_DETECT: u32 = 0x1 << 18;
    const CARD_STATE: u32 = 0x1 << 17;
    const CARD_INSERTED: u32 = 0x1 << 16;
    const BUFFE_READ: u32 = 0x1 << 11;
    const BUFFER_WRITE: u32 = 0x1 << 10;
    const READ_TRANSFER: u32 = 0x1 << 9;
    const WRITE_TRANSFER: u32 = 0x1 << 8;
    const RE_TUNING_REQUEST: u32 = 0x1 << 3;
    const DAT_LINE_ACTIVE: u32 = 0x1 << 2;
    const CMD_INHIBIT1: u32 = 0x1 << 1;
    const CMD_INHIBIT0: u32 = 0x1;

    /// Get command line signal level.
    #[inline]
    pub const fn cmd_line(self) -> u8 {
        ((self.0 & Self::CMD_LINE) >> 24) as u8
    }
    /// Get data line signal level.
    #[inline]
    pub const fn dat_line(self) -> u8 {
        ((self.0 & Self::DAT_LINE) >> 20) as u8
    }
    /// Check if write protect is active.
    #[inline]
    pub const fn is_write_protect(self) -> bool {
        (self.0 & Self::WRITE_PROTECT) >> 19 == 1
    }
    /// Check if sdcard is detected.
    /// Valid only if card detect pin is stable.
    #[inline]
    pub const fn is_card_detect(self) -> bool {
        (self.0 & Self::CARD_DETECT) >> 18 == 0
    }
    /// Check if card detect pin level is stable.
    #[inline]
    pub const fn is_card_detect_stable(self) -> bool {
        (self.0 & Self::CARD_STATE) >> 17 == 1
    }
    /// Check if card is inserted.
    #[inline]
    pub const fn is_card_inserted(self) -> bool {
        (self.0 & Self::CARD_INSERTED) >> 16 == 1
    }
    /// Check if read buffer is empty.
    /// Non-DMA used.
    #[inline]
    pub const fn is_read_buffer_empty(self) -> bool {
        (self.0 & Self::BUFFE_READ) >> 11 == 0
    }
    /// Check if write buffer is empty.
    /// Non-DMA used.
    #[inline]
    pub const fn is_write_buffer_empty(self) -> bool {
        (self.0 & Self::BUFFER_WRITE) >> 10 == 0
    }
    /// Check if read transfer is finished.
    #[inline]
    pub const fn is_read_transfer_finished(self) -> bool {
        (self.0 & Self::READ_TRANSFER) >> 9 == 0
    }
    /// Check if read transfer is finished.
    #[inline]
    pub const fn is_write_transfer_finished(self) -> bool {
        (self.0 & Self::WRITE_TRANSFER) >> 8 == 0
    }
    /// Check if re-tuing request occurs.
    #[inline]
    pub const fn if_re_tuning_occurs(self) -> bool {
        (self.0 & Self::RE_TUNING_REQUEST) >> 3 == 1
    }
    /// Check if data line is active.
    #[inline]
    pub const fn is_dat_line_active(self) -> bool {
        (self.0 & Self::DAT_LINE_ACTIVE) >> 2 == 1
    }
    /// Check if data line is busy.
    #[inline]
    pub const fn is_dat_line_busy(self) -> bool {
        (self.0 & Self::CMD_INHIBIT1) >> 1 == 1
    }
    /// Check if command line is busy.
    #[inline]
    pub const fn is_cmd_line_busy(self) -> bool {
        (self.0 & Self::CMD_INHIBIT0) == 1
    }
}

/// Host control 1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControl1(u8);

/// Source for the card detection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardSignal {
    /// SDCD# is selected (for normal use)
    SDCD,
    /// The card detect test level is selected (for test purpose)
    TestLevel,
}

/// Bus width mode for embedded device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusWidthMode {
    /// 8-bit bus width.
    SelectByDataTransferWidth,
    /// Bus width is selected by data transfer width.
    EightBitWidth,
}

/// DMA mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DmaMode {
    /// SDMA is selected.
    SDMA = 0,
    /// 32-bit address ADMA2 is selected.
    ADMA2 = 2,
    /// Do not use DMA.
    None = 5,
}

/// Speed mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpeedMode {
    NormalSpeed,
    HighSpeed,
}

/// Data transfer width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferWidth {
    /// 1-bit mode
    OneBitMode,
    /// 4-bit mode
    FourBitMode,
}

/// Caution led state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LedState {
    Off,
    On,
}

impl HostControl1 {
    const CARD_DETECT_SIGNAL_SELECT: u8 = 0x1 << 7;
    const CARD_DETECT_TEST_LEVEL: u8 = 0x1 << 6;
    const EXTEND_DATA: u8 = 0x1 << 5;
    const DMA_SELECT: u8 = 0x3 << 3;
    const HIGH_SPEED: u8 = 0x1 << 2;
    const DATA_TRANSFER: u8 = 0x1 << 1;
    const LED_CONTROL: u8 = 0x1;

    /// Set card detect signal.
    #[inline]
    pub const fn set_card_detect_signal(self, val: CardSignal) -> Self {
        Self(
            (self.0 & !Self::CARD_DETECT_SIGNAL_SELECT)
                | (Self::CARD_DETECT_SIGNAL_SELECT & ((val as u8) << 7)),
        )
    }
    /// Get card detect signal.
    #[inline]
    pub const fn card_detect_signal(self) -> CardSignal {
        match (self.0 & Self::CARD_DETECT_SIGNAL_SELECT) >> 7 {
            1 => CardSignal::TestLevel,
            _ => CardSignal::SDCD,
        }
    }
    /// Select the source for the card detect.
    #[inline]
    pub const fn set_card_detect_level(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::CARD_DETECT_TEST_LEVEL)
                | (Self::CARD_DETECT_TEST_LEVEL & ((val as u8) << 6)),
        )
    }
    /// Check if the card is detected.
    /// Enabled while card_detect_signal is testLevel.
    #[inline]
    pub const fn is_card_detected(self) -> bool {
        (self.0 & Self::CARD_DETECT_TEST_LEVEL) >> 6 == 1
    }
    /// Set bus width mode for embedded device.
    #[inline]
    pub const fn set_bus_width(self, val: BusWidthMode) -> Self {
        Self((self.0 & !Self::EXTEND_DATA) | (Self::EXTEND_DATA & ((val as u8) << 5)))
    }
    /// Get bus width mode for embedded device.
    #[inline]
    pub const fn bus_width(self) -> BusWidthMode {
        match (self.0 & Self::EXTEND_DATA) >> 5 {
            1 => BusWidthMode::EightBitWidth,
            _ => BusWidthMode::SelectByDataTransferWidth,
        }
    }
    /// Set DMA mode.
    #[inline]
    pub const fn set_dma_mode(self, val: DmaMode) -> Self {
        Self((self.0 & !Self::DMA_SELECT) | (Self::DMA_SELECT & ((val as u8) << 3)))
    }
    /// Get DMA mode.
    #[inline]
    pub const fn dma_mode(self) -> DmaMode {
        match (self.0 & Self::DMA_SELECT) >> 3 {
            0 => DmaMode::SDMA,
            2 => DmaMode::ADMA2,
            _ => DmaMode::None,
        }
    }
    /// Set speed mode.
    #[inline]
    pub const fn set_speed_mode(self, val: SpeedMode) -> Self {
        Self((self.0 & !Self::HIGH_SPEED) | (Self::HIGH_SPEED & (val as u8) << 2))
    }
    /// Get speed mode.
    #[inline]
    pub const fn speed_mode(self) -> SpeedMode {
        match (self.0 & Self::HIGH_SPEED) >> 2 {
            1 => SpeedMode::HighSpeed,
            _ => SpeedMode::NormalSpeed,
        }
    }
    /// Set data transfer width.
    #[inline]
    pub const fn set_transfer_width(self, val: TransferWidth) -> Self {
        Self((self.0 & !Self::DATA_TRANSFER) | (Self::DATA_TRANSFER & ((val as u8) << 1)))
    }
    /// Get data transfer width.
    #[inline]
    pub const fn transfer_width(self) -> TransferWidth {
        match (self.0 & Self::DATA_TRANSFER) >> 1 {
            1 => TransferWidth::FourBitMode,
            _ => TransferWidth::OneBitMode,
        }
    }
    /// Set caution led state.
    #[inline]
    pub const fn set_led_state(self, val: LedState) -> Self {
        Self((self.0 & !Self::LED_CONTROL) | (Self::LED_CONTROL & (val as u8)))
    }
    /// Get caution led state.
    #[inline]
    pub const fn led_state(self) -> LedState {
        match self.0 & Self::LED_CONTROL {
            1 => LedState::On,
            _ => LedState::Off,
        }
    }
}

/// Power control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PowerControl(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusVoltage {
    /// 1.8V.
    V1_8 = 5,
    /// 3.0V.
    V3_0 = 6,
    /// 3.3V.
    V3_3 = 7,
}

impl PowerControl {
    const SD_BUS_VOLTAGE: u8 = 0x7 << 1;
    const SD_BUS_POWER: u8 = 0x1;

    /// Set SD bus voltage.
    #[inline]
    pub const fn set_bus_voltage(self, val: BusVoltage) -> Self {
        Self((self.0 & !Self::SD_BUS_VOLTAGE) | (Self::SD_BUS_VOLTAGE & ((val as u8) << 1)))
    }
    /// Get SD bus voltage.
    #[inline]
    pub const fn bus_voltage(self) -> BusVoltage {
        match (self.0 & Self::SD_BUS_VOLTAGE) >> 1 {
            5 => BusVoltage::V1_8,
            6 => BusVoltage::V3_0,
            7 => BusVoltage::V3_3,
            _ => unreachable!(),
        }
    }
    /// Enable SD bus power.
    /// Before setting this bit, the SD host driver shall set SD bus voltage select.
    #[inline]
    pub const fn enable_bus_power(self) -> Self {
        Self((self.0 & !Self::SD_BUS_POWER) | (Self::SD_BUS_POWER & 1))
    }
    /// Disable SD bus power.
    /// Host controller detects the no card state, this bit shall be cleared.
    #[inline]
    pub const fn disable_bus_power(self) -> Self {
        Self((self.0 & !Self::SD_BUS_POWER) | (Self::SD_BUS_POWER & 0))
    }
    /// Check if bus power is enabled.
    #[inline]
    pub const fn is_bus_power_enable(self) -> bool {
        self.0 & Self::SD_BUS_POWER == 1
    }
}

/// Block Gap Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockGap(u8);

impl BlockGap {
    const INTERRUPT_AT_BG: u8 = 0x1 << 3;
    const READ_WAIT: u8 = 0x1 << 2;
    const CONTINUE_REQUEST_CTRL: u8 = 0x1 << 1;
    const STOP_AT_BG: u8 = 0x1;

    /// Enable block gap interrupt.
    /// Valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle.
    #[inline]
    pub const fn enable_block_gap_int(self) -> Self {
        Self((self.0 & !Self::INTERRUPT_AT_BG) | (Self::INTERRUPT_AT_BG & (1 << 3)))
    }
    /// Disable block gap interrupt.
    /// Valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle.
    #[inline]
    pub const fn disable_block_gap_int(self) -> Self {
        Self((self.0 & !Self::INTERRUPT_AT_BG) | (Self::INTERRUPT_AT_BG & (0 << 3)))
    }
    /// Check if block gap interrupt is enabled.
    #[inline]
    pub const fn is_block_gap_int_enabled(self) -> bool {
        (self.0 & Self::INTERRUPT_AT_BG) >> 3 == 1
    }
    /// Enable read wait.
    /// Do not use if the card does not support it.
    #[inline]
    pub const fn enable_read_wait(self) -> Self {
        Self((self.0 & !Self::READ_WAIT) | (Self::READ_WAIT & (1 << 2)))
    }
    /// Disable read wait.
    #[inline]
    pub const fn disable_read_wait(self) -> Self {
        Self((self.0 & !Self::READ_WAIT) | (Self::READ_WAIT & (0 << 2)))
    }
    /// Check if read wait is enabled.
    #[inline]
    pub const fn is_read_wait_enabled(self) -> bool {
        (self.0 & Self::READ_WAIT) >> 2 == 1
    }
    /// Restart transaction.
    /// To cancel stop at the block gap, set stop at block gap request to 0 and set this bit 1 to restart the transfer.
    /// The host controller automatically clears this bit.
    #[inline]
    pub const fn restart_transaction(self) -> Self {
        Self((self.0 & !Self::CONTINUE_REQUEST_CTRL) | (Self::CONTINUE_REQUEST_CTRL & (1 << 1)))
    }
    /// Set stop at block gap request bit.
    #[inline]
    pub const fn set_stop_at_block_gap_req(self, val: u8) -> Self {
        Self((self.0 & !Self::STOP_AT_BG) | (Self::STOP_AT_BG & val))
    }
    /// Get stop at block gap request bit.
    #[inline]
    pub const fn stop_at_block_gap_req(self) -> u8 {
        self.0 & Self::STOP_AT_BG
    }
}

/// Register which is mandatory for the host controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WakeupControl(u8);

impl WakeupControl {
    const CARD_REMOVAL: u8 = 0x1 << 2;
    const CARD_INSERTED: u8 = 0x1 << 1;
    const CARD_INTERRUPT: u8 = 0x1;

    /// Wakeup event enable on SD card removal.
    #[inline]
    pub const fn enable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 2)))
    }
    /// Wakeup event disable on SD card removal.
    #[inline]
    pub const fn disable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (0 << 2)))
    }
    /// Check if wakeup event on SD card removal is enabled.
    #[inline]
    pub const fn is_card_removal_enable(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 2 == 1
    }
    /// Wakeup event enable on SD card insertion.
    #[inline]
    pub const fn enable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTED) | (Self::CARD_INSERTED & (1 << 1)))
    }
    /// Wakeup event disable on SD card insertion.
    #[inline]
    pub const fn disable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTED) | (Self::CARD_INSERTED & (0 << 1)))
    }
    /// Check if wakeup event on SD card insertion is enabled.
    #[inline]
    pub const fn is_card_insertion_enable(self) -> bool {
        (self.0 & Self::CARD_INSERTED) >> 1 == 1
    }
    /// Wakeup event enable on card interrupt.
    #[inline]
    pub const fn enable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INTERRUPT) | (Self::CARD_INTERRUPT & 1))
    }
    /// Wakeup event disable on card interrupt.
    #[inline]
    pub const fn disable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INTERRUPT) | (Self::CARD_INTERRUPT & 0))
    }
    /// Check if wakeup event on SD card interrupt is enabled.
    #[inline]
    pub const fn is_card_int_enable(self) -> bool {
        self.0 & Self::CARD_INTERRUPT == 1
    }
}

/// Control register for SDCLK in SD mode and RCLK in UHS-II mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockControl(u16);

/// Clock generator mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClkGenMode {
    /// Divided clock mode.
    DividedClk,
    /// Programmable clock mode.
    ProgrammableClk,
}

impl ClockControl {
    const SD_CLK_FREQ: u16 = 0xFF << 8;
    const SD_CLK_FREQ_UPPER: u16 = 0x3 << 6;
    const CLK_GENERATOR: u16 = 0x1 << 5;
    const SD_CLK_EN: u16 = 0x1 << 2;
    const INTERNAL_CLK_STABLE: u16 = 0x1 << 1;
    const INTERNAL_CLK_EN: u16 = 0x1;

    /// Set SDCLK frequency.
    #[inline]
    pub const fn set_sd_clk_freq(self, val: u8) -> Self {
        Self((self.0 & !Self::SD_CLK_FREQ) | (Self::SD_CLK_FREQ & ((val as u16) << 8)))
    }
    /// Get SDCLK frequency.
    #[inline]
    pub const fn sd_clk_freq(self) -> u8 {
        ((self.0 & Self::SD_CLK_FREQ) >> 8) as u8
    }
    /// Set upper SDCLK frequency.
    /// Host controller version 1.00 and 2.00 do not support these bits and they are treated as 00b fixed value.
    #[inline]
    pub const fn set_sd_clk_freq_upper(self, val: u8) -> Self {
        Self((self.0 & !Self::SD_CLK_FREQ_UPPER) | (Self::SD_CLK_FREQ_UPPER & ((val as u16) << 6)))
    }
    /// Get upper SDCLK frequency.
    /// Host controller version 1.00 and 2.00 do not support these bits and they are treated as 00b fixed value.
    #[inline]
    pub const fn sd_clk_freq_upper(self) -> u8 {
        ((self.0 & Self::SD_CLK_FREQ_UPPER) >> 6) as u8
    }
    /// Set clock generate mode.
    #[inline]
    pub const fn set_clk_gen_mode(self, val: ClkGenMode) -> Self {
        Self((self.0 & !Self::CLK_GENERATOR) | (Self::CLK_GENERATOR & ((val as u16) << 5)))
    }
    /// Get clock generate mode.
    #[inline]
    pub const fn clk_gen_mode(self) -> ClkGenMode {
        match (self.0 & Self::CLK_GENERATOR) >> 5 {
            1 => ClkGenMode::ProgrammableClk,
            _ => ClkGenMode::DividedClk,
        }
    }
    /// Enable SD clock.
    #[inline]
    pub const fn enable_sd_clk(self) -> Self {
        Self((self.0 & !Self::SD_CLK_EN) | (Self::SD_CLK_EN & (1 << 2)))
    }
    /// Disable SD clock.
    #[inline]
    pub const fn disable_sd_clk(self) -> Self {
        Self((self.0 & !Self::SD_CLK_EN) | (Self::SD_CLK_EN & (0 << 2)))
    }
    /// Check if SD clock is enabled.
    #[inline]
    pub const fn is_sd_clk_enabled(self) -> bool {
        (self.0 & Self::SD_CLK_EN) >> 2 == 1
    }
    /// Check if internal clock stable.
    #[inline]
    pub const fn is_internal_clk_stable(self) -> bool {
        (self.0 & Self::INTERNAL_CLK_STABLE) >> 1 == 1
    }
    /// Enable internal clk.
    #[inline]
    pub const fn enable_internal_clk(self) -> Self {
        Self((self.0 & !Self::INTERNAL_CLK_EN) | (Self::INTERNAL_CLK_EN & 1))
    }
    /// Disable internal clk.
    #[inline]
    pub const fn disable_internal_clk(self) -> Self {
        Self((self.0 & !Self::INTERNAL_CLK_EN) | (Self::INTERNAL_CLK_EN & 0))
    }
    /// Check if internal clk is enable.
    #[inline]
    pub const fn is_internal_clk_enable(self) -> bool {
        self.0 & Self::INTERNAL_CLK_EN == 1
    }
}

/// Timeout control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimeoutControl(u8);

impl TimeoutControl {
    const DATA_TIMEOUT_CNT: u8 = 0xF;

    /// Set data timeout counter value.
    #[inline]
    pub const fn set_timeout_val(self, val: u8) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUT_CNT) | (Self::DATA_TIMEOUT_CNT & val))
    }
    /// Get data timeout counter value.
    #[inline]
    pub const fn timeout_val(self) -> u8 {
        self.0 & Self::DATA_TIMEOUT_CNT
    }
}

/// Writting 1 to each bit of this register to generate a reset pulse.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SoftwareReset(u8);

impl SoftwareReset {
    const SOFT_RESET_DAT: u8 = 0x1 << 2;
    const SOFT_RESET_CMD: u8 = 0x1 << 1;
    const SOFT_RESET_ALL: u8 = 0x1;

    /// Software reset data line.
    #[inline]
    pub const fn reset_dat(self) -> Self {
        Self((self.0 & !Self::SOFT_RESET_DAT) | (Self::SOFT_RESET_DAT & (1 << 2)))
    }
    /// Check if data line reset is finished (Cleared to 0).
    #[inline]
    pub const fn is_reset_dat_finished(self) -> bool {
        (self.0 & Self::SOFT_RESET_DAT) >> 2 == 0
    }
    /// Software reset command line.
    #[inline]
    pub const fn reset_cmd(self) -> Self {
        Self((self.0 & !Self::SOFT_RESET_CMD) | (Self::SOFT_RESET_CMD & (1 << 1)))
    }
    /// Check if command line reset is finished (Cleared to 0).
    #[inline]
    pub const fn is_reset_cmd_finished(self) -> bool {
        (self.0 & Self::SOFT_RESET_CMD) >> 1 == 0
    }
    /// Software reset all.
    /// This reset affects the entire host controller except for the card detection circuit.
    #[inline]
    pub const fn reset_all(self) -> Self {
        Self((self.0 & !Self::SOFT_RESET_ALL) | (Self::SOFT_RESET_ALL & 1))
    }
    /// Check if all reset is finished (Cleared to 0).
    #[inline]
    pub const fn is_reset_all_finished(self) -> bool {
        self.0 & Self::SOFT_RESET_ALL == 0
    }
}

/// Register that shows the defined normal interrupt status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalInterruptStatus(u16);

impl NormalInterruptStatus {
    const ERROR_INT: u16 = 0x1 << 15;
    const RE_TUNING_EVENT: u16 = 0x1 << 12;
    const INT_C: u16 = 0x1 << 11;
    const INT_B: u16 = 0x1 << 10;
    const INT_A: u16 = 0x1 << 9;
    const CARD_INT: u16 = 0x1 << 8;
    const CARD_REMOVAL: u16 = 0x1 << 7;
    const CARD_INSERTION: u16 = 0x1 << 6;
    const BUFFER_READ_READY: u16 = 0x1 << 5;
    const BUFFER_WRITE_READY: u16 = 0x1 << 4;
    const DMA_INT: u16 = 0x1 << 3;
    const BG_EVENT: u16 = 0x1 << 2;
    const TRANSFER_COMPLETE: u16 = 0x1 << 1;
    const CMD_COMPLETE: u16 = 0x1;

    /// Check if error interrupt occurs.
    #[inline]
    pub const fn if_err_int_occurs(self) -> bool {
        (self.0 & Self::ERROR_INT) >> 15 == 1
    }
    /// Check if re-tuning Event occurs.
    #[inline]
    pub const fn if_re_tuning_occurs(self) -> bool {
        (self.0 & Self::RE_TUNING_EVENT) >> 12 == 1
    }
    /// Check if INT_C enabled and INT_C# pin is in low level.
    #[inline]
    pub const fn is_int_c_enabled(self) -> bool {
        (self.0 & Self::INT_C) >> 11 == 1
    }
    /// Check if INT_B enabled and INT_B# pin is in low level.
    #[inline]
    pub const fn is_int_b_enabled(self) -> bool {
        (self.0 & Self::INT_B) >> 10 == 1
    }
    /// Check if INT_C enabled and INT_C# pin is in low level.
    #[inline]
    pub const fn is_int_a_enabled(self) -> bool {
        (self.0 & Self::INT_A) >> 9 == 1
    }
    /// Check if card interrupt occurs.
    #[inline]
    pub const fn if_card_int_occurs(self) -> bool {
        (self.0 & Self::CARD_INT) >> 8 == 1
    }
    /// Check if card is removed.
    #[inline]
    pub const fn is_card_removed(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 7 == 1
    }
    /// Clear card inserted bit.
    #[inline]
    pub const fn clear_card_removed(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 7)))
    }
    /// Check if card is inserted.
    #[inline]
    pub const fn is_card_inserted(self) -> bool {
        (self.0 & Self::CARD_INSERTION) >> 6 == 1
    }
    /// Clear card inserted bit.
    #[inline]
    pub const fn clear_card_inserted(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (1 << 6)))
    }
    /// Check if buffer read is ready.
    #[inline]
    pub const fn is_buffer_read_ready(self) -> bool {
        (self.0 & Self::BUFFER_READ_READY) >> 5 == 1
    }
    /// Clear buffer read ready bit.
    #[inline]
    pub const fn clear_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ_READY) | (Self::BUFFER_READ_READY & (1 << 5)))
    }
    /// Check if buffer write is ready.
    #[inline]
    pub const fn is_buffer_write_ready(self) -> bool {
        (self.0 & Self::BUFFER_WRITE_READY) >> 4 == 1
    }
    /// Clear buffer write ready bit.
    #[inline]
    pub const fn clear_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE_READY) | (Self::BUFFER_WRITE_READY & (1 << 4)))
    }
    /// Check if DMA interrupt occurs.
    #[inline]
    pub const fn if_dma_int_occurs(self) -> bool {
        (self.0 & Self::DMA_INT) >> 3 == 1
    }
    /// Clear DMA interrupt bit.
    #[inline]
    pub const fn clear_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (1 << 3)))
    }
    /// Check if block gap event occurs.
    #[inline]
    pub const fn if_block_gap_occurs(self) -> bool {
        (self.0 & Self::BG_EVENT) >> 2 == 1
    }
    /// Clear block gap event bit.
    #[inline]
    pub const fn clear_block_gap(self) -> Self {
        Self((self.0 & !Self::BG_EVENT) | (Self::BG_EVENT | (1 << 2)))
    }
    /// Check if transfer is completed.
    #[inline]
    pub const fn is_transfer_completed(self) -> bool {
        (self.0 & Self::TRANSFER_COMPLETE) >> 1 == 1
    }
    /// Clear transfer complete bit.
    #[inline]
    pub const fn clear_transfer_completed(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (1 << 1)))
    }
    /// Check if command is completed.
    #[inline]
    pub const fn is_cmd_completed(self) -> bool {
        self.0 & Self::CMD_COMPLETE == 1
    }
    /// Clear command complete bit.
    #[inline]
    pub const fn clear_cmd_completed(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 1))
    }
}

/// Register that shows the defined error interrupt status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptStatus(u16);

impl ErrorInterruptStatus {
    const VENDOR_SPECIFIC_ERROR: u16 = 0xF << 12;
    const TUNING_ERROR: u16 = 0x1 << 10;
    const ADMA_ERROR: u16 = 0x1 << 9;
    const AUTO_CMD_ERROR: u16 = 0x1 << 8;
    const CURRENT_LIMIT_ERROR: u16 = 0x1 << 7;
    const DATA_END_BIT_ERROR: u16 = 0x1 << 6;
    const DATA_CRC_ERROR: u16 = 0x1 << 5;
    const DATA_TIMEOUT_ERROR: u16 = 0x1 << 4;
    const CMD_INDEX_ERROR: u16 = 0x1 << 3;
    const CMD_END_BIT_ERROR: u16 = 0x1 << 2;
    const CMD_CRC_ERROR: u16 = 0x1 << 1;
    const CMD_TIMEOUT_ERROR: u16 = 0x1;

    /// Get vendor specific error status.
    #[inline]
    pub const fn vendor_specific_err(self) -> u8 {
        ((self.0 & Self::VENDOR_SPECIFIC_ERROR) >> 12) as u8
    }
    /// Clear vendor specific error status.
    #[inline]
    pub const fn clear_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0xF << 12)))
    }
    /// Check if tuning error occurs.
    #[inline]
    pub const fn if_tuning_err_occurs(self) -> bool {
        (self.0 & Self::TUNING_ERROR) >> 10 == 1
    }
    /// Clear tuning error bit.
    pub const fn clear_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (1 << 10)))
    }
    /// Check if ADMA error occurs.
    #[inline]
    pub const fn if_adma_err_occurs(self) -> bool {
        (self.0 & Self::ADMA_ERROR) >> 9 == 1
    }
    /// Clear ADMA error bit.
    #[inline]
    pub const fn clear_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (1 << 9)))
    }
    /// Check if auto command error occurs.
    #[inline]
    pub const fn if_auto_cmd_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_ERROR) >> 8 == 1
    }
    /// Clear auto command error bit.
    #[inline]
    pub const fn clear_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (1 << 8)))
    }
    /// Check if current limit error occurs.
    #[inline]
    pub const fn if_current_limit_err_occurs(self) -> bool {
        (self.0 & Self::CURRENT_LIMIT_ERROR) >> 7 == 1
    }
    /// Clear current limit error bit.
    #[inline]
    pub const fn clear_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (1 << 7)))
    }
    /// Check if data end bit error occurs.
    #[inline]
    pub const fn if_data_end_bit_err_occurs(self) -> bool {
        (self.0 & Self::DATA_END_BIT_ERROR) >> 6 == 1
    }
    /// Clear data end bit error bit.
    #[inline]
    pub const fn clear_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (1 << 6)))
    }
    /// Check if data crc error occurs.
    #[inline]
    pub const fn if_data_crc_err_occurs(self) -> bool {
        (self.0 & Self::DATA_CRC_ERROR) >> 5 == 1
    }
    /// Clear data crc error bit.
    #[inline]
    pub const fn clear_data_crc_err(self) -> Self {
        Self((self.0 & Self::DATA_CRC_ERROR) | (Self::DATA_CRC_ERROR & (1 << 5)))
    }
    /// Check if data timeout error occurs.
    #[inline]
    pub const fn if_data_timeout_err_occurs(self) -> bool {
        (self.0 & Self::DATA_TIMEOUT_ERROR) >> 4 == 1
    }
    /// Clear data timeout error bit.
    #[inline]
    pub const fn clear_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUT_ERROR) | (Self::DATA_TIMEOUT_ERROR & (1 << 4)))
    }
    /// Check if command index error occurs.
    #[inline]
    pub const fn if_cmd_index_err_occurs(self) -> bool {
        (self.0 & Self::CMD_INDEX_ERROR) >> 3 == 1
    }
    /// Clear command index error bit.
    #[inline]
    pub const fn clear_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_ERROR) | (Self::CMD_INDEX_ERROR & (1 << 3)))
    }
    /// Check if command end bit error occurs.
    #[inline]
    pub const fn if_cmd_end_bit_err_occurs(self) -> bool {
        (self.0 & Self::CMD_END_BIT_ERROR) >> 2 == 1
    }
    /// Clear command end bit error bit.
    #[inline]
    pub const fn clear_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (1 << 2)))
    }
    /// Check if command crc error occurs.
    #[inline]
    pub const fn if_cmd_crc_err_occurs(self) -> bool {
        (self.0 & Self::CMD_CRC_ERROR) >> 1 == 1
    }
    /// Clear command crc error bit.
    #[inline]
    pub const fn clear_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (1 << 1)))
    }
    /// Check if command timeout error occurs.
    #[inline]
    pub const fn if_cmd_timeout_err_occurs(self) -> bool {
        self.0 & Self::CMD_TIMEOUT_ERROR == 1
    }
    /// Clear command timeout error bit.
    #[inline]
    pub const fn clear_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 1))
    }
}

/// Register that sets to 1 enables normal interrupt status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalInterruptStatusEnable(u16);

impl NormalInterruptStatusEnable {
    const FIXED_TO_ZERO: u16 = 0x1 << 15;
    const RE_TUNING_EVENT: u16 = 0x1 << 12;
    const INT_C: u16 = 0x1 << 11;
    const INT_B: u16 = 0x1 << 10;
    const INT_A: u16 = 0x1 << 9;
    const CARD_INT: u16 = 0x1 << 8;
    const CARD_REMOVAL: u16 = 0x1 << 7;
    const CARD_INSERTION: u16 = 0x1 << 6;
    const BUFFER_READ: u16 = 0x1 << 5;
    const BUFFER_WRITE: u16 = 0x1 << 4;
    const DMA_INT: u16 = 0x1 << 3;
    const BLOCK_GAP: u16 = 0x1 << 2;
    const TRANSFER_COMPLETE: u16 = 0x1 << 1;
    const CMD_COMPLETE: u16 = 0x1 << 0;

    /// This bit should always be fixed to zero.
    #[inline]
    pub const fn is_fixed_to_zero(self) -> bool {
        (self.0 & Self::FIXED_TO_ZERO) >> 15 == 0
    }
    /// Enable re-tuning event status.
    #[inline]
    pub const fn enable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING_EVENT) | (Self::RE_TUNING_EVENT & (1 << 12)))
    }
    /// Disable re-tuning event status.
    #[inline]
    pub const fn disable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING_EVENT) | (Self::RE_TUNING_EVENT & (0 << 12)))
    }
    /// Check if re-tuning event status is enabled.
    #[inline]
    pub const fn is_retuning_enabled(self) -> bool {
        (self.0 & Self::RE_TUNING_EVENT) >> 12 == 1
    }
    /// Enable INT_C status.
    #[inline]
    pub const fn enable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (1 << 11)))
    }
    /// Disable INT_C status.
    #[inline]
    pub const fn disable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (0 << 11)))
    }
    /// Check if INT_C status is enabled.
    #[inline]
    pub const fn is_int_c_enabled(self) -> bool {
        (self.0 & Self::INT_C) >> 11 == 1
    }
    /// Enable INT_B status.
    #[inline]
    pub const fn enable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (1 << 10)))
    }
    /// Disable INT_B status.
    #[inline]
    pub const fn disable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (0 << 10)))
    }
    /// Check if INT_B status is enabled.
    #[inline]
    pub const fn is_int_b_enabled(self) -> bool {
        (self.0 & Self::INT_B) >> 10 == 1
    }
    /// Enable INT_A status.
    #[inline]
    pub const fn enable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (1 << 9)))
    }
    /// Disable INT_A status.
    #[inline]
    pub const fn disable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (0 << 9)))
    }
    /// Check if INT_A Status is enabled.
    #[inline]
    pub const fn is_int_a_enabled(self) -> bool {
        (self.0 & Self::INT_A) >> 9 == 1
    }
    /// Enable card interrupt.
    #[inline]
    pub const fn enable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (1 << 8)))
    }
    /// Disable card interrupt.
    #[inline]
    pub const fn disable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (0 << 8)))
    }
    /// Check if card interrupt is enabled.
    #[inline]
    pub const fn is_card_int_enabled(self) -> bool {
        (self.0 & Self::CARD_INT) >> 8 == 1
    }
    /// Enable card removal status.
    #[inline]
    pub const fn enable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 7)))
    }
    /// Disable card removal status.
    #[inline]
    pub const fn disable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (0 << 7)))
    }
    /// Check if card removal status is enabled.
    #[inline]
    pub const fn is_card_removal_enabled(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 7 == 1
    }
    /// Enable card insertion status.
    #[inline]
    pub const fn enable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (1 << 6)))
    }
    /// Disable card insertion status.
    #[inline]
    pub const fn disable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (0 << 6)))
    }
    /// Check if card insertion status is enabled.
    #[inline]
    pub const fn is_card_insertion_enabled(self) -> bool {
        (self.0 & Self::CARD_INSERTION) >> 6 == 1
    }
    /// Enable buffer read ready status.
    #[inline]
    pub const fn enable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (1 << 5)))
    }
    /// Disable buffer read ready status.
    #[inline]
    pub const fn disable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (0 << 5)))
    }
    /// Check if buffer read ready status is enabled.
    #[inline]
    pub const fn is_buffer_read_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_READ) >> 5 == 1
    }
    /// Enable buffer write ready status.
    #[inline]
    pub const fn enable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (1 << 4)))
    }
    /// Disable buffer write ready status.
    #[inline]
    pub const fn disable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (0 << 4)))
    }
    /// Check if buffer write ready status is enabled.
    #[inline]
    pub const fn is_buffer_write_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_WRITE) >> 4 == 1
    }
    /// Enable DMA interrupt status.
    #[inline]
    pub const fn enable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (1 << 3)))
    }
    /// Disable DMA interrupt status.
    #[inline]
    pub const fn disable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (0 << 3)))
    }
    /// Check if DMA interrupt status is enabled.
    #[inline]
    pub const fn is_dma_int_enabled(self) -> bool {
        (self.0 & Self::DMA_INT) >> 3 == 1
    }
    /// Enable block gap status.
    #[inline]
    pub const fn enable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (1 << 2)))
    }
    /// Disable block gap status.
    #[inline]
    pub const fn disable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (0 << 2)))
    }
    /// Check if block gap status is enabled.
    #[inline]
    pub const fn is_block_gap_enabled(self) -> bool {
        (self.0 & Self::BLOCK_GAP) >> 2 == 1
    }
    /// Enable transfer complete status.
    #[inline]
    pub const fn enable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (1 << 1)))
    }
    /// Disable transfer complete status.
    #[inline]
    pub const fn disable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (0 << 1)))
    }
    /// Check if transfer complete status is enabled.
    #[inline]
    pub const fn is_transfer_complete_enabled(self) -> bool {
        (self.0 & Self::TRANSFER_COMPLETE) >> 1 == 1
    }
    /// Enable command complete status.
    #[inline]
    pub const fn enable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 1))
    }
    /// Disable command complete status.
    #[inline]
    pub const fn disable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 0))
    }
    /// Check if command complete is enabled.
    #[inline]
    pub const fn is_cmd_complete_enabled(self) -> bool {
        self.0 & Self::CMD_COMPLETE == 1
    }
}

/// Register that sets to 1 enables error interrupt status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptStatusEnable(u16);

impl ErrorInterruptStatusEnable {
    const VENDOR_SPECIFIC_ERROR: u16 = 0xF << 12;
    const TUNING_ERROR: u16 = 0x1 << 10;
    const ADMA_ERROR: u16 = 0x1 << 9;
    const AUTO_CMD_ERROR: u16 = 0x1 << 8;
    const CURRENT_LIMIT_ERROR: u16 = 0x1 << 7;
    const DATA_END_BIT_ERROR: u16 = 0x1 << 6;
    const DATA_CRC_ERROR: u16 = 0x1 << 5;
    const DATA_TIMEOUE_ERROR: u16 = 0x1 << 4;
    const CMD_INDEX_ERROR: u16 = 0x1 << 3;
    const CMD_END_BIT_ERROR: u16 = 0x1 << 2;
    const CMD_CRC_ERROR: u16 = 0x1 << 1;
    const CMD_TIMEOUT_ERROR: u16 = 0x1;

    /// Enable vendor specific error status.
    #[inline]
    pub const fn enable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0xF << 12)))
    }
    /// Disable vendor specific error status.
    #[inline]
    pub const fn disable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0 << 12)))
    }
    /// Check if vendor specific error status is enabled.
    #[inline]
    pub const fn is_vendor_specific_err_enabled(self) -> bool {
        (self.0 & Self::VENDOR_SPECIFIC_ERROR) >> 12 == 0xF
    }
    /// Enable tuning error status.
    #[inline]
    pub const fn enable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (1 << 10)))
    }
    /// Disable tuning error status.
    #[inline]
    pub const fn disable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (0 << 10)))
    }
    /// Check if tuning error status is enabled.
    #[inline]
    pub const fn is_tuning_err_enabled(self) -> bool {
        (self.0 & Self::TUNING_ERROR) >> 10 == 1
    }
    /// Enable ADMA error status.
    #[inline]
    pub const fn enable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (1 << 9)))
    }
    /// Disable ADMA error status.
    #[inline]
    pub const fn disable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (0 << 9)))
    }
    /// Check if ADMA error status is enabled.
    #[inline]
    pub const fn is_adma_err_enabled(self) -> bool {
        (self.0 & Self::ADMA_ERROR) >> 9 == 1
    }
    /// Enable auto command error status.
    #[inline]
    pub const fn enable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (1 << 8)))
    }
    /// Disable auto command error status.
    #[inline]
    pub const fn disable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (0 << 8)))
    }
    /// Check if auto command error status is enabled.
    #[inline]
    pub const fn is_auto_cmd_err_enabled(self) -> bool {
        (self.0 & Self::AUTO_CMD_ERROR) >> 8 == 1
    }
    /// Enable current limit error status.
    #[inline]
    pub const fn enable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (1 << 7)))
    }
    /// Disable current limit error status.
    #[inline]
    pub const fn disable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (0 << 7)))
    }
    /// Check if current limit error status is enabled.
    #[inline]
    pub const fn is_current_limit_err_enabled(self) -> bool {
        (self.0 & Self::CURRENT_LIMIT_ERROR) >> 7 == 1
    }
    /// Enable data end bit error Status.
    #[inline]
    pub const fn enable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (1 << 6)))
    }
    /// Disable data end bit error Status.
    #[inline]
    pub const fn disable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (0 << 6)))
    }
    /// Check if data end bit error status is enabled.
    #[inline]
    pub const fn is_data_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::DATA_END_BIT_ERROR) >> 6 == 1
    }
    /// Enable data crc error status.
    #[inline]
    pub const fn enable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_ERROR) | (Self::DATA_CRC_ERROR & (1 << 5)))
    }
    /// Disable data crc error status.
    #[inline]
    pub const fn disable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_ERROR) | (Self::DATA_CRC_ERROR & (0 << 5)))
    }
    /// Check if data crc error status is enabled.
    #[inline]
    pub const fn is_data_crc_err_enabled(self) -> bool {
        (self.0 & Self::DATA_CRC_ERROR) >> 5 == 1
    }
    /// Enable data timeout error status.
    #[inline]
    pub const fn enable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (1 << 4)))
    }
    /// Disable data timeout error status.
    #[inline]
    pub const fn disable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (0 << 4)))
    }
    /// Check if data timeout error status is enabled.
    #[inline]
    pub const fn is_data_timeout_err_enabled(self) -> bool {
        (self.0 & Self::DATA_TIMEOUE_ERROR) >> 4 == 1
    }
    /// Enable command index error status.
    #[inline]
    pub const fn enable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_ERROR) | (Self::CMD_INDEX_ERROR & (1 << 3)))
    }
    /// Disable command index error status.
    #[inline]
    pub const fn disable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_ERROR) | (Self::CMD_INDEX_ERROR & (0 << 3)))
    }
    /// Check if command index error status is enabled.
    #[inline]
    pub const fn is_cmd_index_err_enabled(self) -> bool {
        (self.0 & Self::CMD_INDEX_ERROR) >> 3 == 1
    }
    /// Enable command end bit error status.
    #[inline]
    pub const fn enable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (1 << 2)))
    }
    /// Disable command end bit error status.
    #[inline]
    pub const fn disable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (0 << 2)))
    }
    /// Check if command end bit error status is enabled.
    #[inline]
    pub const fn is_cmd_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::CMD_END_BIT_ERROR) >> 2 == 1
    }
    /// Enable command crc error status.
    #[inline]
    pub const fn enable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (1 << 1)))
    }
    /// Disable command crc error status.
    #[inline]
    pub const fn disable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (0 << 1)))
    }
    /// Check if command crc error status is enabled.
    #[inline]
    pub const fn is_cmd_crc_err_enabled(self) -> bool {
        (self.0 & Self::CMD_CRC_ERROR) >> 1 == 1
    }
    /// Enable command timeout error status.
    #[inline]
    pub const fn enable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 1))
    }
    /// Disable command timeout error status.
    #[inline]
    pub const fn disable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 0))
    }
    /// Check if command timeout error status is enabled.
    #[inline]
    pub const fn is_cmd_timeout_err_enabled(self) -> bool {
        self.0 & Self::CMD_TIMEOUT_ERROR == 1
    }
}

/// Register that selects which interrupt status is indicated to the host system as the interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalInterruptSignalEnable(u16);

impl NormalInterruptSignalEnable {
    const FIXED_TO_ZERO: u16 = 0x1 << 15;
    const RE_TUNING: u16 = 0x1 << 12;
    const INT_C: u16 = 0x1 << 11;
    const INT_B: u16 = 0x1 << 10;
    const INT_A: u16 = 0x1 << 9;
    const CARD_INT: u16 = 0x1 << 8;
    const CARD_REMOVAL: u16 = 0x1 << 7;
    const CARD_INSERTION: u16 = 0x1 << 6;
    const BUFFER_READ: u16 = 0x1 << 5;
    const BUFFER_WRITE: u16 = 0x1 << 4;
    const DMA_INT: u16 = 0x1 << 3;
    const BLOCK_GAP: u16 = 0x1 << 2;
    const TRANSFER_COMPLETE: u16 = 0x1 << 1;
    const CMD_COMPLETE: u16 = 0x1;

    /// This bit should always be fixed to zero.
    #[inline]
    pub const fn is_fixed_to_zero(self) -> bool {
        (self.0 & Self::FIXED_TO_ZERO) >> 15 == 0
    }
    /// Enable re-tuning event signal.
    #[inline]
    pub const fn enable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING) | (Self::RE_TUNING & (1 << 12)))
    }
    /// Disable re-tuning event signal.
    #[inline]
    pub const fn disable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING) | (Self::RE_TUNING & (0 << 12)))
    }
    /// Check if re-tuning event signal is enabled.
    #[inline]
    pub const fn is_re_tuning_enabled(self) -> bool {
        (self.0 & Self::RE_TUNING) >> 12 == 1
    }
    /// Enable INT_C signal.
    #[inline]
    pub const fn enable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (1 << 11)))
    }
    /// Disable INT_C signal.
    #[inline]
    pub const fn disable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (0 << 11)))
    }
    /// Check if INT_C signal is enabled.
    #[inline]
    pub const fn is_int_c_enabled(self) -> bool {
        (self.0 & Self::INT_C) >> 11 == 1
    }
    /// Enable INT_B signal.
    #[inline]
    pub const fn enable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (1 << 10)))
    }
    /// Disable INT_B signal.
    #[inline]
    pub const fn disable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (0 << 10)))
    }
    /// Check if INT_B signal is enabled.
    #[inline]
    pub const fn is_int_b_enabled(self) -> bool {
        (self.0 & Self::INT_B) >> 10 == 1
    }
    /// Enable INT_A signal.
    #[inline]
    pub const fn enable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (1 << 9)))
    }
    /// Disable INT_A signal.
    #[inline]
    pub const fn disable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (0 << 9)))
    }
    /// Check if INT_A signal is enabled.
    #[inline]
    pub const fn is_int_a_enabled(self) -> bool {
        (self.0 & Self::INT_A) >> 9 == 1
    }
    /// Enable card interrupt signal.
    #[inline]
    pub const fn enable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (1 << 8)))
    }
    /// Disable card interrupt signal.
    #[inline]
    pub const fn disable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (0 << 8)))
    }
    /// Check if card interrupt signal is enabled.
    #[inline]
    pub const fn is_card_int_enabled(self) -> bool {
        (self.0 & Self::CARD_INT) >> 8 == 1
    }
    /// Enable card removal signal.
    #[inline]
    pub const fn enable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 7)))
    }
    /// Disable card removal signal.
    #[inline]
    pub const fn disable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (0 << 7)))
    }
    /// Check if card removal signal is enabled.
    #[inline]
    pub const fn is_card_removal_enabled(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 7 == 1
    }
    /// Enable card insertion signal.
    #[inline]
    pub const fn enable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (1 << 6)))
    }
    /// Disable card insertion signal.
    #[inline]
    pub const fn disable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (0 << 6)))
    }
    /// Check if card insertion signal is enabled.
    #[inline]
    pub const fn is_card_insertion_enabled(self) -> bool {
        (self.0 & Self::CARD_INSERTION) >> 6 == 1
    }
    /// Enable buffer read ready signal.
    #[inline]
    pub const fn enable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (1 << 5)))
    }
    /// Disable buffer read ready signal.
    #[inline]
    pub const fn disable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (0 << 5)))
    }
    /// Check if buffer read ready signal is enabled.
    #[inline]
    pub const fn is_buffer_read_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_READ) >> 5 == 1
    }
    /// Enable buffer write ready signal.
    #[inline]
    pub const fn enable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (1 << 4)))
    }
    /// Disable buffer write ready signal.
    #[inline]
    pub const fn disable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (0 << 4)))
    }
    /// Check if buffer write ready signal is enabled.
    #[inline]
    pub const fn is_buffer_write_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_WRITE) >> 4 == 1
    }
    /// Enable DMA interrupt signal.
    #[inline]
    pub const fn enable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (1 << 3)))
    }
    /// Disable DMA interrupt signal.
    #[inline]
    pub const fn disable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (0 << 3)))
    }
    /// Check if DMA interrupt signal is enabled.
    #[inline]
    pub const fn is_dma_int_enabled(self) -> bool {
        (self.0 & Self::DMA_INT) >> 3 == 1
    }
    /// Enable block gap event signal.
    #[inline]
    pub const fn enable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (1 << 2)))
    }
    /// Disable block gap event signal.
    #[inline]
    pub const fn disable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (0 << 2)))
    }
    /// Check if block gap event signal is enabled.
    #[inline]
    pub const fn is_block_gap_enabled(self) -> bool {
        (self.0 & Self::BLOCK_GAP) >> 2 == 1
    }
    /// Enable transfer complete signal.
    #[inline]
    pub const fn enable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (1 << 1)))
    }
    /// Disable transfer complete signal.
    #[inline]
    pub const fn disable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (0 << 1)))
    }
    /// Check if transfer complete signal is enabled.
    #[inline]
    pub const fn is_transfer_complete_enabled(self) -> bool {
        (self.0 & Self::TRANSFER_COMPLETE) >> 1 == 1
    }
    /// Enable command complete signal.
    #[inline]
    pub const fn enable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 1))
    }
    /// Disable command complete signal.
    #[inline]
    pub const fn disable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 0))
    }
    /// Check if command complete signal is enabled.
    #[inline]
    pub const fn is_cmd_complete_enabled(self) -> bool {
        self.0 & Self::CMD_COMPLETE == 1
    }
}

/// Register that selects which interrupt status is notified to the host system as the interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptSignalEnable(u16);

impl ErrorInterruptSignalEnable {
    const VENDOR_SPECIFIC_ERROR: u16 = 0xF << 12;
    const TUNING_ERROR: u16 = 0x1 << 10;
    const ADMA_ERROR: u16 = 0x1 << 9;
    const AUTO_CMD_ERROR: u16 = 0x1 << 8;
    const CURRENT_LIMIT_ERROR: u16 = 0x1 << 7;
    const DATA_END_BIT_ERROR: u16 = 0x1 << 6;
    const DATA_CRC_EROOR: u16 = 0x1 << 5;
    const DATA_TIMEOUE_ERROR: u16 = 0x1 << 4;
    const CMD_INDEX_EROOR: u16 = 0x1 << 3;
    const CMD_END_BIT_ERROR: u16 = 0x1 << 2;
    const CMD_CRC_ERROR: u16 = 0x1 << 1;
    const CMD_TIMEOUT_ERROR: u16 = 0x1;

    /// Enable vendor specific error signal.
    #[inline]
    pub const fn enable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0xF << 12)))
    }
    /// Disable vendor specific error signal.
    #[inline]
    pub const fn disable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0 << 12)))
    }
    /// Check if vendor specific error signal is enabled.
    #[inline]
    pub const fn is_vendor_specific_err_enabled(self) -> bool {
        (self.0 & Self::VENDOR_SPECIFIC_ERROR) >> 12 == 0xF
    }
    /// Enable tuning error signal.
    #[inline]
    pub const fn enable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (1 << 10)))
    }
    /// Disable tuning error signal.
    #[inline]
    pub const fn disable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (0 << 10)))
    }
    /// Check if tuning error signal is enabled.
    #[inline]
    pub const fn is_tuning_err_enabled(self) -> bool {
        (self.0 & Self::TUNING_ERROR) >> 10 == 1
    }
    /// Enable ADMA error signal.
    #[inline]
    pub const fn enable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (1 << 9)))
    }
    /// Disable ADMA error signal.
    #[inline]
    pub const fn disable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (0 << 9)))
    }
    /// Check if ADMA error signal is enabled.
    #[inline]
    pub const fn is_adma_err_enabled(self) -> bool {
        (self.0 & Self::ADMA_ERROR) >> 9 == 1
    }
    /// Enable auto command error signal.
    #[inline]
    pub const fn enable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (1 << 8)))
    }
    /// Disable auto command error signal.
    #[inline]
    pub const fn disable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (0 << 8)))
    }
    /// Check if auto command error signal is enabled.
    #[inline]
    pub const fn is_auto_cmd_err_enabled(self) -> bool {
        (self.0 & Self::AUTO_CMD_ERROR) >> 8 == 1
    }
    /// Enable current limit error signal.
    #[inline]
    pub const fn enable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (1 << 7)))
    }
    /// Disable current limit error signal.
    #[inline]
    pub const fn disable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (0 << 7)))
    }
    /// Check if current limit error signal is enabled.
    #[inline]
    pub const fn is_current_limit_err_enabled(self) -> bool {
        (self.0 & Self::CURRENT_LIMIT_ERROR) >> 7 == 1
    }
    /// Enable data end bit error signal.
    #[inline]
    pub const fn enable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (1 << 6)))
    }
    /// Disable data end bit error signal.
    #[inline]
    pub const fn disable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (0 << 6)))
    }
    /// Check if data end bit error signal is enabled.
    #[inline]
    pub const fn is_data_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::DATA_END_BIT_ERROR) >> 6 == 1
    }
    /// Enable data crc error signal.
    #[inline]
    pub const fn enable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_EROOR) | (Self::DATA_CRC_EROOR & (1 << 5)))
    }
    /// Disable data crc error signal.
    #[inline]
    pub const fn disable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_EROOR) | (Self::DATA_CRC_EROOR & (0 << 5)))
    }
    /// Check if data crc error signal is enabled.
    #[inline]
    pub const fn is_data_crc_err_enabled(self) -> bool {
        (self.0 & Self::DATA_CRC_EROOR) >> 5 == 1
    }
    /// Enable data timeout error signal.
    #[inline]
    pub const fn enable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (1 << 4)))
    }
    /// Disable data timeout error signal.
    #[inline]
    pub const fn disable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (0 << 4)))
    }
    /// Check if data timeout error signal is enabled.
    #[inline]
    pub const fn is_data_timeout_err_enabled(self) -> bool {
        (self.0 & Self::DATA_TIMEOUE_ERROR) >> 4 == 1
    }
    /// Enable command index error signal.
    #[inline]
    pub const fn enable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_EROOR) | (Self::CMD_INDEX_EROOR & (1 << 3)))
    }
    /// Disable command index error signal.
    #[inline]
    pub const fn disable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_EROOR) | (Self::CMD_INDEX_EROOR & (0 << 3)))
    }
    /// Check if command index error signal is enabled.
    #[inline]
    pub const fn is_cmd_index_err_enabled(self) -> bool {
        (self.0 & Self::CMD_INDEX_EROOR) >> 3 == 1
    }
    /// Enable command end bit error signal.
    #[inline]
    pub const fn enable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (1 << 2)))
    }
    /// Disable command end bit error signal.
    #[inline]
    pub const fn disable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (0 << 2)))
    }
    /// Check if command end bit error signal is enabled.
    #[inline]
    pub const fn is_cmd_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::CMD_END_BIT_ERROR) >> 2 == 1
    }
    /// Enable command crc error signal.
    #[inline]
    pub const fn enable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (1 << 1)))
    }
    /// Disable command crc error signal.
    #[inline]
    pub const fn disable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (0 << 1)))
    }
    /// Check if command crc error signal is enabled.
    #[inline]
    pub const fn is_cmd_crc_err_enabled(self) -> bool {
        (self.0 & Self::CMD_CRC_ERROR) >> 1 == 1
    }
    /// Enable command timeout error signal.
    #[inline]
    pub const fn enable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 1))
    }
    /// Disable command timeout error signal.
    #[inline]
    pub const fn disable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 0))
    }
    /// Check if command timeout error signal is enabled.
    #[inline]
    pub const fn is_cmd_timeout_err_enabled(self) -> bool {
        self.0 & Self::CMD_TIMEOUT_ERROR == 1
    }
}

/// Register that indicates CMD12 response error of auto CMD12 and CMD23 response error of auto CMD23.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AutoCmdErrorStatus(u16);

impl AutoCmdErrorStatus {
    const CMD_NOT_ISSUED: u16 = 0x1 << 7;
    const AUTO_CMD_INDEX_ERROR: u16 = 0x1 << 4;
    const AUTO_CMD_END_BIT_ERROR: u16 = 0x1 << 3;
    const AUTO_CMD_CRC_ERROR: u16 = 0x1 << 2;
    const AUTO_CMD_TIMEOUT_ERROR: u16 = 0x1 << 1;
    const AUTO_CMD_NOT_EXECUTED: u16 = 0x1;

    /// Check if command is not issued by auto CMD12.
    #[inline]
    pub const fn is_cmd_not_issued(self) -> bool {
        (self.0 & Self::CMD_NOT_ISSUED) >> 7 == 1
    }
    /// Check if auto command index error occurs.
    #[inline]
    pub const fn if_auto_cmd_index_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_INDEX_ERROR) >> 4 == 1
    }
    /// Check if auto command end bit error occurs.
    #[inline]
    pub const fn if_auto_cmd_end_bit_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_END_BIT_ERROR) >> 3 == 1
    }
    /// Check if auto command crc error occurs.
    #[inline]
    pub const fn if_auto_cmd_crc_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_CRC_ERROR) >> 2 == 1
    }
    /// Check if auto command timeout error occurs.
    #[inline]
    pub const fn if_auto_cmd_timeout_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_TIMEOUT_ERROR) >> 1 == 1
    }
    /// Check if auto CMD12 is not executed.
    #[inline]
    pub const fn is_auto_cmd12_not_executed(self) -> bool {
        self.0 & Self::AUTO_CMD_NOT_EXECUTED == 1
    }
}

/// Host control 2 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControl2(u16);

impl HostControl2 {
    const PRESET_VAL_EN: u16 = 0x1 << 15;
    const ASYNCHRONOUS_INT_EN: u16 = 0x1 << 14;
    const SAMPLING_CLK_SELECT: u16 = 0x1 << 7;
    const EXECUTE_TUNING: u16 = 0x1 << 6;
    const DRIVER_STRENGTH_SELECT: u16 = 0x3 << 4;
    const SIGNALING_1_8_V_EN: u16 = 0x1 << 3;
    const UHS_MODE_SELECT: u16 = 0x7;

    /// Enable preset value.
    #[inline]
    pub const fn enable_preset_val(self) -> Self {
        Self((self.0 & !Self::PRESET_VAL_EN) | (Self::PRESET_VAL_EN & (1 << 15)))
    }
    /// Disable preset value.
    #[inline]
    pub const fn disable_preset_val(self) -> Self {
        Self((self.0 & !Self::PRESET_VAL_EN) | (Self::PRESET_VAL_EN & (0 << 15)))
    }
    /// Check if preset value is enabled.
    #[inline]
    pub const fn is_preset_val_enabled(self) -> bool {
        (self.0 & Self::PRESET_VAL_EN) >> 15 == 1
    }
    /// Enable asynchronous interrupt.
    #[inline]
    pub const fn enable_async_int(self) -> Self {
        Self((self.0 & !Self::ASYNCHRONOUS_INT_EN) | (Self::ASYNCHRONOUS_INT_EN & (1 << 14)))
    }
    /// Disable asynchronous interrupt.
    #[inline]
    pub const fn disable_async_int(self) -> Self {
        Self((self.0 & !Self::ASYNCHRONOUS_INT_EN) | (Self::ASYNCHRONOUS_INT_EN & (0 << 14)))
    }
    /// Check if asynchronous interrupt is enabled.
    #[inline]
    pub const fn is_async_int_enabled(self) -> bool {
        (self.0 & Self::ASYNCHRONOUS_INT_EN) >> 14 == 1
    }
    /// Set sampling clock select bit.
    /// Host controller uses this bit to select sampling clock to receive command and data.
    /// Setting 1 means that tuning is completed successfully and setting 0 means that tuning is failed.
    #[inline]
    pub const fn set_sample_clk_select(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::SAMPLING_CLK_SELECT)
                | (Self::SAMPLING_CLK_SELECT & ((val as u16) << 7)),
        )
    }
    /// Get sampling clock select bit.
    #[inline]
    pub const fn sample_clk_select(self) -> u8 {
        ((self.0 & Self::SAMPLING_CLK_SELECT) >> 7) as u8
    }
    /// Check if tuning is completed.
    #[inline]
    pub const fn is_tuning_completed(self) -> bool {
        (self.0 & Self::SAMPLING_CLK_SELECT) >> 7 == 1
    }
    /// Start tuning process.
    #[inline]
    pub const fn start_tuning(self) -> Self {
        Self((self.0 & !Self::EXECUTE_TUNING) | (Self::EXECUTE_TUNING & (1 << 6)))
    }
    /// Check if tuning process is finished.
    #[inline]
    pub const fn is_tuning_finished(self) -> bool {
        (self.0 & Self::EXECUTE_TUNING) >> 6 == 0
    }
    /// Set driver strength select bit.
    #[inline]
    pub const fn set_driver_strength_select(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::DRIVER_STRENGTH_SELECT)
                | (Self::DRIVER_STRENGTH_SELECT & ((val as u16) << 4)),
        )
    }
    /// Get driver strength select bit.
    #[inline]
    pub const fn driver_strength_select(self) -> u8 {
        ((self.0 & Self::DRIVER_STRENGTH_SELECT) >> 4) as u8
    }
    /// Change card signal voltage from 3.3v to 1.8v.
    #[inline]
    pub const fn change_3_3v_to_1_8v(self) -> Self {
        Self((self.0 & !Self::SIGNALING_1_8_V_EN) | (Self::SIGNALING_1_8_V_EN & (1 << 3)))
    }
    /// Check if changing card signal voltage from 3.3v to 1.8v is finished.
    #[inline]
    pub const fn is_3_3v_to_1_8v_finished(self) -> bool {
        (self.0 & Self::SIGNALING_1_8_V_EN) >> 3 == 1
    }
    /// Change card signal voltage from 1.8v to 3.3v.
    #[inline]
    pub const fn change_1_8v_to_3_3v(self) -> Self {
        Self((self.0 & !Self::SIGNALING_1_8_V_EN) | (Self::SIGNALING_1_8_V_EN & (0 << 3)))
    }
    /// Check if changing card signal voltage from 1.8v to 3.3v is finished.
    #[inline]
    pub const fn is_1_8v_to_3_3v_finished(self) -> bool {
        (self.0 & Self::SIGNALING_1_8_V_EN) >> 3 == 0
    }
    /// Set UHS mode select bit.
    #[inline]
    pub const fn set_uhs_mode(self, val: u8) -> Self {
        Self((self.0 & !Self::UHS_MODE_SELECT) | (Self::UHS_MODE_SELECT & (val as u16)))
    }
    /// Get UHS mode select bit.
    #[inline]
    pub const fn uhs_mode(self) -> u8 {
        (self.0 & Self::UHS_MODE_SELECT) as u8
    }
}

/// Register that provides the host driver with information specific to the host controller implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Capabilities(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlotType {
    /// Removable card slot.
    RemovableCardSlot,
    /// Embedded slot for one device.
    EmbeddedSlotforOneDevice,
    /// Shared bus slot.
    SharedBusSlot,
}

impl Capabilities {
    // Capabilities_1.
    const SLOT_TYPE: u64 = 0x3 << 62;
    const ASYNCHRONOUS_INT: u64 = 0x1 << 61;
    const BIT_64_SYS_SUPPORT: u64 = 0x1 << 60;
    const VOLTAGE_SUPPORT_1_8_V: u64 = 0x1 << 58;
    const VOLTAGE_SUPPORT_3_0_V: u64 = 0x1 << 57;
    const VOLTAGE_SUPPORT_3_3_V: u64 = 0x1 << 56;
    const SUSPEND_RESUME_SUPPORT: u64 = 0x1 << 55;
    const SDMA_SUPOORT: u64 = 0x1 << 54;
    const HIGH_SPEED_SUPPORT: u64 = 0x1 << 53;
    const ADMA2_SUPPORT: u64 = 0x1 << 51;
    const BIT_8_SUPPORT: u64 = 0x1 << 50;
    const MAX_BLOCK_LEN: u64 = 0x3 << 48;
    const BASE_CLK_FREQ: u64 = 0xFF << 40;
    const TIMEOUT_CLK_UNIT: u64 = 0x1 << 39;
    const TIMEOUT_CLK_FREQ: u64 = 0x3F << 32;
    // Capabilities_2.
    const CLK_MULTIPLIER: u64 = 0xFF << 16;
    const RE_TUNING_MODES: u64 = 0x3 << 14;
    const USE_TUNING: u64 = 0x1 << 13;
    const TIM_CNT_FOR_RETUNING: u64 = 0xF << 8;
    const DRIVER_TYPE_D_SUPPORT: u64 = 0x1 << 6;
    const DRIVER_TYPE_C_SUPPORT: u64 = 0x1 << 5;
    const DRIVER_TYPE_A_SUPPORT: u64 = 0x1 << 4;
    const DDR50_SUPPORT: u64 = 0x1 << 2;
    const SDR104_SUPPORT: u64 = 0x1 << 1;
    const SDR50_SUPPORT: u64 = 0x1;

    /// Get slot type.
    #[inline]
    pub const fn slot_type(self) -> SlotType {
        match (self.0 & Self::SLOT_TYPE) >> 62 {
            0 => SlotType::RemovableCardSlot,
            1 => SlotType::EmbeddedSlotforOneDevice,
            2 => SlotType::SharedBusSlot,
            _ => unreachable!(),
        }
    }
    /// Check if asynchronous interrupt is supported.
    #[inline]
    pub const fn is_async_int_supported(self) -> bool {
        (self.0 & Self::ASYNCHRONOUS_INT) >> 61 == 1
    }
    /// Check if 64-bit system bus is supported.
    #[inline]
    pub const fn is_64_bit_bus_supported(self) -> bool {
        (self.0 & Self::BIT_64_SYS_SUPPORT) >> 60 == 1
    }
    /// Check if voltage 1.8v is supported.
    #[inline]
    pub const fn is_1_8v_supported(self) -> bool {
        (self.0 & Self::VOLTAGE_SUPPORT_1_8_V) >> 58 == 1
    }
    /// Check if voltage 3.0v is supported.
    #[inline]
    pub const fn is_3_0v_supported(self) -> bool {
        (self.0 & Self::VOLTAGE_SUPPORT_3_0_V) >> 57 == 1
    }
    /// Check if voltage 3.3v is supported.
    #[inline]
    pub const fn is_3_3v_supported(self) -> bool {
        (self.0 & Self::VOLTAGE_SUPPORT_3_3_V) >> 56 == 1
    }
    /// Check if suspend/resume is supported.
    #[inline]
    pub const fn is_suspend_resume_supported(self) -> bool {
        (self.0 & Self::SUSPEND_RESUME_SUPPORT) >> 55 == 1
    }
    /// Check if SDMA is supported.
    #[inline]
    pub const fn is_sdma_supported(self) -> bool {
        (self.0 & Self::SDMA_SUPOORT) >> 54 == 1
    }
    /// Check if 64-bit high speed is supported.
    #[inline]
    pub const fn is_high_speed_supported(self) -> bool {
        (self.0 & Self::HIGH_SPEED_SUPPORT) >> 53 == 1
    }
    /// Check if ADMA2 is supported.
    #[inline]
    pub const fn is_adma2_supported(self) -> bool {
        (self.0 & Self::ADMA2_SUPPORT) >> 51 == 1
    }
    /// Check if 8-bit bus for embedded device is supported.
    #[inline]
    pub const fn is_8_bit_bus_supported(self) -> bool {
        (self.0 & Self::BIT_8_SUPPORT) >> 50 == 1
    }
    /// Get max block length.
    #[inline]
    pub const fn max_block_len(self) -> u8 {
        ((self.0 & Self::MAX_BLOCK_LEN) >> 48) as u8
    }
    /// Get base clock frequency for SD clock.
    #[inline]
    pub const fn base_clk(self) -> u8 {
        ((self.0 & Self::BASE_CLK_FREQ) >> 40) as u8
    }
    /// Get timeout clock unit.
    #[inline]
    pub const fn timeout_clk_unit(self) -> u8 {
        ((self.0 & Self::TIMEOUT_CLK_UNIT) >> 39) as u8
    }
    /// Get timeout clock frequency.
    #[inline]
    pub const fn timeout_clk_freq(self) -> u8 {
        ((self.0 & Self::TIMEOUT_CLK_FREQ) >> 32) as u8
    }
    /// Get clock multiplier.
    #[inline]
    pub const fn clk_multiplier(self) -> u8 {
        ((self.0 & Self::CLK_MULTIPLIER) >> 16) as u8
    }
    /// Get re-tuning Modes.
    #[inline]
    pub const fn re_tuning_modes(self) -> u8 {
        ((self.0 & Self::RE_TUNING_MODES) >> 14) as u8
    }
    /// Check if use tuning for SDR50 is required.
    #[inline]
    pub const fn is_tuning_for_sdr50_required(self) -> bool {
        (self.0 & Self::USE_TUNING) >> 13 == 1
    }
    /// Get timer count for re-tuning.
    #[inline]
    pub const fn tim_cnt_for_re_tuning(self) -> u8 {
        ((self.0 & Self::TIM_CNT_FOR_RETUNING) >> 8) as u8
    }
    /// Check if driver type d is supported.
    #[inline]
    pub const fn is_driver_type_d_supported(self) -> bool {
        (self.0 & Self::DRIVER_TYPE_D_SUPPORT) >> 6 == 1
    }
    /// Check if driver type c is supported.
    #[inline]
    pub const fn is_driver_type_c_supported(self) -> bool {
        (self.0 & Self::DRIVER_TYPE_C_SUPPORT) >> 5 == 1
    }
    /// Check if driver type a is supported.
    #[inline]
    pub const fn is_driver_type_a_supported(self) -> bool {
        (self.0 & Self::DRIVER_TYPE_A_SUPPORT) >> 4 == 1
    }
    /// Check if DDR50 is supported.
    #[inline]
    pub const fn is_ddr50_supported(self) -> bool {
        (self.0 & Self::DDR50_SUPPORT) >> 2 == 1
    }
    /// Check if SDR104 is supported.
    #[inline]
    pub const fn is_sdr104_supprted(self) -> bool {
        (self.0 & Self::SDR104_SUPPORT) >> 1 == 1
    }
    /// Check if SDR50 is supported.
    #[inline]
    pub const fn is_sdr50_supported(self) -> bool {
        self.0 & Self::SDR50_SUPPORT == 1
    }
}

/// Registers that indicates maximum current capability fo each voltage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MaxCurrentCapabilities(u64);

impl MaxCurrentCapabilities {
    const MAX_CURRENT_1_8_V: u64 = 0xFF << 16;
    const MAX_CURRENT_3_0_V: u64 = 0xFF << 8;
    const MAX_CURRENT_3_3_V: u64 = 0xFF;

    /// Get maximum current for 1.8v.
    #[inline]
    pub const fn max_current_1_8v(self) -> u8 {
        ((self.0 & Self::MAX_CURRENT_1_8_V) >> 16) as u8
    }
    /// Get maximum current for 3.0v.
    #[inline]
    pub const fn max_current_3_0v(self) -> u8 {
        ((self.0 & Self::MAX_CURRENT_3_0_V) >> 8) as u8
    }
    /// Get maximum current for 3.3v.
    #[inline]
    pub const fn max_current_3_3v(self) -> u8 {
        (self.0 & Self::MAX_CURRENT_3_3_V) as u8
    }
}

/// Register that simplifies test of the auto command error status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ForceEventAutoCmdErrorStatus(u16);

impl ForceEventAutoCmdErrorStatus {
    const CMD_NOT_ISSUED: u16 = 0x1 << 7;
    const AUTO_CMD_INDEX_ERROR: u16 = 0x1 << 4;
    const AUTO_CMD_END_BIT_ERROR: u16 = 0x1 << 3;
    const AUTO_CMD_CRC_ERROR: u16 = 0x1 << 2;
    const AUTO_CMD_TIMEOUT_ERROR: u16 = 0x1 << 1;
    const AUTO_CMD_NOT_EXECUTED: u16 = 0x1;

    /// Set force event for command not issued by auto CMD12 error bit.
    #[inline]
    pub const fn set_cmd_not_issued(self, val: u16) -> Self {
        Self((self.0 & !Self::CMD_NOT_ISSUED) | (Self::CMD_NOT_ISSUED & (val << 7)))
    }
    /// Set force event for auto command index error bit.
    #[inline]
    pub const fn set_auto_cmd_index(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_INDEX_ERROR) | (Self::AUTO_CMD_INDEX_ERROR & (val << 4)))
    }
    /// Set force event for auto command end bit error bit.
    #[inline]
    pub const fn set_auto_cmd_end_bit(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_END_BIT_ERROR) | (Self::AUTO_CMD_END_BIT_ERROR & (val << 3)))
    }
    /// Set force event for auto command crc error bit.
    #[inline]
    pub const fn set_auto_cmd_crc(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_CRC_ERROR) | (Self::AUTO_CMD_CRC_ERROR & (val << 2)))
    }
    /// Set force event for auto command timeout error bit.
    #[inline]
    pub const fn set_auto_cmd_timeout(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_TIMEOUT_ERROR) | (Self::AUTO_CMD_TIMEOUT_ERROR & (val << 1)))
    }
    /// Set force event for auto CMD12 not executed bit.
    #[inline]
    pub const fn set_auto_cmd12_not_executed(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_NOT_EXECUTED) | (Self::AUTO_CMD_NOT_EXECUTED & val))
    }
}

/// Register that simplifies test of the error interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ForceEventErrorInterruptStatus(u16);

impl ForceEventErrorInterruptStatus {
    const VENDOR_SPECIFIC_ERROR: u16 = 0xF << 12;
    const ADMA_EROR: u16 = 0x1 << 9;
    const AUTO_CMD_ERROR: u16 = 0x1 << 8;
    const CURRENT_LIMIT_ERROR: u16 = 0x1 << 7;
    const DATA_END_BIT_ERROR: u16 = 0x1 << 6;
    const DATA_CRC_ERROR: u16 = 0x1 << 5;
    const DATA_TIMEOUT_ERROR: u16 = 0x1 << 4;
    const CMD_INDEX_ERROR: u16 = 0x1 << 3;
    const CMD_END_BIT_ERROR: u16 = 0x1 << 2;
    const CMD_CRC_ERROR: u16 = 0x1 << 1;
    const CMD_TIMEOUT_ERROR: u16 = 0x1;

    /// Set force event for vendor specific error status bit.
    #[inline]
    pub const fn set_vendor_specific_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::VENDOR_SPECIFIC_ERROR)
                | (Self::VENDOR_SPECIFIC_ERROR & ((val as u16) << 12)),
        )
    }
    /// Set force event for ADMA error bit.
    #[inline]
    pub const fn set_adma_err(self, val: u8) -> Self {
        Self((self.0 & !Self::ADMA_EROR) | (Self::ADMA_EROR & ((val as u16) << 9)))
    }
    /// Set force event for auto command error bit.
    #[inline]
    pub const fn set_auto_cmd_err(self, val: u8) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & ((val as u16) << 8)))
    }
    /// Set force event for current limit error bit.
    #[inline]
    pub const fn set_current_limit_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::CURRENT_LIMIT_ERROR)
                | (Self::CURRENT_LIMIT_ERROR & ((val as u16) << 7)),
        )
    }
    /// Set force event for data end bit error bit.
    #[inline]
    pub const fn set_data_end_bit_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & ((val as u16) << 6)),
        )
    }
    /// Set force event for data crc error bit.
    #[inline]
    pub const fn set_data_crc_err(self, val: u8) -> Self {
        Self((self.0 & !Self::DATA_CRC_ERROR) | (Self::DATA_CRC_ERROR & ((val as u16) << 5)))
    }
    /// Set force event for data timeout error bit.
    #[inline]
    pub const fn set_data_timeout_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::DATA_TIMEOUT_ERROR) | (Self::DATA_TIMEOUT_ERROR & ((val as u16) << 4)),
        )
    }
    /// Set force event for command index error bit.
    #[inline]
    pub const fn set_cmd_index_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_INDEX_ERROR) | (Self::CMD_INDEX_ERROR & ((val as u16) << 3)))
    }
    /// Set force event for command end bit error bit.
    #[inline]
    pub const fn set_cmd_end_bit_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & ((val as u16) << 2)))
    }
    /// Set force event for command crc error bit.
    #[inline]
    pub const fn set_cmd_crc_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & ((val as u16) << 1)))
    }
    /// Set force event for command timeout error bit.
    #[inline]
    pub const fn set_cmd_timeout_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & (val as u16)))
    }
}

/// Register that holds the ADMA state when ADMA error interrupt is occurred.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Adma2ErrorStatus(u32);

impl Adma2ErrorStatus {
    const ADMA_LEN_MISMATCH: u32 = 0x1 << 2;
    const ADMA_ERROR_STATE: u32 = 0x3;

    /// Check if ADMA length mismatch error occurs.
    #[inline]
    pub const fn if_adma_len_mismatch_err_occurs(self) -> bool {
        (self.0 & Self::ADMA_LEN_MISMATCH) >> 2 == 1
    }
    /// Get ADMA error state.
    #[inline]
    pub const fn adma_err_state(self) -> u8 {
        (self.0 & Self::ADMA_ERROR_STATE) as u8
    }
}

/// Register that contains the physical descriptor address used for ADMA data transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Adma2SystemAddress(u64);

impl Adma2SystemAddress {
    const ADMA_SYSTEM_ADDRESS: u64 = 0xFFFF_FFFF_FFFF_FFFF;

    /// Set ADMA system address.
    #[inline]
    pub const fn set_adma_sys_addr(self, val: u64) -> Self {
        Self((self.0 & !Self::ADMA_SYSTEM_ADDRESS) | (Self::ADMA_SYSTEM_ADDRESS & val))
    }
    /// Get ADMA system address.
    #[inline]
    pub const fn adma_sys_addr(self) -> u64 {
        self.0 & Self::ADMA_SYSTEM_ADDRESS
    }
}

/// Preset value register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PresetValue(u128);

impl PresetValue {
    // Preset value register for DDR50.
    const DDR50_DRV_STRENGTH_VAL: u128 = 0x3 << 126;
    const DDR50_CLKGEN_SEL_VAL: u128 = 0x1 << 122;
    const DDR50_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 112;
    // Preset value register for SDR104.
    const SDR104_DRV_STRENGTH_VAL: u128 = 0x3 << 110;
    const SDR104_CLKGEN_SEL_VAL: u128 = 0x1 << 106;
    const SDR104_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 96;
    // Preset value register for SDR50.
    const SDR50_DRV_STRENGTH_VAL: u128 = 0x3 << 94;
    const SDR50_CLKGEN_SEL_VAL: u128 = 0x1 << 90;
    const SDR50_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 80;
    // Preset value register for SDR25.
    const SDR25_DRV_STRENGTH_VAL: u128 = 0x3 << 78;
    const SDR25_CLKGEN_SEL_VAL: u128 = 0x1 << 74;
    const SDR25_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 64;
    // Preset value register for SDR12.
    const SDR12_DRV_STRENGTH_VAL: u128 = 0x3 << 62;
    const SDR12_CLKGEN_SEL_VAL: u128 = 0x1 << 58;
    const SDR12_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 48;
    // Preset value register for high speed.
    const HS_DRV_STRENGTH_VAL: u128 = 0x3 << 46;
    const HS_CLKGEN_SEL_VAL: u128 = 0x1 << 42;
    const HS_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 32;
    // Preset value register for default speed.
    const DEFAULT_DRV_STRENGTH_VAL: u128 = 0x3 << 30;
    const DEFAULT_CLKGEN_SEL_VAL: u128 = 0x1 << 26;
    const DEFAULT_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 16;
    // Preset value register for initialization.
    const INIT_DRV_STRENGTH_VAL: u128 = 0x3 << 14;
    const INIT_CLKGEN_SEL_VAL: u128 = 0x1 << 10;
    const INIT_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF;

    /// Get driver strength value for DDR50.
    #[inline]
    pub const fn ddr50_drv_strength_val(self) -> u16 {
        ((self.0 & Self::DDR50_DRV_STRENGTH_VAL) >> 126) as u16
    }
    /// Get clock generator frequency select value for DDR50.
    #[inline]
    pub const fn ddr50_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::DDR50_CLKGEN_SEL_VAL) >> 122) as u16
    }
    /// Get SD clock generator frequency select value for DDR50.
    #[inline]
    pub const fn ddr50_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::DDR50_SDCLK_FREQ_SEL_VAL) >> 112) as u16
    }

    /// Get driver strength value For SDR104.
    #[inline]
    pub const fn sdr104_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR104_DRV_STRENGTH_VAL) >> 110) as u16
    }
    /// Get clock generator frequency select value for SDR104.
    #[inline]
    pub const fn sdr104_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR104_CLKGEN_SEL_VAL) >> 106) as u16
    }
    /// Get SD clock generator frequency select value for SDR104.
    #[inline]
    pub const fn sdr104_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR104_SDCLK_FREQ_SEL_VAL) >> 96) as u16
    }

    /// Get driver strength value for SDR50.
    #[inline]
    pub const fn sdr50_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR50_DRV_STRENGTH_VAL) >> 94) as u16
    }
    /// Get clock generator frequency select value for SDR50.
    #[inline]
    pub const fn sdr50_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR50_CLKGEN_SEL_VAL) >> 90) as u16
    }
    /// Get SD clock generator frequency select value for SDR50.
    #[inline]
    pub const fn sdr50_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR50_SDCLK_FREQ_SEL_VAL) >> 80) as u16
    }

    /// Get driver strength value for SDR25.
    #[inline]
    pub const fn sdr25_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR25_DRV_STRENGTH_VAL) >> 78) as u16
    }

    /// Get clock generator frequency select value for SDR25.
    #[inline]
    pub const fn sdr25_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR25_CLKGEN_SEL_VAL) >> 74) as u16
    }
    /// Get SD clock generator frequency select value for SDR25.
    #[inline]
    pub const fn sdr25_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR25_SDCLK_FREQ_SEL_VAL) >> 64) as u16
    }

    /// Get driver Strength value for SDR12.
    #[inline]
    pub const fn sdr12_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR12_DRV_STRENGTH_VAL) >> 62) as u16
    }
    /// Get clock generator frequency select value for SDR12.
    #[inline]
    pub const fn sdr12_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR12_CLKGEN_SEL_VAL) >> 58) as u16
    }
    /// Get SD clock generator frequency select value for SDR12.
    #[inline]
    pub const fn sdr12_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR12_SDCLK_FREQ_SEL_VAL) >> 48) as u16
    }

    /// Get driver strength value for high speed.
    #[inline]
    pub const fn hs_drv_strength_val(self) -> u16 {
        ((self.0 & Self::HS_DRV_STRENGTH_VAL) >> 46) as u16
    }
    /// Get clock generator frequency select value for high speed.
    #[inline]
    pub const fn hs_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::HS_CLKGEN_SEL_VAL) >> 42) as u16
    }
    /// Get SD clock generator frequency select value for high speed.
    #[inline]
    pub const fn hs_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::HS_SDCLK_FREQ_SEL_VAL) >> 32) as u16
    }

    /// Get driver strength value for default speed.
    #[inline]
    pub const fn default_drv_strength_val(self) -> u16 {
        ((self.0 & Self::DEFAULT_DRV_STRENGTH_VAL) >> 30) as u16
    }
    /// Get clock generator frequency select value for default speed.
    #[inline]
    pub const fn default_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::DEFAULT_CLKGEN_SEL_VAL) >> 26) as u16
    }
    /// Get SD clock generator frequency select value for default speed.
    #[inline]
    pub const fn default_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::DEFAULT_SDCLK_FREQ_SEL_VAL) >> 16) as u16
    }

    /// Get driver strength value for initialization.
    #[inline]
    pub const fn init_drv_strength_val(self) -> u16 {
        ((self.0 & Self::INIT_DRV_STRENGTH_VAL) >> 14) as u16
    }
    /// Get clock generator frequency select value for initialization.
    #[inline]
    pub const fn init_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::INIT_CLKGEN_SEL_VAL) >> 10) as u16
    }
    /// Get SD clock generator frequency select value for initialization.
    #[inline]
    pub const fn init_sdclk_freq_clk_val(self) -> u16 {
        (self.0 & Self::INIT_SDCLK_FREQ_SEL_VAL) as u16
    }
}

/// ADMA2 intergrated descriptor address register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMA3IntegratedDescriptorAddress(u64);

impl ADMA3IntegratedDescriptorAddress {
    // TODO
}

/// Shared bus control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SharedBusControl(u32);

impl SharedBusControl {
    const BACK_END_CTRL: u32 = 0x7F << 24;
    const INT_PIN_SEL: u32 = 0x7 << 20;
    const CLK_PIN_SEL: u32 = 0x7 << 16;
    const BUS_WIDTH_PRESET: u32 = 0x7F << 8;
    const NUM_OF_INT_INPUT_PINS: u32 = 0x3 << 4;
    const NUM_OF_CLK_PINS: u32 = 0x7;

    /// Set back-end power control.
    #[inline]
    pub const fn set_back_end_pwr_ctrl(self, val: u8) -> Self {
        Self((self.0 & !Self::BACK_END_CTRL) | (Self::BACK_END_CTRL & ((val as u32) << 24)))
    }
    /// Get back-end power control.
    #[inline]
    pub const fn back_end_pwr_ctrl(self) -> u8 {
        ((self.0 & Self::BACK_END_CTRL) >> 24) as u8
    }
    /// Set interrupt pin select.
    #[inline]
    pub const fn set_int_pin_sel(self, val: u8) -> Self {
        Self((self.0 & !Self::INT_PIN_SEL) | (Self::INT_PIN_SEL & ((val as u32) << 20)))
    }
    /// Get interrupt pin select.
    #[inline]
    pub const fn int_pin_sel(self) -> u8 {
        ((self.0 & Self::INT_PIN_SEL) >> 20) as u8
    }
    /// Set clock pin select.
    #[inline]
    pub const fn set_clk_pin_sel(self, val: u8) -> Self {
        Self((self.0 & !Self::CLK_PIN_SEL) | (Self::CLK_PIN_SEL & ((val as u32) << 16)))
    }
    /// Get clock pin select.
    #[inline]
    pub const fn clk_pin_sel(self) -> u8 {
        ((self.0 & Self::CLK_PIN_SEL) >> 16) as u8
    }
    /// Set bus width preset.
    #[inline]
    pub const fn set_bus_width_preset(self, val: u8) -> Self {
        Self((self.0 & !Self::BUS_WIDTH_PRESET) | (Self::BUS_WIDTH_PRESET & ((val as u32) << 8)))
    }
    /// Get bus width preset.
    #[inline]
    pub const fn bus_width_preset(self) -> u8 {
        ((self.0 & Self::BUS_WIDTH_PRESET) >> 8) as u8
    }
    /// Set number of interrupt input pins.
    #[inline]
    pub const fn set_int_input_pin_num(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::NUM_OF_INT_INPUT_PINS)
                | (Self::NUM_OF_INT_INPUT_PINS & ((val as u32) << 4)),
        )
    }
    /// Get number of interrupt input pins.
    #[inline]
    pub const fn int_input_pin_num(self) -> u8 {
        ((self.0 & Self::NUM_OF_INT_INPUT_PINS) >> 4) as u8
    }
    /// Get number of clock pins.
    #[inline]
    pub const fn clk_pin_num(self) -> u8 {
        (self.0 & Self::NUM_OF_CLK_PINS) as u8
    }
}

/// Slot interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SlotInterruptStatus(u16);

impl SlotInterruptStatus {
    const INT_SIGNAL: u16 = 0xFF;

    /// Get interrupt signal for each slot.
    #[inline]
    pub const fn int_signal(self) -> u16 {
        self.0 & Self::INT_SIGNAL
    }
}

/// Host controller version register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControllerVersion(u16);

/// SD host specification version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecificVersion {
    /// SD host specification version 1.00.
    SDHostSpecificVersion1,
    /// SD host specification version 2.00.
    /// Including the feature of the ADMA and test register.
    SDHostSpecificVersion2,
    /// SD host specification version 3.00.
    SDHostSpecificVersion3,
}

impl HostControllerVersion {
    const VENDOR_VERSION: u16 = 0xFF << 8;
    const SPECIFIC_VERION: u16 = 0xFF;

    /// Get vendor version number.
    /// This status is reserved for the vendor version number.
    /// The host driver should not use this status.
    #[inline]
    pub const fn vendor_version(self) -> u8 {
        ((self.0 & Self::VENDOR_VERSION) >> 8) as u8
    }
    /// Get specification version.
    #[inline]
    pub const fn specific_version(self) -> SpecificVersion {
        match self.0 & Self::SPECIFIC_VERION {
            0 => SpecificVersion::SDHostSpecificVersion1,
            1 => SpecificVersion::SDHostSpecificVersion2,
            2 => SpecificVersion::SDHostSpecificVersion3,
            _ => unreachable!(),
        }
    }
}

/// SD extra parameters register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SDExtraParameters(u32);

impl SDExtraParameters {
    const GEN_PAD_CLK_CNT: u32 = 0xF << 24;
    const GEN_PAD_CLK_ON: u32 = 0x1 << 6;
    const SQU_FULL_CHK: u32 = 0x1 << 5;
    const SQU_EMPTY_CHK: u32 = 0x1 << 4;
    const BOOT_ACK: u32 = 0x1 << 3;

    /// Set generator pad clock counter.
    #[inline]
    pub const fn set_gen_clk_cnt(self, val: u8) -> Self {
        Self((self.0 & !Self::GEN_PAD_CLK_CNT) | (Self::GEN_PAD_CLK_CNT & ((val as u32) << 24)))
    }
    /// Get generator pad clock counter.
    #[inline]
    pub const fn gen_clk_cnt(self) -> u8 {
        ((self.0 & Self::GEN_PAD_CLK_CNT) >> 24) as u8
    }
    /// Set generator pad clock on bit.
    #[inline]
    pub const fn set_gen_clk(self) -> Self {
        Self((self.0 & !Self::GEN_PAD_CLK_ON) | (Self::GEN_PAD_CLK_ON & (1 << 6)))
    }
    /// Unset generator pad clock on bit.
    #[inline]
    pub const fn unset_gen_clk(self) -> Self {
        Self((self.0 & !Self::GEN_PAD_CLK_ON) | (Self::GEN_PAD_CLK_ON & (0 << 6)))
    }
    /// Check if generator pad clock is on.
    #[inline]
    pub const fn is_gen_clk_on(self) -> bool {
        (self.0 & Self::GEN_PAD_CLK_ON) >> 6 == 1
    }
    /// Set SQU full check bit.
    #[inline]
    pub const fn set_squ_full(self) -> Self {
        Self((self.0 & !Self::SQU_FULL_CHK) | (Self::SQU_FULL_CHK & (1 << 5)))
    }
    /// Unset SQU full check bit.
    #[inline]
    pub const fn unset_squ_full(self) -> Self {
        Self((self.0 & !Self::SQU_FULL_CHK) | (Self::SQU_FULL_CHK & (0 << 5)))
    }
    /// Check if SQU is full.
    #[inline]
    pub const fn is_squ_full(self) -> bool {
        (self.0 & Self::SQU_FULL_CHK) >> 5 == 1
    }
    /// Set SQU empty check bit.
    #[inline]
    pub const fn set_squ_empty(self) -> Self {
        Self((self.0 & !Self::SQU_EMPTY_CHK) | (Self::SQU_EMPTY_CHK & (1 << 4)))
    }
    /// Unset SQU empty check bit.
    #[inline]
    pub const fn unset_squ_empty(self) -> Self {
        Self((self.0 & !Self::SQU_EMPTY_CHK) | (Self::SQU_EMPTY_CHK & (0 << 4)))
    }
    /// Check if SQU is Empty.
    #[inline]
    pub const fn is_squ_empty(self) -> bool {
        (self.0 & Self::SQU_EMPTY_CHK) >> 4 == 1
    }
    /// Set boot ack bit.
    #[inline]
    pub const fn set_boot_ack(self) -> Self {
        Self((self.0 & !Self::BOOT_ACK) | (Self::BOOT_ACK & (1 << 3)))
    }
    /// Unset boot ack bit.
    #[inline]
    pub const fn unset_boot_ack(self) -> Self {
        Self((self.0 & !Self::BOOT_ACK) | (Self::BOOT_ACK & (0 << 3)))
    }
    /// Check if boot is fininshed.
    #[inline]
    pub const fn is_boot_finished(self) -> bool {
        (self.0 & Self::BOOT_ACK) >> 3 == 1
    }
}

/// FIFO parameters register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoParameters(u32);

impl FifoParameters {
    const _PRE_GATE_CLK_CNT: u32 = 0xF << 16;
    const _PDLVMC: u32 = 0x1 << 14;
    const _PDFVSSM: u32 = 0x1 << 13;
    const _FORCE_CLK_ON: u32 = 0x1 << 12;
    const _OVRRD_CLK_ON: u32 = 0x1 << 11;
    const _CLK_GATE_ON: u32 = 0x1 << 9;
    const _CLK_GATE_CTL: u32 = 0x1 << 8;
    const _USE_DAT3: u32 = 0x1 << 7;
    const _PDWN: u32 = 0x1 << 6;
    const _FIFO_CS: u32 = 0x1 << 5;
    const _FIFO_CLK: u32 = 0x1 << 4;
    const _WTC: u32 = 0x3 << 2;
    const _RTC: u32 = 0x3;

    // TODO
}

/// SPI mode register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SpiMode(u16);

impl SpiMode {
    const SPI_ERR_TOKEN: u16 = 0x1F << 8;
    const SPI_EN: u16 = 0x1;

    /// Set SPI error token.
    #[inline]
    pub const fn set_spi_err_token(self, val: u16) -> Self {
        Self((self.0 & !Self::SPI_ERR_TOKEN) | (Self::SPI_ERR_TOKEN & (val << 8)))
    }
    /// Get SPI error token.
    #[inline]
    pub const fn spi_err_token(self) -> u16 {
        (self.0 & Self::SPI_ERR_TOKEN) >> 8
    }
    /// Enable SPI.
    #[inline]
    pub const fn enable_spi(self) -> Self {
        Self((self.0 & !Self::SPI_EN) | (Self::SPI_EN & 1))
    }
    /// Disable SPI.
    #[inline]
    pub const fn disable_spi(self) -> Self {
        Self((self.0 & !Self::SPI_EN) | (Self::SPI_EN & 0))
    }
    /// Check if SPI is enabled.
    #[inline]
    pub const fn is_spi_enabled(self) -> bool {
        self.0 & Self::SPI_EN == 1
    }
}

/// Burst size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BurstSize {
    /// Burst size is 32 bytes.
    Bytes32,
    /// Burst size is 64 bytes.
    Bytes64,
    /// Burst size is 128 bytes.
    Bytes128,
    /// Burst size is 256 bytes.
    Bytes256,
}

/// FIFO threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FifoThreshold {
    /// FIFO threshold is 64 bytes to generate DMA request.
    Bytes64,
    /// FIFO threshold is 128 bytes to generate DMA request.
    Bytes128,
    /// FIFO threshold is 192 bytes to generate DMA request.
    Bytes192,
    /// FIFO threshold is 256 bytes to generate DMA request.
    Bytes256,
}

/// Clock and burst size setup register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockAndBurstSizeSetup(u16);

impl ClockAndBurstSizeSetup {
    const _WR_OSTDG: u16 = 0x1 << 15;
    const _RD_OSTDG: u16 = 0x1 << 14;
    const _WR_ENDIAN: u16 = 0x1 << 7;
    const _RD_ENDIAN: u16 = 0x1 << 6;
    const _AXI_NON_POST_WR: u16 = 0x1 << 5;
    const _PRIORITY: u16 = 0x1 << 4;
    const DMA_SIZE: u16 = 0x3 << 2;
    const BURST_SIZE: u16 = 0x3;

    // TODO
    /// Set DMA threshold.
    #[inline]
    pub fn set_fifo_threshold(self, val: FifoThreshold) -> Self {
        Self((self.0 & !Self::DMA_SIZE) | (Self::DMA_SIZE & ((val as u16) << 2)))
    }
    /// Get DMA threshold.
    #[inline]
    pub fn fifo_threshold(self) -> FifoThreshold {
        match ((self.0 & Self::DMA_SIZE) >> 2) as u8 {
            0 => FifoThreshold::Bytes64,
            1 => FifoThreshold::Bytes128,
            2 => FifoThreshold::Bytes192,
            _ => FifoThreshold::Bytes256,
        }
    }
    /// Set burst size.
    #[inline]
    pub fn set_burst_size(self, val: BurstSize) -> Self {
        Self((self.0 & !Self::BURST_SIZE) | (Self::BURST_SIZE & (val as u16)))
    }
    /// Get burst size.
    #[inline]
    pub fn burst_size(self) -> BurstSize {
        match (self.0 & Self::BURST_SIZE) as u8 {
            0 => BurstSize::Bytes32,
            1 => BurstSize::Bytes64,
            2 => BurstSize::Bytes128,
            _ => BurstSize::Bytes256,
        }
    }
}

/// CE-ATA register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CeAta(u32);

impl CeAta {
    const _CHK_CPL: u32 = 0x1 << 31;
    const _SND_CPL: u32 = 0x1 << 30;
    const _CEATA_CARD: u32 = 0x1 << 29;
    const _MMC_CARD: u32 = 0x1 << 28;
    const _MMC_RESETN: u32 = 0x1 << 27;
    const _CPL_COMPLETE: u32 = 0x1 << 22;
    const _CPL_COMPLETE_EN: u32 = 0x1 << 21;
    const _CPL_COMPLETE_INT_EN: u32 = 0x1 << 20;
    const _MISC_INT: u32 = 0x1 << 18;
    const _MISC_INT_EN: u32 = 0x1 << 17;
    const _MISC_INT_INT_EN: u32 = 0x1 << 16;
    const _CPL_TIMEOUT: u32 = 0x3FFF;

    // TODO
}

/// PAD I/O setup register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PadIoSetup(u32);

impl PadIoSetup {
    const _ECO_REG: u32 = 0xF << 16;
    const _INAND_SEL: u32 = 0x1 << 1;
    const _ASYNC_IO_EN: u32 = 0x1;

    // TODO
}

/// RX configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RxConfiguration(u32);

impl RxConfiguration {
    const _TUNING_DLY_INC: u32 = 0x3FF << 18;
    const _SDCLK_DELAY: u32 = 0x3FF << 8;
    const _SDCLK_SEL1: u32 = 0x3 << 2;
    const _SDCLK_SEL0: u32 = 0x3;

    // TODO
}

/// TX configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TxConfiguration(u32);

impl TxConfiguration {
    const _TX_MUX_SEL: u32 = 0x1 << 31;
    const TX_INT_CLK_SEL: u32 = 0x1 << 30;
    const _TX_HOLD_DELAY1: u32 = 0x3FF << 16;
    const _TX_HOLD_DELAY0: u32 = 0x3FF;

    /// Set tx interrupt clock select.
    #[inline]
    pub const fn set_tx_int_clk_sel(self, val: u8) -> Self {
        Self((self.0 & !Self::TX_INT_CLK_SEL) | (Self::TX_INT_CLK_SEL & ((val as u32) << 30)))
    }
    /// Get tx interrupt clock select.
    #[inline]
    pub const fn tx_int_clk_sel(self) -> u8 {
        ((self.0 & Self::TX_INT_CLK_SEL) >> 30) as u8
    }
    // TODO
}

/// Tuning config register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TuningConfiguration(u32);

impl TuningConfiguration {
    const _TUNING_SUCCESS_CNT: u32 = 0x3F << 24;
    const _TUNING_CLK_DLY: u32 = 0x3FF << 14;
    const _TUNING_WD_CNT: u32 = 0x3F << 8;
    const _TUNING_TT_CNT: u32 = 0xFF;

    // TODO
}

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

/// Sleep for n milliseconds.
fn sleep_ms(n: u32) {
    for _ in 0..n * 125 {
        unsafe { asm!("nop") }
    }
}

/// SDH hardware initial config.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Config {
    bus_width_mode: BusWidthMode,
    transfer_width: TransferWidth,
    speed_mode: SpeedMode,
    // TODO: implment more configurations if necessary.
}

impl Config {
    /// Default SDH config.
    #[inline]
    pub const fn default() -> Self {
        Self {
            bus_width_mode: BusWidthMode::SelectByDataTransferWidth,
            transfer_width: TransferWidth::OneBitMode,
            speed_mode: SpeedMode::HighSpeed,
        }
    }
    /// Set bus width mode.
    #[inline]
    pub const fn bus_width_mode(mut self, bus_width_mode: BusWidthMode) -> Self {
        self.bus_width_mode = bus_width_mode;
        self
    }
    /// Set transfer width.
    #[inline]
    pub const fn transfer_width(mut self, transfer_width: TransferWidth) -> Self {
        self.transfer_width = transfer_width;
        self
    }
    /// Set speed mode.
    #[inline]
    pub const fn speed_mode(mut self, speed_mode: SpeedMode) -> Self {
        self.speed_mode = speed_mode;
        self
    }
}

/// Managed Secure Digital Host Controller peripheral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sdh<SDH, PADS, const I: usize> {
    sdh: SDH,
    pads: PADS,
    config: Config,
    block_count: u32,
}

impl<SDH: Deref<Target = RegisterBlock>, PADS, const I: usize> Sdh<SDH, PADS, I> {
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

    /// Release the SDH instance and return the pads and configs.
    #[inline]
    pub fn free(self) -> (SDH, PADS, Config) {
        (self.sdh, self.pads, self.config)
    }
}

impl<SDH: Deref<Target = RegisterBlock>, PADS, const I: usize> BlockDevice for Sdh<SDH, PADS, I> {
    type Error = core::convert::Infallible;

    #[inline]
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            self.read_block(block, start_block_idx.0 + i as u32);
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

/// Parse CSD version 2.0.
#[inline]
fn parse_csd_v2(csd: u128) -> (u32, u32) {
    let csd_structure = (((csd >> (32 * 3)) & 0xC00000) >> 22) as u32;
    let c_size = (((csd >> 32) & 0x3FFFFF00) >> 8) as u32;
    (csd_structure, c_size)
}

/// Valid SDH pads.
pub trait Pads<const I: usize> {}

impl<
    'a,
    'b,
    'c,
    'd,
    'e,
    'f,
    const N1: usize,
    const N2: usize,
    const N3: usize,
    const N4: usize,
    const N5: usize,
    const N6: usize,
> Pads<1>
    for (
        Alternate<'a, N1, gpio::Sdh>,
        Alternate<'b, N2, gpio::Sdh>,
        Alternate<'c, N3, gpio::Sdh>,
        Alternate<'d, N4, gpio::Sdh>,
        Alternate<'e, N5, gpio::Sdh>,
        Alternate<'f, N6, gpio::Sdh>,
    )
where
    Alternate<'a, N1, gpio::Sdh>: HasClkSignal,
    Alternate<'b, N2, gpio::Sdh>: HasCmdSignal,
    Alternate<'c, N3, gpio::Sdh>: HasDat0Signal,
    Alternate<'d, N4, gpio::Sdh>: HasDat1Signal,
    Alternate<'e, N5, gpio::Sdh>: HasDat2Signal,
    Alternate<'f, N6, gpio::Sdh>: HasDat3Signal,
{
}

/// Check if target gpio `Pin` is internally connected to SDH clock signal.
pub trait HasClkSignal {}

impl<'a> HasClkSignal for Alternate<'a, 0, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH command signal.
pub trait HasCmdSignal {}

impl<'a> HasCmdSignal for Alternate<'a, 1, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 0 signal.
pub trait HasDat0Signal {}

impl<'a> HasDat0Signal for Alternate<'a, 2, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 1 signal.
pub trait HasDat1Signal {}

impl<'a> HasDat1Signal for Alternate<'a, 3, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 2 signal.
pub trait HasDat2Signal {}

impl<'a> HasDat2Signal for Alternate<'a, 4, gpio::Sdh> {}

/// Check if target gpio `Pin` is internally connected to SDH data 3 signal.
pub trait HasDat3Signal {}

impl<'a> HasDat3Signal for Alternate<'a, 5, gpio::Sdh> {}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use super::{
        Adma2ErrorStatus, Adma2SystemAddress, Argument, AutoCMDMode, AutoCmdErrorStatus,
        BlockCount, BlockGap, BlockMode, BlockSize, BufferDataPort, BurstSize, BusVoltage,
        BusWidthMode, Capabilities, CardSignal, ClkGenMode, ClockAndBurstSizeSetup, ClockControl,
        CmdType, Command, DataTransferMode, DmaMode, ErrorInterruptSignalEnable,
        ErrorInterruptStatus, ErrorInterruptStatusEnable, FifoThreshold,
        ForceEventAutoCmdErrorStatus, ForceEventErrorInterruptStatus, HostControl1, HostControl2,
        HostControllerVersion, LedState, MaxCurrentCapabilities, NormalInterruptSignalEnable,
        NormalInterruptStatus, NormalInterruptStatusEnable, PowerControl, PresentState,
        PresetValue, Response, ResponseType, SDExtraParameters, SharedBusControl,
        SlotInterruptStatus, SlotType, SoftwareReset, SpecificVersion, SpeedMode, SpiMode,
        SystemAddress, TimeoutControl, TransferMode, TransferWidth, TxConfiguration, WakeupControl,
    };
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, system_address), 0x0);
        assert_eq!(offset_of!(RegisterBlock, block_size), 0x4);
        assert_eq!(offset_of!(RegisterBlock, block_count), 0x06);
        assert_eq!(offset_of!(RegisterBlock, argument), 0x08);
        assert_eq!(offset_of!(RegisterBlock, transfer_mode), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, command), 0x0e);
        assert_eq!(offset_of!(RegisterBlock, response), 0x10);
        assert_eq!(offset_of!(RegisterBlock, buffer_data_port), 0x20);
        assert_eq!(offset_of!(RegisterBlock, present_state), 0x24);
        assert_eq!(offset_of!(RegisterBlock, host_control_1), 0x28);
        assert_eq!(offset_of!(RegisterBlock, power_control), 0x29);
        assert_eq!(offset_of!(RegisterBlock, block_gap), 0x2a);
        assert_eq!(offset_of!(RegisterBlock, wakeup_control), 0x2b);
        assert_eq!(offset_of!(RegisterBlock, clock_control), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, timeout_control), 0x2e);
        assert_eq!(offset_of!(RegisterBlock, software_reset), 0x2f);
        assert_eq!(offset_of!(RegisterBlock, normal_interrupt_status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, error_interrupt_status), 0x32);
        assert_eq!(
            offset_of!(RegisterBlock, normal_interrupt_status_enable),
            0x34
        );
        assert_eq!(
            offset_of!(RegisterBlock, error_interrupt_status_enable),
            0x36
        );
        assert_eq!(
            offset_of!(RegisterBlock, normal_interrupt_signal_enable),
            0x38
        );
        assert_eq!(
            offset_of!(RegisterBlock, error_interrupt_signal_enable),
            0x3a
        );
        assert_eq!(offset_of!(RegisterBlock, auto_cmd_error_status), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, host_control_2), 0x3e);
        assert_eq!(offset_of!(RegisterBlock, capabilities), 0x40);
        assert_eq!(offset_of!(RegisterBlock, max_current_capabilities), 0x48);
        assert_eq!(
            offset_of!(RegisterBlock, force_event_auto_cmd_error_status),
            0x50
        );
        assert_eq!(
            offset_of!(RegisterBlock, force_event_error_interrupt_status),
            0x52
        );
        assert_eq!(offset_of!(RegisterBlock, adma2_error_status), 0x54);
        assert_eq!(offset_of!(RegisterBlock, adma2_system_address), 0x58);
        assert_eq!(offset_of!(RegisterBlock, preset_value), 0x60);
        assert_eq!(
            offset_of!(RegisterBlock, adma3_integrated_descriptor_address),
            0x78
        );
        assert_eq!(offset_of!(RegisterBlock, shared_bus_control), 0xe0);
        assert_eq!(offset_of!(RegisterBlock, slot_interrupt_status), 0xfc);
        assert_eq!(offset_of!(RegisterBlock, host_controller_version), 0xfe);
        assert_eq!(offset_of!(RegisterBlock, sd_extra_parameters), 0x100);
        assert_eq!(offset_of!(RegisterBlock, fifo_parameters), 0x104);
        assert_eq!(offset_of!(RegisterBlock, spi_mode), 0x108);
        assert_eq!(offset_of!(RegisterBlock, clock_and_burst_size_setup), 0x10a);
        assert_eq!(offset_of!(RegisterBlock, ce_ata), 0x10c);
        assert_eq!(offset_of!(RegisterBlock, pad_io_setup), 0x110);
        assert_eq!(offset_of!(RegisterBlock, rx_configuration), 0x114);
        assert_eq!(offset_of!(RegisterBlock, tx_configuration), 0x118);
        assert_eq!(offset_of!(RegisterBlock, tuning_configuration), 0x11c);
    }

    #[test]
    fn struct_system_address_functions() {
        let mut val = SystemAddress(0x0);

        val = val.set_arg2(0xFFFF_FFFF);
        assert_eq!(val.addr(), 0xFFFF_FFFF);
        assert_eq!(val.arg2(), 0xFFFF_FFFF);
        assert_eq!(val.0, 0xFFFF_FFFF);
    }

    #[test]
    fn struct_block_size_functions() {
        let mut val = BlockSize(0x0);

        val = val.set_host_sdma(0x7);
        assert_eq!(val.host_sdma(), 0x7);
        assert_eq!(val.0, 0x7000);

        val = BlockSize(0x0);
        val = val.set_transfer_block(0x0FFF);
        assert_eq!(val.transfer_block(), 0xFFF);
        assert_eq!(val.0, 0xFFF);
    }

    #[test]
    fn struct_block_count_functions() {
        let mut val = BlockCount(0x0);

        val = val.set_blocks_count(0xFFFF);
        assert_eq!(val.blocks_count(), 0xFFFF);
        assert_eq!(val.0, 0xFFFF);
    }

    #[test]
    fn struct_argument_functions() {
        let mut val = Argument(0x0);

        val = val.set_cmd_arg(0xFFFF_FFFF);
        assert_eq!(val.cmd_arg(), 0xFFFF_FFFF);
        assert_eq!(val.0, 0xFFFF_FFFF);
    }

    #[test]
    fn struct_transfer_mode_functions() {
        let mut val = TransferMode(0x0);

        val = val.set_block_mode(BlockMode::MultiBlock);
        assert_eq!(val.block_mode(), BlockMode::MultiBlock);
        assert_eq!(val.0, 0x0020);
        val = val.set_block_mode(BlockMode::Other);
        assert_eq!(val.block_mode(), BlockMode::Other);
        assert_eq!(val.0, 0x0000);
        val = val.set_data_transfer_mode(DataTransferMode::MISO);
        assert_eq!(val.data_transfer_mode(), DataTransferMode::MISO);
        assert_eq!(val.0, 0x0010);
        val = val.set_data_transfer_mode(DataTransferMode::Other);
        assert_eq!(val.data_transfer_mode(), DataTransferMode::Other);
        assert_eq!(val.0, 0x0000);

        val = val.set_auto_cmd_mode(AutoCMDMode::CMD12);
        assert_eq!(val.auto_cmd_mode(), AutoCMDMode::CMD12);
        assert_eq!(val.0, 0x0004);
        val = val.set_auto_cmd_mode(AutoCMDMode::CMD23);
        assert_eq!(val.auto_cmd_mode(), AutoCMDMode::CMD23);
        assert_eq!(val.0, 0x0008);

        val = TransferMode(0x0);
        val = val.enable_block_count();
        assert!(val.is_block_count_enabled());
        assert_eq!(val.0, 0x0002);
        val = val.disable_block_count();
        assert!(!val.is_block_count_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_dma();
        assert!(val.is_dma_enabled());
        assert_eq!(val.0, 0x0001);
        val = val.disable_dma();
        assert!(!val.is_dma_enabled());
        assert_eq!(val.0, 0x0000);
    }

    #[test]
    fn struct_command_functions() {
        let mut val = Command(0x0);

        val = val.set_cmd_idx(0x3F);
        assert_eq!(val.cmd_idx(), 0x3F);
        assert_eq!(val.0, 0x3F00);

        val = Command(0x0);
        val = val.set_cmd_type(CmdType::Abort);
        assert_eq!(val.cmd_type(), CmdType::Abort);
        assert_eq!(val.0, 0x00C0);
        val = val.set_cmd_type(CmdType::Resume);
        assert_eq!(val.cmd_type(), CmdType::Resume);
        assert_eq!(val.0, 0x0080);
        val = val.set_cmd_type(CmdType::Suspend);
        assert_eq!(val.cmd_type(), CmdType::Suspend);
        assert_eq!(val.0, 0x0040);
        val = val.set_cmd_type(CmdType::Normal);
        assert_eq!(val.cmd_type(), CmdType::Normal);
        assert_eq!(val.0, 0x0000);

        val = val.set_data_present();
        assert!(val.is_data_present());
        assert_eq!(val.0, 0x0020);
        val = val.unset_data_present();
        assert!(!val.is_data_present());
        assert_eq!(val.0, 0x0000);

        val = val.enable_index_check();
        assert!(val.is_index_check_enabled());
        assert_eq!(val.0, 0x0010);
        val = val.disable_index_check();
        assert!(!val.is_index_check_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_crc();
        assert!(val.is_cmd_crc_enabled());
        assert_eq!(val.0, 0x0008);
        val = val.disable_cmd_crc();
        assert!(!val.is_cmd_crc_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.set_response_type(ResponseType::ResponseLen136);
        assert_eq!(val.response_type(), ResponseType::ResponseLen136);
        assert_eq!(val.0, 0x0001);
        val = val.set_response_type(ResponseType::ResponseLen48);
        assert_eq!(val.response_type(), ResponseType::ResponseLen48);
        assert_eq!(val.0, 0x0002);
        val = val.set_response_type(ResponseType::ResponseLen48Check);
        assert_eq!(val.response_type(), ResponseType::ResponseLen48Check);
        assert_eq!(val.0, 0x0003);
        val = val.set_response_type(ResponseType::NoResponse);
        assert_eq!(val.response_type(), ResponseType::NoResponse);
        assert_eq!(val.0, 0x0000);
    }

    #[test]
    fn struct_response_functions() {
        let mut val = Response(0x0);

        val = val.set_response(0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF);
        assert_eq!(val.response(), 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF);
        assert_eq!(val.0, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF);
    }

    #[test]
    fn struct_buffer_data_port_functions() {
        let mut val = BufferDataPort(0x0);

        val = val.set_buffer_data(0xFFFF_FFFF);
        assert_eq!(val.buffer_data(), 0xFFFF_FFFF);
        assert_eq!(val.0, 0xFFFF_FFFF);
    }

    #[test]
    fn struct_present_state_functions() {
        let mut val = PresentState(0x0100_0000);
        assert_eq!(val.cmd_line(), 0x1);

        val = PresentState(0x00F0_0000);
        assert_eq!(val.dat_line(), 0xF);

        val = PresentState(0x0008_0000);
        assert!(val.is_write_protect());

        val = PresentState(0x0004_0000);
        assert!(!val.is_card_detect());

        val = PresentState(0x0002_0000);
        assert!(val.is_card_detect_stable());

        val = PresentState(0x0001_0000);
        assert!(val.is_card_inserted());

        val = PresentState(0x0000_0800);
        assert!(!val.is_read_buffer_empty());

        val = PresentState(0x0000_0400);
        assert!(!val.is_write_buffer_empty());

        val = PresentState(0x0000_0200);
        assert!(!val.is_read_transfer_finished());

        val = PresentState(0x0000_0100);
        assert!(!val.is_write_transfer_finished());

        val = PresentState(0x0000_0008);
        assert!(val.if_re_tuning_occurs());

        val = PresentState(0x0000_0004);
        assert!(val.is_dat_line_active());

        val = PresentState(0x0000_0002);
        assert!(val.is_dat_line_busy());

        val = PresentState(0x0000_0001);
        assert!(val.is_cmd_line_busy());
    }

    #[test]
    fn struct_host_control_1_functions() {
        let mut val = HostControl1(0x0);

        val = val.set_card_detect_signal(CardSignal::SDCD);
        assert_eq!(val.card_detect_signal(), CardSignal::SDCD);
        assert_eq!(val.0, 0x00);
        val = val.set_card_detect_signal(CardSignal::TestLevel);
        assert_eq!(val.card_detect_signal(), CardSignal::TestLevel);
        assert_eq!(val.0, 0x80);

        val = HostControl1(0x0);
        val = val.set_card_detect_level(0x1);
        assert!(val.is_card_detected());
        assert_eq!(val.0, 0x40);

        val = HostControl1(0x0);
        val = val.set_bus_width(BusWidthMode::EightBitWidth);
        assert_eq!(val.bus_width(), BusWidthMode::EightBitWidth);
        assert_eq!(val.0, 0x20);
        val = val.set_bus_width(BusWidthMode::SelectByDataTransferWidth);
        assert_eq!(val.0, 0x00);

        val = val.set_dma_mode(DmaMode::ADMA2);
        assert_eq!(val.dma_mode(), DmaMode::ADMA2);
        assert_eq!(val.0, 0x10);
        val = val.set_dma_mode(DmaMode::SDMA);
        assert_eq!(val.dma_mode(), DmaMode::SDMA);
        assert_eq!(val.0, 0x00);

        val = val.set_speed_mode(SpeedMode::HighSpeed);
        assert_eq!(val.speed_mode(), SpeedMode::HighSpeed);
        assert_eq!(val.0, 0x04);
        val = val.set_speed_mode(SpeedMode::NormalSpeed);
        assert_eq!(val.speed_mode(), SpeedMode::NormalSpeed);
        assert_eq!(val.0, 0x00);

        val = val.set_transfer_width(TransferWidth::FourBitMode);
        assert_eq!(val.transfer_width(), TransferWidth::FourBitMode);
        assert_eq!(val.0, 0x02);
        val = val.set_transfer_width(TransferWidth::OneBitMode);
        assert_eq!(val.transfer_width(), TransferWidth::OneBitMode);
        assert_eq!(val.0, 0x00);

        val = val.set_led_state(LedState::On);
        assert_eq!(val.led_state(), LedState::On);
        assert_eq!(val.0, 0x01);
        val = val.set_led_state(LedState::Off);
        assert_eq!(val.led_state(), LedState::Off);
        assert_eq!(val.0, 0x00);
    }

    #[test]
    fn struct_power_control_functions() {
        let mut val = PowerControl(0x0);

        val = val.set_bus_voltage(BusVoltage::V1_8);
        assert_eq!(val.bus_voltage(), BusVoltage::V1_8);
        assert_eq!(val.0, 0x0A);
        val = val.set_bus_voltage(BusVoltage::V3_0);
        assert_eq!(val.bus_voltage(), BusVoltage::V3_0);
        assert_eq!(val.0, 0x0C);
        val = val.set_bus_voltage(BusVoltage::V3_3);
        assert_eq!(val.bus_voltage(), BusVoltage::V3_3);
        assert_eq!(val.0, 0xE);

        val = PowerControl(0x0);
        val = val.enable_bus_power();
        assert!(val.is_bus_power_enable());
        assert_eq!(val.0, 0x01);
        val = val.disable_bus_power();
        assert!(!val.is_bus_power_enable());
        assert_eq!(val.0, 0x00);
    }

    #[test]
    fn struct_block_gap_functions() {
        let mut val = BlockGap(0x0);

        val = val.enable_block_gap_int();
        assert!(val.is_block_gap_int_enabled());
        assert_eq!(val.0, 0x08);
        val = val.disable_block_gap_int();
        assert!(!val.is_block_gap_int_enabled());
        assert_eq!(val.0, 0x00);

        val = val.enable_read_wait();
        assert!(val.is_read_wait_enabled());
        assert_eq!(val.0, 0x04);
        val = val.disable_read_wait();
        assert!(!val.is_read_wait_enabled());
        assert_eq!(val.0, 0x00);

        val = val.restart_transaction();
        assert_eq!(val.0, 0x02);

        val = BlockGap(0x0);
        val = val.set_stop_at_block_gap_req(0x1);
        assert_eq!(val.stop_at_block_gap_req(), 0x1);
        assert_eq!(val.0, 0x01);
    }

    #[test]
    fn struct_wakeup_control_functions() {
        let mut val = WakeupControl(0x0);

        val = val.enable_card_removal();
        assert!(val.is_card_removal_enable());
        assert_eq!(val.0, 0x04);
        val = val.disable_card_removal();
        assert!(!val.is_card_removal_enable());
        assert_eq!(val.0, 0x00);

        val = val.enable_card_insertion();
        assert!(val.is_card_insertion_enable());
        assert_eq!(val.0, 0x02);
        val = val.disable_card_insertion();
        assert!(!val.is_card_insertion_enable());
        assert_eq!(val.0, 0x00);

        val = val.enable_card_int();
        assert!(val.is_card_int_enable());
        assert_eq!(val.0, 0x01);
        val = val.disable_card_int();
        assert!(!val.is_card_int_enable());
        assert_eq!(val.0, 0x00);
    }

    #[test]
    fn struct_clock_control_functions() {
        let mut val = ClockControl(0x0);

        val = val.set_sd_clk_freq(0xFF);
        assert_eq!(val.sd_clk_freq(), 0xFF);
        assert_eq!(val.0, 0xFF00);

        val = ClockControl(0x0);
        val = val.set_sd_clk_freq_upper(0x3);
        assert_eq!(val.sd_clk_freq_upper(), 0x3);
        assert_eq!(val.0, 0x00C0);

        val = ClockControl(0x0);
        val = val.set_clk_gen_mode(ClkGenMode::ProgrammableClk);
        assert_eq!(val.clk_gen_mode(), ClkGenMode::ProgrammableClk);
        assert_eq!(val.0, 0x0020);
        val = val.set_clk_gen_mode(ClkGenMode::DividedClk);
        assert_eq!(val.clk_gen_mode(), ClkGenMode::DividedClk);
        assert_eq!(val.0, 0x0000);

        val = val.enable_sd_clk();
        assert!(val.is_sd_clk_enabled());
        assert_eq!(val.0, 0x0004);
        val = val.disable_sd_clk();
        assert!(!val.is_sd_clk_enabled());
        assert_eq!(val.0, 0x0000);

        val = ClockControl(0x0002);
        assert!(val.is_internal_clk_stable());

        val = ClockControl(0x0);
        val = val.enable_internal_clk();
        assert!(val.is_internal_clk_enable());
        assert_eq!(val.0, 0x01);
        val = val.disable_internal_clk();
        assert!(!val.is_internal_clk_enable());
        assert_eq!(val.0, 0x00);
    }

    #[test]
    fn struct_timeout_control_functions() {
        let mut val = TimeoutControl(0x0);

        val = val.set_timeout_val(0xF);
        assert_eq!(val.timeout_val(), 0xF);
        assert_eq!(val.0, 0x0F);
    }

    #[test]
    fn struct_software_reset_functions() {
        let mut val = SoftwareReset(0x0);

        val = val.reset_dat();
        assert_eq!(val.0, 0x04);
        val = SoftwareReset(0x0);
        assert!(val.is_reset_dat_finished());

        val = val.reset_cmd();
        assert_eq!(val.0, 0x02);
        val = SoftwareReset(0x0);
        assert!(val.is_reset_cmd_finished());

        val = val.reset_all();
        assert_eq!(val.0, 0x01);
        val = SoftwareReset(0x0);
        assert!(val.is_reset_all_finished());
    }

    #[test]
    fn struct_normal_interrupt_status_functions() {
        let mut val = NormalInterruptStatus(0xF000);
        assert!(val.if_err_int_occurs());

        val = NormalInterruptStatus(0x1000);
        assert!(val.if_re_tuning_occurs());

        val = NormalInterruptStatus(0x0800);
        assert!(val.is_int_c_enabled());

        val = NormalInterruptStatus(0x0400);
        assert!(val.is_int_b_enabled());

        val = NormalInterruptStatus(0x0200);
        assert!(val.is_int_a_enabled());

        val = NormalInterruptStatus(0x0100);
        assert!(val.if_card_int_occurs());

        val = NormalInterruptStatus(0x0080);
        assert!(val.is_card_removed());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_card_removed();
        assert_eq!(val.0, 0x0080);

        val = NormalInterruptStatus(0x0040);
        assert!(val.is_card_inserted());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_card_inserted();
        assert_eq!(val.0, 0x0040);

        val = NormalInterruptStatus(0x0020);
        assert!(val.is_buffer_read_ready());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_buffer_read_ready();
        assert_eq!(val.0, 0x0020);

        val = NormalInterruptStatus(0x0010);
        assert!(val.is_buffer_write_ready());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_buffer_write_ready();
        assert_eq!(val.0, 0x0010);

        val = NormalInterruptStatus(0x0008);
        assert!(val.if_dma_int_occurs());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_dma_int();
        assert_eq!(val.0, 0x0008);

        val = NormalInterruptStatus(0x0004);
        assert!(val.if_block_gap_occurs());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_block_gap();
        assert_eq!(val.0, 0x0004);

        val = NormalInterruptStatus(0x0002);
        assert!(val.is_transfer_completed());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_transfer_completed();
        assert_eq!(val.0, 0x0002);

        val = NormalInterruptStatus(0x0001);
        assert!(val.is_cmd_completed());
        val = NormalInterruptStatus(0x0000);
        val = val.clear_cmd_completed();
        assert_eq!(val.0, 0x0001);
    }

    #[test]
    fn struct_error_interrupt_status_functions() {
        let mut val = ErrorInterruptStatus(0xF000);
        assert_eq!(val.vendor_specific_err(), 0x0F);
        val = ErrorInterruptStatus(0x0);
        val = val.clear_vendor_specific_err();
        assert_eq!(val.0, 0xF000);

        val = ErrorInterruptStatus(0x0400);
        assert!(val.if_tuning_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_tuning_err();
        assert_eq!(val.0, 0x0400);

        val = ErrorInterruptStatus(0x0200);
        assert!(val.if_adma_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_adma_err();
        assert_eq!(val.0, 0x0200);

        val = ErrorInterruptStatus(0x0100);
        assert!(val.if_auto_cmd_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_auto_cmd_err();
        assert_eq!(val.0, 0x0100);

        val = ErrorInterruptStatus(0x0080);
        assert!(val.if_current_limit_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_current_limit_err();
        assert_eq!(val.0, 0x0080);

        val = ErrorInterruptStatus(0x0040);
        assert!(val.if_data_end_bit_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_data_end_bit_err();
        assert_eq!(val.0, 0x0040);

        val = ErrorInterruptStatus(0x0020);
        assert!(val.if_data_crc_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_data_crc_err();
        assert_eq!(val.0, 0x0020);

        val = ErrorInterruptStatus(0x0010);
        assert!(val.if_data_timeout_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_data_timeout_err();
        assert_eq!(val.0, 0x0010);

        val = ErrorInterruptStatus(0x0008);
        assert!(val.if_cmd_index_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_cmd_index_err();
        assert_eq!(val.0, 0x0008);

        val = ErrorInterruptStatus(0x0004);
        assert!(val.if_cmd_end_bit_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_cmd_end_bit_err();
        assert_eq!(val.0, 0x0004);

        val = ErrorInterruptStatus(0x0002);
        assert!(val.if_cmd_crc_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_cmd_crc_err();
        assert_eq!(val.0, 0x0002);

        val = ErrorInterruptStatus(0x0001);
        assert!(val.if_cmd_timeout_err_occurs());
        val = ErrorInterruptStatus(0x0);
        val = val.clear_cmd_timeout_err();
        assert_eq!(val.0, 0x0001);
    }

    #[test]
    fn struct_normal_interrupt_status_enable_functions() {
        let mut val = NormalInterruptStatusEnable(0x0);

        assert!(val.is_fixed_to_zero());

        val = val.enable_re_tuning();
        assert!(val.is_retuning_enabled());
        assert_eq!(val.0, 0x1000);
        val = val.disable_re_tuning();
        assert!(!val.is_retuning_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_int_c();
        assert!(val.is_int_c_enabled());
        assert_eq!(val.0, 0x0800);
        val = val.disable_int_c();
        assert!(!val.is_int_c_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_int_b();
        assert!(val.is_int_b_enabled());
        assert_eq!(val.0, 0x0400);
        val = val.disable_int_b();
        assert!(!val.is_int_b_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_int_a();
        assert!(val.is_int_a_enabled());
        assert_eq!(val.0, 0x0200);
        val = val.disable_int_a();
        assert!(!val.is_int_a_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_card_int();
        assert!(val.is_card_int_enabled());
        assert_eq!(val.0, 0x0100);
        val = val.disable_card_int();
        assert!(!val.is_card_int_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_card_removal();
        assert!(val.is_card_removal_enabled());
        assert_eq!(val.0, 0x0080);
        val = val.disable_card_removal();
        assert!(!val.is_card_removal_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_card_insertion();
        assert!(val.is_card_insertion_enabled());
        assert_eq!(val.0, 0x0040);
        val = val.disable_card_insertion();
        assert!(!val.is_card_insertion_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_buffer_read_ready();
        assert!(val.is_buffer_read_ready_enabled());
        assert_eq!(val.0, 0x0020);
        val = val.disable_buffer_read_ready();
        assert!(!val.is_buffer_read_ready_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_buffer_write_ready();
        assert!(val.is_buffer_write_ready_enabled());
        assert_eq!(val.0, 0x0010);
        val = val.disable_buffer_write_ready();
        assert!(!val.is_buffer_write_ready_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_dma_int();
        assert!(val.is_dma_int_enabled());
        assert_eq!(val.0, 0x0008);
        val = val.disable_dma_int();
        assert!(!val.is_dma_int_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_block_gap();
        assert!(val.is_block_gap_enabled());
        assert_eq!(val.0, 0x0004);
        val = val.disable_block_gap();
        assert!(!val.is_block_gap_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_transfer_complete();
        assert!(val.is_transfer_complete_enabled());
        assert_eq!(val.0, 0x0002);
        val = val.disable_transfer_complete();
        assert!(!val.is_transfer_complete_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_complete();
        assert!(val.is_cmd_complete_enabled());
        assert_eq!(val.0, 0x0001);
        val = val.disable_cmd_complete();
        assert!(!val.is_cmd_complete_enabled());
        assert_eq!(val.0, 0x0000);
    }

    #[test]
    fn struct_error_interrupt_status_enable_functions() {
        let mut val = ErrorInterruptStatusEnable(0x0);

        val = val.enable_vendor_specific_err();
        assert!(val.is_vendor_specific_err_enabled());
        assert_eq!(val.0, 0xF000);
        val = val.disable_vendor_specific_err();
        assert!(!val.is_vendor_specific_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_tuning_err();
        assert!(val.is_tuning_err_enabled());
        assert_eq!(val.0, 0x0400);
        val = val.disable_tuning_err();
        assert!(!val.is_tuning_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_adma_err();
        assert!(val.is_adma_err_enabled());
        assert_eq!(val.0, 0x0200);
        val = val.disable_adma_err();
        assert!(!val.is_adma_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_auto_cmd_err();
        assert!(val.is_auto_cmd_err_enabled());
        assert_eq!(val.0, 0x0100);
        val = val.disable_auto_cmd_err();
        assert!(!val.is_auto_cmd_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_current_limit_err();
        assert!(val.is_current_limit_err_enabled());
        assert_eq!(val.0, 0x0080);
        val = val.disable_current_limit_err();
        assert!(!val.is_current_limit_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_data_end_bit_err();
        assert!(val.is_data_end_bit_err_enabled());
        assert_eq!(val.0, 0x0040);
        val = val.disable_data_end_bit_err();
        assert!(!val.is_data_end_bit_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_data_crc_err();
        assert!(val.is_data_crc_err_enabled());
        assert_eq!(val.0, 0x0020);
        val = val.disable_data_crc_err();
        assert!(!val.is_data_crc_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_data_timeout_err();
        assert!(val.is_data_timeout_err_enabled());
        assert_eq!(val.0, 0x0010);
        val = val.disable_data_timeout_err();
        assert!(!val.is_data_timeout_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_index_err();
        assert!(val.is_cmd_index_err_enabled());
        assert_eq!(val.0, 0x0008);
        val = val.disable_cmd_index_err();
        assert!(!val.is_cmd_index_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_end_bit_err();
        assert!(val.is_cmd_end_bit_err_enabled());
        assert_eq!(val.0, 0x0004);
        val = val.disable_cmd_end_bit_err();
        assert!(!val.is_cmd_end_bit_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_crc_err();
        assert!(val.is_cmd_crc_err_enabled());
        assert_eq!(val.0, 0x0002);
        val = val.disable_cmd_crc_err();
        assert!(!val.is_cmd_crc_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_timeout_err();
        assert!(val.is_cmd_timeout_err_enabled());
        assert_eq!(val.0, 0x0001);
        val = val.disable_cmd_timeout_err();
        assert!(!val.is_cmd_timeout_err_enabled());
        assert_eq!(val.0, 0x0000);
    }

    #[test]
    fn struct_normal_interrupt_signal_enable_functions() {
        let mut val = NormalInterruptSignalEnable(0x0);

        assert!(val.is_fixed_to_zero());

        val = val.enable_re_tuning();
        assert!(val.is_re_tuning_enabled());
        assert_eq!(val.0, 0x1000);
        val = val.disable_re_tuning();
        assert!(!val.is_re_tuning_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_int_c();
        assert!(val.is_int_c_enabled());
        assert_eq!(val.0, 0x0800);
        val = val.disable_int_c();
        assert!(!val.is_int_c_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_int_b();
        assert!(val.is_int_b_enabled());
        assert_eq!(val.0, 0x0400);
        val = val.disable_int_b();
        assert!(!val.is_int_b_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_int_a();
        assert!(val.is_int_a_enabled());
        assert_eq!(val.0, 0x0200);
        val = val.disable_int_a();
        assert!(!val.is_int_a_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_card_int();
        assert!(val.is_card_int_enabled());
        assert_eq!(val.0, 0x0100);
        val = val.disable_card_int();
        assert!(!val.is_card_int_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_card_removal();
        assert!(val.is_card_removal_enabled());
        assert_eq!(val.0, 0x0080);
        val = val.disable_card_removal();
        assert!(!val.is_card_removal_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_card_insertion();
        assert!(val.is_card_insertion_enabled());
        assert_eq!(val.0, 0x0040);
        val = val.disable_card_insertion();
        assert!(!val.is_card_insertion_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_buffer_read_ready();
        assert!(val.is_buffer_read_ready_enabled());
        assert_eq!(val.0, 0x0020);
        val = val.disable_buffer_read_ready();
        assert!(!val.is_buffer_read_ready_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_buffer_write_ready();
        assert!(val.is_buffer_write_ready_enabled());
        assert_eq!(val.0, 0x0010);
        val = val.disable_buffer_write_ready();
        assert!(!val.is_buffer_write_ready_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_dma_int();
        assert!(val.is_dma_int_enabled());
        assert_eq!(val.0, 0x0008);
        val = val.disable_dma_int();
        assert!(!val.is_dma_int_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_block_gap();
        assert!(val.is_block_gap_enabled());
        assert_eq!(val.0, 0x0004);
        val = val.disable_block_gap();
        assert!(!val.is_block_gap_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_transfer_complete();
        assert!(val.is_transfer_complete_enabled());
        assert_eq!(val.0, 0x0002);
        val = val.disable_transfer_complete();
        assert!(!val.is_transfer_complete_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_complete();
        assert!(val.is_cmd_complete_enabled());
        assert_eq!(val.0, 0x0001);
        val = val.disable_cmd_complete();
        assert!(!val.is_cmd_complete_enabled());
        assert_eq!(val.0, 0x0000);
    }

    #[test]
    fn struct_error_interrupt_signal_enable_functions() {
        let mut val = ErrorInterruptSignalEnable(0xF000);
        assert!(val.is_vendor_specific_err_enabled());

        val = ErrorInterruptSignalEnable(0x0);
        val = val.enable_tuning_err();
        assert!(val.is_tuning_err_enabled());
        assert_eq!(val.0, 0x0400);
        val = val.disable_tuning_err();
        assert!(!val.is_tuning_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_adma_err();
        assert!(val.is_adma_err_enabled());
        assert_eq!(val.0, 0x0200);
        val = val.disable_adma_err();
        assert!(!val.is_adma_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_auto_cmd_err();
        assert!(val.is_auto_cmd_err_enabled());
        assert_eq!(val.0, 0x0100);
        val = val.disable_auto_cmd_err();
        assert!(!val.is_auto_cmd_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_current_limit_err();
        assert!(val.is_current_limit_err_enabled());
        assert_eq!(val.0, 0x0080);
        val = val.disable_current_limit_err();
        assert!(!val.is_current_limit_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_data_end_bit_err();
        assert!(val.is_data_end_bit_err_enabled());
        assert_eq!(val.0, 0x0040);
        val = val.disable_data_end_bit_err();
        assert!(!val.is_data_end_bit_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_data_crc_err();
        assert!(val.is_data_crc_err_enabled());
        assert_eq!(val.0, 0x0020);
        val = val.disable_data_crc_err();
        assert!(!val.is_data_crc_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_data_timeout_err();
        assert!(val.is_data_timeout_err_enabled());
        assert_eq!(val.0, 0x0010);
        val = val.disable_data_timeout_err();
        assert!(!val.is_data_timeout_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_index_err();
        assert!(val.is_cmd_index_err_enabled());
        assert_eq!(val.0, 0x0008);
        val = val.disable_cmd_index_err();
        assert!(!val.is_cmd_index_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_end_bit_err();
        assert!(val.is_cmd_end_bit_err_enabled());
        assert_eq!(val.0, 0x0004);
        val = val.disable_cmd_end_bit_err();
        assert!(!val.is_data_end_bit_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_crc_err();
        assert!(val.is_cmd_crc_err_enabled());
        assert_eq!(val.0, 0x0002);
        val = val.disable_cmd_crc_err();
        assert!(!val.is_cmd_crc_err_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_cmd_timeout_err();
        assert!(val.is_cmd_timeout_err_enabled());
        assert_eq!(val.0, 0x0001);
        val = val.disable_cmd_timeout_err();
        assert!(!val.is_cmd_timeout_err_enabled());
        assert_eq!(val.0, 0x0000);
    }

    #[test]
    fn struct_auto_cmd_error_status_functions() {
        let mut val = AutoCmdErrorStatus(0x0080);
        assert!(val.is_cmd_not_issued());

        val = AutoCmdErrorStatus(0x0010);
        assert!(val.if_auto_cmd_index_err_occurs());

        val = AutoCmdErrorStatus(0x0008);
        assert!(val.if_auto_cmd_end_bit_err_occurs());

        val = AutoCmdErrorStatus(0x0004);
        assert!(val.if_auto_cmd_crc_err_occurs());

        val = AutoCmdErrorStatus(0x0002);
        assert!(val.if_auto_cmd_timeout_err_occurs());

        val = AutoCmdErrorStatus(0x0001);
        assert!(val.is_auto_cmd12_not_executed());
    }

    #[test]
    fn struct_host_control_2_functions() {
        let mut val = HostControl2(0x0);

        val = val.enable_preset_val();
        assert!(val.is_preset_val_enabled());
        assert_eq!(val.0, 0x8000);
        val = val.disable_preset_val();
        assert!(!val.is_preset_val_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.enable_async_int();
        assert!(val.is_async_int_enabled());
        assert_eq!(val.0, 0x4000);
        val = val.disable_async_int();
        assert!(!val.is_async_int_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.set_sample_clk_select(0x1);
        assert_eq!(val.sample_clk_select(), 0x1);
        assert_eq!(val.0, 0x0080);

        val = HostControl2(0x0);
        val = val.start_tuning();
        assert_eq!(val.0, 0x0040);
        val = HostControl2(0x0);
        assert!(val.is_tuning_finished());

        val = val.set_driver_strength_select(0x3);
        assert_eq!(val.driver_strength_select(), 0x3);
        assert_eq!(val.0, 0x0030);

        val = HostControl2(0x0);
        val = val.change_3_3v_to_1_8v();
        assert!(val.is_3_3v_to_1_8v_finished());
        assert_eq!(val.0, 0x0008);
        val = val.change_1_8v_to_3_3v();
        assert!(val.is_1_8v_to_3_3v_finished());
        assert_eq!(val.0, 0x0000);

        val = val.set_uhs_mode(0x7);
        assert_eq!(val.uhs_mode(), 0x7);
        assert_eq!(val.0, 0x0007);
    }

    #[test]
    fn struct_capabilities_functions() {
        let mut val = Capabilities(0x4000_0000_0000_0000);
        assert_eq!(val.slot_type(), SlotType::EmbeddedSlotforOneDevice);
        assert_eq!(val.0, 0x4000_0000_0000_0000);
        val = Capabilities(0x8000_0000_0000_0000);
        assert_eq!(val.slot_type(), SlotType::SharedBusSlot);
        val = Capabilities(0x0000_0000_0000_0000);
        assert_eq!(val.slot_type(), SlotType::RemovableCardSlot);

        val = Capabilities(0x1000_0000_0000_0000);
        assert!(val.is_64_bit_bus_supported());
        val = Capabilities(0x0000_0000_0000_0000);
        assert!(!val.is_64_bit_bus_supported());

        val = Capabilities(0x0400_0000_0000_0000);
        assert!(val.is_1_8v_supported());

        val = Capabilities(0x0200_0000_0000_0000);
        assert!(val.is_3_0v_supported());

        val = Capabilities(0x0100_0000_0000_0000);
        assert!(val.is_3_3v_supported());

        val = Capabilities(0x0080_0000_0000_0000);
        assert!(val.is_suspend_resume_supported());

        val = Capabilities(0x0040_0000_0000_0000);
        assert!(val.is_sdma_supported());

        val = Capabilities(0x0020_0000_0000_0000);
        assert!(val.is_high_speed_supported());

        val = Capabilities(0x0008_0000_0000_0000);
        assert!(val.is_adma2_supported());

        val = Capabilities(0x0004_0000_0000_0000);
        assert!(val.is_8_bit_bus_supported());

        val = Capabilities(0x0003_0000_0000_0000);
        assert_eq!(val.max_block_len(), 0x3);

        val = Capabilities(0x0000_FF00_0000_0000);
        assert_eq!(val.base_clk(), 0xFF);

        val = Capabilities(0x0000_0080_0000_0000);
        assert_eq!(val.timeout_clk_unit(), 0x1);

        val = Capabilities(0x0000_0000_00FF_0000);
        assert_eq!(val.clk_multiplier(), 0xFF);

        val = Capabilities(0x0000_0000_0000_C000);
        assert_eq!(val.re_tuning_modes(), 0x3);

        val = Capabilities(0x0000_0000_0000_2000);
        assert!(val.is_tuning_for_sdr50_required());

        val = Capabilities(0x0000_0000_0000_0F00);
        assert_eq!(val.tim_cnt_for_re_tuning(), 0xF);

        val = Capabilities(0x0000_0000_0000_0040);
        assert!(val.is_driver_type_d_supported());

        val = Capabilities(0x0000_0000_0000_0020);
        assert!(val.is_driver_type_c_supported());

        val = Capabilities(0x0000_0000_0000_0010);
        assert!(val.is_driver_type_a_supported());

        val = Capabilities(0x0000_0000_0000_0004);
        assert!(val.is_ddr50_supported());

        val = Capabilities(0x0000_0000_0000_0002);
        assert!(val.is_sdr104_supprted());

        val = Capabilities(0x0000_0000_0000_0001);
        assert!(val.is_sdr50_supported());
    }

    #[test]
    fn struct_max_current_capabilities_functions() {
        let mut val = MaxCurrentCapabilities(0x0000_0000_00FF_0000);
        assert_eq!(val.max_current_1_8v(), 0xFF);

        val = MaxCurrentCapabilities(0x0000_0000_0000_FF00);
        assert_eq!(val.max_current_3_0v(), 0xFF);

        val = MaxCurrentCapabilities(0x0000_0000_0000_00FF);
        assert_eq!(val.max_current_3_3v(), 0xFF);
    }

    #[test]
    fn struct_force_event_auto_cmd_error_status_functions() {
        let mut val = ForceEventAutoCmdErrorStatus(0x0);
        val = val.set_cmd_not_issued(0x1);
        assert_eq!(val.0, 0x0080);

        val = ForceEventAutoCmdErrorStatus(0x0);
        val = val.set_auto_cmd_index(0x1);
        assert_eq!(val.0, 0x0010);

        val = ForceEventAutoCmdErrorStatus(0x0);
        val = val.set_auto_cmd_end_bit(0x1);
        assert_eq!(val.0, 0x0008);

        val = ForceEventAutoCmdErrorStatus(0x0);
        val = val.set_auto_cmd_crc(0x1);
        assert_eq!(val.0, 0x0004);

        val = ForceEventAutoCmdErrorStatus(0x0);
        val = val.set_auto_cmd_timeout(0x1);
        assert_eq!(val.0, 0x0002);

        val = ForceEventAutoCmdErrorStatus(0x0);
        val = val.set_auto_cmd12_not_executed(0x1);
        assert_eq!(val.0, 0x0001);
    }

    #[test]
    fn struct_force_event_error_interrupt_status_functions() {
        let mut val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_vendor_specific_err(0xF);
        assert_eq!(val.0, 0xF000);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_adma_err(0x1);
        assert_eq!(val.0, 0x0200);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_auto_cmd_err(0x1);
        assert_eq!(val.0, 0x0100);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_current_limit_err(0x1);
        assert_eq!(val.0, 0x0080);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_data_end_bit_err(0x1);
        assert_eq!(val.0, 0x0040);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_data_crc_err(0x1);
        assert_eq!(val.0, 0x0020);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_data_timeout_err(0x1);
        assert_eq!(val.0, 0x0010);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_cmd_index_err(0x1);
        assert_eq!(val.0, 0x0008);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_cmd_end_bit_err(0x1);
        assert_eq!(val.0, 0x0004);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_cmd_crc_err(0x1);
        assert_eq!(val.0, 0x0002);

        val = ForceEventErrorInterruptStatus(0x0);
        val = val.set_cmd_timeout_err(0x1);
        assert_eq!(val.0, 0x0001);
    }

    #[test]
    fn struct_adma2_error_status_functions() {
        let mut val = Adma2ErrorStatus(0x0000_0000_0000_0004);
        assert!(val.if_adma_len_mismatch_err_occurs());

        val = Adma2ErrorStatus(0x0000_0000_0000_0001);
        assert_eq!(val.adma_err_state(), 0x1);
    }

    #[test]
    fn struct_adma2_system_address_functions() {
        let mut val = Adma2SystemAddress(0x0);
        val = val.set_adma_sys_addr(0xFFFF_FFFF_FFFF_FFFF);
        assert_eq!(val.adma_sys_addr(), 0xFFFF_FFFF_FFFF_FFFF);
        assert_eq!(val.0, 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn struct_preset_value_functions() {
        let mut val = PresetValue(0xC0000000_00000000_00000000_00000000);
        assert_eq!(val.ddr50_drv_strength_val(), 0x3);
        val = PresetValue(0x04000000_00000000_00000000_00000000);
        assert_eq!(val.ddr50_clkgen_sel_val(), 0x1);
        val = PresetValue(0x03FF0000_00000000_00000000_00000000);
        assert_eq!(val.ddr50_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x0000C000_00000000_00000000_00000000);
        assert_eq!(val.sdr104_drv_strength_val(), 0x3);
        val = PresetValue(0x00000400_00000000_00000000_00000000);
        assert_eq!(val.sdr104_clkgen_sel_val(), 0x1);
        val = PresetValue(0x000003FF_00000000_00000000_00000000);
        assert_eq!(val.sdr104_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x00000000_C0000000_00000000_00000000);
        assert_eq!(val.sdr50_drv_strength_val(), 0x3);
        val = PresetValue(0x00000000_04000000_00000000_00000000);
        assert_eq!(val.sdr50_clkgen_sel_val(), 0x1);
        val = PresetValue(0x00000000_03FF0000_00000000_00000000);
        assert_eq!(val.sdr50_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x00000000_0000C000_00000000_00000000);
        assert_eq!(val.sdr25_drv_strength_val(), 0x3);
        val = PresetValue(0x00000000_00000400_00000000_00000000);
        assert_eq!(val.sdr25_clkgen_sel_val(), 0x1);
        val = PresetValue(0x00000000_000003FF_00000000_00000000);
        assert_eq!(val.sdr25_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x00000000_00000000_C0000000_00000000);
        assert_eq!(val.sdr12_drv_strength_val(), 0x3);
        val = PresetValue(0x00000000_00000000_04000000_00000000);
        assert_eq!(val.sdr12_clkgen_sel_val(), 0x1);
        val = PresetValue(0x00000000_00000000_03FF0000_00000000);
        assert_eq!(val.sdr12_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x00000000_00000000_0000C000_00000000);
        assert_eq!(val.hs_drv_strength_val(), 0x3);
        val = PresetValue(0x00000000_00000000_00000400_00000000);
        assert_eq!(val.hs_clkgen_sel_val(), 0x1);
        val = PresetValue(0x00000000_00000000_000003FF_00000000);
        assert_eq!(val.hs_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x00000000_00000000_00000000_C0000000);
        assert_eq!(val.default_drv_strength_val(), 0x3);
        val = PresetValue(0x00000000_00000000_00000000_04000000);
        assert_eq!(val.default_clkgen_sel_val(), 0x1);
        val = PresetValue(0x00000000_00000000_00000000_03FF0000);
        assert_eq!(val.default_sdclk_freq_clk_val(), 0x3FF);

        val = PresetValue(0x00000000_00000000_00000000_0000C000);
        assert_eq!(val.init_drv_strength_val(), 0x3);
        val = PresetValue(0x00000000_00000000_00000000_00000400);
        assert_eq!(val.init_clkgen_sel_val(), 0x1);
        val = PresetValue(0x00000000_00000000_00000000_000003FF);
        assert_eq!(val.init_sdclk_freq_clk_val(), 0x3FF);
    }

    #[test]
    fn struct_adma3_integrated_descriptor_address_functions() {
        // TODO
    }

    #[test]
    fn struct_shared_bus_control_functions() {
        let mut val = SharedBusControl(0x0);

        val = val.set_back_end_pwr_ctrl(0x7F);
        assert_eq!(val.back_end_pwr_ctrl(), 0x7F);
        assert_eq!(val.0, 0x7F00_0000);

        val = SharedBusControl(0x0);
        val = val.set_int_pin_sel(0x7);
        assert_eq!(val.int_pin_sel(), 0x7);
        assert_eq!(val.0, 0x0070_0000);

        val = SharedBusControl(0x0);
        val = val.set_clk_pin_sel(0x7);
        assert_eq!(val.clk_pin_sel(), 0x7);
        assert_eq!(val.0, 0x0007_0000);

        val = SharedBusControl(0x0);
        val = val.set_bus_width_preset(0x7F);
        assert_eq!(val.bus_width_preset(), 0x7F);
        assert_eq!(val.0, 0x0000_7F00);

        val = SharedBusControl(0x0);
        val = val.set_int_input_pin_num(0x3);
        assert_eq!(val.int_input_pin_num(), 0x3);
        assert_eq!(val.0, 0x0000_0030);

        val = SharedBusControl(0x0000_0007);
        assert_eq!(val.clk_pin_num(), 0x7);
    }

    #[test]
    fn struct_slot_interrupt_status_functions() {
        let val = SlotInterruptStatus(0x00FF);
        assert_eq!(val.int_signal(), 0xFF);
    }

    #[test]
    fn struct_host_controller_version_functions() {
        let mut val = HostControllerVersion(0xFF00);
        assert_eq!(val.vendor_version(), 0xFF);

        val = HostControllerVersion(0x0002);
        assert_eq!(
            val.specific_version(),
            SpecificVersion::SDHostSpecificVersion3
        );

        val = HostControllerVersion(0x0001);
        assert_eq!(
            val.specific_version(),
            SpecificVersion::SDHostSpecificVersion2
        );

        val = HostControllerVersion(0x0000);
        assert_eq!(
            val.specific_version(),
            SpecificVersion::SDHostSpecificVersion1
        );
    }

    #[test]
    fn struct_sd_extra_parameters_functions() {
        let mut val = SDExtraParameters(0x0);
        val = val.set_gen_clk_cnt(0xF);
        assert_eq!(val.gen_clk_cnt(), 0xF);
        assert_eq!(val.0, 0x0F00_0000);

        val = SDExtraParameters(0x0);
        val = val.set_gen_clk();
        assert!(val.is_gen_clk_on());
        assert_eq!(val.0, 0x0000_0040);
        val = val.unset_gen_clk();
        assert!(!val.is_gen_clk_on());
        assert_eq!(val.0, 0x0000_0000);

        val = SDExtraParameters(0x0);
        val = val.set_squ_full();
        assert!(val.is_squ_full());
        assert_eq!(val.0, 0x0000_0020);
        val = val.unset_squ_full();
        assert!(!val.is_squ_full());
        assert_eq!(val.0, 0x0000_0000);

        val = SDExtraParameters(0x0);
        val = val.set_squ_empty();
        assert!(val.is_squ_empty());
        assert_eq!(val.0, 0x0000_0010);
        val = val.unset_squ_empty();
        assert!(!val.is_squ_empty());
        assert_eq!(val.0, 0x0000_0000);

        val = SDExtraParameters(0x0);
        val = val.set_boot_ack();
        assert!(val.is_boot_finished());
        assert_eq!(val.0, 0x0000_0008);
        val = val.unset_boot_ack();
        assert!(!val.is_boot_finished());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_fifo_parameters_functions() {
        // TODO
    }

    #[test]
    fn struct_spi_mode_functions() {
        let mut val = SpiMode(0x0);

        val = val.set_spi_err_token(0x1F);
        assert_eq!(val.spi_err_token(), 0x1F);
        assert_eq!(val.0, 0x0000_1F00);

        val = SpiMode(0x0);
        val = val.enable_spi();
        assert!(val.is_spi_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_spi();
        assert!(!val.is_spi_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_clock_and_burst_size_setup_functions() {
        // TODO
        let mut val = ClockAndBurstSizeSetup(0x0);
        val = val.set_fifo_threshold(FifoThreshold::Bytes256);
        assert_eq!(val.fifo_threshold(), FifoThreshold::Bytes256);
        assert_eq!(val.0, 0x0000_000C);

        val = val.set_fifo_threshold(FifoThreshold::Bytes192);
        assert_eq!(val.fifo_threshold(), FifoThreshold::Bytes192);
        assert_eq!(val.0, 0x0000_0008);

        val = val.set_fifo_threshold(FifoThreshold::Bytes128);
        assert_eq!(val.fifo_threshold(), FifoThreshold::Bytes128);
        assert_eq!(val.0, 0x0000_0004);

        val = val.set_fifo_threshold(FifoThreshold::Bytes64);
        assert_eq!(val.fifo_threshold(), FifoThreshold::Bytes64);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_burst_size(BurstSize::Bytes256);
        assert_eq!(val.burst_size(), BurstSize::Bytes256);
        assert_eq!(val.0, 0x0000_0003);

        val = val.set_burst_size(BurstSize::Bytes128);
        assert_eq!(val.burst_size(), BurstSize::Bytes128);
        assert_eq!(val.0, 0x0000_0002);

        val = val.set_burst_size(BurstSize::Bytes64);
        assert_eq!(val.burst_size(), BurstSize::Bytes64);
        assert_eq!(val.0, 0x0000_0001);

        val = val.set_burst_size(BurstSize::Bytes32);
        assert_eq!(val.burst_size(), BurstSize::Bytes32);
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_ce_ata_functions() {
        // TODO
    }

    #[test]
    fn struct_pad_io_setup_functions() {
        // TODO
    }

    #[test]
    fn struct_rx_configuration_functions() {
        // TODO
    }

    #[test]
    fn struct_tx_configuration_functions() {
        let mut val = TxConfiguration(0x0);
        val = val.set_tx_int_clk_sel(0x1);
        assert_eq!(val.tx_int_clk_sel(), 0x1);
        assert_eq!(val.0, 0x4000_0000);
        // TODO
    }

    #[test]
    fn struct_tuning_configuration_functions() {
        // TODO
    }
}
