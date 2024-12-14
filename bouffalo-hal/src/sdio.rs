//! Secure Digital Input/Output peripheral.

use volatile_register::RW;

/// Secure Digital Input/Output peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// 32-bit Block Count / (SDMA System Address) Register.
    pub system_address: RW<SystemAddress>,
    /// Configuration register for number of bytes in a data block.
    pub block_size: RW<BlockSize>,
    /// Configuration register for number of data blocks.
    pub block_count: RW<BlockCount>,
    /// Register that contains the SD Command Argument.
    pub argument: RW<Argument>,
    /// Control register for the operation of data transfers.
    pub transfer_mode: RW<TransferMode>,
    /// Command register.
    pub command: RW<Command>,
    /// Register that stores responses from SD cards.
    pub response: RW<Response>,
    /// 32-bit data port register to accesses internal buffer.
    pub buffer_data_port: RW<BufferDataPort>,
    /// 32-bit read only register to get status of the Host Controller.
    pub present_state: RW<PresentState>,
    /// Host Control 1 Register.
    pub host_control_1: RW<HostControl1>,
    /// Power Control Register.
    pub powercontrol: RW<PowerControl>,
    /// Block Gap Control Register.
    pub block_gap: RW<BlockGap>,
    /// Register which is mandatory for the Host Controller.
    pub wakeup_control: RW<WakeupControl>,
    /// Control register for SDCLK in SD Mode and RCLK in UHS-II Mode.
    pub clock_control: RW<ClockControl>,
    /// Timeout Control Register.
    pub timeout_control: RW<TimeoutControl>,
    /// Writting 1 to each bit of this register to generate a reset pulse.
    pub software_reset: RW<SoftwareReset>,
    /// The reads of register are affected by the Normal Interrupt Status Enable.  
    pub normal_interrupt_status: RW<NormalInterruptStatus>,
    /// Register that shows the defined Interrupt Status.
    pub error_interrupt_status: RW<ErrorInterruptStatus>,
    /// Register that sets to 1 enables Interrupt Status.
    pub normal_interrupt_status_enable: RW<NormalInterruptStatusEnable>,
    /// Register that sets to 1 enables Interrupt Status.
    pub error_interrupt_status_enable: RW<ErrorInterruptStatusEnable>,
    /// Register that selects which interrupt status is indicated to the Host System as the interrupt.
    pub normal_interrupt_signal_enable: RW<NormalInterruptSignalEnable>,
    /// Register that selects which interrupt status is notified to the Host System as the interrupt.
    pub error_interrupt_signal_enable: RW<ErrorInterruptSignalEnable>,
    /// Register that indicates CMD12 response error of Auto CMD12 and CMD23 response error of Auto CMD23.
    pub auto_cmd_error_status: RW<AutoCMDErrorStatus>,
    /// Host Control 2 Register.
    pub host_control_2: RW<HostControl2>,
    /// Register that provides the Host Driver with information specific to the Host Controller implementation.
    pub capabilities: RW<Capabilities>,
    /// Registers that indicates maximum current capability fo each voltage.
    pub max_current_capabilities: RW<MaxCurrentCapabilities>,
    /// Register that simplifies test of the Auto CMD Error Status register.
    pub force_event_auto_cmd_error_status: RW<ForceEventAutoCMDErrorStatus>,
    /// Register that simplifies test of the Error Interrupt Status register.
    pub force_event_error_interrupt_status: RW<ForceEventErrorInterruptStatus>,
    /// Register that holds the ADMA state when ADMA Error Interrupt is occurred.
    pub adma_error_status: RW<ADMAErrorStatus>,
    /// Register that contains the physical Descriptor address used for ADMA data transfer.
    pub adma_system_address: RW<ADMASystemAddress>,
    /// Preset Value Registers.
    pub preset_value: RW<PresetValue>,
    _reserved0: [u8; 8],
    /// ADMA3 Intergrated Descriptor Address Register.
    pub adma3_integrated_descriptor_address: RW<ADMA3IntegratedDescriptorAddress>,
    _reserved1: [u8; 96],
    /// Shared Bus Control Register.
    pub shared_bus_control: RW<SharedBusControl>,
    _reserved2: [u8; 24],
    /// Slot Interrupt Status Register.
    pub slot_interrupt_status: RW<SlotInterruptStatus>,
    /// Host Controller Version Register.
    pub host_controller_version: RW<HostControllerVersion>,
    /// SD Extra Parameters Register.
    pub sd_extra_parameters: RW<SDExtraParameters>,
    /// FIFO Parameters Register.
    pub fifo_parameters: RW<FIFOParameters>,
    /// SPI Mode Register.
    pub spi_mode: RW<SPIMode>,
    /// Clock and Burst Size Setup Register.
    pub clock_and_burst_size_setup: RW<ClockAndBurstSizeSetup>,
    /// CE-ATA Register.
    pub ce_ata: RW<CEATA>,
    /// PAD I/O Setup Register.
    pub pad_io_setup: RW<PADIOSetup>,
    /// RX Configuration Register.
    pub rx_configuration: RW<RXConfiguration>,
    /// TX Configuration Register.
    pub tx_configuration: RW<TXConfiguration>,
    /// TUNING CONFIG Register.
    pub tuning_configuration: RW<TUNINGConfiguration>,
}

/// 32-bit Block Count / (SDMA System Address) Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SystemAddress(u32);

impl SystemAddress {
    /// Get SDMA System Address.
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn addr(self) -> u32 {
        self.0
    }
    /// Set Argument2.
    /// Used with the Auto CMD23 to set a 32-bit block count value to the argument of the CMD23 while executing Auto CMD23.
    /// It can be accessed only if no transaction is executing.
    #[inline]
    pub const fn set_arg2(self, val: u32) -> Self {
        Self(val)
    }
    /// Get Argument2.
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
    /// The SDMA transfer stops when the Host Controller detects carry out of the address from bit 11 to 12.
    /// These bits shall be supported when the SDMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1.
    /// ADMA does not use this register.
    #[inline]
    pub const fn set_host_sdma(self, val: u8) -> Self {
        Self((self.0 & !Self::HOST_SDMA) | (Self::HOST_SDMA & ((val as u16) << 12)))
    }
    /// Get HOST SDMA register.
    #[inline]
    pub const fn host_sdma(self) -> u8 {
        ((self.0 & Self::HOST_SDMA) >> 12) as u8
    }
    /// Specifies the block size of data transfers for CMD17, CMD18, CMD24, CMD25, and CMD53.
    /// Values ranging from 1 up to the maximum buffer size can be set.
    /// In case of memory, it shall be set up to 512 bytes (Refer to Implementation Note in Section 1.7.2).
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
    /// Set Left Blocks Count For Current Transfer.
    /// This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers.
    /// The Host Driver shall set this register to a value between 1 and the maximum block count.
    /// This register should be accessed only when no transaction is executing.
    #[inline]
    pub const fn set_blocks_count(self, val: u16) -> Self {
        Self(val)
    }
    /// Get Left Blocks Count For Current Transfer.
    /// This register should be accessed only when no transaction is executing.
    #[inline]
    pub const fn blocks_count(self) -> u16 {
        self.0
    }
}

/// Register that contains the SD Command Argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Argument(u32);

impl Argument {
    /// Set Command Argument 1.
    /// The SD command argument is specified as bit39-8 of Command-Format in the Physical Layer Specification.
    #[inline]
    pub const fn set_cmd_arg(self, val: u32) -> Self {
        Self(val)
    }
    /// Get Command Argument 1.
    #[inline]
    pub const fn cmd_arg(self) -> u32 {
        self.0
    }
}

/// Control register for the operation of data transfers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferMode(u16);

/// Multi/Single Block Select.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockMode {
    /// Other commands.
    Other,
    /// Multiple-block transfer commands using DAT line.
    MultiBlock,
}

/// Data Transfer Mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataTransferMode {
    /// Other modes.
    Other,
    /// Master in, slave out.
    MISO,
}

/// Auto CMD Mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AutoCMDMode {
    /// Auto CMD12 Enable.
    CMD12 = 1,
    /// Auto CMD23 Enable.
    CMD23 = 2,
}

impl TransferMode {
    const BLOCK_SELECT: u16 = 0x1 << 5;
    const DATA_TRANSFER: u16 = 0x1 << 4;
    const AUTO_CMD: u16 = 0x3 << 2;
    const BLOCK_COUNT: u16 = 0x1 << 1;
    const DMA_ENABLE: u16 = 0x1;

    /// Set Multi/Single Block Mode.
    #[inline]
    pub const fn set_block_mode(self, val: BlockMode) -> Self {
        Self((self.0 & !Self::BLOCK_SELECT) | (Self::BLOCK_SELECT & ((val as u16) << 5)))
    }
    /// Get Multi/Single Block Mode.
    #[inline]
    pub const fn block_mode(self) -> BlockMode {
        match (self.0 & Self::BLOCK_SELECT) >> 5 {
            1 => BlockMode::MultiBlock,
            _ => BlockMode::Other,
        }
    }
    /// Set Data Transfer Direction.
    #[inline]
    pub const fn set_data_transfer_mode(self, val: DataTransferMode) -> Self {
        Self((self.0 & !Self::DATA_TRANSFER) | (Self::DATA_TRANSFER & ((val as u16) << 4)))
    }
    /// Get Data Transfer Direction.
    #[inline]
    pub const fn data_transfer_mode(self) -> DataTransferMode {
        match (self.0 & Self::DATA_TRANSFER) >> 4 {
            1 => DataTransferMode::MISO,
            _ => DataTransferMode::Other,
        }
    }
    /// Set Auto CMD Mode.
    #[inline]
    pub const fn set_auto_cmd_mode(self, val: AutoCMDMode) -> Self {
        Self((self.0 & !Self::AUTO_CMD) | (Self::AUTO_CMD & ((val as u16) << 2)))
    }
    /// Get Auto CMD Mode
    #[inline]
    pub const fn auto_cmd_mode(self) -> AutoCMDMode {
        match (self.0 & Self::AUTO_CMD) >> 2 {
            1 => AutoCMDMode::CMD12,
            _ => AutoCMDMode::CMD23,
        }
    }
    /// Enable Block Count register.
    #[inline]
    pub const fn enable_block_count(self) -> Self {
        Self((self.0 & !Self::BLOCK_COUNT) | Self::BLOCK_COUNT & (1 << 1))
    }
    /// Disable Block Count register.
    #[inline]
    pub const fn disable_block_count(self) -> Self {
        Self((self.0 & !Self::BLOCK_COUNT) | Self::BLOCK_COUNT & (0 << 1))
    }
    /// Check if Block Count register is enabled.
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

/// CMD Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CmdType {
    Other,
    Suspend,
    Resume,
    Abort,
}
/// Response Type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResponseType {
    /// No Response.
    NoResponse,
    /// Response Length 136
    ResponseLen136,
    /// Response Length 48
    ResponseLen48,
    /// Response Length 48 check Busy after response
    ResponseLen48Check,
}

impl Command {
    const CMD_INDEX: u16 = 0x3F << 8;
    const CMD_TYPE: u16 = 0x3 << 6;
    const DATA_PRSENT: u16 = 0x1 << 5;
    const CMD_INDEX_CHECK: u16 = 0x1 << 4;
    const CMD_CRC: u16 = 0x1 << 3;
    const RESPONSE_TYPE: u16 = 0x3;

    /// These bits shall be set to the command number (CMD0-63, ACMD0-63) that is specified in bits 45-40 of the Command-Format in the Physical Layer Specification and SDIO Card Specification.
    #[inline]
    pub const fn set_cmd_num(self, val: u16) -> Self {
        Self((self.0 & Self::CMD_INDEX) | (Self::CMD_INDEX & (val << 8)))
    }
    /// Get command number.
    #[inline]
    pub const fn cmd_num(self) -> u16 {
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
            1 => CmdType::Suspend,
            2 => CmdType::Resume,
            3 => CmdType::Abort,
            _ => CmdType::Other,
        }
    }
    /// Set this bit to 1 to indicate that data is present and shall be transferred using the DAT line.
    #[inline]
    pub const fn set_data_present(self) -> Self {
        Self((self.0 & !Self::DATA_PRSENT) | (Self::DATA_PRSENT & (1 << 5)))
    }
    /// Set this bit to 0 for the following:
    /// (1) Commands using only CMD line (ex.CMD52).
    /// (2) Commands with no data transfer but using busy signal on DAT[0] line (R1b or R5b ex. CMD38).
    /// (3) Resume command.
    #[inline]
    pub const fn unset_data_present(self) -> Self {
        Self((self.0 & !Self::DATA_PRSENT) | (Self::DATA_PRSENT & (0 << 5)))
    }
    /// Check if Data Present bit is set.
    #[inline]
    pub const fn is_data_present(self) -> bool {
        (self.0 & Self::DATA_PRSENT) >> 5 == 1
    }
    /// Enable check the Index field.
    #[inline]
    pub const fn enable_index_check(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_CHECK) | (Self::CMD_INDEX_CHECK & (1 << 4)))
    }
    /// Disable check the Index field.
    #[inline]
    pub const fn disable_index_check(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_CHECK) | (Self::CMD_INDEX_CHECK & (0 << 4)))
    }
    /// Check if check the Index field is enabled.
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
    /// Set Command Response.
    #[inline]
    pub const fn set_response(self, val: u128) -> Self {
        Self(val)
    }
    /// Get Command Response.
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
    /// Set Buffer Data
    /// The Host Controller buffer can be accessed through this 32-bit Data Port register.
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

/// 32-bit read only register to get status of the Host Controller.
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

    /// Get CMD line signal level.
    #[inline]
    pub const fn cmd_line(self) -> u8 {
        ((self.0 & Self::CMD_LINE) >> 24) as u8
    }
    /// Get DAT line signal level.
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
    /// Check if DAT line is active.
    #[inline]
    pub const fn is_dat_line_active(self) -> bool {
        (self.0 & Self::DAT_LINE_ACTIVE) >> 2 == 1
    }
    /// Check if DAT line is busy.
    #[inline]
    pub const fn is_dat_line_busy(self) -> bool {
        (self.0 & Self::CMD_INHIBIT1) >> 1 == 1
    }
    /// Check if CMD line is busy.
    #[inline]
    pub const fn is_cmd_line_busy(self) -> bool {
        (self.0 & Self::CMD_INHIBIT0) == 1
    }
}

