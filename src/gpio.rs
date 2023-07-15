//! General Purpose Input/Output.
#[cfg(feature = "glb-v1")]
use crate::glb::v1::{Drive, Function, InterruptMode, Pull};
#[cfg(feature = "glb-v2")]
use crate::glb::v2::{Drive, Function, InterruptMode, Mode, Pull};
#[cfg(any(feature = "glb-v1", feature = "glb-v2"))]
use crate::GLB;
use base_address::BaseAddress;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

/// Individual GPIO pin.
pub struct Pin<A: BaseAddress, const N: usize, M: Alternate> {
    #[cfg(any(feature = "glb-v1", feature = "glb-v2"))]
    pub(crate) base: GLB<A>,
    #[cfg(not(any(feature = "glb-v1", feature = "glb-v2")))]
    pub(crate) _base_not_implemented: PhantomData<A>,
    pub(crate) _mode: PhantomData<M>,
}

/// Alternate type state.
pub trait Alternate {
    /// Function number for this alternate type state.
    #[cfg(feature = "glb-v2")]
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
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::Gpio;
}

impl<MODE> Alternate for Output<MODE> {
    #[cfg(feature = "glb-v2")]
    const F: Function = Function::Gpio;
}

impl Alternate for Disabled {
    #[cfg(feature = "glb-v2")]
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
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                Ok(self.base.gpio_input_value.read() & (1 << N) != 0)
            } else if #[cfg(feature = "glb-v2")] {
                Ok(self.base.gpio_input[N >> 5].read() & (1 << (N & 0x1F)) != 0)
            } else {
                unimplemented!()
            }
        }
    }
    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                Ok(self.base.gpio_input_value.read() & (1 << N) == 0)
            } else if #[cfg(feature = "glb-v2")] {
                Ok(self.base.gpio_input[N >> 5].read() & (1 << (N & 0x1F)) == 0)
            } else {
                unimplemented!()
            }
        }
    }
}

impl<A: BaseAddress, const N: usize, M> OutputPin for Pin<A, N, Output<M>> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let val = self.base.gpio_output_value.read();
                unsafe { self.base.gpio_output_value.write(val & !(1 << N)) };
                Ok(())
            } else if #[cfg(feature = "glb-v2")] {
                unsafe { self.base.gpio_clear[N >> 5].write(1 << (N & 0x1F)) };
                Ok(())
            } else {
                unimplemented!()
            }
        }
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let val = self.base.gpio_output_value.read();
                unsafe { self.base.gpio_output_value.write(val | (1 << N)) };
                Ok(())
            } else if #[cfg(feature = "glb-v2")] {
                unsafe { self.base.gpio_set[N >> 5].write(1 << (N & 0x1F)) };
                Ok(())
            } else {
                unimplemented!()
            }
        }
    }
}

// We do not support StatefulOutputPin and ToggleableOutputPin here, because the hardware does not
// have such functionality to read back the previously set pin state.
// It is recommended that users add a variable to store the pin state if necessary; see examples/gpio-demo.

