use super::{
    channel::Channel,
    register::{ElectricLevel, RegisterBlock},
    signal::{Negative, Positive},
};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// Wrapped GPIO pin with PWM channel feature.
///
/// `PwmPin` implements both `pwm::SetDutyCycle` and `digital::OutputPin` traits.
/// With `PwmPin`, users may seamlessly use both PWM and GPIO functions at the same time
/// without switching GPIO pin mode.
pub struct PwmPin<CHANNEL, PIN, POLARITY> {
    pub(crate) channel: CHANNEL,
    pub(crate) pin: PIN,
    pub(crate) _polarity: PhantomData<POLARITY>,
}

impl<CHANNEL, PIN, POLARITY> PwmPin<CHANNEL, PIN, POLARITY> {
    #[inline]
    pub fn free(self) -> (CHANNEL, PIN) {
        (self.channel, self.pin)
    }
}

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN>
    PwmPin<Channel<PWM, S, I, J>, PIN, Positive>
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

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN>
    PwmPin<Channel<PWM, S, I, J>, PIN, Negative>
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

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN, POLARITY>
    embedded_hal::digital::ErrorType for PwmPin<Channel<PWM, S, I, J>, PIN, POLARITY>
{
    type Error = core::convert::Infallible;
}

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN>
    embedded_hal::digital::OutputPin for PwmPin<Channel<PWM, S, I, J>, PIN, Positive>
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

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN>
    embedded_hal::digital::OutputPin for PwmPin<Channel<PWM, S, I, J>, PIN, Negative>
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

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN, POLARITY> Deref
    for PwmPin<Channel<PWM, S, I, J>, PIN, POLARITY>
{
    type Target = Channel<PWM, S, I, J>;
    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

impl<PWM: Deref<Target = RegisterBlock>, S, const I: usize, const J: usize, PIN, POLARITY> DerefMut
    for PwmPin<Channel<PWM, S, I, J>, PIN, POLARITY>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channel
    }
}
