//! Hibernation (deep-sleep) control peripheral.
use core::cell::UnsafeCell;

use volatile_register::{RO, RW, WO};

/// Hibernation control registers.
#[repr(C)]
pub struct RegisterBlock {
    /// todo: fill in all registers
    /// Miscellaneous control register
    pub control: RW<u32>,
    /// Low bits of hibernate time
    pub time_lo: RW<u32>,
    /// High bits of hibernate time
    pub time_hi: RW<u32>,
    /// Low bits of Real-Time Clock time
    pub rtc_time_lo: RO<u32>,
    /// High bits of Real-Time Clock time
    pub rtc_time_hi: RO<u32>,
    /// Hibernate interrupt contol
    pub interrupt_mode: RW<u32>,
    /// Hibernate interrupt state
    pub interrupt_state: RO<u32>,
    /// Clear hibernate interrupt
    pub interrupt_clear: WO<u32>,
    /// Passive infrared sensor configuration
    pub pir_config: RW<u32>,
    /// Passive infrared sensor voltage threshold
    pub pir_threshold: RW<u32>,
    /// Passive infrared sensor time interval
    pub pir_interval: RW<u32>,
    /// Brown-out reset function configuration
    pub bor_config: RW<u32>,
    /// Global hibernate configuration
    pub global: GLOBAL,
    /// Static Random-Access Memory hibernate control
    pub sram: RW<u32>,
    /// Always-on pad control register 0
    pub pad_control_0: RW<u32>,
    /// Always-on pad control register 1
    pub pad_control_1: RW<u32>,
    _reserved0: [u8; 448],
    /// 32-kHz internal RC oscillator control
    pub rc32k: RW<u32>,
    /// External crystal oscillator control
    pub xtal32k: RW<u32>,
    /// Real-Time Clock control and reset register 0
    pub rtc_control_0: RW<u32>,
    /// Real-Time Clock control and reset register 1
    pub rtc_control_1: RW<u32>,
}

/// Global hibernate configuration register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct GLOBAL(UnsafeCell<u32>);

/// Configuration structure for hibernation global register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Global(u32);

