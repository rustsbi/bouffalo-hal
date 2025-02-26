//! Global configurations on BL808 and BL616 series.
use super::{Drive, Pull};
use volatile_register::{RO, RW, WO};

/// Global configuration registers.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x150],
    /// Universal Asynchronous Receiver/Transmitter clock and mode configurations.
    pub uart_config: RW<UartConfig>,
    /// Universal Asynchronous Receiver/Transmitter signal multiplexer.
    pub uart_mux_group: [RW<UartMuxGroup>; 2],
    _reserved1: [u8; 0x24],
    /// Inter-Integrated Circuit configuration register.
    pub i2c_config: RW<I2cConfig>,
    _reserved2: [u8; 0x2c],
    /// Serial Peripheral Interface configuration register.
    pub spi_config: RW<SpiConfig>,
    _reserved3: [u8; 0x1c],
    /// Pulse Width Modulation configuration register.
    pub pwm_config: RW<PwmConfig>,
    _reserved4: [u8; 0x25c],
    /// SDH configuration register.
    pub sdh_config: RW<SdhConfig>,
    _reserved5: [u8; 0xdd],
    pub param_config: RW<ParamConfig>,
    _reserved6: [u8; 0x6c],
    /// Clock generation configuration 0.
    pub clock_config_0: RW<ClockConfig0>,
    /// Clock generation configuration 1.
    pub clock_config_1: RW<ClockConfig1>,
    /// Clock generation configuration 2.
    pub clock_config_2: RW<ClockConfig2>,
    /// Clock generation configuration 3.
    pub clock_config_3: RW<ClockConfig3>,
    _reserved7: [u8; 0x140],
    /// LDO12UHS config.
    pub ldo12uhs_config: RW<Ldo12uhsConfig>,
    _reserved8: [u8; 0x1f0],
    /// Generic Purpose Input/Output config.
    pub gpio_config: [RW<GpioConfig>; 46],
    _reserved9: [u8; 0x148],
    /// Read value from Generic Purpose Input/Output pads.
    pub gpio_input: [RO<u32>; 2],
    _reserved10: [u8; 0x18],
    /// Write value to Generic Purpose Input/Output pads.
    pub gpio_output: [RW<u32>; 2],
    /// Set pin output value to high.
    pub gpio_set: [WO<u32>; 2],
    /// Clear pin output value to low.
    pub gpio_clear: [WO<u32>; 2],
}

/// Universal Asynchronous Receiver/Transmitter clock and mode configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UartConfig(u32);

impl UartConfig {
    /// TODO: make divide factors a new type(enum), like UartSignal
    const CLOCK_DIVIDE: u32 = 0x7 << 0;
    const CLOCK_ENABLE: u32 = 0x1 << 4;
    // const HBN_CLOCK_SEL: u32 = 0x1 << 7;
    // const HBN_CLOCK_SEL2: u32 = 0x1 << 22;
    // const UART2_IO_SEL: u32 = 0x1 << 24;

    /// Set peripheral clock divide factor.
    #[inline]
    pub const fn set_clock_divide(self, val: u8) -> Self {
        Self(self.0 & !Self::CLOCK_DIVIDE | ((val as u32) << 0) & Self::CLOCK_DIVIDE)
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

/// UART signal multiplexer group configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct UartMuxGroup(u32);

/// UART multiplexer signal configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// Inter-Integrated Circuit clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2cClockSource {
    /// Bus clock.
    Bclk = 0,
    /// Crystal oscillator clock.
    Xclk = 1,
}

/// Inter-Integrated Circuit configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct I2cConfig(u32);

impl I2cConfig {
    const CLOCK_DIVIDE: u32 = 0xff << 16;
    const CLOCK_ENABLE: u32 = 1 << 24;
    const CLOCK_SELECT: u32 = 1 << 25;

