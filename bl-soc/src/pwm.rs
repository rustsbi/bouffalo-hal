//! Pulse Width Modulation peripheral.
use crate::glb::{
    v2::{PwmSignal0, PwmSignal1},
    GLBv2,
};
use crate::gpio::{self, Alternate, Pad};
use crate::{clocks::Clocks, PWM};
use base_address::BaseAddress;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use embedded_time::rate::Hertz;
use volatile_register::{RO, RW, WO};

/// Pulse width modulation registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt configuration.
    pub interrupt_config: RW<InterruptConfig>,
    _reserved: [u8; 0x3c],
    /// control register group.
    pub group: [Group; 2],
}

/// Interrupt configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptConfig(u32);

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
    pub group_config: RW<GroupConfig>,
    /// Channel configuration.
    pub channel_config: RW<ChannelConfig>,
    /// Period configuration.
    pub period_config: RW<PeriodConfig>,
    /// Dead time configuration.
    pub dead_time: RW<DeadTime>,
    /// Threshold configuration.
    pub threshold: [RW<Threshold>; 4],
    /// Interrupt state.
    pub interrupt_state: RO<InterruptState>,
    /// Interrupt mask.
    pub interrupt_mask: RW<InterruptMask>,
    /// Interrupt clear.
    pub interrupt_clear: WO<InterruptClear>,
    /// Interrupt enable.
    pub interrupt_enable: RW<InterruptEnable>,
}

/// Group configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct GroupConfig(u32);

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
    const CLOCK_SOURCE: u32 = 0x3 << 30;

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
        Self((self.0 & !Self::ADC_TRIGGER_SOURCE) | ((val as u32) << 20) & Self::ADC_TRIGGER_SOURCE)
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
            0x0 => ClockSource::Xclk,
            0x1 => ClockSource::Bclk,
            _ => ClockSource::F32kClk,
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

