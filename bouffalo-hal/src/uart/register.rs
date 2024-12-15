use super::{BitOrder, Parity, StopBits, WordLength};
use volatile_register::{RO, RW, WO};

/// Universal Asynchronous Receiver/Transmitter registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Transmit configuration.
    pub transmit_config: RW<TransmitConfig>,
    /// Receive configuration.
    pub receive_config: RW<ReceiveConfig>,
    /// Bit-period in clocks.
    pub bit_period: RW<BitPeriod>,
    /// Data format configuration.
    pub data_config: RW<DataConfig>,
    _reserved1: [u8; 0x10],
    /// Interrupt state register.
    pub interrupt_state: RO<InterruptState>,
    /// Interrupt mask register.
    pub interrupt_mask: RW<InterruptMask>,
    /// Clear interrupt register.
    pub interrupt_clear: WO<InterruptClear>,
    /// Interrupt enable register.
    pub interrupt_enable: RW<InterruptEnable>,
    /// Bus state.
    pub bus_state: RO<BusState>,
    _reserved2: [u8; 0x4c],
    /// First-in first-out queue configuration 0.
    pub fifo_config_0: RW<FifoConfig0>,
    /// First-in first-out queue configuration 1.
    pub fifo_config_1: RW<FifoConfig1>,
    /// Write data into first-in first-out queue.
    pub fifo_write: WO<u8>,
    _reserved3: [u8; 0x3],
    /// Read data from first-in first-out queue.
    pub fifo_read: RO<u8>,
}

/// Transmit configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct TransmitConfig(u32);

// TODO: inherent associated types is unstable, put aliases here as WAR
/// Register fields aliases, defining the bit field shift and bit length
mod transmit_config {
    use crate::BitField;

    pub(crate) type Enable = BitField<1, 0, u32>;
    pub(crate) type ParityEnable = BitField<1, 4, u32>;
    pub(crate) type ParityMode = BitField<1, 5, u32>;
    pub(crate) type WordLength = BitField<3, 8, u32>;
}

impl TransmitConfig {
    const CTS: u32 = 1 << 1;
    const FREERUN: u32 = 1 << 2;
    const LIN_TRANSMIT: u32 = 1 << 3;
    const IR_TRANSMIT: u32 = 1 << 6;
    const IR_INVERSE: u32 = 1 << 7;
    const STOP_BITS: u32 = 0b11 << 11;
    const LIN_BREAK_BITS: u32 = 0b111 << 13;
    const TRANSFER_LENGTH: u32 = 0xffff << 16;