    /// Set peripheral clock divide factor.
    #[inline]
    pub const fn set_clock_divide(self, val: u8) -> Self {
        Self((self.0 & !Self::CLOCK_DIVIDE) | ((val as u32) << 16))
    }
    /// Get peripheral clock divide factor.
    #[inline]
    pub const fn clock_divide(self) -> u8 {
        ((self.0 & Self::CLOCK_DIVIDE) >> 16) as u8
    }
    /// Enable clock for Inter-Integrated Circuit peripheral.
    #[inline]
    pub const fn enable_clock(self) -> Self {
        Self(self.0 | Self::CLOCK_ENABLE)
    }
    /// Disable clock for Inter-Integrated Circuit peripheral.
    #[inline]
    pub const fn disable_clock(self) -> Self {
        Self(self.0 & !Self::CLOCK_ENABLE)
    }
    /// Check if clock for Inter-Integrated Circuit peripheral is enabled.
    #[inline]
    pub const fn is_clock_enabled(self) -> bool {
        self.0 & Self::CLOCK_ENABLE != 0
    }
    /// Set clock source for Inter-Integrated Circuit peripheral.
    #[inline]
    pub const fn set_clock_source(self, val: I2cClockSource) -> Self {
        Self((self.0 & !Self::CLOCK_SELECT) | ((val as u32) << 25))
    }
    /// Get clock source for Inter-Integrated Circuit peripheral.
    #[inline]
    pub const fn clock_source(self) -> I2cClockSource {
        match (self.0 & Self::CLOCK_SELECT) >> 25 {
            0 => I2cClockSource::Bclk,
            1 => I2cClockSource::Xclk,
            _ => unreachable!(),
        }
    }
}

/// Serial Peripheral Interface clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiClockSource {
    /// 160-MHz mutiplexer PLL.
    MuxPll160M = 0,
    /// Crystal oscillator clock.
    Xclk = 1,
}

/// Serial Peripheral Interface configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SpiConfig(u32);

impl SpiConfig {
    const CLOCK_DIVIDE: u32 = 0xff << 0;
    const CLOCK_ENABLE: u32 = 1 << 8;
    const CLOCK_SELECT: u32 = 1 << 9;
    // TODO const SWAP_SELECT: u32 = 1 << 16;

    /// Set peripheral clock divide factor.
    #[inline]
    pub const fn set_clock_divide(self, val: u8) -> Self {
        Self((self.0 & !Self::CLOCK_DIVIDE) | ((val as u32) << 0))
    }
    /// Get peripheral clock divide factor.
    #[inline]
    pub const fn clock_divide(self) -> u8 {
        ((self.0 & Self::CLOCK_DIVIDE) >> 0) as u8
    }
    /// Enable clock for Serial Peripheral Interface peripheral.
    #[inline]
    pub const fn enable_clock(self) -> Self {
        Self(self.0 | Self::CLOCK_ENABLE)
    }
    /// Disable clock for Serial Peripheral Interface peripheral.
    #[inline]
    pub const fn disable_clock(self) -> Self {
        Self(self.0 & !Self::CLOCK_ENABLE)
    }
    /// Check if clock for Serial Peripheral Interface peripheral is enabled.
    #[inline]
    pub const fn is_clock_enabled(self) -> bool {
        self.0 & Self::CLOCK_ENABLE != 0
    }
    /// Set clock source for Serial Peripheral Interface peripheral.
    #[inline]
    pub const fn set_clock_source(self, val: SpiClockSource) -> Self {
        Self((self.0 & !Self::CLOCK_SELECT) | ((val as u32) << 9))
    }
    /// Get clock source for Serial Peripheral Interface peripheral.
    #[inline]
    pub const fn clock_source(self) -> SpiClockSource {
        match (self.0 & Self::CLOCK_SELECT) >> 9 {
            0 => SpiClockSource::MuxPll160M,
            1 => SpiClockSource::Xclk,
            _ => unreachable!(),
        }
    }
}

/// Pulse Width Modulation configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PwmConfig(u32);

impl PwmConfig {
    const SIGNAL_0_SELECT: u32 = 1 << 0;
    const SIGNAL_1_SELECT: u32 = 1 << 1;

