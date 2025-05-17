//! Pulse Width Modulation peripheral.

mod channel;
mod pwm_pad;
mod register;
mod signal;
pub use channel::*;
pub use pwm_pad::PwmPin;
pub use register::*;
pub use signal::*;

#[rustfmt::skip]
mod pads;
