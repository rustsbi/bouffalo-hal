use super::typestate::{I2c, JtagD0, JtagLp, JtagM0, MmUart, Pwm, Sdh, Spi, Uart};
use crate::glb::{Drive, Pull, v2};
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

/// Peripheral instance of a version 2 GPIO pad.
pub trait Instance<'a> {
    /// Retrieve register block for this instance.
    fn register_block(self) -> &'a v2::RegisterBlock;
}

/// Version 2 GPIO pad instance with a number.
pub trait Numbered<'a, const N: usize>: Instance<'a> {}

/// Input mode driver for version 2 GPIO pad.
pub struct Inputv2<'a> {
    number: usize,
    base: &'a v2::RegisterBlock,
}

impl<'a> Inputv2<'a> {
    /// Create a version 2 input driver from GPIO pad instance.
    #[inline]
    pub fn new(gpio: impl Instance<'a>, number: usize, pull: Pull) -> Inputv2<'a> {
        let base = gpio.register_block();
        let config = base.gpio_config[number]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(pull);
        unsafe { base.gpio_config[number].write(config) };
        Inputv2 { number, base }
    }
    /// Internal constructor, DO NOT USE.
    ///
    /// Caller must ensure GPIO pad with `number` has already converted into input mode,
    /// and `base` points to a corresponding GLBv2 register block.
    #[inline]
    pub(crate) fn at(number: usize, base: &'a super::AnyRegisterBlock) -> Inputv2<'a> {
        Inputv2 {
            number,
            base: unsafe { &*(base as *const _ as *const v2::RegisterBlock) },
        }
    }
    /// Internal constructor, DO NOT USE.
    #[inline]
    pub(crate) fn set_pull(&mut self, pull: Pull) {
        unsafe { self.base.gpio_config[self.number].modify(|v| v.set_pull(pull)) };
    }
    /// Internal function, DO NOT USE.
    #[inline]
    pub(crate) fn into_inner(self) -> (usize, &'a super::AnyRegisterBlock) {
        let base = unsafe { &*(self.base as *const _ as *const super::AnyRegisterBlock) };
        (self.number, base)
    }
}

impl<'a> Inputv2<'a> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n].read().enable_schmitt();
        unsafe { self.base.gpio_config[n].write(config) };
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n].read().disable_schmitt();
        unsafe { self.base.gpio_config[n].write(config) };
    }
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n].read().clear_interrupt();
        unsafe { self.base.gpio_config[n].write(config) };
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        let n = self.number;
        self.base.gpio_config[n].read().has_interrupt()
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n].read().mask_interrupt();
        unsafe { self.base.gpio_config[n].write(config) };
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n].read().unmask_interrupt();
        unsafe { self.base.gpio_config[n].write(config) };
    }
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> v2::InterruptMode {
        self.base.gpio_config[self.number].read().interrupt_mode()
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: v2::InterruptMode) {
        let config = self.base.gpio_config[self.number]
            .read()
            .set_interrupt_mode(val);
        unsafe { self.base.gpio_config[self.number].write(config) };
    }
}

impl<'a> ErrorType for Inputv2<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> InputPin for Inputv2<'a> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let n = self.number;
        Ok(self.base.gpio_input[n >> 5].read() & (1 << (n & 0x1F)) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        let n = self.number;
        Ok(self.base.gpio_input[n >> 5].read() & (1 << (n & 0x1F)) == 0)
    }
}

/// Output mode driver for version 2 GPIO pad.
pub struct Outputv2<'a> {
    number: usize,
    base: &'a v2::RegisterBlock,
}

impl<'a> Outputv2<'a> {
    /// Create a version 2 output driver from GPIO pad instance.
    #[inline]
    pub fn new(gpio: impl Instance<'a>, number: usize, pull: Pull) -> Outputv2<'a> {
        let base = gpio.register_block();
        let config = base.gpio_config[number]
            .read()
            .set_function(v2::Function::Gpio)
            .set_mode(v2::Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(pull);
        unsafe { base.gpio_config[number].write(config) };
        Outputv2 { number, base }
    }
    /// Internal constructor, DO NOT USE.
    ///
    /// Caller must ensure GPIO pad with `number` has already converted into output mode,
    /// and `base` points to a corresponding GLBv2 register block.
    #[inline]
    pub(crate) fn at(number: usize, base: &'a super::AnyRegisterBlock) -> Outputv2<'a> {
        let base = unsafe { &*(base as *const _ as *const v2::RegisterBlock) };
        Outputv2 { number, base }
    }
    /// Internal constructor, DO NOT USE.
    #[inline]
    pub(crate) fn set_pull(&mut self, pull: Pull) {
        unsafe { self.base.gpio_config[self.number].modify(|v| v.set_pull(pull)) };
    }
    /// Internal function, DO NOT USE.
    #[inline]
    pub(crate) fn into_inner(self) -> (usize, &'a super::AnyRegisterBlock) {
        let base = unsafe { &*(self.base as *const _ as *const super::AnyRegisterBlock) };
        (self.number, base)
    }
}

impl<'a> Outputv2<'a> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.base.gpio_config[self.number].read().drive()
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        let config = self.base.gpio_config[self.number].read().set_drive(val);
        unsafe { self.base.gpio_config[self.number].write(config) };
    }
}

impl<'a> ErrorType for Outputv2<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for Outputv2<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let n = self.number;
        unsafe { self.base.gpio_clear[n >> 5].write(1 << (n & 0x1F)) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let n = self.number;
        unsafe { self.base.gpio_set[n >> 5].write(1 << (n & 0x1F)) };
        Ok(())
    }
}

