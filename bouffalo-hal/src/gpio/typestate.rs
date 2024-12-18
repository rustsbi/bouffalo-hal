use crate::glb::v2;
use core::marker::PhantomData;

/// Alternate type state.
pub trait Alternate {
    /// Function number for this alternate type state.
    const F: v2::Function;
}

/// Input mode (type state).
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state).
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Disabled (type state).
pub struct Disabled;

/// Pulled down (type state).
pub struct PullDown;

/// Pulled up (type state).
pub struct PullUp;

/// Floating (type state).
pub struct Floating;

impl<MODE> Alternate for Input<MODE> {
    const F: v2::Function = v2::Function::Gpio;
}

impl<MODE> Alternate for Output<MODE> {
    const F: v2::Function = v2::Function::Gpio;
}

impl Alternate for Disabled {
    const F: v2::Function = v2::Function::Gpio;
}

/// UART alternate (type state).
pub struct Uart;

impl Alternate for Uart {
    const F: v2::Function = v2::Function::Uart;
}

/// Multi-media cluster UART alternate (type state).
pub struct MmUart;

impl Alternate for MmUart {
    const F: v2::Function = v2::Function::MmUart;
}

/// D0 core JTAG mode (type state).
pub struct JtagD0;

/// M0 core JTAG mode (type state).
pub struct JtagM0;

/// LP core JTAG mode (type state).
pub struct JtagLp;

impl Alternate for JtagD0 {
    const F: v2::Function = v2::Function::JtagD0;
}

impl Alternate for JtagM0 {
    const F: v2::Function = v2::Function::JtagM0;
}

impl Alternate for JtagLp {
    const F: v2::Function = v2::Function::JtagLp;
}

/// Serial Peripheral Interface mode (type state).
pub struct Spi<const F: usize>;

impl Alternate for Spi<0> {
    const F: v2::Function = v2::Function::Spi0;
}

impl Alternate for Spi<1> {
    const F: v2::Function = v2::Function::Spi1;
}

/// SD Host mode (type state).
pub struct Sdh;

impl Alternate for Sdh {
    const F: v2::Function = v2::Function::Sdh;
}

/// Inter-Integrated Circuit mode (type state).
pub struct I2c<const F: usize>;

impl Alternate for I2c<0> {
    const F: v2::Function = v2::Function::I2c0;
}

impl Alternate for I2c<1> {
    const F: v2::Function = v2::Function::I2c1;
}

impl Alternate for I2c<2> {
    const F: v2::Function = v2::Function::I2c2;
}

impl Alternate for I2c<3> {
    const F: v2::Function = v2::Function::I2c3;
}

/// Pulse Width Modulation signal mode (type state).
pub struct Pwm<const F: usize>;

impl Alternate for Pwm<0> {
    const F: v2::Function = v2::Function::Pwm0;
}

impl Alternate for Pwm<1> {
    const F: v2::Function = v2::Function::Pwm1;
}