/// Channel configuration register.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct ChannelConfig(u32);

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
        assert!(idx < 4);
        Self(self.0 | Self::POSITIVE_OUTPUT_ENABLE << (idx * 4))
    }
    /// Disable positive output.
    #[inline]
    pub const fn disable_positive_output(self, idx: usize) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::POSITIVE_OUTPUT_ENABLE << (idx * 4)))
    }
    /// Check if positive output is enabled.
    #[inline]
    pub const fn is_positive_output_enabled(self, idx: usize) -> bool {
        assert!(idx < 4);
        self.0 & (Self::POSITIVE_OUTPUT_ENABLE << (idx * 4)) != 0
    }
    /// Set positive idle state.
    #[inline]
    pub const fn set_positive_idle_state(self, idx: usize, val: ElectricLevel) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::POSITIVE_IDLE_STATE << (idx * 4)) | ((val as u32) << (1 + idx * 4)))
    }
    /// Get positive idle state.
    #[inline]
    pub const fn positive_idle_state(self, idx: usize) -> ElectricLevel {
        assert!(idx < 4);
        match (self.0 & (Self::POSITIVE_IDLE_STATE << (idx * 4))) >> (1 + idx * 4) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
    /// Enable negative output.
    #[inline]
    pub const fn enable_negative_output(self, idx: usize) -> Self {
        assert!(idx < 4);
        Self(self.0 | Self::NEGATIVE_OUTPUT_ENABLE << (idx * 4))
    }
    /// Disable negative output.
    #[inline]
    pub const fn disable_negative_output(self, idx: usize) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::NEGATIVE_OUTPUT_ENABLE << (idx * 4)))
    }
    /// Check if negative output is enabled.
    #[inline]
    pub const fn is_negative_output_enabled(self, idx: usize) -> bool {
        assert!(idx < 4);
        self.0 & (Self::NEGATIVE_OUTPUT_ENABLE << (idx * 4)) != 0
    }
    /// Set negative idle state.
    #[inline]
    pub const fn set_negative_idle_state(self, idx: usize, val: ElectricLevel) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::NEGATIVE_IDLE_STATE << (idx * 4)) | ((val as u32) << (3 + idx * 4)))
    }
    /// Get negative idle state.
    #[inline]
    pub const fn negative_idle_state(self, idx: usize) -> ElectricLevel {
        assert!(idx < 4);
        match (self.0 & (Self::NEGATIVE_IDLE_STATE << (idx * 4))) >> (3 + idx * 4) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set positive polarity.
    #[inline]
    pub const fn set_positive_polarity(self, idx: usize, val: Polarity) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::POSITIVE_POLARITY << (idx * 2)) | ((val as u32) << (16 + idx * 2)))
    }
    /// Get positive polarity.
    #[inline]
    pub const fn positive_polarity(self, idx: usize) -> Polarity {
        assert!(idx < 4);
        match (self.0 & (Self::POSITIVE_POLARITY << (idx * 2))) >> (16 + idx * 2) {
            0 => Polarity::ActiveLow,
            1 => Polarity::ActiveHigh,
            _ => unreachable!(),
        }
    }
    /// Set negative polarity.
    #[inline]
    pub const fn set_negative_polarity(self, idx: usize, val: Polarity) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::NEGATIVE_POLARITY << (idx * 2)) | ((val as u32) << (17 + idx * 2)))
    }
    /// Get negative polarity.
    #[inline]
    pub const fn negative_polarity(self, idx: usize) -> Polarity {
        assert!(idx < 4);
        match (self.0 & (Self::NEGATIVE_POLARITY << (idx * 2))) >> (17 + idx * 2) {
            0 => Polarity::ActiveLow,
            1 => Polarity::ActiveHigh,
            _ => unreachable!(),
        }
    }
    /// Set positive break state.
    #[inline]
    pub const fn set_positive_break_state(self, idx: usize, val: ElectricLevel) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::POSITIVE_BREAK_STATE << (idx * 2)) | ((val as u32) << (24 + idx * 2)))
    }
    /// Get positive break state.
    #[inline]
    pub const fn positive_break_state(self, idx: usize) -> ElectricLevel {
        assert!(idx < 4);
        match (self.0 & (Self::POSITIVE_BREAK_STATE << (idx * 2))) >> (24 + idx * 2) {
            0 => ElectricLevel::Low,
            1 => ElectricLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set negative break state.
    #[inline]
    pub const fn set_negative_break_state(self, idx: usize, val: ElectricLevel) -> Self {
        assert!(idx < 4);
        Self(self.0 & !(Self::NEGATIVE_BREAK_STATE << (idx * 2)) | ((val as u32) << (25 + idx * 2)))
    }
    /// Get negative break state.
    #[inline]
    pub const fn negative_break_state(self, idx: usize) -> ElectricLevel {
        assert!(idx < 4);
        match (self.0 & (Self::NEGATIVE_BREAK_STATE << (idx * 2))) >> (25 + idx * 2) {
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

/// Period configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct PeriodConfig(u32);

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

/// Dead time configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct DeadTime(u32);

impl DeadTime {
    const DEAD_TIME: u32 = 0xff << 0;

    /// Set dead time for channel.
    #[inline]
    pub const fn set_channel(self, idx: usize, val: u8) -> Self {
        Self((self.0 & !(Self::DEAD_TIME << (idx * 8))) | ((val as u32) << (idx * 8)))
    }
    /// Get dead time for channel.
    #[inline]
    pub const fn channel(self, idx: usize) -> u8 {
        ((self.0 & (Self::DEAD_TIME << (idx * 8))) >> (idx * 8)) as u8
    }
}

/// Threshold register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Threshold(u32);

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct InterruptState(u32);

impl InterruptState {
    /// Check if has interrupt.
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

/// Single end signal settings (type state).
pub struct SingleEnd;

/// Differential end signal settings (type state).
pub struct DifferentialEnd;

/// Brushless DC Motor (BLDC) signal settings (type state).
pub struct BrushlessDcMotor;

/// Valid settings for PWM signal 0.
pub trait Signal0 {
    /// Signal value in `PwmConfig` register on `GLB` peripheral.
    const VALUE: PwmSignal0;
}

/// Valid settings for PWM signal 1.
pub trait Signal1 {
    /// Signal value in `PwmConfig` register on `GLB` peripheral.
    const VALUE: PwmSignal1;
}

impl Signal0 for SingleEnd {
    const VALUE: PwmSignal0 = PwmSignal0::SingleEnd;
}

impl Signal0 for DifferentialEnd {
    const VALUE: PwmSignal0 = PwmSignal0::DifferentialEnd;
}

impl Signal1 for SingleEnd {
    const VALUE: PwmSignal1 = PwmSignal1::SingleEnd;
}

impl Signal1 for BrushlessDcMotor {
    const VALUE: PwmSignal1 = PwmSignal1::BrushlessDcMotor;
}

/// Managed pulse width modulation peripheral.
pub struct Pwm<A: BaseAddress, S> {
    pub group0: Channels<A, S, 0>,
    pub group1: Channels<A, S, 1>,
}

impl<A: BaseAddress, S0: Signal0, S1: Signal1> Pwm<A, (S0, S1)> {
    /// Creates a pulse width modulation instance with given signal settings.
    #[rustfmt::skip]
    #[inline]
    pub fn new(pwm: PWM<A>, signal_0: S0, signal_1: S1, glb: &GLBv2<impl BaseAddress>) -> Self {
        unsafe {
            glb.pwm_config
                .modify(|config| config.set_signal_0(S0::VALUE).set_signal_1(S1::VALUE));
            glb.clock_config_1.modify(|config| config.enable_pwm());
        }
        drop((signal_0, signal_1));
        Pwm {
            group0: Channels {
                channel0: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                channel1: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                channel2: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                channel3: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                external_break: ExternalBreak { _signals: PhantomData },
                pwm: unsafe { core::ptr::read(&pwm as *const _) },
                _signals: PhantomData,
            },
            group1: Channels {
                channel0: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                channel1: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                channel2: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                channel3: Channel { pwm: unsafe { core::ptr::read(&pwm as *const _) }, _signals: PhantomData },
                external_break: ExternalBreak { _signals: PhantomData },
                pwm,
                _signals: PhantomData,
            },
        }
    }
}

/// PWM group with all its channels.
pub struct Channels<A: BaseAddress, S, const I: usize> {
    /// Channel 0 of current PWM group.
    pub channel0: Channel<A, S, I, 0>,
    /// Channel 1 of current PWM group.
    pub channel1: Channel<A, S, I, 1>,
    /// Channel 2 of current PWM group.
    pub channel2: Channel<A, S, I, 2>,
    /// Channel 3 of current PWM group.
    pub channel3: Channel<A, S, I, 3>,
    /// External break signal for current PWM group.
    pub external_break: ExternalBreak<S, I>,
    pwm: PWM<A>,
    _signals: PhantomData<S>,
}

impl<A: BaseAddress, S, const I: usize> Channels<A, S, I> {
    /// Configure clock settings for current PWM group.
    ///
    /// Clock settings would affect all the channels in the PWM group.
    #[inline]
    pub fn set_clock(&mut self, frequency: Hertz, source: ClockSource, clocks: &Clocks) {
        let source_freq = match source {
            ClockSource::Xclk => clocks.xclk(),
            ClockSource::Bclk => todo!(),
            ClockSource::F32kClk => todo!(),
        };
        let clock_divisor = source_freq.0 / frequency.0;
        if !(1..=65535).contains(&clock_divisor) {
            panic!("impossible frequency");
        }
        unsafe {
            self.pwm.group[I].group_config.modify(|val| {
                val.set_clock_source(source)
                    .set_clock_divide(clock_divisor as u16)
            })
        };
    }
    /// Configure maximum duty cycle for this PWM group.
    #[inline]
    pub fn set_max_duty_cycle(&mut self, duty: u16) {
        unsafe {
            self.pwm.group[I]
                .period_config
                .modify(|val| val.set_period(duty))
        }
    }
    /// Start current PWM group.
    #[inline]
    pub fn start(&mut self) {
        unsafe {
            self.pwm.group[I]
                .group_config
                .modify(|val| val.disable_stop().disable_software_break())
        };
        while self.pwm.group[I].group_config.read().is_stopped() {
            core::hint::spin_loop();
        }
    }
    /// Stop current PWM group.
    #[inline]
    pub fn stop(&mut self) {
        unsafe {
            self.pwm.group[I]
                .group_config
                .modify(|val| val.enable_stop())
        }
        while !self.pwm.group[I].group_config.read().is_stopped() {
            core::hint::spin_loop();
        }
    }
}

/// Pulse Width Modulation channel.
pub struct Channel<A: BaseAddress, S, const I: usize, const J: usize> {
    pwm: PWM<A>,
    _signals: PhantomData<S>,
}

impl<A1: BaseAddress, S, const I: usize, const J: usize> Channel<A1, S, I, J> {
    /// Wrap current channel as positive signal with GPIO pin.
    ///
    /// This function statically checks if target GPIO pin mode matches current PWM channel.
    /// If won't match, it will raise compile error.
    #[inline]
    pub fn positive_signal_pin<A2: BaseAddress, const N: usize, const F: usize>(
        self,
        pin: Pad<A2, N, gpio::Pwm<F>>,
    ) -> PwmPin<Self, Pad<A2, N, gpio::Pwm<F>>, Positive>
    where
        gpio::Pwm<F>: Alternate,
        Pad<A2, N, gpio::Pwm<F>>: HasPwmSignal<S, I, J, Positive>,
    {
        PwmPin {
            channel: self,
            pin,
            _polarity: PhantomData,
        }
    }
    /// Wrap current channel as negative signal with GPIO pin.
    ///
    /// This function statically checks if target GPIO pin mode matches current PWM channel.
    /// If won't match, it will raise compile error.
    #[inline]
    pub fn negative_signal_pin<A2: BaseAddress, const N: usize, const F: usize>(
        self,
        pin: Pad<A2, N, gpio::Pwm<F>>,
    ) -> PwmPin<Self, Pad<A2, N, gpio::Pwm<F>>, Negative>
    where
        gpio::Pwm<F>: Alternate,
        Pad<A2, N, gpio::Pwm<F>>: HasPwmSignal<S, I, J, Negative>,
    {
        PwmPin {
            channel: self,
            pin,
            _polarity: PhantomData,
        }
    }
}

/// Pulse Width Modulation external break signal.
pub struct ExternalBreak<S, const I: usize> {
    _signals: PhantomData<S>,
}

impl<S, const I: usize> ExternalBreak<S, I> {
    /// Wrap current channel as external break signal with GPIO pin.
    ///
    /// This function statically checks if target GPIO pin mode matches the external
    /// break signal of current PWM group. If won't match, it will raise compile error.
    #[inline]
    pub fn external_break_pin<A2: BaseAddress, const N: usize, const F: usize>(
        self,
        pin: Pad<A2, N, gpio::Pwm<F>>,
    ) -> PwmPin<Self, Pad<A2, N, gpio::Pwm<F>>, ()>
    where
        gpio::Pwm<F>: Alternate,
        Pad<A2, N, gpio::Pwm<F>>: HasPwmExternalBreak<I>,
    {
        PwmPin {
            channel: self,
            pin,
            _polarity: PhantomData,
        }
    }
}

/// Positive signal polarity (type state).
pub struct Positive;

/// Negative signal polarity (type state).
pub struct Negative;

/// Check if target is internally connected to PWM signal, polarity under signal settings.
///
/// It checks if it is connected to PWM group `I`, channel `J` and polarity `P` with signal settings `S`.
pub trait HasPwmSignal<S, const I: usize, const J: usize, P> {}

/// Check if target is internally connected to PWM external break signal.
///
/// It checks if it is connected to external break signal of PWM group `I`.
pub trait HasPwmExternalBreak<const I: usize> {}

impl<A: BaseAddress, S, const I: usize, const J: usize> embedded_hal::pwm::ErrorType
    for Channel<A, S, I, J>
{
    type Error = core::convert::Infallible;
}

impl<A: BaseAddress, S, const I: usize, const J: usize> embedded_hal::pwm::SetDutyCycle
    for Channel<A, S, I, J>
{
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        self.pwm.group[I].period_config.read().period()
    }
    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        unsafe { self.pwm.group[I].threshold[J].modify(|val| val.set_low(0).set_high(duty)) };
        Ok(())
    }
}

/// Wrapped GPIO pin with PWM channel feature.
///
/// `PwmPin` implements both `pwm::SetDutyCycle` and `digital::OutputPin` traits.
/// With `PwmPin`, users may seamlessly use both PWM and GPIO functions at the same time
/// without switching GPIO pin mode.
pub struct PwmPin<CHANNEL, PIN, POLARITY> {
    channel: CHANNEL,
    pin: PIN,
    _polarity: PhantomData<POLARITY>,
}

impl<CHANNEL, PIN, POLARITY> PwmPin<CHANNEL, PIN, POLARITY> {
    #[inline]
    pub fn free(self) -> (CHANNEL, PIN) {
        (self.channel, self.pin)
    }
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN>
    PwmPin<Channel<A1, S, I, J>, PIN, Positive>
{
    /// Enable PWM output for this pin.
    #[inline]
    pub fn enable_pwm_output(&mut self) {
        unsafe {
            self.channel.pwm.group[I]
                .channel_config
                .modify(|val| val.enable_positive_output(J))
        }
    }
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN>
    PwmPin<Channel<A1, S, I, J>, PIN, Negative>
{
    /// Enable PWM output for this pin.
    #[inline]
    pub fn enable_pwm_output(&mut self) {
        unsafe {
            self.channel.pwm.group[I]
                .channel_config
                .modify(|val| val.enable_negative_output(J))
        }
    }
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN, POLARITY>
    embedded_hal::digital::ErrorType for PwmPin<Channel<A1, S, I, J>, PIN, POLARITY>
{
    type Error = core::convert::Infallible;
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN> embedded_hal::digital::OutputPin
    for PwmPin<Channel<A1, S, I, J>, PIN, Positive>
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.channel.pwm.group[I].channel_config.modify(|val| {
                val.set_positive_idle_state(J, ElectricLevel::Low)
                    .disable_positive_output(J)
            })
        }
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.channel.pwm.group[I].channel_config.modify(|val| {
                val.set_positive_idle_state(J, ElectricLevel::High)
                    .disable_positive_output(J)
            })
        }
        Ok(())
    }
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN> embedded_hal::digital::OutputPin
    for PwmPin<Channel<A1, S, I, J>, PIN, Negative>
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.channel.pwm.group[I].channel_config.modify(|val| {
                val.set_negative_idle_state(J, ElectricLevel::Low)
                    .disable_negative_output(J)
            })
        }
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.channel.pwm.group[I].channel_config.modify(|val| {
                val.set_negative_idle_state(J, ElectricLevel::High)
                    .disable_negative_output(J)
            })
        }
        Ok(())
    }
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN, POLARITY> Deref
    for PwmPin<Channel<A1, S, I, J>, PIN, POLARITY>
{
    type Target = Channel<A1, S, I, J>;
    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

impl<A1: BaseAddress, S, const I: usize, const J: usize, PIN, POLARITY> DerefMut
    for PwmPin<Channel<A1, S, I, J>, PIN, POLARITY>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channel
    }
}