impl GLOBAL {
    /// Read global configuration.
    #[inline]
    pub fn read(&self) -> Global {
        Global(unsafe { self.0.get().read_volatile() })
    }
    /// Write global configuration.
    #[inline]
    pub fn write(&self, val: Global) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl Global {
    const ROOT_CLOCK_SOURCE_1: u32 = 1 << 0;
    const ROOT_CLOCK_SOURCE_2: u32 = 1 << 1;
    const UART_CLOCK_SOURCE_1: u32 = 1 << 2;
    const F32K_SELECT: u32 = 0x3 << 3;
    const RESET_EVENT: u32 = 0x3f << 7;
    const CLEAR_RESET_EVENT: u32 = 1 << 13;
    const UART_CLOCK_SOURCE_2: u32 = 1 << 15;

    /// Set root clock source 1.
    #[inline]
    pub const fn set_root_clock_1(self, val: RootClockSource1) -> Self {
        Self((self.0 & !Self::ROOT_CLOCK_SOURCE_1) | (val as u32))
    }
    /// Get root clock source 1.
    #[inline]
    pub const fn root_clock_1(self) -> RootClockSource1 {
        match (self.0 & Self::ROOT_CLOCK_SOURCE_1) >> 0 {
            0 => RootClockSource1::RC32M,
            1 => RootClockSource1::Xtal,
            _ => unreachable!(),
        }
    }
    /// Set root clock source 2.
    #[inline]
    pub const fn set_root_clock_2(self, val: RootClockSource2) -> Self {
        Self((self.0 & !Self::ROOT_CLOCK_SOURCE_2) | ((val as u32) << 1))
    }
    /// Get root clock source 2.
    #[inline]
    pub const fn root_clock_2(self) -> RootClockSource2 {
        match (self.0 & Self::ROOT_CLOCK_SOURCE_2) >> 1 {
            0 => RootClockSource2::Xclk,
            1 => RootClockSource2::Pllsel,
            _ => unreachable!(),
        }
    }
    /// Set f32k source.
    #[inline]
    pub const fn set_f32k_source(self, val: F32kSource) -> Self {
        Self((self.0 & !Self::F32K_SELECT) | ((val as u32) << 3))
    }
    /// Get f32k source.
    #[inline]
    pub const fn f32k_source(self) -> F32kSource {
        match (self.0 & Self::F32K_SELECT) >> 3 {
            0 => F32kSource::RC32K,
            1 => F32kSource::Xtal32K,
            2 => F32kSource::Dig32K,
            _ => unreachable!(),
        }
    }
    /// Clear reset event.
    #[inline]
    pub const fn clear_reset_event(self) -> Self {
        Self(self.0 | Self::CLEAR_RESET_EVENT)
    }
    /// Get reset event.
    #[inline]
    pub const fn reset_event(self) -> ResetEvent {
        match (self.0 & Self::RESET_EVENT) >> 7 {
            0 => ResetEvent::CpuM0,
            1 => ResetEvent::CpuLp,
            2 => ResetEvent::Bus1,
            3 => ResetEvent::Glb,
            4 => ResetEvent::Mix,
            5 => ResetEvent::Gpip,
            6 => ResetEvent::SecEng,
            7 => ResetEvent::TZ,
            8 => ResetEvent::Efuse,
            9 => ResetEvent::Dma,
            10 => ResetEvent::Psram,
            11 => ResetEvent::Usb,
            12 => ResetEvent::Emac,
            13 => ResetEvent::Audio,
            14 => ResetEvent::Dma2,
            15 => ResetEvent::Pds,
            16 => ResetEvent::Uart0,
            17 => ResetEvent::Uart1,
            18 => ResetEvent::Spi,
            19 => ResetEvent::I2c,
            20 => ResetEvent::Pwm,
            21 => ResetEvent::Timer,
            22 => ResetEvent::Irr,
            23 => ResetEvent::Uart2Can,
            24 => ResetEvent::I2s,
            25 => ResetEvent::Pdm,
            26 => ResetEvent::Wifi,
            27 => ResetEvent::Ble,
            28 => ResetEvent::CpuD0,
            29 => ResetEvent::Bus2,
            30 => ResetEvent::MmMisc,
            31 => ResetEvent::MmDma,
            32 => ResetEvent::Mm2ddma,
            33 => ResetEvent::MmUart,
            34 => ResetEvent::MmI2c,
            35 => ResetEvent::MmIpc,
            36 => ResetEvent::MmTimer,
            37 => ResetEvent::UhsCtrl,
            38 => ResetEvent::DispTsrc,
            39 => ResetEvent::Nr3dCtrl,
            40 => ResetEvent::Dvp2busA,
            41 => ResetEvent::Dvp2busB,
            42 => ResetEvent::Dvp2busC,
            43 => ResetEvent::Dvp2busD,
            44 => ResetEvent::Dvp2busE,
            45 => ResetEvent::Dvp2busF,
            46 => ResetEvent::Dvp2busG,
            47 => ResetEvent::Dvp2busH,
            48 => ResetEvent::Jdec,
            49 => ResetEvent::Blai,
            _ => unreachable!(),
        }
    }
    /// Set uart clock source.
    #[inline]
    pub const fn set_uart_clock_source(self, val: UartClockSource) -> Self {
        Self(
            (self.0 & !((Self::UART_CLOCK_SOURCE_1 << 13) | Self::UART_CLOCK_SOURCE_2))
                | ((val as u32) << 15),
        )
    }
    /// Get uart clock source.
    #[inline]
    pub const fn uart_clock_source(self) -> UartClockSource {
        match (self.0 & ((Self::UART_CLOCK_SOURCE_1 << 13) | Self::UART_CLOCK_SOURCE_2)) >> 15 {
            0 => UartClockSource::McuBclk,
            1 => UartClockSource::MuxPll160M,
            2 => UartClockSource::Xclk,
            _ => unreachable!(),
        }
    }
}

/// Root clock source 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootClockSource1 {
    /// Internal 32-MHz RC oscillator
    RC32M = 0,
    /// External crystal oscillator
    Xtal = 1,
}

