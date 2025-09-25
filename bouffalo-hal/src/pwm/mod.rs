//! Pulse Width Modulation peripheral.

mod channel;
mod pwm_pad;
mod register;
mod signal;
pub use channel::*;
pub use pwm_pad::PwmPin;
pub use register::*;
pub use signal::*;

pub use register::ClockSource;

#[rustfmt::skip]
mod pads;

use embedded_time::rate::Hertz;

/// Peripheral instance for PWM.
pub trait Instance<'a> {
    /// Retrieve register block from this instance.
    fn register_block(self) -> &'a RegisterBlock;
}

/// PWM clock source.
pub trait Clock {
    /// Clock frequency source in hertz, chosen by `choice`.
    fn pwm_clock(self, choice: ClockSource) -> Hertz;
}