#[rustfmt::skip]
mod gpio_impls {
    use super::*;

    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 0, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 1, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 2, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 3, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 4, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 5, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 6, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 7, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 8, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 9, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 10, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 11, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 12, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 13, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 14, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 15, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 16, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 17, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 18, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 19, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 20, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 21, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 22, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 23, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 24, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 25, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 26, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 27, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 28, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 29, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 30, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 31, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 32, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 33, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 34, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 35, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 36, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 37, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 38, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 39, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 40, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 41, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 2, Positive> for Pad<A, 42, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 3, Positive> for Pad<A, 43, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 0, Positive> for Pad<A, 44, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(SingleEnd, S2), 0, 1, Positive> for Pad<A, 45, gpio::Pwm<0>> {}

    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Positive> for Pad<A, 0, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Negative> for Pad<A, 1, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Positive> for Pad<A, 2, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Negative> for Pad<A, 3, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Positive> for Pad<A, 4, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Negative> for Pad<A, 5, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Positive> for Pad<A, 6, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Negative> for Pad<A, 7, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Positive> for Pad<A, 8, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Negative> for Pad<A, 9, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Positive> for Pad<A, 10, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Negative> for Pad<A, 11, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Positive> for Pad<A, 12, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Negative> for Pad<A, 13, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Positive> for Pad<A, 14, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Negative> for Pad<A, 15, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Positive> for Pad<A, 16, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Negative> for Pad<A, 17, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Positive> for Pad<A, 18, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Negative> for Pad<A, 19, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Positive> for Pad<A, 20, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Negative> for Pad<A, 21, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Positive> for Pad<A, 22, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Negative> for Pad<A, 23, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Positive> for Pad<A, 24, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Negative> for Pad<A, 25, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Positive> for Pad<A, 26, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Negative> for Pad<A, 27, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Positive> for Pad<A, 28, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Negative> for Pad<A, 29, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Positive> for Pad<A, 30, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Negative> for Pad<A, 31, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Positive> for Pad<A, 32, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Negative> for Pad<A, 33, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Positive> for Pad<A, 34, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Negative> for Pad<A, 35, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Positive> for Pad<A, 36, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Negative> for Pad<A, 37, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Positive> for Pad<A, 38, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 3, Negative> for Pad<A, 39, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Positive> for Pad<A, 40, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 0, Negative> for Pad<A, 41, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Positive> for Pad<A, 42, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 1, Negative> for Pad<A, 43, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Positive> for Pad<A, 44, gpio::Pwm<0>> {}
    impl<A: BaseAddress, S2> HasPwmSignal<(DifferentialEnd, S2), 0, 2, Negative> for Pad<A, 45, gpio::Pwm<0>> {}

    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 0, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 1, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 2, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 3, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 4, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 5, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 6, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 7, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 8, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 9, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 10, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 11, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 12, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 13, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 14, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 15, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 16, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 17, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 18, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 19, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 20, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 21, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 22, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 23, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 24, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 25, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 26, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 27, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 28, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 29, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 30, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 31, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 32, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 33, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 34, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 35, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 36, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 37, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 38, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 39, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 40, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 41, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 2, Positive> for Pad<A, 42, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 3, Positive> for Pad<A, 43, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 0, Positive> for Pad<A, 44, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, SingleEnd), 1, 1, Positive> for Pad<A, 45, gpio::Pwm<1>> {}

    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 0, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 1, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 2, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 3, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 4, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 5, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 6, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 7, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 8, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 9, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 10, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 11, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 12, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 13, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 14, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 15, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 16, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 17, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 18, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 19, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 20, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 21, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 22, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 23, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 24, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 25, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 26, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 27, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 28, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 29, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 30, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 31, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 32, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 33, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 34, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 35, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 36, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 37, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 38, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 39, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 40, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 41, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 1, Positive> for Pad<A, 42, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 2, Positive> for Pad<A, 43, gpio::Pwm<1>> {}
    impl<A: BaseAddress> HasPwmExternalBreak<0> for Pad<A, 44, gpio::Pwm<1>> {}
    impl<A: BaseAddress, S1> HasPwmSignal<(S1, BrushlessDcMotor), 0, 0, Positive> for Pad<A, 45, gpio::Pwm<1>> {}    
}