impl<A: BaseAddress, const N: usize, M> Pin<A, N, Input<M>> {
    /// Enable schmitt trigger.
    #[inline]
    pub fn enable_schmitt(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1].read().enable_schmitt(N & 0x1);
                self.base.gpio_config[N >> 1].write(config);
            } else if #[cfg(feature = "glb-v2")] {
                let config = self.base.gpio_config[N].read().enable_schmitt();
                self.base.gpio_config[N].write(config);
            } else {
                unimplemented!()
            }
        }
    }
    /// Disable schmitt trigger.
    #[inline]
    pub fn disable_schmitt(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1].read().disable_schmitt(N & 0x1);
                self.base.gpio_config[N >> 1].write(config);
            } else if #[cfg(feature = "glb-v2")] {
                let config = self.base.gpio_config[N].read().disable_schmitt();
                self.base.gpio_config[N].write(config);
            } else {
                unimplemented!()
            }
        }
    }
    /// Clear interrupt flag.
    #[inline]
    pub fn clear_interrupt(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                unsafe { self.base.gpio_interrupt_clear.write(1 << N) };
            } else if #[cfg(feature = "glb-v2")] {
                let config = self.base.gpio_config[N].read().clear_interrupt();
                self.base.gpio_config[N].write(config);
            } else {
                unimplemented!()
            }
        }
    }
    /// Check if interrupt flag is set.
    #[inline]
    pub fn has_interrupt(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                self.base.gpio_interrupt_state.read() & (1 << N) != 0
            } else if #[cfg(feature = "glb-v2")] {
                self.base.gpio_config[N].read().has_interrupt()
            } else {
                unimplemented!()
            }
        }
    }
    /// Mask interrupt.
    #[inline]
    pub fn mask_interrupt(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_interrupt_mask.read() | (1 << N);
                unsafe { self.base.gpio_interrupt_mask.write(config) };
            } else if #[cfg(feature = "glb-v2")] {
                let config = self.base.gpio_config[N].read().mask_interrupt();
                self.base.gpio_config[N].write(config);
            } else {
                unimplemented!()
            }
        }
    }
    /// Unmask interrupt.
    #[inline]
    pub fn unmask_interrupt(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_interrupt_mask.read() & !(1 << N);
                unsafe { self.base.gpio_interrupt_mask.write(config) };
            } else if #[cfg(feature = "glb-v2")] {
                let config = self.base.gpio_config[N].read().unmask_interrupt();
                self.base.gpio_config[N].write(config);
            } else {
                unimplemented!()
            }
        }
    }
}

#[cfg(feature = "glb-v1")]
impl<A: BaseAddress, const N: usize, M> Pin<A, N, Input<M>> {
    /// Get interrupt mode.
    #[inline]
    pub fn interrupt_mode(&self) -> InterruptMode {
        self.base.gpio_interrupt_mode[N >> 1]
            .read()
            .interrupt_mode(N & 0x1)
    }
    /// Set interrupt mode.
    #[inline]
    pub fn set_interrupt_mode(&mut self, val: InterruptMode) {
        let config = self.base.gpio_interrupt_mode[N >> 1]
            .read()
            .set_interrupt_mode(N & 0x1, val);
        self.base.gpio_interrupt_mode[N >> 1].write(config);
    }
}