    /// Set source for signal group 0.
    #[inline]
    pub const fn set_signal_0(self, val: PwmSignal0) -> Self {
        Self((self.0 & !Self::SIGNAL_0_SELECT) | (val as u32))
    }
    /// Get source for signal group 0.
    #[inline]
    pub const fn signal_0(self) -> PwmSignal0 {
        match self.0 & Self::SIGNAL_0_SELECT {
            0 => PwmSignal0::SingleEnd,
            1 => PwmSignal0::DifferentialEnd,
            _ => unreachable!(),
        }
    }
    /// Set source for signal group 1.
    #[inline]
    pub const fn set_signal_1(self, val: PwmSignal1) -> Self {
        Self((self.0 & !Self::SIGNAL_1_SELECT) | ((val as u32) << 1))
    }
    /// Get source for signal group 1.
    #[inline]
    pub const fn signal_1(self) -> PwmSignal1 {
        match (self.0 & Self::SIGNAL_1_SELECT) >> 1 {
            0 => PwmSignal1::SingleEnd,
            1 => PwmSignal1::BrushlessDcMotor,
            _ => unreachable!(),
        }
    }
}

/// Signal group 0 source for Pulse Width Modulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwmSignal0 {
    /// Positive signals only for PWM0 channel 0, 1, 2 and 3.
    SingleEnd = 0,
    /// Both positive and negative signals for PWM0 channel 0, 1, 2 and 3.
    DifferentialEnd = 1,
}

/// Signal group 1 source for Pulse Width Modulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwmSignal1 {
    /// Positive signals only for PWM1 channel 0, 1, 2 and 3.
    SingleEnd = 0,
    /// Positive signals for PWM0 channel 0, 1, 2 and external break signal for PWM0.
    BrushlessDcMotor = 1,
}

/// Param configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ParamConfig(u32);

impl ParamConfig {
    const SPI_0_MASTER_MODE: u32 = 0x1 << 12;
    const SPI_1_MASTER_MODE: u32 = 0x1 << 27;

    /// Set mode for Serial Peripheral Interface.
    #[inline]
    pub const fn set_spi_mode<const I: usize>(self, mode: SpiMode) -> Self {
        match mode {
            SpiMode::Master => match I {
                0 => Self(self.0 | Self::SPI_0_MASTER_MODE),
                1 => Self(self.0 | Self::SPI_1_MASTER_MODE),
                _ => unreachable!(),
            },
            SpiMode::Slave => match I {
                0 => Self(self.0 & !Self::SPI_0_MASTER_MODE),
                1 => Self(self.0 & !Self::SPI_1_MASTER_MODE),
                _ => unreachable!(),
            },
        }
    }
    /// Get mode for Serial Peripheral Interface.
    #[inline]
    pub const fn spi_mode<const I: usize>(self) -> SpiMode {
        match I {
            0 => {
                if self.0 & Self::SPI_0_MASTER_MODE != 0 {
                    SpiMode::Master
                } else {
                    SpiMode::Slave
                }
            }
            1 => {
                if self.0 & Self::SPI_1_MASTER_MODE != 0 {
                    SpiMode::Master
                } else {
                    SpiMode::Slave
                }
            }
            _ => unreachable!(),
        }
    }
}

/// SDH configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct SdhConfig(u16);

impl SdhConfig {
    const SDH_CLK_EN: u16 = 0x1 << 13;
    const SDH_CLK_SEL: u16 = 0x1 << 12;
    const SDH_CLK_DIV_LEN: u16 = 0x7 << 9;

