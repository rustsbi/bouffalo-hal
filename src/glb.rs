//! Global peripheral.
use core::cell::UnsafeCell;

use volatile_register::{RO, RW, WO};

/// Generic Purpose Input/Output registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x150],
    /// Universal Asynchronous Receiver/Transmitter clock and mode configurations.
    pub uart_config: UART_CONFIG,
    /// Universal Asynchronous Receiver/Transmitter signal multiplexer.
    pub uart_mux_group: [UART_MUX_GROUP; 2],
    _reserved1: [u8; 0x768],
    /// Generic Purpose Input/Output config.
    pub gpio_config: [GPIO_CONFIG; 46],
    _reserved2: [u8; 0x148],
    /// Read value from Generic Purpose Input/Output pins.
    pub gpio_input: [RO<u32>; 2],
    _reserved3: [u8; 0x18],
    /// Write value to Generic Purpose Input/Output pins.
    pub gpio_output: [RW<u32>; 2],
    /// Set pin output value to high.
    pub gpio_set: [WO<u32>; 2],
    /// Clear pin output value to low.
    pub gpio_clear: [WO<u32>; 2],
}

/// Universal Asynchronous Receiver/Transmitter clock and mode configuration.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct UART_CONFIG(UnsafeCell<u32>);

/// Configuration structure for Universal Asynchronous Receiver/Transmitter clock and mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UartConfig(u32);

impl UART_CONFIG {
    /// Read UART config.
    #[inline]
    pub fn read(&self) -> UartConfig {
        UartConfig(unsafe { self.0.get().read_volatile() })
    }
    /// Write UART config.
    #[inline]
    pub fn write(&self, val: UartConfig) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl UartConfig {
    const CLOCK_DIVIDE: u32 = 0x3 << 0;
    const CLOCK_ENABLE: u32 = 0x1 << 4;

    /// Set peripheral clock divide factor.
    #[inline]
    pub const fn set_clock_divide(self, val: u8) -> Self {
        Self(self.0 & !Self::CLOCK_DIVIDE | (val as u32))
    }
    /// Get peripheral clock divide factor.
    #[inline]
    pub const fn clock_divide(self) -> u8 {
        (self.0 & Self::CLOCK_DIVIDE) as u8
    }
    /// Enable peripheral level clock gate.
    #[inline]
    pub const fn enable_clock(self) -> Self {
        Self(self.0 | Self::CLOCK_ENABLE)
    }
    /// Disable peripheral level clock gate.
    #[inline]
    pub const fn disable_clock(self) -> Self {
        Self(self.0 & !Self::CLOCK_ENABLE)
    }
    /// Check if peripheral level clock gate is enabled.
    #[inline]
    pub const fn is_clock_enabled(self) -> bool {
        self.0 & Self::CLOCK_ENABLE != 0
    }
}

/// Universal Asynchronous Receiver/Transmitter signal configuration register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct UART_MUX_GROUP(UnsafeCell<u32>);

/// Configuration structure for UART signal multiplexer group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UartMuxGroup(u32);

impl UART_MUX_GROUP {
    /// Read UART signal multiplexer group configuration.
    #[inline]
    pub fn read(&self) -> UartMuxGroup {
        UartMuxGroup(unsafe { self.0.get().read_volatile() })
    }
    /// Write UART signal multiplexer group configuration.
    #[inline]
    pub fn write(&self, val: UartMuxGroup) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

/// UART multiplexer signal configuration.
#[repr(u8)]
pub enum UartSignal {
    /// UART0 Request-to-Send signal.
    Rts0 = 0,
    /// UART0 Clear-to-Send signal.
    Cts0 = 1,
    /// UART0 Transmit signal.
    Txd0 = 2,
    /// UART0 Receive signal.
    Rxd0 = 3,
    /// UART1 Request-to-Send signal.
    Rts1 = 4,
    /// UART1 Clear-to-Send signal.
    Cts1 = 5,
    /// UART1 Transmit signal.
    Txd1 = 6,
    /// UART1 Receive signal.
    Rxd1 = 7,
    /// UART2 Request-to-Send signal.
    Rts2 = 8,
    /// UART2 Clear-to-Send signal.
    Cts2 = 9,
    /// UART2 Transmit signal.
    Txd2 = 10,
    /// UART2 Receive signal.
    Rxd2 = 11,
}

impl UartMuxGroup {
    const SIGNAL: u32 = 0xf;

