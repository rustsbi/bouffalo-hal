//! Generic DAC, ADC and ACOMP interface control peripheral.

mod adc;
pub mod asynch;
mod register;

pub use adc::*;
pub use asynch::*;
pub use register::*;
