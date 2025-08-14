use crate::glb::{Drive, Pull, v1};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

/// Peripheral instance of a version 1 GPIO pad.
pub trait Instance<'a> {
    /// Retrieve register block for this instance.
    fn register_block(self) -> &'a v1::RegisterBlock;
}

/// Version 1 GPIO pad instance with a number.
pub trait Numbered<'a, const N: usize>: Instance<'a> {}

/// Input mode driver for version 1 GPIO pad.
pub struct Inputv1<'a> {
    number: usize,
    base: &'a v1::RegisterBlock,
}

impl<'a> Inputv1<'a> {
    /// Create a version 2 input driver from GPIO pad instance.
    #[inline]
    pub fn new(gpio: impl Instance<'a>, number: usize, pull: Pull) -> Inputv1<'a> {
        let base = gpio.register_block();
        let config = base.gpio_config[number >> 1]
            .read()
            .set_function(number & 0x1, v1::Function::Gpio)
            .enable_input(number & 0x1)
            .set_pull(number & 0x1, pull);
        unsafe { base.gpio_config[number >> 1].write(config) };
        let val = base.gpio_output_enable.read();
        unsafe { base.gpio_output_enable.write(val & !(1 << number)) };
        Inputv1 { number, base }
    }
    /// Internal constructor, DO NOT USE.
    ///
    /// Caller must ensure GPIO pad with `number` has already converted into input mode,
    /// and `base` points to a corresponding GLBv1 register block.
    #[inline]
    pub(crate) fn at(number: usize, base: &'a super::AnyRegisterBlock) -> Inputv1<'a> {
        Inputv1 {
            number,
            base: unsafe { &*(base as *const _ as *const v1::RegisterBlock) },
        }
    }
    /// Internal constructor, DO NOT USE.
    #[inline]
    pub(crate) fn set_pull(&mut self, pull: Pull) {
        let config = self.base.gpio_config[self.number >> 1]
            .read()
            .set_pull(self.number & 0x1, pull);
        unsafe { self.base.gpio_config[self.number >> 1].write(config) };
    }
    /// Internal function, DO NOT USE.
    #[inline]
    pub(crate) fn into_inner(self) -> (usize, &'a super::AnyRegisterBlock) {
        let base = unsafe { &*(self.base as *const _ as *const super::AnyRegisterBlock) };
        (self.number, base)
    }
}

impl<'a> Inputv1<'a> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n >> 1].read().enable_schmitt(n & 0x1);
        unsafe { self.base.gpio_config[n >> 1].write(config) };
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_config[n >> 1]
            .read()
            .disable_schmitt(n & 0x1);
        unsafe { self.base.gpio_config[n >> 1].write(config) };
    }
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        let n = self.number;
        unsafe { self.base.gpio_interrupt_clear.write(1 << n) };
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        let n = self.number;
        self.base.gpio_interrupt_state.read() & (1 << n) != 0
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_interrupt_mask.read() | (1 << n);
        unsafe { self.base.gpio_interrupt_mask.write(config) };
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        let n = self.number;
        let config = self.base.gpio_interrupt_mask.read() & !(1 << n);
        unsafe { self.base.gpio_interrupt_mask.write(config) };
    }
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> v1::InterruptMode {
        let n = self.number;
        self.base.gpio_interrupt_mode[n >> 1]
            .read()
            .interrupt_mode(n & 0x1)
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: v1::InterruptMode) {
        let n = self.number;
        let config = self.base.gpio_interrupt_mode[n >> 1]
            .read()
            .set_interrupt_mode(n & 0x1, val);
        unsafe { self.base.gpio_interrupt_mode[n >> 1].write(config) };
    }
}

impl<'a> ErrorType for Inputv1<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> InputPin for Inputv1<'a> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let n = self.number;
        Ok(self.base.gpio_input_value.read() & (1 << n) != 0)
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        let n = self.number;
        Ok(self.base.gpio_input_value.read() & (1 << n) == 0)
    }
}

/// Output mode driver for version 1 GPIO pad.
pub struct Outputv1<'a> {
    number: usize,
    base: &'a v1::RegisterBlock,
}

impl<'a> Outputv1<'a> {
    /// Create a version 1 output driver from GPIO pad instance.
    #[inline]
    pub fn new(gpio: impl Instance<'a>, number: usize, pull: Pull) -> Outputv1<'a> {
        let base = gpio.register_block();
        let config = base.gpio_config[number >> 1]
            .read()
            .set_function(number & 0x1, v1::Function::Gpio)
            .disable_input(number & 0x1)
            .set_pull(number & 0x1, pull);
        unsafe { base.gpio_config[number >> 1].write(config) };
        let val = base.gpio_output_enable.read();
        unsafe { base.gpio_output_enable.write(val | (1 << number)) };
        Outputv1 { number, base }
    }
    /// Internal constructor, DO NOT USE.
    ///
    /// Caller must ensure GPIO pad with `number` has already converted into output mode,
    /// and `base` points to a corresponding GLBv1 register block.
    #[inline]
    pub(crate) fn at(number: usize, base: &'a super::AnyRegisterBlock) -> Outputv1<'a> {
        let base = unsafe { &*(base as *const _ as *const v1::RegisterBlock) };
        Outputv1 { number, base }
    }
    /// Internal constructor, DO NOT USE.
    #[inline]
    pub(crate) fn set_pull(&mut self, pull: Pull) {
        let config = self.base.gpio_config[self.number >> 1]
            .read()
            .set_pull(self.number & 0x1, pull);
        unsafe { self.base.gpio_config[self.number >> 1].write(config) };
    }
    /// Internal function, DO NOT USE.
    #[inline]
    pub(crate) fn into_inner(self) -> (usize, &'a super::AnyRegisterBlock) {
        let base = unsafe { &*(self.base as *const _ as *const super::AnyRegisterBlock) };
        (self.number, base)
    }
}

impl<'a> Outputv1<'a> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        let n = self.number;
        self.base.gpio_config[n >> 1].read().drive(n & 0x1)
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        let n = self.number;
        let config = self.base.gpio_config[n >> 1].read().set_drive(n & 0x1, val);
        unsafe { self.base.gpio_config[n >> 1].write(config) };
    }
}

impl<'a> ErrorType for Outputv1<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for Outputv1<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let n = self.number;
        let val = self.base.gpio_output_value.read();
        unsafe { self.base.gpio_output_value.write(val & !(1 << n)) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        let n = self.number;
        let val = self.base.gpio_output_value.read();
        unsafe { self.base.gpio_output_value.write(val | (1 << n)) };
        Ok(())
    }
}

// TODO: need to verify if this is usable for v1
impl<'a> StatefulOutputPin for Outputv1<'a> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        let n = self.number;
        Ok(self.base.gpio_output_value.read() & (1 << n) != 0)
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        let n = self.number;
        Ok(self.base.gpio_output_value.read() & (1 << n) == 0)
    }
}
