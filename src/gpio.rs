use crate::{glb::Function, GLB};
use base_address::BaseAddress;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

/// Individual GPIO pin.
pub struct Pin<A: BaseAddress, const N: usize, M: Alternate> {
    base: GLB<A>,
    _mode: PhantomData<M>,
}

/// Alternate type state.
pub trait Alternate {
    /// Function number for this alternate type state.
    const F: Function;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Pulled down (type state)
pub struct PullDown;

/// Pulled up (type state)
pub struct PullUp;

/// Floating (type state)
pub struct Floating;

impl<MODE> Alternate for Input<MODE> {
    const F: Function = Function::Gpio;
}

impl<MODE> Alternate for Output<MODE> {
    const F: Function = Function::Gpio;
}

impl<A: BaseAddress, const N: usize, M> ErrorType for Pin<A, N, Input<M>> {
    type Error = core::convert::Infallible;
}

impl<A: BaseAddress, const N: usize, M> ErrorType for Pin<A, N, Output<M>> {
    type Error = core::convert::Infallible;
}

impl<A: BaseAddress, const N: usize, M> InputPin for Pin<A, N, Input<M>> {
    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.base.gpio_input[N >> 5].read() & (1 << (N & 0x1F)) != 0)
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.base.gpio_input[N >> 5].read() & (1 << (N & 0x1F)) == 0)
    }
}

impl<A: BaseAddress, const N: usize, M> OutputPin for Pin<A, N, Output<M>> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { self.base.gpio_clear[N >> 5].write(1 << (N & 0x1F)) };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { self.base.gpio_set[N >> 5].write(1 << (N & 0x1F)) };
        Ok(())
    }
}

impl<A: BaseAddress, const N: usize, M> Pin<A, N, Input<M>> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        let config = self.base.gpio_config[N].read().enable_schmitt();
        self.base.gpio_config[N].write(config);
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        let config = self.base.gpio_config[N].read().disable_schmitt();
        self.base.gpio_config[N].write(config);
    }
}