/// Host Control 1 Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControl1(u8);

/// Source for the card detection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardSignal {
    /// SDCD# is selected (for normal use)
    SDCD,
    /// The Card Detect Test Level is selected (for test purpose)
    TestLevel,
}

/// Bus width mode for embedded device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BusWidthMode {
    /// 8-bit Bus Width.
    SelectByDataTrabsferWidth,
    /// Bus Width is Selected by Data Transfer Width.
    EightBitWidth,
}

/// DMA mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DMAMode {
    /// SDMA is selected.
    SDMA = 0,
    /// 32-bit Address ADMA2 is selected.
    ADMA2 = 2,
}

/// Speed Mode.
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

/// Caution LED state.
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
    /// Enabled while card_detect_signal is TestLevel.
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
            _ => BusWidthMode::SelectByDataTrabsferWidth,
        }
    }
    /// Set DMA mode.
    #[inline]
    pub const fn set_dma_mode(self, val: DMAMode) -> Self {
        Self((self.0 & !Self::DMA_SELECT) | (Self::DMA_SELECT & ((val as u8) << 3)))
    }
    /// Get DMA mode.
    #[inline]
    pub const fn dma_mode(self) -> DMAMode {
        match (self.0 & Self::DMA_SELECT) >> 3 {
            0 => DMAMode::SDMA,
            2 => DMAMode::ADMA2,
            _ => unreachable!(),
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

/// Power Control Register.
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

    /// Set SD Bus Voltage.
    #[inline]
    pub const fn set_bus_voltage(self, val: BusVoltage) -> Self {
        Self((self.0 & !Self::SD_BUS_VOLTAGE) | (Self::SD_BUS_VOLTAGE & ((val as u8) << 1)))
    }
    /// Get SD Bus Voltage.
    #[inline]
    pub const fn bus_voltage(self) -> BusVoltage {
        match (self.0 & Self::SD_BUS_VOLTAGE) >> 1 {
            5 => BusVoltage::V1_8,
            6 => BusVoltage::V3_0,
            7 => BusVoltage::V3_3,
            _ => unreachable!(),
        }
    }
    /// Enable SD Bus power.
    /// Before setting this bit, the SD Host Driver shall set SD Bus Voltage Select.
    #[inline]
    pub const fn enable_bus_power(self) -> Self {
        Self((self.0 & !Self::SD_BUS_POWER) | (Self::SD_BUS_POWER & 1))
    }
    /// Disable SD Bus power.
    /// Host Controller detects the No Card state, this bit shall be cleared.
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
    /// To cancel stop at the block gap, set Stop At Block Gap Request to 0 and set this bit 1 to restart the transfer.
    /// The Host Controller automatically clears this bit.
    #[inline]
    pub const fn restart_transaction(self) -> Self {
        Self((self.0 & !Self::CONTINUE_REQUEST_CTRL) | (Self::CONTINUE_REQUEST_CTRL & (1 << 1)))
    }
    /// Set Stop At Block Gap Request bit.
    #[inline]
    pub const fn set_stop_at_block_gap_req(self, val: u8) -> Self {
        Self((self.0 & !Self::STOP_AT_BG) | (Self::STOP_AT_BG & val))
    }
    /// Get Stop At Block Gap Request bit.
    #[inline]
    pub const fn stop_at_block_gap_req(self) -> u8 {
        self.0 & Self::STOP_AT_BG
    }
}

/// Register which is mandatory for the Host Controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WakeupControl(u8);

impl WakeupControl {
    const CARD_REMOVAL: u8 = 0x1 << 2;
    const CARD_INSERTED: u8 = 0x1 << 1;
    const CARD_INTERRUPT: u8 = 0x1;

    /// Wakeup Event Enable On SD Card Removal.
    #[inline]
    pub const fn enable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 2)))
    }
    /// Wakeup Event Disable On SD Card Removal.
    #[inline]
    pub const fn disable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (0 << 2)))
    }
    /// Check if Wakeup Event On SD Card Removal is enabled.
    #[inline]
    pub const fn is_card_removal_enable(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 2 == 1
    }
    /// Wakeup Event Enable On SD Card Insertion.
    #[inline]
    pub const fn enable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTED) | (Self::CARD_INSERTED & (1 << 1)))
    }
    /// Wakeup Event Disable On SD Card Insertion.
    #[inline]
    pub const fn disable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTED) | (Self::CARD_INSERTED & (0 << 1)))
    }
    /// Check if Wakeup Event On SD Card Insertion is enabled.
    #[inline]
    pub const fn is_card_insertion_enable(self) -> bool {
        (self.0 & Self::CARD_INSERTED) >> 1 == 1
    }
    /// Wakeup Event Enable On Card Interrupt.
    #[inline]
    pub const fn enable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INTERRUPT) | (Self::CARD_INTERRUPT & 1))
    }
    /// Wakeup Event Disable On Card Interrupt.
    #[inline]
    pub const fn disable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INTERRUPT) | (Self::CARD_INTERRUPT & 0))
    }
    /// Check if Wakeup Event On SD Card Interrupt is enabled.
    #[inline]
    pub const fn is_card_int_enable(self) -> bool {
        self.0 & Self::CARD_INTERRUPT == 1
    }
}

/// Control register for SDCLK in SD Mode and RCLK in UHS-II Mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockControl(u16);

/// Clock generator mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClkGenMode {
    /// Divided Clock Mode.
    DividedClk,
    /// Programmable Clock Mode.
    ProgrammableClk,
}

impl ClockControl {
    const SD_CLK_FREQ: u16 = 0xFF << 8;
    const SD_CLK_FREQ_UPPER: u16 = 0x3 << 6;
    const CLK_GENERATOR: u16 = 0x1 << 5;
    const SD_CLK_EN: u16 = 0x1 << 2;
    const INTERNAL_CLK_STABLE: u16 = 0x1 << 1;
    const INTERNAL_CLK_EN: u16 = 0x1;

    /// Set SDCLK Frequency.
    #[inline]
    pub const fn set_sd_clk_freq(self, val: u8) -> Self {
        Self((self.0 & !Self::SD_CLK_FREQ) | (Self::SD_CLK_FREQ & ((val as u16) << 8)))
    }
    /// Get SDCLK Frequency.
    #[inline]
    pub const fn sd_clk_freq(self) -> u8 {
        ((self.0 & Self::SD_CLK_FREQ) >> 8) as u8
    }
    /// Set Upper SDCLK Frequency.
    /// Host Controller Version 1.00 and 2.00 do not support these bits and they are treated as 00b fixed value.
    #[inline]
    pub const fn set_sd_clk_freq_upper(self, val: u8) -> Self {
        Self((self.0 & !Self::SD_CLK_FREQ_UPPER) | (Self::SD_CLK_FREQ_UPPER & ((val as u16) << 6)))
    }
    /// Get Upper SDCLK Frequency.
    /// Host Controller Version 1.00 and 2.00 do not support these bits and they are treated as 00b fixed value.
    #[inline]
    pub const fn sd_clk_freq_upper(self) -> u8 {
        ((self.0 & Self::SD_CLK_FREQ_UPPER) >> 6) as u8
    }
    /// Set Clock Generato mode.
    #[inline]
    pub const fn set_clk_gen_mode(self, val: ClkGenMode) -> Self {
        Self((self.0 & !Self::CLK_GENERATOR) | (Self::CLK_GENERATOR & ((val as u16) << 5)))
    }
    /// Get Clock Generato mode.
    #[inline]
    pub const fn clk_gen_mode(self) -> ClkGenMode {
        match (self.0 & Self::CLK_GENERATOR) >> 5 {
            1 => ClkGenMode::ProgrammableClk,
            _ => ClkGenMode::DividedClk,
        }
    }
    /// Enable SD Clock.
    #[inline]
    pub const fn enable_sd_clk(self) -> Self {
        Self((self.0 & !Self::SD_CLK_EN) | (Self::SD_CLK_EN & (1 << 2)))
    }
    /// Disable SD Clock.
    #[inline]
    pub const fn disable_sd_clk(self) -> Self {
        Self((self.0 & !Self::SD_CLK_EN) | (Self::SD_CLK_EN & (0 << 2)))
    }
    /// Check if SD Clock is enabled.
    #[inline]
    pub const fn is_sd_clk_enabled(self) -> bool {
        (self.0 & Self::SD_CLK_EN) >> 2 == 1
    }
    /// Check if Internal Clock stable.
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
    /// Check if internal clk is enablee.
    #[inline]
    pub const fn is_internal_clk_enable(self) -> bool {
        self.0 & Self::INTERNAL_CLK_EN == 1
    }
}

/// Timeout Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimeoutControl(u8);

impl TimeoutControl {
    const DATA_TIMEOUT_CNT: u8 = 0xF;

    /// Set Data Timeout Counter Value.
    #[inline]
    pub const fn set_timeout_val(self, val: u8) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUT_CNT) | (Self::DATA_TIMEOUT_CNT & val))
    }
    /// Get Data Timeout Counter Value.
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

    /// Software reset DAT line.
    #[inline]
    pub const fn reset_dat(self) -> Self {
        Self((self.0 & !Self::SOFT_RESET_DAT) | (Self::SOFT_RESET_DAT & (1 << 2)))
    }
    /// Check if dat line reset is finished (Cleared to 0).
    #[inline]
    pub const fn is_reset_dat_finished(self) -> bool {
        (self.0 & Self::SOFT_RESET_DAT) >> 2 == 0
    }
    /// Software reset CMD line.
    #[inline]
    pub const fn reset_cmd(self) -> Self {
        Self((self.0 & !Self::SOFT_RESET_CMD) | (Self::SOFT_RESET_CMD & (1 << 1)))
    }
    /// Check if cmd line reset is finished (Cleared to 0).
    #[inline]
    pub const fn is_reset_cmd_finished(self) -> bool {
        (self.0 & Self::SOFT_RESET_CMD) >> 1 == 0
    }
    /// Software reset all.
    /// This reset affects the entire Host Controller except for the card detection circuit.
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

/// The reads of register are affected by the Normal Interrupt Status Enable.  
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

    /// Check if Error Interrupt occurs.
    #[inline]
    pub const fn if_err_int_occurs(self) -> bool {
        (self.0 & Self::ERROR_INT) >> 15 == 1
    }
    /// Check if Re-Tuning Event occurs.
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
    /// Check if Card Interrupt occurs.
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

/// Register that shows the defined Interrupt Status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptStatus(u16);

impl ErrorInterruptStatus {
    const VENDOR_SPECIFIC_ERROR: u16 = 0xF << 12;
    const TUNING_ERROR: u16 = 0x1 << 10;
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

    /// Get Vendor Specific Error Status.
    #[inline]
    pub const fn vendor_specific_err(self) -> u8 {
        ((self.0 & Self::VENDOR_SPECIFIC_ERROR) >> 12) as u8
    }
    /// Clear Vendor Specific Error Status.
    #[inline]
    pub const fn clear_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0xF << 12)))
    }
    /// Check if Tuning Error occurs.
    #[inline]
    pub const fn if_tuning_err_occurs(self) -> bool {
        (self.0 & Self::TUNING_ERROR) >> 10 == 1
    }
    /// Clear Tuning Error bit.
    pub const fn clear_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (1 << 10)))
    }
    /// Check if ADMA Error occurs.
    #[inline]
    pub const fn if_adma_err_occurs(self) -> bool {
        (self.0 & Self::ADMA_EROR) >> 9 == 1
    }
    /// Clear ADMA Error bit.
    #[inline]
    pub const fn clear_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_EROR) | (Self::ADMA_EROR & (1 << 9)))
    }
    /// Check if Auto CMD Error occurs.
    #[inline]
    pub const fn if_auto_cmd_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_ERROR) >> 8 == 1
    }
    /// Clear Auto CMD Error bit.
    #[inline]
    pub const fn clear_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (1 << 8)))
    }
    /// Check if Current Limit Error occurs.
    #[inline]
    pub const fn if_current_limit_err_occurs(self) -> bool {
        (self.0 & Self::CURRENT_LIMIT_ERROR) >> 7 == 1
    }
    /// Clear Current Limit Error bit.
    #[inline]
    pub const fn clear_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (1 << 7)))
    }
    /// Check if Data End Bit Error occurs.
    #[inline]
    pub const fn if_data_end_bit_err_occurs(self) -> bool {
        (self.0 & Self::DATA_END_BIT_ERROR) >> 6 == 1
    }
    /// Clear Data End Bit Error bit.
    #[inline]
    pub const fn clear_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (1 << 6)))
    }
    /// Check if Data CRC Error occurs.
    #[inline]
    pub const fn if_data_crc_err_occurs(self) -> bool {
        (self.0 & Self::DATA_CRC_ERROR) >> 5 == 1
    }
    /// Clear Data CRC Error bit.
    #[inline]
    pub const fn clear_data_crc_err(self) -> Self {
        Self((self.0 & Self::DATA_CRC_ERROR) | (Self::DATA_CRC_ERROR & (1 << 5)))
    }
    /// Check if Data Timeout Error occurs.
    #[inline]
    pub const fn if_data_timeout_err_occurs(self) -> bool {
        (self.0 & Self::DATA_TIMEOUT_ERROR) >> 4 == 1
    }
    /// Clear Data Timeout Error bit.
    #[inline]
    pub const fn clear_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUT_ERROR) | (Self::DATA_TIMEOUT_ERROR & (1 << 4)))
    }
    /// Check if Command Index Error occurs.
    #[inline]
    pub const fn if_cmd_index_err_occurs(self) -> bool {
        (self.0 & Self::CMD_INDEX_ERROR) >> 3 == 1
    }
    /// Clear Command Index Error bit.
    #[inline]
    pub const fn clear_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_ERROR) | (Self::CMD_INDEX_ERROR & (1 << 3)))
    }
    /// Check if Command End Bit Error occurs.
    #[inline]
    pub const fn if_cmd_end_bit_err_occurs(self) -> bool {
        (self.0 & Self::CMD_END_BIT_ERROR) >> 2 == 1
    }
    /// Clear Command End Bit Error bit.
    #[inline]
    pub const fn clear_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (1 << 2)))
    }
    /// Check if Command CRC Error occurs.
    #[inline]
    pub const fn if_cmd_crc_err_occurs(self) -> bool {
        (self.0 & Self::CMD_CRC_ERROR) >> 1 == 1
    }
    /// Clear Command CRC Error bit.
    #[inline]
    pub const fn clear_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (1 << 1)))
    }
    /// Check if Command Timeout Error occurs.
    #[inline]
    pub const fn if_cmd_timeout_err_occurs(self) -> bool {
        self.0 & Self::CMD_TIMEOUT_ERROR == 1
    }
    /// Clear Command Timeout Error bit.
    #[inline]
    pub const fn clear_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 1))
    }
}