/// Alternate funtion mode driver for version 2 GPIO pad.
pub struct Alternatev2<'a, const N: usize, M> {
    base: &'a v2::RegisterBlock,
    _mode: PhantomData<M>,
}

const UART_GPIO_CONFIG: v2::GpioConfig = v2::GpioConfig::RESET_VALUE
    .enable_input()
    .enable_output()
    .enable_schmitt()
    .set_drive(Drive::Drive0)
    .set_pull(Pull::Up)
    .set_function(v2::Function::Uart);
const JTAG_GPIO_CONFIG: v2::GpioConfig = v2::GpioConfig::RESET_VALUE
    .enable_input()
    .disable_output()
    .enable_schmitt()
    .set_drive(Drive::Drive0)
    .set_pull(Pull::None);

impl<'a, const N: usize> Alternatev2<'a, N, ()> {
    /// Create a UART signal driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_uart(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, Uart> {
        let base = pad.register_block();
        unsafe { base.gpio_config[N].write(UART_GPIO_CONFIG) };
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a multi-media cluster UART signal driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_mm_uart(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, MmUart> {
        let base = pad.register_block();
        unsafe { base.gpio_config[N].write(UART_GPIO_CONFIG.set_function(v2::Function::MmUart)) };
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a Pulse Width Modulation signal driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_pwm<const I: usize>(
        pad: impl Numbered<'a, N>,
        pull: Pull,
    ) -> Alternatev2<'a, N, Pwm<I>> {
        let base = pad.register_block();
        let config = v2::GpioConfig::RESET_VALUE
            .disable_input()
            .enable_output()
            .enable_schmitt()
            .set_drive(Drive::Drive0)
            .set_pull(pull)
            .set_function(Pwm::<I>::FUNCTION_V2);
        unsafe { base.gpio_config[N].write(config) };
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a I2C signal driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_i2c<const I: usize>(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, I2c<I>> {
        let base = pad.register_block();
        let config = v2::GpioConfig::RESET_VALUE
            .enable_input()
            .enable_output()
            .enable_schmitt()
            .set_drive(Drive::Drive0)
            .set_pull(Pull::Up)
            .set_function(I2c::<I>::FUNCTION_V2);
        unsafe {
            base.gpio_config[N].write(config);
        }
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a D0 core JTAG pad driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_jtag_d0(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, JtagD0> {
        let base = pad.register_block();
        let config = JTAG_GPIO_CONFIG.set_function(v2::Function::JtagD0);
        unsafe { base.gpio_config[N].write(config) };
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a M0 core JTAG pad driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_jtag_m0(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, JtagM0> {
        let base = pad.register_block();
        let config = JTAG_GPIO_CONFIG.set_function(v2::Function::JtagM0);
        unsafe { base.gpio_config[N].write(config) };
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a LP core JTAG pad driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_jtag_lp(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, JtagLp> {
        let base = pad.register_block();
        let config = JTAG_GPIO_CONFIG.set_function(v2::Function::JtagLp);
        unsafe { base.gpio_config[N].write(config) };
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a SPI pad driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_spi<const I: usize>(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, Spi<I>> {
        let base = pad.register_block();
        let config = v2::GpioConfig::RESET_VALUE
            .enable_input()
            .disable_output()
            .enable_schmitt()
            .set_pull(Pull::Up)
            .set_drive(Drive::Drive0)
            .set_function(Spi::<I>::FUNCTION_V2);
        unsafe {
            base.gpio_config[N].write(config);
        }
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
    /// Create a SDH pad driver from a version 2 GPIO pad instance.
    #[inline]
    pub fn new_sdh(pad: impl Numbered<'a, N>) -> Alternatev2<'a, N, Sdh> {
        let base = pad.register_block();
        let config = v2::GpioConfig::RESET_VALUE
            .enable_input()
            .disable_output()
            .enable_schmitt()
            .set_pull(Pull::Up)
            .set_drive(Drive::Drive0)
            .set_function(v2::Function::Sdh);
        unsafe {
            base.gpio_config[N].write(config);
        }
        Alternatev2 {
            base,
            _mode: PhantomData,
        }
    }
}

impl<'a, const N: usize, M> Alternatev2<'a, N, M> {
    /// Internal function, DO NOT USE.
    #[inline]
    pub(crate) fn into_inner(self) -> &'a super::AnyRegisterBlock {
        unsafe { &*(self.base as *const _ as *const super::AnyRegisterBlock) }
    }
}
