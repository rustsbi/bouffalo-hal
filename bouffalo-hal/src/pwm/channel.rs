use crate::glb;
use crate::gpio::{self, Alternate};
use core::marker::PhantomData;
use embedded_time::rate::Hertz;

use super::{
    Clock, Instance,
    pwm_pad::PwmPin,
    register::RegisterBlock,
    signal::{HasPwmExternalBreak, HasPwmSignal, Negative, Positive, Signal0, Signal1},
};

/// Managed pulse width modulation peripheral.
pub struct Pwm<'a, S> {
    /// Group 0 of current PWM peripheral.
    pub group0: Channels<'a, S, 0>,
    /// Group 1 of current PWM peripheral.
    pub group1: Channels<'a, S, 1>,
}

impl<'a, S0: Signal0, S1: Signal1> Pwm<'a, (S0, S1)> {
    /// Creates a pulse width modulation instance with given signal settings.
    #[rustfmt::skip]
    #[inline]
    pub fn new(pwm: impl Instance<'a>, signal_0: S0, signal_1: S1, glb: &glb::v2::RegisterBlock) -> Self {
        unsafe {
            glb.pwm_config
                .modify(|config| config.set_signal_0(S0::VALUE).set_signal_1(S1::VALUE));
            glb.clock_config_1.modify(|config| config.enable_pwm());
        }
        drop((signal_0, signal_1));
        let pwm = pwm.register_block();
        Pwm {
            group0: Channels {
                channel0: Channel::new(pwm),
                channel1: Channel::new(pwm),
                channel2: Channel::new(pwm),
                channel3: Channel::new(pwm),
                external_break: ExternalBreak { _signals: PhantomData },
                pwm,
                _signals: PhantomData,
            },
            group1: Channels {
                channel0: Channel::new(pwm),
                channel1: Channel::new(pwm),
                channel2: Channel::new(pwm),
                channel3: Channel::new(pwm),
                external_break: ExternalBreak { _signals: PhantomData },
                pwm,
                _signals: PhantomData,
            },
        }
    }
}

/// PWM group with all its channels.
pub struct Channels<'a, S, const I: usize> {
    /// Channel 0 of current PWM group.
    pub channel0: Channel<'a, S, I, 0>,
    /// Channel 1 of current PWM group.
    pub channel1: Channel<'a, S, I, 1>,
    /// Channel 2 of current PWM group.
    pub channel2: Channel<'a, S, I, 2>,
    /// Channel 3 of current PWM group.
    pub channel3: Channel<'a, S, I, 3>,
    /// External break signal for current PWM group.
    pub external_break: ExternalBreak<S, I>,
    pwm: &'a RegisterBlock,
    _signals: PhantomData<S>,
}

impl<'a, S, const I: usize> Channels<'a, S, I> {
    /// Configure clock settings for current PWM group.
    ///
    /// Clock settings would affect all the channels in the PWM group.
    #[inline]
    pub fn set_clock(&mut self, frequency: Hertz, choice: super::ClockSource, source: impl Clock) {
        let source_freq = source.pwm_clock(choice);
        let clock_divisor = source_freq.0 / frequency.0;
        if !(1..=65535).contains(&clock_divisor) {
            panic!("impossible frequency");
        }
        unsafe {
            self.pwm.group[I].group_config.modify(|val| {
                val.set_clock_source(choice)
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
pub struct Channel<'a, S, const I: usize, const J: usize> {
    pub(crate) pwm: &'a RegisterBlock,
    _signals: PhantomData<S>,
}

impl<'a, S, const I: usize, const J: usize> Channel<'a, S, I, J> {
    /// Wrap current channel as positive signal with GPIO pin.
    ///
    /// This function statically checks if target GPIO pin mode matches current PWM channel.
    /// If won't match, it will raise compile error.
    #[inline]
    pub fn positive_signal_pin<'b, const N: usize, const F: usize>(
        self,
        pin: Alternate<'b, N, gpio::Pwm<F>>,
    ) -> PwmPin<Self, Alternate<'b, N, gpio::Pwm<F>>, Positive>
    where
        Alternate<'b, N, gpio::Pwm<F>>: HasPwmSignal<S, I, J, Positive>,
    {
        PwmPin::new(self, pin)
    }
    /// Wrap current channel as negative signal with GPIO pin.
    ///
    /// This function statically checks if target GPIO pin mode matches current PWM channel.
    /// If won't match, it will raise compile error.
    #[inline]
    pub fn negative_signal_pin<'b, const N: usize, const F: usize>(
        self,
        pin: Alternate<'b, N, gpio::Pwm<F>>,
    ) -> PwmPin<Self, Alternate<'b, N, gpio::Pwm<F>>, Negative>
    where
        Alternate<'b, N, gpio::Pwm<F>>: HasPwmSignal<S, I, J, Negative>,
    {
        PwmPin::new(self, pin)
    }

    /// Internal constructor.
    #[inline]
    const fn new(pwm: &'a RegisterBlock) -> Self {
        Self {
            pwm,
            _signals: PhantomData,
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
    pub fn external_break_pin<'a, const N: usize, const F: usize>(
        self,
        pin: Alternate<'a, N, gpio::Pwm<F>>,
    ) -> PwmPin<Self, Alternate<'a, N, gpio::Pwm<F>>, ()>
    where
        Alternate<'a, N, gpio::Pwm<F>>: HasPwmExternalBreak<I>,
    {
        PwmPin::new(self, pin)
    }
}

impl<'a, S, const I: usize, const J: usize> embedded_hal::pwm::ErrorType for Channel<'a, S, I, J> {
    type Error = core::convert::Infallible;
}

impl<'a, S, const I: usize, const J: usize> embedded_hal::pwm::SetDutyCycle
    for Channel<'a, S, I, J>
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