/// Register that sets to 1 enables Interrupt Status.
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
    /// Enable Re-Tuning Event Status.
    #[inline]
    pub const fn enable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING_EVENT) | (Self::RE_TUNING_EVENT & (1 << 12)))
    }
    /// Disable Re-Tuning Event Status.
    #[inline]
    pub const fn disable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING_EVENT) | (Self::RE_TUNING_EVENT & (0 << 12)))
    }
    /// Check if Re-Tuning Event Status is enabled.
    #[inline]
    pub const fn is_retuning_enabled(self) -> bool {
        (self.0 & Self::RE_TUNING_EVENT) >> 12 == 1
    }
    /// Enable INT_C Status.
    #[inline]
    pub const fn enable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (1 << 11)))
    }
    /// Disable INT_C Status.
    #[inline]
    pub const fn disable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (0 << 11)))
    }
    /// Check if INT_C Status is enabled.
    #[inline]
    pub const fn is_int_c_enabled(self) -> bool {
        (self.0 & Self::INT_C) >> 11 == 1
    }
    /// Enable INT_B Status.
    #[inline]
    pub const fn enable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (1 << 10)))
    }
    /// Disable INT_B Status.
    #[inline]
    pub const fn disable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (0 << 10)))
    }
    /// Check if INT_B Status is enabled.
    #[inline]
    pub const fn is_int_b_enabled(self) -> bool {
        (self.0 & Self::INT_B) >> 10 == 1
    }
    /// Enable INT_A Status.
    #[inline]
    pub const fn enable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (1 << 9)))
    }
    /// Disable INT_B Status.
    #[inline]
    pub const fn disable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (0 << 9)))
    }
    /// Check if INT_A Status is enabled.
    #[inline]
    pub const fn is_int_a_enabled(self) -> bool {
        (self.0 & Self::INT_A) >> 9 == 1
    }
    /// Enable Card Interrupt.
    #[inline]
    pub const fn enable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (1 << 8)))
    }
    /// Disable Card Interrupt.
    #[inline]
    pub const fn disable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (0 << 8)))
    }
    /// Check if Card Interrupt is enabled.
    #[inline]
    pub const fn is_card_int_enabled(self) -> bool {
        (self.0 & Self::CARD_INT) >> 8 == 1
    }
    /// Enable Card Removal Status.
    #[inline]
    pub const fn enable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 7)))
    }
    /// Disable Card Removal Status.
    #[inline]
    pub const fn disable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (0 << 7)))
    }
    /// Check if Card Removal Status is enabled.
    #[inline]
    pub const fn is_card_removal_enabled(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 7 == 1
    }
    /// Enable Card Insertion Status.
    #[inline]
    pub const fn enable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (1 << 6)))
    }
    /// Disable Card Insertion Status.
    #[inline]
    pub const fn disable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (0 << 6)))
    }
    /// Check if Card Insertion Status is enabled.
    #[inline]
    pub const fn is_card_insertion_enabled(self) -> bool {
        (self.0 & Self::CARD_INSERTION) >> 6 == 1
    }
    /// Enable Buffer Read Ready Status.
    #[inline]
    pub const fn enable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (1 << 5)))
    }
    /// Disable Buffer Read Ready Status.
    #[inline]
    pub const fn disable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (0 << 5)))
    }
    /// Check if Buffer Read Ready Status is enabled.
    #[inline]
    pub const fn is_buffer_read_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_READ) >> 5 == 1
    }
    /// Enable Buffer Write Ready Status.
    #[inline]
    pub const fn enable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (1 << 4)))
    }
    /// Disable Buffer Write Ready  Status.
    #[inline]
    pub const fn disable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (0 << 4)))
    }
    /// Check if Buffer Write Ready Status is enabled.
    #[inline]
    pub const fn is_buffer_write_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_WRITE) >> 4 == 1
    }
    /// Enable DMA Interrupt Status.
    #[inline]
    pub const fn enable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (1 << 3)))
    }
    /// Disable DMA Interrupt Status.
    #[inline]
    pub const fn disable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (0 << 3)))
    }
    /// Check if DMA Interrupt Status is enabled.
    #[inline]
    pub const fn is_dma_int_enabled(self) -> bool {
        (self.0 & Self::DMA_INT) >> 3 == 1
    }
    /// Enable Block Gap Status.
    #[inline]
    pub const fn enable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (1 << 2)))
    }
    /// Disable Block Gap Status.
    #[inline]
    pub const fn disable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (0 << 2)))
    }
    /// Check if Block Gap Status is enabled.
    #[inline]
    pub const fn is_block_gap_enabled(self) -> bool {
        (self.0 & Self::BLOCK_GAP) >> 2 == 1
    }

    /// Enable Transfer Complete Status.
    #[inline]
    pub const fn enable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (1 << 1)))
    }
    /// Disable Transfer Complete Status.
    #[inline]
    pub const fn disable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (0 << 1)))
    }
    /// Check if Transfer Complete Status is enabled.
    #[inline]
    pub const fn is_transfer_complete_enabled(self) -> bool {
        (self.0 & Self::TRANSFER_COMPLETE) >> 1 == 1
    }
    /// Enable Command Complete Status.
    #[inline]
    pub const fn enable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 1))
    }
    /// Disable Command Complete Status.
    #[inline]
    pub const fn disable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 0))
    }
    /// Check if Command Complete is enabled.
    #[inline]
    pub const fn is_cmd_complete_enabled(self) -> bool {
        self.0 & Self::CMD_COMPLETE == 1
    }
}

/// Register that sets to 1 enables Interrupt Status.
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
    const DATA_CRC_EROOR: u16 = 0x1 << 5;
    const DATA_TIMEOUE_ERROR: u16 = 0x1 << 4;
    const CMD_INDEX_EROOR: u16 = 0x1 << 3;
    const CMD_END_BIT_ERROR: u16 = 0x1 << 2;
    const CMD_CRC_ERROR: u16 = 0x1 << 1;
    const CMD_TIMEOUT_ERROR: u16 = 0x1;

    /// Enable Vendor Specific Error Status.
    #[inline]
    pub const fn enable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0xF << 12)))
    }
    /// Disable Vendor Specific Error Status.
    #[inline]
    pub const fn disable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0 << 12)))
    }
    /// Check if Vendor Specific Error Status is enabled.
    #[inline]
    pub const fn is_vendor_specific_err_enabled(self) -> bool {
        (self.0 & Self::VENDOR_SPECIFIC_ERROR) >> 12 == 0xF
    }
    /// Enable Tuning Error Status.
    #[inline]
    pub const fn enable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (1 << 10)))
    }
    /// Disable Tuning Error Status.
    #[inline]
    pub const fn disable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (0 << 10)))
    }
    /// Check if Tuning Error Status is enabled.
    #[inline]
    pub const fn is_tuning_err_enabled(self) -> bool {
        (self.0 & Self::TUNING_ERROR) >> 10 == 1
    }
    /// Enable ADMA Error Status.
    #[inline]
    pub const fn enable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (1 << 9)))
    }
    /// Disable ADMA Error Status.
    #[inline]
    pub const fn disable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (0 << 9)))
    }
    /// Check if ADMA Error Status is enabled.
    #[inline]
    pub const fn is_adma_err_enabled(self) -> bool {
        (self.0 & Self::ADMA_ERROR) >> 9 == 1
    }
    /// Enable Auto CMD Error Status.
    #[inline]
    pub const fn enable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (1 << 8)))
    }
    /// Disable Auto CMD Error Status.
    #[inline]
    pub const fn disable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (0 << 8)))
    }
    /// Check if Auto CMD Error Status is enabled.
    #[inline]
    pub const fn is_auto_cmd_err_enabled(self) -> bool {
        (self.0 & Self::AUTO_CMD_ERROR) >> 8 == 1
    }
    /// Enable Current Limit Error Status.
    #[inline]
    pub const fn enable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (1 << 7)))
    }
    /// Disable Current Limit Error Status.
    #[inline]
    pub const fn disable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (0 << 7)))
    }
    /// Check if Current Limit Error Status is enabled.
    #[inline]
    pub const fn is_current_limit_err_enabled(self) -> bool {
        (self.0 & Self::CURRENT_LIMIT_ERROR) >> 7 == 1
    }
    /// Enable Data End Bit Error Status.
    #[inline]
    pub const fn enable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (1 << 6)))
    }
    /// Disable Data End Bit Error Status.
    #[inline]
    pub const fn disable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (0 << 6)))
    }
    /// Check if Data End Bit Error Status is enabled.
    #[inline]
    pub const fn is_data_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::DATA_END_BIT_ERROR) >> 6 == 1
    }
    /// Enable Data CRC Error Status.
    #[inline]
    pub const fn enable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_EROOR) | (Self::DATA_CRC_EROOR & (1 << 5)))
    }
    /// Disable Data CRC Error Status.
    #[inline]
    pub const fn disable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_EROOR) | (Self::DATA_CRC_EROOR & (0 << 5)))
    }
    /// Check if Data CRC Error Status is enabled.
    #[inline]
    pub const fn is_data_crc_err_enabled(self) -> bool {
        (self.0 & Self::DATA_CRC_EROOR) >> 5 == 1
    }
    /// Enable Data Timeout Error Status.
    #[inline]
    pub const fn enable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (1 << 4)))
    }
    /// Disable Data Timeout Error Status.
    #[inline]
    pub const fn disable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (0 << 4)))
    }
    /// Check if Data Timeout Error Status is enabled.
    #[inline]
    pub const fn is_data_timeout_err_enabled(self) -> bool {
        (self.0 & Self::DATA_TIMEOUE_ERROR) >> 4 == 1
    }

    /// Enable Command Index Error Status.
    #[inline]
    pub const fn enable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_EROOR) | (Self::CMD_INDEX_EROOR & (1 << 3)))
    }
    /// Disable Command Index Error Status.
    #[inline]
    pub const fn disable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_EROOR) | (Self::CMD_INDEX_EROOR & (0 << 3)))
    }
    /// Check if Command Index Error Status is enabled.
    #[inline]
    pub const fn is_cmd_index_err_enabled(self) -> bool {
        (self.0 & Self::CMD_INDEX_EROOR) >> 3 == 1
    }
    /// Enable Command End Bit Error Status.
    #[inline]
    pub const fn enable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (1 << 2)))
    }
    /// Disable Command End Bit Error Status.
    #[inline]
    pub const fn disable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (0 << 2)))
    }
    /// Check if Command End Bit Error Status is enabled.
    #[inline]
    pub const fn is_cmd_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::CMD_END_BIT_ERROR) >> 2 == 1
    }
    /// Enable Command CRC Error Status.
    #[inline]
    pub const fn enable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (1 << 1)))
    }
    /// Disable Command CRC Error Status.
    #[inline]
    pub const fn disable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (0 << 1)))
    }
    /// Check if Command CRC Error Status is enabled.
    #[inline]
    pub const fn is_cmd_crc_err_enabled(self) -> bool {
        (self.0 & Self::CMD_CRC_ERROR) >> 1 == 1
    }
    /// Enable Command Timeout Error Status.
    #[inline]
    pub const fn enable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 1))
    }
    /// Disable Command Timeout Error Status.
    #[inline]
    pub const fn disable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 0))
    }
    /// Check if Command Timeout Error Status is enabled.
    #[inline]
    pub const fn is_cmd_timeout_err_enabled(self) -> bool {
        self.0 & Self::CMD_TIMEOUT_ERROR == 1
    }
}