#[cfg(feature = "glb-v2")]
impl<A: BaseAddress, const N: usize, M> Pin<A, N, Input<M>> {
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

#[cfg(feature = "glb-v1")]
impl<A: BaseAddress, const N: usize, M> Pin<A, N, Output<M>> {
    /// Get drive strength of this pin.
    #[inline]
    pub fn drive(&self) -> Drive {
        self.base.gpio_config[N >> 1].read().drive(N & 0x1)
    }
    /// Set drive strength of this pin.
    #[inline]
    pub fn set_drive(&mut self, val: Drive) {
        let config = self.base.gpio_config[N >> 1].read().set_drive(N & 0x1, val);
        self.base.gpio_config[N >> 1].write(config);
    }
}

#[cfg(feature = "glb-v2")]
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
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1]
                    .read()
                    .set_function(N & 0x1, Function::Gpio)
                    .disable_input(N & 0x1)
                    .set_pull(N & 0x1, Pull::Up);
                self.base.gpio_config[N >> 1].write(config);
                let val = self.base.gpio_output_enable.read();
                unsafe { self.base.gpio_output_enable.write(val | (1 << N)) };
                Pin {
                    base: self.base,
                    _mode: PhantomData,
                }
            } else if #[cfg(feature = "glb-v2")] {
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
            } else {
                unimplemented!()
            }
        }
    }
    /// Configures the pin to operate as a pull down output pin.
    #[inline]
    pub fn into_pull_down_output(self) -> Pin<A, N, Output<PullDown>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1]
                    .read()
                    .set_function(N & 0x1, Function::Gpio)
                    .disable_input(N & 0x1)
                    .set_pull(N & 0x1, Pull::Down);
                self.base.gpio_config[N >> 1].write(config);
                let val = self.base.gpio_output_enable.read();
                unsafe { self.base.gpio_output_enable.write(val | (1 << N)) };
                Pin {
                    base: self.base,
                    _mode: PhantomData,
                }
            } else if #[cfg(feature = "glb-v2")] {
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
            } else {
                unimplemented!()
            }
        }
    }
    /// Configures the pin to operate as a floating output pin.
    #[inline]
    pub fn into_floating_output(self) -> Pin<A, N, Output<Floating>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1]
                    .read()
                    .set_function(N & 0x1, Function::Gpio)
                    .disable_input(N & 0x1)
                    .set_pull(N & 0x1, Pull::None);
                self.base.gpio_config[N >> 1].write(config);
                let val = self.base.gpio_output_enable.read();
                unsafe { self.base.gpio_output_enable.write(val | (1 << N)) };
                Pin {
                    base: self.base,
                    _mode: PhantomData,
                }
            } else if #[cfg(feature = "glb-v2")] {
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
            } else {
                unimplemented!()
            }
        }
    }
    /// Configures the pin to operate as a pull up input pin.
    #[inline]
    pub fn into_pull_up_input(self) -> Pin<A, N, Input<PullUp>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1]
                    .read()
                    .set_function(N & 0x1, Function::Gpio)
                    .enable_input(N & 0x1)
                    .set_pull(N & 0x1, Pull::Up);
                self.base.gpio_config[N >> 1].write(config);
                let val = self.base.gpio_output_enable.read();
                unsafe { self.base.gpio_output_enable.write(val & !(1 << N)) };
                Pin {
                    base: self.base,
                    _mode: PhantomData,
                }
            } else if #[cfg(feature = "glb-v2")] {
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
            } else {
                unimplemented!()
            }
        }
    }
    /// Configures the pin to operate as a pull down input pin.
    #[inline]
    pub fn into_pull_down_input(self) -> Pin<A, N, Input<PullDown>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1]
                    .read()
                    .set_function(N & 0x1, Function::Gpio)
                    .enable_input(N & 0x1)
                    .set_pull(N & 0x1, Pull::Down);
                self.base.gpio_config[N >> 1].write(config);
                let val = self.base.gpio_output_enable.read();
                unsafe { self.base.gpio_output_enable.write(val & !(1 << N)) };
                Pin {
                    base: self.base,
                    _mode: PhantomData,
                }
            } else if #[cfg(feature = "glb-v2")] {
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
            } else {
                unimplemented!()
            }
        }
    }
    /// Configures the pin to operate as a floating input pin.
    #[inline]
    pub fn into_floating_input(self) -> Pin<A, N, Input<Floating>> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "glb-v1")] {
                let config = self.base.gpio_config[N >> 1]
                    .read()
                    .set_function(N & 0x1, Function::Gpio)
                    .enable_input(N & 0x1)
                    .set_pull(N & 0x1, Pull::None);
                self.base.gpio_config[N >> 1].write(config);
                let val = self.base.gpio_output_enable.read();
                unsafe { self.base.gpio_output_enable.write(val & !(1 << N)) };
                Pin {
                    base: self.base,
                    _mode: PhantomData,
                }
            } else if #[cfg(feature = "glb-v2")] {
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
            } else {
                unimplemented!()
            }
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
    // GPIO I/O 10.
    pub io10: Pin<A, 10, Disabled>,
    // GPIO I/O 11.
    pub io11: Pin<A, 11, Disabled>,
    // GPIO I/O 12.
    pub io12: Pin<A, 12, Disabled>,
    // GPIO I/O 13.
    pub io13: Pin<A, 13, Disabled>,
    // GPIO I/O 14.
    pub io14: Pin<A, 14, Disabled>,
    // GPIO I/O 15.
    pub io15: Pin<A, 15, Disabled>,
    // GPIO I/O 22.
    pub io22: Pin<A, 22, Disabled>,
    // GPIO I/O 23.
    pub io23: Pin<A, 23, Disabled>,
    // GPIO I/O 27.
    pub io27: Pin<A, 27, Disabled>,
    // GPIO I/O 28.
    pub io28: Pin<A, 28, Disabled>,
}
