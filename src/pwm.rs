//! Pulse Width Modulation peripheral.
use core::cell::UnsafeCell;

/// Pulse width modulation registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt configuration.
    pub interrupt_config: INTERRUPT_CONFIG,
    /// control register group.
    pub group: [Group; 2],
}

/// Interrupt configuration register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct INTERRUPT_CONFIG(UnsafeCell<u32>);

/// Configuration structure for interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptConfig(u32);

impl INTERRUPT_CONFIG {
    /// Read interrupt config.
    #[inline]
    pub fn read(&self) -> InterruptConfig {
        InterruptConfig(unsafe { self.0.get().read_volatile() })
    }
    /// Write interrupt config.
    #[inline]
    pub fn write(&self, val: InterruptConfig) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl InterruptConfig {
    const CLEAR_GROUP_0_INTERRUPT: u32 = 1 << 0;
    const CLEAR_GROUP_1_INTERRUPT: u32 = 1 << 1;
    const GROUP_0_HAS_INTERRUPT: u32 = 1 << 8;
    const GROUP_1_HAS_INTERRUPT: u32 = 1 << 9;

    /// Clear group 0 interrupt.
    #[inline]
    pub const fn clear_group_0_interrupt(self) -> Self {
        Self(self.0 | Self::CLEAR_GROUP_0_INTERRUPT)
    }
    /// Clear group 1 interrupt.
    #[inline]
    pub const fn clear_group_1_interrupt(self) -> Self {
        Self(self.0 | Self::CLEAR_GROUP_1_INTERRUPT)
    }
    /// Check if group 0 has interrupt.
    #[inline]
    pub const fn group_0_has_interrupt(self) -> bool {
        self.0 & Self::GROUP_0_HAS_INTERRUPT != 0
    }
    /// Check if group 1 has interrupt.
    #[inline]
    pub const fn group_1_has_interrupt(self) -> bool {
        self.0 & Self::GROUP_1_HAS_INTERRUPT != 0
    }
}

/// Control register group.
#[repr(C)]
pub struct Group {
    /// Group configuration.
    pub group_config: GROUP_CONFIG,
    /// Channel configuration.
    pub channel_config: CHANNEL_CONFIG,
    /// Period configuration.
    pub period_config: PERIOD_CONFIG,
    /// Dead time configuration.
    pub dead_time: DEAD_TIME,
    /// Threshold configuration.
    pub threshold: [THRESHOLD; 4],
    /// Interrupt state.
    pub interrupt_state: INTERRUPT_STATE,
    /// Interrupt mask.
    pub interrupt_mask: INTERRUPT_MASK,
    /// Interrupt clear.
    pub interrupt_clear: INTERRUPT_CLEAR,
    /// Interrupt enable.
    pub interrupt_enable: INTERRUPT_ENABLE,
}

/// Group configuration register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct GROUP_CONFIG(UnsafeCell<u32>);

/// Configuration structure for group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GroupConfig(u32);

impl GROUP_CONFIG {
    /// Read group config.
    #[inline]
    pub fn read(&self) -> GroupConfig {
        GroupConfig(unsafe { self.0.get().read_volatile() })
    }
    /// Write group config.
    #[inline]
    pub fn write(&self, val: GroupConfig) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl GroupConfig {
    const CLOCK_DIVIDE: u32 = 0xffff << 0;
    const STOP_ON_REPEAT: u32 = 1 << 19;
    const ADC_TRIGGER_SOURCE: u32 = 0xf << 20;
    const SOFTWARE_BREAK_ENABLE: u32 = 1 << 24;
    const EXTERNAL_BREAK_ENABLE: u32 = 1 << 25;
    const EXTERNAL_BREAK_POLARITY: u32 = 1 << 26;
    const STOP_ENABLE: u32 = 1 << 27;
    const STOP_MODE: u32 = 1 << 28;
    const STOP_STATE: u32 = 1 << 29;
    const CLOCK_SOURCE: u32 = 0x11 << 30;

    /// Set clock divide.
    #[inline]
    pub const fn set_clock_divide(self, val: u16) -> Self {
        Self(self.0 & !Self::CLOCK_DIVIDE | ((val as u32) << 0) & Self::CLOCK_DIVIDE)
    }
    /// Get clock divide.
    #[inline]
    pub const fn clock_divide(self) -> u16 {
        ((self.0 & Self::CLOCK_DIVIDE) >> 0) as u16
    }
    /// Enable stop on repeat.
    #[inline]
    pub const fn enable_stop_on_repeat(self) -> Self {
        Self(self.0 | Self::STOP_ON_REPEAT)
    }
    /// Disable stop on repeat.
    #[inline]
    pub const fn disable_stop_on_repeat(self) -> Self {
        Self(self.0 & !Self::STOP_ON_REPEAT)
    }
    /// Check if stop on repeat is enabled.
    #[inline]
    pub const fn is_stop_on_repeat(self) -> bool {
        self.0 & Self::STOP_ON_REPEAT != 0
    }
    /// Set ADC trigger source.
    #[inline]
    pub const fn set_adc_trigger_source(self, val: AdcTriggerSource) -> Self {
        Self(self.0 & !Self::ADC_TRIGGER_SOURCE | ((val as u32) << 20) & Self::ADC_TRIGGER_SOURCE)
    }
    /// Get ADC trigger source.
    #[inline]
    pub const fn adc_trigger_source(self) -> AdcTriggerSource {
        match (self.0 & Self::ADC_TRIGGER_SOURCE) >> 20 {
            0 => AdcTriggerSource::Channel0LowThreashold,
            1 => AdcTriggerSource::Channel0HighThreashold,
            2 => AdcTriggerSource::Channel1LowThreashold,
            3 => AdcTriggerSource::Channel1HighThreashold,
            4 => AdcTriggerSource::Channel2LowThreashold,
            5 => AdcTriggerSource::Channel2HighThreashold,
            6 => AdcTriggerSource::Channel3LowThreashold,
            7 => AdcTriggerSource::Channel3HighThreashold,
            8 => AdcTriggerSource::PeriodEnd,
            _ => unreachable!(),
        }
    }
    /// Enable software break.
    #[inline]
    pub const fn enable_software_break(self) -> Self {
        Self(self.0 | Self::SOFTWARE_BREAK_ENABLE)
    }
    /// Disable software break.
    #[inline]
    pub const fn disable_software_break(self) -> Self {
        Self(self.0 & !Self::SOFTWARE_BREAK_ENABLE)
    }
    /// Check if software break is enabled.
    #[inline]
    pub const fn is_software_break_enabled(self) -> bool {
        self.0 & Self::SOFTWARE_BREAK_ENABLE != 0
    }
    /// Enable external break.
    #[inline]
    pub const fn enable_external_break(self) -> Self {
        Self(self.0 | Self::EXTERNAL_BREAK_ENABLE)
    }
    /// Disable external break.
    #[inline]
    pub const fn disable_external_break(self) -> Self {
        Self(self.0 & !Self::EXTERNAL_BREAK_ENABLE)
    }
    /// Check if external break is enabled.
    #[inline]
    pub const fn is_external_break_enabled(self) -> bool {
        self.0 & Self::EXTERNAL_BREAK_ENABLE != 0
    }
    /// Set external break polarity.
    #[inline]
    pub const fn set_external_break_polarity(self, val: Polarity) -> Self {
        Self(
            self.0 & !Self::EXTERNAL_BREAK_POLARITY
                | ((val as u32) << 26) & Self::EXTERNAL_BREAK_POLARITY,
        )
    }
    /// Get external break polarity.
    #[inline]
    pub const fn external_break_polarity(self) -> Polarity {
        match (self.0 & Self::EXTERNAL_BREAK_POLARITY) >> 26 {
            0 => Polarity::ActiveLow,
            1 => Polarity::ActiveHigh,
            _ => unreachable!(),
        }
    }
    /// Enable stop.
    #[inline]
    pub const fn enable_stop(self) -> Self {
        Self(self.0 | Self::STOP_ENABLE)
    }
    /// Disable stop.
    #[inline]
    pub const fn disable_stop(self) -> Self {
        Self(self.0 & !Self::STOP_ENABLE)
    }
    /// Check if stop is enabled.
    #[inline]
    pub const fn is_stop_enabled(self) -> bool {
        self.0 & Self::STOP_ENABLE != 0
    }
    /// Set stop mode.
    #[inline]
    pub const fn set_stop_mode(self, val: StopMode) -> Self {
        Self(self.0 & !Self::STOP_MODE | ((val as u32) << 28) & Self::STOP_MODE)
    }
    /// Get stop mode.
    #[inline]
    pub const fn stop_mode(self) -> StopMode {
        match (self.0 & Self::STOP_MODE) >> 28 {
            0 => StopMode::Abrupt,
            1 => StopMode::Graceful,
            _ => unreachable!(),
        }
    }
    /// Check if stopped.
    #[inline]
    pub const fn is_stopped(self) -> bool {
        self.0 & Self::STOP_STATE != 0
    }
    /// Set clock source.
    #[inline]
    pub const fn set_clock_source(self, val: ClockSource) -> Self {
        Self(self.0 & !Self::CLOCK_SOURCE | ((val as u32) << 30) & Self::CLOCK_SOURCE)
    }
    /// Get clock source.
    #[inline]
    pub const fn clock_source(self) -> ClockSource {
        match (self.0 & Self::CLOCK_SOURCE) >> 30 {
            0x11 => ClockSource::Xclk,
            0x12 => ClockSource::Bclk,
            0x13 => ClockSource::F32kClk,
            _ => unreachable!(),
        }
    }
}

/// ADC trigger source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcTriggerSource {
    Channel0LowThreashold = 0,
    Channel0HighThreashold = 1,
    Channel1LowThreashold = 2,
    Channel1HighThreashold = 3,
    Channel2LowThreashold = 4,
    Channel2HighThreashold = 5,
    Channel3LowThreashold = 6,
    Channel3HighThreashold = 7,
    PeriodEnd = 8,
}

/// Polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}

/// Stop mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StopMode {
    Abrupt = 0,
    Graceful = 1,
}

/// Clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSource {
    Xclk = 0,
    Bclk = 1,
    F32kClk = 2,
}

/// Channel config register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct CHANNEL_CONFIG(UnsafeCell<u32>);

/// Configuration structure for channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ChannelConfig(u32);

impl CHANNEL_CONFIG {
    /// Read channel config.
    #[inline]
    pub fn read(&self) -> ChannelConfig {
        ChannelConfig(unsafe { self.0.get().read_volatile() })
    }
    /// Write channel config.
    #[inline]
    pub fn write(&self, val: ChannelConfig) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl ChannelConfig {
    const POSITIVE_OUTPUT_ENABLE: u32 = 0x1 << 0;
    const POSITIVE_IDLE_STATE: u32 = 0x1 << 1;
    const NEGATIVE_OUTPUT_ENABLE: u32 = 0x1 << 2;
    const NEGATIVE_IDLE_STATE: u32 = 0x1 << 3;

    const POSITIVE_POLARITY: u32 = 0x1 << 16;
    const NEGATIVE_POLARITY: u32 = 0x1 << 17;

    const POSITIVE_BREAK_STATE: u32 = 0x1 << 24;
    const NEGATIVE_BREAK_STATE: u32 = 0x1 << 25;

    /// Enable positive output.
    #[inline]
    pub const fn enable_positive_output(self, idx: usize) -> Self {
        Self(self.0 | Self::POSITIVE_OUTPUT_ENABLE << (idx * 4))
    }
    /// Disable positive output.
    #[inline]
    pub const fn disable_positive_output(self, idx: usize) -> Self {
        Self(self.0 & !(Self::POSITIVE_OUTPUT_ENABLE << (idx * 4)))
    }
    /// Check if positive output is enabled.
    #[inline]
    pub const fn is_positive_output_enabled(self, idx: usize) -> bool {
        self.0 & (Self::POSITIVE_OUTPUT_ENABLE << (idx * 4)) != 0
    }
    /// Set positive idle state.
    #[inline]
    pub const fn set_positive_idle_state(self, idx: usize, val: ElectricLevel) -> Self {
        Self(self.0 & !(Self::POSITIVE_IDLE_STATE << (idx * 4)) | ((val as u32) << (idx * 4)))
    }
    /// Get positive idle state.
    #[inline]
    pub const fn positive_idle_state(self, idx: usize) -> ElectricLevel {
        match (self.0 & (Self::POSITIVE_IDLE_STATE << (idx * 4))) >> (idx * 4) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
    /// Enable negative output.
    #[inline]
    pub const fn enable_negative_output(self, idx: usize) -> Self {
        Self(self.0 | Self::NEGATIVE_OUTPUT_ENABLE << (idx * 4))
    }
    /// Disable negative output.
    #[inline]
    pub const fn disable_negative_output(self, idx: usize) -> Self {
        Self(self.0 & !(Self::NEGATIVE_OUTPUT_ENABLE << (idx * 4)))
    }
    /// Check if negative output is enabled.
    #[inline]
    pub const fn is_negative_output_enabled(self, idx: usize) -> bool {
        self.0 & (Self::NEGATIVE_OUTPUT_ENABLE << (idx * 4)) != 0
    }
    /// Set negative idle state.
    #[inline]
    pub const fn set_negative_idle_state(self, idx: usize, val: ElectricLevel) -> Self {
        Self(self.0 & !(Self::NEGATIVE_IDLE_STATE << (idx * 4)) | ((val as u32) << (idx * 4)))
    }
    /// Get negative idle state.
    #[inline]
    pub const fn negative_idle_state(self, idx: usize) -> ElectricLevel {
        match (self.0 & (Self::NEGATIVE_IDLE_STATE << (idx * 4))) >> (idx * 4) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set positive polarity.
    #[inline]
    pub const fn set_positive_polarity(self, idx: usize, val: Polarity) -> Self {
        Self(self.0 & !(Self::POSITIVE_POLARITY << (idx * 2)) | ((val as u32) << (idx * 2)))
    }
    /// Get positive polarity.
    #[inline]
    pub const fn positive_polarity(self, idx: usize) -> Polarity {
        match (self.0 & (Self::POSITIVE_POLARITY << (idx * 2))) >> (idx * 2) {
            0 => Polarity::ActiveLow,
            1 => Polarity::ActiveHigh,
            _ => unreachable!(),
        }
    }
    /// Set negative polarity.
    #[inline]
    pub const fn set_negative_polarity(self, idx: usize, val: Polarity) -> Self {
        Self(self.0 & !(Self::NEGATIVE_POLARITY << (idx * 2)) | ((val as u32) << (idx * 2)))
    }
    /// Get negative polarity.
    #[inline]
    pub const fn negative_polarity(self, idx: usize) -> Polarity {
        match (self.0 & (Self::NEGATIVE_POLARITY << (idx * 2))) >> (idx * 2) {
            0 => Polarity::ActiveLow,
            1 => Polarity::ActiveHigh,
            _ => unreachable!(),
        }
    }
    /// Set positive break state.
    #[inline]
    pub const fn set_positive_break_state(self, idx: usize, val: ElectricLevel) -> Self {
        Self(self.0 & !(Self::POSITIVE_BREAK_STATE << (idx * 2)) | ((val as u32) << (idx * 2)))
    }
    /// Get positive break state.
    #[inline]
    pub const fn positive_break_state(self, idx: usize) -> ElectricLevel {
        match (self.0 & (Self::POSITIVE_BREAK_STATE << (idx * 2))) >> (idx * 2) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set negative break state.
    #[inline]
    pub const fn set_negative_break_state(self, idx: usize, val: ElectricLevel) -> Self {
        Self(self.0 & !(Self::NEGATIVE_BREAK_STATE << (idx * 2)) | ((val as u32) << (idx * 2)))
    }
    /// Get negative break state.
    #[inline]
    pub const fn negative_break_state(self, idx: usize) -> ElectricLevel {
        match (self.0 & (Self::NEGATIVE_BREAK_STATE << (idx * 2))) >> (idx * 2) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
}

/// Electric level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ElectricLevel {
    Low = 0,
    High = 1,
}

/// Period config register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct PERIOD_CONFIG(UnsafeCell<u32>);

/// Configuration structure for period.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PeriodConfig(u32);

impl PERIOD_CONFIG {
    /// Read period config.
    #[inline]
    pub fn read(&self) -> PeriodConfig {
        PeriodConfig(unsafe { self.0.get().read_volatile() })
    }
    /// Write period config.
    #[inline]
    pub fn write(&self, val: PeriodConfig) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl PeriodConfig {
    const PERIOD: u32 = 0xffff << 0;
    const INTERRUPT_PERIOD: u32 = 0xffff << 16;

    /// Set period.
    #[inline]
    pub const fn set_period(self, val: u16) -> Self {
        Self(self.0 & !Self::PERIOD | ((val as u32) << 0) & Self::PERIOD)
    }
    /// Get period.
    #[inline]
    pub const fn period(self) -> u16 {
        ((self.0 & Self::PERIOD) >> 0) as u16
    }
    /// Set interrupt period.
    #[inline]
    pub const fn set_interrupt_period(self, val: u16) -> Self {
        Self(self.0 & !Self::INTERRUPT_PERIOD | ((val as u32) << 16) & Self::INTERRUPT_PERIOD)
    }
    /// Get interrupt period.
    #[inline]
    pub const fn interrupt_period(self) -> u16 {
        ((self.0 & Self::INTERRUPT_PERIOD) >> 16) as u16
    }
}

/// Dead time register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct DEAD_TIME(UnsafeCell<u32>);

/// Configuration structure for dead time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeadTime(u32);

impl DEAD_TIME {
    /// Read dead time.
    #[inline]
    pub fn read(&self) -> DeadTime {
        DeadTime(unsafe { self.0.get().read_volatile() })
    }
    /// Write dead time.
    #[inline]
    pub fn write(&self, val: DeadTime) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl DeadTime {
    const DEAD_TIME: u32 = 0xff << 0;

    /// Set dead time for channel.
    #[inline]
    pub const fn set_channel(self, idx: usize, val: u8) -> Self {
        Self(
            self.0 & !(Self::DEAD_TIME << (idx * 8))
                | ((val as u32) << (idx * 8)) & Self::DEAD_TIME,
        )
    }
    /// Get dead time for channel.
    #[inline]
    pub const fn channel(self, idx: usize) -> u8 {
        ((self.0 & (Self::DEAD_TIME << (idx * 8))) >> (idx * 8)) as u8
    }
}

/// Threshold register.
#[repr(transparent)]
pub struct THRESHOLD(UnsafeCell<u32>);

/// Configuration structure for threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Threshold(u32);

impl THRESHOLD {
    /// Read threshold.
    #[inline]
    pub fn read(&self) -> Threshold {
        Threshold(unsafe { self.0.get().read_volatile() })
    }
    /// Write threshold.
    #[inline]
    pub fn write(&self, val: Threshold) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl Threshold {
    const LOW: u32 = 0xffff << 0;
    const HIGH: u32 = 0xffff << 16;

    /// Set low threshold.
    #[inline]
    pub const fn set_low(self, val: u16) -> Self {
        Self(self.0 & !Self::LOW | ((val as u32) << 0) & Self::LOW)
    }
    /// Get low threshold.
    #[inline]
    pub const fn low(self) -> u16 {
        ((self.0 & Self::LOW) >> 0) as u16
    }
    /// Set high threshold.
    #[inline]
    pub const fn set_high(self, val: u16) -> Self {
        Self(self.0 & !Self::HIGH | ((val as u32) << 16) & Self::HIGH)
    }
    /// Get high threshold.
    #[inline]
    pub const fn high(self) -> u16 {
        ((self.0 & Self::HIGH) >> 16) as u16
    }
}

/// Interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Interrupt {
    Channel0LowThreashold = 0,
    Channel0HighThreashold = 1,
    Channel1LowThreashold = 2,
    Channel1HighThreashold = 3,
    Channel2LowThreashold = 4,
    Channel2HighThreashold = 5,
    Channel3LowThreashold = 6,
    Channel3HighThreashold = 7,
    PeriodEnd = 8,
    ExternalBreak = 9,
    RepeatCount = 10,
}

/// Interrupt state register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct INTERRUPT_STATE(UnsafeCell<u32>);

/// Interrupt state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptState(u32);

impl INTERRUPT_STATE {
    /// Read interrupt state.
    #[inline]
    pub fn read(&self) -> InterruptState {
        InterruptState(unsafe { self.0.get().read_volatile() })
    }
}

impl InterruptState {
    /// Check if has interrupt.
    #[inline]
    pub const fn has_interrupt(self, val: Interrupt) -> bool {
        (self.0 & (1 << (val as u32))) != 0
    }
}

/// Interrupt mask register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct INTERRUPT_MASK(UnsafeCell<u32>);

/// Interrupt mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptMask(u32);

impl INTERRUPT_MASK {
    /// Read interrupt mask.
    #[inline]
    pub fn read(&self) -> InterruptMask {
        InterruptMask(unsafe { self.0.get().read_volatile() })
    }
    /// Write interrupt mask.
    #[inline]
    pub fn write(&self, val: InterruptMask) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

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
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct INTERRUPT_CLEAR(UnsafeCell<u32>);

/// Interrupt clear.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptClear(u32);

impl INTERRUPT_CLEAR {
    /// Write interrupt clear.
    #[inline]
    pub fn write(&self, val: InterruptClear) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

impl InterruptClear {
    /// Clear interrupt.
    #[inline]
    pub const fn clear_interrupt(self, val: Interrupt) -> Self {
        Self(self.0 | (1 << (val as u32)))
    }
}

/// Interrupt enable register.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct INTERRUPT_ENABLE(UnsafeCell<u32>);

/// Interrupt enable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptEnable(u32);

impl INTERRUPT_ENABLE {
    /// Read interrupt enable.
    #[inline]
    pub fn read(&self) -> InterruptEnable {
        InterruptEnable(unsafe { self.0.get().read_volatile() })
    }
    /// Write interrupt enable.
    #[inline]
    pub fn write(&self, val: InterruptEnable) {
        unsafe { self.0.get().write_volatile(val.0) }
    }
}

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