#[cfg(test)]
mod tests {
    use super::{
        AdcTriggerSource, ChannelConfig, ClockSource, DeadTime, ElectricLevel, Group, GroupConfig,
        Interrupt, InterruptClear, InterruptConfig, InterruptEnable, InterruptMask, InterruptState,
        PeriodConfig, Polarity, RegisterBlock, StopMode, Threshold,
    };
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, interrupt_config), 0x00);
        assert_eq!(offset_of!(RegisterBlock, group), 0x40);
    }

    #[test]
    fn struct_group_offset() {
        assert_eq!(offset_of!(Group, group_config), 0x00);
        assert_eq!(offset_of!(Group, channel_config), 0x04);
        assert_eq!(offset_of!(Group, period_config), 0x08);
        assert_eq!(offset_of!(Group, dead_time), 0x0c);
        assert_eq!(offset_of!(Group, threshold), 0x10);
        assert_eq!(offset_of!(Group, interrupt_state), 0x20);
        assert_eq!(offset_of!(Group, interrupt_mask), 0x24);
        assert_eq!(offset_of!(Group, interrupt_clear), 0x28);
        assert_eq!(offset_of!(Group, interrupt_enable), 0x2c);
    }

    #[test]
    fn struct_interrupt_config_functions() {
        let mut val = InterruptConfig(0x0);

        val = val.clear_group_0_interrupt();
        assert_eq!(val.0, 0x00000001);

        val = InterruptConfig(0x0);
        val = val.clear_group_1_interrupt();
        assert_eq!(val.0, 0x00000002);

        val = InterruptConfig(0x00000100);
        assert!(val.group_0_has_interrupt());
        val = InterruptConfig(0x00000000);
        assert!(!val.group_0_has_interrupt());
        val = InterruptConfig(0x00000200);
        assert!(val.group_1_has_interrupt());
        val = InterruptConfig(0x00000000);
        assert!(!val.group_1_has_interrupt());
    }

    #[test]
    fn struct_group_config_functions() {
        let mut val;

        for iter in [0x0000, 0x1037, 0xabcd, 0xffff] {
            val = GroupConfig(iter);
            assert_eq!(val.clock_divide(), iter as u16);
        }

        val = GroupConfig(0x0);
        val = val.enable_stop_on_repeat();
        assert_eq!(val.0, 0x00080000);
        assert!(val.is_stop_on_repeat());
        val = val.disable_stop_on_repeat();
        assert_eq!(val.0, 0x00000000);
        assert!(!val.is_stop_on_repeat());

        val = GroupConfig(0x0);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel0LowThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel0LowThreashold
        );
        assert_eq!(AdcTriggerSource::Channel0LowThreashold as u32, 0x00000000);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel0HighThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel0HighThreashold
        );
        assert_eq!(AdcTriggerSource::Channel0HighThreashold as u32, 0x00000001);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel1LowThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel1LowThreashold
        );
        assert_eq!(AdcTriggerSource::Channel1LowThreashold as u32, 0x00000002);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel1HighThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel1HighThreashold
        );
        assert_eq!(AdcTriggerSource::Channel1HighThreashold as u32, 0x00000003);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel2LowThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel2LowThreashold
        );
        assert_eq!(AdcTriggerSource::Channel2LowThreashold as u32, 0x00000004);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel2HighThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel2HighThreashold
        );
        assert_eq!(AdcTriggerSource::Channel2HighThreashold as u32, 0x00000005);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel3LowThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel3LowThreashold
        );
        assert_eq!(AdcTriggerSource::Channel3LowThreashold as u32, 0x00000006);
        val = val.set_adc_trigger_source(AdcTriggerSource::Channel3HighThreashold);
        assert_eq!(
            val.adc_trigger_source(),
            AdcTriggerSource::Channel3HighThreashold
        );
        assert_eq!(AdcTriggerSource::Channel3HighThreashold as u32, 0x00000007);
        val = val.set_adc_trigger_source(AdcTriggerSource::PeriodEnd);
        assert_eq!(val.adc_trigger_source(), AdcTriggerSource::PeriodEnd);
        assert_eq!(AdcTriggerSource::PeriodEnd as u32, 0x00000008);

        val = GroupConfig(0x0);
        val = val.enable_software_break();
        assert!(val.is_software_break_enabled());
        assert_eq!(val.0, 0x01000000);
        val = val.disable_software_break();
        assert!(!val.is_software_break_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.enable_external_break();
        assert!(val.is_external_break_enabled());
        assert_eq!(val.0, 0x02000000);
        val = val.disable_external_break();
        assert!(!val.is_external_break_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_external_break_polarity(Polarity::ActiveHigh);
        assert_eq!(val.external_break_polarity(), Polarity::ActiveHigh);
        assert_eq!(val.0, 0x04000000);
        val = val.set_external_break_polarity(Polarity::ActiveLow);
        assert_eq!(val.external_break_polarity(), Polarity::ActiveLow);
        assert_eq!(val.0, 0x00000000);

        val = val.enable_stop();
        assert!(val.is_stop_enabled());
        assert_eq!(val.0, 0x08000000);
        val = val.disable_stop();
        assert!(!val.is_stop_enabled());
        assert_eq!(val.0, 0x00000000);

        val = val.set_stop_mode(StopMode::Abrupt);
        assert_eq!(val.stop_mode(), StopMode::Abrupt);
        val = val.set_stop_mode(StopMode::Graceful);
        assert_eq!(val.stop_mode(), StopMode::Graceful);

        val = GroupConfig(0x20000000);
        assert!(val.is_stopped());
        val = GroupConfig(0x00000000);
        assert!(!val.is_stopped());

        val = GroupConfig(0x00000000);
        assert_eq!(val.clock_source(), ClockSource::Xclk);
        val = GroupConfig(0x40000000);
        assert_eq!(val.clock_source(), ClockSource::Bclk);
        val = GroupConfig(0x80000000);
        assert_eq!(val.clock_source(), ClockSource::F32kClk);
        val = GroupConfig(0xc0000000);
        assert_eq!(val.clock_source(), ClockSource::F32kClk);
    }

    #[test]
    fn struct_channel_config_functions() {
        for idx in 0..=3 {
            let mut val = ChannelConfig(0x0);
            val = val.enable_positive_output(idx);
            assert_eq!(val.0, 0x00000001 << (idx * 4));
            assert!(val.is_positive_output_enabled(idx));
            val = val.disable_positive_output(idx);
            assert_eq!(val.0, 0x00000000 << (idx * 4));
            assert!(!val.is_positive_output_enabled(idx));

            val = val.set_positive_idle_state(idx, ElectricLevel::High);
            assert_eq!(val.0, 0x00000002 << (idx * 4));
            assert_eq!(ElectricLevel::High, val.positive_idle_state(idx));
            val = val.set_positive_idle_state(idx, ElectricLevel::Low);
            assert_eq!(val.0, 0x00000000 << (idx * 4));
            assert_eq!(ElectricLevel::Low, val.positive_idle_state(idx));

            val = val.enable_negative_output(idx);
            assert_eq!(val.0, 0x00000004 << (idx * 4));
            assert!(val.is_negative_output_enabled(idx));
            val = val.disable_negative_output(idx);
            assert_eq!(val.0, 0x00000000 << (idx * 4));
            assert!(!val.is_negative_output_enabled(idx));

            val = val.set_negative_idle_state(idx, ElectricLevel::High);
            assert_eq!(val.0, 0x00000008 << (idx * 4));
            assert_eq!(ElectricLevel::High, val.negative_idle_state(idx));
            val = val.set_negative_idle_state(idx, ElectricLevel::Low);
            assert_eq!(val.0, 0x00000000 << (idx * 4));
            assert_eq!(ElectricLevel::Low, val.negative_idle_state(idx));
        }

        for idx in 0..=3 {
            let mut val = ChannelConfig(0x0);

            val = val.set_positive_polarity(idx, Polarity::ActiveHigh);
            assert_eq!(val.0, 0x00000001 << (16 + idx * 2));
            assert_eq!(val.positive_polarity(idx), Polarity::ActiveHigh);
            val = val.set_positive_polarity(idx, Polarity::ActiveLow);
            assert_eq!(val.0, 0x00000000 << (16 + idx * 2));
            assert_eq!(val.positive_polarity(idx), Polarity::ActiveLow);

            val = val.set_negative_polarity(idx, Polarity::ActiveHigh);
            assert_eq!(val.0, 0x00000001 << (17 + idx * 2));
            assert_eq!(val.negative_polarity(idx), Polarity::ActiveHigh);
            val = val.set_negative_polarity(idx, Polarity::ActiveLow);
            assert_eq!(val.0, 0x00000000 << (17 + idx * 2));
            assert_eq!(val.negative_polarity(idx), Polarity::ActiveLow);

            val = val.set_positive_break_state(idx, ElectricLevel::High);
            assert_eq!(val.0, 0x00000001 << (24 + idx * 2));
            assert_eq!(val.positive_break_state(idx), ElectricLevel::High);
            val = val.set_positive_break_state(idx, ElectricLevel::Low);
            assert_eq!(val.0, 0x00000000 << (24 + idx * 2));
            assert_eq!(val.positive_break_state(idx), ElectricLevel::Low);

            val = val.set_negative_break_state(idx, ElectricLevel::High);
            assert_eq!(val.0, 0x00000001 << (25 + idx * 2));
            assert_eq!(val.negative_break_state(idx), ElectricLevel::High);
            val = val.set_negative_break_state(idx, ElectricLevel::Low);
            assert_eq!(val.0, 0x00000000 << (25 + idx * 2));
            assert_eq!(val.negative_break_state(idx), ElectricLevel::Low);
        }
    }

    #[test]
    fn struct_period_config_functions() {
        let mut val = PeriodConfig(0x0);
        for iter in [0x0000, 0x1037, 0xabcd, 0xffff] {
            val = val.set_period(iter);
            assert_eq!(val.0, iter as u32);
        }

        val = PeriodConfig(0x0);
        for iter in [0x0000, 0x1037, 0xabcd, 0xffff] {
            val = val.set_interrupt_period(iter);
            assert_eq!(val.0, (iter as u32) << 16);
        }
    }

    #[test]
    fn struct_deadtime_functions() {
        let mut val: DeadTime;
        for idx in 0..=3 {
            for iter in 0..=1 {
                val = DeadTime(0x0);
                val = val.set_channel(idx, iter);
                assert_eq!(val.channel(idx), iter);
                assert_eq!(val.0, (iter as u32) << (idx * 8));
            }
        }
    }

    #[test]
    fn struct_threshold_functions() {
        let mut val: Threshold;
        for iter in [0x0000, 0x1037, 0xabcd, 0xffff] {
            val = Threshold(0x0);
            val = val.set_low(iter);
            assert_eq!(val.low(), iter);
            assert_eq!(val.0, iter as u32);

            val = Threshold(0x0);
            val = val.set_high(iter);
            assert_eq!(val.high(), iter);
            assert_eq!(val.0, (iter as u32) << 16);
        }
    }

    impl Interrupt {
        fn from_u32(value: u32) -> Interrupt {
            match value {
                0 => Interrupt::Channel0LowThreashold,
                1 => Interrupt::Channel0HighThreashold,
                2 => Interrupt::Channel1LowThreashold,
                3 => Interrupt::Channel1HighThreashold,
                4 => Interrupt::Channel2LowThreashold,
                5 => Interrupt::Channel2HighThreashold,
                6 => Interrupt::Channel3LowThreashold,
                7 => Interrupt::Channel3HighThreashold,
                8 => Interrupt::PeriodEnd,
                9 => Interrupt::ExternalBreak,
                10 => Interrupt::RepeatCount,
                _ => panic!("Unknown value: {}", value),
            }
        }
    }

    #[test]
    fn struct_interrupt_state_functions() {
        let mut val: InterruptState;
        for idx in 0..=10 {
            val = InterruptState(0x00000001 << idx);
            assert!(val.has_interrupt(Interrupt::from_u32(idx)));
        }
    }

    #[test]
    fn struct_interrupt_mask_functions() {
        let mut val: InterruptMask = InterruptMask(0x0);
        for idx in 0..=10 {
            val = val.mask_interrupt(Interrupt::from_u32(idx));
            assert!(val.is_interrupt_masked(Interrupt::from_u32(idx)));
            assert_eq!(val.0, 0x00000001 << idx);
            val = val.unmask_interrupt(Interrupt::from_u32(idx));
            assert!(!val.is_interrupt_masked(Interrupt::from_u32(idx)));
            assert_eq!(val.0, 0x00000000 << idx);
        }
    }

    #[test]
    fn struct_interrupt_clear_functions() {
        let mut val: InterruptClear;
        for idx in 0..=10 {
            val = InterruptClear(0x0);
            val = val.clear_interrupt(Interrupt::from_u32(idx));
            assert_eq!(val.0, 0x00000001 << idx);
        }
    }

    #[test]
    fn struct_interrupt_enable_functions() {
        let mut val: InterruptEnable;
        for idx in 0..=10 {
            val = InterruptEnable(0x0);
            val = val.enable_interrupt(Interrupt::from_u32(idx));
            assert!(val.is_interrupt_enabled(Interrupt::from_u32(idx)));
            assert_eq!(val.0, 0x00000001 << idx);
            val = val.disable_interrupt(Interrupt::from_u32(idx));
            assert!(!val.is_interrupt_enabled(Interrupt::from_u32(idx)));
            assert_eq!(val.0, 0x00000000 << idx);
        }
    }
}
