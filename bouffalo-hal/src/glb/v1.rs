//! Global configurations on BL602 and BL702 series.
use super::{Drive, Pull};
use volatile_register::{RO, RW, WO};

/// Global configuration registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Clock configuration register.
    pub clock_config0: RW<ClockConfig0>,
    _reserved0: [u8; 0xFC],
    /// Generic Purpose Input/Output configuration register.
    pub gpio_config: [RW<GpioConfig>; 16],
    _reserved1: [u8; 0x40],
    /// Read value from Generic Purpose Input/Output pads.
    pub gpio_input_value: RO<u32>,
    _reserved2: [u8; 0x4],
    /// Write value to Generic Purpose Input/Output pads.
    pub gpio_output_value: RW<u32>,
    _reserved3: [u8; 0x4],
    /// Enable output function of Generic Purpose Input/Output pads.
    pub gpio_output_enable: RW<u32>,
    /// Interrupt mask of Generic Purpose Input/Output pads.
    pub gpio_interrupt_mask: RW<u32>,
    _reserved4: [u8; 0x10],
    /// Interrupt state of Generic Purpose Input/Output pads.
    pub gpio_interrupt_state: RO<u32>,
    _reserved5: [u8; 0x4],
    /// Clear interrupt state of Generic Purpose Input/Output pads.
    pub gpio_interrupt_clear: WO<u32>,
    _reserved6: [u8; 0xc],
    /// Generic Purpose Input/Output interrupt mode register.
    pub gpio_interrupt_mode: [RW<GpioInterruptMode>; 16],
}

/// Root clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GlbRootClk {
    Rc32M = 0,
    Xtal = 1,
    Pll = 2,
}

/// Clock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ClockConfig0(u32);

impl ClockConfig0 {
    // const GLB_ID: u32 = 0xF << 28;
    // const GLB_CHIP_RDY: u32 = 0x1 << 27;
    // const GLB_FCLK_SW_STATE: u32 = 0x7 << 24;
    const GLB_REG_BCLK_DIV: u32 = 0xFF << 16;
    const GLB_REG_HCLK_DIV: u32 = 0xFF << 8;
    const GLB_HBN_ROOT_CLK_SEL: u32 = 0x3 << 6;
    // const GLB_REG_PLL_SEL: u32 = 0x3 << 4;
    const GLB_REG_BCLK_EN: u32 = 0x1 << 3;
    const GLB_REG_HCLK_EN: u32 = 0x1 << 2;
    const GLB_REG_FCLK_EN: u32 = 0x1 << 1;
    const GLB_REG_PLL_EN: u32 = 0x1 << 0;