    /// Enable SDH clock.
    #[inline]
    pub const fn enable_sdh_clk(self) -> Self {
        Self((self.0 & !Self::SDH_CLK_EN) | (Self::SDH_CLK_EN & (1 << 13)))
    }
    /// Disable SDH clock.
    #[inline]
    pub const fn disable_sdh_clk(self) -> Self {
        Self((self.0 & !Self::SDH_CLK_EN) | (Self::SDH_CLK_EN & (0 << 13)))
    }
    // If SDH clock is enabled.
    #[inline]
    pub const fn is_sdh_clk_enabled(self) -> bool {
        (self.0 & Self::SDH_CLK_EN) >> 13 == 1
    }
    /// Set SDH clock select.
    #[inline]
    pub const fn set_sdh_clk_sel(self, val: u8) -> Self {
        Self((self.0 & !Self::SDH_CLK_SEL) | (Self::SDH_CLK_SEL & ((val as u16) << 12)))
    }
    /// Get SDH clock select.
    #[inline]
    pub const fn sdh_clk_sel(self) -> u8 {
        ((self.0 & Self::SDH_CLK_SEL) >> 12) as u8
    }
    /// Set SDH clock divider length.
    #[inline]
    pub const fn set_sdh_clk_div_len(self, val: u8) -> Self {
        Self((self.0 & !Self::SDH_CLK_DIV_LEN) | (Self::SDH_CLK_DIV_LEN & ((val as u16) << 9)))
    }
    /// Get SDH clock divider length.
    #[inline]
    pub const fn sdh_clk_div_len(self) -> u8 {
        ((self.0 & Self::SDH_CLK_DIV_LEN) >> 9) as u8
    }
}

/// Mode for Serial Peripheral Interface Bus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiMode {
    Master = 0,
    Slave = 1,
}

/// Clock generation configuration register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockConfig0(u32);

impl ClockConfig0 {
    // const CCI: u32 = 0x1 << 4;
    const DMA: u32 = 0x1 << 3;
    // const SEC: u32 = 0x1 << 2;
    // const SDU: u32 = 0x1 << 1;
    // const CPU: u32 = 0x1;

    /// Enable clock gate for Direct Memory Access controller.
    #[inline]
    pub const fn enable_dma(self) -> Self {
        Self(self.0 | Self::DMA)
    }
    /// Disable clock gate for Direct Memory Access controller.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        Self(self.0 & !Self::DMA)
    }
    /// Check if clock gate for Direct Memory Access controller is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        (self.0 & Self::DMA) != 0
    }
    // TODO: implment left fields.
}

/// Clock generation configuration register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockConfig1(u32);

impl ClockConfig1 {
    const DMA0: u32 = 0x1 << 12;
    const UART0: u32 = 0x1 << 16;
    const UART1: u32 = 0x1 << 17;
    const I2C: u32 = 0x1 << 19;
    const PWM: u32 = 0x1 << 20;
    const DMA2: u32 = 0x1 << 24;
    const UART2: u32 = 0x1 << 26;
    const LZ4D: u32 = 0x1 << 29;