/// Register that selects which interrupt status is indicated to the Host System as the interrupt.
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
    /// Enable Re-Tuning Event Signal.
    #[inline]
    pub const fn enable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING) | (Self::RE_TUNING & (1 << 12)))
    }
    /// Disable Re-Tuning Event Signal.
    #[inline]
    pub const fn disable_re_tuning(self) -> Self {
        Self((self.0 & !Self::RE_TUNING) | (Self::RE_TUNING & (0 << 12)))
    }
    /// Check if Re-Tuning Event Signal is enabled.
    #[inline]
    pub const fn is_re_tuning_enabled(self) -> bool {
        (self.0 & Self::RE_TUNING) >> 12 == 1
    }
    /// Enable INT_C Signal.
    #[inline]
    pub const fn enable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (1 << 11)))
    }
    /// Disable INT_C Signal.
    #[inline]
    pub const fn disable_int_c(self) -> Self {
        Self((self.0 & !Self::INT_C) | (Self::INT_C & (0 << 11)))
    }
    /// Check if INT_C Signal is enabled.
    #[inline]
    pub const fn is_int_c_enabled(self) -> bool {
        (self.0 & Self::INT_C) >> 11 == 1
    }
    /// Enable INT_B Signal.
    #[inline]
    pub const fn enable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (1 << 10)))
    }
    /// Disable INT_B Signal.
    #[inline]
    pub const fn disable_int_b(self) -> Self {
        Self((self.0 & !Self::INT_B) | (Self::INT_B & (0 << 10)))
    }
    /// Check if INT_B Signal is enabled.
    #[inline]
    pub const fn is_int_b_enabled(self) -> bool {
        (self.0 & Self::INT_B) >> 10 == 1
    }
    /// Enable INT_A Signal.
    #[inline]
    pub const fn enable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (1 << 9)))
    }
    /// Disable INT_A Signal.
    #[inline]
    pub const fn disable_int_a(self) -> Self {
        Self((self.0 & !Self::INT_A) | (Self::INT_A & (0 << 9)))
    }
    /// Check if INT_A Signal is enabled.
    #[inline]
    pub const fn is_int_a_enabled(self) -> bool {
        (self.0 & Self::INT_A) >> 9 == 1
    }
    /// Enable Card Interrupt Signal.
    #[inline]
    pub const fn enable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (1 << 8)))
    }
    /// Disable Card Interrupt Signal.
    #[inline]
    pub const fn disable_card_int(self) -> Self {
        Self((self.0 & !Self::CARD_INT) | (Self::CARD_INT & (0 << 8)))
    }
    /// Check if Card Interrupt Signal is enabled.
    #[inline]
    pub const fn is_card_int_enabled(self) -> bool {
        (self.0 & Self::CARD_INT) >> 8 == 1
    }
    /// Enable Card Removal Signal.
    #[inline]
    pub const fn enable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (1 << 7)))
    }
    /// Disable Card Removal Signal.
    #[inline]
    pub const fn disable_card_removal(self) -> Self {
        Self((self.0 & !Self::CARD_REMOVAL) | (Self::CARD_REMOVAL & (0 << 7)))
    }
    /// Check if Card Removal Signal is enabled.
    #[inline]
    pub const fn is_card_removal_enabled(self) -> bool {
        (self.0 & Self::CARD_REMOVAL) >> 7 == 1
    }
    /// Enable Card Insertion Signal.
    #[inline]
    pub const fn enable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (1 << 6)))
    }
    /// Disable Card Insertion Signal.
    #[inline]
    pub const fn disable_card_insertion(self) -> Self {
        Self((self.0 & !Self::CARD_INSERTION) | (Self::CARD_INSERTION & (0 << 6)))
    }
    /// Check if Card Insertion Signal is enabled.
    #[inline]
    pub const fn is_card_insertion_enabled(self) -> bool {
        (self.0 & Self::CARD_INSERTION) >> 6 == 1
    }
    /// Enable Buffer Read Ready Signal.
    #[inline]
    pub const fn enable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (1 << 5)))
    }
    /// Disable Buffer Read Ready Signal.
    #[inline]
    pub const fn disable_buffer_read_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_READ) | (Self::BUFFER_READ & (0 << 5)))
    }
    /// Check if Buffer Read Ready Signal is enabled.
    #[inline]
    pub const fn is_buffer_read_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_READ) >> 5 == 1
    }
    /// Enable Buffer Write Ready Signal.
    #[inline]
    pub const fn enable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (1 << 4)))
    }
    /// Disable Buffer Write Ready Signal.
    #[inline]
    pub const fn disable_buffer_write_ready(self) -> Self {
        Self((self.0 & !Self::BUFFER_WRITE) | (Self::BUFFER_WRITE & (0 << 4)))
    }
    /// Check if Buffer Write Ready Signal is enabled.
    #[inline]
    pub const fn is_buffer_write_ready_enabled(self) -> bool {
        (self.0 & Self::BUFFER_WRITE) >> 4 == 1
    }
    /// Enable DMA Interrupt Signal.
    #[inline]
    pub const fn enable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (1 << 3)))
    }
    /// Disable DMA Interrupt Signal.
    #[inline]
    pub const fn disable_dma_int(self) -> Self {
        Self((self.0 & !Self::DMA_INT) | (Self::DMA_INT & (0 << 3)))
    }
    /// Check if DMA Interrupt Signal is enabled.
    #[inline]
    pub const fn is_dma_int_enabled(self) -> bool {
        (self.0 & Self::DMA_INT) >> 3 == 1
    }
    /// Enable Block Gap Event Signal.
    #[inline]
    pub const fn enable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (1 << 2)))
    }
    /// Disable Block Gap Event Signal.
    #[inline]
    pub const fn disable_block_gap(self) -> Self {
        Self((self.0 & !Self::BLOCK_GAP) | (Self::BLOCK_GAP & (0 << 2)))
    }
    /// Check if Block Gap Event Signal is enabled.
    #[inline]
    pub const fn is_block_gap_enabled(self) -> bool {
        (self.0 & Self::BLOCK_GAP) >> 2 == 1
    }
    /// Enable Transfer Complete Signal.
    #[inline]
    pub const fn enable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (1 << 1)))
    }
    /// Disable Transfer Complete Signal.
    #[inline]
    pub const fn disable_transfer_complete(self) -> Self {
        Self((self.0 & !Self::TRANSFER_COMPLETE) | (Self::TRANSFER_COMPLETE & (0 << 1)))
    }
    /// Check if Transfer Complete Signal is enabled.
    #[inline]
    pub const fn is_transfer_complete_enabled(self) -> bool {
        (self.0 & Self::TRANSFER_COMPLETE) >> 1 == 1
    }
    /// Enable Command Complete Signal.
    #[inline]
    pub const fn enable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 1))
    }
    /// Disable Command Complete Signal.
    #[inline]
    pub const fn disable_cmd_complete(self) -> Self {
        Self((self.0 & !Self::CMD_COMPLETE) | (Self::CMD_COMPLETE & 0))
    }
    /// Check if Command Complete Signal is enabled.
    #[inline]
    pub const fn is_cmd_complete_enabled(self) -> bool {
        self.0 & Self::CMD_COMPLETE == 1
    }
}

/// Register that selects which interrupt status is notified to the Host System as the interrupt.
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

    /// Enable Vendor Specific Error Signal.
    #[inline]
    pub const fn enable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0xF << 12)))
    }
    /// Disable Vendor Specific Error Signal.
    #[inline]
    pub const fn disable_vendor_specific_err(self) -> Self {
        Self((self.0 & !Self::VENDOR_SPECIFIC_ERROR) | (Self::VENDOR_SPECIFIC_ERROR & (0 << 12)))
    }
    /// Check if Vendor Specific Error Signal is enabled.
    #[inline]
    pub const fn is_vendor_specific_err_enabled(self) -> bool {
        (self.0 & Self::VENDOR_SPECIFIC_ERROR) >> 12 == 0xF
    }
    /// Enable Tuning Error Signal.
    #[inline]
    pub const fn enable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (1 << 10)))
    }
    /// Disable Tuning Error Signal.
    #[inline]
    pub const fn disable_tuning_err(self) -> Self {
        Self((self.0 & !Self::TUNING_ERROR) | (Self::TUNING_ERROR & (0 << 10)))
    }
    /// Check if Tuning Error Signal is enabled.
    #[inline]
    pub const fn is_tuning_err_enabled(self) -> bool {
        (self.0 & Self::TUNING_ERROR) >> 10 == 1
    }
    /// Enable ADMA Error Signal.
    #[inline]
    pub const fn enable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (1 << 9)))
    }
    /// Disable ADMA Error Signal.
    #[inline]
    pub const fn disable_adma_err(self) -> Self {
        Self((self.0 & !Self::ADMA_ERROR) | (Self::ADMA_ERROR & (0 << 9)))
    }
    /// Check if ADMA Error Signal is enabled.
    #[inline]
    pub const fn is_adma_err_enabled(self) -> bool {
        (self.0 & Self::ADMA_ERROR) >> 9 == 1
    }
    /// Enable Auto CMD Error Signal.
    #[inline]
    pub const fn enable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (1 << 8)))
    }
    /// Disable Auto CMD Error Signal.
    #[inline]
    pub const fn disable_auto_cmd_err(self) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & (0 << 8)))
    }
    /// Check if Auto CMD Error Signal is enabled.
    #[inline]
    pub const fn is_auto_cmd_err_enabled(self) -> bool {
        (self.0 & Self::AUTO_CMD_ERROR) >> 8 == 1
    }
    /// Enable Current Limit Error Signal.
    #[inline]
    pub const fn enable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (1 << 7)))
    }
    /// Disable Current Limit Error Signal.
    #[inline]
    pub const fn disable_current_limit_err(self) -> Self {
        Self((self.0 & !Self::CURRENT_LIMIT_ERROR) | (Self::CURRENT_LIMIT_ERROR & (0 << 7)))
    }
    /// Check if Current Limit Error Signal is enabled.
    #[inline]
    pub const fn is_current_limit_err_enabled(self) -> bool {
        (self.0 & Self::CURRENT_LIMIT_ERROR) >> 7 == 1
    }
    /// Enable Data End Bit Error Signal.
    #[inline]
    pub const fn enable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (1 << 6)))
    }
    /// Disable Data End Bit Error Signal.
    #[inline]
    pub const fn disable_data_end_bit_err(self) -> Self {
        Self((self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & (0 << 6)))
    }
    /// Check if Data End Bit Error Signal is enabled.
    #[inline]
    pub const fn is_data_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::DATA_END_BIT_ERROR) >> 6 == 1
    }
    /// Enable Data CRC Error Signal.
    #[inline]
    pub const fn enable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_EROOR) | (Self::DATA_CRC_EROOR & (1 << 5)))
    }
    /// Disable Data CRC Error Signal.
    #[inline]
    pub const fn disable_data_crc_err(self) -> Self {
        Self((self.0 & !Self::DATA_CRC_EROOR) | (Self::DATA_CRC_EROOR & (0 << 5)))
    }
    /// Check if Data CRC Error Signal is enabled.
    #[inline]
    pub const fn is_data_crc_err_enabled(self) -> bool {
        (self.0 & Self::DATA_CRC_EROOR) >> 5 == 1
    }
    /// Enable Data Timeout Error Signal.
    #[inline]
    pub const fn enable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (1 << 4)))
    }
    /// Disable Data Timeout Error Signal.
    #[inline]
    pub const fn disable_data_timeout_err(self) -> Self {
        Self((self.0 & !Self::DATA_TIMEOUE_ERROR) | (Self::DATA_TIMEOUE_ERROR & (0 << 4)))
    }
    /// Check if Data Timeout Error Signal is enabled.
    #[inline]
    pub const fn is_data_timeout_err_enabled(self) -> bool {
        (self.0 & Self::DATA_TIMEOUE_ERROR) >> 4 == 1
    }
    /// Enable Command Index Error Signal.
    #[inline]
    pub const fn enable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_EROOR) | (Self::CMD_INDEX_EROOR & (1 << 3)))
    }
    /// Disable Command Index Error Signal.
    #[inline]
    pub const fn disable_cmd_index_err(self) -> Self {
        Self((self.0 & !Self::CMD_INDEX_EROOR) | (Self::CMD_INDEX_EROOR & (0 << 3)))
    }
    /// Check if Command Index Error Signal is enabled.
    #[inline]
    pub const fn is_cmd_index_err_enabled(self) -> bool {
        (self.0 & Self::CMD_INDEX_EROOR) >> 3 == 1
    }
    /// Enable Command End Bit Error Signal.
    #[inline]
    pub const fn enable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (1 << 2)))
    }
    /// Disable Command End Bit Error Signal.
    #[inline]
    pub const fn disable_cmd_end_bit_err(self) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & (0 << 2)))
    }
    /// Check if Command End Bit Error Signal is enabled.
    #[inline]
    pub const fn is_cmd_end_bit_err_enabled(self) -> bool {
        (self.0 & Self::CMD_END_BIT_ERROR) >> 2 == 1
    }
    /// Enable Command CRC Error Signal.
    #[inline]
    pub const fn enable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (1 << 1)))
    }
    /// Disable Command CRC Error Signal.
    #[inline]
    pub const fn disable_cmd_crc_err(self) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & (0 << 1)))
    }
    /// Check if Command CRC Error Signal is enabled.
    #[inline]
    pub const fn is_cmd_crc_err_enabled(self) -> bool {
        (self.0 & Self::CMD_CRC_ERROR) >> 1 == 1
    }
    /// Enable Command Timeout Error Signal.
    #[inline]
    pub const fn enable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 1))
    }
    /// Disable Command Timeout Error Signal.
    #[inline]
    pub const fn disable_cmd_timeout_err(self) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & 0))
    }
    /// Check if Command Timeout Error Signal is enabled.
    #[inline]
    pub const fn is_cmd_timeout_err_enabled(self) -> bool {
        self.0 & Self::CMD_TIMEOUT_ERROR == 1
    }
}

/// Register that indicates CMD12 response error of Auto CMD12 and CMD23 response error of Auto CMD23.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AutoCMDErrorStatus(u16);

impl AutoCMDErrorStatus {
    const CMD_NOT_ISSUED: u16 = 0x1 << 7;
    const AUTO_CMD_INDEX_ERROR: u16 = 0x1 << 4;
    const AUTO_CMD_END_BIT_ERROR: u16 = 0x1 << 3;
    const AUTO_CMD_CRC_ERROR: u16 = 0x1 << 2;
    const AUTO_CMD_TIMEOUT_ERROR: u16 = 0x1 << 1;
    const AUTO_CMD_NOT_EXECUTED: u16 = 0x1;

    /// Check if Command is not issued by Auto CMD12.
    #[inline]
    pub const fn is_cmd_not_issued(self) -> bool {
        (self.0 & Self::CMD_NOT_ISSUED) >> 7 == 1
    }
    /// Check if Auto CMD Index Error occurs.
    #[inline]
    pub const fn if_auto_cmd_index_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_INDEX_ERROR) >> 4 == 1
    }
    /// Check if Auto CMD End Bit Error occurs.
    #[inline]
    pub const fn if_auto_cmd_end_bit_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_END_BIT_ERROR) >> 3 == 1
    }
    /// Check if Auto CMD CRC Error occurs.
    #[inline]
    pub const fn if_auto_cmd_crc_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_CRC_ERROR) >> 2 == 1
    }
    /// Check if Auto CMD Timeout Error occurs.
    #[inline]
    pub const fn if_auto_cmd_timeout_err_occurs(self) -> bool {
        (self.0 & Self::AUTO_CMD_TIMEOUT_ERROR) >> 1 == 1
    }
    /// Check if Auto CMD12 is not executed.
    #[inline]
    pub const fn is_auto_cmd12_not_executed(self) -> bool {
        self.0 & Self::AUTO_CMD_NOT_EXECUTED == 1
    }
}

