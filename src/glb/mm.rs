//! Multi-media subsystem global peripheral.

use volatile_register::RW;

/// Multi-media subsystem global peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    /// CPU clock configuration register 0.
    pub cpu_config_0: RW<CpuConfig0>,
    /// CPU clock configuration register 1.
    pub cpu_config_1: RW<CpuConfig1>,
}

/// CPU clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuClockSource {
    /// 240-MHz multiplexer PLL.
    MuxPll240M = 0,
    /// 320-MHz multiplexer PLL.
    MuxPll320M = 1,
    /// 400-MHz CPU multiplexer PLL.
    CpuPll400M = 2,
}

/// CPU root clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuRootClockSource {
    /// Crystal oscillator clock.
    Xclk = 0,
    /// CPU clock multiplexer PLL.
    Pll = 1,
}

/// CPU clock configuration register 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CpuConfig0(u32);

impl CpuConfig0 {
    const CPU_CLOCK_ENABLE: u32 = 0x1 << 1;
    const CPU_CLOCK_SELECT: u32 = 0x3 << 8;
    const CPU_ROOT_CLOCK_SELECT: u32 = 0x1 << 11;

    /// Enable clock for CPU.
    #[inline]
    pub const fn enable_cpu_clock(self) -> Self {
        Self(self.0 | Self::CPU_CLOCK_ENABLE)
    }
    /// Disable clock for CPU.
    #[inline]
    pub const fn disable_cpu_clock(self) -> Self {
        Self(self.0 & !Self::CPU_CLOCK_ENABLE)
    }
    /// Check if clock for CPU is enabled.
    #[inline]
    pub const fn is_cpu_clock_enabled(self) -> bool {
        self.0 & Self::CPU_CLOCK_ENABLE != 0
    }
    /// Set clock source for CPU.
    #[inline]
    pub const fn set_cpu_clock_source(self, val: CpuClockSource) -> Self {
        Self((self.0 & !Self::CPU_CLOCK_SELECT) | ((val as u32) << 8))
    }
    /// Get clock source for CPU.
    #[inline]
    pub const fn cpu_clock_source(self) -> CpuClockSource {
        match (self.0 & Self::CPU_CLOCK_SELECT) >> 25 {
            0 => CpuClockSource::MuxPll240M,
            1 => CpuClockSource::MuxPll320M,
            _ => CpuClockSource::CpuPll400M,
        }
    }
    /// Set source for CPU root clock.
    #[inline]
    pub const fn set_cpu_root_clock_source(self, val: CpuRootClockSource) -> Self {
        Self((self.0 & !Self::CPU_ROOT_CLOCK_SELECT) | ((val as u32) << 8))
    }
    /// Get source for CPU root clock.
    #[inline]
    pub const fn cpu_root_clock_source(self) -> CpuRootClockSource {
        match (self.0 & Self::CPU_ROOT_CLOCK_SELECT) >> 8 {
            0 => CpuRootClockSource::Xclk,
            1 => CpuRootClockSource::Pll,
            _ => unreachable!(),
        }
    }
}

/// CPU clock configuration register 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct CpuConfig1(u32);

impl CpuConfig1 {
    const CPU_CLOCK_DIVIDE: u32 = 0xff << 0;

    /// Set CPU clock divide factor.
    #[inline]
    pub const fn set_cpu_clock_divide(self, val: u8) -> Self {
        Self((self.0 & !Self::CPU_CLOCK_DIVIDE) | ((val as u32) << 0))
    }
    /// Get CPU clock divide factor.
    #[inline]
    pub const fn cpu_clock_divide(self) -> u8 {
        ((self.0 & Self::CPU_CLOCK_DIVIDE) >> 0) as u8
    }
}