    /// Enable clock gate for Direct Memory Access controller 0.
    #[inline]
    pub const fn enable_dma<const I: usize>(self) -> Self {
        match I {
            0 => Self(self.0 | Self::DMA0),
            1 => self,
            2 => Self(self.0 | Self::DMA2),
            _ => unreachable!(),
        }
    }
    /// Disable clock gate for Direct Memory Access controller 0.
    #[inline]
    pub const fn disable_dma<const I: usize>(self) -> Self {
        match I {
            0 => Self(self.0 & !Self::DMA0),
            1 => self,
            2 => Self(self.0 & !Self::DMA2),
            _ => unreachable!(),
        }
    }
    /// Check if clock gate for Direct Memory Access controller 0 is enabled.
    #[inline]
    pub const fn is_dma_enabled<const I: usize>(self) -> bool {
        match I {
            0 => (self.0 & Self::DMA0) != 0,
            1 => true,
            2 => (self.0 & Self::DMA2) != 0,
            _ => unreachable!(),
        }
    }
    /// Enable clock gate for Universal Asynchronous Receiver/Transmitter peripheral.
    #[inline]
    pub const fn enable_uart<const I: usize>(self) -> Self {
        let val = match I {
            0 => Self::UART0,
            1 => Self::UART1,
            2 => Self::UART2,
            _ => unreachable!(),
        };
        Self(self.0 | val)
    }
    /// Disable clock gate for Universal Asynchronous Receiver/Transmitter peripheral.
    #[inline]
    pub const fn disable_uart<const I: usize>(self) -> Self {
        let val = match I {
            0 => Self::UART0,
            1 => Self::UART1,
            2 => Self::UART2,
            _ => unreachable!(),
        };
        Self(self.0 & !val)
    }
    /// Check if clock gate for Universal Asynchronous Receiver/Transmitter is enabled.
    #[inline]
    pub const fn is_uart_enabled<const I: usize>(self) -> bool {
        let val = match I {
            0 => Self::UART0,
            1 => Self::UART1,
            2 => Self::UART2,
            _ => unreachable!(),
        };
        self.0 & val != 0
    }
    /// Enable clock gate for Inter-Integrated Circuit peripheral.
    #[inline]
    pub const fn enable_i2c(self) -> Self {
        Self(self.0 | Self::I2C)
    }
    /// Disable clock gate for Inter-Integrated Circuit peripheral.
    #[inline]
    pub const fn disable_i2c(self) -> Self {
        Self(self.0 & !Self::I2C)
    }
    /// Check if clock gate for Inter-Integrated Circuit is enabled.
    #[inline]
    pub const fn is_i2c_enabled(self) -> bool {
        self.0 & Self::I2C != 0
    }
    /// Enable clock gate for Pulse Width Modulation peripheral.
    #[inline]
    pub const fn enable_pwm(self) -> Self {
        Self(self.0 | Self::PWM)
    }
    /// Disable clock gate for Pulse Width Modulation peripheral.
    #[inline]
    pub const fn disable_pwm(self) -> Self {
        Self(self.0 & !Self::PWM)
    }
    /// Check if clock gate for Pulse Width Modulation is enabled.
    #[inline]
    pub const fn is_pwm_enabled(self) -> bool {
        self.0 & Self::PWM != 0
    }
    /// Enable clock gate for hardware LZ4 decompressor peripheral.
    #[inline]
    pub const fn enable_lz4d(self) -> Self {
        Self(self.0 | Self::LZ4D)
    }
    /// Disable clock gate for hardware LZ4 decompressor peripheral.
    #[inline]
    pub const fn disable_lz4d(self) -> Self {
        Self(self.0 & !Self::LZ4D)
    }
    /// Check if clock gate for hardware LZ4 decompressor is enabled.
    #[inline]
    pub const fn is_lz4d_enabled(self) -> bool {
        self.0 & Self::LZ4D != 0
    }
}

/// Clock generation configuration register 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockConfig2(u32);

impl ClockConfig2 {
    // TODO
}

/// Clock generation configuration register 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockConfig3(u32);

impl ClockConfig3 {
    // TODO
}

/// Generic Purpose Input/Output Configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpioConfig(u32);

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
    #[allow(unused)]
    pub(crate) const RESET_VALUE: Self = Self(0x0040_0b02);
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

/// Ldo12uhs configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Ldo12uhsConfig(u32);

impl Ldo12uhsConfig {
    const POWER: u32 = 0x1 << 0;
    const VOUT_SEL: u32 = 0xf << 20;

