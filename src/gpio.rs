//! General Purpose Input/Output.
use crate::{
    glb::{Drive, Function, InterruptMode, Mode, Pull},
    GLB,
};
use base_address::BaseAddress;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

/// Individual GPIO pin.
pub struct Pin<A: BaseAddress, const N: usize, M: Alternate> {
    pub(crate) base: GLB<A>,
    pub(crate) _mode: PhantomData<M>,
}

/// Alternate type state.
pub trait Alternate {
    /// Function number for this alternate type state.
    const F: Function;
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
    const F: Function = Function::Gpio;
}

impl<MODE> Alternate for Output<MODE> {
    const F: Function = Function::Gpio;
}

impl Alternate for Disabled {
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

// We do not support StatefulOutputPin and ToggleableOutputPin here, because the hardware does not 
// have such functionality to read back the previously set pin state.
// It is recommended that users add a variable to store the pin state if necessary; see examples/gpio-demo.

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
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        let config = self.base.gpio_config[N].read().clear_interrupt();
        self.base.gpio_config[N].write(config);
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        self.base.gpio_config[N].read().has_interrupt()
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        let config = self.base.gpio_config[N].read().mask_interrupt();
        self.base.gpio_config[N].write(config);
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        let config = self.base.gpio_config[N].read().unmask_interrupt();
        self.base.gpio_config[N].write(config);
    }
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> InterruptMode {
        self.base.gpio_config[N].read().interrupt_mode()
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: InterruptMode) {
        let config = self.base.gpio_config[N].read().set_interrupt_mode(val);
        self.base.gpio_config[N].write(config);
    }
}

impl<A: BaseAddress, const N: usize, M> Pin<A, N, Output<M>> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.base.gpio_config[N].read().drive()
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        let config = self.base.gpio_config[N].read().set_drive(val);
        self.base.gpio_config[N].write(config);
    }
}

impl<A: BaseAddress, const N: usize, M: Alternate> Pin<A, N, M> {
    /// Configures the pin to operate as a pull up output pin.
    #[inline]
    pub fn into_pull_up_output(self) -> Pin<A, N, Output<PullUp>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(Function::Gpio)
            .set_mode(Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(Pull::Up);
        self.base.gpio_config[N].write(config);
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull down output pin.
    #[inline]
    pub fn into_pull_down_output(self) -> Pin<A, N, Output<PullDown>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(Function::Gpio)
            .set_mode(Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(Pull::Down);
        self.base.gpio_config[N].write(config);
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a floating output pin.
    #[inline]
    pub fn into_floating_output(self) -> Pin<A, N, Output<Floating>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(Function::Gpio)
            .set_mode(Mode::SetClear)
            .disable_input()
            .enable_output()
            .set_pull(Pull::None);
        self.base.gpio_config[N].write(config);
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull up input pin.
    #[inline]
    pub fn into_pull_up_input(self) -> Pin<A, N, Input<PullUp>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(Function::Gpio)
            .set_mode(Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(Pull::Up);
        self.base.gpio_config[N].write(config);
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a pull down input pin.
    #[inline]
    pub fn into_pull_down_input(self) -> Pin<A, N, Input<PullDown>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(Function::Gpio)
            .set_mode(Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(Pull::Down);
        self.base.gpio_config[N].write(config);
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
    /// Configures the pin to operate as a floating input pin.
    #[inline]
    pub fn into_floating_input(self) -> Pin<A, N, Input<Floating>> {
        let config = self.base.gpio_config[N]
            .read()
            .set_function(Function::Gpio)
            .set_mode(Mode::SetClear)
            .enable_input()
            .disable_output()
            .set_pull(Pull::None);
        self.base.gpio_config[N].write(config);
        Pin {
            base: self.base,
            _mode: PhantomData,
        }
    }
}

/// Available GPIO pins.
pub struct Pins<A: BaseAddress> {
    // GPIO I/O 0.
    pub io0: Pin<A, 0, Disabled>,
    // GPIO I/O 1.
    pub io1: Pin<A, 1, Disabled>,
    // GPIO I/O 2.
    pub io2: Pin<A, 2, Disabled>,
    // GPIO I/O 3.
    pub io3: Pin<A, 3, Disabled>,
    // GPIO I/O 8.
    pub io8: Pin<A, 8, Disabled>,
    // GPIO I/O 22.
    pub io22: Pin<A, 22, Disabled>,
    // GPIO I/O 23.
    pub io23: Pin<A, 23, Disabled>,
}
