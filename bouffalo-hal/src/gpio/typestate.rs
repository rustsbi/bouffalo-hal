use crate::glb::v2;

/// UART alternate (type state).
pub struct Uart;

/// Multi-media cluster UART alternate (type state).
pub struct MmUart;

/// D0 core JTAG mode (type state).
pub struct JtagD0;

/// M0 core JTAG mode (type state).
pub struct JtagM0;

/// LP core JTAG mode (type state).
pub struct JtagLp;

/// Serial Peripheral Interface mode (type state).
pub struct Spi<const F: usize>;

impl<const F: usize> Spi<F> {
    /// SPI function constant in GLB v2 peripheral.
    pub const FUNCTION_V2: v2::Function = match F {
        0 => v2::Function::Spi0,
        1 => v2::Function::Spi1,
        _ => unreachable!(),
    };
}

/// SD Host mode (type state).
pub struct Sdh;

/// Inter-Integrated Circuit mode (type state).
pub struct I2c<const F: usize>;

impl<const F: usize> I2c<F> {
    /// I2C function constant in GLB v2 peripheral.
    pub const FUNCTION_V2: v2::Function = match F {
        0 => v2::Function::I2c0,
        1 => v2::Function::I2c1,
        2 => v2::Function::I2c2,
        3 => v2::Function::I2c3,
        _ => unreachable!(),
    };
}

/// Pulse Width Modulation signal mode (type state).
pub struct Pwm<const F: usize>;

impl<const F: usize> Pwm<F> {
    /// PWM function constant in GLB v2 peripheral.
    pub const FUNCTION_V2: v2::Function = match F {
        0 => v2::Function::Pwm0,
        1 => v2::Function::Pwm1,
        _ => unreachable!(),
    };
}