    /// Enable transmit.
    #[inline]
    pub const fn enable_txd(self) -> Self {
        Self(transmit_config::Enable::from(self.0).enable())
    }
    /// Disable transmit.
    #[inline]
    pub const fn disable_txd(self) -> Self {
        Self(transmit_config::Enable::from(self.0).disable())
    }
    /// Check if transmit is enabled.
    #[inline]
    pub const fn is_txd_enabled(self) -> bool {
        transmit_config::Enable::from(self.0).is_enabled()
    }
    /// Enable Clear-to-Send signal.
    #[inline]
    pub const fn enable_cts(self) -> Self {
        Self(self.0 | Self::CTS)
    }
    /// Disable Clear-to-Send signal.
    #[inline]
    pub const fn disable_cts(self) -> Self {
        Self(self.0 & !Self::CTS)
    }
    /// Check if Clear-to-Send signal is enabled.
    #[inline]
    pub const fn is_cts_enabled(self) -> bool {
        self.0 & Self::CTS != 0
    }
    /// Enable free-run mode.
    #[inline]
    pub const fn enable_freerun(self) -> Self {
        Self(self.0 | Self::FREERUN)
    }
    /// Disable free-run mode.
    #[inline]
    pub const fn disable_freerun(self) -> Self {
        Self(self.0 & !Self::FREERUN)
    }
    /// Check if free-run mode is enabled.
    #[inline]
    pub const fn is_freerun_enabled(self) -> bool {
        self.0 & Self::FREERUN != 0
    }
    /// Enable LIN protocol transmission.
    #[inline]
    pub const fn enable_lin_transmit(self) -> Self {
        Self(self.0 | Self::LIN_TRANSMIT)
    }
    /// Disable LIN protocol transmission.
    #[inline]
    pub const fn disable_lin_transmit(self) -> Self {
        Self(self.0 & !Self::LIN_TRANSMIT)
    }
    /// Check if LIN protocol transmission is enabled.
    #[inline]
    pub const fn is_lin_transmit_enabled(self) -> bool {
        self.0 & Self::LIN_TRANSMIT != 0
    }
    /// Set parity check mode.
    #[inline]
    pub const fn set_parity(self, parity: Parity) -> Self {
        let field_en = transmit_config::ParityEnable::from(self.0);

        match parity {
            Parity::Even => {
                let field_odd = transmit_config::ParityMode::from(field_en.enable());
                Self(field_odd.disable())
            }
            Parity::Odd => {
                let field_odd = transmit_config::ParityMode::from(field_en.enable());
                Self(field_odd.enable())
            }
            Parity::None => Self(field_en.disable()),
        }
    }
    /// Get parity check mode.
    #[inline]
    pub const fn parity(self) -> Parity {
        let field_en = transmit_config::ParityEnable::from(self.0);
        let field_odd = transmit_config::ParityMode::from(self.0);

        if !field_en.is_enabled() {
            Parity::None
        } else if !field_odd.is_enabled() {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
    /// Enable IR transmission.
    #[inline]
    pub const fn enable_ir_transmit(self) -> Self {
        Self(self.0 | Self::IR_TRANSMIT)
    }
    /// Disable IR transmission.
    #[inline]
    pub const fn disable_ir_transmit(self) -> Self {
        Self(self.0 & !Self::IR_TRANSMIT)
    }
    /// Check if IR transmission is enabled.
    #[inline]
    pub const fn is_ir_transmit_enabled(self) -> bool {
        self.0 & Self::IR_TRANSMIT != 0
    }
    /// Invert transmit signal output in IR mode.
    #[inline]
    pub const fn enable_ir_inverse(self) -> Self {
        Self(self.0 | Self::IR_INVERSE)
    }
    /// Don't invert transmit signal output in IR mode.
    #[inline]
    pub const fn disable_ir_inverse(self) -> Self {
        Self(self.0 & !Self::IR_INVERSE)
    }
    /// Check if transmit signal output in IR mode is inverted.
    #[inline]
    pub const fn is_ir_inverse_enabled(self) -> bool {
        self.0 & Self::IR_INVERSE != 0
    }
    /// Set word length.
    #[inline]
    pub const fn set_word_length(self, val: WordLength) -> Self {
        let field = transmit_config::WordLength::from(self.0);
        let val = match val {
            WordLength::Five => 4,
            WordLength::Six => 5,
            WordLength::Seven => 6,
            WordLength::Eight => 7,
        };
        Self(field.set(val))
    }
    /// Get word length.
    #[inline]
    pub const fn word_length(self) -> WordLength {
        let field = transmit_config::WordLength::from(self.0);
        match field.get() {
            4 => WordLength::Five,
            5 => WordLength::Six,
            6 => WordLength::Seven,
            7 => WordLength::Eight,
            _ => unreachable!(),
        }
    }
    /// Set stop-bit configuration.
    #[inline]
    pub const fn set_stop_bits(self, val: StopBits) -> Self {
        let val = match val {
            StopBits::ZeroPointFive => 0,
            StopBits::One => 1,
            StopBits::OnePointFive => 2,
            StopBits::Two => 3,
        };
        Self(self.0 & !Self::STOP_BITS | val << 11)
    }
    /// Get stop-bit configuration.
    #[inline]
    pub const fn stop_bits(self) -> StopBits {
        let val = (self.0 & Self::STOP_BITS) >> 11;
        match val {
            0 => StopBits::ZeroPointFive,
            1 => StopBits::One,
            2 => StopBits::OnePointFive,
            3 => StopBits::Two,
            _ => unreachable!(),
        }
    }
    /// Set synchronize interval under LIN mode.
    ///
    /// # Parameters
    ///
    /// * `bits` - Interval in bits, the value should be 0 ~ 7.
    #[inline]
    pub const fn set_lin_break_bits(self, bits: u8) -> Self {
        Self(self.0 & !Self::LIN_BREAK_BITS | (bits as u32) << 13)
    }
    /// Get synchronize interval under LIN mode.
    ///
    /// Return value is 0 ~ 7, represent in bits.
    #[inline]
    pub const fn lin_break_bits(self) -> u8 {
        ((self.0 & Self::LIN_BREAK_BITS) >> 13) as u8
    }
    /// Trigger interrupt when specified length of data is sent.
    ///
    /// NOTE: This bit is not valid when it is running under free-run mode.
    #[inline]
    pub const fn set_transfer_length(self, length: u16) -> Self {
        Self(self.0 & !Self::TRANSFER_LENGTH | (length as u32) << 16)
    }
    /// Get the length of data that triggers the interrupt.
    #[inline]
    pub const fn transfer_length(self) -> u16 {
        ((self.0 & Self::TRANSFER_LENGTH) >> 16) as u16
    }
}

impl Default for TransmitConfig {
    #[inline]
    fn default() -> Self {
        Self(0x0000_8f00)
    }
}

/// Receive configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ReceiveConfig(u32);

mod receive_config {
    use crate::BitField;

    pub(crate) type Enable = BitField<1, 0, u32>;
    pub(crate) type ParityEnable = BitField<1, 4, u32>;
    pub(crate) type ParityMode = BitField<1, 5, u32>;
    pub(crate) type WordLength = BitField<3, 8, u32>;
}

impl ReceiveConfig {
    const ABR: u32 = 1 << 1;
    const LIN_RECEIVE: u32 = 1 << 3;
    const IR_RECEIVE: u32 = 1 << 6;
    const IR_INVERSE: u32 = 1 << 7;
    const DEGLICH: u32 = 1 << 11;
    const DEGLICH_CYCLE: u32 = 0xf << 12;
    const TRANSFER_LENGTH: u32 = 0xffff << 16;

    /// Enable receive.
    #[inline]
    pub const fn enable_rxd(self) -> Self {
        Self(receive_config::Enable::from(self.0).enable())
    }
    /// Disable receive.
    #[inline]
    pub const fn disable_rxd(self) -> Self {
        Self(receive_config::Enable::from(self.0).disable())
    }
    /// Check if receive is enabled.
    #[inline]
    pub const fn is_rxd_enabled(self) -> bool {
        receive_config::Enable::from(self.0).is_enabled()
    }
    /// Enable auto baud rate detection.
    #[inline]
    pub const fn enable_auto_baudrate(self) -> Self {
        Self(self.0 | Self::ABR)
    }
    /// Disable auto baud rate detection.
    #[inline]
    pub const fn disable_auto_baudrate(self) -> Self {
        Self(self.0 & !Self::ABR)
    }
    /// Check if auto baud rate detection is enabled.
    #[inline]
    pub const fn is_auto_baudrate_enabled(self) -> bool {
        self.0 & Self::ABR != 0
    }
    /// Enable LIN protocol receive.
    #[inline]
    pub const fn enable_lin_receive(self) -> Self {
        Self(self.0 | Self::LIN_RECEIVE)
    }
    /// Disable LIN protocol receive.
    #[inline]
    pub const fn disable_lin_receive(self) -> Self {
        Self(self.0 & !Self::LIN_RECEIVE)
    }
    /// Check if LIN protocol receive is enabled.
    #[inline]
    pub const fn is_lin_receive_enabled(self) -> bool {
        self.0 & Self::LIN_RECEIVE != 0
    }
    /// Set parity check mode.
    #[inline]
    pub const fn set_parity(self, parity: Parity) -> Self {
        let field_en = receive_config::ParityEnable::from(self.0);

        match parity {
            Parity::Even => {
                let field_odd = receive_config::ParityMode::from(field_en.enable());
                Self(field_odd.disable())
            }
            Parity::Odd => {
                let field_odd = receive_config::ParityMode::from(field_en.enable());
                Self(field_odd.enable())
            }
            Parity::None => Self(field_en.disable()),
        }
    }
    /// Get parity check mode.
    #[inline]
    pub const fn parity(self) -> Parity {
        let field_en = receive_config::ParityEnable::from(self.0);
        let field_odd = receive_config::ParityMode::from(self.0);

        if !field_en.is_enabled() {
            Parity::None
        } else if !field_odd.is_enabled() {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
    /// Enable IR receive.
    #[inline]
    pub const fn enable_ir_receive(self) -> Self {
        Self(self.0 | Self::IR_RECEIVE)
    }
    /// Disable IR receive.
    #[inline]
    pub const fn disable_ir_receive(self) -> Self {
        Self(self.0 & !Self::IR_RECEIVE)
    }
    /// Check if IR receive is enabled.
    #[inline]
    pub const fn is_ir_receive_enabled(self) -> bool {
        self.0 & Self::IR_RECEIVE != 0
    }
    /// Invert receive signal output in IR mode.
    #[inline]
    pub const fn enable_ir_inverse(self) -> Self {
        Self(self.0 | Self::IR_INVERSE)
    }
    /// Don't invert receive signal output in IR mode.
    #[inline]
    pub const fn disable_ir_inverse(self) -> Self {
        Self(self.0 & !Self::IR_INVERSE)
    }
    /// Check if receive signal output in IR mode is inverted.
    #[inline]
    pub const fn is_ir_inverse_enabled(self) -> bool {
        self.0 & Self::IR_INVERSE != 0
    }
    /// Set word length.
    #[inline]
    pub const fn set_word_length(self, val: WordLength) -> Self {
        let field = receive_config::WordLength::from(self.0);
        let val = match val {
            WordLength::Five => 4,
            WordLength::Six => 5,
            WordLength::Seven => 6,
            WordLength::Eight => 7,
        };
        Self(field.set(val))
    }
    /// Get word length.
    #[inline]
    pub const fn word_length(self) -> WordLength {
        let field = receive_config::WordLength::from(self.0);
        match field.get() {
            4 => WordLength::Five,
            5 => WordLength::Six,
            6 => WordLength::Seven,
            7 => WordLength::Eight,
            _ => unreachable!(),
        }
    }
    /// Enable de-glitch function.
    #[inline]
    pub const fn enable_deglitch(self) -> Self {
        Self(self.0 | Self::DEGLICH)
    }
    /// Disable de-glitch function.
    #[inline]
    pub const fn disable_deglitch(self) -> Self {
        Self(self.0 & !Self::DEGLICH)
    }
    /// Check if de-glitch function is enabled.
    #[inline]
    pub const fn is_deglitch_enabled(self) -> bool {
        self.0 & Self::DEGLICH != 0
    }
    /// Set de-glich function cycle count.
    #[inline]
    pub const fn set_deglitch_cycles(self, val: u8) -> Self {
        Self(self.0 & !Self::DEGLICH_CYCLE | ((val as u32) << 12))
    }
    /// Get de-glich function cycle count.
    #[inline]
    pub const fn deglitch_cycles(self) -> u8 {
        ((self.0 & Self::DEGLICH_CYCLE) >> 12) as u8
    }
    /// Set the length of data that triggers the interrupt.
    #[inline]
    pub const fn set_transfer_length(self, length: u16) -> Self {
        Self(self.0 & !Self::TRANSFER_LENGTH | (length as u32) << 16)
    }
    /// Get the length of data that triggers the interrupt.
    #[inline]
    pub const fn transfer_length(self) -> u16 {
        ((self.0 & Self::TRANSFER_LENGTH) >> 16) as u16
    }
}

impl Default for ReceiveConfig {
    #[inline]
    fn default() -> Self {
        Self(0x0000_0700)
    }
}

/// Bit period configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BitPeriod(u32);

impl BitPeriod {
    const TRANSMIT: u32 = 0xffff;
    const RECEIVE: u32 = 0xffff << 16;

    /// Set transmit time interval.
    #[inline]
    pub const fn set_transmit_time_interval(self, val: u16) -> Self {
        Self(self.0 & !Self::TRANSMIT | val as u32)
    }
    /// Get transmit time interval.
    #[inline]
    pub const fn transmit_time_interval(self) -> u16 {
        (self.0 & Self::TRANSMIT) as u16
    }
    /// Set receive time interval.
    #[inline]
    pub const fn set_receive_time_interval(self, val: u16) -> Self {
        Self(self.0 & !Self::RECEIVE | ((val as u32) << 16))
    }
    /// Get receive time interval.
    #[inline]
    pub const fn receive_time_interval(self) -> u16 {
        ((self.0 & Self::RECEIVE) >> 16) as u16
    }
}

impl Default for BitPeriod {
    #[inline]
    fn default() -> Self {
        Self(0x00ff_00ff)
    }
}

/// Data configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct DataConfig(u32);

impl DataConfig {
    const BIT_ORDER: u32 = 1 << 0;

    /// Set the bit order in each data word.
    #[inline]
    pub const fn set_bit_order(self, val: BitOrder) -> Self {
        match val {
            BitOrder::LsbFirst => Self(self.0 & !Self::BIT_ORDER),
            BitOrder::MsbFirst => Self(self.0 | Self::BIT_ORDER),
        }
    }
    /// Get the bit order in each data word.
    #[inline]
    pub const fn bit_order(self) -> BitOrder {
        if self.0 & Self::BIT_ORDER == 0 {
            BitOrder::LsbFirst
        } else {
            BitOrder::MsbFirst
        }
    }
}

impl Default for DataConfig {
    #[inline]
    fn default() -> Self {
        Self(0x0000_0000)
    }
}

/// Interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Interrupt {
    TransmitEnd = 0,
    ReceiveEnd = 1,
    TransmitFifoReady = 2,
    ReceiveFifoReady = 3,
    ReceiveTimeout = 4,
    ReceiveParityError = 5,
    TransmitFifoError = 6,
    ReceiveFifoError = 7,
    ReceiveSyncError = 8,
    ReceiveByteCountReached = 9,
    ReceiveAutoBaudrateByStartBit = 10,
    ReceiveAutoBaudrateByFiveFive = 11,
}

/// Interrupt state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptState(u32);

impl InterruptState {
    /// Check if there is an interrupt flag.
    #[inline]
    pub const fn has_interrupt(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptMask(u32);

impl InterruptMask {
    /// Set interrupt mask.
    #[inline]
    pub const fn mask_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
    /// Clear interrupt mask.
    #[inline]
    pub const fn unmask_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 & !(1 << (val as u32)))
    }
    /// Check if interrupt is masked.
    #[inline]
    pub const fn is_interrupt_masked(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt clear register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptClear(u32);

impl InterruptClear {
    /// Clear interrupt.
    #[inline]
    pub const fn clear_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
}

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptEnable(u32);

impl InterruptEnable {
    /// Enable interrupt.
    #[inline]
    pub const fn enable_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
    /// Disable interrupt.
    #[inline]
    pub const fn disable_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 & !(1 << (val as u32)))
    }
    /// Check if interrupt is enabled.
    #[inline]
    pub const fn is_interrupt_enabled(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Bus state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct BusState(u32);

impl BusState {
    const TRANSMIT_BUSY: u32 = 1 << 0;
    const RECEIVE_BUSY: u32 = 1 << 1;

    /// Get if UART transmit bus is busy.
    #[inline]
    pub const fn transmit_busy(self) -> bool {
        self.0 & Self::TRANSMIT_BUSY != 0
    }
    /// Get if UART receive bus is busy.
    #[inline]
    pub const fn receive_busy(self) -> bool {
        self.0 & Self::RECEIVE_BUSY != 0
    }
}

/// First-in first-out queue configuration 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoConfig0(u32);

impl FifoConfig0 {
    const TRANSMIT_DMA_ENABLE: u32 = 1 << 0;
    const RECEIVE_DMA_ENABLE: u32 = 1 << 1;
    const TRANSMIT_FIFO_CLEAR: u32 = 1 << 2;
    const RECEIVE_FIFO_CLEAR: u32 = 1 << 3;
    const TRANSMIT_FIFO_OVERFLOW: u32 = 1 << 4;
    const TRANSMIT_FIFO_UNDERFLOW: u32 = 1 << 5;
    const RECEIVE_FIFO_OVERFLOW: u32 = 1 << 6;
    const RECEIVE_FIFO_UNDERFLOW: u32 = 1 << 7;

    /// Enable transmit DMA.
    #[inline]
    pub const fn enable_transmit_dma(self) -> Self {
        Self(self.0 | Self::TRANSMIT_DMA_ENABLE)
    }
    /// Disable transmit DMA.
    #[inline]
    pub const fn disable_transmit_dma(self) -> Self {
        Self(self.0 & !Self::TRANSMIT_DMA_ENABLE)
    }
    /// Check if transmit DMA is enabled.
    #[inline]
    pub const fn is_transmit_dma_enabled(self) -> bool {
        self.0 & Self::TRANSMIT_DMA_ENABLE != 0
    }
    /// Enable receive DMA.
    #[inline]
    pub const fn enable_receive_dma(self) -> Self {
        Self(self.0 | Self::RECEIVE_DMA_ENABLE)
    }
    /// Disable receive DMA.
    #[inline]
    pub const fn disable_receive_dma(self) -> Self {
        Self(self.0 & !Self::RECEIVE_DMA_ENABLE)
    }
    /// Check if receive DMA is enabled.
    #[inline]
    pub const fn is_receive_dma_enabled(self) -> bool {
        self.0 & Self::RECEIVE_DMA_ENABLE != 0
    }
    /// Clear transmit FIFO.
    #[inline]
    pub const fn clear_transmit_fifo(self) -> Self {
        Self(self.0 | Self::TRANSMIT_FIFO_CLEAR)
    }
    /// Clear receive FIFO.
    #[inline]
    pub const fn clear_receive_fifo(self) -> Self {
        Self(self.0 | Self::RECEIVE_FIFO_CLEAR)
    }
    /// Check if transmit FIFO is overflow.
    #[inline]
    pub const fn transmit_fifo_overflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_OVERFLOW != 0
    }
    /// Check if transmit FIFO is underflow.
    #[inline]
    pub const fn transmit_fifo_underflow(self) -> bool {
        self.0 & Self::TRANSMIT_FIFO_UNDERFLOW != 0
    }
    /// Check if receive FIFO is overflow.
    #[inline]
    pub const fn receive_fifo_overflow(self) -> bool {
        self.0 & Self::RECEIVE_FIFO_OVERFLOW != 0
    }
    /// Check if receive FIFO is underflow.
    #[inline]
    pub const fn receive_fifo_underflow(self) -> bool {
        self.0 & Self::RECEIVE_FIFO_UNDERFLOW != 0
    }
}

/// First-in first-out queue configuration 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct FifoConfig1(u32);

impl FifoConfig1 {
    const TRANSMIT_COUNT: u32 = 0x3f;
    const RECEIVE_COUNT: u32 = 0x3f << 8;
    const TRANSMIT_THRESHOLD: u32 = 0x1f << 16;
    const RECEIVE_THRESHOLD: u32 = 0x1f << 24;

    /// Get number of empty spaces remained in transmit FIFO queue.
    #[inline]
    pub const fn transmit_available_bytes(self) -> u8 {
        (self.0 & Self::TRANSMIT_COUNT) as u8
    }
    /// Get number of available bytes received in receive FIFO queue.
    #[inline]
    pub const fn receive_available_bytes(self) -> u8 {
        ((self.0 & Self::RECEIVE_COUNT) >> 8) as u8
    }
    /// Set transmit FIFO threshold.
    #[inline]
    pub const fn set_transmit_threshold(self, val: u8) -> Self {
        Self(self.0 & !Self::TRANSMIT_THRESHOLD | ((val as u32) << 16))
    }
    /// Get transmit FIFO threshold.
    #[inline]
    pub const fn transmit_threshold(self) -> u8 {
        ((self.0 & Self::TRANSMIT_THRESHOLD) >> 16) as u8
    }
    /// Set receive FIFO threshold.
    #[inline]
    pub const fn set_receive_threshold(self, val: u8) -> Self {
        Self(self.0 & !Self::RECEIVE_THRESHOLD | ((val as u32) << 24))
    }
    /// Get receive FIFO threshold.
    #[inline]
    pub const fn receive_threshold(self) -> u8 {
        ((self.0 & Self::RECEIVE_THRESHOLD) >> 24) as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::uart::{StopBits, WordLength};

    use super::{BitPeriod, Parity, ReceiveConfig, RegisterBlock, TransmitConfig};
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, transmit_config), 0x0);
        assert_eq!(offset_of!(RegisterBlock, receive_config), 0x4);
        assert_eq!(offset_of!(RegisterBlock, bit_period), 0x08);
        assert_eq!(offset_of!(RegisterBlock, data_config), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, interrupt_state), 0x20);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mask), 0x24);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x28);
        assert_eq!(offset_of!(RegisterBlock, interrupt_enable), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, bus_state), 0x30);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_0), 0x80);
        assert_eq!(offset_of!(RegisterBlock, fifo_config_1), 0x84);
        assert_eq!(offset_of!(RegisterBlock, fifo_write), 0x88);
        assert_eq!(offset_of!(RegisterBlock, fifo_read), 0x8c);
    }

    #[test]
    fn struct_transmit_config_functions() {
        let mut val: TransmitConfig = TransmitConfig(0x0);

        val = val.enable_txd();
        assert_eq!(val.0, 0x00000001);
        assert!(val.is_txd_enabled());
        val = val.disable_txd();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_txd_enabled());

        val = val.enable_cts();
        assert_eq!(val.0, 0x00000002);
        assert!(val.is_cts_enabled());
        val = val.disable_cts();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_cts_enabled());

        val = val.enable_freerun();
        assert_eq!(val.0, 0x00000004);
        assert!(val.is_freerun_enabled());
        val = val.disable_freerun();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_freerun_enabled());

        val = val.enable_lin_transmit();
        assert_eq!(val.0, 0x00000008);
        assert!(val.is_lin_transmit_enabled());
        val = val.disable_lin_transmit();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lin_transmit_enabled());

        val = val.set_parity(Parity::Even);
        assert_eq!(val.0, 0x00000010);
        assert_eq!(val.parity(), Parity::Even);
        val = val.set_parity(Parity::Odd);
        assert_eq!(val.0, 0x00000030);
        assert_eq!(val.parity(), Parity::Odd);
        val = val.set_parity(Parity::None);
        assert_eq!(val.0 & 0x00000010, 0x00000000);
        assert_eq!(val.parity(), Parity::None);

        val = TransmitConfig(0x0);

        val = val.enable_ir_transmit();
        assert_eq!(val.0, 0x00000040);
        assert!(val.is_ir_transmit_enabled());
        val = val.disable_ir_transmit();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_ir_transmit_enabled());

        val = val.enable_ir_inverse();
        assert_eq!(val.0, 0x00000080);
        assert!(val.is_ir_inverse_enabled());
        val = val.disable_ir_inverse();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_ir_inverse_enabled());

        val = val.set_word_length(WordLength::Five);
        assert_eq!(val.0, 0x00000400);
        assert_eq!(val.word_length(), WordLength::Five);
        val = val.set_word_length(WordLength::Six);
        assert_eq!(val.0, 0x00000500);
        assert_eq!(val.word_length(), WordLength::Six);
        val = val.set_word_length(WordLength::Seven);
        assert_eq!(val.0, 0x00000600);
        assert_eq!(val.word_length(), WordLength::Seven);
        val = val.set_word_length(WordLength::Eight);
        assert_eq!(val.0, 0x00000700);
        assert_eq!(val.word_length(), WordLength::Eight);

        val = TransmitConfig(0x0);

        val = val.set_stop_bits(StopBits::Two);
        assert_eq!(val.0, 0x00001800);
        assert_eq!(val.stop_bits(), StopBits::Two);
        val = val.set_stop_bits(StopBits::OnePointFive);
        assert_eq!(val.0, 0x00001000);
        assert_eq!(val.stop_bits(), StopBits::OnePointFive);
        val = val.set_stop_bits(StopBits::One);
        assert_eq!(val.0, 0x00000800);
        assert_eq!(val.stop_bits(), StopBits::One);
        val = val.set_stop_bits(StopBits::ZeroPointFive);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.stop_bits(), StopBits::ZeroPointFive);

        for num in 0..=7 {
            val = val.set_lin_break_bits(num);
            assert_eq!(val.0, (num as u32) << 13);
            assert_eq!(val.lin_break_bits(), num);
        }

        val = TransmitConfig(0x0);

        for length in [0x0000, 0x1234, 0xabcd, 0xffff] {
            val = val.set_transfer_length(length);
            assert_eq!(val.0, (length as u32) << 16);
            assert_eq!(val.transfer_length(), length);
        }

        let default = TransmitConfig::default();
        assert_eq!(default.transfer_length(), 0);
        assert_eq!(default.lin_break_bits(), 4);
        assert_eq!(default.stop_bits(), StopBits::One);
        assert_eq!(default.word_length(), WordLength::Eight);
        assert!(!default.is_ir_inverse_enabled());
        assert!(!default.is_ir_transmit_enabled());
        assert_eq!(default.parity(), Parity::None);
        assert!(!default.is_lin_transmit_enabled());
        assert!(!default.is_freerun_enabled());
        assert!(!default.is_cts_enabled());
        assert!(!default.is_txd_enabled());
    }

    #[test]
    fn struct_bit_period_functions() {
        let mut val: BitPeriod = BitPeriod(0x0);

        for trans in [0x0000, 0x1037, 0xabcd, 0xffff] {
            val = val.set_transmit_time_interval(trans);
            assert_eq!(val.0, trans as u32);
            assert_eq!(val.transmit_time_interval(), trans);
        }

        val = BitPeriod(0x0);

        for recv in [0x0000, 0x1037, 0xabcd, 0xffff] {
            val = val.set_receive_time_interval(recv);
            assert_eq!(val.0, (recv as u32) << 16);
            assert_eq!(val.receive_time_interval(), recv);
        }

        // TODO: use getter functions to check default value for BitPeriod
    }

    #[test]
    fn struct_receive_config_functions() {
        let mut val: ReceiveConfig = ReceiveConfig(0x0);

        val = val.enable_rxd();
        assert_eq!(val.0, 0x00000001);
        assert!(val.is_rxd_enabled());
        val = val.disable_rxd();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_rxd_enabled());

        val = val.enable_auto_baudrate();
        assert_eq!(val.0, 0x00000002);
        assert!(val.is_auto_baudrate_enabled());
        val = val.disable_auto_baudrate();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_auto_baudrate_enabled());

        val = val.enable_lin_receive();
        assert_eq!(val.0, 0x00000008);
        assert!(val.is_lin_receive_enabled());
        val = val.disable_lin_receive();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_lin_receive_enabled());

        val = val.set_parity(Parity::Even);
        assert_eq!(val.0, 0x00000010);
        assert_eq!(val.parity(), Parity::Even);
        val = val.set_parity(Parity::Odd);
        assert_eq!(val.0, 0x00000030);
        assert_eq!(val.parity(), Parity::Odd);
        val = val.set_parity(Parity::None);
        assert_eq!(val.0 & 0x00000010, 0x00000000);
        assert_eq!(val.parity(), Parity::None);

        val = ReceiveConfig(0x0);

        val = val.enable_ir_receive();
        assert_eq!(val.0, 0x00000040);
        assert!(val.is_ir_receive_enabled());
        val = val.disable_ir_receive();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_ir_receive_enabled());

        val = val.enable_ir_inverse();
        assert_eq!(val.0, 0x00000080);
        assert!(val.is_ir_inverse_enabled());
        val = val.disable_ir_inverse();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_ir_inverse_enabled());

        val = val.set_word_length(WordLength::Five);
        assert_eq!(val.0, 0x00000400);
        assert_eq!(val.word_length(), WordLength::Five);
        val = val.set_word_length(WordLength::Six);
        assert_eq!(val.0, 0x00000500);
        assert_eq!(val.word_length(), WordLength::Six);
        val = val.set_word_length(WordLength::Seven);
        assert_eq!(val.0, 0x00000600);
        assert_eq!(val.word_length(), WordLength::Seven);
        val = val.set_word_length(WordLength::Eight);
        assert_eq!(val.0, 0x00000700);
        assert_eq!(val.word_length(), WordLength::Eight);

        val = ReceiveConfig(0x0);

        val = val.enable_deglitch();
        assert_eq!(val.0, 0x00000800);
        assert!(val.is_deglitch_enabled());
        val = val.disable_deglitch();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_deglitch_enabled());

        for num in 0..=7 {
            val = val.set_deglitch_cycles(num);
            assert_eq!(val.0, (num as u32) << 12);
            assert_eq!(val.deglitch_cycles(), num);
        }

        val = ReceiveConfig(0x0);

        for length in [0x0000, 0x1234, 0xabcd, 0xffff] {
            val = val.set_transfer_length(length);
            assert_eq!(val.0, (length as u32) << 16);
            assert_eq!(val.transfer_length(), length);
        }
    }

    // TODO: use getter functions to check default value for ReceiveConfig
}