    /// Power up LDO12UHS.
    #[inline]
    pub const fn power_up(self) -> Self {
        Self(self.0 | Self::POWER)
    }
    /// Power down LDO12UHS.
    #[inline]
    pub const fn power_down(self) -> Self {
        Self(self.0 & !Self::POWER)
    }
    /// Check if LDO12UHS is powered up.
    #[inline]
    pub const fn is_powered_up(self) -> bool {
        self.0 & Self::POWER != 0
    }
    /// Set output voltage of LDO12UHS.
    #[inline]
    pub const fn set_output_voltage(self, val: u8) -> Self {
        Self((self.0 & !Self::VOUT_SEL) | ((val as u32) << 20))
    }
    /// Get output voltage of LDO12UHS.
    #[inline]
    pub const fn get_output_voltage(self) -> u8 {
        ((self.0 & Self::VOUT_SEL) >> 20) as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::glb::v2::SpiClockSource;

    use super::{
        Drive, Function, GpioConfig, I2cClockSource, I2cConfig, InterruptMode, Mode, Pull,
        PwmConfig, PwmSignal0, PwmSignal1, RegisterBlock, SdhConfig, SpiConfig, UartConfig,
        UartMuxGroup, UartSignal,
    };
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, uart_config), 0x150);
        assert_eq!(offset_of!(RegisterBlock, uart_mux_group), 0x154);
        assert_eq!(offset_of!(RegisterBlock, i2c_config), 0x180);
        assert_eq!(offset_of!(RegisterBlock, spi_config), 0x1b0);
        assert_eq!(offset_of!(RegisterBlock, pwm_config), 0x1d0);
        assert_eq!(offset_of!(RegisterBlock, sdh_config), 0x430);
        assert_eq!(offset_of!(RegisterBlock, param_config), 0x510);
        assert_eq!(offset_of!(RegisterBlock, clock_config_0), 0x580);
        assert_eq!(offset_of!(RegisterBlock, clock_config_1), 0x584);
        assert_eq!(offset_of!(RegisterBlock, clock_config_2), 0x588);
        assert_eq!(offset_of!(RegisterBlock, clock_config_3), 0x58c);
        assert_eq!(offset_of!(RegisterBlock, ldo12uhs_config), 0x6d0);
        assert_eq!(offset_of!(RegisterBlock, gpio_config), 0x8c4);
        assert_eq!(offset_of!(RegisterBlock, gpio_input), 0xac4);
        assert_eq!(offset_of!(RegisterBlock, gpio_output), 0xae4);
        assert_eq!(offset_of!(RegisterBlock, gpio_set), 0xaec);
        assert_eq!(offset_of!(RegisterBlock, gpio_clear), 0xaf4);
    }

    #[test]
    fn struct_gpio_config_functions() {
        let mut val = GpioConfig(0x0);

        val = val.enable_input();
        assert_eq!(val.0, 0x00000001);
        assert!(val.is_input_enabled());
        val = val.disable_input();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_input_enabled());

        val = val.enable_schmitt();
        assert_eq!(val.0, 0x00000002);
        assert!(val.is_schmitt_enabled());
        val = val.disable_schmitt();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_schmitt_enabled());

        val = val.enable_output();
        assert_eq!(val.0, 0x00000040);
        assert!(val.is_output_enabled());
        val = val.disable_output();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_output_enabled());

        val = val.mask_interrupt();
        assert_eq!(val.0, 0x00400000);
        assert!(val.is_interrupt_masked());
        val = val.unmask_interrupt();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_interrupt_masked());

        assert!(GpioConfig(0x01000000).output());
        assert!(!GpioConfig(0x00000000).output());

        assert!(GpioConfig(0x10000000).input());
        assert!(!GpioConfig(0x00000000).input());

        assert!(GpioConfig(0x00200000).has_interrupt());
        assert!(!GpioConfig(0x00000000).has_interrupt());

        assert_eq!(GpioConfig(0x0).set(), GpioConfig(0x02000000));
        assert_eq!(GpioConfig(0x0).clear(), GpioConfig(0x04000000));

        assert_eq!(GpioConfig(0x0).clear_interrupt(), GpioConfig(0x00100000));

        let mut val = GpioConfig(0x0);
        val = val.set_drive(Drive::Drive0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.drive(), Drive::Drive0);
        val = val.set_drive(Drive::Drive1);
        assert_eq!(val.0, 0x00000004);
        assert_eq!(val.drive(), Drive::Drive1);
        val = val.set_drive(Drive::Drive2);
        assert_eq!(val.0, 0x00000008);
        assert_eq!(val.drive(), Drive::Drive2);
        val = val.set_drive(Drive::Drive3);
        assert_eq!(val.0, 0x0000000c);
        assert_eq!(val.drive(), Drive::Drive3);

        let mut val = GpioConfig(0x0);
        val = val.set_function(Function::Gpio);
        assert_eq!(val.0, 0x00000b00);
        assert_eq!(val.function(), Function::Gpio);

        let mut val = GpioConfig(0x0);
        val = val.set_interrupt_mode(InterruptMode::AsyncFallingEdge);
        assert_eq!(val.0, 0x00080000);
        assert_eq!(val.interrupt_mode(), InterruptMode::AsyncFallingEdge);

        let mut val = GpioConfig(0x0);
        val = val.set_mode(Mode::Normal);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.mode(), Mode::Normal);
        val = val.set_mode(Mode::SetClear);
        assert_eq!(val.0, 0x40000000);
        assert_eq!(val.mode(), Mode::SetClear);
        val = val.set_mode(Mode::Programmable);
        assert_eq!(val.0, 0x80000000);
        assert_eq!(val.mode(), Mode::Programmable);
        val = val.set_mode(Mode::BufferedSetClear);
        assert_eq!(val.0, 0xc0000000);
        assert_eq!(val.mode(), Mode::BufferedSetClear);

        let mut val = GpioConfig(0x0);
        val = val.set_pull(Pull::None);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.pull(), Pull::None);
        val = val.set_pull(Pull::Up);
        assert_eq!(val.0, 0x00000010);
        assert_eq!(val.pull(), Pull::Up);
        val = val.set_pull(Pull::Down);
        assert_eq!(val.0, 0x00000020);
        assert_eq!(val.pull(), Pull::Down);
    }

    #[test]
    fn struct_uart_config_functions() {
        let mut config = UartConfig(0x0);
        config = config.set_clock_divide(1);
        assert_eq!(config.0, 1);
        assert_eq!(config.clock_divide(), 1);

        config = UartConfig(0x0);
        config = config.set_clock_divide(0x0F);
        assert_eq!(config.0, 0x00000007);
        assert_eq!(config.clock_divide(), 0x07);

        config = UartConfig(0x8);
        config = config.set_clock_divide(1);
        assert_eq!(config.0, 9);
        assert_eq!(config.clock_divide(), 1);

        config = UartConfig(0x10);
        config = config.set_clock_divide(0x0F);
        assert_eq!(config.0, 0x00000017);
        assert_eq!(config.clock_divide(), 0x07);

        config = UartConfig(0x0);
        config = config.enable_clock();
        assert_eq!(config.0, 0x00000010);
        assert!(config.is_clock_enabled());

        config = config.disable_clock();
        assert_eq!(config.0, 0x0);
        assert!(!config.is_clock_enabled());
    }

    #[test]
    fn struct_uart_mux_group_functions() {
        let mut val = UartMuxGroup(0x0);
        val = val.set_signal(0, UartSignal::Rts0);
        assert_eq!(val.0, UartSignal::Rts0 as u32);
        assert_eq!(val.signal(0), UartSignal::Rts0);

        val = UartMuxGroup(0x0);
        val = val.set_signal(1, UartSignal::Rxd2);
        assert_eq!(val.0, (UartSignal::Rxd2 as u32) << 4);
        assert_eq!(val.signal(1), UartSignal::Rxd2);

        val = UartMuxGroup(0xFF);
        val = val.set_signal(2, UartSignal::Txd2);
        assert_eq!(val.0, 0xFF | (UartSignal::Txd2 as u32) << 8);
        assert_eq!(val.signal(2), UartSignal::Txd2);
    }

    #[test]
    fn struct_i2c_config_functions() {
        let mut config = I2cConfig(0x0);

        config = config.set_clock_divide(1);
        assert_eq!(config.0, 0x00010000);
        assert_eq!(config.clock_divide(), 1);

        config = I2cConfig(0x0);
        config = config.set_clock_divide(0xff);
        assert_eq!(config.0, 0x00FF0000);
        assert_eq!(config.clock_divide(), 0xff);

        config = I2cConfig(0x8);
        config = config.set_clock_divide(1);
        assert_eq!(config.0, 0x00010008);
        assert_eq!(config.clock_divide(), 1);

        config = I2cConfig(0x10);
        config = config.set_clock_divide(0x0F);
        assert_eq!(config.0, 0x000F0010);
        assert_eq!(config.clock_divide(), 0x0F);

        config = I2cConfig(0x0);
        config = config.enable_clock();
        assert_eq!(config.0, 0x01000000);
        assert!(config.is_clock_enabled());

        config = config.disable_clock();
        assert_eq!(config.0, 0x00000000);
        assert!(!config.is_clock_enabled());

        config = I2cConfig(0x0);
        config = config.set_clock_source(I2cClockSource::Bclk);
        assert_eq!(config.0, 0x00000000);
        assert_eq!(config.clock_source(), I2cClockSource::Bclk);

        config = I2cConfig(0x0);
        config = config.set_clock_source(I2cClockSource::Xclk);
        assert_eq!(config.0, 0x02000000);
        assert_eq!(config.clock_source(), I2cClockSource::Xclk);
    }

    #[test]
    fn struct_spi_config_functions() {
        let mut config = SpiConfig(0x0);

        config = config.set_clock_divide(1);
        assert_eq!(config.0, 0x00000001);
        assert_eq!(config.clock_divide(), 1);

        config = SpiConfig(0x0);
        config = config.set_clock_divide(0xff);
        assert_eq!(config.0, 0x000000ff);
        assert_eq!(config.clock_divide(), 0xff);

        config = SpiConfig(0x0);
        config = config.set_clock_divide(0x0F);
        assert_eq!(config.0, 0x0000000f);
        assert_eq!(config.clock_divide(), 0x0f);

        config = SpiConfig(0x0);
        config = config.enable_clock();
        assert_eq!(config.0, 0x00000100);
        assert!(config.is_clock_enabled());

        config = config.disable_clock();
        assert_eq!(config.0, 0x00000000);
        assert!(!config.is_clock_enabled());

        config = SpiConfig(0x0);
        config = config.set_clock_source(SpiClockSource::MuxPll160M);
        assert_eq!(config.0, 0x00000000);
        assert_eq!(config.clock_source(), SpiClockSource::MuxPll160M);

        config = SpiConfig(0x0);
        config = config.set_clock_source(SpiClockSource::Xclk);
        assert_eq!(config.0, 0x00000200);
        assert_eq!(config.clock_source(), SpiClockSource::Xclk);
    }

    #[test]
    fn struct_pwm_config_functions() {
        let mut config = PwmConfig(0x0);

        config = config.set_signal_0(PwmSignal0::SingleEnd);
        assert_eq!(config.0, 0x00000000);
        assert_eq!(config.signal_0(), PwmSignal0::SingleEnd);

        config = PwmConfig(0x0);
        config = config.set_signal_0(PwmSignal0::DifferentialEnd);
        assert_eq!(config.0, 0x00000001);
        assert_eq!(config.signal_0(), PwmSignal0::DifferentialEnd);

        config = PwmConfig(0x0);
        config = config.set_signal_1(PwmSignal1::SingleEnd);
        assert_eq!(config.0, 0x00000000);
        assert_eq!(config.signal_1(), PwmSignal1::SingleEnd);

        config = PwmConfig(0x0);
        config = config.set_signal_1(PwmSignal1::BrushlessDcMotor);
        assert_eq!(config.0, 0x00000002);
        assert_eq!(config.signal_1(), PwmSignal1::BrushlessDcMotor);
    }

    #[test]
    fn struct_sdh_config_functions() {
        let mut val = SdhConfig(0x0);
        val = val.enable_sdh_clk();
        assert!(val.is_sdh_clk_enabled());
        assert_eq!(val.0, 0x2000);
        val = val.disable_sdh_clk();
        assert!(!val.is_sdh_clk_enabled());
        assert_eq!(val.0, 0x0000);

        val = val.set_sdh_clk_sel(1);
        assert_eq!(val.sdh_clk_sel(), 1);
        assert_eq!(val.0, 0x1000);

        val = SdhConfig(0x0);
        val = val.set_sdh_clk_div_len(0x7);
        assert_eq!(val.sdh_clk_div_len(), 0x7);
        assert_eq!(val.0, 0x0E00);
    }
}