    /// Set signal for UART multiplexer.
    #[inline]
    pub const fn set_signal(self, idx: usize, val: UartSignal) -> Self {
        assert!(idx <= 7);
        Self((self.0 & !(Self::SIGNAL << (idx * 4))) | ((val as u32) << (idx * 4)))
    }
    /// Get signal for UART multiplexer.
    #[inline]
    pub const fn signal(self, idx: usize) -> UartSignal {
        assert!(idx <= 7);
        match (self.0 & (Self::SIGNAL << (idx * 4))) >> (idx * 4) {
            0 => UartSignal::Rts0,
            1 => UartSignal::Cts0,
            2 => UartSignal::Txd0,
            3 => UartSignal::Rxd0,
            4 => UartSignal::Rts1,
            5 => UartSignal::Cts1,
            6 => UartSignal::Txd1,
            7 => UartSignal::Rxd1,
            8 => UartSignal::Rts2,
            9 => UartSignal::Cts2,
            10 => UartSignal::Txd2,
            11 => UartSignal::Rxd2,
            _ => unreachable!(),
        }
    }
}

/// Generic Purpose Input/Output Configuration register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct GPIO_CONFIG(UnsafeCell<u32>);

/// Configuration structure for current GPIO pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpioConfig(u32);

impl GPIO_CONFIG {
    /// Read GPIO pin configuration.
    #[inline]
    pub fn read(&self) -> GpioConfig {
        GpioConfig(unsafe { self.0.get().read_volatile() })
    }
    /// Write GPIO pin configuration.
    #[inline]
    pub fn write(&self, val: GpioConfig) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl GpioConfig {
    const INPUT_ENABLE: u32 = 1 << 0;
    const SCHMITT: u32 = 1 << 1;
    const DRIVE: u32 = 0x3 << 2;
    const PULL: u32 = 0x3 << 4;
    const OUTPUT_ENABLE: u32 = 1 << 6;
    const FUNCTION: u32 = 0x1f << 8;
    const INTERRUPT_MODE: u32 = 0xf << 16;
    const CLEAR_INTERRUPT: u32 = 1 << 20;
    const HAS_INTERRUPT: u32 = 1 << 21;
    const INTERRUPT_MASK: u32 = 1 << 22;
    const OUTPUT: u32 = 1 << 24;
    const SET: u32 = 1 << 25;
    const CLEAR: u32 = 1 << 26;
    const INPUT: u32 = 1 << 28;
    const MODE: u32 = 0x3 << 30;

    /// Enable input function of current pin.
    #[inline]
    pub const fn enable_input(self) -> Self {
        Self(self.0 | Self::INPUT_ENABLE)
    }
    /// Disable input function of current pin.
    #[inline]
    pub const fn disable_input(self) -> Self {
        Self(self.0 & !Self::INPUT_ENABLE)
    }
    /// Check if input function of current pin is enabled.
    #[inline]
    pub const fn is_input_enabled(self) -> bool {
        self.0 & Self::INPUT_ENABLE != 0
    }
    /// Enable Schmitt trigger function of current pin.
    #[inline]
    pub const fn enable_schmitt(self) -> Self {
        Self(self.0 | Self::SCHMITT)
    }
    /// Disable Schmitt trigger function of current pin.
    #[inline]
    pub const fn disable_schmitt(self) -> Self {
        Self(self.0 & !Self::SCHMITT)
    }
    /// Check if Schmitt trigger function of current pin is enabled.
    #[inline]
    pub const fn is_schmitt_enabled(self) -> bool {
        self.0 & Self::SCHMITT != 0
    }
    /// Enable output function of current pin.
    #[inline]
    pub const fn enable_output(self) -> Self {
        Self(self.0 | Self::OUTPUT_ENABLE)
    }
    /// Disable output function of current pin.
    #[inline]
    pub const fn disable_output(self) -> Self {
        Self(self.0 & !Self::OUTPUT_ENABLE)
    }
    /// Check if output function of current pin is enabled.
    #[inline]
    pub const fn is_output_enabled(self) -> bool {
        self.0 & Self::OUTPUT_ENABLE != 0
    }
    /// Enable interrupt function of current pin.
    #[inline]
    pub const fn mask_interrupt(self) -> Self {
        Self(self.0 | Self::INTERRUPT_MASK)
    }
    /// Disable interrupt function of current pin.
    #[inline]
    pub const fn unmask_interrupt(self) -> Self {
        Self(self.0 & !Self::INTERRUPT_MASK)
    }
    /// Check if interrupt function of current pin is enabled.
    #[inline]
    pub const fn is_interrupt_masked(self) -> bool {
        self.0 & Self::INTERRUPT_MASK != 0
    }
    /// Get output of current pin.
    #[inline]
    pub const fn output(self) -> bool {
        self.0 & Self::OUTPUT != 0
    }
    /// Get intput of current pin.
    #[inline]
    pub const fn input(self) -> bool {
        self.0 & Self::INPUT != 0
    }
    /// Check if current pin has interrupt function.
    #[inline]
    pub const fn has_interrupt(self) -> bool {
        self.0 & Self::HAS_INTERRUPT != 0
    }
    /// Set pin output value to high.
    #[inline]
    pub const fn set(self) -> Self {
        Self(self.0 | Self::SET)
    }
    /// Clear pin output value to low.
    #[inline]
    pub const fn clear(self) -> Self {
        Self(self.0 | Self::CLEAR)
    }
    /// Clear interrupt pin output flag.
    #[inline]
    pub const fn clear_interrupt(self) -> Self {
        Self(self.0 | Self::CLEAR_INTERRUPT)
    }
    /// Get drive strength of current pin.
    #[inline]
    pub const fn drive(self) -> Drive {
        match (self.0 & Self::DRIVE) >> 2 {
            0 => Drive::Drive0,
            1 => Drive::Drive1,
            2 => Drive::Drive2,
            3 => Drive::Drive3,
            _ => unreachable!(),
        }
    }
    /// Set drive strength of current pin.
    #[inline]
    pub const fn set_drive(self, val: Drive) -> Self {
        Self((self.0 & !Self::DRIVE) | ((val as u32) << 2))
    }
    /// Get function of current pin.
    #[inline]
    pub const fn function(self) -> Function {
        match (self.0 & Self::FUNCTION) >> 8 {
            0 => Function::Sdh,
            1 => Function::Spi0,
            2 => Function::Flash,
            3 => Function::I2s,
            4 => Function::Pdm,
            5 => Function::I2c0,
            6 => Function::I2c1,
            7 => Function::Uart,
            8 => Function::Emac,
            9 => Function::Cam,
            10 => Function::Analog,
            11 => Function::Gpio,
            16 => Function::Pwm0,
            17 => Function::Pwm1,
            18 => Function::Spi1,
            19 => Function::I2c2,
            20 => Function::I2c3,
            21 => Function::MmUart,
            22 => Function::DbiB,
            23 => Function::DbiC,
            24 => Function::Dpi,
            25 => Function::JtagLp,
            26 => Function::JtagM0,
            27 => Function::JtagD0,
            31 => Function::ClockOut,
            _ => unreachable!(),
        }
    }
    /// Set function of current pin.
    #[inline]
    pub const fn set_function(self, val: Function) -> Self {
        Self((self.0 & !Self::FUNCTION) | ((val as u32) << 8))
    }
    /// Get interrupt mode of current pin.
    pub const fn interrupt_mode(self) -> InterruptMode {
        match (self.0 & Self::INTERRUPT_MODE) >> 16 {
            0 => InterruptMode::SyncFallingEdge,
            1 => InterruptMode::SyncRisingEdge,
            2 => InterruptMode::SyncLowLevel,
            3 => InterruptMode::SyncHighLevel,
            4 => InterruptMode::SyncBothEdges,
            8 => InterruptMode::AsyncFallingEdge,
            9 => InterruptMode::AsyncRisingEdge,
            10 => InterruptMode::AsyncLowLevel,
            11 => InterruptMode::AsyncHighLevel,
            _ => unreachable!(),
        }
    }
    /// Set interrupt mode of current pin.
    #[inline]
    pub const fn set_interrupt_mode(self, val: InterruptMode) -> Self {
        Self((self.0 & !Self::INTERRUPT_MODE) | ((val as u32) << 16))
    }
    /// Get mode of current pin.
    pub const fn mode(self) -> Mode {
        match (self.0 & Self::MODE) >> 30 {
            0 => Mode::Normal,
            1 => Mode::SetClear,
            2 => Mode::Programmable,
            3 => Mode::BufferedSetClear,
            _ => unreachable!(),
        }
    }
    /// Set mode of current pin.
    #[inline]
    pub const fn set_mode(self, val: Mode) -> Self {
        Self((self.0 & !Self::MODE) | ((val as u32) << 30))
    }
    /// Get pull direction of current pin.
    pub const fn pull(self) -> Pull {
        match (self.0 & Self::PULL) >> 4 {
            0 => Pull::None,
            1 => Pull::Up,
            2 => Pull::Down,
            _ => unreachable!(),
        }
    }
    /// Set pull direction of current pin.
    #[inline]
    pub const fn set_pull(self, val: Pull) -> Self {
        Self((self.0 & !Self::PULL) | ((val as u32) << 4))
    }
    /// Reset value of GPIO_CONFIG register.
    pub(crate) const RESET_VALUE: Self = Self(0x0040_0b02);
}

/// Pin drive strength.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drive {
    /// Drive strength 0.
    Drive0 = 0,
    /// Drive strength 1.
    Drive1 = 1,
    /// Drive strength 2.
    Drive2 = 2,
    /// Drive strength 3.
    Drive3 = 3,
}

/// Pin alternate function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Function {
    Sdh = 0,
    Spi0 = 1,
    Flash = 2,
    I2s = 3,
    Pdm = 4,
    I2c0 = 5,
    I2c1 = 6,
    Uart = 7,
    Emac = 8,
    Cam = 9,
    Analog = 10,
    Gpio = 11,
    Pwm0 = 16,
    Pwm1 = 17,
    Spi1 = 18,
    I2c2 = 19,
    I2c3 = 20,
    MmUart = 21,
    DbiB = 22,
    DbiC = 23,
    Dpi = 24,
    JtagLp = 25,
    JtagM0 = 26,
    JtagD0 = 27,
    ClockOut = 31,
}

/// Pin interrupt mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InterruptMode {
    SyncFallingEdge = 0,
    SyncRisingEdge = 1,
    SyncLowLevel = 2,
    SyncHighLevel = 3,
    SyncBothEdges = 4,
    AsyncFallingEdge = 8,
    AsyncRisingEdge = 9,
    AsyncLowLevel = 10,
    AsyncHighLevel = 11,
}

/// Pin mode as GPIO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Normal = 0,
    SetClear = 1,
    Programmable = 2,
    BufferedSetClear = 3,
}

/// Pin pull direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pull {
    None = 0,
    Up = 1,
    Down = 2,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, uart_config), 0x150);
        assert_eq!(offset_of!(RegisterBlock, uart_mux_group), 0x154);
        assert_eq!(offset_of!(RegisterBlock, gpio_config), 0x8c4);
        assert_eq!(offset_of!(RegisterBlock, gpio_input), 0xac4);
        assert_eq!(offset_of!(RegisterBlock, gpio_output), 0xae4);
        assert_eq!(offset_of!(RegisterBlock, gpio_set), 0xaec);
        assert_eq!(offset_of!(RegisterBlock, gpio_clear), 0xaf4);
    }
}