/// Host Control 2 Register.
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

    /// Enable Preset Value.
    #[inline]
    pub const fn enable_preset_val(self) -> Self {
        Self((self.0 & !Self::PRESET_VAL_EN) | (Self::PRESET_VAL_EN & (1 << 15)))
    }
    // Disable Preset Value.
    #[inline]
    pub const fn disable_preset_val(self) -> Self {
        Self((self.0 & !Self::PRESET_VAL_EN) | (Self::PRESET_VAL_EN & (0 << 15)))
    }
    /// Check if Preset value is enabled.
    #[inline]
    pub const fn is_preset_val_enabled(self) -> bool {
        (self.0 & Self::PRESET_VAL_EN) >> 15 == 1
    }
    /// Enable Asynchronous Interrupt.
    #[inline]
    pub const fn enable_async_int(self) -> Self {
        Self((self.0 & !Self::ASYNCHRONOUS_INT_EN) | (Self::ASYNCHRONOUS_INT_EN & (1 << 14)))
    }
    // Disable Asynchronous Interrupt.
    #[inline]
    pub const fn disable_async_int(self) -> Self {
        Self((self.0 & !Self::ASYNCHRONOUS_INT_EN) | (Self::ASYNCHRONOUS_INT_EN & (0 << 14)))
    }
    /// Check if Asynchronous Interrupt is enabled.
    #[inline]
    pub const fn is_async_int_enabled(self) -> bool {
        (self.0 & Self::ASYNCHRONOUS_INT_EN) >> 14 == 1
    }
    /// Set Sampling Clock Select bit.
    /// Host Controller uses this bit to select sampling clock to receive CMD and DAT.
    /// Setting 1 means that tuning is completed successfully and setting 0 means that tuning is failed.
    #[inline]
    pub const fn set_sample_clk_select(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::SAMPLING_CLK_SELECT)
                | (Self::SAMPLING_CLK_SELECT & ((val as u16) << 7)),
        )
    }
    /// Get Sampling Clock Select bit.
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
    /// Set Driver Strength Select bit.
    #[inline]
    pub const fn set_driver_strength_select(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::DRIVER_STRENGTH_SELECT)
                | (Self::DRIVER_STRENGTH_SELECT & ((val as u16) << 4)),
        )
    }
    /// Get Driver Strength Select bit.
    #[inline]
    pub const fn driver_strength_select(self) -> u8 {
        ((self.0 & Self::DRIVER_STRENGTH_SELECT) >> 4) as u8
    }
    /// Change card signal voltage from 3.3V to 1.8V.
    #[inline]
    pub const fn change_3_3v_to_1_8v(self) -> Self {
        Self((self.0 & !Self::SIGNALING_1_8_V_EN) | (Self::SIGNALING_1_8_V_EN & (1 << 3)))
    }
    /// Check if changing card signal voltage from 3.3V to 1.8V is finished.
    #[inline]
    pub const fn is_3_3v_to_1_8v_finished(self) -> bool {
        (self.0 & Self::SIGNALING_1_8_V_EN) >> 3 == 1
    }
    /// Change card signal voltage from 1.8V to 3.3V.
    #[inline]
    pub const fn change_1_8v_to_3_3v(self) -> Self {
        Self((self.0 & !Self::SIGNALING_1_8_V_EN) | (Self::SIGNALING_1_8_V_EN & (0 << 3)))
    }
    /// Check if changing card signal voltage from 1.8V to 3.3V is finished.
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

/// Register that provides the Host Driver with information specific to the Host Controller implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Capabilities(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlotType {
    /// Removable Card Slot.
    RemovableCardSlot,
    /// Embedded Slot for One Device.
    EmbeddedSlotforOneDevice,
    /// Shared Bus Slot.
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
    // Capabilities_2
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

    /// Set slot type.
    #[inline]
    pub const fn set_slot_type(self, val: SlotType) -> Self {
        Self((self.0 & !Self::SLOT_TYPE) | (Self::SLOT_TYPE & ((val as u64) << 62)))
    }
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
    /// Enable Asynchronous Interrupt Support.
    #[inline]
    pub const fn enable_async_int(self) -> Self {
        Self((self.0 & !Self::ASYNCHRONOUS_INT) | (Self::ASYNCHRONOUS_INT & (1 << 61)))
    }
    /// Disable Asynchronous Interrupt Support.
    #[inline]
    pub const fn disable_async_int(self) -> Self {
        Self((self.0 & !Self::ASYNCHRONOUS_INT) | (Self::ASYNCHRONOUS_INT & (0 << 61)))
    }
    /// Check if Asynchronous Interrupt Support is enabled.
    #[inline]
    pub const fn is_async_int_enabled(self) -> bool {
        (self.0 & Self::ASYNCHRONOUS_INT) >> 61 == 1
    }
    /// Enable 64-bit System Bus Support.
    #[inline]
    pub const fn enable_64_bit_bus(self) -> Self {
        Self((self.0 & !Self::BIT_64_SYS_SUPPORT) | (Self::BIT_64_SYS_SUPPORT & (1 << 60)))
    }
    /// Disable 64-bit System Bus Support.
    #[inline]
    pub const fn disable_64_bit_bus(self) -> Self {
        Self((self.0 & !Self::BIT_64_SYS_SUPPORT) | (Self::BIT_64_SYS_SUPPORT & (0 << 60)))
    }
    /// Check if 64-bit System Bus Support is enabled.
    #[inline]
    pub const fn is_64_bit_bus_enabled(self) -> bool {
        (self.0 & Self::BIT_64_SYS_SUPPORT) >> 60 == 1
    }
    /// Enable Voltage Support 1.8V.
    #[inline]
    pub const fn enable_1_8v(self) -> Self {
        Self((self.0 & !Self::VOLTAGE_SUPPORT_1_8_V) | (Self::VOLTAGE_SUPPORT_1_8_V & (1 << 58)))
    }
    /// Disable Voltage Support 1.8V.
    #[inline]
    pub const fn disable_1_8v(self) -> Self {
        Self((self.0 & !Self::VOLTAGE_SUPPORT_1_8_V) | (Self::VOLTAGE_SUPPORT_1_8_V & (0 << 58)))
    }
    /// Check if Voltage Support 1.8V is enabled.
    #[inline]
    pub const fn is_1_8v_enabled(self) -> bool {
        (self.0 & Self::VOLTAGE_SUPPORT_1_8_V) >> 58 == 1
    }
    /// Enable Voltage Support 3.0V.
    #[inline]
    pub const fn enable_3_0v(self) -> Self {
        Self((self.0 & !Self::VOLTAGE_SUPPORT_3_0_V) | (Self::VOLTAGE_SUPPORT_3_0_V & (1 << 57)))
    }
    /// Disable Voltage Support 3.0V.
    #[inline]
    pub const fn disable_3_0v(self) -> Self {
        Self((self.0 & !Self::VOLTAGE_SUPPORT_3_0_V) | (Self::VOLTAGE_SUPPORT_3_0_V & (0 << 57)))
    }
    /// Check if Voltage Support 3.0V is enabled.
    #[inline]
    pub const fn is_3_0v_enabled(self) -> bool {
        (self.0 & Self::VOLTAGE_SUPPORT_3_0_V) >> 57 == 1
    }
    /// Enable Voltage Support 3.3V.
    #[inline]
    pub const fn enable_3_3v(self) -> Self {
        Self((self.0 & !Self::VOLTAGE_SUPPORT_3_3_V) | (Self::VOLTAGE_SUPPORT_3_3_V & (1 << 56)))
    }
    /// Disable Voltage Support 3.3V.
    #[inline]
    pub const fn disable_3_3v(self) -> Self {
        Self((self.0 & !Self::VOLTAGE_SUPPORT_3_3_V) | (Self::VOLTAGE_SUPPORT_3_3_V & (0 << 56)))
    }
    /// Check if Voltage Support 3.3V is enabled.
    #[inline]
    pub const fn is_3_3v_enabled(self) -> bool {
        (self.0 & Self::VOLTAGE_SUPPORT_3_3_V) >> 56 == 1
    }
    /// Enable Suspend/Resume Support.
    #[inline]
    pub const fn enable_suspend_resume(self) -> Self {
        Self((self.0 & !Self::SUSPEND_RESUME_SUPPORT) | (Self::SUSPEND_RESUME_SUPPORT & (1 << 55)))
    }
    /// Disable Suspend/Resume Support.
    #[inline]
    pub const fn disable_suspend_resume(self) -> Self {
        Self((self.0 & !Self::SUSPEND_RESUME_SUPPORT) | (Self::SUSPEND_RESUME_SUPPORT & (0 << 5)))
    }
    /// Check if Suspend/Resume Support is enabled.
    #[inline]
    pub const fn is_suspend_resume_enabled(self) -> bool {
        (self.0 & Self::SUSPEND_RESUME_SUPPORT) >> 55 == 1
    }
    /// Enable SDMA Support.
    #[inline]
    pub const fn enable_sdma(self) -> Self {
        Self((self.0 & !Self::SDMA_SUPOORT) | (Self::SDMA_SUPOORT & (1 << 54)))
    }
    /// Disable SDMA Support.
    #[inline]
    pub const fn disable_sdma(self) -> Self {
        Self((self.0 & !Self::SDMA_SUPOORT) | (Self::SDMA_SUPOORT & (0 << 54)))
    }
    /// Check if SDMA Support is enabled.
    #[inline]
    pub const fn is_sdma_enabled(self) -> bool {
        (self.0 & Self::SDMA_SUPOORT) >> 54 == 1
    }
    /// Enable High Speed Support.
    #[inline]
    pub const fn enable_high_speed(self) -> Self {
        Self((self.0 & !Self::HIGH_SPEED_SUPPORT) | (Self::HIGH_SPEED_SUPPORT & (1 << 53)))
    }
    /// Disable High Speed Support.
    #[inline]
    pub const fn disable_high_speed(self) -> Self {
        Self((self.0 & !Self::HIGH_SPEED_SUPPORT) | (Self::HIGH_SPEED_SUPPORT & (0 << 53)))
    }
    /// Check if 64-bit High Speed Support is enabled.
    #[inline]
    pub const fn is_high_speed_enabled(self) -> bool {
        (self.0 & Self::HIGH_SPEED_SUPPORT) >> 53 == 1
    }
    /// Enable ADMA2 Support.
    #[inline]
    pub const fn enable_adma2(self) -> Self {
        Self((self.0 & !Self::ADMA2_SUPPORT) | (Self::ADMA2_SUPPORT & (1 << 51)))
    }
    /// Disable ADMA2 Support.
    #[inline]
    pub const fn disable_adma2(self) -> Self {
        Self((self.0 & !Self::ADMA2_SUPPORT) | (Self::ADMA2_SUPPORT & (0 << 51)))
    }
    /// Check if ADMA2 Support is enabled.
    #[inline]
    pub const fn is_adma2_enabled(self) -> bool {
        (self.0 & Self::ADMA2_SUPPORT) >> 51 == 1
    }
    /// Enable 8-bit Bus Support for Embedded Device.
    #[inline]
    pub const fn enable_8_bit_bus(self) -> Self {
        Self((self.0 & !Self::BIT_8_SUPPORT) | (Self::BIT_8_SUPPORT & (1 << 50)))
    }
    /// Disable 8-bit Bus Support for Embedded Device.
    #[inline]
    pub const fn disable_8_bit_bus(self) -> Self {
        Self((self.0 & !Self::BIT_8_SUPPORT) | (Self::BIT_8_SUPPORT & (0 << 50)))
    }
    /// Check if 8-bit Bus Support for Embedded Device.
    #[inline]
    pub const fn is_8_bit_bus_enabled(self) -> bool {
        (self.0 & Self::BIT_8_SUPPORT) >> 50 == 1
    }
    /// Set Max Block Length.
    #[inline]
    pub const fn set_max_block_len(self, val: u8) -> Self {
        Self((self.0 & !Self::MAX_BLOCK_LEN) | (Self::MAX_BLOCK_LEN & ((val as u64) << 48)))
    }
    /// Get Max Block Length.
    #[inline]
    pub const fn max_block_len(self) -> u8 {
        ((self.0 & Self::MAX_BLOCK_LEN) >> 48) as u8
    }
    /// Set Base Clock Frequency For SD Clock.
    #[inline]
    pub const fn set_base_clk(self, val: u8) -> Self {
        Self((self.0 & !Self::BASE_CLK_FREQ) | (Self::BASE_CLK_FREQ & ((val as u64) << 40)))
    }
    /// Get Base Clock Frequency For SD Clock.
    #[inline]
    pub const fn base_clk(self) -> u8 {
        ((self.0 & Self::BASE_CLK_FREQ) >> 40) as u8
    }
    /// Set Timeout Clock Unit.
    #[inline]
    pub const fn set_timeout_clk_unit(self, val: u8) -> Self {
        Self((self.0 & !Self::TIMEOUT_CLK_UNIT) | (Self::TIMEOUT_CLK_UNIT & ((val as u64) << 39)))
    }
    /// Get Timeout Clock Unit.
    #[inline]
    pub const fn timeout_clk_unit(self) -> u8 {
        ((self.0 & Self::TIMEOUT_CLK_UNIT) >> 39) as u8
    }
    /// Set Timeout Clock Frequency.
    #[inline]
    pub const fn set_timeout_clk_freq(self, val: u8) -> Self {
        Self((self.0 & !Self::TIMEOUT_CLK_FREQ) | (Self::TIMEOUT_CLK_FREQ & ((val as u64) << 32)))
    }
    /// Get Timeout Clock Frequency.
    #[inline]
    pub const fn timeout_clk_freq(self) -> u8 {
        ((self.0 & Self::TIMEOUT_CLK_FREQ) >> 32) as u8
    }
    /// Set Clock Multiplier.
    #[inline]
    pub const fn set_clk_multiplier(self, val: u8) -> Self {
        Self((self.0 & !Self::CLK_MULTIPLIER) | (Self::CLK_MULTIPLIER & ((val as u64) << 16)))
    }
    /// Get Clock Multiplier.
    #[inline]
    pub const fn clk_multiplier(self) -> u8 {
        ((self.0 & Self::CLK_MULTIPLIER) >> 16) as u8
    }
    /// Set Re-Tuning Modes.
    #[inline]
    pub const fn set_re_tuning_modes(self, val: u8) -> Self {
        Self((self.0 & !Self::RE_TUNING_MODES) | (Self::RE_TUNING_MODES & ((val as u64) << 14)))
    }
    /// Get Re-Tuning Modes.
    #[inline]
    pub const fn re_tuning_modes(self) -> u8 {
        ((self.0 & Self::RE_TUNING_MODES) >> 14) as u8
    }
    /// Enable Use Tuning for SDR50.
    #[inline]
    pub const fn enable_tuning_for_sdr50(self) -> Self {
        Self((self.0 & !Self::USE_TUNING) | (Self::USE_TUNING & (1 << 13)))
    }
    /// Disable Use Tuning for SDR50.
    #[inline]
    pub const fn disable_tuning_for_sdr50(self) -> Self {
        Self((self.0 & !Self::USE_TUNING) | (Self::USE_TUNING & (0 << 13)))
    }
    /// Check if Use Tuning for SDR50 is enabled.
    #[inline]
    pub const fn is_tuning_for_sdr50_enabled(self) -> bool {
        (self.0 & Self::USE_TUNING) >> 13 == 1
    }
    /// Set Timer Count for Re-Tuning.
    #[inline]
    pub const fn set_tim_cnt_for_re_tuning(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::TIM_CNT_FOR_RETUNING)
                | (Self::TIM_CNT_FOR_RETUNING & ((val as u64) << 8)),
        )
    }
    /// Get Timer Count for Re-Tuning.
    #[inline]
    pub const fn tim_cnt_for_re_tuning(self) -> u8 {
        ((self.0 & Self::TIM_CNT_FOR_RETUNING) >> 8) as u8
    }
    /// Enable Driver Type D Support.
    #[inline]
    pub const fn enable_driver_type_d(self) -> Self {
        Self((self.0 & !Self::DRIVER_TYPE_D_SUPPORT) | (Self::DRIVER_TYPE_D_SUPPORT & (1 << 6)))
    }
    /// Disable Driver Type D Support.
    #[inline]
    pub const fn disable_driver_type_d(self) -> Self {
        Self((self.0 & !Self::DRIVER_TYPE_D_SUPPORT) | (Self::DRIVER_TYPE_D_SUPPORT & (0 << 6)))
    }
    /// Check if Driver Type D Support is enabled.
    #[inline]
    pub const fn is_driver_type_d_enabled(self) -> bool {
        (self.0 & Self::DRIVER_TYPE_D_SUPPORT) >> 6 == 1
    }
    /// Enable Driver Type C Support.
    #[inline]
    pub const fn enable_driver_type_c(self) -> Self {
        Self((self.0 & !Self::DRIVER_TYPE_C_SUPPORT) | (Self::DRIVER_TYPE_C_SUPPORT & (1 << 5)))
    }
    /// Disable Driver Type C Support.
    #[inline]
    pub const fn disable_driver_type_c(self) -> Self {
        Self((self.0 & !Self::DRIVER_TYPE_C_SUPPORT) | (Self::DRIVER_TYPE_C_SUPPORT & (0 << 5)))
    }
    /// Check if Driver Type C Support is enabled.
    #[inline]
    pub const fn is_driver_type_c_enabled(self) -> bool {
        (self.0 & Self::DRIVER_TYPE_C_SUPPORT) >> 5 == 1
    }
    /// Enable Driver Type A Support.
    #[inline]
    pub const fn enable_driver_type_a(self) -> Self {
        Self((self.0 & !Self::DRIVER_TYPE_A_SUPPORT) | (Self::DRIVER_TYPE_A_SUPPORT & (1 << 4)))
    }
    /// Disable Driver Type A Support.
    #[inline]
    pub const fn disable_driver_type_a(self) -> Self {
        Self((self.0 & !Self::DRIVER_TYPE_A_SUPPORT) | (Self::DRIVER_TYPE_A_SUPPORT & (0 << 4)))
    }
    /// Check if Driver Type A Support is enabled.
    #[inline]
    pub const fn is_driver_type_a_enabled(self) -> bool {
        (self.0 & Self::DRIVER_TYPE_A_SUPPORT) >> 4 == 1
    }
    /// Enable DDR50 Support.
    #[inline]
    pub const fn enable_ddr50(self) -> Self {
        Self((self.0 & !Self::DDR50_SUPPORT) | (Self::DDR50_SUPPORT & (1 << 2)))
    }
    /// Disable DDR50 Support.
    #[inline]
    pub const fn disable_ddr50(self) -> Self {
        Self((self.0 & !Self::DDR50_SUPPORT) | (Self::DDR50_SUPPORT & (0 << 2)))
    }
    /// Check if DDR50 Support is enabled.
    #[inline]
    pub const fn is_ddr50_enabled(self) -> bool {
        (self.0 & Self::DDR50_SUPPORT) >> 2 == 1
    }
    /// Enable SDR104 Support.
    /// SDR104 requires tuning.
    #[inline]
    pub const fn enable_sdr104(self) -> Self {
        Self((self.0 & !Self::SDR104_SUPPORT) | (Self::SDR104_SUPPORT & (1 << 1)))
    }
    /// Disable Driver Type A Support.
    #[inline]
    pub const fn disable_sdr104(self) -> Self {
        Self((self.0 & !Self::SDR104_SUPPORT) | (Self::SDR104_SUPPORT & (0 << 1)))
    }
    /// Check if Driver Type A Support is enabled.
    #[inline]
    pub const fn is_sdr104_enabled(self) -> bool {
        (self.0 & Self::SDR104_SUPPORT) >> 1 == 1
    }
    /// Enable SDR50 Support.
    #[inline]
    pub const fn enable_sdr50(self) -> Self {
        Self((self.0 & !Self::SDR50_SUPPORT) | (Self::SDR50_SUPPORT & 1))
    }
    /// Disable SDR50 Support.
    #[inline]
    pub const fn disable_sdr50(self) -> Self {
        Self((self.0 & !Self::SDR50_SUPPORT) | (Self::SDR50_SUPPORT & 0))
    }
    /// Check if SDR50 Support is enabled.
    #[inline]
    pub const fn is_sdr50_enabled(self) -> bool {
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

    /// Set Maximum Current for 1.8V.
    #[inline]
    pub const fn set_max_current_1_8v(self, val: u8) -> Self {
        Self((self.0 & !Self::MAX_CURRENT_1_8_V) | (Self::MAX_CURRENT_1_8_V & ((val as u64) << 16)))
    }
    /// Get Maximum Current for 1.8V.
    #[inline]
    pub const fn max_current_1_8v(self) -> u8 {
        ((self.0 & Self::MAX_CURRENT_1_8_V) >> 16) as u8
    }
    /// Set Maximum Current for 3.0V.
    #[inline]
    pub const fn set_max_current_3_0v(self, val: u8) -> Self {
        Self((self.0 & !Self::MAX_CURRENT_3_0_V) | (Self::MAX_CURRENT_3_0_V & ((val as u64) << 8)))
    }
    /// Get Maximum Current for 3.0V.
    #[inline]
    pub const fn max_current_3_0v(self) -> u8 {
        ((self.0 & Self::MAX_CURRENT_3_0_V) >> 8) as u8
    }
    /// Set Maximum Current for 3.3V.
    #[inline]
    pub const fn set_max_current_3_3v(self, val: u8) -> Self {
        Self((self.0 & !Self::MAX_CURRENT_3_3_V) | (Self::MAX_CURRENT_3_3_V & (val as u64)))
    }
    /// Get Maximum Current for 3.3V.
    #[inline]
    pub const fn max_current_3_3v(self) -> u8 {
        (self.0 & Self::MAX_CURRENT_3_3_V) as u8
    }
}