/// Root clock source 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RootClockSource2 {
    /// External clock
    Xclk = 0,
    /// PLL select
    Pllsel = 1,
}

/// F32k clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum F32kSource {
    /// Internal 32-kHz RC oscillator
    RC32K = 0,
    /// External 32-kHz crystal oscillator
    Xtal32K = 1,
    /// Digital 32-kHz clock
    Dig32K = 2,
}

/// Uart clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UartClockSource {
    /// Microcontroller bus clock
    McuBclk = 0,
    /// 160-MHz mutiplexer PLL
    MuxPll160M = 1,
    /// External clock
    Xclk = 2,
}

/// Reset event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetEvent {
    CpuM0 = 0,
    CpuLp = 1,
    Bus1 = 2,
    Glb = 3,
    Mix = 4,
    Gpip = 5,
    SecEng = 6,
    TZ = 7,
    Efuse = 8,
    Dma = 9,
    Psram = 10,
    Usb = 11,
    Emac = 12,
    Audio = 13,
    Dma2 = 14,
    Pds = 15,
    Uart0 = 16,
    Uart1 = 17,
    Spi = 18,
    I2c = 19,
    Pwm = 20,
    Timer = 21,
    Irr = 22,
    Uart2Can = 23,
    I2s = 24,
    Pdm = 25,
    Wifi = 26,
    Ble = 27,
    CpuD0 = 28,
    Bus2 = 29,
    MmMisc = 30,
    MmDma = 31,
    Mm2ddma = 32,
    MmUart = 33,
    MmI2c = 34,
    MmIpc = 35,
    MmTimer = 36,
    UhsCtrl = 37,
    DispTsrc = 38,
    Nr3dCtrl = 39,
    Dvp2busA = 40,
    Dvp2busB = 41,
    Dvp2busC = 42,
    Dvp2busD = 43,
    Dvp2busE = 44,
    Dvp2busF = 45,
    Dvp2busG = 46,
    Dvp2busH = 47,
    Jdec = 48,
    Blai = 49,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, control), 0x00);
        assert_eq!(offset_of!(RegisterBlock, time_lo), 0x04);
        assert_eq!(offset_of!(RegisterBlock, time_hi), 0x08);
        assert_eq!(offset_of!(RegisterBlock, rtc_time_lo), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, rtc_time_hi), 0x10);
        assert_eq!(offset_of!(RegisterBlock, interrupt_mode), 0x14);
        assert_eq!(offset_of!(RegisterBlock, interrupt_state), 0x18);
        assert_eq!(offset_of!(RegisterBlock, interrupt_clear), 0x1c);
        assert_eq!(offset_of!(RegisterBlock, pir_config), 0x20);
        assert_eq!(offset_of!(RegisterBlock, pir_threshold), 0x24);
        assert_eq!(offset_of!(RegisterBlock, pir_interval), 0x28);
        assert_eq!(offset_of!(RegisterBlock, bor_config), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, global), 0x30);
        assert_eq!(offset_of!(RegisterBlock, sram), 0x34);
        assert_eq!(offset_of!(RegisterBlock, pad_control_0), 0x38);
        assert_eq!(offset_of!(RegisterBlock, pad_control_1), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, rc32k), 0x200);
        assert_eq!(offset_of!(RegisterBlock, xtal32k), 0x204);
        assert_eq!(offset_of!(RegisterBlock, rtc_control_0), 0x208);
        assert_eq!(offset_of!(RegisterBlock, rtc_control_1), 0x20c);
    }
}