    /// Enable bclk.
    #[inline]
    pub const fn enable_bclk(self) -> Self {
        Self(self.0 | Self::GLB_REG_BCLK_EN)
    }
    /// Disable bclk.
    #[inline]
    pub const fn disable_bclk(self) -> Self {
        Self(self.0 & !Self::GLB_REG_BCLK_EN)
    }
    /// Check if bclk is enabled.
    #[inline]
    pub const fn is_bclk_enabled(self) -> bool {
        self.0 & Self::GLB_REG_BCLK_EN != 0
    }
    /// Enable hclk (cpu clock).
    #[inline]
    pub const fn enable_hclk(self) -> Self {
        Self(self.0 | Self::GLB_REG_HCLK_EN)
    }
    /// Disable hclk (cpu clock).
    #[inline]
    pub const fn disable_hclk(self) -> Self {
        Self(self.0 & !Self::GLB_REG_HCLK_EN)
    }
    /// Check if hclk (cpu clock) is enabled.
    #[inline]
    pub const fn is_hclk_enabled(self) -> bool {
        self.0 & Self::GLB_REG_HCLK_EN != 0
    }
    /// Enable fclk.
    #[inline]
    pub const fn enable_fclk(self) -> Self {
        Self(self.0 | Self::GLB_REG_FCLK_EN)
    }
    /// Disable fclk.
    #[inline]
    pub const fn disable_fclk(self) -> Self {
        Self(self.0 & !Self::GLB_REG_FCLK_EN)
    }
    /// Check if fclk is enabled.
    #[inline]
    pub const fn is_fclk_enabled(self) -> bool {
        self.0 & Self::GLB_REG_FCLK_EN != 0
    }
    /// Enable pll.
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::GLB_REG_PLL_EN)
    }
    /// Disable pll.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::GLB_REG_PLL_EN)
    }
    /// Check if pll is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        self.0 & Self::GLB_REG_PLL_EN != 0
    }
    /// Get bclk divider.
    #[inline]
    pub const fn bclk_div(self) -> u8 {
        ((self.0 & Self::GLB_REG_BCLK_DIV) >> 16) as u8
    }
    /// Set bclk divider.
    #[inline]
    pub const fn set_bclk_div(self, val: u8) -> Self {
        Self((self.0 & !Self::GLB_REG_BCLK_DIV) | ((val as u32) << 16))
    }
    /// Get hclk (cpu clock) divider.
    #[inline]
    pub const fn hclk_div(self) -> u8 {
        ((self.0 & Self::GLB_REG_HCLK_DIV) >> 8) as u8
    }
    /// Set hclk (cpu clock) divider.
    #[inline]
    pub const fn set_hclk_div(self, val: u8) -> Self {
        Self((self.0 & !Self::GLB_REG_HCLK_DIV) | ((val as u32) << 8))
    }
    /// Get root clock source in hbn mode.
    #[inline]
    pub const fn hbn_root_clk_sel(self) -> GlbRootClk {
        match ((self.0 & Self::GLB_HBN_ROOT_CLK_SEL) >> 6) as u8 {
            0 => GlbRootClk::Rc32M,
            1 => GlbRootClk::Xtal,
            2 | 3 => GlbRootClk::Pll,
            _ => unreachable!(),
        }
    }
    /// Set root clock source in hbn mode.
    #[inline]
    pub const fn set_hbn_root_clk_sel(self, val: GlbRootClk) -> Self {
        Self((self.0 & !Self::GLB_HBN_ROOT_CLK_SEL) | ((val as u32) << 6))
    }
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
    const FUNCTION: u32 = 0x1f << 8;

    /// Enable input function of current pin.
    #[inline]
    pub const fn enable_input(self, idx: usize) -> Self {
        Self(self.0 | (Self::INPUT_ENABLE << (idx * 16)))
    }
    /// Disable input function of current pin.
    #[inline]
    pub const fn disable_input(self, idx: usize) -> Self {
        Self(self.0 & !(Self::INPUT_ENABLE << (idx * 16)))
    }
    /// Check if input function of current pin is enabled.
    #[inline]
    pub const fn is_input_enabled(self, idx: usize) -> bool {
        self.0 & (Self::INPUT_ENABLE << (idx * 16)) != 0
    }
    /// Enable Schmitt trigger function of current pin.
    #[inline]
    pub const fn enable_schmitt(self, idx: usize) -> Self {
        Self(self.0 | (Self::SCHMITT << (idx * 16)))
    }
    /// Disable Schmitt trigger function of current pin.
    #[inline]
    pub const fn disable_schmitt(self, idx: usize) -> Self {
        Self(self.0 & !(Self::SCHMITT << (idx * 16)))
    }
    /// Check if Schmitt trigger function of current pin is enabled.
    #[inline]
    pub const fn is_schmitt_enabled(self, idx: usize) -> bool {
        self.0 & (Self::SCHMITT << (idx * 16)) != 0
    }
    /// Get drive strength of current pin.
    #[inline]
    pub const fn drive(self, idx: usize) -> Drive {
        match ((self.0 >> (idx * 16)) & Self::DRIVE) >> 2 {
            0 => Drive::Drive0,
            1 => Drive::Drive1,
            2 => Drive::Drive2,
            3 => Drive::Drive3,
            _ => unreachable!(),
        }
    }
    /// Set drive strength of current pin.
    #[inline]
    pub const fn set_drive(self, idx: usize, val: Drive) -> Self {
        Self((self.0 & !(Self::DRIVE << (idx * 16))) | ((val as u32) << (2 + (idx * 16))))
    }
    /// Get pull direction of current pin.
    pub const fn pull(self, idx: usize) -> Pull {
        match ((self.0 >> (idx * 16)) & Self::PULL) >> 4 {
            0 => Pull::None,
            1 => Pull::Up,
            2 => Pull::Down,
            _ => unreachable!(),
        }
    }
    /// Set pull direction of current pin.
    #[inline]
    pub const fn set_pull(self, idx: usize, val: Pull) -> Self {
        Self((self.0 & !(Self::PULL << (idx * 16))) | ((val as u32) << (4 + (idx * 16))))
    }
    /// Set function of current pin.
    #[inline]
    pub const fn set_function(self, idx: usize, val: Function) -> Self {
        Self((self.0 & !(Self::FUNCTION << (idx * 16))) | ((val as u32) << (8 + (idx * 16))))
    }
    /// Get function of current pin.
    #[inline]
    pub const fn function(self, idx: usize) -> Function {
        match ((self.0 >> (idx * 16)) & Self::FUNCTION) >> 8 {
            0 => Function::ClkOut,
            1 => Function::BtCoexist,
            2 => Function::Flash,
            3 => Function::I2s,
            4 => Function::Spi,
            6 => Function::I2c,
            7 => Function::Uart,
            8 => Function::Pwm,
            9 => Function::Cam,
            10 => Function::Analog,
            11 => Function::Gpio,
            12 => Function::RfTest,
            13 => Function::Scan,
            14 => Function::E21Jtag,
            15 => Function::Debug,
            16 => Function::ExternalPa,
            17 => Function::UsbTranceiver,
            18 => Function::UsbController,
            19 => Function::EMac,
            20 => Function::Qdec,
            21 => Function::KeyScanIn,
            22 => Function::KeyScanDrive,
            23 => Function::CamMisc,
            _ => unreachable!(),
        }
    }
}