/// Register that simplifies test of the Auto CMD Error Status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ForceEventAutoCMDErrorStatus(u16);

impl ForceEventAutoCMDErrorStatus {
    const CMD_NOT_ISSUED: u16 = 0x1 << 7;
    const AUTO_CMD_INDEX_ERROR: u16 = 0x1 << 4;
    const AUTO_CMD_END_BIT_ERROR: u16 = 0x1 << 3;
    const AUTO_CMD_CRC_ERROR: u16 = 0x1 << 2;
    const AUTO_CMD_TIMEOUT_ERROR: u16 = 0x1 << 1;
    const AUTO_CMD_NOT_EXECUTED: u16 = 0x1;

    /// Set Force Event for Command Not Issued By Auto CMD12 Error bit.
    #[inline]
    pub const fn set_cmd_not_issued(self, val: u16) -> Self {
        Self((self.0 & !Self::CMD_NOT_ISSUED) | (Self::CMD_NOT_ISSUED & (val << 7)))
    }
    /// Set Force Event for Auto CMD Index Error bit.
    #[inline]
    pub const fn set_auto_cmd_index(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_INDEX_ERROR) | (Self::AUTO_CMD_INDEX_ERROR & (val << 4)))
    }
    /// Set Force Event for Auto CMD End Bit Error bit.
    #[inline]
    pub const fn set_auto_cmd_end_bit(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_END_BIT_ERROR) | (Self::AUTO_CMD_END_BIT_ERROR & (val << 3)))
    }
    /// Set Force Event for Auto CMD CRC Error bit.
    #[inline]
    pub const fn set_auto_cmd_crc(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_CRC_ERROR) | (Self::AUTO_CMD_CRC_ERROR & (val << 2)))
    }
    /// Set Force Event for Auto CMD Timeout Error bit.
    #[inline]
    pub const fn set_auto_cmd_timeout(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_TIMEOUT_ERROR) | (Self::AUTO_CMD_TIMEOUT_ERROR & (val << 1)))
    }
    /// Set Force Event for Auto CMD12 Not Executed bit.
    #[inline]
    pub const fn set_auto_cmd12_not_executed(self, val: u16) -> Self {
        Self((self.0 & !Self::AUTO_CMD_NOT_EXECUTED) | (Self::AUTO_CMD_NOT_EXECUTED & val))
    }
}

/// Register that simplifies test of the Error Interrupt Status register.
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

    /// Set Force Event for Vendor Specific Error Status bit.
    #[inline]
    pub const fn set_vendor_specific_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::VENDOR_SPECIFIC_ERROR)
                | (Self::VENDOR_SPECIFIC_ERROR & ((val as u16) << 12)),
        )
    }
    /// Set Force Event for ADMA Error bit.
    #[inline]
    pub const fn set_adma_err(self, val: u8) -> Self {
        Self((self.0 & !Self::ADMA_EROR) | (Self::ADMA_EROR & ((val as u16) << 9)))
    }
    /// Set Force Event for Auto CMD Error bit.
    #[inline]
    pub const fn set_auto_cmd_err(self, val: u8) -> Self {
        Self((self.0 & !Self::AUTO_CMD_ERROR) | (Self::AUTO_CMD_ERROR & ((val as u16) << 8)))
    }
    /// Set Force Event for Current Limit Error bit.
    #[inline]
    pub const fn set_current_limit_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::CURRENT_LIMIT_ERROR)
                | (Self::CURRENT_LIMIT_ERROR & ((val as u16) << 7)),
        )
    }
    /// Set Force Event for Data End Bit Error bit.
    #[inline]
    pub const fn set_data_end_bit_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::DATA_END_BIT_ERROR) | (Self::DATA_END_BIT_ERROR & ((val as u16) << 6)),
        )
    }
    /// Set Force Event for Data CRC Error bit.
    #[inline]
    pub const fn set_data_crc_err(self, val: u8) -> Self {
        Self((self.0 & !Self::DATA_CRC_ERROR) | (Self::DATA_CRC_ERROR & ((val as u16) << 5)))
    }
    /// Set Force Event for Data Timeout Error bit.
    #[inline]
    pub const fn set_data_timeout_err(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::DATA_TIMEOUT_ERROR) | (Self::DATA_TIMEOUT_ERROR & ((val as u16) << 4)),
        )
    }
    /// Set Force Event for Command Index Error bit.
    #[inline]
    pub const fn set_cmd_index_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_INDEX_ERROR) | (Self::CMD_INDEX_ERROR & ((val as u16) << 3)))
    }
    /// Set Force Event for Command End Bit Error bit.
    #[inline]
    pub const fn set_cmd_end_bit_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_END_BIT_ERROR) | (Self::CMD_END_BIT_ERROR & ((val as u16) << 2)))
    }
    /// Set Force Event for Command CRC Error bit.
    #[inline]
    pub const fn set_cmd_crc_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_CRC_ERROR) | (Self::CMD_CRC_ERROR & ((val as u16) << 1)))
    }
    /// Set Force Event for Command Timeout Error bit.
    #[inline]
    pub const fn set_cmd_timeout_err(self, val: u8) -> Self {
        Self((self.0 & !Self::CMD_TIMEOUT_ERROR) | (Self::CMD_TIMEOUT_ERROR & (val as u16)))
    }
}

/// Register that holds the ADMA state when ADMA Error Interrupt is occurred.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMAErrorStatus(u32);

impl ADMAErrorStatus {
    const ADMA_LEN_MISMATCH: u32 = 0x1 << 2;
    const ADMA_ERROR_STATE: u32 = 0x3;

    /// Check if ADMA Length Mismatch Error occurs.
    #[inline]
    pub const fn if_adma_len_mismatch_err_occurs(self) -> bool {
        (self.0 & Self::ADMA_LEN_MISMATCH) >> 2 == 1
    }
    /// Get ADMA Error State.
    #[inline]
    pub const fn adma_err_state(self) -> u8 {
        (self.0 & Self::ADMA_ERROR_STATE) as u8
    }
}

/// Register that contains the physical Descriptor address used for ADMA data transfer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMASystemAddress(u64);

impl ADMASystemAddress {
    const ADMA_SYSTEM_ADDRESS: u64 = 0xFFFF_FFFF_FFFF_FFFF;

    /// Set ADMA System Address.
    #[inline]
    pub const fn set_adma_sys_addr(self, val: u64) -> Self {
        Self((self.0 & !Self::ADMA_SYSTEM_ADDRESS) | (Self::ADMA_SYSTEM_ADDRESS & val))
    }
    /// Get ADMA System Address.
    #[inline]
    pub const fn adma_sys_addr(self) -> u64 {
        self.0 & Self::ADMA_SYSTEM_ADDRESS
    }
}

/// Preset Value Registers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PresetValue(u128);

impl PresetValue {
    // Preset Value Register for DDR50.
    const DDR50_DRV_STRENGTH_VAL: u128 = 0x3 << 126;
    const DDR50_CLKGEN_SEL_VAL: u128 = 0x1 << 122;
    const DDR50_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 112;
    // Preset Value Register for SDR104.
    const SDR104_DRV_STRENGTH_VAL: u128 = 0x3 << 110;
    const SDR104_CLKGEN_SEL_VAL: u128 = 0x1 << 106;
    const SDR104_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 96;
    // Preset Value Register for SDR50.
    const SDR50_DRV_STRENGTH_VAL: u128 = 0x3 << 94;
    const SDR50_CLKGEN_SEL_VAL: u128 = 0x1 << 90;
    const SDR50_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 80;
    // Preset Value Register for SDR25.
    const SDR25_DRV_STRENGTH_VAL: u128 = 0x3 << 78;
    const SDR25_CLKGEN_SEL_VAL: u128 = 0x1 << 74;
    const SDR25_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 64;
    // Preset Value Register for SDR12.
    const SDR12_DRV_STRENGTH_VAL: u128 = 0x3 << 62;
    const SDR12_CLKGEN_SEL_VAL: u128 = 0x1 << 58;
    const SDR12_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 48;
    // Preset Value Register for High Speed.
    const HS_DRV_STRENGTH_VAL: u128 = 0x3 << 46;
    const HS_CLKGEN_SEL_VAL: u128 = 0x1 << 42;
    const HS_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 32;
    // Preset Value Register for Default Speed.
    const DEFAULT_DRV_STRENGTH_VAL: u128 = 0x3 << 30;
    const DEFAULT_CLKGEN_SEL_VAL: u128 = 0x1 << 26;
    const DEFAULT_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF << 16;
    // Preset Value Register for Initialization.
    const INIT_DRV_STRENGTH_VAL: u128 = 0x3 << 14;
    const INIT_CLKGEN_SEL_VAL: u128 = 0x1 << 10;
    const INIT_SDCLK_FREQ_SEL_VAL: u128 = 0x3FF;

    /// Get Driver Strength Value For DDR50.
    #[inline]
    pub const fn ddr50_drv_strength_val(self) -> u16 {
        ((self.0 & Self::DDR50_DRV_STRENGTH_VAL) >> 126) as u16
    }
    /// Get Clock Generator Frequency Select Value For DDR50.
    #[inline]
    pub const fn ddr50_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::DDR50_CLKGEN_SEL_VAL) >> 122) as u16
    }
    /// Get SD Clock Generator Frequency Select Value For DDR50.
    #[inline]
    pub const fn ddr50_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::DDR50_SDCLK_FREQ_SEL_VAL) >> 112) as u16
    }

    // Get Driver Strength Value For SDR104.
    #[inline]
    pub const fn sdr104_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR104_DRV_STRENGTH_VAL) >> 110) as u16
    }
    // Get Clock Generator Frequency Select Value For SDR104.
    #[inline]
    pub const fn sdr104_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR104_CLKGEN_SEL_VAL) >> 106) as u16
    }
    // Get SD Clock Generator Frequency Select Value For SDR104.
    #[inline]
    pub const fn sdr104_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR104_SDCLK_FREQ_SEL_VAL) >> 96) as u16
    }

    // Get Driver Strength Value For SDR50.
    #[inline]
    pub const fn sdr50_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR50_DRV_STRENGTH_VAL) >> 94) as u16
    }
    // Get Clock Generator Frequency Select Value For SDR50.
    #[inline]
    pub const fn sdr50_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR50_CLKGEN_SEL_VAL) >> 90) as u16
    }
    // Get SD Clock Generator Frequency Select Value For SDR50.
    #[inline]
    pub const fn sdr50_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR50_SDCLK_FREQ_SEL_VAL) >> 80) as u16
    }

    // Get Driver Strength Value For SDR25.
    #[inline]
    pub const fn sdr25_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR25_DRV_STRENGTH_VAL) >> 78) as u16
    }
    // Get Clock Generator Frequency Select Value For SDR25.
    #[inline]
    pub const fn sdr25_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR25_CLKGEN_SEL_VAL) >> 74) as u16
    }
    // Get SD Clock Generator Frequency Select Value For SDR25.
    #[inline]
    pub const fn sdr25_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR25_SDCLK_FREQ_SEL_VAL) >> 64) as u16
    }

    // Get Driver Strength Value For SDR12.
    #[inline]
    pub const fn sdr12_drv_strength_val(self) -> u16 {
        ((self.0 & Self::SDR12_DRV_STRENGTH_VAL) >> 62) as u16
    }
    // Get Clock Generator Frequency Select Value For SDR12.
    #[inline]
    pub const fn sdr12_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::SDR12_CLKGEN_SEL_VAL) >> 58) as u16
    }
    // Get SD Clock Generator Frequency Select Value For SDR12.
    #[inline]
    pub const fn sdr12_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::SDR12_SDCLK_FREQ_SEL_VAL) >> 48) as u16
    }

    // Get Driver Strength Value For High Speed.
    #[inline]
    pub const fn hs_drv_strength_val(self) -> u16 {
        ((self.0 & Self::HS_DRV_STRENGTH_VAL) >> 46) as u16
    }
    // Get Clock Generator Frequency Select Value For High Speed.
    #[inline]
    pub const fn hs_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::HS_CLKGEN_SEL_VAL) >> 42) as u16
    }
    // Get SD Clock Generator Frequency Select Value For High Speed.
    #[inline]
    pub const fn hs_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::HS_SDCLK_FREQ_SEL_VAL) >> 32) as u16
    }

    // Get Driver Strength Value For Default Speed.
    #[inline]
    pub const fn default_drv_strength_val(self) -> u16 {
        ((self.0 & Self::DEFAULT_DRV_STRENGTH_VAL) >> 30) as u16
    }
    // Get Clock Generator Frequency Select Value For Default Speed.
    #[inline]
    pub const fn default_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::DEFAULT_CLKGEN_SEL_VAL) >> 26) as u16
    }
    // Get SD Clock Generator Frequency Select Value For Default Speed.
    #[inline]
    pub const fn default_sdclk_freq_clk_val(self) -> u16 {
        ((self.0 & Self::DEFAULT_SDCLK_FREQ_SEL_VAL) >> 16) as u16
    }

    // Get Driver Strength Value For Initialization.
    #[inline]
    pub const fn init_drv_strength_val(self) -> u16 {
        ((self.0 & Self::INIT_DRV_STRENGTH_VAL) >> 14) as u16
    }
    // Get Clock Generator Frequency Select Value For Initialization.
    #[inline]
    pub const fn init_clkgen_sel_val(self) -> u16 {
        ((self.0 & Self::INIT_CLKGEN_SEL_VAL) >> 10) as u16
    }
    // Get SD Clock Generator Frequency Select Value For Initialization.
    #[inline]
    pub const fn init_sdclk_freq_clk_val(self) -> u16 {
        (self.0 & Self::INIT_SDCLK_FREQ_SEL_VAL) as u16
    }
}

/// ADMA3 Intergrated Descriptor Address Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMA3IntegratedDescriptorAddress(u64);

impl ADMA3IntegratedDescriptorAddress {
    // TODO
}

/// Shared Bus Control Register.
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

    /// Set Back-End Power Control.
    #[inline]
    pub const fn set_back_end_pwr_ctrl(self, val: u8) -> Self {
        Self((self.0 & !Self::BACK_END_CTRL) | (Self::BACK_END_CTRL & ((val as u32) << 24)))
    }
    /// Get Back-End Power Control.
    #[inline]
    pub const fn back_end_pwr_ctrl(self) -> u8 {
        ((self.0 & Self::BACK_END_CTRL) >> 24) as u8
    }
    /// Set Interrupt Pin Select.
    #[inline]
    pub const fn set_int_pin_sel(self, val: u8) -> Self {
        Self((self.0 & !Self::INT_PIN_SEL) | (Self::INT_PIN_SEL & ((val as u32) << 20)))
    }
    /// Get Interrupt Pin Select.
    #[inline]
    pub const fn int_pin_sel(self) -> u8 {
        ((self.0 & Self::INT_PIN_SEL) >> 20) as u8
    }
    /// Set Clock Pin Select.
    #[inline]
    pub const fn set_clk_pin_sel(self, val: u8) -> Self {
        Self((self.0 & !Self::CLK_PIN_SEL) | (Self::CLK_PIN_SEL & ((val as u32) << 16)))
    }
    /// Get Clock Pin Select.
    #[inline]
    pub const fn clk_pin_sel(self) -> u8 {
        ((self.0 & Self::CLK_PIN_SEL) >> 16) as u8
    }
    /// Set Bus Width Preset.
    #[inline]
    pub const fn set_bus_width_preset(self, val: u8) -> Self {
        Self((self.0 & !Self::BUS_WIDTH_PRESET) | (Self::BUS_WIDTH_PRESET & ((val as u32) << 8)))
    }
    /// Get Bus Width Preset.
    #[inline]
    pub const fn bus_width_preset(self) -> u8 {
        ((self.0 & Self::BUS_WIDTH_PRESET) >> 8) as u8
    }
    /// Set Number of Interrupt Input Pins.
    #[inline]
    pub const fn set_int_input_pin_num(self, val: u8) -> Self {
        Self(
            (self.0 & !Self::NUM_OF_INT_INPUT_PINS)
                | (Self::NUM_OF_INT_INPUT_PINS & ((val as u32) << 4)),
        )
    }
    /// Get Number of Interrupt Input Pins.
    #[inline]
    pub const fn int_input_pin_num(self) -> u8 {
        ((self.0 & Self::NUM_OF_INT_INPUT_PINS) >> 4) as u8
    }
    /// Set Number of Clock Pins.
    #[inline]
    pub const fn set_clk_pin_num(self, val: u8) -> Self {
        Self((self.0 & !Self::NUM_OF_CLK_PINS) | (Self::NUM_OF_CLK_PINS & (val as u32)))
    }
    /// Get Number of Clock Pins.
    #[inline]
    pub const fn clk_pin_num(self) -> u8 {
        (self.0 & Self::NUM_OF_CLK_PINS) as u8
    }
}

/// Slot Interrupt Status Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SlotInterruptStatus(u16);

impl SlotInterruptStatus {
    const INT_SIGNAL: u16 = 0xFF;

    /// Get Interrupt Signal For Each Slot.
    #[inline]
    pub const fn int_signal(self) -> u16 {
        self.0 & Self::INT_SIGNAL
    }
}

/// Host Controller Version Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControllerVersion(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecificVersion {
    /// SD Host Specification Version 1.00.
    SDHostSpecificVersion1,
    /// SD Host Specification Version 2.00.
    /// Including the feature of the ADMA and Test Register.
    SDHostSpecificVersion2,
    /// SD Host Specification Version 3.00.
    SDHostSpecificVersion3,
}

impl HostControllerVersion {
    const VENDOR_VERSION: u16 = 0xFF << 8;
    const SPECIFIC_VERION: u16 = 0xFF;

    /// Set Vendor Version Number.
    /// This status is reserved for the vendor version number.
    /// The Host Driver should not use this status.
    #[inline]
    pub const fn set_vendor_version(self, val: u8) -> Self {
        Self((self.0 & !Self::VENDOR_VERSION) | (Self::VENDOR_VERSION & ((val as u16) << 8)))
    }
    /// Get Vendor Version Number.
    #[inline]
    pub const fn vendor_version(self) -> u8 {
        ((self.0 & Self::VENDOR_VERSION) >> 8) as u8
    }
    /// Set Specification Version.
    #[inline]
    pub const fn set_specific_version(self, val: SpecificVersion) -> Self {
        Self((self.0 & !Self::SPECIFIC_VERION) | (Self::SPECIFIC_VERION & (val as u16)))
    }
    /// Get Specification Version.
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

/// SD Extra Parameters Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SDExtraParameters(u32);

impl SDExtraParameters {
    const GEN_PAD_CLK_CNT: u32 = 0xF << 24;
    const GEN_PAD_CLK_ON: u32 = 0x1 << 6;
    const SQU_FULL_CHK: u32 = 0x1 << 5;
    const SQU_EMPTY_CHK: u32 = 0x1 << 4;
    const BOOT_ACK: u32 = 0x1 << 3;

    /// Set Generator Pad Clock Counter.
    #[inline]
    pub const fn set_gen_clk_cnt(self, val: u8) -> Self {
        Self((self.0 & !Self::GEN_PAD_CLK_CNT) | (Self::GEN_PAD_CLK_CNT & ((val as u32) << 24)))
    }
    /// Get Generator Pad Clock Counter.
    #[inline]
    pub const fn gen_clk_cnt(self) -> u8 {
        ((self.0 & Self::GEN_PAD_CLK_CNT) >> 24) as u8
    }
    /// Set Generator Pad Clock On bit.
    #[inline]
    pub const fn set_gen_clk(self) -> Self {
        Self((self.0 & !Self::GEN_PAD_CLK_ON) | (Self::GEN_PAD_CLK_ON & (1 << 6)))
    }
    /// Unset Generator Pad Clock On bit.
    #[inline]
    pub const fn unset_gen_clk(self) -> Self {
        Self((self.0 & !Self::GEN_PAD_CLK_ON) | (Self::GEN_PAD_CLK_ON & (0 << 6)))
    }
    /// Check if Generator Pad Clock is on.
    #[inline]
    pub const fn is_gen_clk_on(self) -> bool {
        (self.0 & Self::GEN_PAD_CLK_ON) >> 6 == 1
    }
    /// Set SQU Full Check bit.
    #[inline]
    pub const fn set_squ_full(self) -> Self {
        Self((self.0 & !Self::SQU_FULL_CHK) | (Self::SQU_FULL_CHK & (1 << 5)))
    }
    /// Unset SQU Full Check bit.
    #[inline]
    pub const fn unset_squ_full(self) -> Self {
        Self((self.0 & !Self::SQU_FULL_CHK) | (Self::SQU_FULL_CHK & (0 << 5)))
    }
    /// Check if SQU is full.
    #[inline]
    pub const fn is_squ_full(self) -> bool {
        (self.0 & Self::SQU_FULL_CHK) >> 5 == 1
    }
    /// Set SQU Empty Check bit.
    #[inline]
    pub const fn set_squ_empty(self) -> Self {
        Self((self.0 & !Self::SQU_EMPTY_CHK) | (Self::SQU_EMPTY_CHK & (1 << 4)))
    }
    /// Unset SQU Empty Check bit.
    #[inline]
    pub const fn unset_squ_empty(self) -> Self {
        Self((self.0 & !Self::SQU_EMPTY_CHK) | (Self::SQU_EMPTY_CHK & (0 << 4)))
    }
    /// Check if SQU is Empty.
    #[inline]
    pub const fn is_squ_empty(self) -> bool {
        (self.0 & Self::SQU_EMPTY_CHK) >> 4 == 1
    }
    /// Set Boot Ack bit.
    #[inline]
    pub const fn set_boot_ack(self) -> Self {
        Self((self.0 & !Self::BOOT_ACK) | (Self::BOOT_ACK & (1 << 3)))
    }
    /// Unset Boot Ack bit.
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

/// FIFO Parameters Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FIFOParameters(u32);

impl FIFOParameters {
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

/// SPI Mode Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SPIMode(u16);

impl SPIMode {
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

/// Clock and Burst Size Setup Register.
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
    const _DMA_SIZE: u16 = 0x3 << 2;
    const _BURST_SIZE: u16 = 0x3;