/// Generic Purpose Input/Output interrupt mode register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GpioInterruptMode(u32);

impl GpioInterruptMode {
    const INTERRUPT_MODE: u32 = 0x7;

    /// Set interrupt mode of current pin.
    #[inline]
    pub const fn set_interrupt_mode(self, idx: usize, val: InterruptMode) -> Self {
        Self((self.0 & !(Self::INTERRUPT_MODE << (idx * 3))) | ((val as u32) << (idx * 3)))
    }
    /// Get interrupt mode of current pin.
    #[inline]
    pub const fn interrupt_mode(self, idx: usize) -> InterruptMode {
        match (self.0 >> (idx * 3)) & Self::INTERRUPT_MODE {
            0 => InterruptMode::SyncFallingEdge,
            1 => InterruptMode::SyncRisingEdge,
            2 => InterruptMode::SyncLowLevel,
            3 => InterruptMode::SyncHighLevel,
            4 => InterruptMode::AsyncFallingEdge,
            5 => InterruptMode::AsyncRisingEdge,
            6 => InterruptMode::AsyncLowLevel,
            7 => InterruptMode::AsyncHighLevel,
            _ => unreachable!(),
        }
    }
}

/// Pin alternate function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Function {
    ClkOut = 0,
    BtCoexist = 1,
    Flash = 2,
    I2s = 3,
    Spi = 4,
    I2c = 6,
    Uart = 7,
    Pwm = 8,
    Cam = 9,
    Analog = 10,
    Gpio = 11,
    RfTest = 12,
    Scan = 13,
    E21Jtag = 14,
    Debug = 15,
    ExternalPa = 16,
    UsbTranceiver = 17,
    UsbController = 18,
    EMac = 19,
    Qdec = 20,
    KeyScanIn = 21,
    KeyScanDrive = 22,
    CamMisc = 23,
}

/// Pin interrupt mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InterruptMode {
    SyncFallingEdge = 0,
    SyncRisingEdge = 1,
    SyncLowLevel = 2,
    SyncHighLevel = 3,
    AsyncFallingEdge = 4,
    AsyncRisingEdge = 5,
    AsyncLowLevel = 6,
    AsyncHighLevel = 7,
}

// TODO unit tests