    // TODO
}

/// CE-ATA Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CEATA(u32);

impl CEATA {
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

/// PAD I/O Setup Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PADIOSetup(u32);

impl PADIOSetup {
    const _ECO_REG: u32 = 0xF << 16;
    const _INAND_SEL: u32 = 0x1 << 1;
    const _ASYNC_IO_EN: u32 = 0x1;

    // TODO
}

/// RX Configuration Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RXConfiguration(u32);

impl RXConfiguration {
    const _TUNING_DLY_INC: u32 = 0x3FF << 18;
    const _SDCLK_DELAY: u32 = 0x3FF << 8;
    const _SDCLK_SEL1: u32 = 0x3 << 2;
    const _SDCLK_SEL0: u32 = 0x3;

    // TODO
}

/// TX Configuration Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TXConfiguration(u32);

impl TXConfiguration {
    const _TX_MUX_SEL: u32 = 0x1 << 31;
    const _TX_INT_CLK_SEL: u32 = 0x1 << 30;
    const _TX_HOLD_DELAY1: u32 = 0x3FF << 16;
    const _TX_HOLD_DELAY0: u32 = 0x3FF;

    // TODO
}

/// TUNING CONFIG Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TUNINGConfiguration(u32);

impl TUNINGConfiguration {
    const _TUNING_SUCCESS_CNT: u32 = 0x3F << 24;
    const _TUNING_CLK_DLY: u32 = 0x3FF << 14;
    const _TUNING_WD_CNT: u32 = 0x3F << 8;
    const _TUNING_TT_CNT: u32 = 0xFF;

    // TODO
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use super::{
        ADMAErrorStatus, ADMASystemAddress, Argument, AutoCMDErrorStatus, AutoCMDMode, BlockCount,
        BlockGap, BlockMode, BlockSize, BufferDataPort, BusVoltage, BusWidthMode, Capabilities,
        CardSignal, ClkGenMode, ClockControl, CmdType, Command, DMAMode, DataTransferMode,
        ErrorInterruptSignalEnable, ErrorInterruptStatus, ErrorInterruptStatusEnable,
        ForceEventAutoCMDErrorStatus, ForceEventErrorInterruptStatus, HostControl1, HostControl2,
        HostControllerVersion, LedState, MaxCurrentCapabilities, NormalInterruptSignalEnable,
        NormalInterruptStatus, NormalInterruptStatusEnable, PowerControl, PresentState,
        PresetValue, Response, ResponseType, SDExtraParameters, SPIMode, SharedBusControl,
        SlotInterruptStatus, SlotType, SoftwareReset, SpecificVersion, SpeedMode, SystemAddress,
        TimeoutControl, TransferMode, TransferWidth, WakeupControl,
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
        assert_eq!(offset_of!(RegisterBlock, powercontrol), 0x29);
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
        assert_eq!(offset_of!(RegisterBlock, adma_error_status), 0x54);
        assert_eq!(offset_of!(RegisterBlock, adma_system_address), 0x58);
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

        val = val.set_cmd_num(0x3F);
        assert_eq!(val.cmd_num(), 0x3F);
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
        val = val.set_cmd_type(CmdType::Other);
        assert_eq!(val.cmd_type(), CmdType::Other);
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
        val = val.set_bus_width(BusWidthMode::SelectByDataTrabsferWidth);
        assert_eq!(val.0, 0x00);

        val = val.set_dma_mode(DMAMode::ADMA2);
        assert_eq!(val.dma_mode(), DMAMode::ADMA2);
        assert_eq!(val.0, 0x10);
        val = val.set_dma_mode(DMAMode::SDMA);
        assert_eq!(val.dma_mode(), DMAMode::SDMA);
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
    fn struct_powercontrol_functions() {
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
        let mut val = AutoCMDErrorStatus(0x0080);
        assert!(val.is_cmd_not_issued());

        val = AutoCMDErrorStatus(0x0010);
        assert!(val.if_auto_cmd_index_err_occurs());

        val = AutoCMDErrorStatus(0x0008);
        assert!(val.if_auto_cmd_end_bit_err_occurs());

        val = AutoCMDErrorStatus(0x0004);
        assert!(val.if_auto_cmd_crc_err_occurs());

        val = AutoCMDErrorStatus(0x0002);
        assert!(val.if_auto_cmd_timeout_err_occurs());

        val = AutoCMDErrorStatus(0x0001);
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
        let mut val = Capabilities(0x0);

        val = val.set_slot_type(SlotType::EmbeddedSlotforOneDevice);
        assert_eq!(val.slot_type(), SlotType::EmbeddedSlotforOneDevice);
        assert_eq!(val.0, 0x4000_0000_0000_0000);
        val = val.set_slot_type(SlotType::SharedBusSlot);
        assert_eq!(val.slot_type(), SlotType::SharedBusSlot);
        assert_eq!(val.0, 0x8000_0000_0000_0000);
        val = val.set_slot_type(SlotType::RemovableCardSlot);
        assert_eq!(val.slot_type(), SlotType::RemovableCardSlot);
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_64_bit_bus();
        assert!(val.is_64_bit_bus_enabled());
        assert_eq!(val.0, 0x1000_0000_0000_0000);
        val = val.disable_64_bit_bus();
        assert!(!val.is_64_bit_bus_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_1_8v();
        assert!(val.is_1_8v_enabled());
        assert_eq!(val.0, 0x0400_0000_0000_0000);
        val = val.disable_1_8v();
        assert!(!val.is_1_8v_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_3_0v();
        assert!(val.is_3_0v_enabled());
        assert_eq!(val.0, 0x0200_0000_0000_0000);
        val = val.disable_3_0v();
        assert!(!val.is_3_0v_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_3_3v();
        assert!(val.is_3_3v_enabled());
        assert_eq!(val.0, 0x0100_0000_0000_0000);
        val = val.disable_3_3v();
        assert!(!val.is_3_3v_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_suspend_resume();
        assert!(val.is_suspend_resume_enabled());
        assert_eq!(val.0, 0x0080_0000_0000_0000);
        val = val.disable_suspend_resume();
        assert!(!val.is_suspend_resume_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_sdma();
        assert!(val.is_sdma_enabled());
        assert_eq!(val.0, 0x0040_0000_0000_0000);
        val = val.disable_sdma();
        assert!(!val.is_sdma_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_high_speed();
        assert!(val.is_high_speed_enabled());
        assert_eq!(val.0, 0x0020_0000_0000_0000);
        val = val.disable_high_speed();
        assert!(!val.is_high_speed_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_adma2();
        assert!(val.is_adma2_enabled());
        assert_eq!(val.0, 0x0008_0000_0000_0000);
        val = val.disable_adma2();
        assert!(!val.is_adma2_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_8_bit_bus();
        assert!(val.is_8_bit_bus_enabled());
        assert_eq!(val.0, 0x0004_0000_0000_0000);
        val = val.disable_8_bit_bus();
        assert!(!val.is_8_bit_bus_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.set_max_block_len(0x3);
        assert_eq!(val.max_block_len(), 0x3);
        assert_eq!(val.0, 0x0003_0000_0000_0000);

        val = Capabilities(0x0);
        val = val.set_base_clk(0xFF);
        assert_eq!(val.base_clk(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00_0000_0000);

        val = Capabilities(0x0);
        val = val.set_timeout_clk_unit(0x1);
        assert_eq!(val.timeout_clk_unit(), 0x1);
        assert_eq!(val.0, 0x0000_0080_0000_0000);

        val = Capabilities(0x0);
        val = val.set_clk_multiplier(0xFF);
        assert_eq!(val.clk_multiplier(), 0xFF);
        assert_eq!(val.0, 0x0000_0000_00FF_0000);

        val = Capabilities(0x0);
        val = val.set_re_tuning_modes(0x3);
        assert_eq!(val.re_tuning_modes(), 0x3);
        assert_eq!(val.0, 0x0000_0000_0000_C000);

        val = Capabilities(0x0);
        val = val.enable_tuning_for_sdr50();
        assert!(val.is_tuning_for_sdr50_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_2000);
        val = val.disable_tuning_for_sdr50();
        assert!(!val.is_tuning_for_sdr50_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.set_tim_cnt_for_re_tuning(0xF);
        assert_eq!(val.tim_cnt_for_re_tuning(), 0xF);
        assert_eq!(val.0, 0x0000_0000_0000_0F00);

        val = Capabilities(0x0);
        val = val.enable_driver_type_d();
        assert!(val.is_driver_type_d_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0040);
        val = val.disable_driver_type_d();
        assert!(!val.is_driver_type_d_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_driver_type_c();
        assert!(val.is_driver_type_c_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0020);
        val = val.disable_driver_type_c();
        assert!(!val.is_driver_type_c_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_driver_type_a();
        assert!(val.is_driver_type_a_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0010);
        val = val.disable_driver_type_a();
        assert!(!val.is_driver_type_a_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_ddr50();
        assert!(val.is_ddr50_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0004);
        val = val.disable_ddr50();
        assert!(!val.is_ddr50_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_sdr104();
        assert!(val.is_sdr104_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0002);
        val = val.disable_sdr104();
        assert!(!val.is_sdr104_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);

        val = val.enable_sdr50();
        assert!(val.is_sdr50_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0001);
        val = val.disable_sdr50();
        assert!(!val.is_sdr50_enabled());
        assert_eq!(val.0, 0x0000_0000_0000_0000);
    }

    #[test]
    fn struct_max_current_capabilities_functions() {
        let mut val = MaxCurrentCapabilities(0x0);

        val = val.set_max_current_1_8v(0xFF);
        assert_eq!(val.max_current_1_8v(), 0xFF);
        assert_eq!(val.0, 0x0000_0000_00FF_0000);

        val = MaxCurrentCapabilities(0x0);
        val = val.set_max_current_3_0v(0xFF);
        assert_eq!(val.max_current_3_0v(), 0xFF);
        assert_eq!(val.0, 0x0000_0000_0000_FF00);

        val = MaxCurrentCapabilities(0x0);
        val = val.set_max_current_3_3v(0xFF);
        assert_eq!(val.max_current_3_3v(), 0xFF);
        assert_eq!(val.0, 0x0000_0000_0000_00FF);
    }

    #[test]
    fn struct_force_event_auto_cmd_error_status_functions() {
        let mut val = ForceEventAutoCMDErrorStatus(0x0);
        val = val.set_cmd_not_issued(0x1);
        assert_eq!(val.0, 0x0080);

        val = ForceEventAutoCMDErrorStatus(0x0);
        val = val.set_auto_cmd_index(0x1);
        assert_eq!(val.0, 0x0010);

        val = ForceEventAutoCMDErrorStatus(0x0);
        val = val.set_auto_cmd_end_bit(0x1);
        assert_eq!(val.0, 0x0008);

        val = ForceEventAutoCMDErrorStatus(0x0);
        val = val.set_auto_cmd_crc(0x1);
        assert_eq!(val.0, 0x0004);

        val = ForceEventAutoCMDErrorStatus(0x0);
        val = val.set_auto_cmd_timeout(0x1);
        assert_eq!(val.0, 0x0002);

        val = ForceEventAutoCMDErrorStatus(0x0);
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
    fn struct_adma_error_status_functions() {
        let mut val = ADMAErrorStatus(0x0000_0000_0000_0004);
        assert!(val.if_adma_len_mismatch_err_occurs());

        val = ADMAErrorStatus(0x0000_0000_0000_0001);
        assert_eq!(val.adma_err_state(), 0x1);
    }

    #[test]
    fn struct_adma_system_address_functions() {
        let mut val = ADMASystemAddress(0x0);
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

        val = SharedBusControl(0x0);
        val = val.set_clk_pin_num(0x7);
        assert_eq!(val.clk_pin_num(), 0x7);
        assert_eq!(val.0, 0x0000_0007);
    }

    #[test]
    fn struct_slot_interrupt_status_functions() {
        let val = SlotInterruptStatus(0x00FF);
        assert_eq!(val.int_signal(), 0xFF);
    }

    #[test]
    fn struct_host_controller_version_functions() {
        let mut val = HostControllerVersion(0x0);
        val = val.set_vendor_version(0xFF);
        assert_eq!(val.vendor_version(), 0xFF);
        assert_eq!(val.0, 0xFF00);

        val = HostControllerVersion(0x0);
        val = val.set_specific_version(SpecificVersion::SDHostSpecificVersion3);
        assert_eq!(
            val.specific_version(),
            SpecificVersion::SDHostSpecificVersion3
        );
        assert_eq!(val.0, 0x0002);

        val = HostControllerVersion(0x0);
        val = val.set_specific_version(SpecificVersion::SDHostSpecificVersion2);
        assert_eq!(
            val.specific_version(),
            SpecificVersion::SDHostSpecificVersion2
        );
        assert_eq!(val.0, 0x0001);

        val = HostControllerVersion(0x0);
        val = val.set_specific_version(SpecificVersion::SDHostSpecificVersion1);
        assert_eq!(
            val.specific_version(),
            SpecificVersion::SDHostSpecificVersion1
        );
        assert_eq!(val.0, 0x0000);
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
        let mut val = SPIMode(0x0);

        val = val.set_spi_err_token(0x1F);
        assert_eq!(val.spi_err_token(), 0x1F);
        assert_eq!(val.0, 0x0000_1F00);

        val = SPIMode(0x0);
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
        // TODO
    }

    #[test]
    fn struct_tuning_configuration_functions() {
        // TODO
    }
}
